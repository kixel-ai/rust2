/* The api fixture's C test: the script fixtures/api/examples/transcript.rs runs
 * natively, through the exported C ABI. tests/fixtures.rs compares the transcripts.
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

/* Print each of `numbers`' lists, its floats as their bits, as transcript.rs does. */
#define PRINT_LIST(list, fmt, cast)                                           \
	do {                                                                  \
		printf("[");                                                  \
		for (size_t i_ = 0; i_ < (list).len; i_++) {                  \
			printf(i_ ? ", " fmt : fmt, cast((list).ptr[i_]));    \
		}                                                             \
		printf("]");                                                  \
	} while (0)

static unsigned long long f32_bits(float f) {
	uint32_t bits;
	memcpy(&bits, &f, sizeof bits);
	return bits;
}

static unsigned long long f64_bits(double f) {
	uint64_t bits;
	memcpy(&bits, &f, sizeof bits);
	return bits;
}

static void numbers(const char *what, const api_Numbers *n) {
	printf("%s u16 ", what);
	PRINT_LIST(n->u16s, "%u", (unsigned));
	printf("\n");
	printf("%s u32 ", what);
	PRINT_LIST(n->u32s, "%u", (unsigned));
	printf("\n");
	printf("%s u64 ", what);
	PRINT_LIST(n->u64s, "%llu", (unsigned long long));
	printf("\n");
	printf("%s i8 ", what);
	PRINT_LIST(n->i8s, "%d", (int));
	printf("\n");
	printf("%s i16 ", what);
	PRINT_LIST(n->i16s, "%d", (int));
	printf("\n");
	printf("%s i32 ", what);
	PRINT_LIST(n->i32s, "%d", (int));
	printf("\n");
	printf("%s i64 ", what);
	PRINT_LIST(n->i64s, "%lld", (long long));
	printf("\n");
	printf("%s f32 ", what);
	PRINT_LIST(n->f32s, "%llu", f32_bits);
	printf("\n");
	printf("%s f64 ", what);
	PRINT_LIST(n->f64s, "%llu", f64_bits);
	printf("\n");
	if (n->some.ptr) {
		printf("%s some Some(", what);
		PRINT_LIST(n->some, "%d", (int));
		printf(")\n");
	} else {
		printf("%s some None\n", what);
	}
}

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

/* made's closure: a string it hands Rust, as api_buf_copy made it. */
static api_buf made_times(void *data, uint32_t n) {
	(void)data;
	char text[32];
	snprintf(text, sizeof text, "%u times", (unsigned)n);
	return api_buf_copy(str(text));
}

/* reissue's closures: a new ticket, and a copy of one C keeps. */
static api_Ticket reissue_next(void *data, api_Ticket ticket) {
	(void)data;
	return api_Ticket_new(api_Ticket_number(&ticket) + 1);
}

static api_Ticket reissue_kept(void *data, api_Ticket ticket) {
	(void)ticket;
	return *(const api_Ticket *)data;
}

/* shifted's closure: the moment 500 ticks later. */
static api_Moment moment_after(void *data, api_Moment moment) {
	(void)data;
	return api_Moment_after(&moment, 500);
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

/* Closures C implements that Rust keeps or that return what Rust then owns, and readers
 * C implements: whether each behaved. */
static bool closures(void) {
	print("made ", api_made(3, made_times, NULL), "\n");

	/* A closure taking and returning `Copy` values copies them, never running their `Clone`. */
	api_Ticket kept = api_Ticket_new(9);
	api_Ticket seven = api_Ticket_new(7);
	api_Ticket given = api_reissue(seven, reissue_next, NULL);
	api_Ticket held = api_reissue(seven, reissue_kept, &kept);
	printf("reissued %u %u, cloned %u\n", (unsigned)api_Ticket_number(&given), (unsigned)api_Ticket_number(&held),
		(unsigned)api_Ticket_clones());
	/* A record's traits borrow the checked handle it holds. */
	api_Mark *nine = api_Mark_of(9);
	api_Seat front = api_seat(nine, 1, NULL);
	api_Seat back = api_seat(nine, 2, NULL);
	print("seat ", api_Seat_debug(&front, NULL), "");
	printf(" %s %s\n", api_Seat_eq(&front, &front, NULL) ? "true" : "false",
		api_Seat_eq(&front, &back, NULL) ? "true" : "false");
	api_Seat_free(&back);
	api_Seat_free(&front);
	api_Mark_free(nine);

	/* A kept closure, called on another thread after the call that took it returned. */
	bool spawned_freed = false;
	api_Spawned *spawned = api_Spawned_new(spawned_task, &spawned_freed, spawned_free);
	uint32_t joined = api_Spawned_join(spawned, NULL);
	api_Spawned_free(spawned);
	if (joined != 7 || !spawned_freed) {
		return false;
	}

	api_waker waker = {.wake = wake, .drop = drop, .data = NULL};
	api_Moment start = api_Moment_new(0, 1000).value;
	driver d = {.start = start};
	api_drive_future *drive = api_drive(start, drive_poll, &d, drive_free);
	api_buf stopped;
	int woken = wakes;
	while (api_drive_poll(drive, waker, &stopped) == API_POLL_PENDING) {
	}
	print("drive ", stopped, "\n");
	bool ok = d.freed && wakes == woken + 1;
	api_drive_future_free(drive);

	/* Dropped before it finishes, it frees the closure it kept. */
	driver idle = {.start = start};
	drive = api_drive(start, drive_idle, &idle, drive_free);
	ok = ok && api_drive_poll(drive, waker, &stopped) == API_POLL_PENDING && !idle.freed;
	api_drive_future_free(drive);
	ok = ok && idle.freed;

	memory hello = {.bytes = "hello world", .len = 11};
	tail((api_foreign_reader){.read = memory_read, .seek = memory_seek, .data = &hello});
	tail((api_foreign_reader){.read = broken_read, .seek = broken_seek, .data = NULL});

	/* A seek from the start reaches the reader whole, past INT64_MAX, as `Cursor` takes one. */
	uint64_t landed = 0;
	api_buf landed_err;
	if (api_seek_to((api_foreign_reader){.read = memory_read, .seek = memory_seek, .data = &hello}, UINT64_MAX, &landed,
			&landed_err)) {
		printf("sought %llu\n", (unsigned long long)landed);
	} else {
		print("sought error ", landed_err, "\n");
	}
	print("non zeros ", api_describe_non_zeros(api_non_zeros()), "\n");
	api_option_non_zero_u64 hops[] = {{0}, {.some = true, .value = 1}, {.some = true, .value = UINT64_MAX}};
	for (size_t i = 0; i < sizeof hops / sizeof hops[0]; i++) {
		api_option_non_zero_u64 hop = api_next_hop(hops[i]);
		if (hop.some) {
			printf("hop Some(%llu)\n", (unsigned long long)hop.value);
		} else {
			printf("hop None\n");
		}
	}
	printf("farther hop %u\n", (unsigned)api_farther_hop((api_Pair_non_zero_u16){.left = 3, .right = UINT16_MAX}));
	{
		api_status local_status = 0;
		api_Local *local = api_Local_new();
		api_Local *shared = api_local_or_new(local, &local_status);
		uint32_t once = api_Local_bump(local, &local_status);
		printf("local %u %u\n", once, api_Local_bump(shared, &local_status));
		api_Local *fresh = api_local_or_new(NULL, &local_status);
		printf("local fresh %u\n", api_Local_bump(fresh, &local_status));
		printf("local either %u\n", api_bump_either(NULL, shared, &local_status));
		const api_Local *froms[2] = {local, NULL};
		const char *labels[2] = {"local later", "local later fresh"};
		for (int i = 0; i < 2; i++) {
			api_local_or_new_later_future *later = api_local_or_new_later(froms[i], &local_status);
			api_Local *made = NULL;
			while (local_status == 0 &&
				   api_local_or_new_later_poll(later, (api_waker){0}, &made, &local_status) == API_POLL_PENDING) {
			}
			printf("%s %u\n", labels[i], made ? api_Local_bump(made, &local_status) : 0);
			api_Local_free(made);
			api_local_or_new_later_future_free(later);
		}
		if (local_status != 0) {
			ok = false;
		}
		api_Local_free(fresh);
		api_Local_free(shared);
		api_Local_free(local);
	}
	return ok;
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

/* A lent closure: adds data's amount to the counter lent for the call. */
static bool edit(void *data, api_Counter *counter) {
	api_Counter_add(counter, *(const uint64_t *)data, NULL, NULL, NULL);
	return api_Counter_edited(counter, NULL) && api_Counter_get(counter, NULL) == 6;
}

/* Lent closures borrowing their object shared: one `Sync`, one not. */
static uint32_t seven(void *data) {
	(void)data;
	return 7;
}

/* A lent closure of an inferred instance's type. */
static uint32_t triple(void *data, uint32_t side) {
	(void)data;
	return side * 3;
}

static uint64_t visit(void *data, const api_Cursor *cursor) {
	(void)data;
	return api_Cursor_at(cursor, NULL) * 2;
}

static void print_visit(void *data, uint32_t at) {
	(void)data;
	printf("visit %u\n", at);
}

static uint32_t peek_tally(void *data, const api_Tally *tally) {
	(void)data;
	return api_Tally_bump(tally, NULL);
}

/* A lent closure, which Rust calls on another thread: keeps levels above data's. */
static bool keep_line(void *data, uint8_t level, api_buf line) {
	printf("filter %u %.*s\n", level, (int)line.len, (const char *)line.ptr);
	return level > *(const uint8_t *)data;
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

/* Bytes as Rust's `escape_ascii` prints those of a path. */
static void print_escaped(const char *before, api_buf buf, const char *after) {
	printf("%s", before);
	for (size_t i = 0; i < buf.len; i++) {
		uint8_t b = buf.ptr[i];
		switch (b) {
		case '\t':
			printf("\\t");
			break;
		case '\r':
			printf("\\r");
			break;
		case '\n':
			printf("\\n");
			break;
		case '\\':
		case '\'':
		case '"':
			printf("\\%c", b);
			break;
		default:
			if (b >= 0x20 && b < 0x7f) {
				putchar(b);
			} else {
				printf("\\x%02x", b);
			}
		}
	}
	printf("%s", after);
	api_buf_free(buf);
}

/* A socket address, as Rust's Display prints the ones the script makes. */
static void print_addr(api_socket_addr addr) {
	if (!addr.v6) {
		const uint8_t *ip = addr.addr.v4.ip.octets;
		printf("%u.%u.%u.%u:%u", ip[0], ip[1], ip[2], ip[3], addr.addr.v4.port);
		return;
	}
	/* Only the loopback, `::1`. */
	printf("[::%u%%%u]:%u", addr.addr.v6.ip.octets[15], addr.addr.v6.scope_id, addr.addr.v6.port);
}

/* Bytes in hex. */
static void print_hex(const uint8_t *bytes, size_t len) {
	for (size_t i = 0; i < len; i++) {
		printf("%02x", bytes[i]);
	}
}

/* A socket address as the script's socket_hex prints it. */
static void print_socket_hex(api_socket_addr addr) {
	if (!addr.v6) {
		print_hex(addr.addr.v4.ip.octets, 4);
		printf(":%u", addr.addr.v4.port);
		return;
	}
	print_hex(addr.addr.v6.ip.octets, 16);
	printf(":%u:%u:%u", addr.addr.v6.port, addr.addr.v6.flowinfo, addr.addr.v6.scope_id);
}

/* An IP address's bytes in hex, so its length shows its family. */
static void print_ip_hex(api_ip_addr ip) {
	if (ip.v6) {
		print_hex(ip.ip.v6.octets, 16);
	} else {
		print_hex(ip.ip.v4.octets, 4);
	}
}

/* A bound, as Rust's Debug prints it. */
static void print_bound(api_bound_u64 bound) {
	switch (bound.kind) {
	case API_BOUND_INCLUDED:
		printf("Included(%llu)", (unsigned long long)bound.value);
		break;
	case API_BOUND_EXCLUDED:
		printf("Excluded(%llu)", (unsigned long long)bound.value);
		break;
	case API_BOUND_UNBOUNDED:
		printf("Unbounded");
		break;
	}
}

static int script(void) {
	printf("limit %llu\n", (unsigned long long)API_LIMIT);

	api_Point start = {.x = 1, .y = 2};
	api_Point p = api_Point_offset(&start, 2, 3);
	printf("offset %d %d\n", p.x, p.y);
	print("debug ", api_Point_debug(&p), "\n");
	api_Point expected = {.x = 3, .y = 5};
	printf("eq %s\n", api_Point_eq(&p, &expected) ? "true" : "false");
	api_Point d = api_Point_default();
	printf("default %d %d\n", d.x, d.y);

	api_Counter *c = api_Counter_new(str("c"), 5);
	printf("get %llu\n", (unsigned long long)api_Counter_get(c, NULL));
	print("name ", api_Counter_name(c, NULL), "\n");
	uint64_t v;
	api_Error err;
	if (api_Counter_add(c, 10, &v, &err, NULL)) {
		printf("add ok %llu\n", (unsigned long long)v);
	}
	if (!api_Counter_add(c, 2000, &v, &err, NULL)) {
		print("add err ", api_Error_to_string(&err), " ");
		print("", api_Error_debug(&err), "\n");
		api_Error_free(&err);
	}
	if (!api_Counter_sub(c, 20, &v, &err, NULL)) {
		printf("sub err underflow by %llu ", (unsigned long long)err.underflow.by);
		print("", api_Error_debug(&err), "\n");
		api_Error_free(&err);
	}
	api_Counter_rename(c, str("d"), NULL);
	print("name ", api_Counter_name(c, NULL), "\n");
	printf("is d %s\n", api_Counter_is(c, str("d"), NULL) ? "true" : "false");
	api_Counter *copy = api_Counter_clone(c, NULL);
	api_Counter_add(copy, 1, NULL, NULL, NULL);
	printf("clone %llu original %llu\n", (unsigned long long)api_Counter_get(copy, NULL),
		   (unsigned long long)api_Counter_get(c, NULL));
	api_Counter_free(copy);
	print("debug ", api_Counter_debug(c, NULL), "\n");
	api_Label label = api_Counter_into_label(c, (api_option_Point){.some = true, .value = p}, NULL);
	api_Counter_free(c);
	print("label ", api_Label_debug(&label), "\n");
	api_Label_free(&label);
	/* Empty text is not none: only a null pointer is. */
	api_Note empty = {.text = {.ptr = (const uint8_t *)"", .len = 0}};
	printf("note %s %s\n", api_has_text(empty) ? "true" : "false", api_has_text((api_Note){0}) ? "true" : "false");

	/* A tuple returns through one out pointer per element. */
	api_Counter *whole = api_Counter_new(str("s"), 7);
	api_Counter *below, *above;
	api_Counter_split(whole, 5, &below, &above, NULL);
	api_Counter_free(whole);
	print("split ", api_Counter_name(below, NULL), " ");
	printf("%llu ", (unsigned long long)api_Counter_get(below, NULL));
	print("", api_Counter_name(above, NULL), " ");
	printf("%llu\n", (unsigned long long)api_Counter_get(above, NULL));
	api_Counter_free(below);
	api_Counter_free(above);
	api_Counter *one = api_Counter_new(str("o"), 3);
	api_Counter *only;
	api_only(one, &only, NULL);
	api_Counter_free(one);
	print("only ", api_Counter_name(only, NULL), " ");
	printf("%llu\n", (unsigned long long)api_Counter_get(only, NULL));
	api_Counter_free(only);
	uint8_t w8;
	int16_t w16;
	uint32_t w32;
	uint64_t w64;
	api_widths(98305, &w8, &w16, &w32, &w64);
	printf("widths %u %d %u %llu\n", w8, w16, w32, (unsigned long long)w64);
	uint64_t divisors[] = {2, 0};
	for (int i = 0; i < 2; i++) {
		uint64_t q, r;
		if (api_divide(7, divisors[i], &q, &r, &err)) {
			printf("divide %llu %llu\n", (unsigned long long)q, (unsigned long long)r);
		} else {
			print("divide err ", api_Error_to_string(&err), "\n");
			api_Error_free(&err);
		}
	}
	/* A tuple inside another type is a struct, its fields _0, _1, and so on. */
	uint64_t spreads[] = {98305, 0};
	for (int i = 0; i < 2; i++) {
		api_option_tuple_u8_i16_u32_u64 spread = api_spread(spreads[i]);
		if (spread.some) {
			printf("spread %u %d %u %llu\n", spread.value._0, spread.value._1, spread.value._2,
				   (unsigned long long)spread.value._3);
		} else {
			printf("spread none\n");
		}
	}
	const char *needles[] = {"lo", "x"};
	for (int i = 0; i < 2; i++) {
		api_option_tuple_usize_usize found = api_find(str("hello"), str(needles[i]));
		if (found.some) {
			printf("find %zu %zu\n", found.value._0, found.value._1);
		} else {
			printf("find none\n");
		}
	}
	api_Point row_points[] = {{.x = 1, .y = 2}, {.x = 3, .y = 3}};
	api_vec_tuple_string_Shape_Point stat_rows = api_rows((api_vec_Point){.ptr = row_points, .len = 2});
	printf("rows");
	for (size_t i = 0; i < stat_rows.len; i++) {
		/* The array's free frees each element's buffer. */
		printf("%s%.*s ", i == 0 ? " " : ", ", (int)stat_rows.ptr[i]._0.len, (const char *)stat_rows.ptr[i]._0.ptr);
		print("", api_Shape_debug(&stat_rows.ptr[i]._1), " ");
		printf("%d %d", stat_rows.ptr[i]._2.x, stat_rows.ptr[i]._2.y);
	}
	printf("\n");
	api_vec_tuple_string_Shape_Point_free(stat_rows);
	/* Each handle in a tuple moves in, as one taken by value does. */
	api_tuple_string_Counter meters[] = {
		{._0 = str("low"), ._1 = api_Counter_new(str("l"), 1)},
		{._0 = str("high"), ._1 = api_Counter_new(str("h"), 9)},
	};
	api_Board *board = api_Board_new((api_vec_tuple_string_Counter){.ptr = meters, .len = 2}, NULL);
	api_Counter_free(meters[0]._1);
	api_Counter_free(meters[1]._1);
	api_vec_tuple_string_Counter ranked = api_Board_ranked(board, NULL);
	printf("ranked");
	for (size_t i = 0; i < ranked.len; i++) {
		printf("%s%.*s ", i == 0 ? " " : ", ", (int)ranked.ptr[i]._0.len, (const char *)ranked.ptr[i]._0.ptr);
		print("", api_Counter_name(ranked.ptr[i]._1, NULL), " ");
		printf("%llu", (unsigned long long)api_Counter_get(ranked.ptr[i]._1, NULL));
	}
	printf("\n");
	api_vec_tuple_string_Counter_free(ranked);
	api_Counter *best = api_Board_best(board, NULL);
	print("best ", api_Counter_name(best, NULL), " ");
	printf("%llu\n", (unsigned long long)api_Counter_get(best, NULL));
	api_Counter_free(best);
	api_Board_free(board);
	/* A map crosses as its pairs and a set as its items, which Rust collects in order, each
	 * key once, the last kept. */
	api_tuple_string_Point atlas_points[] = {
		{._0 = str("b"), ._1 = {.x = 1, .y = 1}},
		{._0 = str("ab"), ._1 = {.x = 2, .y = 3}},
		{._0 = str("b"), ._1 = {.x = 4, .y = 4}},
	};
	uint32_t atlas_ids[] = {3, 1, 3};
	api_Atlas loose = {
		.points = {.ptr = atlas_points, .len = 3},
		.ids = {.ptr = atlas_ids, .len = 3},
	};
	api_Atlas atlas = api_Atlas_with(&loose, str("c"), (api_Point){.x = 5, .y = 6});
	printf("atlas");
	for (size_t i = 0; i < atlas.points.len; i++) {
		printf("%s%.*s ", i == 0 ? " " : ", ", (int)atlas.points.ptr[i]._0.len, (const char *)atlas.points.ptr[i]._0.ptr);
		printf("%d %d", atlas.points.ptr[i]._1.x, atlas.points.ptr[i]._1.y);
	}
	printf("; ids");
	for (size_t i = 0; i < atlas.ids.len; i++) {
		printf(" %u", atlas.ids.ptr[i]);
	}
	printf("\n");
	api_Atlas_free(&atlas);
	/* Another crate's error crosses as its message. */
	const char *port_texts[] = {"80", "x"};
	for (size_t i = 0; i < 2; i++) {
		uint16_t port;
		api_buf port_err;
		if (api_port(str(port_texts[i]), &port, &port_err)) {
			printf("port %u\n", port);
		} else {
			print("port err ", port_err, "\n");
		}
	}
	api_Span span = {
		.bounds = {._0 = 2, ._1 = 10},
		.named = {.some = true, .value = {._0 = str("s"), ._1 = API_SHAPE_SQUARE}},
	};
	api_tuple_u32_u32 lower_half, upper_half;
	api_halves(span, &lower_half, &upper_half);
	printf("halves %u %u %u %u ", lower_half._0, lower_half._1, upper_half._0, upper_half._1);
	print("", api_Span_debug(&span), "\n");
	uint32_t swapped_n;
	api_buf swapped_text;
	api_swapped((api_tuple_string_u32){._0 = str("t"), ._1 = 4}, &swapped_n, &swapped_text);
	printf("swapped %u ", swapped_n);
	print("", swapped_text, "\n");
	api_option_tuple_option_u32 doubles[] = {
		{.some = false},
		{.some = true, .value = {._0 = {.some = false}}},
		{.some = true, .value = {._0 = {.some = true, .value = 3}}},
	};
	for (int i = 0; i < 3; i++) {
		api_option_tuple_option_u32 doubled = api_doubled(doubles[i]);
		if (!doubled.some) {
			printf("doubled None\n");
		} else if (!doubled.value._0.some) {
			printf("doubled Some((None,))\n");
		} else {
			printf("doubled Some((Some(%u),))\n", doubled.value._0.value);
		}
	}
	/* A borrowed or boxed tuple of one is a tuple struct, not its element. */
	uint8_t lent_bytes[] = {1, 2};
	uint8_t boxed_bytes[] = {3};
	api_tuple_bytes lent_one = api_lent_back((api_tuple_bytes){._0 = {.ptr = lent_bytes, .len = 2}});
	api_tuple_bytes boxed_one = api_boxed_tuple((api_buf){.ptr = boxed_bytes, .len = 1});
	api_buf both[] = {lent_one._0, boxed_one._0};
	for (int i = 0; i < 2; i++) {
		printf("%s[", i == 0 ? "lent back " : " boxed ");
		for (size_t j = 0; j < both[i].len; j++) {
			printf("%s%u", j == 0 ? "" : ", ", both[i].ptr[j]);
		}
		printf("]");
		api_buf_free(both[i]);
	}
	printf("\n");
	/* A borrowed string and an owned one share a tuple struct. */
	api_option_tuple_string formal = api_title(true);
	api_option_tuple_string informal = api_title(false);
	api_option_tuple_string named = api_named(str("x"));
	print("title Some((\"", formal.value._0, "\",)) ");
	printf("%s ", informal.some ? "Some" : "None");
	print("named Some((\"", named.value._0, "\",))\n");

	/* Each lent counter is a new handle to a clone. */
	/* A handle whose value moved in is still the caller's to free. */
	api_Counter *moved = api_Counter_new(str("first"), 1);
	api_Registry *registry = api_Registry_new(moved, NULL);
	api_Counter_free(moved);
	printf("last %s\n", api_Registry_last(registry, NULL) != NULL ? "true" : "false");
	moved = api_Counter_new(str("last"), 2);
	api_Registry_push(registry, moved, NULL);
	api_Counter_free(moved);
	api_Counter *first = api_Registry_first_get(registry, NULL);
	api_Counter_add(first, 10, NULL, NULL, NULL);
	api_Counter *lent = api_Registry_first_get(registry, NULL);
	printf("first %llu lent %llu\n", (unsigned long long)api_Counter_get(lent, NULL),
		   (unsigned long long)api_Counter_get(first, NULL));
	api_Counter_free(lent);
	api_Counter_free(first);
	api_Counter *last = api_Registry_last(registry, NULL);
	api_Counter_add(last, 10, NULL, NULL, NULL);
	lent = api_Registry_last(registry, NULL);
	printf("last %llu lent %llu\n", (unsigned long long)api_Counter_get(lent, NULL),
		   (unsigned long long)api_Counter_get(last, NULL));
	api_Counter_free(lent);
	api_Counter_free(last);
	uint64_t by = 5;
	bool edited = api_Registry_edit(registry, edit, &by, NULL);
	first = api_Registry_first_get(registry, NULL);
	printf("edit %s %llu\n", edited ? "true" : "false", (unsigned long long)api_Counter_get(first, NULL));
	api_Counter_free(first);
	/* A setter moves an object in, as assignment does, leaving the handle to be freed. */
	api_Counter *second = api_Counter_new(str("second"), 3);
	api_Registry_first_set(registry, second, NULL);
	api_Counter_free(second);
	first = api_Registry_first_get(registry, NULL);
	printf("set first %llu ", (unsigned long long)api_Counter_get(first, NULL));
	print("", api_Counter_name(first, NULL), "\n");
	api_Counter_free(first);
	printf("walk %llu\n", (unsigned long long)api_walk(3, visit, NULL));
	printf("each %u\n", api_each(2, print_visit, NULL));
	printf("offset %u\n", api_offset(seven, NULL, 3));
	printf("shift %u\n", api_shift(seven, NULL, 4));
	api_Registry_free(registry);

	/* A type generic only over lifetimes is a handle to its 'static form. */
	api_Name *name = api_Name_owned(str("a"));
	api_Name *b = api_Name_owned(str("b"));
	api_Name *joined = api_Name_join(name, b, NULL);
	api_Name_free(b);
	print("joined ", api_Name_as_str(joined, NULL), " ");
	print("", api_Name_debug(joined, NULL), " ");
	printf("%s\n", api_Name_eq(joined, name, NULL) ? "true" : "false");
	api_Name_free(name);
	char text[] = "c";
	api_Name_rename(joined, str(text), NULL);
	/* The name took a copy: the caller's buffer is its own again. */
	text[0] = 'x';
	print("renamed ", api_Name_as_str(joined, NULL), "\n");
	api_Name *dd = api_Name_owned(str("dd"));
	api_Name *longest = api_longer(joined, dd, NULL);
	print("longer ", api_Name_as_str(longest, NULL), "\n");
	api_Name_free(longest);
	api_Name_free(dd);
	api_Name_free(joined);
	api_Name *unnamed = api_unnamed();
	print("unnamed [", api_Name_as_str(unnamed, NULL), "]\n");
	api_Name_free(unnamed);
	/* Two buffers of equal text, interned into one string. */
	char lit_text[] = "lit", again_text[] = "lit";
	api_Name *lit = api_Name_borrowed(str(lit_text));
	api_Name *again = api_Name_borrowed(str(again_text));
	api_Name *other = api_Name_borrowed(str("other"));
	print("borrowed ", api_Name_as_str(lit, NULL), " ");
	printf("%s %s\n", api_Name_shares(lit, again, NULL) ? "true" : "false", api_Name_shares(lit, other, NULL) ? "true" : "false");
	api_Name_free(lit);
	api_Name_free(again);
	api_Name_free(other);

	printf("total %llu\n", (unsigned long long)api_total((api_range_u64){.start = 2, .end = 5}));
	printf("reach %u\n", (unsigned)api_reach(2, 5));
	api_option_u128 scaled[] = {
		api_scaled((api_u128){.lo = UINT64_MAX, .hi = UINT64_MAX}, 1),
		api_scaled((api_u128){.lo = 1, .hi = 1}, 1ULL << 63),
		api_scaled((api_u128){.hi = 1ULL << 63}, 2),
	};
	printf("scaled");
	for (size_t i = 0; i < 3; i++) {
		if (scaled[i].some) {
			printf(" %016llx%016llx", (unsigned long long)scaled[i].value.hi, (unsigned long long)scaled[i].value.lo);
		} else {
			printf(" none");
		}
	}
	printf("\n");
	api_array_u8_4 sum, unparsed;
	api_buf unparsed_err;
	if (!api_fingerprint(str("deadbeef"), &sum, &unparsed_err) || api_fingerprint(str("zz"), &unparsed, &unparsed_err)) {
		return 1;
	}
	api_Digest *digest = api_Digest_new();
	api_Digest_sum_set(digest, sum, NULL);
	api_array_u8_4 got = api_Digest_sum_get(digest, NULL);
	api_array_u8_4 masked = api_Digest_flipped(digest, (api_array_u8_4){{0xff, 0, 0xff, 0}}, NULL);
	printf("digest %02x%02x%02x%02x %02x%02x%02x%02x", got.items[0], got.items[1], got.items[2], got.items[3],
		masked.items[0], masked.items[1], masked.items[2], masked.items[3]);
	print(" ", unparsed_err, "\n");
	api_Digest_free(digest);
	api_Batch *batch = api_Batch_new();
	unsigned accepted = 0;
	for (uint32_t v = 1; v <= 5; v++) {
		accepted += api_Batch_push(batch, v, NULL);
	}
	api_Batch_2 *batch2 = api_pair();
	unsigned paired = 0;
	for (uint32_t v = 1; v <= 3; v++) {
		paired += api_Batch_2_push(batch2, v, NULL);
	}
	unsigned long long emptied = api_drain_4(batch, NULL);
	printf("batch %zu %u %llu %llu %zu %u %llu\n", api_Batch_capacity(batch, NULL), accepted, emptied,
		(unsigned long long)api_drain_4(batch, NULL), api_Batch_2_capacity(batch2, NULL), paired, (unsigned long long)api_drain_2(batch2, NULL));
	printf("capacities %zu %zu\n", api_capacities_4_2(batch, batch2, NULL), api_capacities_2_2(batch2, batch2, NULL));
	api_Batch_2_free(batch2);
	api_Batch_free(batch);
	api_range_u64 wide = api_widen((api_range_u64){.start = 2, .end = 5}, 1);
	printf("widen %llu %llu\n", (unsigned long long)wide.start, (unsigned long long)wide.end);
	api_bounds_u64 ranges[] = {
		{.start = {API_BOUND_INCLUDED, 2}, .end = {API_BOUND_EXCLUDED, 5}},
		{.start = {API_BOUND_INCLUDED, 2}, .end = {API_BOUND_INCLUDED, 5}},
		{.start = {API_BOUND_INCLUDED, 3}},
		{.start = {API_BOUND_UNBOUNDED, 0}},
		{.start = {API_BOUND_EXCLUDED, 2}, .end = {API_BOUND_INCLUDED, 4}},
	};
	printf("count");
	for (int i = 0; i < 5; i++) {
		printf(" %zu", api_count(ranges[i]));
	}
	printf("\n");
	printf("end ");
	print_bound(api_end(3));
	printf(" ");
	print_bound(api_end(0));
	printf("\n");
	api_bound_u64 ends_start, ends_end;
	api_ends(3, &ends_start, &ends_end);
	printf("ends ");
	print_bound(ends_start);
	printf(" ");
	print_bound(ends_end);
	printf("\n");
	uint64_t belows[] = {2, 5};
	for (int i = 0; i < 2; i++) {
		uint64_t n;
		api_bounds_u64 outside;
		if (api_below(belows[i], 3, &n, &outside)) {
			printf("below %llu\n", (unsigned long long)n);
		} else {
			printf("below err ");
			print_bound(outside.start);
			printf(" ");
			print_bound(outside.end);
			printf("\n");
		}
	}
	api_Window window = {.range = {.start = 2, .end = 8}, .end = api_end(5)};
	printf("window %s %s ", api_Window_contains(&window, 4) ? "true" : "false",
		   api_Window_contains(&window, 6) ? "true" : "false");
	print("", api_Window_debug(&window), "\n");

	print("", api_greet(str("x"), API_SHAPE_SQUARE), "\n");
	printf("len %zu\n", api_len(str("abc")));
	api_buf binary = {.ptr = (const uint8_t *)"\xff\0a", .len = 3};
	print("hex ", api_hex(binary), "\n");
	print("timed hex ", api_timed_hex(binary), "\n");
	print("sample hex ", api_sample_hex(binary), "\n");
	print("queued hex ", api_queued_hex(binary), "\n");
	print("tick ", api_tick_name((api_Label){.name = str("tick")}), "\n");
	print("rooted ", api_rooted_name((api_Label){.name = str("rooted")}), "\n");
	print("stored ", api_stored_text(str("stored")), "\n");
	print("layer ", api_layer_name((api_Label){.name = str("layer")}), "\n");
	api_Event events[3] = {
		{.tag = API_EVENT_MOVED, .moved = {._0 = p}},
		{.tag = API_EVENT_NAMED, .named = {.name = str("n")}},
		{.tag = API_EVENT_DONE},
	};
	for (int i = 0; i < 3; i++) {
		print("", api_describe(events[i]), "\n");
	}

	/* A `Copy` value passed by value is copied, so it stays usable, and frees nothing. */
	api_Stamp stamp = api_Stamp_from_millis(2500);
	printf("secs %llu\n", (unsigned long long)api_Stamp_secs(&stamp));
	api_Stamp later = api_Stamp_from_millis(3000);
	printf("later %s\n", api_Stamp_later(&later, stamp) ? "true" : "false");
	printf("secs %llu\n", (unsigned long long)api_Stamp_secs(&stamp));
	api_Stamp rounded = api_Stamp_whole(&stamp);
	print("whole ", api_Stamp_debug(&rounded), "\n");
	api_Message *message = api_Message_new((api_option_Stamp){.some = true, .value = stamp}, str("body"));
	print("payload ", api_Message_payload_get(message, NULL), "\n");
	api_option_Stamp read = api_Message_stamp_get(message, NULL);
	print("stamp ", api_Stamp_debug(&read.value), "\n");
	printf("sent %s\n", api_Message_sent(message, NULL) ? "true" : "false");
	/* Setters assign in place: the bytes are copied in, and a `Copy` value copied. */
	api_Stamp reset = api_Stamp_from_millis(4000);
	api_Message_stamp_set(message, (api_option_Stamp){.some = true, .value = reset}, NULL);
	api_Message_payload_set(message, str("edited"), NULL);
	print("payload ", api_Message_payload_get(message, NULL), "\n");
	read = api_Message_stamp_get(message, NULL);
	print("stamp ", api_Stamp_debug(&read.value), "\n");
	printf("secs %llu\n", (unsigned long long)api_Stamp_secs(&reset));
	api_Message_stamp_set(message, (api_option_Stamp){0}, NULL);
	printf("stamp %s\n", api_Message_stamp_get(message, NULL).some ? "true" : "false");
	api_Message_free(message);

	/* A value past a word, copied across: a closure takes and returns one by value. */
	api_option_Moment made = api_Moment_new(1500, 1000);
	api_Moment moment = made.value;
	printf("moment %llu %s\n", (unsigned long long)api_Moment_millis(&moment),
		   api_Moment_new(1, 0).some ? "false" : "true");
	api_Moment shifted = api_shifted(moment, moment_after, NULL);
	api_Moment later_moment = api_Moment_after(&moment, 500);
	printf("shifted %llu %s\n", (unsigned long long)api_Moment_millis(&shifted),
		   api_Moment_eq(&shifted, &later_moment) ? "true" : "false");
	api_Moment wrapped = api_Moment_after(&moment, UINT64_MAX);
	printf("wrapped %llu %s\n", (unsigned long long)api_Moment_millis(&wrapped),
		   api_Moment_eq(&moment, &shifted) ? "true" : "false");

	/* A value with padding, or bytes that vary, is a handle, as no table says where padding is. */
	api_Gap *gap = api_Gap_new(7, 3);
	api_Gap *same_gap = api_Gap_new(7, 3);
	printf("gap %llu %s\n", (unsigned long long)api_Gap_sum(gap), api_Gap_eq(gap, same_gap) ? "true" : "false");
	api_Gap_free(same_gap);
	api_Gap_free(gap);
	api_Berth *berth = api_Berth_new((api_option_u32){.some = true, .value = 4});
	api_Berth *empty_berth = api_Berth_new((api_option_u32){0});
	api_option_u32 berth_at = api_Berth_at(berth);
	printf("berth Some(%u) %s\n", berth_at.value, api_Berth_at(empty_berth).some ? "Some" : "None");
	api_Berth_free(empty_berth);
	api_Berth_free(berth);

	/* A `Copy` type changed in place is a value, whose change reaches only the copy changed. */
	api_Score score = api_Score_of(2);
	api_Score step = api_Score_of(3);
	api_Score_add(&score, step);
	api_Score_add(&score, step);
	api_Score doubled_score = api_Score_doubled(&step);
	printf("score %llu %llu %llu\n", (unsigned long long)api_Score_count(&score),
		   (unsigned long long)api_Score_count(&doubled_score), (unsigned long long)api_Score_count(&step));
	api_Score score_copy = score;
	uint64_t taken = api_Score_take(&score_copy);
	printf("taken %llu %llu %llu\n", (unsigned long long)taken, (unsigned long long)api_Score_count(&score_copy),
		   (unsigned long long)api_Score_count(&score));
	api_Score other_score = step;
	uint64_t before0, before1;
	api_Score_absorb(&score_copy, other_score, &other_score, &before0, &before1);
	printf("absorbed %llu %llu %llu %llu %llu\n", (unsigned long long)before0, (unsigned long long)before1,
		   (unsigned long long)api_Score_count(&score_copy), (unsigned long long)api_Score_count(&other_score),
		   (unsigned long long)api_Score_count(&step));
	api_Hits hits = api_Hits_of(1, 2);
	api_Hits kept = hits;
	api_Hits_add(&hits, api_Hits_of(10, 20));
	api_Hits_hits_set(&hits, 7);
	printf("hits %llu %llu %llu %llu %s\n", (unsigned long long)api_Hits_hits_get(&hits),
		   (unsigned long long)api_Hits_misses_get(&hits), (unsigned long long)api_Hits_hits_get(&kept),
		   (unsigned long long)api_Hits_misses_get(&kept),
		   api_Hits_eq(&hits, &kept) ? "true" : "false");
	api_Hits counted = kept;
	bool counted_same = api_Hits_eq_(&counted, kept);
	printf("counted %s %llu\n", counted_same ? "true" : "false", (unsigned long long)api_Hits_hits_get(&counted));
	api_Extent extent = api_Extent_new(5, 3);
	api_Extent extent_start = extent;
	uint64_t old = api_Extent_slide(&extent, 10);
	api_Extent_start_set(&extent, api_Extent_start_get(&extent) + 1);
	printf("extent %llu %llu %llu %llu %llu\n", (unsigned long long)old,
		   (unsigned long long)api_Extent_start_get(&extent), (unsigned long long)api_Extent_end(&extent),
		   (unsigned long long)api_Extent_start_get(&extent_start), (unsigned long long)api_Extent_end(&extent_start));
	api_Weighted weighted = {.score = step, .weight = 4};
	printf("weighted %llu %llu\n", (unsigned long long)api_Weighted_total(&weighted),
		   (unsigned long long)(api_Score_count(&weighted.score) + api_Score_count(&step)));
	api_Weighted_free(&weighted);
	/* A checked mark beside bytes in a tuple and in a record lent, and bytes beside a move. */
	api_Mark *mark = api_Mark_of(1);
	api_Mark_add(mark, 2, NULL);
	api_Counter *hefted = api_Counter_new(str("w"), 1);
	printf("heft %llu\n", (unsigned long long)api_heft((api_tuple_Mark_bytes){mark, str("ab")},
							 (api_Parcel){.mark = mark, .payload = str("abc")},
							 (api_tuple_Counter_bytes){hefted, str("abcd")}, NULL));
	api_Counter_free(hefted);
	/* A value's `&mut self` beside a checked handle. */
	api_Score noted = api_Score_of(2);
	api_Mark *noted_by = api_Mark_of(5);
	uint64_t noted_before = api_Score_note(&noted, noted_by, NULL);
	printf("noted %llu %llu\n", (unsigned long long)noted_before, (unsigned long long)api_Score_count(&noted));
	api_Mark_free(noted_by);
	api_Mark_free(mark);

	static const api_Log_vtable print_log = {.free = log_free, .log = log_line};
	uint8_t levels[] = {1, 2, 3, 4};
	api_buf lines = {.ptr = levels, .len = sizeof levels};
	printf("kept %u\n", api_emit((api_Log){.vtable = &print_log}, lines));
	api_Logger *logger = api_Logger_new((api_Log){.vtable = &print_log});
	printf("write %s\n", api_Logger_write(logger, str("hi")) ? "true" : "false");
	api_Logger_free(logger);
	printf("dropped\n");
	uint8_t threshold = 2;
	printf("filtered %u\n", api_filter(lines, keep_line, &threshold));

	api_Channel *channel = api_Channel_new();
	api_waker waker = {.wake = wake, .drop = drop, .data = NULL};
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
		return 1;
	}
	api_Channel_clear(channel, NULL);
	/* The waiter a pending poll registers lives until the next poll, so the send wakes it. */
	take = api_Channel_poll_take(channel, NULL);
	if (api_Channel_poll_take_poll(take, waker, &payload) != API_POLL_PENDING) {
		return 1;
	}
	woken = wakes;
	api_Channel_send(channel, str("later"), NULL);
	if (wakes != woken + 1 || api_Channel_poll_take_poll(take, waker, &payload) != API_POLL_READY) {
		return 1;
	}
	api_buf_free(payload);
	api_Channel_poll_take_future_free(take);
	api_Channel_free(channel);

	/* A value that holds itself is an object. */
	api_Node *list = api_Node_new(1, NULL, NULL);
	api_Node_push(list, 2, NULL);
	printf("node %llu\n", (unsigned long long)api_Node_sum(list, NULL));
	api_Node_free(list);

	api_Tally *tally = api_Tally_new();
	api_Tally_bump(tally, NULL);
	uint32_t count = api_Tally_bump(tally, NULL);
	printf("tally %u %s\n", count, api_Tally_eq(tally, tally, NULL) ? "true" : "false");
	printf("both %u\n", api_bump_both(tally, tally, NULL));
	printf("peek %u\n", api_peek(tally, peek_tally, NULL, NULL));
	api_Tally_free(tally);

	/* Another export's types, which this export's function takes and returns. */
	api_Area *small = api_Area_new((api_Size){.width = 2, .height = 3});
	api_Area *area = api_grow(small, 1);
	api_Area_free(small);
	api_Size size = api_Area_size(area);
	printf("area %u %u %llu\n", size.width, size.height, (unsigned long long)api_Area_pixels(area));
	api_Area_free(area);

	/* Generic types, each at the instances it infers, named by their arguments. */
	api_Pair_u32 pair = {.left = 2, .right = 5};
	api_Pair_u32 swapped = api_Pair_u32_swap(&pair);
	printf("pair %u %u sum %u ", swapped.left, swapped.right, api_Pair_u32_sum(&pair));
	print("", api_Pair_u32_debug(&pair), " ");
	printf("%s\n", api_Pair_u32_eq(&pair, &swapped) ? "true" : "false");
	api_Pair_u32 twins = {.left = 7, .right = 7};
	printf("even %s %s\n", api_Pair_u32_even(&pair) ? "true" : "false",
	       api_Pair_u32_even(&twins) ? "true" : "false");
	/* An instance renamed alone, beside another of `Pair` that keeps its name. */
	api_Pair_Size_u8 sizes = api_tag((api_Size){.width = 1, .height = 2}, 3);
	api_Pair_u8_Size flipped = api_Pair_Size_u8_swap(&sizes);
	printf("sized %u %u ", flipped.left, flipped.right.width);
	print("", api_Pair_u8_Size_debug(&flipped), "\n");
	api_Pair_u32 byte = {.left = 258, .right = 9};
	api_Pair_u8_Size beside = api_Pair_u32_beside_Size(&byte, sizes.left);
	printf("beside %u ", beside.left);
	print("", api_Pair_u8_Size_debug(&beside), "\n");
	/* At its default, named as the type itself. */
	api_Stack *stack = api_Stack_new();
	const char *names[] = {"a", "b"};
	for (int i = 0; i < 2; i++) {
		api_Counter *pushed = api_Counter_new(str(names[i]), (uint64_t)i + 1);
		api_Stack_push(stack, pushed, NULL);
		api_Counter_free(pushed);
	}
	api_Counter *top = api_Stack_pop(stack, NULL);
	printf("stack %zu Some(%llu)", api_Stack_depth(stack, NULL), (unsigned long long)api_Counter_get(top, NULL));
	api_Counter_free(top);
	api_Counter *next = api_Stack_pop(stack, NULL);
	printf(" %s", next != NULL ? "true" : "false");
	api_Counter_free(next);
	printf(" %s\n", api_Stack_pop(stack, NULL) == NULL ? "true" : "false");
	/* Generic functions and methods, at each implementation of the trait bounding them, by
	   reference, which leaves C its handle. */
	api_Counter *from_point = api_Counter_seeded_Point(str("p"), (api_Point){.x = -3, .y = 4});
	api_Counter *from_size = api_Counter_seeded_Size(str("s"), (api_Size){.width = 5, .height = 6});
	api_Counter *from_counter = api_Counter_seeded_Counter(str("c"), from_point, NULL);
	/* Two instances whose arguments share a last segment: one takes its module too. */
	api_Counter *from_polar = api_Counter_seeded_polar_Point(str("q"), (api_polar_Point){.turns = 1, .distance = 2});
	printf("seeded %llu %llu %llu %llu past %llu seeds %llu\n", (unsigned long long)api_Counter_get(from_point, NULL),
	       (unsigned long long)api_Counter_get(from_size, NULL), (unsigned long long)api_Counter_get(from_counter, NULL),
	       (unsigned long long)api_Counter_get(from_polar, NULL),
	       (unsigned long long)api_Stack_depth_past_Point(stack, (api_Point){.x = 1, .y = 2}, NULL),
	       (unsigned long long)api_seeds_Point_Size((api_Size){.width = 2, .height = 3}));
	api_Counter_free(from_point);
	api_Counter_free(from_size);
	api_Counter_free(from_counter);
	api_Counter_free(from_polar);
	api_Stack_free(stack);

	/* A trait implemented for `Point` and `&Point` apart: one instance. */
	printf("weigh %u\n", api_weigh_Point((api_Point){.x = -1, .y = 2}));
	/* A function named as the type it returns, and one named as a type it does not. */
	printf("polar %u %s area %llu\n", api_polar_point(2).value.distance, api_polar_point(0).some ? "false" : "true",
	       (unsigned long long)api_area((api_Size){.width = 2, .height = 3}));
	printf("wind %u\n", api_wind((api_PolarTurn){.whole = 2}, (api_polar_Turn){.part = 3}));
	/* Names that clash in some language, each kept apart there. */
	api_clash_Tone tone = {.hz = 440};
	api_clash_TONE loud = {.db = 3};
	api_Cancelled cancelled = {.step = 2};
	print("clash ", api_clash_Tone_debug(&tone), " ");
	print("", api_clash_TONE_debug(&loud), " ");
	print("", api_Cancelled_debug(&cancelled), "");
	api_clash_Span clash_span = {.width = 4};
	printf(" span %u ", api_clash_Span_width(&clash_span));
	print("", api_clash_Span_to_string_(&clash_span), " ");
	print("", api_clash_Span_to_string(&clash_span), "");
	api_clash_Store *clash_store = api_clash_Store_new(7);
	printf(" store %u %u", api_clash_Store_free_(clash_store), api_clash_Store_close(clash_store));
	api_clash_Store_free(clash_store);
	api_option_tuple_tuple_u8_u16_u32 clash_pair = {.some = true, .value = {._0 = {._0 = 1, ._1 = 2}, ._1 = 3}};
	api_option_tuple_tuple_u8_u16_u32_ clash_triple = {.some = true, .value = {._0 = {._0 = 1, ._1 = 2, ._2 = 3}}};
	printf(" audio %u %u nested %u\n", api_clash_audio_encoder_finish(), api_clash_audio_encoder_finish_(),
	       api_clash_nested(clash_pair, clash_triple));
	/* Instances that name each other, and one made only by an impl of it. */
	api_Outer_Inner_u8 *outer = api_Outer_Inner_u8_new(4);
	printf("cross %u outer %u\n",
	       api_cross((api_Left_Right_u8){.value = {.value = 1}}, (api_Right_Left_u8){.value = {.value = 2}}),
	       api_Outer_Inner_u8_get(outer));
	api_Outer_Inner_u8_free(outer);
	/* A bound that outlives the call takes a value, and an instance only a constant names. */
	printf("kept %llu boxed %u\n", (unsigned long long)api_kept_Point((api_Point){.x = 1, .y = -2}), api_BOXED().value);
	printf("keep %d\n", api_keep_Point((api_Point){.x = 1, .y = -2}));
	api_Point five = {.x = 5, .y = 2};
	printf("kept or %d %d keeper %d\n", api_kept_or_Point((api_option_Point){.some = true, .value = five}),
		api_kept_or_Point((api_option_Point){0}), api_keeper_Point(five));
	/* Inferred instances taking a lent closure and a foreign implementation, and one only
	 * its impl of a trait names. */
	api_Pair_u32 sides = {.left = 2, .right = 5};
	printf("apply %u\n", api_apply_u32(sides, triple, NULL));
	printf("report %u\n", api_report_u32(sides, (api_Log){.vtable = &print_log}));
	printf("wrapped %llu\n", (unsigned long long)api_kept_Wrap_u32((api_Wrap_u32){.value = 4}));
	printf("keeps %u\n", api_keeps((api_Log){.vtable = &print_log}).value);

	/* Its Drop runs when C frees the handle, not before. */
	api_Token *token = api_Token_new(7);
	printf("token %u dropped %u\n", api_Token_id_get(token, NULL), api_dropped());
	api_Token_free(token);
	printf("dropped %u\n", api_dropped());

	/* A field and the method of its name, each its own function. */
	api_Retry *retry = api_Retry_new((api_option_u32){.some = true, .value = 5});
	api_option_u32 limit = api_Retry_limit_get(retry, NULL);
	printf("retry %d %u %u %u\n", limit.some, limit.value, api_Retry_limit(retry, NULL), api_Retry_delay(retry, NULL));
	/* The renamed field's setter, named as its getter. */
	api_Retry_limit_set(retry, (api_option_u32){0}, NULL);
	printf("retry %u\n", api_Retry_limit(retry, NULL));
	api_Retry_free(retry);
	api_Slot_u8 *slot = api_Slot_u8_new(200);
	printf("slot %u\n", api_Slot_u8_value(slot));
	api_Slot_u8_free(slot);
	/* Foreign types: a URL parsed from text, socket addresses, OS paths, and `Vec`s. */
	api_Url *base;
	api_buf parse_err;
	if (!api_Url_parse(str("https://example.com/a/b"), &base, &parse_err)) {
		return 1;
	}
	print("url ", api_Url_to_string(base, NULL), " host ");
	api_buf host = api_host(base, NULL);
	if (host.ptr != NULL) {
		print("Some(\"", host, "\")\n");
	} else {
		printf("None\n");
	}
	api_Url *resolved;
	api_buf resolve_err;
	api_Url *base_clone = api_Url_clone(base, NULL);
	if (api_resolve(base_clone, str("c?d"), &resolved, &resolve_err, NULL)) {
		print("resolve ", api_Url_to_string(resolved, NULL), " ");
		printf("%s\n", api_Url_eq(resolved, base, NULL) ? "true" : "false");
		api_Url_free(resolved);
	} else {
		print("resolve err ", resolve_err, "\n");
	}
	api_Url_free(base_clone);
	api_Url_free(base);
	api_Url *bad;
	if (api_Url_parse(str("no scheme"), &bad, &parse_err)) {
		print("parse ", api_Url_to_string(bad, NULL), "\n");
		api_Url_free(bad);
	} else {
		print("parse err ", parse_err, "\n");
	}

	api_socket_addr addrs[2] = {
		{.addr.v4 = {.ip = {{127, 0, 0, 1}}, .port = 8080}},
		{.v6 = true, .addr.v6 = {.ip = {{[15] = 1}}, .port = 443, .scope_id = 3}},
	};
	api_vec_u16 ports = api_ports((api_vec_socket_addr){.ptr = addrs, .len = 2});
	printf("ports [%u, %u]\n", ports.ptr[0], ports.ptr[1]);
	api_vec_u16_free(ports);
	api_buf peer_names[] = {str("a"), str("b")};
	api_Peer peer_start = {.addr = addrs[1], .names = {.ptr = peer_names, .len = 2}};
	api_Peer peer = api_Peer_next(&peer_start, str("c"));
	printf("peer Peer { addr: ");
	print_addr(peer.addr);
	printf(", names: [");
	for (size_t i = 0; i < peer.names.len; i++) {
		printf("%s\"%.*s\"", i == 0 ? "" : ", ", (int)peer.names.ptr[i].len, (const char *)peer.names.ptr[i].ptr);
	}
	printf("] } [");
	api_vec_string aliases = api_Peer_aliases(&peer);
	for (size_t i = 0; i < aliases.len; i++) {
		printf("%s\"%.*s\"", i == 0 ? "" : ", ", (int)aliases.ptr[i].len, (const char *)aliases.ptr[i].ptr);
	}
	printf("]\n");
	api_vec_string_free(aliases);
	api_buf debug = api_Peer_debug(&peer);
	api_Peer_free(&peer);
	api_buf_free(debug);

	/* A path's bytes as the OS holds them, which need not be UTF-8. */
	api_buf stray = api_stray();
	/* A second owner of the buffer goes to Rust, which takes it, so stray stays ours. */
	api_buf in_stray = api_file(api_buf_clone(stray), str("f"));
	if (in_stray.len != 3 || memcmp(in_stray.ptr, "\xff/f", 3) != 0) {
		return 1;
	}
	api_buf_free(stray);
	api_buf_free(in_stray);
	api_buf file = api_file(str("dir"), str("f\\'\"\t"));
	api_buf paths[] = {file, str("/")};
	api_vec_option_path parents = api_parents((api_vec_path){.ptr = paths, .len = 2});
	print_escaped("file ", file, "\n");
	printf("parents");
	for (size_t i = 0; i < parents.len; i++) {
		if (parents.ptr[i].ptr != NULL) {
			print_escaped(" ", parents.ptr[i], "");
		} else {
			printf(" none");
		}
	}
	printf("\n");
	/* Each element taken: the array alone is freed. */
	parents.len = 0;
	api_vec_option_path_free(parents);

	api_vec_string words = api_words(str(" one two  three "));
	printf("words %zu [", words.len);
	for (size_t i = 0; i < words.len; i++) {
		printf("%s\"%.*s\"", i == 0 ? "" : ", ", (int)words.ptr[i].len, (const char *)words.ptr[i].ptr);
	}
	printf("]\n");
	print("sentence ", api_sentence(words), " ");
	print("", api_sentence((api_vec_string){0}), "\n");
	api_vec_string_free(words);
	/* An empty array that is not None: its pointer is not null. */
	api_buf no_words[1];
	print("sentence ", api_sentence((api_vec_string){.ptr = no_words, .len = 0}), "|\n");
	/* An optional borrow: a null buffer or handle is None. */
	printf("either %zu %zu\n", api_either(str("abc")), api_either((api_buf){0}));
	print("prefixed ", api_prefixed(str("cd"), str("ab")), " ");
	print("", api_prefixed(str("cd"), (api_buf){0}), "\n");
	api_Point corner = {.x = 2, .y = 3};
	printf("coordinates %d %d\n", api_coordinates((api_option_Point){.some = true, .value = corner}),
		api_coordinates((api_option_Point){0}));
	api_Counter *two = api_Counter_new(str("two"), 2);
	api_Counter *three = api_Counter_new(str("three"), 3);
	printf("plus %llu %llu\n", (unsigned long long)api_Counter_plus(two, three, NULL),
		(unsigned long long)api_Counter_plus(two, NULL, NULL));
	bool added = api_Counter_add_to(two, three, NULL);
	bool added_none = api_Counter_add_to(two, NULL, NULL);
	printf("add_to %s %s %llu\n", added ? "true" : "false", added_none ? "true" : "false",
		(unsigned long long)api_Counter_get(three, NULL));
	api_Counter_free(two);
	api_Counter_free(three);

	/* A value passed for &mut: Rust writes back what the call left, which the caller frees.
	 * A view passed stays the caller's, and a buffer Rust returned Rust takes back. */
	api_buf encoded, appended;
	api_put_number(1, str("0,"), &encoded);
	api_put_number(23, encoded, &appended);
	print("put_number ", appended, "\n");
	api_tuple_bytes_u32 stamped;
	uint32_t stamps = api_push_counted((api_tuple_bytes_u32){._0 = str("ab"), ._1 = 1}, 'c', &stamped);
	printf("push_counted %u ", stamps);
	print("", stamped._0, " ");
	printf("%u\n", stamped._1);
	api_buf digits_left, digits_rest;
	uint8_t first_digit, last_digit;
	api_Error digit_error;
	api_digit(str("7x9"), &digits_left, &first_digit, &digit_error);
	/* The failed call took a byte too. Each buffer Rust returned it takes back. */
	bool skipped = !api_digit(digits_left, &digits_rest, &last_digit, &digit_error);
	api_digit(digits_rest, &digits_left, &last_digit, &digit_error);
	printf("digit %u %s %u %zu\n", first_digit, skipped ? "true" : "false", last_digit, digits_left.len);
	api_buf_free(digits_left);
	api_Point nudged = {.x = 1, .y = 2};
	api_nudge(nudged, 3, &nudged);
	printf("nudge %d %d\n", nudged.x, nudged.y);

	/* An Arc of another crate's type: an opaque handle, taken back by value or lent. */
	api_tls_Config *config = api_secure(str("h3"));
	print("secure ", api_alpn_lent(config, NULL), " ");
	api_tls_Config *config_copy = api_tls_Config_clone(config, NULL);
	print("", api_alpn(config_copy, NULL), "\n");
	api_tls_Config_free(config_copy);
	api_tls_Config_free(config);
	/* Another crate's value by value: an opaque handle, lent or taken back. */
	api_tls_Cert *cert = api_certify(str("der"));
	printf("cert %llu ", (unsigned long long)api_cert_len(cert, NULL));
	print("", api_cert_der(cert, NULL), " ");
	api_tls_Cert_free(cert);
	api_tls_Cert *chain[2] = {api_certify(str("a")), api_certify(str("bc"))};
	printf("%llu\n", (unsigned long long)api_chain_len((api_vec_tls_Cert){.ptr = chain, .len = 2}, NULL));
	api_tls_Cert_free(chain[0]);
	api_tls_Cert_free(chain[1]);
	/* Another crate's trait objects: an `Arc`, taken back, and `&'static` ones, held by value. */
	api_tls_Resolver *resolver = api_resolver(str("r1"));
	print("resolver ", api_resolved(resolver, NULL), "\n");
	api_tls_Resolver_free(resolver);
	api_Codecs *codecs = api_Codecs_new();
	printf("codecs %u\n", api_Codecs_ids(codecs, NULL));
	api_vec_tls_Codec listed = api_Codecs_list_get(codecs, NULL);
	api_tls_Codec *codecs_reversed[2] = {listed.ptr[1], listed.ptr[0]};
	api_Codecs_list_set(codecs, (api_vec_tls_Codec){.ptr = codecs_reversed, .len = 2}, NULL);
	/* The setter took each handle's value: the array, and the handles, are still ours to free. */
	api_vec_tls_Codec_free(listed);
	printf("codecs reversed %u\n", api_Codecs_ids(codecs, NULL));
	api_Codecs_free(codecs);
	/* A standard library value no type maps, in a handle. */
	api_std_io_Error *failure = api_failure(str("boom"));
	print("failure ", api_failure_message(failure, NULL), " ");
	print("", api_failure_kind(failure, NULL), "\n");
	api_std_io_Error_free(failure);
	/* Iterators, passed as the arrays of their items. */
	const uint32_t summed_items[] = {1, 2, 3};
	printf("sum_all %u\n", api_sum_all((api_vec_u32){.ptr = summed_items, .len = 3}));
	/* Each counter moves in, leaving its handle only to free. */
	api_Counter *weighed = api_Counter_new(str("w"), 3);
	printf("weigh_all %llu\n", (unsigned long long)api_weigh_all_Counter((api_vec_Counter){.ptr = &weighed, .len = 1}, NULL));
	api_Counter_free(weighed);
	const api_buf alpns[] = {str("h3"), str("moq-00")};
	printf("offered_len %zu\n", api_offered_len((api_vec_bytes){.ptr = alpns, .len = 2}));
	const api_buf joined_words[] = {str("a"), str("b"), str("c")};
	print("join_words ", api_join_words((api_vec_string){.ptr = joined_words, .len = 3}), "\n");
	/* Bytes that must be `'static`, interned. */
	for (int i = 0; i < 2; i++) {
		api_Der *der = api_Der_from_slice(str("der"));
		print(i == 0 ? "der " : "der again ", api_Der_bytes(der), "\n");
		api_Der_free(der);
	}
	/* Another crate's type only signatures name, exported by its path there, and its error,
	 * which crosses as its message. */
	api_tls_Session *session;
	api_buf session_err;
	if (!api_resume(str("h3"), 60, &session, &session_err)) {
		return 1;
	}
	print("resume ", api_tls_Session_alpn(session), " ");
	printf("%u\n", api_tls_Session_lifetime(session).seconds);
	api_tls_Session_free(session);
	if (api_resume(str(""), 60, &session, &session_err)) {
		return 1;
	}
	print("resume err ", session_err, "\n");
	/* A borrow of a meter that is not Clone, lent from its owner's handle. */
	{
		api_Meters *lender = api_Meters_new();
		api_Meter *first = api_Meters_first(lender, NULL);
		api_Meter_add(first, 2, NULL);
		unsigned long long added = api_Meter_add(first, 3, NULL);
		api_Meter *again = api_Meters_first(lender, NULL);
		printf("lent %llu %llu\n", added, (unsigned long long)api_Meter_count(again, NULL));
		api_Meter_free(again);
		api_Meter_free(first);
		api_Meters_clear(lender, NULL);
		again = api_Meters_first(lender, NULL);
		printf("cleared %llu\n", (unsigned long long)api_Meter_count(again, NULL));
		api_Meter_free(again);
		/* A field that is not Clone, lent as a method's borrowed return is. */
		api_Meter *last = api_Meters_last_get(lender, NULL);
		api_Meter_add(last, 4, NULL);
		printf("field lent %llu\n", (unsigned long long)api_Meter_add(last, 1, NULL));
		api_Meter_free(last);
		/* An optional field that is not Clone, lent as `Some` of a handle. */
		api_Meter *spare = api_Meters_spare_get(lender, NULL);
		printf("spare %s\n", spare != NULL ? "true" : "false");
		api_Meter *fresh = api_Meter_default();
		api_Meters_spare_set(lender, fresh, NULL);
		api_Meter_free(fresh);
		spare = api_Meters_spare_get(lender, NULL);
		api_Meter_add(spare, 5, NULL);
		printf("spare lent %llu\n", (unsigned long long)api_Meter_add(spare, 1, NULL));
		api_Meter_free(spare);
		api_Meters_free(lender);
		/* One lent from a handle that is not Sync, on one thread at a time. */
		api_Tallies *tallies = api_Tallies_new();
		api_Tally *tallied = api_Tallies_first(tallies, NULL);
		api_Tally_bump(tallied, NULL);
		api_Tally_free(tallied);
		tallied = api_Tallies_first(tallies, NULL);
		printf("tallied %u\n", api_Tally_bump(tallied, NULL));
		api_Tally_free(tallied);
		api_Tallies_free(tallies);
	}
	/* A draft borrowing its ledger exclusively until it drops, through which its sheet's
	 * members are reached by Deref and DerefMut. */
	{
		api_Ledger *ledger = api_Ledger_new();
		api_Draft *draft;
		api_Error err;
		api_Ledger_edit(ledger, &draft, &err, NULL);
		api_Draft_title_set(draft, str("minutes"), NULL);
		api_Draft_write(draft, str("first"), NULL);
		print("draft ", api_Draft_title_get(draft, NULL), " ");
		printf("%u ", api_Draft_lines(draft, NULL));
		print("", api_Draft_describe(draft, NULL), " ");
		printf("%s\n", api_Draft_edited(draft, NULL) ? "true" : "false");
		printf("committed %u\n", api_Draft_commit(draft, NULL));
		api_Draft_free(draft);
		print("ledger ", api_Ledger_title(ledger, NULL), " ");
		printf("%u\n", api_Ledger_published(ledger, NULL));
		api_Ledger_edit(ledger, &draft, &err, NULL);
		printf("read %u %s\n", api_Draft_lines(draft, NULL), api_Draft_edited(draft, NULL) ? "true" : "false");
		api_Draft_free(draft);
		api_Ledger_edit(ledger, &draft, &err, NULL);
		api_Draft_write(draft, str("second"), NULL);
		api_Draft_free(draft);
		printf("dropped %u\n", api_Ledger_published(ledger, NULL));
		api_Receipt *receipt = api_Ledger_receipt(ledger, NULL);
		printf("receipt %u\n", api_Receipt_count(receipt, NULL));
		api_Receipt_free(receipt);
		api_Ledger_seal(ledger, NULL);
		if (!api_Ledger_edit(ledger, &draft, &err, NULL)) {
			print("sealed ", api_Error_to_string(&err), "\n");
			api_Error_free(&err);
		}
		api_Ledger_free(ledger);
	}

	api_vec_bytes chunks = api_chunks(str("abcde"), 2);
	printf("chunks [");
	for (size_t i = 0; i < chunks.len; i++) {
		printf("%s[", i == 0 ? "" : ", ");
		for (size_t j = 0; j < chunks.ptr[i].len; j++) {
			printf("%s%u", j == 0 ? "" : ", ", chunks.ptr[i].ptr[j]);
		}
		printf("]");
	}
	printf("]\n");
	api_vec_bytes_free(chunks);

	api_buf counter_names[] = {str("x"), str("y")};
	api_vec_Counter counters = api_counters((api_vec_string){.ptr = counter_names, .len = 2});
	printf("counters");
	for (size_t i = 0; i < counters.len; i++) {
		print(i == 0 ? " " : ", ", api_Counter_name(counters.ptr[i], NULL), " ");
		printf("%llu", (unsigned long long)api_Counter_get(counters.ptr[i], NULL));
	}
	printf("\n");
	/* Each counter moves in, leaving its handle only to free. */
	uint64_t worth = api_worth((api_vec_Counter){.ptr = counters.ptr, .len = 1}, (api_vec_Counter){.ptr = counters.ptr + 1, .len = 1}, NULL);
	uint64_t none = api_worth((api_vec_Counter){0}, (api_vec_Counter){.ptr = counters.ptr, .len = 0}, NULL);
	printf("worth %llu %llu\n", (unsigned long long)worth, (unsigned long long)none);
	api_vec_Counter_free(counters);
	api_Counter *held = api_Counter_new(str("h"), 4);
	api_option_u64 counts[] = {
		api_held_count((api_option_tuple_Counter){.some = true, .value = {._0 = held}}, NULL),
		api_held_count((api_option_tuple_Counter){0}, NULL),
	};
	api_Counter_free(held);
	printf("held");
	for (size_t i = 0; i < 2; i++) {
		if (counts[i].some) {
			printf(" Some(%llu)", (unsigned long long)counts[i].value);
		} else {
			printf(" None");
		}
	}
	printf("\n");
	api_Counter *boxed = api_boxed_counter(str("b"));
	print("boxed Some(\"", api_boxed_name(boxed, NULL), "\") ");
	api_Counter_free(boxed);
	printf("%s %s\n", api_boxed_name(NULL, NULL).ptr ? "Some" : "None", api_boxed_counter((api_buf){0}) ? "false" : "true");
	printf("less %u\n", api_less(5, 2));

	uint32_t row0[] = {1, 2};
	uint32_t row2[] = {3};
	api_vec_u32 rows[] = {{.ptr = row0, .len = 2}, {.ptr = row0, .len = 0}, {.ptr = row2, .len = 1}};
	api_vec_u32 sums = api_sums((api_vec_vec_u32){.ptr = rows, .len = 3});
	printf("sums [%u, %u, %u]\n", sums.ptr[0], sums.ptr[1], sums.ptr[2]);
	api_vec_u32_free(sums);
	uint32_t set0[] = {3, 1};
	uint32_t set2[] = {2};
	api_vec_u32 sets[] = {{.ptr = set0, .len = 2}, {.ptr = set0, .len = 0}, {.ptr = set2, .len = 1}};
	api_vec_vec_u32 least = api_least((api_vec_vec_u32){.ptr = sets, .len = 3});
	printf("least");
	for (size_t i = 0; i < least.len; i++) {
		printf(" [");
		for (size_t j = 0; j < least.ptr[i].len; j++) {
			printf(j ? ", %u" : "%u", least.ptr[i].ptr[j]);
		}
		printf("]");
	}
	printf("\n");
	api_vec_vec_u32_free(least);
	api_Numbers edges = api_Numbers_edges();
	numbers("edges", &edges);
	api_Numbers reversed = api_Numbers_reversed(&edges);
	numbers("reversed", &reversed);
	api_Numbers others = api_Numbers_edges();
	api_Numbers negative = api_Numbers_zero(true);
	api_Numbers positive = api_Numbers_zero(false);
	api_Measure nan = api_Measure_new(NAN, false);
	api_Measure under = api_Measure_new(-0.0, true);
	api_Measure over = api_Measure_new(0.0, false);
	printf("float equal %s %s %s %s %s\n", api_Numbers_eq(&edges, &edges) ? "true" : "false",
	       api_Numbers_eq(&edges, &others) ? "true" : "false", api_Numbers_eq(&negative, &positive) ? "true" : "false",
	       api_Measure_eq(&nan, &nan) ? "true" : "false", api_Measure_eq(&under, &over) ? "true" : "false");
	api_Numbers_free(&edges);
	api_Numbers_free(&reversed);
	api_Numbers_free(&others);
	api_Numbers_free(&negative);
	api_Numbers_free(&positive);
	api_Measure_free(&nan);
	api_Measure_free(&under);
	api_Measure_free(&over);
	uint32_t totals[] = {UINT32_MAX, UINT32_MAX, 2};
	printf("sum u32 %llu %llu\n", (unsigned long long)api_sum_u32((api_vec_u32){0}),
	       (unsigned long long)api_sum_u32((api_vec_u32){.ptr = totals, .len = 3}));
	int64_t doubling[] = {INT64_MIN, -1, 0, INT64_MAX};
	api_vec_i64 doubled = api_doubled_i64((api_vec_i64){.ptr = doubling, .len = 4});
	printf("doubled i64 ");
	PRINT_LIST(doubled, "%lld", (long long));
	printf("\n");
	api_vec_i64_free(doubled);
	uint64_t nan_bits = 0x7ff8000000000001;
	double floats[2] = {-0.0, 0};
	memcpy(&floats[1], &nan_bits, sizeof nan_bits);
	api_vec_f64 boxed_floats = api_boxed_floats((api_vec_f64){.ptr = floats, .len = 2});
	printf("boxed floats ");
	PRINT_LIST(boxed_floats, "%llu", f64_bits);
	printf("\n");
	api_vec_f64_free(boxed_floats);
	api_Point summands[] = {{.x = 1, .y = 2}, {.x = -4, .y = 6}};
	api_Point summed = api_summed((api_vec_Point){.ptr = summands, .len = 2});
	api_Point none_summed = api_summed((api_vec_Point){.ptr = summands, .len = 0});
	printf("summed %d %d %d %d\n", summed.x, summed.y, none_summed.x, none_summed.y);
	api_buf longest_words[] = {str("a"), str("abc")};
	size_t most = api_longest((api_vec_string){.ptr = longest_words, .len = 2});
	size_t no_longest = api_longest((api_vec_string){.ptr = longest_words, .len = 0});
	printf("longest %zu %zu\n", most, no_longest);
	api_buf no_first = api_first_of((api_vec_string){.ptr = longest_words, .len = 0});
	print("first of Some(\"", api_first_of((api_vec_string){.ptr = longest_words, .len = 2}), "\") ");
	printf("%s\n", no_first.ptr == NULL ? "None" : "Some");
	api_Limits_bytes limits = api_byte_limits(str("a"), str("bc"));
	api_duration breadth = api_breadth((api_Limits_duration){.low = {.secs = 0, .nanos = 5000000}, .high = {.secs = 2, .nanos = 0}});
	printf("limits %.*s %.*s breadth %llu\n", (int)limits.low.len, (const char *)limits.low.ptr, (int)limits.high.len,
		(const char *)limits.high.ptr, (unsigned long long)(breadth.secs * 1000 + breadth.nanos / 1000000));
	api_Limits_bytes_free(&limits);
	api_buf parts[] = {str("ab"), str(""), str("c")};
	print("joined ", api_joined((api_vec_bytes){.ptr = parts, .len = 3}), " ");
	api_buf no_parts = api_joined((api_vec_bytes){.ptr = parts, .len = 0});
	printf("%zu read %llu\n", no_parts.len, (unsigned long long)api_read_len(str("hello")));
	api_buf_free(no_parts);

	/* JSON crosses as a document, made from text and read back as text. */
	api_Json *doc;
	if (!api_Json_parse(str("{\"b\": [1, 2.5, null], \"a\": \"\\u00e9\"}"), &doc, &parse_err)) {
		return 1;
	}
	print("json ", api_encode(doc, NULL), "\n");
	print("timed text ", api_timed_text(doc, NULL), "\n");
	api_Json *tagged = api_tagged(doc, str("t"), NULL);
	api_Json_free(doc);
	print("tagged ", api_Json_to_text(tagged, NULL), "\n");
	api_Json_free(tagged);
	/* A JSON object's entries cross as pairs of a key and its document, each moved in. */
	api_tuple_string_Json json_pairs[2] = {{._0 = str("b")}, {._0 = str("a")}};
	if (!api_Json_parse(str("[1]"), &json_pairs[0]._1, &parse_err) ||
	    !api_Json_parse(str("2"), &json_pairs[1]._1, &parse_err)) {
		return 1;
	}
	api_vec_tuple_string_Json json_entries = api_entries((api_vec_tuple_string_Json){.ptr = json_pairs, .len = 2}, str("t"), NULL);
	api_Json_free(json_pairs[0]._1);
	api_Json_free(json_pairs[1]._1);
	printf("entries");
	for (size_t i = 0; i < json_entries.len; i++) {
		printf("%s%.*s ", i == 0 ? " " : ", ", (int)json_entries.ptr[i]._0.len, (const char *)json_entries.ptr[i]._0.ptr);
		print("", api_Json_to_text(json_entries.ptr[i]._1, NULL), "");
	}
	printf("\n");
	api_vec_tuple_string_Json_free(json_entries);
	api_Json *broken;
	if (api_Json_parse(str("{\"a\":"), &broken, &parse_err)) {
		print("json parse ", api_Json_to_text(broken, NULL), "\n");
		api_Json_free(broken);
	} else {
		print("json parse err ", parse_err, "\n");
	}

	api_Url *home;
	if (!api_Url_parse(str("https://example.com/a/b"), &home, &parse_err)) {
		return 1;
	}
	api_Site *site = api_Site_new(home, NULL);
	api_Url *home0 = api_Site_home_get(site, NULL);
	api_Url *home1 = api_Site_home_get(site, NULL);
	print("site ", api_Url_to_string(home0, NULL), " ");
	print("", api_Url_to_string(home1, NULL), "\n");
	api_Url_free(home0);
	api_Url_free(home1);
	api_Site_free(site);
	api_Url_free(home);

	/* Slices, passed in and returned as clones, and the socket address of each family. */
	api_vec_Shape shapes = api_Shape_ALL();
	printf("shapes [");
	for (size_t i = 0; i < shapes.len; i++) {
		print(i == 0 ? "" : ", ", api_Shape_debug(&shapes.ptr[i]), "");
	}
	api_vec_Shape_free(shapes);
	api_vec_string greetings = api_GREETINGS();
	printf("] greetings [");
	for (size_t i = 0; i < greetings.len; i++) {
		printf("%s\"%.*s\"", i == 0 ? "" : ", ", (int)greetings.ptr[i].len, (const char *)greetings.ptr[i].ptr);
	}
	printf("]\n");
	api_vec_string_free(greetings);
	api_vec_Shape shapes_both = api_Shape_BOTH();
	printf("both [");
	for (size_t i = 0; i < shapes_both.len; i++) {
		print(i == 0 ? "" : ", ", api_Shape_debug(&shapes_both.ptr[i]), "");
	}
	api_vec_Shape_free(shapes_both);
	api_vec_u32 primes = api_PRIMES();
	printf("] primes [");
	for (size_t i = 0; i < primes.len; i++) {
		printf("%s%u", i == 0 ? "" : ", ", primes.ptr[i]);
	}
	printf("]\n");
	api_vec_u32_free(primes);
	api_Point trail_points[] = {{.x = 1, .y = 2}, {.x = 3, .y = 4}};
	api_buf trail_names[] = {str("a"), str("b")};
	api_Trail *trail =
		api_Trail_new((api_vec_Point){.ptr = trail_points, .len = 2}, (api_vec_string){.ptr = trail_names, .len = 2});
	api_vec_Point points = api_Trail_points(trail, NULL);
	printf("trail [");
	for (size_t i = 0; i < points.len; i++) {
		print(i == 0 ? "" : ", ", api_Point_debug(&points.ptr[i]), "");
	}
	api_vec_Point_free(points);
	api_vec_Label labels = api_Trail_labels(trail, NULL);
	printf("] [");
	for (size_t i = 0; i < labels.len; i++) {
		print(i == 0 ? "" : ", ", api_Label_debug(&labels.ptr[i]), "");
	}
	printf("]\n");
	api_vec_Label_free(labels);
	/* Returned iterators, each collected at the call: drained labels are gone after. */
	api_vec_Point iter_points = api_Trail_iter(trail, NULL);
	printf("iterated [");
	for (size_t i = 0; i < iter_points.len; i++) {
		print(i == 0 ? "" : ", ", api_Point_debug(&iter_points.ptr[i]), "");
	}
	api_vec_Point_free(iter_points);
	api_vec_string iter_names = api_Trail_names(trail, NULL);
	printf("] [");
	for (size_t i = 0; i < iter_names.len; i++) {
		printf("%s\"%.*s\"", i == 0 ? "" : ", ", (int)iter_names.ptr[i].len, (const char *)iter_names.ptr[i].ptr);
	}
	api_vec_string_free(iter_names);
	api_buf iter_bytes = api_Trail_bytes(trail, NULL);
	printf("] [");
	for (size_t i = 0; i < iter_bytes.len; i++) {
		printf("%s%u", i == 0 ? "" : ", ", iter_bytes.ptr[i]);
	}
	printf("]\n");
	api_buf_free(iter_bytes);
	api_vec_Label drained = api_Trail_drain(trail, NULL);
	printf("drained [");
	for (size_t i = 0; i < drained.len; i++) {
		print(i == 0 ? "" : ", ", api_Label_debug(&drained.ptr[i]), "");
	}
	api_vec_Label_free(drained);
	api_vec_Label left = api_Trail_labels(trail, NULL);
	printf("] then [");
	for (size_t i = 0; i < left.len; i++) {
		print(i == 0 ? "" : ", ", api_Label_debug(&left.ptr[i]), "");
	}
	printf("]\n");
	api_vec_Label_free(left);
	/* A filter, collected at the call. */
	api_vec_Point beyond = api_Trail_beyond(trail, 1, NULL);
	printf("beyond");
	for (size_t i = 0; i < beyond.len; i++) {
		printf("%s%d %d", i == 0 ? " " : ", ", beyond.ptr[i].x, beyond.ptr[i].y);
	}
	printf("\n");
	api_vec_Point_free(beyond);
	api_Trail_free(trail);
	api_buf numbered_names[] = {str("p"), str("q")};
	api_vec_Counter numbered = api_numbered((api_vec_string){.ptr = numbered_names, .len = 2});
	printf("numbered");
	for (size_t i = 0; i < numbered.len; i++) {
		print(i == 0 ? " " : ", ", api_Counter_name(numbered.ptr[i], NULL), " ");
		printf("%llu", (unsigned long long)api_Counter_get(numbered.ptr[i], NULL));
	}
	printf("\n");
	api_vec_Counter_free(numbered);
	api_socket_addr_v4 at4 = {.ip = {{127, 0, 0, 1}}, .port = 8080};
	api_socket_addr_v6 at6 = {.ip = {{[15] = 1}}, .port = 443, .scope_id = 3};
	api_socket_addr_v4 next4;
	api_socket_addr_v6 next6;
	api_nexts(at4, at6, &next4, &next6);
	const uint8_t *n4 = next4.ip.octets;
	printf("nexts %u.%u.%u.%u %u %u %u\n", n4[0], n4[1], n4[2], n4[3], next4.port, next6.port, next6.scope_id);
	api_ipv4_addr ip4 = {{10, 0, 0, 255}}, nextip4;
	api_ipv6_addr ip6 = {{[14] = 0xff, [15] = 0xff}}, nextip6;
	api_next_ips(ip4, ip6, &nextip4, &nextip6);
	api_ip_addr mapped = api_flip((api_ip_addr){.ip.v4 = {{127, 0, 0, 1}}});
	api_ip_addr ips[] = {
		{.ip.v4 = nextip4},
		{.v6 = true, .ip.v6 = nextip6},
		mapped,
		api_flip(mapped),
		api_flip((api_ip_addr){.v6 = true, .ip.v6 = {{[15] = 1}}}),
	};
	printf("ips");
	for (size_t i = 0; i < sizeof ips / sizeof ips[0]; i++) {
		printf(" ");
		print_ip_hex(ips[i]);
	}
	printf("\n");

	/* Lossless round trips: each type's edge values, passed through Rust and back. */
	api_duration durations[] = {{0, 0}, {0, 1}, {9223372036u, 854775808u}, {UINT64_MAX, 999999999u}};
	printf("lossless duration");
	for (size_t i = 0; i < sizeof durations / sizeof durations[0]; i++) {
		api_duration d = api_same_duration(durations[i]);
		printf(" %llu.%09u", (unsigned long long)d.secs, d.nanos);
	}
	printf("\n");
	printf("lossless u64 %llu %llu %llu\n", (unsigned long long)api_same_u64(0),
		   (unsigned long long)api_same_u64(1ull << 63), (unsigned long long)api_same_u64(UINT64_MAX));
	printf("lossless usize %zu %zu\n", api_same_usize(0), api_same_usize(SIZE_MAX));
	printf("lossless isize %td %td\n", api_same_isize(PTRDIFF_MIN), api_same_isize(PTRDIFF_MAX));
	api_u128 u128s[] = {{0, 0}, {0, 1}, {UINT64_MAX, UINT64_MAX}};
	printf("lossless u128");
	for (size_t i = 0; i < sizeof u128s / sizeof u128s[0]; i++) {
		api_u128 v = api_same_u128(u128s[i]);
		printf(" %016llx%016llx", (unsigned long long)v.hi, (unsigned long long)v.lo);
	}
	printf("\n");
	uint32_t chars[] = {0, 0xd7ff, 0xe000, 0x10ffff};
	printf("lossless char");
	for (size_t i = 0; i < sizeof chars / sizeof chars[0]; i++) {
		printf(" %x", api_same_char(chars[i]));
	}
	printf("\n");
	api_buf same_paths[] = {str("a/\xc3\xa9"), {.ptr = (const uint8_t *)"\xff", .len = 1}};
	printf("lossless path");
	for (size_t i = 0; i < sizeof same_paths / sizeof same_paths[0]; i++) {
		api_buf path = api_same_path(same_paths[i]);
		printf(" ");
		print_hex(path.ptr, path.len);
		api_buf_free(path);
	}
	printf("\n");
	api_socket_addr_v4 sv4 = api_same_socket_addr_v4((api_socket_addr_v4){.ip = {{255, 255, 255, 255}}, .port = UINT16_MAX});
	api_socket_addr_v6 sv6 = api_same_socket_addr_v6(
		(api_socket_addr_v6){.ip = {{0, 1, [15] = 2}}, .port = 3, .flowinfo = UINT32_MAX, .scope_id = UINT32_MAX});
	api_socket_addr sockets[] = {{.addr.v4 = sv4}, {.v6 = true, .addr.v6 = sv6}};
	printf("lossless socket");
	for (size_t i = 0; i < 2; i++) {
		printf(" ");
		print_socket_hex(sockets[i]);
	}
	for (size_t i = 0; i < 2; i++) {
		printf(" ");
		print_socket_hex(api_same_socket_addr(sockets[i]));
	}
	printf("\n");
	api_Key *key_a = api_Key_new(str("a")), *key_a2 = api_Key_new(str("a")), *key_b = api_Key_new(str("b"));
	if (api_Key_hash(key_a) != api_Key_hash_of(key_a)) {
		fprintf(stderr, "api_Key_hash differs from Rust's own hash\n");
		abort();
	}
	printf("key %s %s\n", api_Key_hash(key_a) == api_Key_hash(key_a2) ? "true" : "false",
		   api_Key_hash(key_a) == api_Key_hash(key_b) ? "true" : "false");
	api_Key_free(key_a);
	api_Key_free(key_a2);
	api_Key_free(key_b);
	printf("lossless ip ");
	print_ip_hex(api_same_ip_addr((api_ip_addr){.ip.v4 = {{255, 255, 255, 255}}}));
	printf(" ");
	print_ip_hex(api_same_ip_addr((api_ip_addr){.v6 = true, .ip.v6 = {{0xff, 0xff, [15] = 1}}}));
	printf("\n");

	/* serde_json's map iterator, collected as pairs of a key and its document. */
	api_tuple_string_Json section_pairs[2] = {{._0 = str("b")}, {._0 = str("a")}};
	if (!api_Json_parse(str("[1]"), &section_pairs[0]._1, &parse_err) ||
	    !api_Json_parse(str("2"), &section_pairs[1]._1, &parse_err)) {
		return 1;
	}
	api_vec_tuple_string_Json sections = api_sections((api_vec_tuple_string_Json){.ptr = section_pairs, .len = 2}, NULL);
	api_Json_free(section_pairs[0]._1);
	api_Json_free(section_pairs[1]._1);
	printf("sections");
	for (size_t i = 0; i < sections.len; i++) {
		printf("%s%.*s ", i == 0 ? " " : ", ", (int)sections.ptr[i]._0.len, (const char *)sections.ptr[i]._0.ptr);
		print("", api_Json_to_text(sections.ptr[i]._1, NULL), "");
	}
	printf("\n");
	api_vec_tuple_string_Json_free(sections);

	/* An instant only Rust makes, a handle, as its padding is wherever rustc puts it, which
	 * each call copies out of, and compares. */
	api_Instant *began = api_now();
	api_Instant *ended = api_later(began, (api_duration){.secs = 1, .nanos = 500000000});
	api_Instant *same = api_later(began, (api_duration){.secs = 0, .nanos = 0});
	api_duration forward = api_since(ended, began);
	api_duration backward = api_since(began, ended);
	printf("instants %llu %llu %s %s\n", (unsigned long long)(forward.secs * 1000 + forward.nanos / 1000000),
		   (unsigned long long)(backward.secs * 1000 + backward.nanos / 1000000), api_Instant_eq(began, same) ? "true" : "false",
		   api_Instant_eq(began, ended) ? "true" : "false");
	/* An instant lent inside an `Option`, which takes no handle. */
	printf("has_instant %s %s\n", api_has_instant(began) ? "true" : "false", api_has_instant(NULL) ? "true" : "false");
	api_Instant_free(same);
	api_Instant_free(ended);
	api_Instant_free(began);

	/* tracing's level, and its filter as an optional level whose none is `OFF`. */
	api_Verbosity *verbosity = api_Verbosity_new(API_LEVEL_WARN);
	print("verbosity ", api_spell(api_Verbosity_filter(verbosity, NULL)), " ");
	print("", api_spell(api_louder(api_Verbosity_filter(verbosity, NULL))), "\n");
	api_Verbosity_level_set(verbosity, API_LEVEL_TRACE, NULL);
	printf("verbosity %s ", api_Verbosity_level_get(verbosity, NULL) == API_LEVEL_TRACE ? "true" : "false");
	print("", api_spell(api_Verbosity_filter(verbosity, NULL)), " ");
	print("", api_spell(api_louder(api_Verbosity_filter(verbosity, NULL))), " ");
	print("", api_spell(api_louder((api_option_Level){.some = false})), "\n");
	api_Verbosity_free(verbosity);

	/* A returned `Vec<&Token>` holds clones, which its free drops. */
	api_Token *twice = api_Token_new(9);
	uint32_t before = api_dropped();
	api_vec_Token tokens = api_Token_twice(twice, NULL);
	if (tokens.len != 2 || api_Token_id_get(tokens.ptr[1], NULL) != 9) {
		return 1;
	}
	api_vec_Token_free(tokens);
	if (api_dropped() != before + 2) {
		return 1;
	}
	api_Token_free(twice);

	/* Each future dropped its waker once it finished, and Rust the one it was given. */
	if (drops != 15) {
		return 1;
	}
	return closures() && foreign_futures() && forwarding() && started_futures() ? 0 : 1;
}

/* Each breaks a contract, which must abort with a message naming the function. */
/* A counter a closure was lent, which it fails to move out, then reads as it was. */
static bool move_lent_status(void *data, api_Counter *counter) {
	api_status *status = data;
	api_Counter_into_label(counter, (api_option_Point){0}, status);
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
	api_Local_poll_count_poll(e->count, (api_waker){0}, &out, &e->counted);
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
		api_Ledger_edit(ledger, &draft, &err, NULL);
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
		api_Ledger_edit(ledger, &draft, &err, NULL);
		api_Draft_write(draft, str("y"), NULL);
		status = 0;
		api_Draft_drop(draft, &status);
		if (status != 0 || api_Ledger_published(ledger, &status) != 2 || status != 0) {
			fprintf(stderr, "a ledger after a close wrote %d\n", status);
			failed = 1;
		}
		api_Draft_free(draft);
		/* The ledger freed while lent drops once the draft does, which still works. */
		api_Ledger_edit(ledger, &draft, &err, NULL);
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
		api_Slip *slip = api_Ledger_slip(ledger, NULL);
		api_Slip *twin = api_Slip_clone(slip, NULL);
		api_Slip_free(slip);
		status = 0;
		api_Slip_receipt_get(twin, &status);
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
		/* A method hides no field: `Pad<u16>`'s `title` beside its sheet's. */
		api_buf titled = api_Pad_u16_title_get(wide);
		if (api_Pad_u16_title(wide) != 2 || titled.len != 0) {
			fprintf(stderr, "a pad's title is %u and its sheet's %.*s\n", api_Pad_u16_title(wide), (int)titled.len,
			        (const char *)titled.ptr);
			failed = 1;
		}
		api_buf_free(titled);
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
		api_greet(bad, API_SHAPE_CIRCLE);
	} else if (strcmp(which, "finished") == 0) {
		api_Channel *channel = api_Channel_new();
		api_Channel_send(channel, str("x"), NULL);
		api_Channel_recv_future *recv = api_Channel_recv(channel, NULL);
		api_waker waker = {0};
		api_buf payload;
		api_Channel_recv_poll(recv, waker, &payload);
		api_buf_free(payload);
		api_Channel_recv_poll(recv, waker, &payload);
	} else if (strcmp(which, "enum") == 0) {
		api_greet(str("x"), (api_Shape)2);
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
		api_made(1, made_view, NULL);
	} else if (strcmp(which, "reader") == 0) {
		api_buf bytes;
		uint64_t at;
		api_buf err;
		api_tail((api_foreign_reader){0}, 1, &bytes, &at, &err);
	} else if (strcmp(which, "bound") == 0) {
		api_count((api_bounds_u64){.start = {.kind = (api_bound)3}});
	} else if (strcmp(which, "future") == 0) {
		/* A future with no poll. */
		api_after((api_foreign_future){0}, 1);
	} else if (strcmp(which, "started") == 0) {
		/* A future with no poll, through a start, which the abort names. */
		uint32_t out;
		api_after_future *after;
		api_after_start((api_foreign_future){0}, 1, &out, &after);
	} else if (strcmp(which, "start") == 0) {
		/* A start with nowhere to put the handle of a future that pends. */
		uint32_t out;
		api_yielded_start(1, &out, NULL);
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
	return script();
}
