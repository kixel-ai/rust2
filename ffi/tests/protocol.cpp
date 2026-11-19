// Runs scripts tests/protocol/model.rs writes for C++ through the api fixture's C++ header:
// each step on the main thread or a worker, printing what it returned a line, then the
// wakes and drops of the wakers its polls passed. An object frees as its destructor runs,
// and a future as it is destroyed. The script never fails a check, which aborts in C++.

#include "api.hpp"

#include <atomic>
#include <condition_variable>
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <functional>
#include <memory>
#include <mutex>
#include <optional>
#include <sstream>
#include <string>
#include <thread>
#include <vector>

namespace {

std::atomic<unsigned long> wakes{0};
std::atomic<unsigned long> drops{0};

// Counts a waker's drop as its callable, which Rust's references and ours share, goes.
struct dropped {
	~dropped() {
		drops++;
	}
};

// One slot's handle, of whichever kind it holds.
struct slot {
	std::string kind;
	std::optional<api::Counter> counter;
	std::optional<api::Ledger> ledger;
	std::optional<api::Draft> draft;
	std::optional<api::Receipt> receipt;
	std::optional<api::Tally> tally;
	std::optional<api::Local> local;
	std::optional<api::Meters> meters;
	std::optional<api::Meter> meter;
	std::optional<api::Counter_plus_later_future> plus_later;
	std::optional<api::Tally_bump_later_future> bump_later;
	std::optional<api::Local_later_future> local_later;
	// The waker each poll of its future passes, made at the first, so Rust keeps one.
	std::optional<api::waker> waker;
};

std::vector<slot> slots;

struct step {
	int thread = 0;
	std::string op;
	// Its slots and numbers, -1 for an absent handle.
	long long args[3] = {-1, -1, -1};
	std::string out;
};

std::string value(unsigned long long v) {
	return std::to_string(v);
}

const api::Counter *counter_or_null(long long h) {
	return h < 0 ? nullptr : &*slots[h].counter;
}

template <class F>
std::string poll(slot &s, F &future) {
	if (!s.waker) {
		auto token = std::make_shared<dropped>();
		s.waker.emplace([token]() { wakes++; });
	}
	auto polled = future->poll(*s.waker);
	if (!polled) {
		return "pending";
	}
	future.reset();
	s.waker.reset();
	return value(*polled);
}

// Free a slot's handle: destroy whatever it holds.
void free_slot(slot &s) {
	s.counter.reset();
	s.ledger.reset();
	s.draft.reset();
	s.receipt.reset();
	s.tally.reset();
	s.local.reset();
	s.meters.reset();
	s.meter.reset();
	s.plus_later.reset();
	s.bump_later.reset();
	s.local_later.reset();
	s.waker.reset();
}

// Run one step on this thread, writing what it returned.
void run(step &st) {
	long long a = st.args[0], b = st.args[1], c = st.args[2];
	const std::string &op = st.op;
	st.out = "ok";
	if (op == "counter_new") slots[a].counter.emplace(api::Counter::new_("c", static_cast<std::uint64_t>(b)));
	else if (op == "counter_get") st.out = value(slots[a].counter->get());
	else if (op == "counter_add") {
		auto added = slots[a].counter->add(static_cast<std::uint64_t>(b));
		st.out = added ? value(added.value()) : "error";
	} else if (op == "counter_into_sum") {
		st.out = value(std::move(*slots[a].counter).into_sum(static_cast<std::uint64_t>(b)));
	} else if (op == "counter_plus") st.out = value(slots[a].counter->plus(counter_or_null(b)));
	else if (op == "counter_add_to") {
		st.out = value(slots[a].counter->add_to(b < 0 ? nullptr : &*slots[b].counter));
	} else if (op == "counter_clone") slots[a].counter.emplace(slots[b].counter->clone());
	else if (op == "plus_later") slots[a].plus_later.emplace(slots[b].counter->plus_later(counter_or_null(c)));
	else if (op == "ledger_new") slots[a].ledger.emplace(api::Ledger::new_());
	else if (op == "ledger_edit") {
		auto edited = slots[b].ledger->edit();
		if (edited) {
			slots[a].draft.emplace(std::move(edited.value()));
			st.out = "1";
		} else {
			st.out = "error";
		}
	} else if (op == "ledger_published") st.out = value(slots[a].ledger->published());
	else if (op == "ledger_seal") slots[a].ledger->seal();
	else if (op == "ledger_receipt") slots[a].receipt.emplace(slots[b].ledger->receipt());
	else if (op == "draft_write") slots[a].draft->write("line");
	else if (op == "draft_lines") st.out = value(slots[a].draft->lines());
	else if (op == "draft_commit") st.out = value(std::move(*slots[a].draft).commit());
	else if (op == "tallied") st.out = value(api::tallied(std::move(*slots[a].receipt)));
	else if (op == "stored") st.out = value(api::stored(std::move(*slots[a].receipt)));
	else if (op == "receipt_count") st.out = value(slots[a].receipt->count());
	else if (op == "receipt_clone") slots[a].receipt.emplace(slots[b].receipt->clone());
	else if (op == "tally_new") slots[a].tally.emplace(api::Tally::new_());
	else if (op == "tally_bump") st.out = value(slots[a].tally->bump());
	else if (op == "bump_later") slots[a].bump_later.emplace(slots[b].tally->bump_later());
	else if (op == "local_new") slots[a].local.emplace(api::Local::new_());
	else if (op == "local_bump") st.out = value(slots[a].local->bump());
	else if (op == "local_share") slots[a].local.emplace(slots[b].local->share());
	else if (op == "local_later") slots[a].local_later.emplace(slots[b].local->later());
	else if (op == "meters_new") slots[a].meters.emplace(api::Meters::new_());
	else if (op == "meters_first") slots[a].meter.emplace(slots[b].meters->first());
	else if (op == "meters_clear") slots[a].meters->clear();
	else if (op == "meter_new") slots[a].meter.emplace(api::Meter::default_());
	else if (op == "meter_add") st.out = value(slots[a].meter->add(static_cast<std::uint64_t>(b)));
	else if (op == "meter_reset") slots[a].meter->reset();
	else if (op == "free") free_slot(slots[a]);
	else if (op == "poll") {
		slot &s = slots[a];
		if (s.kind == "Counter_plus_later") st.out = poll(s, s.plus_later);
		else if (s.kind == "Tally_bump_later") st.out = poll(s, s.bump_later);
		else st.out = poll(s, s.local_later);
	} else if (op == "drain") api_thread_drain();
	else {
		std::fprintf(stderr, "unknown op %s\n", op.c_str());
		std::abort();
	}
}

// The worker: runs each step the main thread hands it, one at a time.
std::mutex lock;
std::condition_variable changed;
step *handed = nullptr;
bool finished = false;

void worker() {
	std::unique_lock<std::mutex> guard(lock);
	for (;;) {
		changed.wait(guard, [] { return handed != nullptr || finished; });
		if (handed == nullptr) {
			return;
		}
		guard.unlock();
		run(*handed);
		guard.lock();
		handed = nullptr;
		changed.notify_all();
	}
}

void on_worker(step &st) {
	std::unique_lock<std::mutex> guard(lock);
	handed = &st;
	changed.notify_all();
	changed.wait(guard, [] { return handed == nullptr; });
}

} // namespace

int main(int argc, char **argv) {
	if (argc != 2) {
		std::fprintf(stderr, "usage: protocol <script>\n");
		return 2;
	}
	const char *path = argv[1];
	std::ifstream in(path);
	if (!in) {
		std::perror(path);
		return 2;
	}
	std::string line;
	std::getline(in, line);
	{
		std::istringstream kinds(line);
		std::string kind;
		while (kinds >> kind) {
			slots.emplace_back();
			slots.back().kind = kind;
		}
	}
	std::thread thread(worker);
	while (std::getline(in, line)) {
		std::istringstream tokens(line);
		step st;
		tokens >> st.thread >> st.op;
		std::string token;
		for (int i = 0; i < 3 && tokens >> token; i++) {
			st.args[i] = token == "-" ? -1 : std::atoll(token.c_str());
		}
		if (st.thread == 0) run(st);
		else on_worker(st);
		std::printf("%s\n", st.out.c_str());
	}
	{
		std::lock_guard<std::mutex> guard(lock);
		finished = true;
	}
	changed.notify_all();
	thread.join();
	std::printf("wakes %lu drops %lu\n", wakes.load(), drops.load());
	return 0;
}
