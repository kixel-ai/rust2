// api.hpp
#ifndef API_HPP_CRATE7
#define API_HPP_CRATE7
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#elif defined(_MSC_VER)
#pragma warning(push)
#pragma warning(disable : 4996)
#endif
#include "rt.hpp"
#include "core.hpp"
#include "dep.hpp"
#include "serdejson.hpp"
#include "std.hpp"
#include "tls.hpp"
#include "tracingcore.hpp"
#include "url.hpp"
namespace api {
	enum class Shape {
		circle = 1,
		square = 4,
	};
	enum class Status_ {
		moved = 0,
		kept = 1,
	};
	enum class Family {
		native = 0,
		wasm = 1,
	};
	enum class VecClashTone {
		error = 0,
	};
	enum class Tier {
		low = 0,
		high = 1,
		compare_to = 2,
		partial_compare_to = 3,
	};
	enum class clash_Level {
		default_ = 0,
		debug = 1,
	};
	enum class clash_Op {
		delete_ = 0,
		delete__ = 1,
	};
	inline constexpr ::std::uint64_t LIMIT = API_LIMIT;
	inline constexpr ::std::int64_t FAR = API_FAR;
	inline constexpr ::std::int32_t EOF_ = API_EOF;
	inline constexpr ::std::uint64_t URL = API_URL;
	[[deprecated("pass a step to `Dial::turn`, whose `}` or `{` alone stays in the note")]] inline constexpr ::std::uint64_t STEP = API_STEP;
	class RUST2_FFI_CONSUMABLE Board {
		RUST2_FFI_UNCONSUMED Board(Board &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Board &operator=(Board &&other) noexcept = default;
		static ::api::Board new_(::api::taken<::std::tuple<::api::view, ::api::Counter>> counters);
		RUST2_FFI_CONSUMES ::std::vector<::std::tuple<::api::buf, ::api::Counter>> into_counters() &&;
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::api::buf, ::api::Counter>> ranked() const;
		RUST2_FFI_CALLABLE ::api::Counter best() const;
	};
	class RUST2_FFI_CONSUMABLE Tab {
		RUST2_FFI_UNCONSUMED Tab(Tab &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Tab &operator=(Tab &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::api::Ledger ledger_get() const;
		RUST2_FFI_CALLABLE ::api::buf label_get() const;
	};
	class RUST2_FFI_CONSUMABLE Receipt {
		RUST2_FFI_UNCONSUMED Receipt(Receipt &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Receipt &operator=(Receipt &&other) noexcept = default;
		RUST2_FFI_CALLABLE Receipt clone() const;
		RUST2_FFI_CALLABLE ::std::uint32_t count() const;
	};
	class RUST2_FFI_CONSUMABLE Draft {
		RUST2_FFI_UNCONSUMED Draft(Draft &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Draft &operator=(Draft &&other) noexcept = default;
		RUST2_FFI_CALLABLE bool edited() const;
		RUST2_FFI_CALLABLE ::api::buf describe() const;
		RUST2_FFI_CONSUMES ::std::uint32_t commit() &&;
		RUST2_FFI_CALLABLE ::api::buf title_get() const;
		RUST2_FFI_CALLABLE void title_set(::api::input value);
		RUST2_FFI_CALLABLE ::std::uint32_t lines() const;
		RUST2_FFI_CALLABLE void write(::api::input line);
	};
	class RUST2_FFI_CONSUMABLE Filed_Receipt {
		RUST2_FFI_UNCONSUMED Filed_Receipt(Filed_Receipt &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Filed_Receipt &operator=(Filed_Receipt &&other) noexcept = default;
		RUST2_FFI_CALLABLE Filed_Receipt clone() const;
		RUST2_FFI_CALLABLE ::std::uint32_t count() const;
	};
	class RUST2_FFI_CONSUMABLE Pad_u16 {
		RUST2_FFI_UNCONSUMED Pad_u16(Pad_u16 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Pad_u16 &operator=(Pad_u16 &&other) noexcept = default;
		static ::api::Pad_u16 wide();
		RUST2_FFI_CALLABLE ::std::uint16_t tag() const;
		RUST2_FFI_CALLABLE ::std::uint16_t title() const;
		RUST2_FFI_CALLABLE ::api::buf title_get() const;
		RUST2_FFI_CALLABLE ::std::uint32_t lines() const;
		RUST2_FFI_CALLABLE ::api::buf describe() const;
	};
	class RUST2_FFI_CONSUMABLE Pad_u8 {
		RUST2_FFI_UNCONSUMED Pad_u8(Pad_u8 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Pad_u8 &operator=(Pad_u8 &&other) noexcept = default;
		static ::api::Pad_u8 small();
		RUST2_FFI_CALLABLE ::api::buf describe() const;
		RUST2_FFI_CALLABLE ::api::buf title_get() const;
		RUST2_FFI_CALLABLE ::std::uint32_t lines() const;
	};
	class RUST2_FFI_CONSUMABLE Layered {
		RUST2_FFI_UNCONSUMED Layered(Layered &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Layered &operator=(Layered &&other) noexcept = default;
		static ::api::Layered new_();
		RUST2_FFI_CALLABLE ::api::buf title_get() const;
		RUST2_FFI_CALLABLE ::std::uint32_t lines() const;
	};
	class RUST2_FFI_CONSUMABLE Marker {
		RUST2_FFI_UNCONSUMED Marker(Marker &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Marker &operator=(Marker &&other) noexcept = default;
		static ::api::Marker new_(::api::input label, ::std::int32_t x, ::std::int32_t y);
		RUST2_FFI_CALLABLE ::api::buf label() const;
		RUST2_FFI_CALLABLE ::std::int32_t x() const;
		static ::std::int32_t y();
		RUST2_FFI_CALLABLE ::std::int32_t x_get() const;
		RUST2_FFI_CALLABLE void x_set(::std::int32_t value);
		RUST2_FFI_CALLABLE ::std::int32_t y_get() const;
		RUST2_FFI_CALLABLE void y_set(::std::int32_t value);
		RUST2_FFI_CALLABLE ::api::Point offset(::std::int32_t dx, ::std::int32_t dy) const;
	};
	class RUST2_FFI_CONSUMABLE Batch {
		RUST2_FFI_UNCONSUMED Batch(Batch &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Batch &operator=(Batch &&other) noexcept = default;
		static ::api::Batch new_();
		RUST2_FFI_CALLABLE bool push(::std::uint32_t v);
		RUST2_FFI_CALLABLE ::std::size_t capacity() const;
	};
	class RUST2_FFI_CONSUMABLE Batch_2 {
		RUST2_FFI_UNCONSUMED Batch_2(Batch_2 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Batch_2 &operator=(Batch_2 &&other) noexcept = default;
		static ::api::Batch_2 new_();
		RUST2_FFI_CALLABLE bool push(::std::uint32_t v);
		RUST2_FFI_CALLABLE ::std::size_t capacity() const;
	};
	class RUST2_FFI_CONSUMABLE Waiter {
		RUST2_FFI_UNCONSUMED Waiter(Waiter &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Waiter &operator=(Waiter &&other) noexcept = default;
		static ::api::Waiter new_(const ::api::waker &waker_);
		RUST2_FFI_CALLABLE ::api::waker waker() const;
		RUST2_FFI_CALLABLE ::api::waker context() const;
		template <class F0> static ::api::Waiter_poll_future_future poll_future(F0 future_);
	};
	class RUST2_FFI_CONSUMABLE Feed {
		RUST2_FFI_UNCONSUMED Feed(Feed &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Feed &operator=(Feed &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::api::Feed_next_future next();
	};
	class RUST2_FFI_CONSUMABLE Countdown {
		RUST2_FFI_UNCONSUMED Countdown(Countdown &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Countdown &operator=(Countdown &&other) noexcept = default;
		static ::api::Countdown new_(::std::uint32_t from);
		static ::api::Countdown told(::std::uint32_t from, ::std::unique_ptr<::api::Log> log);
		RUST2_FFI_CALLABLE ::api::Countdown_next_future next();
	};
	class RUST2_FFI_CONSUMABLE Retired {
		RUST2_FFI_UNCONSUMED Retired(Retired &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Retired &operator=(Retired &&other) noexcept = default;
		static ::api::Retired new_(::std::uint32_t from);
		[[deprecated("count with `Countdown`")]] RUST2_FFI_CALLABLE ::api::Retired_next_future next();
	};
	class RUST2_FFI_CONSUMABLE Folio {
		RUST2_FFI_UNCONSUMED Folio(Folio &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Folio &operator=(Folio &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::std::uint16_t page_get() const;
		RUST2_FFI_CALLABLE void page_set(::std::uint16_t value);
		RUST2_FFI_CALLABLE ::api::buf title_get() const;
		RUST2_FFI_CALLABLE void title_set(::api::input value);
		RUST2_FFI_CALLABLE ::std::optional<::api::buf> note_get() const;
		RUST2_FFI_CALLABLE void note_set(::std::optional<::api::input> value);
		RUST2_FFI_CALLABLE ::api::Hasp hasp_get() const;
		RUST2_FFI_CALLABLE void hasp_set(::api::Hasp value);
		RUST2_FFI_CALLABLE ::api::Hasp dock_get() const;
		static ::api::Folio new_(::std::uint16_t page, ::api::input title);
		RUST2_FFI_CALLABLE ::std::uint8_t seed() const;
	};
	class RUST2_FFI_CONSUMABLE Logger {
		RUST2_FFI_UNCONSUMED Logger(Logger &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Logger &operator=(Logger &&other) noexcept = default;
		static ::api::Logger new_(::std::unique_ptr<::api::Log> sink);
		RUST2_FFI_CALLABLE bool write(::api::input line) const;
		RUST2_FFI_CALLABLE bool write_from_thread(::api::input line) const;
	};
	class RUST2_FFI_CONSUMABLE Journal {
		RUST2_FFI_UNCONSUMED Journal(Journal &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Journal &operator=(Journal &&other) noexcept = default;
		static ::api::Journal new_(::std::unique_ptr<::api::Log> sink);
		RUST2_FFI_CALLABLE void redirect(::std::unique_ptr<::api::Log> sink);
		RUST2_FFI_CALLABLE bool write(::api::input line) const;
	};
	class RUST2_FFI_CONSUMABLE Local {
		RUST2_FFI_UNCONSUMED Local(Local &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Local &operator=(Local &&other) noexcept = default;
		friend bool operator==(const Local &a__, const Local &b_);
		friend bool operator!=(const Local &a__, const Local &b_);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		static Local default_();
		static ::api::Local new_();
		RUST2_FFI_CALLABLE ::std::uint32_t bump() const;
		RUST2_FFI_CALLABLE ::api::Local share() const;
		RUST2_FFI_CALLABLE ::api::Local_later_future later() const;
		template <class F0> RUST2_FFI_CALLABLE ::std::uint32_t visit(F0 &&f) const;
		RUST2_FFI_CALLABLE ::api::Local_poll_count_future poll_count() const;
		RUST2_FFI_CALLABLE ::std::uint64_t thread_number() const;
		RUST2_FFI_CALLABLE bool on_own_thread() const;
	};
	class RUST2_FFI_CONSUMABLE Wrapped {
		RUST2_FFI_UNCONSUMED Wrapped(Wrapped &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Wrapped &operator=(Wrapped &&other) noexcept = default;
		static ::api::Wrapped new_(::std::uint64_t n);
		RUST2_FFI_CALLABLE ::std::uint64_t n_get() const;
		RUST2_FFI_CALLABLE void n_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE bool on_get() const;
		RUST2_FFI_CALLABLE void on_set(bool value);
		RUST2_FFI_CALLABLE ::std::uint64_t count() const;
		RUST2_FFI_CALLABLE void recount(::std::uint64_t n);
		RUST2_FFI_CALLABLE ::std::uint64_t steps() const;
		RUST2_FFI_CALLABLE ::std::uint32_t locals() const;
	};
	class RUST2_FFI_CONSUMABLE Buffer {
		RUST2_FFI_UNCONSUMED Buffer(Buffer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Buffer &operator=(Buffer &&other) noexcept = default;
		static ::api::Buffer new_(::std::uint32_t len);
		RUST2_FFI_CALLABLE void blank();
		RUST2_FFI_CALLABLE bool zeroed() const;
	};
	class RUST2_FFI_CONSUMABLE LocalWaiter {
		RUST2_FFI_UNCONSUMED LocalWaiter(LocalWaiter &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		LocalWaiter &operator=(LocalWaiter &&other) noexcept = default;
		static ::api::LocalWaiter new_(const ::api::waker &waker_);
		static ::api::LocalWaiter_poll_ready_future poll_ready(::std::uint32_t n);
	};
	class RUST2_FFI_CONSUMABLE Retry {
		RUST2_FFI_UNCONSUMED Retry(Retry &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Retry &operator=(Retry &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> limit_get() const;
		RUST2_FFI_CALLABLE void limit_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::std::optional<::std::uint32_t> &value);
		static ::api::Retry new_(const ::std::optional<::std::uint32_t> &limit);
		RUST2_FFI_CALLABLE ::std::uint32_t limit() const;
		RUST2_FFI_CALLABLE ::std::uint32_t delay() const;
		RUST2_FFI_CALLABLE void set_limit(::std::uint32_t limit);
	};
	class RUST2_FFI_CONSUMABLE Digest {
		RUST2_FFI_UNCONSUMED Digest(Digest &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Digest &operator=(Digest &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::std::array<::std::uint8_t, 4> sum_get() const;
		RUST2_FFI_CALLABLE void sum_set(const ::std::array<::std::uint8_t, 4> &value);
		static ::api::Digest new_();
		RUST2_FFI_CALLABLE ::std::array<::std::uint8_t, 4> flipped(const ::std::array<::std::uint8_t, 4> &mask) const;
	};
	class RUST2_FFI_CONSUMABLE Cursor {
		RUST2_FFI_UNCONSUMED Cursor(Cursor &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Cursor &operator=(Cursor &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::std::uint64_t at() const;
	};
	class RUST2_FFI_CONSUMABLE Outer_Inner_u8 {
		RUST2_FFI_UNCONSUMED Outer_Inner_u8(Outer_Inner_u8 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Outer_Inner_u8 &operator=(Outer_Inner_u8 &&other) noexcept = default;
		static ::api::Outer_Inner_u8 new_(::std::uint8_t value);
		RUST2_FFI_CALLABLE ::std::uint8_t get() const;
	};
	class RUST2_FFI_CONSUMABLE Slot_u32 {
		RUST2_FFI_UNCONSUMED Slot_u32(Slot_u32 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Slot_u32 &operator=(Slot_u32 &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::std::uint32_t value_get() const;
		RUST2_FFI_CALLABLE void value_set(::std::uint32_t value);
		static ::api::Slot_u32 new_(::std::uint32_t value);
	};
	class RUST2_FFI_CONSUMABLE Slot_u8 {
		RUST2_FFI_UNCONSUMED Slot_u8(Slot_u8 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Slot_u8 &operator=(Slot_u8 &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::std::uint8_t value_get() const;
		RUST2_FFI_CALLABLE void value_set(::std::uint8_t value);
		static ::api::Slot_u8 new_(::std::uint8_t value);
		RUST2_FFI_CALLABLE ::std::uint16_t value() const;
	};
	class RUST2_FFI_CONSUMABLE Resolving {
		RUST2_FFI_UNCONSUMED Resolving(Resolving &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Resolving &operator=(Resolving &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::api::tls_Resolver _0_get() const;
		RUST2_FFI_CALLABLE void _0_set(::api::tls_Resolver value);
		static ::api::Resolving new_(::api::input name);
	};
	class RUST2_FFI_CONSUMABLE Codecs {
		RUST2_FFI_UNCONSUMED Codecs(Codecs &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Codecs &operator=(Codecs &&other) noexcept = default;
		static Codecs default_();
		RUST2_FFI_CALLABLE ::std::vector<::api::tls_Codec> list_get() const;
		RUST2_FFI_CALLABLE void list_set(::std::vector<::api::tls_Codec> value);
		static ::api::Codecs new_();
		RUST2_FFI_CALLABLE ::std::uint32_t ids();
	};
	class RUST2_FFI_CONSUMABLE Der {
		RUST2_FFI_UNCONSUMED Der(Der &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Der &operator=(Der &&other) noexcept = default;
		static ::api::Der from_slice(::api::input bytes);
		RUST2_FFI_CALLABLE ::api::buf bytes() const;
	};
	class RUST2_FFI_CONSUMABLE Verbosity {
		RUST2_FFI_UNCONSUMED Verbosity(Verbosity &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Verbosity &operator=(Verbosity &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::api::Level level_get() const;
		RUST2_FFI_CALLABLE void level_set(::api::Level value);
		static ::api::Verbosity new_(::api::Level level);
		RUST2_FFI_CALLABLE ::std::optional<::api::Level> filter() const;
	};
	class RUST2_FFI_CONSUMABLE Site {
		RUST2_FFI_UNCONSUMED Site(Site &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Site &operator=(Site &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::api::Url home_get() const;
		RUST2_FFI_CALLABLE void home_set(::api::Url value);
		static ::api::Site new_(::api::Url home);
	};
	class RUST2_FFI_CONSUMABLE Trail {
		RUST2_FFI_UNCONSUMED Trail(Trail &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Trail &operator=(Trail &&other) noexcept = default;
		static ::api::Trail new_(const ::std::vector<::api::Point> &points, const ::api::items<::api::view> &names);
		RUST2_FFI_CALLABLE ::std::vector<::api::Point> points() const;
		RUST2_FFI_CALLABLE ::std::vector<::api::Label> labels() const;
		RUST2_FFI_CALLABLE ::std::vector<::api::Point> iter() const;
		RUST2_FFI_CALLABLE ::std::vector<::api::buf> names() const;
		RUST2_FFI_CALLABLE ::std::vector<::api::Point> beyond(::std::int32_t x) const;
		RUST2_FFI_CALLABLE ::api::buf bytes() const;
		RUST2_FFI_CALLABLE ::std::vector<::api::Label> drain();
	};
	class RUST2_FFI_CONSUMABLE Worker {
		RUST2_FFI_UNCONSUMED Worker(Worker &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Worker &operator=(Worker &&other) noexcept = default;
		static ::api::Worker new_(::std::uint32_t id);
		RUST2_FFI_CALLABLE ::std::uint32_t id() const;
	};
	class RUST2_FFI_CONSUMABLE Handle {
		RUST2_FFI_UNCONSUMED Handle(Handle &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Handle &operator=(Handle &&other) noexcept = default;
		static ::api::Handle new_(::std::uint32_t count);
		RUST2_FFI_CALLABLE ::std::uint32_t count() const;
		static ::api::Handle_later_future later(::std::uint32_t count);
	};
	class RUST2_FFI_CONSUMABLE Holder {
		RUST2_FFI_UNCONSUMED Holder(Holder &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Holder &operator=(Holder &&other) noexcept = default;
		static ::api::Holder new_(::std::uint64_t level);
		[[deprecated("use `Dial`, whose 'level' is \"the same\" at $0\nand past it")]] RUST2_FFI_CALLABLE ::std::uint64_t level_get() const;
		[[deprecated("use `Dial`, whose 'level' is \"the same\" at $0\nand past it")]] RUST2_FFI_CALLABLE ::std::uint64_t doubled() const;
	};
	class RUST2_FFI_CONSUMABLE Tag {
		RUST2_FFI_UNCONSUMED Tag(Tag &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Tag &operator=(Tag &&other) noexcept = default;
		static Tag default_();
		static ::api::Tag new_();
		template <class F0> RUST2_FFI_CALLABLE ::api::buf tagged(::std::uint32_t &n, F0 &&f);
	};
	class RUST2_FFI_CONSUMABLE Counter {
		RUST2_FFI_UNCONSUMED Counter(Counter &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Counter &operator=(Counter &&other) noexcept = default;
		RUST2_FFI_CALLABLE Counter clone() const;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static Counter default_();
		static ::api::Counter new_(::api::input name, ::std::uint64_t start);
		RUST2_FFI_CALLABLE ::std::uint64_t get() const;
		RUST2_FFI_CALLABLE ::std::uint64_t offset(::std::uint64_t handle, ::std::uint64_t h) const;
		RUST2_FFI_CONSUMES ::std::uint64_t into_sum(::std::uint64_t close) &&;
		RUST2_FFI_CALLABLE ::api::buf name() const;
		RUST2_FFI_CALLABLE ::api::result<::std::uint64_t, ::api::Error> add(::std::uint64_t n);
		RUST2_FFI_CALLABLE ::api::result<::std::uint64_t, ::api::Error> sub(::std::uint64_t n);
		RUST2_FFI_CALLABLE bool edited() const;
		RUST2_FFI_CALLABLE ::std::uint64_t plus(const ::api::Counter *other) const;
		RUST2_FFI_CALLABLE ::std::uint64_t clamped(const ::api::Counter *low, const ::api::Counter *high) const;
		RUST2_FFI_CALLABLE ::api::Counter_plus_later_future plus_later(const ::api::Counter *other) const;
		RUST2_FFI_CALLABLE bool add_to(::api::Counter *other) const;
		RUST2_FFI_CONSUMES ::api::Label into_label(const ::std::optional<::api::Point> &at) &&;
		RUST2_FFI_CALLABLE void rename(::api::input name);
		RUST2_FFI_CONSUMES ::std::tuple<::api::Counter, ::api::Counter> split(::std::uint64_t at) &&;
		RUST2_FFI_CALLABLE bool is(::api::input name) const;
		static ::api::Counter seeded_Point(::api::input name, const ::api::Point &from);
		static ::api::Counter seeded_Counter(::api::input name, const ::api::Counter &from);
		static ::api::Counter seeded_Size(::api::input name, const ::api::Size &from);
		static ::api::Counter seeded_Wrap_u32(::api::input name, const ::api::Wrap_u32 &from);
		static ::api::Counter seeded_polar_Point(::api::input name, const ::api::polar_Point &from);
	};
	class RUST2_FFI_CONSUMABLE Meter {
		RUST2_FFI_UNCONSUMED Meter(Meter &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Meter &operator=(Meter &&other) noexcept = default;
		static Meter default_();
		RUST2_FFI_CALLABLE ::std::uint64_t add(::std::uint64_t n) const;
		RUST2_FFI_CALLABLE ::std::uint64_t count() const;
		RUST2_FFI_CALLABLE void reset();
	};
	class RUST2_FFI_CONSUMABLE Meters {
		RUST2_FFI_UNCONSUMED Meters(Meters &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Meters &operator=(Meters &&other) noexcept = default;
		static Meters default_();
		RUST2_FFI_CALLABLE ::api::Meter last_get() const;
		RUST2_FFI_CALLABLE void last_set(::api::Meter value);
		RUST2_FFI_CALLABLE ::std::optional<::api::Meter> spare_get() const;
		RUST2_FFI_CALLABLE void spare_set(::std::optional<::api::Meter> value);
		static ::api::Meters new_();
		RUST2_FFI_CALLABLE ::api::Meter first() const;
		RUST2_FFI_CALLABLE void clear();
	};
	class RUST2_FFI_CONSUMABLE Registry {
		RUST2_FFI_UNCONSUMED Registry(Registry &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Registry &operator=(Registry &&other) noexcept = default;
		static Registry default_();
		RUST2_FFI_CALLABLE ::api::Counter first_get() const;
		RUST2_FFI_CALLABLE void first_set(::api::Counter value);
		static ::api::Registry new_(::api::Counter first);
		RUST2_FFI_CALLABLE ::std::optional<::api::Counter> last() const;
		RUST2_FFI_CALLABLE void push(::api::Counter counter);
		template <class F0> RUST2_FFI_CALLABLE bool edit(F0 &&edit);
	};
	class RUST2_FFI_CONSUMABLE Ledger {
		RUST2_FFI_UNCONSUMED Ledger(Ledger &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Ledger &operator=(Ledger &&other) noexcept = default;
		static Ledger default_();
		static ::api::Ledger new_();
		RUST2_FFI_CALLABLE ::api::result<::api::Draft, ::api::Error> edit();
		RUST2_FFI_CALLABLE ::std::uint32_t published() const;
		RUST2_FFI_CALLABLE ::api::buf title() const;
		RUST2_FFI_CALLABLE void seal();
		RUST2_FFI_CALLABLE ::api::Receipt receipt();
		RUST2_FFI_CALLABLE ::api::Receipt noted();
		RUST2_FFI_CALLABLE ::api::Filed_Receipt file_Receipt(::api::Receipt item);
		RUST2_FFI_CALLABLE ::api::Tab tab();
		RUST2_FFI_CALLABLE ::api::Slip slip();
	};
	class RUST2_FFI_CONSUMABLE Slip {
		RUST2_FFI_UNCONSUMED Slip(Slip &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Slip &operator=(Slip &&other) noexcept = default;
		RUST2_FFI_CALLABLE Slip clone() const;
		RUST2_FFI_CALLABLE ::api::Receipt receipt_get() const;
		RUST2_FFI_CALLABLE void receipt_set(::api::Receipt value);
		RUST2_FFI_CALLABLE ::api::Receipt global_get() const;
		RUST2_FFI_CALLABLE void global_set(::api::Receipt value);
		RUST2_FFI_CALLABLE ::std::uint32_t count() const;
	};
	class RUST2_FFI_CONSUMABLE Sheet {
		RUST2_FFI_UNCONSUMED Sheet(Sheet &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Sheet &operator=(Sheet &&other) noexcept = default;
		RUST2_FFI_CALLABLE Sheet clone() const;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static Sheet default_();
		RUST2_FFI_CALLABLE ::api::buf title_get() const;
		RUST2_FFI_CALLABLE void title_set(::api::input value);
		static ::api::Sheet new_();
		RUST2_FFI_CALLABLE ::std::uint32_t lines() const;
		RUST2_FFI_CALLABLE void write(::api::input line);
		RUST2_FFI_CALLABLE ::api::buf describe() const;
	};
	class RUST2_FFI_CONSUMABLE Name {
		RUST2_FFI_UNCONSUMED Name(Name &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Name &operator=(Name &&other) noexcept = default;
		RUST2_FFI_CALLABLE Name clone() const;
		friend bool operator==(const Name &a__, const Name &b_);
		friend bool operator!=(const Name &a__, const Name &b_);
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static ::api::Name owned(::api::input text);
		RUST2_FFI_CALLABLE ::api::buf as_str() const;
		RUST2_FFI_CALLABLE ::api::Name join(const ::api::Name &other) const;
		static ::api::Name borrowed(::api::input text);
		RUST2_FFI_CALLABLE bool shares(const ::api::Name &other) const;
		RUST2_FFI_CALLABLE void rename(::api::input text);
	};
	class RUST2_FFI_CONSUMABLE Channel {
		RUST2_FFI_UNCONSUMED Channel(Channel &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Channel &operator=(Channel &&other) noexcept = default;
		RUST2_FFI_CALLABLE Channel clone() const;
		static Channel default_();
		static ::api::Channel new_();
		RUST2_FFI_CALLABLE void clear();
		RUST2_FFI_CALLABLE void send(::api::input payload) const;
		RUST2_FFI_CALLABLE ::api::Channel_recv_future recv() const;
		RUST2_FFI_CALLABLE void notify(const ::api::waker &waker_) const;
		RUST2_FFI_CALLABLE ::api::Channel_send_later_future send_later(::api::input payload) const;
		RUST2_FFI_CALLABLE ::api::Channel_tagged_future tagged(::api::input tag) const;
		RUST2_FFI_CALLABLE ::api::Channel_itemized_future itemized(::std::optional<::api::slice<::std::uint32_t>> items_) const;
		RUST2_FFI_CALLABLE ::api::Channel_poll_take_future poll_take() const;
		RUST2_FFI_CALLABLE ::api::Channel_poll_measure_future poll_measure(::std::size_t extra);
		RUST2_FFI_CALLABLE void listen(const ::api::waker &cx) const;
		RUST2_FFI_CALLABLE ::api::Channel_recv_sized_future recv_sized() const;
		RUST2_FFI_CALLABLE ::api::Channel_recv_nonempty_future recv_nonempty() const;
		RUST2_FFI_CALLABLE ::api::Feed feed() const;
		RUST2_FFI_CALLABLE void send_from_thread(::api::input payload) const;
	};
	class RUST2_FFI_CONSUMABLE Gap {
		RUST2_FFI_UNCONSUMED Gap(Gap &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Gap &operator=(Gap &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Gap(const Gap &other) = default;
		Gap &operator=(const Gap &other) = default;
		friend bool operator==(const Gap &a__, const Gap &b_);
		friend bool operator!=(const Gap &a__, const Gap &b_);
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static ::api::Gap new_(::std::uint64_t wide, ::std::uint8_t narrow);
		RUST2_FFI_CALLABLE ::std::uint64_t sum() const;
	};
	class RUST2_FFI_CONSUMABLE Berth {
		RUST2_FFI_UNCONSUMED Berth(Berth &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Berth &operator=(Berth &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Berth(const Berth &other) = default;
		Berth &operator=(const Berth &other) = default;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static ::api::Berth new_(const ::std::optional<::std::uint32_t> &at);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> at() const;
	};
	class RUST2_FFI_CONSUMABLE Mark {
		RUST2_FFI_UNCONSUMED Mark(Mark &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Mark &operator=(Mark &&other) noexcept = default;
		RUST2_FFI_CALLABLE Mark clone() const;
		friend bool operator==(const Mark &a__, const Mark &b_);
		friend bool operator!=(const Mark &a__, const Mark &b_);
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static ::api::Mark of(::std::uint64_t count);
		RUST2_FFI_CALLABLE ::std::uint64_t count() const;
		RUST2_FFI_CALLABLE void add(::std::uint64_t by);
	};
	class RUST2_FFI_CONSUMABLE Hasp {
		RUST2_FFI_UNCONSUMED Hasp(Hasp &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Hasp &operator=(Hasp &&other) noexcept = default;
		static Hasp default_();
		RUST2_FFI_CALLABLE bool set() const;
	};
	class RUST2_FFI_CONSUMABLE Message {
		RUST2_FFI_UNCONSUMED Message(Message &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Message &operator=(Message &&other) noexcept = default;
		RUST2_FFI_CALLABLE Message clone() const;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		RUST2_FFI_CALLABLE ::std::optional<::api::Stamp> stamp_get() const;
		RUST2_FFI_CALLABLE void stamp_set(const ::std::optional<::api::Stamp> &value);
		RUST2_FFI_CALLABLE ::api::buf payload_get() const;
		RUST2_FFI_CALLABLE void payload_set(::api::input value);
		static ::api::Message new_(const ::std::optional<::api::Stamp> &stamp, ::api::input payload);
		RUST2_FFI_CALLABLE bool sent() const;
	};
	class RUST2_FFI_CONSUMABLE Node {
		RUST2_FFI_UNCONSUMED Node(Node &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Node &operator=(Node &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		RUST2_FFI_CALLABLE ::std::uint64_t value_get() const;
		RUST2_FFI_CALLABLE void value_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::optional<::api::Node> next_get() const;
		RUST2_FFI_CALLABLE void next_set(::std::optional<::api::Node> value);
		static ::api::Node new_(::std::uint64_t value, ::std::optional<::api::Node> next);
		RUST2_FFI_CALLABLE void push(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t sum() const;
	};
	class RUST2_FFI_CONSUMABLE Tally {
		RUST2_FFI_UNCONSUMED Tally(Tally &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Tally &operator=(Tally &&other) noexcept = default;
		friend bool operator==(const Tally &a__, const Tally &b_);
		friend bool operator!=(const Tally &a__, const Tally &b_);
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static Tally default_();
		static ::api::Tally new_();
		RUST2_FFI_CALLABLE ::std::uint32_t bump() const;
		RUST2_FFI_CALLABLE ::api::Tally_bump_later_future bump_later() const;
	};
	class RUST2_FFI_CONSUMABLE Tallies {
		RUST2_FFI_UNCONSUMED Tallies(Tallies &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Tallies &operator=(Tallies &&other) noexcept = default;
		static Tallies default_();
		static ::api::Tallies new_();
		RUST2_FFI_CALLABLE ::api::Tally first() const;
	};
	class RUST2_FFI_CONSUMABLE Plain {
		RUST2_FFI_UNCONSUMED Plain(Plain &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Plain &operator=(Plain &&other) noexcept = default;
		friend bool operator==(const Plain &a__, const Plain &b_);
		friend bool operator!=(const Plain &a__, const Plain &b_);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		RUST2_FFI_CALLABLE ::std::uint64_t n_get() const;
		RUST2_FFI_CALLABLE void n_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE bool on_get() const;
		RUST2_FFI_CALLABLE void on_set(bool value);
		static ::api::Plain new_(::std::uint64_t n, bool on);
		RUST2_FFI_CALLABLE ::std::uint64_t count() const;
		RUST2_FFI_CALLABLE void recount(::std::uint64_t n);
		RUST2_FFI_CALLABLE ::std::uint64_t steps() const;
		RUST2_FFI_CALLABLE ::std::uint32_t locals() const;
	};
	class RUST2_FFI_CONSUMABLE Labeled {
		RUST2_FFI_UNCONSUMED Labeled(Labeled &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Labeled &operator=(Labeled &&other) noexcept = default;
		friend bool operator==(const Labeled &a__, const Labeled &b_);
		friend bool operator!=(const Labeled &a__, const Labeled &b_);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static ::api::Labeled new_(::api::input label);
	};
	class RUST2_FFI_CONSUMABLE Gauge {
		RUST2_FFI_UNCONSUMED Gauge(Gauge &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Gauge &operator=(Gauge &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Gauge(const Gauge &other) = default;
		Gauge &operator=(const Gauge &other) = default;
		static ::api::Gauge new_(::std::uint64_t level);
		RUST2_FFI_CALLABLE void set(::std::uint64_t level) const;
		RUST2_FFI_CALLABLE ::std::uint64_t level() const;
		RUST2_FFI_CONSUMES ::std::uint64_t into_level() &&;
		RUST2_FFI_CALLABLE ::api::Gauge_later_future later() const;
		RUST2_FFI_CALLABLE ::api::Gauge_reaches_future reaches(::std::uint64_t this_) const;
	};
	class RUST2_FFI_CONSUMABLE Latch {
		RUST2_FFI_UNCONSUMED Latch(Latch &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Latch &operator=(Latch &&other) noexcept = default;
		RUST2_FFI_CALLABLE Latch clone() const;
		static Latch default_();
		RUST2_FFI_CALLABLE bool set(::std::uint32_t value) const;
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> get() const;
	};
	class RUST2_FFI_CONSUMABLE Token {
		RUST2_FFI_UNCONSUMED Token(Token &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Token &operator=(Token &&other) noexcept = default;
		RUST2_FFI_CALLABLE Token clone() const;
		RUST2_FFI_CALLABLE ::std::uint32_t id_get() const;
		RUST2_FFI_CALLABLE void id_set(::std::uint32_t value);
		static ::api::Token new_(::std::uint32_t id);
		RUST2_FFI_CALLABLE ::std::vector<::api::Token> twice() const;
	};
	class RUST2_FFI_CONSUMABLE Stack {
		RUST2_FFI_UNCONSUMED Stack(Stack &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Stack &operator=(Stack &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static Stack default_();
		static ::api::Stack new_();
		RUST2_FFI_CALLABLE void push(::api::Counter item);
		RUST2_FFI_CALLABLE ::std::optional<::api::Counter> pop();
		RUST2_FFI_CALLABLE ::std::size_t depth() const;
		RUST2_FFI_CALLABLE ::std::uint64_t depth_past_Point(const ::api::Point &from) const;
		RUST2_FFI_CALLABLE ::std::uint64_t depth_past_Counter(const ::api::Counter &from) const;
		RUST2_FFI_CALLABLE ::std::uint64_t depth_past_Size(const ::api::Size &from) const;
		RUST2_FFI_CALLABLE ::std::uint64_t depth_past_Wrap_u32(const ::api::Wrap_u32 &from) const;
		RUST2_FFI_CALLABLE ::std::uint64_t depth_past_polar_Point(const ::api::polar_Point &from) const;
	};
	class RUST2_FFI_CONSUMABLE Refusal {
		RUST2_FFI_UNCONSUMED Refusal(Refusal &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Refusal &operator=(Refusal &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::api::buf to_string() const;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE Fault {
		RUST2_FFI_UNCONSUMED Fault(Fault &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Fault &operator=(Fault &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::api::buf to_string() const;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE c {
		RUST2_FFI_UNCONSUMED c(c &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		c &operator=(c &&other) noexcept = default;
		friend bool operator==(const c &a__, const c &b_);
		friend bool operator!=(const c &a__, const c &b_);
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static ::api::c new_(::std::uint8_t value);
	};
	class RUST2_FFI_CONSUMABLE Quantity {
		RUST2_FFI_UNCONSUMED Quantity(Quantity &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Quantity &operator=(Quantity &&other) noexcept = default;
		friend bool operator==(const Quantity &a__, const Quantity &b_);
		friend bool operator!=(const Quantity &a__, const Quantity &b_);
		friend bool operator<(const Quantity &a__, const Quantity &b_);
		friend bool operator<=(const Quantity &a__, const Quantity &b_);
		friend bool operator>(const Quantity &a__, const Quantity &b_);
		friend bool operator>=(const Quantity &a__, const Quantity &b_);
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static ::api::Quantity new_(::std::int64_t value, char32_t unit);
	};
	class RUST2_FFI_CONSUMABLE Pass {
		RUST2_FFI_UNCONSUMED Pass(Pass &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Pass &operator=(Pass &&other) noexcept = default;
		friend bool operator==(const Pass &a__, const Pass &b_);
		friend bool operator!=(const Pass &a__, const Pass &b_);
		friend bool operator<(const Pass &a__, const Pass &b_);
		friend bool operator<=(const Pass &a__, const Pass &b_);
		friend bool operator>(const Pass &a__, const Pass &b_);
		friend bool operator>=(const Pass &a__, const Pass &b_);
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static ::api::Pass new_(::std::uint32_t number);
		RUST2_FFI_CONSUMES ::std::uint32_t redeem() &&;
	};
	class RUST2_FFI_CONSUMABLE Key {
		RUST2_FFI_UNCONSUMED Key(Key &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Key &operator=(Key &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Key(const Key &other) = default;
		Key &operator=(const Key &other) = default;
		friend bool operator==(const Key &a__, const Key &b_);
		friend bool operator!=(const Key &a__, const Key &b_);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static ::api::Key new_(::api::input name);
		RUST2_FFI_CALLABLE ::std::uint64_t hash_of() const;
	};
	class RUST2_FFI_CONSUMABLE clash_Seed {
		RUST2_FFI_UNCONSUMED clash_Seed(clash_Seed &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		clash_Seed &operator=(clash_Seed &&other) noexcept = default;
		static clash_Seed default_();
		static ::api::clash_Seed default__();
		RUST2_FFI_CALLABLE ::std::uint32_t value() const;
		RUST2_FFI_CALLABLE bool bool_() const;
	};
	class RUST2_FFI_CONSUMABLE clash_Empty_string {
		RUST2_FFI_UNCONSUMED clash_Empty_string(clash_Empty_string &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		clash_Empty_string &operator=(clash_Empty_string &&other) noexcept = default;
		RUST2_FFI_CALLABLE bool is_empty() const;
	};
	class RUST2_FFI_CONSUMABLE clash_Store {
		RUST2_FFI_UNCONSUMED clash_Store(clash_Store &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		clash_Store &operator=(clash_Store &&other) noexcept = default;
		static ::api::clash_Store new_(::std::uint32_t count);
		RUST2_FFI_CALLABLE ::std::uint32_t free_() const;
		RUST2_FFI_CALLABLE ::std::uint32_t close() const;
		RUST2_FFI_CALLABLE void wait() const;
		RUST2_FFI_CALLABLE void finalize() const;
		RUST2_FFI_CALLABLE ::std::uint32_t take(const ::api::Ticket &ticket) const;
		static ::std::uint32_t take_(const ::api::Ticket &ticket);
		static ::std::uint32_t count_();
		RUST2_FFI_CALLABLE ::std::uint32_t count() const;
		static ::api::clash_Store open_();
		RUST2_FFI_CALLABLE ::std::uint32_t open() const;
	};
	class RUST2_FFI_CONSUMABLE clash_Reader {
		RUST2_FFI_UNCONSUMED clash_Reader(clash_Reader &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		clash_Reader &operator=(clash_Reader &&other) noexcept = default;
		static ::api::clash_Reader new_(::api::input bytes);
		RUST2_FFI_CALLABLE ::api::result<::std::uint8_t, ::api::clash_Malformed> read_byte();
		RUST2_FFI_CALLABLE ::api::result<void, ::api::clash_Malformed> unread_byte();
		RUST2_FFI_CALLABLE void wait(::std::uint64_t ms) const;
	};
	class RUST2_FFI_CONSUMABLE Spawned {
		RUST2_FFI_UNCONSUMED Spawned(Spawned &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Spawned &operator=(Spawned &&other) noexcept = default;
		template <class F0> static ::api::Spawned new_(F0 &&task_);
		RUST2_FFI_CONSUMES ::std::uint32_t join() &&;
	};
	class RUST2_FFI_CONSUMABLE LogThread {
		RUST2_FFI_UNCONSUMED LogThread(LogThread &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		LogThread &operator=(LogThread &&other) noexcept = default;
		static ::api::LogThread new_(::std::unique_ptr<::api::Log> log, ::api::input line);
		RUST2_FFI_CONSUMES bool join() &&;
	};
	class RUST2_FFI_CONSUMABLE Pinned {
		RUST2_FFI_UNCONSUMED Pinned(Pinned &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Pinned &operator=(Pinned &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::std::uint64_t value_get() const;
		RUST2_FFI_CALLABLE void value_set(::std::uint64_t value);
		static ::api::Pinned new_(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::api::Part part() const;
	};
	class RUST2_FFI_CONSUMABLE Guarded {
		RUST2_FFI_UNCONSUMED Guarded(Guarded &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Guarded &operator=(Guarded &&other) noexcept = default;
		static ::api::Guarded new_(::std::uint64_t n);
		RUST2_FFI_CALLABLE ::std::uint64_t n_get() const;
		RUST2_FFI_CALLABLE bool on_get() const;
		RUST2_FFI_CALLABLE ::std::uint64_t count() const;
		RUST2_FFI_CALLABLE ::std::uint64_t steps() const;
		RUST2_FFI_CALLABLE ::std::uint32_t locals() const;
	};
	class RUST2_FFI_CONSUMABLE Hooked {
		RUST2_FFI_UNCONSUMED Hooked(Hooked &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Hooked &operator=(Hooked &&other) noexcept = default;
		friend bool operator==(const Hooked &a__, const Hooked &b_);
		friend bool operator!=(const Hooked &a__, const Hooked &b_);
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		RUST2_FFI_CALLABLE ::std::uint64_t n_get() const;
		RUST2_FFI_CALLABLE void n_set(::std::uint64_t value);
		static ::api::Hooked new_(::std::uint64_t n);
		RUST2_FFI_CALLABLE ::std::uint64_t count() const;
	};
	class RUST2_FFI_CONSUMABLE Forged {
		RUST2_FFI_UNCONSUMED Forged(Forged &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Forged &operator=(Forged &&other) noexcept = default;
		friend bool operator==(const Forged &a__, const Forged &b_);
		friend bool operator!=(const Forged &a__, const Forged &b_);
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		RUST2_FFI_CALLABLE ::std::uint64_t n_get() const;
		RUST2_FFI_CALLABLE void n_set(::std::uint64_t value);
		static ::api::Forged new_(::std::uint64_t n);
	};
	class RUST2_FFI_CONSUMABLE Part {
		RUST2_FFI_UNCONSUMED Part(Part &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Part &operator=(Part &&other) noexcept = default;
		friend bool operator==(const Part &a__, const Part &b_);
		friend bool operator!=(const Part &a__, const Part &b_);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		RUST2_FFI_CALLABLE ::std::uint64_t value_get() const;
		RUST2_FFI_CALLABLE void value_set(::std::uint64_t value);
	};
	class Log {
		virtual ~Log() = default;
		virtual bool log(::std::uint8_t level, ::api::view line) const = 0;
	};
	class Scan {
		virtual ~Scan() = default;
		virtual ::std::uint64_t scan(::api::view payload, ::std::optional<::api::view> rest) const = 0;
	};
	class Beat {
		virtual ~Beat() = default;
		virtual ::std::uint32_t beat() const = 0;
	};
	class Turner {
		virtual ~Turner() = default;
		virtual ::std::uint64_t by() const = 0;
		[[deprecated("turn `Turner::by` instead")]] virtual ::std::uint64_t once() const = 0;
	};
	class Undo {
		virtual ~Undo() = default;
		virtual ::std::uint32_t delete_(::std::uint32_t n) const = 0;
		virtual ::std::uint32_t free() const = 0;
		virtual ::std::uint8_t type() const = 0;
	};
	class Stretch {
		virtual ~Stretch() = default;
		virtual ::std::size_t stretch() const = 0;
	};
	class clash_Source {
		virtual ~clash_Source() = default;
		virtual void wait() const = 0;
		virtual ::std::uint8_t read_byte() const = 0;
	};
	class clash_Eraser {
		virtual ~clash_Eraser() = default;
		virtual ::std::uint8_t delete_(::std::uint8_t new_, ::std::uint8_t new__) const = 0;
		virtual ::std::uint8_t delete__() const = 0;
	};
	struct Parcel {
		::api::Mark mark;
		::api::buf payload;
	};
	class Score {
		::api::buf debug() const;
		static ::api::Score of(::std::uint64_t count);
		::std::uint64_t count() const;
		void add(const ::api::Score &other);
		::api::Score doubled() const;
		::std::uint64_t take();
		::std::uint64_t note(const ::api::Mark &mark);
		::std::tuple<::std::uint64_t, ::std::uint64_t> absorb(::api::Score &other);
	};
	struct Weighted {
		::api::Score score;
		::std::uint32_t weight;
		::std::uint64_t total() const;
	};
	class Stamp {
		friend bool operator==(const Stamp &a__, const Stamp &b_);
		friend bool operator!=(const Stamp &a__, const Stamp &b_);
		friend bool operator<(const Stamp &a__, const Stamp &b_);
		friend bool operator<=(const Stamp &a__, const Stamp &b_);
		friend bool operator>(const Stamp &a__, const Stamp &b_);
		friend bool operator>=(const Stamp &a__, const Stamp &b_);
		::api::buf debug() const;
		static Stamp default_();
		static ::api::Stamp from_millis(::std::uint64_t millis);
		::std::uint64_t secs() const;
		::api::Stamp whole() const;
		bool later(const ::api::Stamp &other) const;
	};
	struct Stamped {
		::api::Stamp at;
		::std::uint32_t count;
		::std::uint64_t total() const;
	};
	struct CancellationError {
		::std::uint32_t code;
	};
	struct Packet {
		::std::uint32_t seq;
		::api::buf payload;
	};
	struct Hasher {
		::std::uint8_t seed;
	};
	struct Point {
		::std::int32_t x;
		::std::int32_t y;
		friend bool operator==(const Point &a__, const Point &b_);
		friend bool operator!=(const Point &a__, const Point &b_);
		::api::buf debug() const;
		static Point default_();
		::api::Point offset(::std::int32_t dx, ::std::int32_t dy) const;
		static ::std::int32_t get_x();
	};
	struct Label {
		::api::buf name;
		::std::optional<::api::Point> at;
		::std::optional<::api::duration> ttl;
		friend bool operator==(const Label &a__, const Label &b_);
		friend bool operator!=(const Label &a__, const Label &b_);
		::api::buf debug() const;
	};
	struct Note {
		::std::optional<::api::buf> text;
		friend bool operator==(const Note &a__, const Note &b_);
		friend bool operator!=(const Note &a__, const Note &b_);
		::api::buf debug() const;
		static Note default_();
	};
	struct Reading {
		bool ok;
		float level;
		double ratio;
		char32_t mark;
		::api::Shape shape;
		::api::Point at;
		::api::buf note;
		::std::uint64_t count;
		::std::int8_t offset;
		::std::size_t len;
		::std::uint16_t tag;
		friend bool operator==(const Reading &a__, const Reading &b_);
		friend bool operator!=(const Reading &a__, const Reading &b_);
		::api::buf debug() const;
		::std::optional<::api::Reading> next() const;
	};
	struct Packaged {
		::std::uint32_t api;
		friend bool operator==(const Packaged &a__, const Packaged &b_);
		friend bool operator!=(const Packaged &a__, const Packaged &b_);
		::api::buf debug() const;
		::api::Point point() const;
		::api::Shape shape() const;
	};
	struct Atlas {
		::std::vector<::std::tuple<::api::buf, ::api::Point>> points;
		::std::vector<::std::uint32_t> ids;
		friend bool operator==(const Atlas &a__, const Atlas &b_);
		friend bool operator!=(const Atlas &a__, const Atlas &b_);
		::api::buf debug() const;
		::api::Atlas with(::api::input name, const ::api::Point &at) const;
	};
	struct Span {
		::std::tuple<::std::uint32_t, ::std::uint32_t> bounds;
		::std::optional<::std::tuple<::api::buf, ::api::Shape>> named;
		friend bool operator==(const Span &a__, const Span &b_);
		friend bool operator!=(const Span &a__, const Span &b_);
		::api::buf debug() const;
	};
	struct Window {
		::api::range_u64 range;
		::api::bound_u64 end;
		friend bool operator==(const Window &a__, const Window &b_);
		friend bool operator!=(const Window &a__, const Window &b_);
		::api::buf debug() const;
		bool contains(::std::uint64_t n) const;
	};
	class Ticket {
		friend bool operator==(const Ticket &a__, const Ticket &b_);
		friend bool operator!=(const Ticket &a__, const Ticket &b_);
		::api::buf debug() const;
		static ::api::Ticket new_(::std::uint32_t number);
		::std::uint32_t number() const;
		void wait() const;
		static ::std::uint32_t clones();
	};
	struct Seat {
		::api::Mark mark;
		::std::uint32_t row;
		friend bool operator==(const Seat &a__, const Seat &b_);
		friend bool operator!=(const Seat &a__, const Seat &b_);
		::api::buf debug() const;
		::std::uint64_t count() const;
	};
	class Moment {
		friend bool operator==(const Moment &a__, const Moment &b_);
		friend bool operator!=(const Moment &a__, const Moment &b_);
		::api::buf debug() const;
		static ::std::optional<::api::Moment> new_(::std::uint64_t ticks, ::std::uint64_t rate);
		::std::uint64_t millis() const;
		::api::Moment after(::std::uint64_t ticks) const;
	};
	class LongArray {
		friend bool operator==(const LongArray &a__, const LongArray &b_);
		friend bool operator!=(const LongArray &a__, const LongArray &b_);
		::api::buf debug() const;
		static ::api::LongArray new_(::std::uint64_t low, ::std::uint64_t high);
		::std::uint64_t c__() const;
		template <class F0> void also(F0 &&f) const;
	};
	struct JvmField {
		::std::uint32_t n;
		friend bool operator==(const JvmField &a__, const JvmField &b_);
		friend bool operator!=(const JvmField &a__, const JvmField &b_);
		::api::buf debug() const;
	};
	struct JvmStatic {
		::std::uint32_t n;
		friend bool operator==(const JvmStatic &a__, const JvmStatic &b_);
		friend bool operator!=(const JvmStatic &a__, const JvmStatic &b_);
		::api::buf debug() const;
	};
	struct OptIn {
		::std::uint32_t n;
		friend bool operator==(const OptIn &a__, const OptIn &b_);
		friend bool operator!=(const OptIn &a__, const OptIn &b_);
		::api::buf debug() const;
	};
	class Wide {
		friend bool operator==(const Wide &a__, const Wide &b_);
		friend bool operator!=(const Wide &a__, const Wide &b_);
		::api::buf debug() const;
		static ::api::Wide new_(::std::uint64_t seed);
		::std::uint64_t sum() const;
	};
	struct Clocked {
		::api::Moment at;
		::std::uint32_t count;
		friend bool operator==(const Clocked &a__, const Clocked &b_);
		friend bool operator!=(const Clocked &a__, const Clocked &b_);
		::api::buf debug() const;
	};
	class Rated {
		friend bool operator==(const Rated &a__, const Rated &b_);
		friend bool operator!=(const Rated &a__, const Rated &b_);
		::api::buf debug() const;
		static ::api::Rated new_(::std::uint64_t count, ::std::uint64_t scale);
		::std::uint64_t scale() const;
	};
	class Hits {
		friend bool operator==(const Hits &a__, const Hits &b_);
		friend bool operator!=(const Hits &a__, const Hits &b_);
		::api::buf debug() const;
		static Hits default_();
		::std::uint64_t hits_get() const;
		void hits_set(::std::uint64_t value);
		::std::uint64_t misses_get() const;
		void misses_set(::std::uint64_t value);
		static ::api::Hits of(::std::uint64_t hits, ::std::uint64_t misses);
		void add(const ::api::Hits &other);
		bool eq_(const ::api::Hits &other);
	};
	class Regs {
		friend bool operator==(const Regs &a__, const Regs &b_);
		friend bool operator!=(const Regs &a__, const Regs &b_);
		::api::buf debug() const;
		::std::uint16_t fcw_get() const;
		void fcw_set(::std::uint16_t value);
		::std::uint64_t wide_get() const;
		void wide_set(::std::uint64_t value);
		::api::Point at_get() const;
		void at_set(const ::api::Point &value);
		::std::uint8_t tag_get() const;
		void tag_set(::std::uint8_t value);
		static ::api::Regs new_(::std::uint16_t fcw, ::std::uint64_t wide, const ::api::Point &at);
		::std::uint8_t seed() const;
	};
	class Extent {
		friend bool operator==(const Extent &a__, const Extent &b_);
		friend bool operator!=(const Extent &a__, const Extent &b_);
		::api::buf debug() const;
		::std::uint64_t start_get() const;
		void start_set(::std::uint64_t value);
		static ::api::Extent new_(::std::uint64_t start, ::std::uint64_t len);
		::std::uint64_t end() const;
		::std::uint64_t slide(::std::uint64_t by);
	};
	struct Pair_Size_u8 {
		::api::Size left;
		::std::uint8_t right;
		friend bool operator==(const Pair_Size_u8 &a__, const Pair_Size_u8 &b_);
		friend bool operator!=(const Pair_Size_u8 &a__, const Pair_Size_u8 &b_);
		::api::buf debug() const;
		::api::Pair_u8_Size swap() const;
	};
	struct Pair_non_zero_u16 {
		::std::uint16_t left;
		::std::uint16_t right;
		friend bool operator==(const Pair_non_zero_u16 &a__, const Pair_non_zero_u16 &b_);
		friend bool operator!=(const Pair_non_zero_u16 &a__, const Pair_non_zero_u16 &b_);
		::api::buf debug() const;
		::api::Pair_non_zero_u16 swap() const;
		bool even() const;
	};
	struct Pair_u32 {
		::std::uint32_t left;
		::std::uint32_t right;
		friend bool operator==(const Pair_u32 &a__, const Pair_u32 &b_);
		friend bool operator!=(const Pair_u32 &a__, const Pair_u32 &b_);
		::api::buf debug() const;
		::api::Pair_u32 swap() const;
		bool even() const;
		::std::uint32_t sum() const;
		::api::Pair_u8_Size beside_Size(const ::api::Size &right_) const;
	};
	struct Pair_u8_Size {
		::std::uint8_t left;
		::api::Size right;
		friend bool operator==(const Pair_u8_Size &a__, const Pair_u8_Size &b_);
		friend bool operator!=(const Pair_u8_Size &a__, const Pair_u8_Size &b_);
		::api::buf debug() const;
		::api::Pair_Size_u8 swap() const;
	};
	struct Right_u8 {
		::std::uint8_t value;
		friend bool operator==(const Right_u8 &a__, const Right_u8 &b_);
		friend bool operator!=(const Right_u8 &a__, const Right_u8 &b_);
		::api::buf debug() const;
	};
	struct Left_Right_u8 {
		::api::Right_u8 value;
		friend bool operator==(const Left_Right_u8 &a__, const Left_Right_u8 &b_);
		friend bool operator!=(const Left_Right_u8 &a__, const Left_Right_u8 &b_);
		::api::buf debug() const;
	};
	struct Left_u8 {
		::std::uint8_t value;
		friend bool operator==(const Left_u8 &a__, const Left_u8 &b_);
		friend bool operator!=(const Left_u8 &a__, const Left_u8 &b_);
		::api::buf debug() const;
	};
	struct Right_Left_u8 {
		::api::Left_u8 value;
		friend bool operator==(const Right_Left_u8 &a__, const Right_Left_u8 &b_);
		friend bool operator!=(const Right_Left_u8 &a__, const Right_Left_u8 &b_);
		::api::buf debug() const;
	};
	struct Inner_u8 {
		::std::uint8_t value;
		friend bool operator==(const Inner_u8 &a__, const Inner_u8 &b_);
		friend bool operator!=(const Inner_u8 &a__, const Inner_u8 &b_);
		::api::buf debug() const;
	};
	struct Boxed_u16 {
		::std::uint16_t value;
		friend bool operator==(const Boxed_u16 &a__, const Boxed_u16 &b_);
		friend bool operator!=(const Boxed_u16 &a__, const Boxed_u16 &b_);
		::api::buf debug() const;
	};
	struct Boxed_u8 {
		::std::uint8_t value;
		friend bool operator==(const Boxed_u8 &a__, const Boxed_u8 &b_);
		friend bool operator!=(const Boxed_u8 &a__, const Boxed_u8 &b_);
		::api::buf debug() const;
	};
	struct LogCount {
		::std::uint32_t kept;
		friend bool operator==(const LogCount &a__, const LogCount &b_);
		friend bool operator!=(const LogCount &a__, const LogCount &b_);
		::api::buf debug() const;
	};
	struct Wrap_u32 {
		::std::uint32_t value;
		friend bool operator==(const Wrap_u32 &a__, const Wrap_u32 &b_);
		friend bool operator!=(const Wrap_u32 &a__, const Wrap_u32 &b_);
		::api::buf debug() const;
	};
	struct PolarTurn {
		::std::uint8_t whole;
		friend bool operator==(const PolarTurn &a__, const PolarTurn &b_);
		friend bool operator!=(const PolarTurn &a__, const PolarTurn &b_);
		::api::buf debug() const;
	};
	struct Peer {
		::api::socket_addr addr;
		::std::vector<::api::buf> names;
		friend bool operator==(const Peer &a__, const Peer &b_);
		friend bool operator!=(const Peer &a__, const Peer &b_);
		::api::buf debug() const;
		::std::vector<::api::buf> aliases() const;
		::api::Peer next(::api::input name) const;
	};
	struct Shadowed {
		::std::vector<::std::int32_t> other;
		::std::uint8_t a;
		::std::uint8_t b;
		friend bool operator==(const Shadowed &a__, const Shadowed &b_);
		friend bool operator!=(const Shadowed &a__, const Shadowed &b_);
		::api::buf debug() const;
	};
	struct Numbers {
		::std::vector<::std::uint16_t> u16s;
		::std::vector<::std::uint32_t> u32s;
		::std::vector<::std::uint64_t> u64s;
		::std::vector<::std::int8_t> i8s;
		::std::vector<::std::int16_t> i16s;
		::std::vector<::std::int32_t> i32s;
		::std::vector<::std::int64_t> i64s;
		::std::vector<float> f32s;
		::std::vector<double> f64s;
		::std::optional<::std::vector<::std::int32_t>> some;
		friend bool operator==(const Numbers &a__, const Numbers &b_);
		friend bool operator!=(const Numbers &a__, const Numbers &b_);
		::api::buf debug() const;
		static ::api::Numbers edges();
		static ::api::Numbers zero(bool negative);
		::api::Numbers reversed() const;
	};
	struct Measure {
		double value;
		::api::Numbers numbers;
		friend bool operator==(const Measure &a__, const Measure &b_);
		friend bool operator!=(const Measure &a__, const Measure &b_);
		::api::buf debug() const;
		static ::api::Measure new_(double value_, bool negative);
	};
	struct Limits_bytes {
		::api::buf low;
		::api::buf high;
		friend bool operator==(const Limits_bytes &a__, const Limits_bytes &b_);
		friend bool operator!=(const Limits_bytes &a__, const Limits_bytes &b_);
		::api::buf debug() const;
	};
	struct Limits_duration {
		::api::duration low;
		::api::duration high;
		friend bool operator==(const Limits_duration &a__, const Limits_duration &b_);
		friend bool operator!=(const Limits_duration &a__, const Limits_duration &b_);
		::api::buf debug() const;
	};
	struct Page {
		::std::uint32_t size;
		friend bool operator==(const Page &a__, const Page &b_);
		friend bool operator!=(const Page &a__, const Page &b_);
		::api::buf debug() const;
		static Page default_();
	};
	struct Booth {
		::api::Mark mark;
		friend bool operator==(const Booth &a__, const Booth &b_);
		friend bool operator!=(const Booth &a__, const Booth &b_);
		::api::buf debug() const;
	};
	struct Cancelled {
		::std::uint8_t step;
		friend bool operator==(const Cancelled &a__, const Cancelled &b_);
		friend bool operator!=(const Cancelled &a__, const Cancelled &b_);
		::api::buf debug() const;
	};
	struct StartSeekable {
		::std::uint64_t at;
		friend bool operator==(const StartSeekable &a__, const StartSeekable &b_);
		friend bool operator!=(const StartSeekable &a__, const StartSeekable &b_);
		::api::buf debug() const;
	};
	struct DoubleArray {
		::std::uint32_t len;
		friend bool operator==(const DoubleArray &a__, const DoubleArray &b_);
		friend bool operator!=(const DoubleArray &a__, const DoubleArray &b_);
		::api::buf debug() const;
	};
	struct NonZeros {
		::std::uint8_t u8;
		::std::uint16_t u16;
		::std::uint32_t u32;
		::std::uint64_t u64;
		::std::size_t usize;
		::std::int8_t i8;
		::std::int16_t i16;
		::std::int32_t i32;
		::std::int64_t i64;
		::std::ptrdiff_t isize;
		::api::buf debug() const;
	};
	struct a {
		::std::uint8_t b;
		friend bool operator==(const a &a__, const a &b_);
		friend bool operator!=(const a &a__, const a &b_);
		::api::buf debug() const;
	};
	struct Qualifiers {
		double api;
		double kotlin;
		friend bool operator==(const Qualifiers &a__, const Qualifiers &b_);
		friend bool operator!=(const Qualifiers &a__, const Qualifiers &b_);
	};
	struct Version {
		::std::uint16_t major;
		::std::uint16_t minor;
		friend bool operator==(const Version &a__, const Version &b_);
		friend bool operator!=(const Version &a__, const Version &b_);
		friend bool operator<(const Version &a__, const Version &b_);
		friend bool operator<=(const Version &a__, const Version &b_);
		friend bool operator>(const Version &a__, const Version &b_);
		friend bool operator>=(const Version &a__, const Version &b_);
		::api::buf debug() const;
	};
	struct Spot {
		::std::uint32_t x;
		::std::uint32_t y;
		friend bool operator==(const Spot &a__, const Spot &b_);
		friend bool operator!=(const Spot &a__, const Spot &b_);
		::api::buf debug() const;
		static ::api::result<::api::Spot, ::api::buf> deserialize(::api::Json deserializer);
	};
	struct Deprecated {
		bool on;
		[[deprecated]] ::std::uint32_t lower;
		bool deprecated;
		friend bool operator==(const Deprecated &a__, const Deprecated &b_);
		friend bool operator!=(const Deprecated &a__, const Deprecated &b_);
		::api::buf debug() const;
		static Deprecated default_();
	};
	struct Dial {
		::std::uint64_t level;
		[[deprecated("keep the `Dial` from before the turn")]] ::std::uint64_t before;
		friend bool operator==(const Dial &a__, const Dial &b_);
		friend bool operator!=(const Dial &a__, const Dial &b_);
		::api::buf debug() const;
		static Dial default_();
		::api::result<::api::Dial, ::api::Error> turn(::std::uint64_t by) const;
		[[deprecated("turn it by `LIMIT` with `Dial::turn`")]] ::api::Dial max() const;
	};
	struct [[deprecated("use `Dial`, whose 'level' is \"the same\" at $0\nand past it")]] Knob {
		::std::uint64_t level;
		friend bool operator==(const Knob &a__, const Knob &b_);
		friend bool operator!=(const Knob &a__, const Knob &b_);
		::api::buf debug() const;
		static Knob default_();
		::std::uint64_t doubled() const;
	};
	struct polar_Point {
		::std::uint8_t turns;
		::std::uint32_t distance;
		friend bool operator==(const polar_Point &a__, const polar_Point &b_);
		friend bool operator!=(const polar_Point &a__, const polar_Point &b_);
		::api::buf debug() const;
	};
	struct polar_Turn {
		::std::uint8_t part;
		friend bool operator==(const polar_Turn &a__, const polar_Turn &b_);
		friend bool operator!=(const polar_Turn &a__, const polar_Turn &b_);
		::api::buf debug() const;
	};
	struct macros_Streams {
		::std::uint8_t stdin_;
		::std::uint8_t stdout_;
		::std::uint8_t stderr_;
		::std::uint8_t errno_;
		friend bool operator==(const macros_Streams &a__, const macros_Streams &b_);
		friend bool operator!=(const macros_Streams &a__, const macros_Streams &b_);
		::api::buf debug() const;
	};
	struct clash_Tone {
		::std::uint16_t hz;
		friend bool operator==(const clash_Tone &a__, const clash_Tone &b_);
		friend bool operator!=(const clash_Tone &a__, const clash_Tone &b_);
		::api::buf debug() const;
	};
	struct clash_TONE {
		::std::uint8_t db;
		friend bool operator==(const clash_TONE &a__, const clash_TONE &b_);
		friend bool operator!=(const clash_TONE &a__, const clash_TONE &b_);
		::api::buf debug() const;
	};
	struct clash_Span {
		::std::uint32_t width;
		friend bool operator==(const clash_Span &a__, const clash_Span &b_);
		friend bool operator!=(const clash_Span &a__, const clash_Span &b_);
		::api::buf to_string() const;
		::api::buf debug() const;
		::std::uint32_t width_() const;
		::api::buf to_string_() const;
	};
	struct clash_Any {
		::api::buf type_url;
		friend bool operator==(const clash_Any &a__, const clash_Any &b_);
		friend bool operator!=(const clash_Any &a__, const clash_Any &b_);
		::api::buf debug() const;
	};
	struct clash_Break {
		::std::uint8_t _0;
		friend bool operator==(const clash_Break &a__, const clash_Break &b_);
		friend bool operator!=(const clash_Break &a__, const clash_Break &b_);
		::api::buf debug() const;
		static ::api::clash_Break Sp();
		static ::api::clash_Break SP();
	};
	struct clash_Malformed {
		::std::uint8_t at;
		friend bool operator==(const clash_Malformed &a__, const clash_Malformed &b_);
		friend bool operator!=(const clash_Malformed &a__, const clash_Malformed &b_);
		::api::buf to_string() const;
		::api::buf debug() const;
		bool is(::std::uint8_t at_) const;
	};
	struct clash_Described {
		::std::uint8_t description;
		::std::uint8_t debug_description;
		friend bool operator==(const clash_Described &a__, const clash_Described &b_);
		friend bool operator!=(const clash_Described &a__, const clash_Described &b_);
		::api::buf to_string() const;
		::api::buf debug() const;
	};
	struct clash_Errno {
		::std::int32_t errno_;
		friend bool operator==(const clash_Errno &a__, const clash_Errno &b_);
		friend bool operator!=(const clash_Errno &a__, const clash_Errno &b_);
		::api::buf debug() const;
		static ::api::clash_Errno NULL_();
	};
	struct clash_Escaped {
		::std::uint8_t new_;
		friend bool operator==(const clash_Escaped &a__, const clash_Escaped &b_);
		friend bool operator!=(const clash_Escaped &a__, const clash_Escaped &b_);
		::api::buf debug() const;
		::std::uint8_t delete_(::std::uint8_t new__, ::std::uint8_t new___) const;
		::std::uint8_t delete__() const;
	};
	struct clash_Underscored {
		::std::uint8_t new_;
		::std::uint8_t new__;
		::std::uint8_t debug_;
		::std::uint8_t default__;
		friend bool operator==(const clash_Underscored &a__, const clash_Underscored &b_);
		friend bool operator!=(const clash_Underscored &a__, const clash_Underscored &b_);
		::api::buf debug() const;
		static clash_Underscored default_();
	};
	struct Core {
		::std::uint32_t id;
	};
	struct Choice_Empty {
	};
	struct Choice_Picked {
		::api::Mark _0;
	};
	struct Array_U8x4 {
		::std::uint8_t _0;
	};
	struct Platform_Wasm {
		::std::uint32_t bits;
	};
	struct Platform_Native {
		::std::uint32_t _0;
	};
	struct Placement_Anywhere {
	};
	struct Placement_Pinned {
		::api::Core _0;
	};
	struct Event_Moved {
		::api::Point _0;
	};
	struct Event_Named {
		::api::buf name;
	};
	struct Event_Done {
	};
	struct Clash_Point {
		::api::Point _0;
	};
	struct Clash_Shape {
		::api::Shape _0;
	};
	struct Clash_Event {
		::api::Event _0;
	};
	struct Clash_Double {
		double _0;
	};
	struct Clash_Float {
		float _0;
	};
	struct Clash_String {
		::api::buf _0;
	};
	struct Clash_Int {
		::std::int32_t _0;
	};
	struct Clash_Boolean {
		bool _0;
	};
	struct Clash_Native {
		::std::uint32_t _0;
	};
	struct Clash_Bytes {
		::api::buf _0;
	};
	struct Clash_Any {
	};
	struct Clash_Unit {
	};
	struct Clash_Waker {
	};
	struct Clash_Mut {
	};
	struct Clash_Pair {
		::std::uint8_t _0;
		::std::uint8_t _1;
	};
	struct Clash_Stamp {
		::std::int64_t _0;
	};
	struct Clash_Moment {
		::api::Moment _0;
	};
	struct Stage_Init {
		::std::uint32_t _0;
	};
	struct Stage_Done {
	};
	struct Error_Underflow {
		::std::uint64_t by;
	};
	struct Error_Overflow {
	};
	struct Labelled_Tagged {
		::std::uint32_t tag;
	};
	struct Labelled_Bare {
	};
	struct b_One {
	};
	struct b_Two {
		::std::uint8_t _0;
	};
	struct Builtin_Any {
		::std::vector<::std::int32_t> _0;
	};
	struct Builtin_Boolean {
		double _0;
	};
	struct Builtin_Int {
		double _0;
	};
	struct Builtin_Native {
		double _0;
	};
	struct Rank_Unranked {
	};
	struct Rank_Ranked {
		::std::uint8_t compare;
		::std::uint8_t partial_compare;
		::std::uint8_t compare_to;
		::std::uint8_t partial_compare_to;
	};
	struct clash_Candidate_None {
	};
	struct clash_Candidate_Match {
		::std::uint8_t _0;
	};
	struct clash_Tag_Application {
		::std::uint32_t number;
	};
	struct clash_Tag_Private {
		::std::uint32_t number;
	};
	struct clash_Change_Set {
		::std::uint8_t new_;
		::std::uint8_t new__;
	};
	struct clash_Change_Clear {
	};
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::partial_ordering operator<=>(const Quantity &a__, const Quantity &b_);
	#endif
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const Pass &a__, const Pass &b_);
	#endif
	inline bool operator==(const Stamp &a__, const Stamp &b_);
	inline bool operator<(const Stamp &a__, const Stamp &b_);
	inline bool operator<=(const Stamp &a__, const Stamp &b_);
	inline bool operator>(const Stamp &a__, const Stamp &b_);
	inline bool operator>=(const Stamp &a__, const Stamp &b_);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const Stamp &a__, const Stamp &b_);
	#endif
	inline bool operator==(const Point &a__, const Point &b_);
	inline bool operator==(const Label &a__, const Label &b_);
	inline bool operator==(const Note &a__, const Note &b_);
	inline bool operator==(const Reading &a__, const Reading &b_);
	inline bool operator==(const Packaged &a__, const Packaged &b_);
	inline bool operator==(const Atlas &a__, const Atlas &b_);
	inline bool operator==(const Span &a__, const Span &b_);
	inline bool operator==(const Window &a__, const Window &b_);
	inline bool operator==(const Ticket &a__, const Ticket &b_);
	inline bool operator==(const Seat &a__, const Seat &b_);
	inline bool operator==(const Moment &a__, const Moment &b_);
	inline bool operator==(const LongArray &a__, const LongArray &b_);
	inline bool operator==(const JvmField &a__, const JvmField &b_);
	inline bool operator==(const JvmStatic &a__, const JvmStatic &b_);
	inline bool operator==(const OptIn &a__, const OptIn &b_);
	inline bool operator==(const Wide &a__, const Wide &b_);
	inline bool operator==(const Clocked &a__, const Clocked &b_);
	inline bool operator==(const Rated &a__, const Rated &b_);
	inline bool operator==(const Hits &a__, const Hits &b_);
	inline bool operator==(const Regs &a__, const Regs &b_);
	inline bool operator==(const Extent &a__, const Extent &b_);
	inline bool operator==(const Pair_Size_u8 &a__, const Pair_Size_u8 &b_);
	inline bool operator==(const Pair_non_zero_u16 &a__, const Pair_non_zero_u16 &b_);
	inline bool operator==(const Pair_u32 &a__, const Pair_u32 &b_);
	inline bool operator==(const Pair_u8_Size &a__, const Pair_u8_Size &b_);
	inline bool operator==(const Right_u8 &a__, const Right_u8 &b_);
	inline bool operator==(const Left_Right_u8 &a__, const Left_Right_u8 &b_);
	inline bool operator==(const Left_u8 &a__, const Left_u8 &b_);
	inline bool operator==(const Right_Left_u8 &a__, const Right_Left_u8 &b_);
	inline bool operator==(const Inner_u8 &a__, const Inner_u8 &b_);
	inline bool operator==(const Boxed_u16 &a__, const Boxed_u16 &b_);
	inline bool operator==(const Boxed_u8 &a__, const Boxed_u8 &b_);
	inline bool operator==(const LogCount &a__, const LogCount &b_);
	inline bool operator==(const Wrap_u32 &a__, const Wrap_u32 &b_);
	inline bool operator==(const PolarTurn &a__, const PolarTurn &b_);
	inline bool operator==(const Peer &a__, const Peer &b_);
	inline bool operator==(const Shadowed &a__, const Shadowed &b_);
	inline bool operator==(const Numbers &a__, const Numbers &b_);
	inline bool operator==(const Measure &a__, const Measure &b_);
	inline bool operator==(const Limits_bytes &a__, const Limits_bytes &b_);
	inline bool operator==(const Limits_duration &a__, const Limits_duration &b_);
	inline bool operator==(const Page &a__, const Page &b_);
	inline bool operator==(const Booth &a__, const Booth &b_);
	inline bool operator==(const Cancelled &a__, const Cancelled &b_);
	inline bool operator==(const StartSeekable &a__, const StartSeekable &b_);
	inline bool operator==(const DoubleArray &a__, const DoubleArray &b_);
	inline bool operator==(const a &a__, const a &b_);
	inline bool operator==(const Qualifiers &a__, const Qualifiers &b_);
	inline bool operator==(const Version &a__, const Version &b_);
	inline bool operator<(const Version &a__, const Version &b_);
	inline bool operator<=(const Version &a__, const Version &b_);
	inline bool operator>(const Version &a__, const Version &b_);
	inline bool operator>=(const Version &a__, const Version &b_);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const Version &a__, const Version &b_);
	#endif
	inline bool operator==(const Spot &a__, const Spot &b_);
	inline bool operator==(const Deprecated &a__, const Deprecated &b_);
	inline bool operator==(const Dial &a__, const Dial &b_);
	inline bool operator==(const Knob &a__, const Knob &b_);
	inline bool operator==(const polar_Point &a__, const polar_Point &b_);
	inline bool operator==(const polar_Turn &a__, const polar_Turn &b_);
	inline bool operator==(const macros_Streams &a__, const macros_Streams &b_);
	inline bool operator==(const clash_Tone &a__, const clash_Tone &b_);
	inline bool operator==(const clash_TONE &a__, const clash_TONE &b_);
	inline bool operator==(const clash_Span &a__, const clash_Span &b_);
	inline bool operator==(const clash_Any &a__, const clash_Any &b_);
	inline bool operator==(const clash_Break &a__, const clash_Break &b_);
	inline bool operator==(const clash_Malformed &a__, const clash_Malformed &b_);
	inline bool operator==(const clash_Described &a__, const clash_Described &b_);
	inline bool operator==(const clash_Errno &a__, const clash_Errno &b_);
	inline bool operator==(const clash_Escaped &a__, const clash_Escaped &b_);
	inline bool operator==(const clash_Underscored &a__, const clash_Underscored &b_);
	inline bool operator==(::api::Shape a__, ::api::Shape b_);
	inline bool operator!=(::api::Shape a__, ::api::Shape b_);
	inline ::api::buf Shape_debug(::api::Shape self);
	inline ::std::optional<::api::Shape> Shape_value_of(::api::input name);
	inline ::std::vector<::api::Shape> Shape_ALL();
	inline ::std::vector<::api::Shape> Shape_BOTH();
	inline bool operator==(const ::api::Event &a__, const ::api::Event &b_);
	inline bool operator!=(const ::api::Event &a__, const ::api::Event &b_);
	inline ::api::buf Event_debug(const ::api::Event &self);
	inline bool operator==(const ::api::Clash &a__, const ::api::Clash &b_);
	inline bool operator!=(const ::api::Clash &a__, const ::api::Clash &b_);
	inline ::api::buf Clash_debug(const ::api::Clash &self);
	inline ::std::optional<::api::Point> Clash_point(const ::api::Clash &self);
	inline ::api::Clash Clash_of(double value);
	inline ::api::buf Clash_bytes(const ::api::Clash &self);
	inline ::api::Reading Clash_reading(const ::api::Clash &self, ::std::uint32_t kotlin);
	inline ::std::vector<::std::vector<::std::uint32_t>> Clash_listed(::api::slice<::std::uint32_t> java, ::std::uint32_t &kotlin);
	template <class F0> inline ::api::Clash_after_future Clash_after(F0 first, double kotlin);
	inline ::api::buf Clash_joined(const ::api::Clash &self, ::api::input lent, ::api::input owned, ::std::optional<::api::input> maybe);
	inline ::api::Clash_tagged_future Clash_tagged(const ::api::Clash &self, ::api::input tag);
	inline ::api::Stamp Clash_stamp(const ::api::Clash &self, ::std::uint64_t kotlin);
	inline ::api::Moment Clash_moment(const ::api::Clash &self, ::std::uint64_t kotlin);
	inline ::api::Clash Clash_at(const ::api::Point &api);
	inline ::api::buf Clash_describe(const ::api::Clash &self, ::api::input text, ::std::uint32_t n, ::api::Shape shape, ::std::uint32_t &out);
	inline bool operator==(const ::api::Stage &a__, const ::api::Stage &b_);
	inline bool operator!=(const ::api::Stage &a__, const ::api::Stage &b_);
	inline ::api::buf Stage_debug(const ::api::Stage &self);
	inline bool operator==(const ::api::Error &a__, const ::api::Error &b_);
	inline bool operator!=(const ::api::Error &a__, const ::api::Error &b_);
	inline ::api::buf Error_to_string(const ::api::Error &self);
	inline ::api::buf Error_debug(const ::api::Error &self);
	inline bool operator==(::api::Status_ a__, ::api::Status_ b_);
	inline bool operator!=(::api::Status_ a__, ::api::Status_ b_);
	inline ::api::buf Status__debug(::api::Status_ self);
	inline bool operator==(const ::api::Labelled &a__, const ::api::Labelled &b_);
	inline bool operator!=(const ::api::Labelled &a__, const ::api::Labelled &b_);
	inline ::api::buf Labelled_debug(const ::api::Labelled &self);
	inline bool operator==(::api::Family a__, ::api::Family b_);
	inline bool operator!=(::api::Family a__, ::api::Family b_);
	inline ::api::buf Family_debug(::api::Family self);
	inline bool operator==(::api::VecClashTone a__, ::api::VecClashTone b_);
	inline bool operator!=(::api::VecClashTone a__, ::api::VecClashTone b_);
	inline ::api::buf VecClashTone_debug(::api::VecClashTone self);
	inline bool operator==(const ::api::b &a__, const ::api::b &b_);
	inline bool operator!=(const ::api::b &a__, const ::api::b &b_);
	inline ::api::buf b_debug(const ::api::b &self);
	inline bool operator==(const ::api::Builtin &a__, const ::api::Builtin &b_);
	inline bool operator!=(const ::api::Builtin &a__, const ::api::Builtin &b_);
	inline bool operator==(const ::api::Rank &a__, const ::api::Rank &b_);
	inline bool operator!=(const ::api::Rank &a__, const ::api::Rank &b_);
	inline bool operator<(const ::api::Rank &a__, const ::api::Rank &b_);
	inline bool operator<=(const ::api::Rank &a__, const ::api::Rank &b_);
	inline bool operator>(const ::api::Rank &a__, const ::api::Rank &b_);
	inline bool operator>=(const ::api::Rank &a__, const ::api::Rank &b_);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const ::api::Rank &a__, const ::api::Rank &b_);
	#endif
	inline ::api::buf Rank_debug(const ::api::Rank &self);
	inline bool operator==(::api::Tier a__, ::api::Tier b_);
	inline bool operator!=(::api::Tier a__, ::api::Tier b_);
	inline bool operator<(::api::Tier a__, ::api::Tier b_);
	inline bool operator<=(::api::Tier a__, ::api::Tier b_);
	inline bool operator>(::api::Tier a__, ::api::Tier b_);
	inline bool operator>=(::api::Tier a__, ::api::Tier b_);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(::api::Tier a__, ::api::Tier b_);
	#endif
	inline ::api::buf Tier_debug(::api::Tier self);
	inline bool operator==(const ::api::clash_Candidate &a__, const ::api::clash_Candidate &b_);
	inline bool operator!=(const ::api::clash_Candidate &a__, const ::api::clash_Candidate &b_);
	inline ::api::buf clash_Candidate_debug(const ::api::clash_Candidate &self);
	inline bool operator==(::api::clash_Level a__, ::api::clash_Level b_);
	inline bool operator!=(::api::clash_Level a__, ::api::clash_Level b_);
	inline ::api::buf clash_Level_debug(::api::clash_Level self);
	inline clash_Level clash_Level_default();
	inline ::std::uint8_t clash_Level_index(::api::clash_Level self);
	inline bool operator==(const ::api::clash_Tag &a__, const ::api::clash_Tag &b_);
	inline bool operator!=(const ::api::clash_Tag &a__, const ::api::clash_Tag &b_);
	inline ::api::buf clash_Tag_debug(const ::api::clash_Tag &self);
	inline ::std::uint32_t clash_Tag_number(const ::api::clash_Tag &self);
	inline bool operator==(::api::clash_Op a__, ::api::clash_Op b_);
	inline bool operator!=(::api::clash_Op a__, ::api::clash_Op b_);
	inline ::api::buf clash_Op_debug(::api::clash_Op self);
	inline bool operator==(const ::api::clash_Change &a__, const ::api::clash_Change &b_);
	inline bool operator!=(const ::api::clash_Change &a__, const ::api::clash_Change &b_);
	inline ::api::buf clash_Change_debug(const ::api::clash_Change &self);
	inline bool has_text(const ::api::Note &note);
	inline ::std::uint64_t marked(const ::api::Mark &api_marked);
	inline ::std::optional<::api::Reading> reading(::std::uint32_t n);
	inline ::api::buf reading_debug(::std::uint32_t n);
	inline ::std::optional<::api::Reading> reading_echo(const ::std::optional<::api::Reading> &reading);
	inline void reading_bump(::std::optional<::api::Reading> &reading);
	inline ::api::Stage advance(const ::api::Stage &stage);
	inline ::std::tuple<::api::Counter> only(::api::Counter counter);
	inline ::std::tuple<::std::uint8_t, ::std::int16_t, ::std::uint32_t, ::std::uint64_t> widths(::std::uint64_t n);
	inline ::api::result<::std::tuple<::std::uint64_t, ::std::uint64_t>, ::api::Error> divide(::std::uint64_t a__, ::std::uint64_t b_);
	inline ::api::result<::std::tuple<::std::uint32_t, ::std::uint32_t>, ::api::Error> corner(bool unit);
	inline ::api::corner_later_future corner_later();
	inline ::std::optional<::std::tuple<::std::uint8_t, ::std::int16_t, ::std::uint32_t, ::std::uint64_t>> spread(::std::uint64_t n);
	inline ::std::optional<::std::tuple<::std::size_t, ::std::size_t>> find(::api::input haystack, ::api::input needle);
	inline ::std::vector<::std::tuple<::api::buf, ::api::Shape, ::api::Point>> rows(const ::std::vector<::api::Point> &points);
	inline ::api::result<::std::uint16_t, ::api::buf> port(::api::input text);
	inline ::std::uint32_t span(::std::uint32_t start, ::std::uint32_t end);
	inline ::std::tuple<::std::tuple<::std::uint32_t, ::std::uint32_t>, ::std::tuple<::std::uint32_t, ::std::uint32_t>> halves(const ::api::Span &span);
	inline ::std::optional<::std::tuple<::std::optional<::std::uint32_t>>> doubled(const ::std::optional<::std::tuple<::std::optional<::std::uint32_t>>> &value);
	inline ::std::optional<::std::tuple<::api::buf>> title(bool formal);
	inline ::std::optional<::std::tuple<::api::buf>> named(::api::input name);
	inline ::std::tuple<::api::buf> lent_back(const ::std::tuple<::api::view> &value);
	inline ::std::tuple<::api::buf> boxed_tuple(::api::input bytes);
	inline ::std::tuple<::std::uint32_t, ::api::buf> swapped(const ::std::tuple<::api::view, ::std::uint32_t> &pair);
	inline bool lasting(const ::api::Draft &draft);
	inline ::std::uint32_t stored(::api::Receipt receipt);
	inline ::std::uint32_t tallied(::api::Receipt receipt);
	inline ::std::uint32_t summed_up_Receipt(::api::Receipt totaled);
	inline ::std::uint32_t erased(::api::Receipt receipt);
	inline ::api::Name longer(::api::Name a__, const ::api::Name &b_);
	inline ::api::Name unnamed();
	inline ::std::uint64_t total(const ::api::range_u64 &range);
	inline ::std::uint32_t reach(::std::uint32_t start, ::std::uint32_t end);
	inline ::std::optional<::api::u128> scaled(::api::u128 v, ::std::uint64_t by);
	inline ::api::Batch_2 pair();
	inline ::std::uint64_t drain_4(::api::Batch &batch);
	inline ::std::uint64_t drain_2(::api::Batch_2 &batch);
	inline ::std::size_t capacities_4_4(const ::api::Batch &a__, const ::api::Batch &b_);
	inline ::std::size_t capacities_4_2(const ::api::Batch &a__, const ::api::Batch_2 &b_);
	inline ::std::size_t capacities_2_4(const ::api::Batch_2 &a__, const ::api::Batch &b_);
	inline ::std::size_t capacities_2_2(const ::api::Batch_2 &a__, const ::api::Batch_2 &b_);
	inline ::api::fill_4_future fill_4(::api::Batch &batch, ::std::uint32_t v);
	inline ::api::fill_2_future fill_2(::api::Batch_2 &batch, ::std::uint32_t v);
	inline ::api::range_u64 widen(const ::api::range_u64 &range, ::std::uint64_t by);
	inline ::std::size_t count(const ::api::bounds_u64 &range);
	inline ::api::bound_u64 end(::std::uint64_t limit);
	inline ::std::tuple<::api::bound_u64, ::api::bound_u64> ends(::std::uint64_t limit);
	inline ::api::result<::std::uint64_t, ::api::bounds_u64> below(::std::uint64_t n, ::std::uint64_t limit);
	template <class F0> inline ::api::drive_future drive(const ::api::Moment &start, F0 &&poll);
	template <class F0> inline ::api::buf made(::std::uint32_t n, F0 &&make);
	inline ::std::vector<::std::tuple<::api::buf, ::api::Ticket>> tickets(const ::api::items<::std::tuple<::api::view, ::api::Ticket>> &entries);
	template <class F0> inline ::api::Ticket reissue(const ::api::Ticket &ticket, F0 &&swap);
	template <class F0> inline ::api::Ticket reissue_lent(const ::api::Ticket &ticket, F0 &&swap);
	inline ::api::Seat seat(const ::api::Mark &mark, ::std::uint32_t row);
	inline ::std::uint64_t admit(::std::tuple<::api::Counter, ::api::Mark, ::api::Seat, ::api::Choice> entry);
	inline ::std::uint64_t choose(const ::api::Choice &choice);
	inline ::std::uint64_t heft(const ::std::tuple<::api::Mark, ::api::view> &marked, const ::api::Parcel &lent, ::std::tuple<::api::Counter, ::api::view> moved);
	inline ::api::result<::std::uint64_t, ::api::buf> spend(::api::Counter e, ::api::Counter it);
	inline bool is_kept(::api::Status_ status);
	template <class F0> inline ::api::result<void, ::api::buf> attempt(F0 &&check);
	template <class F0> inline ::api::result<::std::tuple<::api::buf, ::std::uint64_t>, ::api::buf> tail(F0 &file, ::std::uint64_t len);
	template <class F0> inline ::api::result<::std::uint64_t, ::api::buf> seek_to(F0 &file, ::std::uint64_t at);
	inline ::api::halved_future halved(::std::uint32_t n);
	inline ::api::yielded_future yielded(::std::uint32_t value);
	template <class F0> inline ::api::after_future after(F0 first, ::std::uint32_t value);
	template <class F0> inline ::api::within_future within(::std::uint32_t polls, F0 future_);
	inline ::api::result<::api::Stamp, ::api::Stamp> past(const ::api::Stamp &at, const ::api::Stamp &deadline);
	inline ::api::result<::std::uint32_t, ::std::uint32_t> odd(::std::uint32_t n);
	inline ::api::result<::api::Moment, ::api::buf> moment_at(::std::uint64_t ticks, ::std::uint64_t rate);
	inline ::api::result<::api::Stamp, ::api::buf> stamp_at(::std::uint64_t millis);
	template <class F0> inline ::api::Moment shifted(const ::api::Moment &moment, F0 &&shift);
	inline ::std::uint32_t jvm_sum(const ::api::JvmField &field, const ::api::JvmStatic &method, const ::api::OptIn &opt);
	inline ::api::Clocked clocked_after(const ::api::Clocked &clocked, ::std::uint64_t ticks);
	inline ::std::uint32_t emit(::std::unique_ptr<::api::Log> log, ::api::input lines);
	template <class F0> inline ::api::counted_future counted(::std::uint32_t n, F0 &&count);
	template <class F0> inline ::std::uint32_t filter(::api::input lines, F0 &&keep);
	inline ::api::log_later_future log_later(::std::unique_ptr<::api::Log> log);
	inline ::std::uint32_t log_counts(::std::unique_ptr<::api::Log> log, ::std::uint8_t level, const ::api::Counter &counter, ::std::vector<::api::Counter> more);
	inline bool on_first_thread();
	inline ::std::uint32_t locals_dropped();
	inline ::api::Local local_or_new(const ::api::Local *local);
	inline ::api::local_or_new_later_future local_or_new_later(const ::api::Local *local);
	inline ::std::uint32_t bump_either(const ::api::Local *first, const ::api::Local &second);
	inline ::api::local_after_future local_after();
	inline ::api::local_later_future local_later(::std::uint32_t n);
	inline ::api::poll_tripled_future poll_tripled(::std::uint32_t n);
	template <class F0> inline ::std::uint64_t watch(const ::api::Gauge &gauge, F0 &&watch);
	inline ::api::buf token_bytes(::std::uint32_t id, ::std::uint32_t len);
	inline ::api::buf token_slice(::std::uint32_t id, ::std::uint32_t len, ::std::uint32_t of);
	inline ::api::Packet next_packet(const ::api::Packet &packet);
	inline ::api::result<::std::array<::std::uint8_t, 4>, ::api::buf> fingerprint(::api::input hex);
	inline ::std::uint32_t dropped();
	inline ::std::uint32_t bump_both(const ::api::Tally &a__, const ::api::Tally &b_);
	template <class F0> inline ::std::uint32_t offset(F0 &&by, ::std::uint32_t by_data);
	template <class F0> inline ::std::uint32_t shift(F0 &&by_, ::std::uint32_t c_by__);
	template <class F0> inline ::std::uint64_t walk(::std::uint64_t steps, F0 &&visit);
	template <class F0> inline ::std::uint32_t each(::std::uint32_t steps, F0 &&visit);
	template <class F0> inline ::std::uint32_t peek(const ::api::Tally &tally, F0 &&peek);
	inline ::std::size_t len(::api::input payload);
	inline ::api::buf hex(::api::input payload);
	inline ::api::buf timed_hex(::api::input payload);
	inline ::api::buf sample_hex(::api::input sample);
	inline ::api::buf queued_hex(::api::input queued);
	inline ::api::buf tick_name(const ::api::Label &tick);
	inline ::api::buf rooted_name(const ::api::Label &rooted);
	inline ::api::buf stored_text(::api::input stored);
	inline ::api::buf layer_name(const ::api::Label &layer);
	inline ::api::buf greet(::api::input name, ::api::Shape shape);
	inline ::api::buf describe(const ::api::Event &event);
	inline ::api::Pair_Size_u8 tag(const ::api::Size &size, ::std::uint8_t tag);
	inline ::std::uint8_t cross(const ::api::Left_Right_u8 &left, const ::api::Right_Left_u8 &right);
	inline ::std::uint64_t kept_Point(const ::api::Point &from);
	inline ::std::uint64_t kept_Counter(::api::Counter from);
	inline ::std::uint64_t kept_Size(const ::api::Size &from);
	inline ::std::uint64_t kept_Wrap_u32(const ::api::Wrap_u32 &from);
	inline ::std::uint64_t kept_polar_Point(const ::api::polar_Point &from);
	inline ::std::int32_t keep_Point(const ::api::Point &kept);
	inline ::std::int32_t kept_or_Point(const ::std::optional<::api::Point> &kept);
	inline ::std::int32_t keeper_Point(const ::api::Point &keeper);
	inline ::api::Boxed_u16 BOXED();
	inline ::api::Boxed_u8 keeps(::std::unique_ptr<::api::Log> log);
	inline ::std::uint32_t weigh_Point(const ::api::Point &from);
	template <class F0> inline ::std::uint32_t apply_non_zero_u16(const ::api::Pair_non_zero_u16 &pair, F0 &&f);
	template <class F0> inline ::std::uint32_t apply_u32(const ::api::Pair_u32 &pair, F0 &&f);
	inline ::std::uint32_t report_non_zero_u16(const ::api::Pair_non_zero_u16 &pair, ::std::unique_ptr<::api::Log> log);
	inline ::std::uint32_t report_u32(const ::api::Pair_u32 &pair, ::std::unique_ptr<::api::Log> log);
	inline ::std::uint32_t tally_LogCount(::std::unique_ptr<::api::Log> log);
	inline ::std::uint32_t wind(const ::api::PolarTurn &whole, const ::api::polar_Turn &part);
	inline ::std::uint64_t area(const ::api::Size &size);
	inline ::std::uint64_t seeds_Point_Point(const ::api::Point &from);
	inline ::std::uint64_t seeds_Point_Counter(const ::api::Counter &from);
	inline ::std::uint64_t seeds_Point_Size(const ::api::Size &from);
	inline ::std::uint64_t seeds_Point_Wrap_u32(const ::api::Wrap_u32 &from);
	inline ::std::uint64_t seeds_Point_polar_Point(const ::api::polar_Point &from);
	inline ::std::uint64_t seeds_Counter_Point(const ::api::Point &from);
	inline ::std::uint64_t seeds_Counter_Counter(const ::api::Counter &from);
	inline ::std::uint64_t seeds_Counter_Size(const ::api::Size &from);
	inline ::std::uint64_t seeds_Counter_Wrap_u32(const ::api::Wrap_u32 &from);
	inline ::std::uint64_t seeds_Counter_polar_Point(const ::api::polar_Point &from);
	inline ::api::Area grow(const ::api::Area &area, ::std::uint32_t by);
	inline ::api::Slot_u32 wide(::std::uint32_t value);
	inline ::std::optional<::api::buf> host(const ::api::Url &url);
	inline ::api::result<::api::Url, ::api::buf> resolve(::api::Url base, ::api::input path);
	inline ::std::vector<::std::uint16_t> ports(const ::std::vector<::api::socket_addr> &addrs);
	inline ::api::duration doubled_ttl(::api::duration ttl);
	inline ::api::buf file(::api::input dir, ::api::input name);
	inline ::api::buf stray();
	template <class F0> inline void visit_stray(F0 &&visit);
	inline ::std::optional<char32_t> next_char(char32_t c_);
	inline ::std::optional<::api::duration> extended(::api::duration duration_, ::api::duration by);
	inline ::std::vector<::std::optional<::api::buf>> parents(const ::api::items<::api::view> &paths);
	inline ::std::vector<::api::buf> words(::api::input text);
	inline ::api::buf sentence(const ::std::optional<::api::items<::api::view>> &words);
	inline void put_number(::std::uint32_t value, ::api::buf &buf_);
	inline ::api::duration second_later(::api::duration w235);
	inline ::api::result<::std::uint8_t, ::api::Error> digit(::api::buf &r);
	inline ::std::uint32_t push_counted(::std::tuple<::api::buf, ::std::uint32_t> &entry, ::std::uint8_t byte);
	inline void nudge(::api::Point &point, ::std::int32_t by);
	inline void nudge_by(::api::Point &point, const ::api::Refusal &refusal);
	inline ::api::tls_Config secure(::api::input alpn);
	inline ::api::buf alpn(::api::tls_Config config);
	inline ::api::buf alpn_lent(const ::api::tls_Config &config);
	inline ::api::tls_Cert certify(::api::input der);
	inline ::api::byte_len_future byte_len(::api::input bytes);
	inline ::std::size_t cert_len(const ::api::tls_Cert &cert);
	inline ::api::buf cert_der(::api::tls_Cert cert);
	inline ::std::size_t chain_len(::std::vector<::api::tls_Cert> chain);
	inline ::api::result<::std::uint32_t, ::api::buf> verify(const ::api::tls_Cert &cert);
	inline ::api::result<::std::uint32_t, ::api::buf> parse_cert(::api::input text);
	inline ::api::parse_later_future parse_later(::api::input text);
	inline ::api::tls_Resolver resolver(::api::input name);
	inline ::api::buf resolved(::api::tls_Resolver resolver);
	inline ::api::std_io_Error failure(::api::input message);
	inline ::api::buf failure_message(const ::api::std_io_Error &error);
	inline ::api::buf failure_kind(::api::std_io_Error error);
	inline ::std::uint32_t sum_all(const ::std::vector<::std::uint32_t> &items_);
	inline ::std::uint64_t weigh_all_Counter(::std::vector<::api::Counter> items_);
	inline ::std::size_t offered_len(const ::api::items<::api::view> &alpns);
	inline ::api::buf join_words(const ::api::items<::api::view> &words);
	inline ::api::result<::api::tls_Session, ::api::buf> resume(::api::input alpn, ::std::uint32_t seconds);
	inline ::std::size_t either(::std::optional<::api::input> text);
	inline ::api::either_later_future either_later(::std::optional<::api::input> text);
	inline ::api::buf prefixed(::api::input payload, ::std::optional<::api::input> prefix);
	inline ::std::int32_t sum_lists(::api::slice<::std::int32_t> a__, ::std::optional<::api::slice<::std::int32_t>> b_, ::std::optional<::api::slice<::std::int32_t>> c_);
	inline ::std::int32_t coordinates(const ::std::optional<::api::Point> &point);
	inline ::std::vector<::api::buf> chunks(::api::input bytes, ::std::size_t size);
	inline ::std::vector<::api::Counter> counters(const ::api::items<::api::view> &names);
	inline ::std::uint64_t worth(::std::optional<::std::vector<::api::Counter>> some, ::std::vector<::api::Counter> boxed);
	inline ::std::optional<::std::uint64_t> held_count(::std::optional<::std::tuple<::api::Counter>> held);
	inline ::std::optional<::api::Counter> boxed_counter(::std::optional<::api::input> name);
	inline ::std::optional<::api::buf> boxed_name(::std::optional<::api::Counter> boxed);
	inline ::std::uint32_t less(::std::uint32_t value, ::std::uint32_t _value);
	inline ::std::vector<::std::uint32_t> sums(const ::std::vector<::std::vector<::std::uint32_t>> &rows);
	inline ::std::vector<::std::vector<::std::uint32_t>> least(const ::std::vector<::std::vector<::std::uint32_t>> &sets);
	inline ::std::tuple<::std::vector<::std::int32_t>, ::std::uint32_t> split_last(::api::slice<::std::int32_t> items_);
	inline ::std::uint64_t sum_u32(::api::slice<::std::uint32_t> items_);
	inline ::std::vector<::std::int64_t> doubled_i64(::api::slice<::std::int64_t> items_);
	template <class F0> inline ::std::vector<::std::uint64_t> visit_chunks(::api::input payload, ::std::size_t size, F0 &&visit);
	inline ::std::uint64_t scanned(::std::unique_ptr<::api::Scan> scan, ::api::input payload);
	inline ::std::vector<double> boxed_floats(::api::slice<double> items_);
	inline ::api::Point summed(const ::std::vector<::api::Point> &points);
	inline ::api::duration breadth(const ::api::Limits_duration &limits);
	inline ::api::Limits_bytes byte_limits(::api::input low, ::api::input high);
	inline ::api::buf joined(const ::api::items<::api::view> &parts);
	inline ::std::optional<::api::buf> first_of(const ::api::items<::api::view> &items_);
	inline ::std::uint64_t read_len(::api::input reader);
	inline ::std::size_t longest(const ::api::items<::api::view> &words);
	inline ::api::buf encode(const ::api::Json &value);
	inline ::api::Json tagged(::api::Json doc, ::api::input tag);
	inline ::std::vector<::std::tuple<::api::buf, ::api::Json>> entries(::api::taken<::std::tuple<::api::view, ::api::Json>> entries, ::api::input tag);
	inline ::std::vector<::std::tuple<::api::buf, ::api::Json>> sections(::api::taken<::std::tuple<::api::view, ::api::Json>> entries);
	inline ::api::Instant now();
	inline ::api::Instant later(const ::api::Instant &at, ::api::duration by);
	inline ::api::duration since(const ::api::Instant &later, const ::api::Instant &earlier);
	inline bool has_instant(const ::api::Instant *at);
	inline ::std::optional<::api::Level> louder(const ::std::optional<::api::Level> &filter);
	inline ::api::buf spell(const ::std::optional<::api::Level> &filter);
	inline ::std::vector<::api::buf> GREETINGS();
	inline ::std::vector<::std::uint32_t> PRIMES();
	inline ::std::vector<::api::Counter> numbered(const ::api::items<::api::view> &names);
	inline ::std::tuple<::api::socket_addr_v4, ::api::socket_addr_v6> nexts(::api::socket_addr_v4 v4, ::api::socket_addr_v6 v6);
	inline ::std::tuple<::api::ipv4_addr, ::api::ipv6_addr> next_ips(::api::ipv4_addr v4, ::api::ipv6_addr v6);
	inline ::api::ip_addr flip(::api::ip_addr ip);
	inline ::api::buf reasons(::std::vector<::std::tuple<::api::Counter, ::api::Refusal>> pairs, ::std::optional<::std::tuple<::api::Counter, ::api::Refusal>> last);
	inline ::std::size_t excuse(::api::input note, ::std::tuple<::api::view, ::api::Refusal> entry);
	inline ::std::uint32_t relay(::std::unique_ptr<::api::Log> log, ::std::vector<::api::Refusal> refusals);
	inline ::api::result<::std::uint32_t, ::api::Refusal> refuse(::api::input reason);
	inline ::api::result<::std::uint32_t, ::api::Fault> jam(::std::uint32_t n);
	inline ::api::Platform platform();
	inline ::std::uint32_t pointer_bits(const ::api::Platform &platform);
	inline ::api::Labelled same_labelled(const ::api::Labelled &labelled);
	inline ::api::Placement placement();
	inline bool is_family(::api::Family family);
	inline ::std::uint32_t wasm_only();
	inline ::api::result<::std::uint64_t, ::api::Error> wasm_take(::api::Counter counter);
	inline void wasm_log(::std::unique_ptr<::api::Log> log);
	inline ::std::uint64_t booth_count(const ::api::Booth &booth);
	inline ::api::Page page();
	inline ::std::uint32_t beats(::std::unique_ptr<::api::Beat> beat);
	template <class F0, class F1> inline ::std::uint8_t paired(F0 &&f, F1 &&f_unwind);
	inline ::std::int32_t same(double a__, double b_);
	inline ::std::int32_t errno_();
	inline bool assert_(bool holds);
	inline ::std::uint8_t offsetof_(::std::uint8_t field);
	inline ::api::result<::std::tuple<::api::buf, ::std::optional<::std::tuple<::std::uint8_t, ::std::int16_t, ::std::uint32_t, ::std::uint64_t>>, ::std::vector<::api::buf>>, ::api::Error> return_values(::api::Point &point, bool fail);
	inline ::api::result<::api::Counter, ::api::Error> return_moved(::api::Counter counter, bool fail);
	inline ::api::NonZeros non_zeros();
	inline ::api::buf describe_non_zeros(const ::api::NonZeros &non_zeros);
	inline ::std::optional<::std::uint64_t> next_hop(const ::std::optional<::std::uint64_t> &hop);
	inline ::std::uint16_t farther_hop(const ::api::Pair_non_zero_u16 &hops);
	inline ::std::int32_t WEXITSTATUS_(::std::int32_t status);
	inline ::api::duration same_duration(::api::duration v);
	inline ::std::uint64_t same_u64(::std::uint64_t v);
	inline ::std::size_t same_usize(::std::size_t v);
	inline ::std::ptrdiff_t same_isize(::std::ptrdiff_t v);
	inline ::api::u128 same_u128(::api::u128 v);
	inline char32_t same_char(char32_t v);
	inline ::api::buf same_string(::api::input v);
	inline ::api::buf same_path(::api::input v);
	inline ::api::socket_addr same_socket_addr(::api::socket_addr v);
	inline ::api::socket_addr_v4 same_socket_addr_v4(::api::socket_addr_v4 v);
	inline ::api::socket_addr_v6 same_socket_addr_v6(::api::socket_addr_v6 v);
	inline ::api::ip_addr same_ip_addr(::api::ip_addr v);
	inline ::api::buf unescaped(::api::input text);
	inline ::api::unescaped_later_future unescaped_later(::api::input text);
	inline ::api::unescaped_maybe_future unescaped_maybe(::std::optional<::api::input> text);
	inline ::api::buf unzeroed(::api::input bytes);
	inline ::std::uint8_t deprecated(::std::uint8_t note);
	inline ::std::uint8_t _deprecated(::std::uint8_t note);
	inline ::std::uint8_t __all___(::std::uint8_t note);
	inline ::std::uint8_t __all____(::std::uint8_t note);
	[[deprecated]] inline ::api::Knob knob(const ::api::Dial &dial);
	[[deprecated("read its `level`")]] inline ::api::level_later_future level_later(const ::api::Dial &dial);
	inline ::api::result<::api::Dial, ::api::Error> turned(const ::api::Dial &dial, ::std::unique_ptr<::api::Turner> turner);
	inline ::std::uint32_t undone(::std::unique_ptr<::api::Undo> undo, ::std::uint32_t n);
	inline ::std::uint32_t unwound();
	template <class F0> inline ::std::size_t measured(F0 &&f);
	template <class F0> inline ::std::ptrdiff_t negated(F0 &&f);
	template <class F0> inline char32_t lettered(F0 &&f);
	template <class F0> inline char32_t bumped(::std::uint32_t &n, F0 &&f);
	inline ::std::size_t stretched(::std::unique_ptr<::api::Stretch> stretch);
	inline ::std::uint64_t crowded(const ::api::Reading &reading, const ::api::Stamp &a__, const ::api::Stamp &b_, ::std::uint64_t n0, ::std::uint64_t n1, ::std::uint64_t n2, ::std::uint64_t n3, ::std::uint64_t n4, ::std::uint64_t n5, ::std::uint64_t n6, ::std::uint64_t n7, ::std::uint64_t n8, ::std::uint64_t n9, ::std::uint64_t n10, ::std::uint64_t n11, ::std::uint64_t n12, ::std::uint64_t n13, ::std::uint64_t n14, ::std::uint64_t n15, ::std::uint64_t n16, ::std::uint64_t n17, ::std::uint64_t n18, ::std::uint64_t n19, ::std::uint64_t n20, ::std::uint64_t n21, ::std::uint64_t n22, ::std::uint64_t n23, ::std::uint64_t n24, ::std::uint64_t n25, ::std::uint64_t n26, ::std::uint64_t n27, ::std::uint64_t n28, ::std::uint64_t n29, ::std::uint64_t n30, ::std::uint64_t n31, ::std::uint64_t n32, ::std::uint64_t n33, ::std::uint64_t n34, ::std::uint64_t n35, ::std::uint64_t n36, ::std::uint64_t n37, ::std::uint64_t n38, ::std::uint64_t n39, ::std::uint64_t n40, ::std::uint64_t n41, ::std::uint64_t n42, ::std::uint64_t n43, ::std::uint64_t n44, ::std::uint64_t n45, ::std::uint64_t n46, ::std::uint64_t n47, ::std::uint64_t n48, ::std::uint64_t n49, ::std::uint64_t n50, ::std::uint64_t n51, ::std::uint64_t n52, ::std::uint64_t n53, ::std::uint64_t n54, ::std::uint64_t n55, ::std::uint64_t n56, ::std::uint64_t n57, ::std::uint64_t n58, ::std::uint64_t n59, ::std::uint64_t n60, ::std::uint64_t n61, ::std::uint64_t n62, ::std::uint64_t n63, ::std::uint64_t n64, ::std::uint64_t n65, ::std::uint64_t n66, ::std::uint64_t n67, ::std::uint64_t n68, ::std::uint64_t n69, ::std::uint64_t n70, ::std::uint64_t n71, ::std::uint64_t n72, ::std::uint64_t n73, ::std::uint64_t n74, ::std::uint64_t n75, ::std::uint64_t n76, ::std::uint64_t n77, ::std::uint64_t n78, ::std::uint64_t n79, ::std::uint64_t n80, ::std::uint64_t n81, ::std::uint64_t n82, ::std::uint64_t n83, ::std::uint64_t n84, ::std::uint64_t n85, ::std::uint64_t n86, ::std::uint64_t n87, ::std::uint64_t n88, ::std::uint64_t n89, ::std::uint64_t n90, ::std::uint64_t n91, ::std::uint64_t n92, ::std::uint64_t n93, ::std::uint64_t n94, ::std::uint64_t n95, ::std::uint64_t n96, ::std::uint64_t n97, ::std::uint64_t n98, ::std::uint64_t n99, ::std::uint64_t n100, ::std::uint64_t n101, ::std::uint64_t n102, ::std::uint64_t n103, ::std::uint64_t n104, ::std::uint64_t n105, ::std::uint64_t n106, ::std::uint64_t n107, ::std::uint64_t n108, ::std::uint64_t n109, ::std::uint64_t n110, ::std::uint64_t n111, ::std::uint64_t n112, ::std::uint64_t n113, ::std::uint64_t n114, ::std::uint64_t n115, ::std::uint64_t n116, ::std::uint64_t n117, ::std::uint64_t n118);
	inline ::std::uint64_t crammed(const ::api::Moment &a__, const ::api::Moment &b_, ::std::uint64_t n0, ::std::uint64_t n1, ::std::uint64_t n2, ::std::uint64_t n3, ::std::uint64_t n4, ::std::uint64_t n5, ::std::uint64_t n6, ::std::uint64_t n7, ::std::uint64_t n8, ::std::uint64_t n9, ::std::uint64_t n10, ::std::uint64_t n11, ::std::uint64_t n12, ::std::uint64_t n13, ::std::uint64_t n14, ::std::uint64_t n15, ::std::uint64_t n16, ::std::uint64_t n17, ::std::uint64_t n18, ::std::uint64_t n19, ::std::uint64_t n20, ::std::uint64_t n21, ::std::uint64_t n22, ::std::uint64_t n23, ::std::uint64_t n24, ::std::uint64_t n25, ::std::uint64_t n26, ::std::uint64_t n27, ::std::uint64_t n28, ::std::uint64_t n29, ::std::uint64_t n30, ::std::uint64_t n31, ::std::uint64_t n32, ::std::uint64_t n33, ::std::uint64_t n34, ::std::uint64_t n35, ::std::uint64_t n36, ::std::uint64_t n37, ::std::uint64_t n38, ::std::uint64_t n39, ::std::uint64_t n40, ::std::uint64_t n41, ::std::uint64_t n42, ::std::uint64_t n43, ::std::uint64_t n44, ::std::uint64_t n45, ::std::uint64_t n46, ::std::uint64_t n47, ::std::uint64_t n48, ::std::uint64_t n49, ::std::uint64_t n50, ::std::uint64_t n51, ::std::uint64_t n52, ::std::uint64_t n53, ::std::uint64_t n54, ::std::uint64_t n55, ::std::uint64_t n56, ::std::uint64_t n57, ::std::uint64_t n58, ::std::uint64_t n59, ::std::uint64_t n60, ::std::uint64_t n61, ::std::uint64_t n62, ::std::uint64_t n63, ::std::uint64_t n64, ::std::uint64_t n65, ::std::uint64_t n66, ::std::uint64_t n67, ::std::uint64_t n68, ::std::uint64_t n69, ::std::uint64_t n70, ::std::uint64_t n71, ::std::uint64_t n72, ::std::uint64_t n73, ::std::uint64_t n74, ::std::uint64_t n75, ::std::uint64_t n76, ::std::uint64_t n77, ::std::uint64_t n78, ::std::uint64_t n79, ::std::uint64_t n80, ::std::uint64_t n81, ::std::uint64_t n82, ::std::uint64_t n83, ::std::uint64_t n84, ::std::uint64_t n85, ::std::uint64_t n86, ::std::uint64_t n87, ::std::uint64_t n88, ::std::uint64_t n89, ::std::uint64_t n90, ::std::uint64_t n91, ::std::uint64_t n92, ::std::uint64_t n93, ::std::uint64_t n94, ::std::uint64_t n95, ::std::uint64_t n96, ::std::uint64_t n97, ::std::uint64_t n98, ::std::uint64_t n99, ::std::uint64_t n100, ::std::uint64_t n101, ::std::uint64_t n102, ::std::uint64_t n103, ::std::uint64_t n104, ::std::uint64_t n105, ::std::uint64_t n106, ::std::uint64_t n107, ::std::uint64_t n108, ::std::uint64_t n109, ::std::uint64_t n110, ::std::uint64_t n111, ::std::uint64_t n112, ::std::uint64_t n113, ::std::uint64_t n114, ::std::uint64_t n115, ::std::uint64_t n116, ::std::uint64_t n117, ::std::uint64_t n118, ::std::uint64_t n119, ::std::uint64_t n120, ::std::uint64_t n121, ::std::uint64_t n122, ::std::uint64_t n123);
	inline ::std::uint32_t jni_one();
	inline ::std::optional<::api::polar_Point> polar_point(::std::uint32_t distance);
	inline ::std::uint32_t macros_streamed(const ::api::macros_Streams &streams, ::std::uint8_t errno_);
	inline ::api::clash_Any clash_any(::api::input type_url);
	inline ::api::clash_Candidate clash_candidate(::std::uint8_t at);
	inline ::std::uint8_t clash_ignore(::std::uint8_t arg0, ::std::uint8_t arg1, ::std::uint8_t arg2);
	inline ::api::clash_Empty_string clash_empty();
	inline ::std::uint32_t clash_described(const ::api::clash_Described &described);
	inline ::std::uint8_t clash_waited(::std::unique_ptr<::api::clash_Source> source);
	inline ::std::uint32_t clash_underscored(const ::api::clash_Underscored &underscored, ::api::clash_Op op, const ::api::clash_Change &change);
	inline ::std::tuple<::api::clash_Underscored, ::api::clash_Change> clash_underscored_next(const ::api::clash_Underscored &underscored, const ::api::clash_Change &change);
	inline ::std::uint32_t clash_erased(::std::unique_ptr<::api::clash_Eraser> eraser);
	inline ::std::uint8_t clash_erase_();
	inline ::std::uint8_t clash_erase();
	inline ::std::uint32_t clash_lend_(const ::api::clash_Store *store);
	inline ::std::uint32_t clash_lend();
	template <class F0> inline ::std::uint32_t clash_run(F0 &&work_, ::std::uint8_t work);
	inline ::api::result<::api::clash_Tone, ::std::vector<::api::clash_Tone>> clash_audible(const ::api::clash_Tone &tone);
	inline ::std::uint32_t clash_nested(const ::std::optional<::std::tuple<::std::tuple<::std::uint8_t, ::std::uint16_t>, ::std::uint32_t>> &a__, const ::std::optional<::std::tuple<::std::tuple<::std::uint8_t, ::std::uint16_t, ::std::uint32_t>>> &b_);
	inline ::std::uint8_t clash_audio_encoder_finish();
	inline ::std::uint8_t clash_audio_encoder_finish_();
	inline ::std::uint32_t emit_from_thread(::std::unique_ptr<::api::Log> log, ::api::input lines);
	inline ::api::emit_later_future emit_later(::std::unique_ptr<::api::Log> log, ::api::input lines);
	inline ::api::log_on_drop_future log_on_drop(::std::unique_ptr<::api::Log> log);
	inline ::api::waker log_waker(::std::unique_ptr<::api::Log> log);
	template <class F0> inline void hook(F0 &&f);
	inline void unhook();
}
namespace std {
	template <> struct hash<::api::Local> {
		size_t operator()(const ::api::Local &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::api::Plain> {
		size_t operator()(const ::api::Plain &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::api::Labeled> {
		size_t operator()(const ::api::Labeled &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::api::Key> {
		size_t operator()(const ::api::Key &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::api::Part> {
		size_t operator()(const ::api::Part &value) const noexcept;
	};
}
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic pop
#elif defined(_MSC_VER)
#pragma warning(pop)
#endif
#endif

// core.hpp
#ifndef API_HPP_CRATE0
#define API_HPP_CRATE0
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#elif defined(_MSC_VER)
#pragma warning(push)
#pragma warning(disable : 4996)
#endif
#include "rt.hpp"
namespace api {
	class RUST2_FFI_CONSUMABLE std_io_Error {
		RUST2_FFI_UNCONSUMED std_io_Error(std_io_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		std_io_Error &operator=(std_io_Error &&other) noexcept = default;
	};
}
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic pop
#elif defined(_MSC_VER)
#pragma warning(pop)
#endif
#endif

// dep.hpp
#ifndef API_HPP_CRATE2
#define API_HPP_CRATE2
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#elif defined(_MSC_VER)
#pragma warning(push)
#pragma warning(disable : 4996)
#endif
#include "rt.hpp"
namespace api {
	class RUST2_FFI_CONSUMABLE Area {
		RUST2_FFI_UNCONSUMED Area(Area &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Area &operator=(Area &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Area(const Area &other) = default;
		Area &operator=(const Area &other) = default;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static ::api::Area new_(const ::api::Size &size);
		RUST2_FFI_CALLABLE ::api::Size size() const;
		RUST2_FFI_CALLABLE ::std::uint64_t pixels() const;
		RUST2_FFI_CALLABLE ::std::uint64_t scaled_Point(const ::api::Point &by) const;
	};
	struct Size {
		::std::uint32_t width;
		::std::uint32_t height;
		friend bool operator==(const Size &a__, const Size &b_);
		friend bool operator!=(const Size &a__, const Size &b_);
		::api::buf debug() const;
	};
	inline bool operator==(const Size &a__, const Size &b_);
}
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic pop
#elif defined(_MSC_VER)
#pragma warning(pop)
#endif
#endif

// rt.hpp
#ifndef API_HPP_RT
#define API_HPP_RT
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#elif defined(_MSC_VER)
#pragma warning(push)
#pragma warning(disable : 4996)
#endif
#include "../api.h"
#include <array>
#include <atomic>
#include <condition_variable>
#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <functional>
#include <initializer_list>
#include <ios>
#include <memory>
#include <mutex>
#include <optional>
#include <string_view>
#include <thread>
#include <tuple>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>
#if defined(__cpp_impl_coroutine) && __cpp_impl_coroutine >= 201902L && __has_include(<coroutine>)
#include <coroutine>
#endif
#if defined(__cpp_impl_three_way_comparison) && __cpp_impl_three_way_comparison >= 201907L && __has_include(<compare>)
#include <compare>
#endif
namespace api {
	#if defined(__clang__)
	#define RUST2_FFI_OWNER(T) [[gsl::Owner(T)]]
	#define RUST2_FFI_POINTER(T) [[gsl::Pointer(T)]]
	#define RUST2_FFI_LIFETIMEBOUND [[clang::lifetimebound]]
	#define RUST2_FFI_CONSUMABLE [[clang::consumable(unconsumed)]]
	#define RUST2_FFI_CALLABLE [[clang::callable_when("unconsumed", "unknown")]]
	#define RUST2_FFI_CONSUMES [[clang::callable_when("unconsumed", "unknown"), clang::set_typestate(consumed)]]
	#define RUST2_FFI_UNCONSUMED [[clang::return_typestate(unconsumed)]]
	#define RUST2_FFI_MOVED_FROM [[clang::return_typestate(consumed)]]
	#else
	#define RUST2_FFI_OWNER(T)
	#define RUST2_FFI_POINTER(T)
	#define RUST2_FFI_LIFETIMEBOUND
	#define RUST2_FFI_CONSUMABLE
	#define RUST2_FFI_CALLABLE
	#define RUST2_FFI_CONSUMES
	#define RUST2_FFI_UNCONSUMED
	#define RUST2_FFI_MOVED_FROM
	#endif
	class buf;
	class input;
	template <class T> class borrow {
		explicit borrow(T &object) noexcept : object_(object);
		borrow(const borrow &) = delete;
		borrow &operator=(const borrow &) = delete;
		T *operator->() const noexcept;
	};
	class RUST2_FFI_POINTER(::std::uint8_t) view {
		view() noexcept : data_(detail::bytes(nullptr, 0)), size_(0);
		view(const void *data, ::std::size_t size) noexcept : data_(detail::bytes(data, size)), size_(size);
		view(const char *text) noexcept : view(text, ::std::strlen(text));
		template <class R, class = ::std::enable_if_t<sizeof(*::std::declval<const R &>().data()) == 1 && !::std::is_pointer_v<::std::decay_t<R>>>, class = decltype(::std::declval<const R &>().size())> view(const R &range RUST2_FFI_LIFETIMEBOUND) noexcept : view(range.data(), range.size());
		const ::std::uint8_t *data() const noexcept;
		::std::size_t size() const noexcept;
		bool empty() const noexcept;
		const ::std::uint8_t *begin() const noexcept;
		const ::std::uint8_t *end() const noexcept;
		::std::string_view str() const noexcept;
	};
	#if defined(__GNUC__) && !defined(__clang__)
	#pragma GCC diagnostic push
	#pragma GCC diagnostic ignored "-Winit-list-lifetime"
	#endif
	template <class T> class RUST2_FFI_POINTER(T) slice {
		slice() noexcept : data_(nullptr), size_(0);
		slice(const T *data, ::std::size_t size) noexcept : data_(data), size_(size);
		template <::std::size_t N> slice(const T (&array RUST2_FFI_LIFETIMEBOUND)[N]) noexcept : data_(array), size_(N);
		slice(::std::initializer_list<T> list RUST2_FFI_LIFETIMEBOUND) noexcept : data_(list.begin()), size_(list.size());
		template <class R, class = ::std::enable_if_t<::std::is_same_v< ::std::remove_cv_t<::std::remove_pointer_t<decltype(::std::declval<const R &>().data())>>, T>>, class = decltype(::std::declval<const R &>().size())> slice(const R &range RUST2_FFI_LIFETIMEBOUND) noexcept : data_(range.data()), size_(range.size());
		const T *data() const noexcept;
		::std::size_t size() const noexcept;
		bool empty() const noexcept;
		const T *begin() const noexcept;
		const T *end() const noexcept;
	};
	#if defined(__GNUC__) && !defined(__clang__)
	#pragma GCC diagnostic pop
	#endif
	class [[nodiscard]] RUST2_FFI_OWNER(::std::uint8_t) buf {
		buf() noexcept;
		explicit buf(view bytes) : buf();
		buf(const buf &other) : buf();
		buf(buf &&other) noexcept : raw_(::std::exchange(other.raw_, detail::c_buf{nullptr, 0, nullptr}));
		buf &operator=(const buf &other);
		buf &operator=(buf &&other) noexcept;
		~buf();
		const ::std::uint8_t *data() const noexcept RUST2_FFI_LIFETIMEBOUND;
		::std::size_t size() const noexcept;
		bool empty() const noexcept;
		const ::std::uint8_t *begin() const noexcept;
		const ::std::uint8_t *end() const noexcept;
		::std::string_view str() const noexcept RUST2_FFI_LIFETIMEBOUND;
	};
	class input {
		input(const char *text) noexcept : input(view(text));
		template <class R, class = ::std::enable_if_t<::std::is_constructible_v<view, const R &> && !::std::is_pointer_v<::std::decay_t<R>>>> input(const R &range RUST2_FFI_LIFETIMEBOUND) noexcept;
		input(buf &&bytes) noexcept;
		input(input &&other) noexcept : raw_(::std::exchange(other.raw_, detail::c_buf{nullptr, 0, nullptr})), held_(::std::move(other.held_));
		input(const input &) = delete;
		input &operator=(const input &) = delete;
		input &operator=(input &&) = delete;
		~input();
	};
	template <class T> class RUST2_FFI_POINTER(T) items {
		items(const ::std::vector<T> &vector RUST2_FFI_LIFETIMEBOUND) noexcept : data_(vector.data()), size_(vector.size());
		items(::std::initializer_list<T> list) : converted_(list), data_(converted_.data()), size_(converted_.size());
		template <class R, class = ::std::enable_if_t<!::std::is_same_v<::std::decay_t<R>, ::std::vector<T>> && !::std::is_same_v<::std::decay_t<R>, items> && detail::lasting<const R, T> && ::std::is_convertible_v<decltype(*::std::begin(::std::declval<const R &>())), T>>> items(const R &range RUST2_FFI_LIFETIMEBOUND);
		template <class U, class = ::std::enable_if_t<!::std::is_same_v<U, T> && ::std::is_convertible_v<U &&, T>>> items(::std::vector<U> &&vector RUST2_FFI_LIFETIMEBOUND);
		items(const items &) = delete;
		items &operator=(const items &) = delete;
		items(items &&other) noexcept = default;
		const T *begin() const noexcept;
		const T *end() const noexcept;
		::std::size_t size() const noexcept;
		bool empty() const noexcept;
	};
	template <class T> class taken {
		taken() noexcept = default;
		taken(::std::vector<T> &&vector) noexcept : items_(::std::move(vector));
		template <class R, class = ::std::enable_if_t<!::std::is_reference_v<R> && !::std::is_same_v<R, ::std::vector<T>> && !::std::is_same_v<R, taken> && detail::lasting<R, T> && ::std::is_convertible_v<decltype(::std::move(*::std::begin(::std::declval<R &>()))), T>>> taken(R &&range RUST2_FFI_LIFETIMEBOUND);
		T *begin() noexcept;
		T *end() noexcept;
		::std::size_t size() const noexcept;
		bool empty() const noexcept;
	};
	template <class T, class E> class [[nodiscard]] result {
		static result ok(T value);
		static result err(E error);
		bool has_value() const noexcept;
		explicit operator bool() const noexcept;
		T &value() &;
		const T &value() const &;
		T &&value() &&;
		T &operator*() &;
		const T &operator*() const &;
		T &&operator*() &&;
		T *operator->();
		const T *operator->() const;
		E &error() &;
		const E &error() const &;
		E &&error() &&;
	};
	template <class E> class [[nodiscard]] result<void, E> {
		static result ok();
		static result err(E error);
		bool has_value() const noexcept;
		explicit operator bool() const noexcept;
		void value() const noexcept;
		E &error() &;
		const E &error() const &;
		E &&error() &&;
	};
	class waker {
		template <class F, class = ::std::enable_if_t<!::std::is_same_v<::std::decay_t<F>, waker>>> explicit waker(F f) : node_(new callable<F>(::std::move(f)));
		waker(const waker &other) noexcept : node_(other.node_);
		waker &operator=(const waker &other) noexcept;
		~waker();
		void wake() const noexcept;
	};
	template <class T> using poll_t = ::std::conditional_t<::std::is_void_v<T>, bool, ::std::optional<T>>;
	class task;
	class [[nodiscard]] task {
		task() noexcept = default;
		task(task &&other) noexcept : node_(::std::exchange(other.node_, nullptr));
		task &operator=(task &&other) noexcept;
		~task();
	};
	template <class Op> class [[nodiscard]] future {
		using output = typename Op::output;
		future(future &&other) noexcept : raw_(::std::exchange(other.raw_, nullptr)), last_(::std::exchange(other.last_, nullptr));
		future &operator=(future &&other) noexcept;
		~future();
		poll_t<output> poll(const waker &w);
		output get() &&;
		template <class Exec, class F> task then(Exec exec, F f) &&;
		template <class Exec> auto via(Exec exec) &&;
	};
	using u128 = ::api_u128;
	using duration = ::api_duration;
	using socket_addr = ::api_socket_addr;
	using socket_addr_v4 = ::api_socket_addr_v4;
	using socket_addr_v6 = ::api_socket_addr_v6;
	using ip_addr = ::api_ip_addr;
	using ipv4_addr = ::api_ipv4_addr;
	using ipv6_addr = ::api_ipv6_addr;
	using bound = ::api_bound;
	using bound_u64 = ::api_bound_u64;
	using bounds_u64 = ::api_bounds_u64;
	using range_u64 = ::api_range_u64;
	enum class Shape;
	enum class Status_;
	enum class Family;
	enum class VecClashTone;
	enum class Tier;
	enum class clash_Level;
	enum class clash_Op;
	enum class Level;
	struct Parcel;
	struct Weighted;
	struct Stamped;
	struct CancellationError;
	struct Packet;
	struct Hasher;
	struct Size;
	struct Point;
	struct Label;
	struct Note;
	struct Reading;
	struct Packaged;
	struct Atlas;
	struct Span;
	struct Window;
	class Ticket;
	struct Seat;
	class Stamp;
	class Moment;
	class LongArray;
	struct JvmField;
	struct JvmStatic;
	struct OptIn;
	class Wide;
	struct Clocked;
	class Rated;
	class Score;
	class Hits;
	class Regs;
	class Extent;
	struct Pair_Size_u8;
	struct Pair_non_zero_u16;
	struct Pair_u32;
	struct Pair_u8_Size;
	struct Left_Right_u8;
	struct Left_u8;
	struct Right_Left_u8;
	struct Right_u8;
	struct Inner_u8;
	struct Boxed_u16;
	struct Boxed_u8;
	struct LogCount;
	struct Wrap_u32;
	struct PolarTurn;
	struct Peer;
	struct Shadowed;
	struct Numbers;
	struct Measure;
	struct Limits_bytes;
	struct Limits_duration;
	struct Page;
	struct Booth;
	struct Cancelled;
	struct StartSeekable;
	struct DoubleArray;
	struct NonZeros;
	struct a;
	struct Qualifiers;
	struct Version;
	struct Spot;
	struct Deprecated;
	struct Dial;
	struct Knob;
	struct polar_Point;
	struct polar_Turn;
	struct macros_Streams;
	struct clash_Tone;
	struct clash_TONE;
	struct clash_Span;
	struct clash_Any;
	struct clash_Break;
	struct clash_Malformed;
	struct clash_Described;
	struct clash_Errno;
	struct clash_Escaped;
	struct clash_Underscored;
	struct tls_policy_Lifetime;
	struct Core;
	class Board;
	class Tab;
	class Receipt;
	class Draft;
	class Filed_Receipt;
	class Pad_u16;
	class Pad_u8;
	class Layered;
	class Marker;
	class Batch;
	class Batch_2;
	class Waiter;
	class Feed;
	class Countdown;
	class Retired;
	class Folio;
	class Logger;
	class Journal;
	class Local;
	class Wrapped;
	class Buffer;
	class LocalWaiter;
	class Retry;
	class Digest;
	class Cursor;
	class Outer_Inner_u8;
	class Slot_u32;
	class Slot_u8;
	class Resolving;
	class Codecs;
	class Der;
	class Verbosity;
	class Site;
	class Trail;
	class Worker;
	class Handle;
	class Holder;
	class Tag;
	class Area;
	class Counter;
	class Meter;
	class Meters;
	class Registry;
	class Ledger;
	class Slip;
	class Sheet;
	class Name;
	class Channel;
	class Gap;
	class Berth;
	class Mark;
	class Hasp;
	class Message;
	class Node;
	class Tally;
	class Tallies;
	class Plain;
	class Labeled;
	class Gauge;
	class Latch;
	class Token;
	class Stack;
	class Refusal;
	class Fault;
	class c;
	class Quantity;
	class Pass;
	class Key;
	class clash_Seed;
	class clash_Empty_string;
	class clash_Store;
	class clash_Reader;
	class tls_Session;
	class Url;
	class Json;
	class Instant;
	class std_io_Error;
	class tls_Config;
	class tls_Cert;
	class tls_Resolver;
	class tls_Codec;
	class Spawned;
	class LogThread;
	class Pinned;
	class Guarded;
	class Hooked;
	class Forged;
	class Part;
	class Log;
	class Scan;
	class Beat;
	class Turner;
	class Undo;
	class Stretch;
	class clash_Source;
	class clash_Eraser;
	struct Choice_Empty;
	struct Choice_Picked;
	using Choice = ::std::variant<Choice_Empty, Choice_Picked>;
	struct Array_U8x4;
	using Array = ::std::variant<Array_U8x4>;
	struct Platform_Wasm;
	struct Platform_Native;
	using Platform = ::std::variant<Platform_Wasm, Platform_Native>;
	struct Placement_Anywhere;
	struct Placement_Pinned;
	using Placement = ::std::variant<Placement_Anywhere, Placement_Pinned>;
	struct Event_Moved;
	struct Event_Named;
	struct Event_Done;
	using Event = ::std::variant<Event_Moved, Event_Named, Event_Done>;
	struct Clash_Point;
	struct Clash_Shape;
	struct Clash_Event;
	struct Clash_Double;
	struct Clash_Float;
	struct Clash_String;
	struct Clash_Int;
	struct Clash_Boolean;
	struct Clash_Native;
	struct Clash_Bytes;
	struct Clash_Any;
	struct Clash_Unit;
	struct Clash_Waker;
	struct Clash_Mut;
	struct Clash_Pair;
	struct Clash_Stamp;
	struct Clash_Moment;
	using Clash = ::std::variant<Clash_Point, Clash_Shape, Clash_Event, Clash_Double, Clash_Float, Clash_String, Clash_Int, Clash_Boolean, Clash_Native, Clash_Bytes, Clash_Any, Clash_Unit, Clash_Waker, Clash_Mut, Clash_Pair, Clash_Stamp, Clash_Moment>;
	struct Stage_Init;
	struct Stage_Done;
	using Stage = ::std::variant<Stage_Init, Stage_Done>;
	struct Error_Underflow;
	struct Error_Overflow;
	using Error = ::std::variant<Error_Underflow, Error_Overflow>;
	struct Labelled_Tagged;
	struct Labelled_Bare;
	using Labelled = ::std::variant<Labelled_Tagged, Labelled_Bare>;
	struct b_One;
	struct b_Two;
	using b = ::std::variant<b_One, b_Two>;
	struct Builtin_Any;
	struct Builtin_Boolean;
	struct Builtin_Int;
	struct Builtin_Native;
	using Builtin = ::std::variant<Builtin_Any, Builtin_Boolean, Builtin_Int, Builtin_Native>;
	struct Rank_Unranked;
	struct Rank_Ranked;
	using Rank = ::std::variant<Rank_Unranked, Rank_Ranked>;
	struct clash_Candidate_None;
	struct clash_Candidate_Match;
	using clash_Candidate = ::std::variant<clash_Candidate_None, clash_Candidate_Match>;
	struct clash_Tag_Application;
	struct clash_Tag_Private;
	using clash_Tag = ::std::variant<clash_Tag_Application, clash_Tag_Private>;
	struct clash_Change_Set;
	struct clash_Change_Clear;
	using clash_Change = ::std::variant<clash_Change_Set, clash_Change_Clear>;
	using Clash_after_future = ::api::future<detail::Clash_after_future_op>;
	using Clash_tagged_future = ::api::future<detail::Clash_tagged_future_op>;
	using Waiter_poll_future_future = ::api::future<detail::Waiter_poll_future_future_op>;
	using Feed_next_future = ::api::future<detail::Feed_next_future_op>;
	using Countdown_next_future = ::api::future<detail::Countdown_next_future_op>;
	using Retired_next_future = ::api::future<detail::Retired_next_future_op>;
	using Local_later_future = ::api::future<detail::Local_later_future_op>;
	using Local_poll_count_future = ::api::future<detail::Local_poll_count_future_op>;
	using LocalWaiter_poll_ready_future = ::api::future<detail::LocalWaiter_poll_ready_future_op>;
	using Handle_later_future = ::api::future<detail::Handle_later_future_op>;
	using Counter_plus_later_future = ::api::future<detail::Counter_plus_later_future_op>;
	using Channel_recv_future = ::api::future<detail::Channel_recv_future_op>;
	using Channel_send_later_future = ::api::future<detail::Channel_send_later_future_op>;
	using Channel_tagged_future = ::api::future<detail::Channel_tagged_future_op>;
	using Channel_itemized_future = ::api::future<detail::Channel_itemized_future_op>;
	using Channel_poll_take_future = ::api::future<detail::Channel_poll_take_future_op>;
	using Channel_poll_measure_future = ::api::future<detail::Channel_poll_measure_future_op>;
	using Channel_recv_sized_future = ::api::future<detail::Channel_recv_sized_future_op>;
	using Channel_recv_nonempty_future = ::api::future<detail::Channel_recv_nonempty_future_op>;
	using Tally_bump_later_future = ::api::future<detail::Tally_bump_later_future_op>;
	using Gauge_later_future = ::api::future<detail::Gauge_later_future_op>;
	using Gauge_reaches_future = ::api::future<detail::Gauge_reaches_future_op>;
	using corner_later_future = ::api::future<detail::corner_later_future_op>;
	using fill_4_future = ::api::future<detail::fill_4_future_op>;
	using fill_2_future = ::api::future<detail::fill_2_future_op>;
	using drive_future = ::api::future<detail::drive_future_op>;
	using halved_future = ::api::future<detail::halved_future_op>;
	using yielded_future = ::api::future<detail::yielded_future_op>;
	using after_future = ::api::future<detail::after_future_op>;
	using within_future = ::api::future<detail::within_future_op>;
	using counted_future = ::api::future<detail::counted_future_op>;
	using log_later_future = ::api::future<detail::log_later_future_op>;
	using local_or_new_later_future = ::api::future<detail::local_or_new_later_future_op>;
	using local_after_future = ::api::future<detail::local_after_future_op>;
	using local_later_future = ::api::future<detail::local_later_future_op>;
	using poll_tripled_future = ::api::future<detail::poll_tripled_future_op>;
	using byte_len_future = ::api::future<detail::byte_len_future_op>;
	using parse_later_future = ::api::future<detail::parse_later_future_op>;
	using either_later_future = ::api::future<detail::either_later_future_op>;
	using unescaped_later_future = ::api::future<detail::unescaped_later_future_op>;
	using unescaped_maybe_future = ::api::future<detail::unescaped_maybe_future_op>;
	using level_later_future = ::api::future<detail::level_later_future_op>;
	using emit_later_future = ::api::future<detail::emit_later_future_op>;
	using log_on_drop_future = ::api::future<detail::log_on_drop_future_op>;
}
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic pop
#elif defined(_MSC_VER)
#pragma warning(pop)
#endif
#endif

// serdejson.hpp
#ifndef API_HPP_CRATE4
#define API_HPP_CRATE4
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#elif defined(_MSC_VER)
#pragma warning(push)
#pragma warning(disable : 4996)
#endif
#include "rt.hpp"
namespace api {
	class RUST2_FFI_CONSUMABLE Json {
		RUST2_FFI_UNCONSUMED Json(Json &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Json &operator=(Json &&other) noexcept = default;
		static ::api::result<::api::Json, ::api::buf> parse(::api::input input_);
		RUST2_FFI_CALLABLE ::api::buf to_text() const;
	};
}
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic pop
#elif defined(_MSC_VER)
#pragma warning(pop)
#endif
#endif

// std.hpp
#ifndef API_HPP_CRATE1
#define API_HPP_CRATE1
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#elif defined(_MSC_VER)
#pragma warning(push)
#pragma warning(disable : 4996)
#endif
#include "rt.hpp"
namespace api {
	class RUST2_FFI_CONSUMABLE Instant {
		RUST2_FFI_UNCONSUMED Instant(Instant &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Instant &operator=(Instant &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Instant(const Instant &other) = default;
		Instant &operator=(const Instant &other) = default;
		friend bool operator==(const Instant &a__, const Instant &b_);
		friend bool operator!=(const Instant &a__, const Instant &b_);
		friend bool operator<(const Instant &a__, const Instant &b_);
		friend bool operator<=(const Instant &a__, const Instant &b_);
		friend bool operator>(const Instant &a__, const Instant &b_);
		friend bool operator>=(const Instant &a__, const Instant &b_);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
	};
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const Instant &a__, const Instant &b_);
	#endif
}
namespace std {
	template <> struct hash<::api::Instant> {
		size_t operator()(const ::api::Instant &value) const noexcept;
	};
}
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic pop
#elif defined(_MSC_VER)
#pragma warning(pop)
#endif
#endif

// tls.hpp
#ifndef API_HPP_CRATE6
#define API_HPP_CRATE6
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#elif defined(_MSC_VER)
#pragma warning(push)
#pragma warning(disable : 4996)
#endif
#include "rt.hpp"
namespace api {
	class RUST2_FFI_CONSUMABLE tls_Session {
		RUST2_FFI_UNCONSUMED tls_Session(tls_Session &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tls_Session &operator=(tls_Session &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::api::buf alpn() const;
		RUST2_FFI_CALLABLE ::api::tls_policy_Lifetime lifetime() const;
	};
	class RUST2_FFI_CONSUMABLE tls_Config {
		RUST2_FFI_UNCONSUMED tls_Config(tls_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tls_Config &operator=(tls_Config &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED tls_Config(const tls_Config &other) = default;
		tls_Config &operator=(const tls_Config &other) = default;
	};
	class RUST2_FFI_CONSUMABLE tls_Cert {
		RUST2_FFI_UNCONSUMED tls_Cert(tls_Cert &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tls_Cert &operator=(tls_Cert &&other) noexcept = default;
	};
	class RUST2_FFI_CONSUMABLE tls_Resolver {
		RUST2_FFI_UNCONSUMED tls_Resolver(tls_Resolver &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tls_Resolver &operator=(tls_Resolver &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED tls_Resolver(const tls_Resolver &other) = default;
		tls_Resolver &operator=(const tls_Resolver &other) = default;
	};
	class RUST2_FFI_CONSUMABLE tls_Codec {
		RUST2_FFI_UNCONSUMED tls_Codec(tls_Codec &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tls_Codec &operator=(tls_Codec &&other) noexcept = default;
	};
	struct tls_policy_Lifetime {
		::std::uint32_t seconds;
		friend bool operator==(const tls_policy_Lifetime &a__, const tls_policy_Lifetime &b_);
		friend bool operator!=(const tls_policy_Lifetime &a__, const tls_policy_Lifetime &b_);
		::api::buf debug() const;
	};
	inline bool operator==(const tls_policy_Lifetime &a__, const tls_policy_Lifetime &b_);
}
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic pop
#elif defined(_MSC_VER)
#pragma warning(pop)
#endif
#endif

// tracingcore.hpp
#ifndef API_HPP_CRATE3
#define API_HPP_CRATE3
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#elif defined(_MSC_VER)
#pragma warning(push)
#pragma warning(disable : 4996)
#endif
#include "rt.hpp"
namespace api {
	enum class Level {
		error = 0,
		warn = 1,
		info = 2,
		debug = 3,
		trace = 4,
	};
	inline bool operator==(::api::Level a__, ::api::Level b_);
	inline bool operator!=(::api::Level a__, ::api::Level b_);
	inline bool operator<(::api::Level a__, ::api::Level b_);
	inline bool operator<=(::api::Level a__, ::api::Level b_);
	inline bool operator>(::api::Level a__, ::api::Level b_);
	inline bool operator>=(::api::Level a__, ::api::Level b_);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(::api::Level a__, ::api::Level b_);
	#endif
	inline ::api::buf Level_to_string(::api::Level self);
}
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic pop
#elif defined(_MSC_VER)
#pragma warning(pop)
#endif
#endif

// url.hpp
#ifndef API_HPP_CRATE5
#define API_HPP_CRATE5
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#elif defined(_MSC_VER)
#pragma warning(push)
#pragma warning(disable : 4996)
#endif
#include "rt.hpp"
namespace api {
	class RUST2_FFI_CONSUMABLE Url {
		RUST2_FFI_UNCONSUMED Url(Url &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Url &operator=(Url &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Url(const Url &other) = default;
		Url &operator=(const Url &other) = default;
		friend bool operator==(const Url &a__, const Url &b_);
		friend bool operator!=(const Url &a__, const Url &b_);
		friend bool operator<(const Url &a__, const Url &b_);
		friend bool operator<=(const Url &a__, const Url &b_);
		friend bool operator>(const Url &a__, const Url &b_);
		friend bool operator>=(const Url &a__, const Url &b_);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::api::buf to_string() const;
		RUST2_FFI_CALLABLE ::api::buf debug() const;
		static ::api::result<::api::Url, ::api::buf> parse(::api::input input_);
	};
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const Url &a__, const Url &b_);
	#endif
}
namespace std {
	template <> struct hash<::api::Url> {
		size_t operator()(const ::api::Url &value) const noexcept;
	};
}
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic pop
#elif defined(_MSC_VER)
#pragma warning(pop)
#endif
#endif
