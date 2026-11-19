/* Runs a script tests/protocol/model.rs writes through the api fixture's C ABI: each step
 * on the main thread or a worker, printing what it returned a line, then the counting
 * waker's wakes and drops once every step ran and the worker exited. The model's
 * transcript must match it line for line. */

#include "api.h"

#include <pthread.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_SLOTS 4096
#define MAX_LINE 128

static void *slots[MAX_SLOTS];
static char kinds[MAX_SLOTS][32];

static atomic_ulong wakes;
static atomic_ulong drops;

static void wake(void *data) {
	(void)data;
	atomic_fetch_add(&wakes, 1);
}

static void drop_waker(void *data) {
	(void)data;
	atomic_fetch_add(&drops, 1);
}

/* One waker for every poll, so a future polled again keeps the one it adopted. */
static const api_waker waker = {wake, drop_waker, NULL};

static api_buf view(const char *s) {
	api_buf buf = {(const uint8_t *)s, strlen(s), NULL};
	return buf;
}

static const char *status_name(api_status status) {
	switch (status) {
	case API_STATUS_MOVED: return "moved";
	case API_STATUS_EXCLUSIVE: return "exclusive";
	case API_STATUS_BORROWED: return "borrowed";
	case API_STATUS_THREAD: return "thread";
	case API_STATUS_LENT: return "lent";
	case API_STATUS_OUTLIVED: return "outlived";
	case API_STATUS_NOT_SEND: return "not_send";
	default: return "unknown";
	}
}

struct step {
	int thread;
	char op[32];
	/* Its slots and numbers, -1 for an absent handle. */
	long long args[3];
	char out[64];
};

static void *at(long long slot) {
	return slot < 0 ? NULL : slots[slot];
}

#define VALUE(v) snprintf(s->out, sizeof s->out, "%llu", (unsigned long long)(v))
#define OP(name) (strcmp(s->op, name) == 0)

/* Each slot's type's _free and _drop, by its kind's name. */
#define OBJECT(T) \
	if (strcmp(kind, #T) == 0) { \
		if (drop) api_##T##_drop(slots[h], status); \
		else api_##T##_free(slots[h]); \
		return; \
	}

static void release(long long h, int drop, api_status *status) {
	const char *kind = kinds[h];
	OBJECT(Counter)
	OBJECT(Ledger)
	OBJECT(Draft)
	OBJECT(Receipt)
	OBJECT(Tally)
	OBJECT(Local)
	OBJECT(Meters)
	OBJECT(Meter)
	if (strcmp(kind, "Counter_plus_later") == 0) api_Counter_plus_later_future_free(slots[h]);
	else if (strcmp(kind, "Tally_bump_later") == 0) api_Tally_bump_later_future_free(slots[h]);
	else if (strcmp(kind, "Local_later") == 0) api_Local_later_future_free(slots[h]);
	else {
		fprintf(stderr, "unknown kind %s\n", kind);
		abort();
	}
}

static void poll(struct step *s, api_status *status) {
	long long f = s->args[0];
	const char *kind = kinds[f];
	api_poll polled;
	uint64_t out = 0;
	uint32_t out32 = 0;
	if (strcmp(kind, "Counter_plus_later") == 0) {
		polled = api_Counter_plus_later_poll(slots[f], waker, &out);
	} else if (strcmp(kind, "Tally_bump_later") == 0) {
		polled = api_Tally_bump_later_poll(slots[f], waker, &out32, status);
		out = out32;
	} else {
		polled = api_Local_later_poll(slots[f], waker, &out32, status);
		out = out32;
	}
	if (*status) return;
	if (polled == API_POLL_PENDING) snprintf(s->out, sizeof s->out, "pending");
	else VALUE(out);
}

/* Run one step on this thread, writing what it returned. */
static void run(struct step *s) {
	api_status status = 0;
	long long a = s->args[0], b = s->args[1], c = s->args[2];
	snprintf(s->out, sizeof s->out, "ok");
	void *made = NULL;
	if (OP("counter_new")) slots[a] = api_Counter_new(view("c"), (uint64_t)b);
	else if (OP("counter_get")) VALUE(api_Counter_get(at(a), &status));
	else if (OP("counter_add") || OP("counter_into_sum")) {
		if (OP("counter_into_sum")) {
			uint64_t v = api_Counter_into_sum(at(a), (uint64_t)b, &status);
			VALUE(v);
		} else {
			uint64_t out = 0;
			api_Error err;
			bool ok = api_Counter_add(at(a), (uint64_t)b, &out, &err, &status);
			if (ok) VALUE(out);
			else if (!status) {
				api_Error_free(&err);
				snprintf(s->out, sizeof s->out, "error");
			}
		}
	} else if (OP("counter_plus")) VALUE(api_Counter_plus(at(a), at(b), &status));
	else if (OP("counter_add_to")) VALUE(api_Counter_add_to(at(a), at(b), &status));
	else if (OP("counter_clone")) made = api_Counter_clone(at(b), &status);
	else if (OP("plus_later")) made = api_Counter_plus_later(at(b), at(c), &status);
	else if (OP("ledger_new")) slots[a] = api_Ledger_new();
	else if (OP("ledger_edit")) {
		api_Draft *draft = NULL;
		api_Error err;
		bool ok = api_Ledger_edit(at(b), &draft, &err, &status);
		if (ok) {
			made = draft;
			VALUE(1);
		} else if (!status) {
			api_Error_free(&err);
			snprintf(s->out, sizeof s->out, "error");
		}
	} else if (OP("ledger_published")) VALUE(api_Ledger_published(at(a), &status));
	else if (OP("ledger_seal")) api_Ledger_seal(at(a), &status);
	else if (OP("ledger_receipt")) made = api_Ledger_receipt(at(b), &status);
	else if (OP("draft_write")) api_Draft_write(at(a), view("line"), &status);
	else if (OP("draft_lines")) VALUE(api_Draft_lines(at(a), &status));
	else if (OP("draft_commit")) VALUE(api_Draft_commit(at(a), &status));
	else if (OP("receipt_count")) VALUE(api_Receipt_count(at(a), &status));
	else if (OP("receipt_clone")) made = api_Receipt_clone(at(b), &status);
	else if (OP("tallied")) VALUE(api_tallied(at(a), &status));
	else if (OP("stored")) VALUE(api_stored(at(a), &status));
	else if (OP("tally_new")) slots[a] = api_Tally_new();
	else if (OP("tally_bump")) VALUE(api_Tally_bump(at(a), &status));
	else if (OP("bump_later")) made = api_Tally_bump_later(at(b), &status);
	else if (OP("local_new")) slots[a] = b ? api_Local_new_homed() : api_Local_new();
	else if (OP("local_bump")) VALUE(api_Local_bump(at(a), &status));
	else if (OP("local_share")) made = api_Local_share(at(b), &status);
	else if (OP("local_later")) made = api_Local_later(at(b), &status);
	else if (OP("meters_new")) slots[a] = api_Meters_new();
	else if (OP("meters_first")) made = api_Meters_first(at(b), &status);
	else if (OP("meters_clear")) api_Meters_clear(at(a), &status);
	else if (OP("meter_new")) slots[a] = api_Meter_default();
	else if (OP("meter_add")) VALUE(api_Meter_add(at(a), (uint64_t)b, &status));
	else if (OP("meter_reset")) api_Meter_reset(at(a), &status);
	else if (OP("drop")) release(a, 1, &status);
	else if (OP("free")) release(a, 0, &status);
	else if (OP("poll")) poll(s, &status);
	else if (OP("drain")) api_thread_drain();
	else {
		fprintf(stderr, "unknown op %s\n", s->op);
		abort();
	}
	if (status) snprintf(s->out, sizeof s->out, "%s", status_name(status));
	else if (made) slots[a] = made;
}

/* The worker: runs each step the main thread hands it, one at a time. */
static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t changed = PTHREAD_COND_INITIALIZER;
static struct step *handed;
static int finished;

static void *worker(void *arg) {
	(void)arg;
	pthread_mutex_lock(&lock);
	for (;;) {
		while (!handed && !finished) pthread_cond_wait(&changed, &lock);
		if (!handed) break;
		pthread_mutex_unlock(&lock);
		run(handed);
		pthread_mutex_lock(&lock);
		handed = NULL;
		pthread_cond_broadcast(&changed);
	}
	pthread_mutex_unlock(&lock);
	return NULL;
}

static void on_worker(struct step *s) {
	pthread_mutex_lock(&lock);
	handed = s;
	pthread_cond_broadcast(&changed);
	while (handed) pthread_cond_wait(&changed, &lock);
	pthread_mutex_unlock(&lock);
}

static long long arg(const char *token) {
	return strcmp(token, "-") == 0 ? -1 : atoll(token);
}

int main(int argc, char **argv) {
	if (argc != 2) {
		fprintf(stderr, "usage: protocol <script>\n");
		return 2;
	}
	FILE *script = fopen(argv[1], "r");
	if (!script) {
		perror(argv[1]);
		return 2;
	}
	/* The first line names each slot's kind. */
	static char line[1 << 16];
	if (!fgets(line, sizeof line, script)) return 2;
	int n = 0;
	for (char *kind = strtok(line, " \n"); kind; kind = strtok(NULL, " \n")) {
		if (n == MAX_SLOTS) return 2;
		snprintf(kinds[n++], sizeof kinds[0], "%s", kind);
	}
	pthread_t thread;
	int err = pthread_create(&thread, NULL, worker, NULL);
	if (err) {
		fprintf(stderr, "pthread_create: %s\n", strerror(err));
		fclose(script);
		return 2;
	}
	while (fgets(line, sizeof line, script)) {
		struct step s;
		memset(&s, 0, sizeof s);
		s.args[0] = s.args[1] = s.args[2] = -1;
		char *token = strtok(line, " \n");
		s.thread = atoi(token);
		snprintf(s.op, sizeof s.op, "%s", strtok(NULL, " \n"));
		for (int i = 0; i < 3 && (token = strtok(NULL, " \n")); i++) s.args[i] = arg(token);
		if (s.thread == 0) run(&s);
		else on_worker(&s);
		printf("%s\n", s.out);
	}
	fclose(script);
	pthread_mutex_lock(&lock);
	finished = 1;
	pthread_cond_broadcast(&changed);
	pthread_mutex_unlock(&lock);
	pthread_join(thread, NULL);
	printf("wakes %lu drops %lu\n", atomic_load(&wakes), atomic_load(&drops));
	return 0;
}
