// moq.c's script through the generated C++ header: a broadcast's track, published and
// subscribed to in one process, with every read polled as a future, and an origin, with
// moq-pattern, kio, moq-tokio, hang, and moq-mux exported into the same library,
// the catalog lent to C++ for a call, and a session's config filled in field by field.
//
// The local reads need no runtime: the waker counts wakes, and each poll that must wait
// is polled again after the write that wakes it. A session over QUIC runs on moq-tokio's
// fallback runtime, whose threads wake C++'s.

#include <atomic>
#include <chrono>
#include <condition_variable>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <deque>
#include <functional>
#include <mutex>
#include <string>
#include <thread>

#include "moq.hpp"

namespace {

int failures = 0;

#define CHECK(cond)                                                                                                    \
	do {                                                                                                               \
		if (!(cond)) {                                                                                                 \
			std::fprintf(stderr, "%s:%d: %s\n", __FILE__, __LINE__, #cond);                                            \
			failures++;                                                                                                \
		}                                                                                                              \
	} while (0)

// Whether a returned buffer holds `s`.
bool is(const moq::buf &buf, const char *s) {
	bool same = buf.str() == s;
	if (!same) {
		std::fprintf(stderr, "got \"%.*s\", want \"%s\"\n", static_cast<int>(buf.size()),
		             reinterpret_cast<const char *>(buf.data()), s);
	}
	return same;
}

int wakes = 0;

// Runs the jobs an executor is given on a thread of its own, until destroyed.
class worker {
	std::mutex mutex_;
	std::condition_variable cv_;
	std::deque<std::function<void()>> jobs_;
	bool stopping_ = false;
	std::thread thread_;

	void run() {
		for (;;) {
			std::function<void()> job;
			{
				std::unique_lock<std::mutex> lock(mutex_);
				cv_.wait(lock, [&]() { return stopping_ || !jobs_.empty(); });
				if (jobs_.empty()) {
					return;
				}
				job = std::move(jobs_.front());
				jobs_.pop_front();
			}
			job();
		}
	}

public:
	worker() : thread_([this]() { run(); }) {}
	~worker() {
		{
			std::lock_guard<std::mutex> lock(mutex_);
			stopping_ = true;
		}
		cv_.notify_one();
		thread_.join();
	}

	auto executor() {
		return [this](std::function<void()> job) {
			{
				std::lock_guard<std::mutex> lock(mutex_);
				jobs_.push_back(std::move(job));
			}
			cv_.notify_one();
		};
	}
};

// Fails the run rather than hanging when what it guards stalls past `limit`.
class watchdog {
	std::mutex mutex_;
	std::condition_variable cv_;
	bool done_ = false;
	std::thread thread_;

public:
	explicit watchdog(std::chrono::seconds limit)
	    : thread_([this, limit]() {
		      std::unique_lock<std::mutex> lock(mutex_);
		      if (!cv_.wait_for(lock, limit, [&]() { return done_; })) {
			      std::fprintf(stderr, "stalled for %lld seconds\n", static_cast<long long>(limit.count()));
			      std::abort();
		      }
	      }) {}
	~watchdog() {
		{
			std::lock_guard<std::mutex> lock(mutex_);
			done_ = true;
		}
		cv_.notify_one();
		thread_.join();
	}
};

} // namespace

int main() {
	moq::waker waker([]() { wakes++; });

	moq::broadcast_Producer broadcast = moq::broadcast_Info::new_().produce();
	auto track = broadcast.create_track("video", std::nullopt);
	CHECK(track && is(track->name(), "video"));

	// A second track of the same name is an error, printed as Rust prints it.
	auto duplicate = broadcast.create_track("video", std::nullopt);
	CHECK(!duplicate && is(duplicate.error().to_string(), "duplicate"));

	moq::track_Consumer consumer = track->consume();
	auto subscribed = consumer.subscribe(std::nullopt).poll(waker);
	CHECK(subscribed && *subscribed);
	moq::track_Subscriber subscriber = std::move(**subscribed);

	// A group's arrival wakes the subscriber's pending receive.
	auto receiving = subscriber.recv_group();
	CHECK(!receiving.poll(waker));
	auto writer = track->append_group();
	CHECK(writer && wakes == 1);
	auto received = receiving.poll(waker);
	CHECK(received && *received && received->value());
	moq::group_Consumer group = std::move(*received->value());
	CHECK(group.index() == 0);

	// So does a frame's.
	{
		auto reading = group.read_frame();
		CHECK(!reading.poll(waker));
		CHECK(writer->write_frame(nullptr, "hello"));
		CHECK(wakes == 2);
		auto frame = reading.poll(waker);
		CHECK(frame && *frame && frame->value());
		const moq::frame_Frame &hello = *frame->value();
		CHECK(is(hello.payload_get(), "hello"));
		CHECK(!hello.timestamp_get());
		// When it arrived: kio's clock's `Instant`, from kio's own export.
		CHECK(moq::clock_now().checked_duration_since(hello.arrived()).has_value());
	}

	// A timestamp is a `Copy` value behind a handle: passing it copies it out. The track is
	// untimed, so a timestamped frame is refused.
	auto at = moq::Timestamp::from_millis(1500);
	CHECK(at && at->as_secs() == 1);
	auto late = writer->write_frame(&*at, "late");
	CHECK(!late && is(late.error().to_string(), "frame timestamp doesn't fit its track"));
	CHECK(at->as_secs() == 1);

	// The last frame, bytes that are not UTF-8, then the group's end, which reads as None,
	// both blocking.
	CHECK(writer->write_frame(nullptr, "\xffworld"));
	CHECK(std::move(*writer).finish());
	auto world = group.read_frame().get();
	CHECK(world && world.value() && is(world.value()->payload_get(), "\xffworld"));
	auto end = group.read_frame().get();
	CHECK(end && !end.value());

	// An origin and the driver that runs it, a tuple. Its root, which it lends, is returned
	// as a new handle to a clone.
	auto [origin, driver] = moq::origin_Producer::new_(moq::origin_Config::default_());
	CHECK(is(origin.root().as_str(), ""));

	// Scoped beneath a root to patterns, which moq-pattern's own export builds.
	{
		auto alice = moq::Pattern::subtree("alice");
		CHECK(alice);
		moq::Patterns patterns = moq::Patterns::new_();
		CHECK(patterns.insert(*alice));
		auto scoped = origin.scope("room", patterns);
		CHECK(scoped);
		CHECK(is(scoped->root().as_str(), "room"));
		auto absolute = scoped->absolute("alice");
		CHECK(absolute && is(absolute->as_str(), "room/alice"));
		moq::Patterns allowed = scoped->allowed();
		CHECK(allowed.matches("alice/cam"));
		CHECK(!allowed.matches("bob"));
		absolute = scoped->consume().absolute("bob");
		CHECK(absolute && is(absolute->as_str(), "room/bob"));
	}

	// A broadcast the origin creates is stamped with its path, which its info lends.
	{
		auto publication = origin.create_broadcast("room/alice");
		CHECK(publication);
		moq::broadcast_Producer producer = publication->broadcast();
		moq::Path path = producer.info().path_get();
		CHECK(is(path.as_str(), "room/alice"));
		// A path's clone is a value no call tells apart, so its copies share one handle,
		// while a producer's clone counts producers, so it crosses into Rust.
		moq::Path shared = path;
		CHECK(shared == path);
		moq::broadcast_Producer cloned = producer.clone();
		CHECK(is(cloned.info().path_get().as_str(), "room/alice"));

		// Paths own their segments, so C++ builds them, and relative ones, from a string.
		auto empty = moq::Path::new_("/cam/");
		CHECK(!empty && empty.error() == moq::path_Error::empty_segment);
		auto cam = moq::Path::new_("cam");
		CHECK(cam && is(cam->as_str(), "cam"));
		auto joined = path.join(*cam);
		CHECK(joined && is(joined->as_str(), "room/alice/cam"));
		std::optional<moq::Path> stripped = joined->strip_prefix(path);
		CHECK(stripped && *stripped == *cam);
		CHECK(!path.strip_prefix(*joined));
		auto up = moq::path_Relative::new_("../bob");
		CHECK(up);
		auto resolved = joined->resolve(*up);
		CHECK(resolved && is(resolved->as_str(), "room/bob"));
	}

	// The driver, polled as a language's event loop would: at the time, and with a waiter
	// made from the waker and held across polls, since what a poll registers lives only as
	// long as its waiter. It runs until every owner of the origin is gone.
	moq::Waiter waiter = moq::Waiter::new_(waker);
	CHECK(driver.poll(moq::clock_now(), waiter));
	{
		// A client publishes the origin through moq-net's generic `with_publisher`, at each
		// type that implements its bound, by reference, so C++ keeps its handle.
		moq::Client publishing = moq::Client::new_().with_publisher_Producer(origin);
		(void)publishing;
	}
	int before = wakes;
	{
		moq::origin_Producer last = std::move(origin);
	}
	// The last owner's drop wakes the driver, which then finishes, cleanly.
	CHECK(wakes > before);
	auto closed = driver.poll(moq::clock_now(), waiter);
	CHECK(!closed && is(closed.error().to_string(), "closed"));

	// moq-mux reads an OpusHead into hang's catalog config: 44.1 kHz in, stereo, which the
	// catalog lists at the decoder's 48 kHz, keeping the head as its description.
	static const std::uint8_t head[] = {'O', 'p', 'u', 's', 'H', 'e', 'a', 'd', 1, 2, 0x38, 0x01, 0x44, 0xac, 0, 0, 0, 0, 0};
	auto audio = moq::mux_codec_opus_config(moq::view(head, sizeof head));
	CHECK(audio);
	CHECK(audio->sample_rate_get() == 48000);
	CHECK(audio->channel_count_get() == 2);
	CHECK(std::holds_alternative<moq::hang_catalog_AudioCodec_Opus>(audio->codec_get()));
	auto description = audio->description_get();
	CHECK(description && description->size() == sizeof head && std::memcmp(description->data(), head, sizeof head) == 0);
	// A truncated head is moq-mux's error.
	auto truncated = moq::mux_codec_opus_config(moq::view(head, 8));
	CHECK(!truncated && !truncated.error().to_string().empty());

	// A subscription's ranges, from any iterable of them: a vector.
	{
		moq::track_Range live = moq::track_Range::live();
		moq::track_Subscription ranged = moq::track_Subscription::default_().with_ranges(
		    {live, moq::track_Range::groups({{MOQ_BOUND_INCLUDED, 2}, {MOQ_BOUND_EXCLUDED, 6}})});
		std::vector<moq::track_Range> got = ranged.ranges_get();
		CHECK(got.size() == 2 && got[0] == live);
		CHECK(got.size() == 2 && std::get<moq::track_Anchor_Absolute>(got[1].start)._0 == 2);
		CHECK(got.size() == 2 && got[1].end && std::get<moq::track_Anchor_Absolute>(*got[1].end)._0 == 6);
	}

	// A catalog at moq-mux's untyped extension, from the config C++ builds, whose generic
	// `is_claimed` is named by the last segment of its argument, and whose `mutate` lends
	// the catalog to C++.
	{
		moq::broadcast_Producer publisher = moq::broadcast_Info::new_().produce();
		moq::mux_catalog_Config_Extra extra =
		    moq::mux_catalog_Config::default_().with_catalog_Extra(moq::hang_Catalog_Extra::default_());
		auto catalog = moq::mux_catalog_Producer_Extra::new_(publisher, std::move(extra));
		CHECK(catalog);
		CHECK(!catalog->is_claimed_BinaryConfig("state"));
		int read_catalog = 0;
		CHECK(catalog->mutate([&](moq::borrow<moq::hang_Catalog_Extra> lent) {
			auto json = lent->to_json();
			CHECK(json);
			if (json && json.value().str().substr(0, 1) == "{") {
				read_catalog++;
			}
		}));
		CHECK(read_catalog == 1);

		// A JSON track the catalog lists, published through moq-mux's generic
		// `json_snapshot` at a JSON document, and a section C++ parses from text, set in the
		// lent catalog and read back as text.
		// Its track has a timescale, since every write is stamped.
		auto milli = moq::Timescale::MILLI();
		auto status = publisher.create_track("status", moq::track_Info::default_().with_timescale(&milli));
		CHECK(status);
		auto status_snapshot = catalog->json_snapshot_Config(std::move(*status), moq::mux_json_Config::default_(),
		                                                     moq::mux_json_Options::default_());
		CHECK(status_snapshot && is(status_snapshot->name(), "status"));
		CHECK(catalog->is_claimed_JsonConfig("status"));
		// A value C++ parses is published as the track's latest, which a subscriber reads
		// back as JSON text.
		CHECK(status_snapshot->update(moq::Json::parse(R"({"on": true})").value(), moq::Timestamp::now()));
		moq::track_Subscriber status_subscriber = status_snapshot->consume();
		auto status_group = status_subscriber.recv_group().get();
		CHECK(status_group && status_group.value());
		if (status_group && status_group.value()) {
			auto status_frame = status_group.value()->read_frame().get();
			CHECK(status_frame && status_frame.value() && is(status_frame.value()->payload_get(), R"({"on":true})"));
		}
		auto app = moq::Json::parse(R"({"v": [1, true]})");
		CHECK(app);
		CHECK(catalog->mutate([&](moq::borrow<moq::hang_Catalog_Extra> lent) {
			moq::mux_catalog_hang_Extra sections = lent->ext_get();
			CHECK(sections.set("app", std::move(*app)));
			lent->ext_set(std::move(sections));
		}));
		CHECK(catalog->mutate([&](moq::borrow<moq::hang_Catalog_Extra> lent) {
			auto json = lent->to_json();
			CHECK(json && json.value().str().find("\"status\"") != std::string_view::npos);
			auto section = lent->ext_get().get("app");
			CHECK(section && is(section->to_text(), R"({"v":[1,true]})"));
		}));
		// Malformed text fails to parse, and a reserved section is moq-mux's error.
		auto broken = moq::Json::parse(R"({"v":)");
		CHECK(!broken && is(broken.error(), "EOF while parsing a value at line 1 column 5"));
		moq::mux_catalog_hang_Extra sections = moq::mux_catalog_hang_Extra::default_();
		auto reserved = sections.set("json", moq::Json::parse("1").value());
		CHECK(!reserved && is(reserved.error().to_string(), "reserved catalog section: json"));
	}

	// A session's config, filled in as Rust assigns it, and a nested field by getting a copy,
	// editing it, and setting it back.
	{
		moq::tokio_connect_Config connect = moq::tokio_connect_Config::default_();
		moq::tokio_tls_Connect tls = connect.tls_get();
		tls.insecure_set(true);
		tls.host_name_set("relay.example");
		tls.fingerprint_set({"ab:cd"});
		connect.tls_set(std::move(tls));
		connect.implementation_set("moq.cpp");
		connect.url_set(moq::Url::parse("https://relay.example:4443/demo").value());
		CHECK(connect.tls_get().insecure_get() == std::optional<bool>(true));
		CHECK(is(*connect.tls_get().host_name_get(), "relay.example"));
		auto fingerprints = connect.tls_get().fingerprint_get();
		CHECK(fingerprints.size() == 1 && is(fingerprints[0], "ab:cd"));
		CHECK(is(*connect.implementation_get(), "moq.cpp"));
		CHECK(is(connect.url_get()->to_string(), "https://relay.example:4443/demo"));

		// A listener's bind, a socket address.
		moq::socket_addr addr{};
		addr.ip[15] = 1;
		addr.port = 4443;
		addr.v6 = true;
		moq::tokio_listen_Config listen = moq::tokio_listen_Config::default_();
		listen.bind_set(moq::tokio_listen_Bind(moq::tokio_listen_Bind_Addr{addr}));
		auto bind = listen.bind_get();
		CHECK(bind && is(moq::tokio_listen_Bind_to_string(*bind), "[::1]:4443"));
		// The preferred address of each family, which takes only that family.
		listen.preferred_v6_set(addr);
		auto preferred = listen.preferred_v6_get();
		CHECK(preferred && preferred->v6 && preferred->port == 4443 && !listen.preferred_v4_get());
		// A constant slice, as a vector of clones.
		CHECK(!moq::ALPNS().empty());
	}

	// moq-tokio's terminal connect errors, which moq-net's `Error` shares no names with.
	CHECK(moq::tokio_ConnectError_is_auth(moq::tokio_ConnectError::unauthorized));
	CHECK(is(moq::tokio_ConnectError_to_string(moq::tokio_ConnectError::unauthorized), "unauthorized"));

	// A session, as moq.c runs it: a server bound to port 0 reads back the port it got and
	// serves with one call, on an executor of its own, and a client dials it with one call
	// and reads a frame back, blocking on each read while moq-tokio's fallback runtime wakes
	// it from its threads.
	{
		watchdog limit(std::chrono::seconds(30));
		moq::origin_Producer served = moq::tokio_origin_spawn();
		auto live = served.create_broadcast("live");
		CHECK(live && live->announce(moq::origin_Route::default_()));
		moq::broadcast_Producer live_broadcast = live->broadcast();
		auto live_track = live_broadcast.create_track("video", std::nullopt);
		CHECK(live_track);
		auto live_group = live_track->append_group();
		CHECK(live_group && live_group->write_frame(nullptr, "hello"));
		CHECK(std::move(*live_group).finish());

		moq::tokio_server_Config server_config = moq::tokio_server_Config::default_();
		moq::tokio_listen_Config listen = server_config.listen_get();
		moq::socket_addr any{};
		any.v6 = true;
		listen.bind_set(moq::tokio_listen_Bind(moq::tokio_listen_Bind_Addr{any}));
		moq::tokio_tls_Listen tls = listen.tls_get();
		tls.generate_set({"localhost"});
		listen.tls_set(std::move(tls));
		server_config.listen_set(std::move(listen));
		auto server = std::move(server_config).init();
		CHECK(server);
		auto local = server->local_addr();
		CHECK(local && local->v6 && local->port != 0);
		worker serving_thread;
		std::atomic<bool> stopped{false};
		moq::task serving = std::move(*server).serve_publish(served.consume()).then(serving_thread.executor(),
		                                                                            [&](auto &&) { stopped = true; });

		moq::tokio_connect_Config connect = moq::tokio_connect_Config::default_();
		connect.url_set(moq::Url::parse("moqt://localhost:" + std::to_string(local->port)).value());
		moq::tokio_tls_Connect insecure = connect.tls_get();
		insecure.insecure_set(true);
		connect.tls_set(std::move(insecure));
		moq::tokio_client_Config client_config = moq::tokio_client_Config::default_();
		client_config.connect_set(std::move(connect));
		auto client = std::move(client_config).init();
		CHECK(client);
		moq::origin_Producer incoming = moq::tokio_origin_spawn();
		moq::origin_Consumer remote = incoming.consume();
		auto connection = client->clone().with_reconnect(false).consume(std::move(incoming));
		CHECK(connection);
		auto established = std::move(*connection).established().get();
		CHECK(established);

		auto remote_broadcast = remote.request("live", std::nullopt).get();
		CHECK(remote_broadcast);
		auto remote_track = remote_broadcast->track("video");
		CHECK(remote_track);
		auto remote_subscriber = remote_track->subscribe(std::nullopt).get();
		CHECK(remote_subscriber);
		auto remote_group = remote_subscriber->recv_group().get();
		CHECK(remote_group && remote_group.value());
		auto remote_frame = remote_group.value()->read_frame().get();
		CHECK(remote_frame && remote_frame.value() && is(remote_frame.value()->payload_get(), "hello"));

		CHECK(std::move(*established).close().get());
		std::move(*client).close().get();
		serving = moq::task();
		CHECK(!stopped);
	}

	if (failures == 0) {
		std::printf("ok\n");
	}
	return failures != 0;
}
