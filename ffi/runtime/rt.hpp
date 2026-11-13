// The runtime every C++ binding shares: handles, bytes, results, wakers, futures, and
// readers over the C ABI. rust2-ffi copies it into each header, inside the export's
// namespace, after `detail` names the export's C types. It throws nothing and reads no
// type information, so it builds with exceptions and RTTI off.

// Clang's checks of ownership, which a caller enables with `-Wconsumed -Wdangling`: an
// object is used after a move, or a view outlives the bytes it reads. Other compilers see
// nothing. A method callable "unknown" takes a reference parameter's object, whose state
// Clang does not track.
#if defined(__clang__)
#define RUST2_FFI_LIFETIMEBOUND [[clang::lifetimebound]]
#define RUST2_FFI_CONSUMABLE [[clang::consumable(unconsumed)]]
#define RUST2_FFI_CALLABLE [[clang::callable_when("unconsumed", "unknown")]]
#define RUST2_FFI_CONSUMES [[clang::callable_when("unconsumed", "unknown"), clang::set_typestate(consumed)]]
#define RUST2_FFI_UNCONSUMED [[clang::return_typestate(unconsumed)]]
#define RUST2_FFI_MOVED_FROM [[clang::return_typestate(consumed)]]
#else
#define RUST2_FFI_LIFETIMEBOUND
#define RUST2_FFI_CONSUMABLE
#define RUST2_FFI_CALLABLE
#define RUST2_FFI_CONSUMES
#define RUST2_FFI_UNCONSUMED
#define RUST2_FFI_MOVED_FROM
#endif

namespace detail {

[[noreturn]] inline void fail(const char *what) noexcept {
	::std::fputs(what, stderr);
	::std::fputc('\n', stderr);
	::std::abort();
}

struct access;

// A `[u8; N]`'s C struct, of its bytes, and back.
template <class C, ::std::size_t N> C lower_array(const ::std::array<::std::uint8_t, N> &a) noexcept {
	static_assert(sizeof(C::items) == N, "an array's C struct holds its N bytes");
	C c;
	::std::memcpy(c.items, a.data(), N);
	return c;
}

template <::std::size_t N, class C> ::std::array<::std::uint8_t, N> lift_array(const C &c) noexcept {
	::std::array<::std::uint8_t, N> a;
	::std::memcpy(a.data(), c.items, N);
	return a;
}

// A count of references, held by C++ and, once given a waker, by Rust: freed with the last.
struct node {
	::std::atomic<::std::size_t> refs{1};

	node() = default;
	node(const node &) = delete;
	node &operator=(const node &) = delete;
	virtual ~node() = default;
	virtual void wake() noexcept = 0;

	void ref() noexcept {
		refs.fetch_add(1, ::std::memory_order_relaxed);
	}

	void unref() noexcept {
		if (refs.fetch_sub(1, ::std::memory_order_acq_rel) == 1) {
			delete this;
		}
	}
};

inline void wake_node(void *data) noexcept {
	static_cast<node *>(data)->wake();
}

inline void drop_node(void *data) noexcept {
	static_cast<node *>(data)->unref();
}

// `n` as a waker that borrows its reference, which Rust reuses when given it again.
inline c_waker lend(node *n) noexcept {
	return c_waker{&wake_node, &drop_node, n};
}

// `n` as a waker holding a new reference, which Rust drops.
inline c_waker give(node *n) noexcept {
	n->ref();
	return lend(n);
}

// A waker Rust returned, which wakes through Rust until its last owner drops it.
class returned {
	c_waker raw_;

public:
	explicit returned(c_waker raw) noexcept : raw_(raw) {}
	returned(returned &&other) noexcept : raw_(other.raw_) {
		other.raw_.drop = nullptr;
	}
	returned(const returned &) = delete;
	returned &operator=(const returned &) = delete;
	returned &operator=(returned &&) = delete;
	~returned() {
		if (raw_.drop != nullptr) {
			raw_.drop(raw_.data);
		}
	}

	void operator()() const noexcept {
		raw_.wake(raw_.data);
	}
};

// An owned reference to a node.
template <class T>
class ptr {
	T *node_;

public:
	// Adopts a reference already counted.
	explicit ptr(T *node) noexcept : node_(node) {}
	ptr(const ptr &other) noexcept : node_(other.node_) {
		node_->ref();
	}
	ptr(ptr &&other) noexcept : node_(::std::exchange(other.node_, nullptr)) {}
	ptr &operator=(const ptr &) = delete;
	ptr &operator=(ptr &&) = delete;
	~ptr() {
		if (node_ != nullptr) {
			node_->unref();
		}
	}
	T *operator->() const noexcept {
		return node_;
	}
};

// A finished poll's value: none, then the output.
template <class T>
class slot {
	::std::optional<T> value_;

public:
	template <class... A>
	void emplace(A &&...args) {
		value_.emplace(::std::forward<A>(args)...);
	}
	explicit operator bool() const noexcept {
		return value_.has_value();
	}
	T take() {
		return ::std::move(*value_);
	}
	::std::optional<T> release() {
		return ::std::move(value_);
	}
};

template <>
class slot<void> {
	bool ready_ = false;

public:
	void emplace() noexcept {
		ready_ = true;
	}
	explicit operator bool() const noexcept {
		return ready_;
	}
	void take() noexcept {}
	bool release() noexcept {
		return ready_;
	}
};

// Frees a handle of the C type `R` with its `_free`.
template <class R, void (*Free)(R *)>
struct deleter {
	void operator()(R *raw) const noexcept {
		Free(raw);
	}
};

// The handle of an object one C++ object owns, which moves and never copies. A moved-from
// one holds null, which any call aborts on.
template <class R, void (*Free)(R *)>
class unique {
	::std::unique_ptr<R, deleter<R, Free>> ptr_;

public:
	explicit unique(R *raw) noexcept : ptr_(raw) {}

	R *get() const noexcept {
		return ptr_.get();
	}
	// The handle for a call that moves the value out.
	R *own() noexcept {
		return ptr_.get();
	}
	// `raw`, which Rust lends for a call and frees itself.
	static unique lend(R *raw) noexcept {
		return unique(raw);
	}
	// Let go of the lent `raw` unfreed, or say how C++ kept it.
	const char *give_back(R *raw) noexcept {
		return ptr_.release() == raw ? nullptr : "a lent object was moved out of";
	}
};

// The handle of an object whose copies share it, since its clone is a value no call
// tells apart from the original but by an `Arc`'s or `Rc`'s count, which counts one for
// every copy. The last copy frees it, on whichever thread drops it.
template <class R, void (*Free)(R *), R *(*Clone)(const R *), bool (*Borrowed)(const R *)>
class shared {
	::std::shared_ptr<R> ptr_;

	shared() noexcept = default;

public:
	explicit shared(R *raw) : ptr_(raw, deleter<R, Free>{}) {}

	R *get() const noexcept {
		return ptr_.get();
	}
	// The handle for a call that moves the value out, held by this copy alone: a clone's
	// while another copy shares it, as Rust would move a clone out and leave the copy be.
	// A pending future made from a copy holds no copy here, but borrows the value in Rust,
	// as though it held one. A moved-from copy is left for the call to refuse, naming it.
	R *own() {
		if (ptr_.use_count() > 1 || (ptr_ != nullptr && Borrowed(ptr_.get()))) {
			*this = shared(Clone(ptr_.get()));
		}
		return ptr_.get();
	}
	// `raw`, which Rust lends for a call and frees itself.
	static shared lend(R *raw) {
		shared s;
		s.ptr_ = ::std::shared_ptr<R>(raw, [](R *) noexcept {});
		return s;
	}
	// Let go of the lent `raw`, or say how C++ kept it: a copy left past the call would
	// outlive the loan.
	const char *give_back(R *raw) noexcept {
		if (ptr_.get() != raw) {
			return "a lent object was moved out of";
		}
		if (ptr_.use_count() != 1) {
			return "a lent object was copied past its call";
		}
		ptr_.reset();
		return nullptr;
	}
};

// The bytes of a view, never null, so a view of none is told apart from a missing one.
inline const ::std::uint8_t *bytes(const void *data, ::std::size_t len) noexcept {
	static const ::std::uint8_t empty = 0;
	if (data == nullptr && len != 0) {
		fail("a view of null bytes has a length");
	}
	return len == 0 ? &empty : static_cast<const ::std::uint8_t *>(data);
}

} // namespace detail

class buf;

// An object Rust lends a closure as `&mut`: its methods through `->`, but neither a move
// nor a copy of it, which would keep the loan past the call.
template <class T>
class borrow {
	T &object_;

public:
	explicit borrow(T &object) noexcept : object_(object) {}
	borrow(const borrow &) = delete;
	borrow &operator=(const borrow &) = delete;
	T *operator->() const noexcept {
		return &object_;
	}
};

// Bytes lent to a call: text or binary, read during the call and never kept. A contiguous
// range, so from C++20 it converts to `std::span<const std::uint8_t>`, and back.
class view {
	const ::std::uint8_t *data_;
	::std::size_t size_;

public:
	view() noexcept : data_(detail::bytes(nullptr, 0)), size_(0) {}
	view(const void *data, ::std::size_t size) noexcept : data_(detail::bytes(data, size)), size_(size) {}
	view(const char *text) noexcept : view(text, ::std::strlen(text)) {}
	// Any contiguous range of bytes, such as `std::string`, `std::string_view`, `std::vector<uint8_t>`, or `buf`.
	template <class R, class = ::std::enable_if_t<sizeof(*::std::declval<const R &>().data()) == 1 &&
	                                              !::std::is_pointer_v<::std::decay_t<R>>>,
	          class = decltype(::std::declval<const R &>().size())>
	view(const R &range RUST2_FFI_LIFETIMEBOUND) noexcept : view(range.data(), range.size()) {}

	const ::std::uint8_t *data() const noexcept {
		return data_;
	}
	::std::size_t size() const noexcept {
		return size_;
	}
	bool empty() const noexcept {
		return size_ == 0;
	}
	const ::std::uint8_t *begin() const noexcept {
		return data_;
	}
	const ::std::uint8_t *end() const noexcept {
		return data_ + size_;
	}
	::std::string_view str() const noexcept {
		return ::std::string_view(reinterpret_cast<const char *>(data_), size_);
	}

	friend struct detail::access;
};

// Owned bytes: returned by Rust without a copy, or copied from C++. They outlive the object
// they came from; a view or span of them lives only as long as they do.
class buf {
	// Rust's when `owner` is set, else C++'s, allocated with `new[]` unless empty.
	detail::c_buf raw_;

	explicit buf(detail::c_buf raw) noexcept : raw_(raw) {}

	void reset() noexcept {
		if (raw_.owner != nullptr) {
			detail::c_buf_free(raw_);
		} else if (raw_.len != 0) {
			delete[] raw_.ptr;
		}
		raw_ = detail::c_buf{nullptr, 0, nullptr};
	}

public:
	buf() noexcept : raw_{nullptr, 0, nullptr} {}
	// A copy of `bytes`.
	explicit buf(view bytes) : buf() {
		if (!bytes.empty()) {
			auto *copy = new ::std::uint8_t[bytes.size()];
			::std::memcpy(copy, bytes.data(), bytes.size());
			raw_ = detail::c_buf{copy, bytes.size(), nullptr};
		}
	}
	buf(const buf &other) : buf(view(other)) {}
	buf(buf &&other) noexcept : raw_(::std::exchange(other.raw_, detail::c_buf{nullptr, 0, nullptr})) {}
	buf &operator=(const buf &other) {
		if (this != &other) {
			*this = buf(other);
		}
		return *this;
	}
	buf &operator=(buf &&other) noexcept {
		::std::swap(raw_, other.raw_);
		return *this;
	}
	~buf() {
		reset();
	}

	const ::std::uint8_t *data() const noexcept RUST2_FFI_LIFETIMEBOUND {
		return detail::bytes(raw_.ptr, raw_.len);
	}
	::std::size_t size() const noexcept {
		return raw_.len;
	}
	bool empty() const noexcept {
		return raw_.len == 0;
	}
	const ::std::uint8_t *begin() const noexcept {
		return data();
	}
	const ::std::uint8_t *end() const noexcept {
		return data() + size();
	}
	::std::string_view str() const noexcept RUST2_FFI_LIFETIMEBOUND {
		return view(*this).str();
	}

	friend struct detail::access;
};

// Items lent to a call whose items hold bytes, such as `view`s: a `std::vector<T>` in place,
// or any other range whose items convert to `T`, such as the `std::vector<buf>` a call
// returned or a `std::vector<std::string>`, converted item by item.
template <class T>
class items {
	::std::vector<T> converted_;
	const T *data_;
	::std::size_t size_;

public:
	items(const ::std::vector<T> &vector RUST2_FFI_LIFETIMEBOUND) noexcept : data_(vector.data()), size_(vector.size()) {}
	// Copied, since a list's array may not outlive the call's first full-expression.
	items(::std::initializer_list<T> list) : converted_(list), data_(converted_.data()), size_(converted_.size()) {}
	template <class R,
	          class = ::std::enable_if_t<!::std::is_same_v<::std::decay_t<R>, ::std::vector<T>> &&
	                                     !::std::is_same_v<::std::decay_t<R>, items> &&
	                                     ::std::is_convertible_v<decltype(*::std::begin(::std::declval<const R &>())), T>>>
	items(const R &range RUST2_FFI_LIFETIMEBOUND) : converted_(::std::begin(range), ::std::end(range)),
	                                                 data_(converted_.data()), size_(converted_.size()) {}
	// A copy would point into the original's converted items.
	items(const items &) = delete;
	items &operator=(const items &) = delete;
	// A moved vector keeps its array, so the pointer stays valid.
	items(items &&other) noexcept = default;

	const T *begin() const noexcept {
		return data_;
	}
	const T *end() const noexcept {
		return data_ + size_;
	}
	::std::size_t size() const noexcept {
		return size_;
	}
	bool empty() const noexcept {
		return size_ == 0;
	}
};

// What a fallible call returns: its value, or the error that stopped it.
template <class T, class E>
class result {
	::std::variant<T, E> value_;

	template <::std::size_t I, class A>
	result(::std::in_place_index_t<I> index, A &&arg) : value_(index, ::std::forward<A>(arg)) {}

	void check() const noexcept {
		if (value_.index() != 0) {
			detail::fail("value() of a result that holds an error");
		}
	}

public:
	static result ok(T value) {
		return result(::std::in_place_index<0>, ::std::move(value));
	}
	static result err(E error) {
		return result(::std::in_place_index<1>, ::std::move(error));
	}

	bool has_value() const noexcept {
		return value_.index() == 0;
	}
	explicit operator bool() const noexcept {
		return has_value();
	}
	// The value, aborting if it holds an error.
	T &value() & {
		check();
		return *::std::get_if<0>(&value_);
	}
	const T &value() const & {
		check();
		return *::std::get_if<0>(&value_);
	}
	T &&value() && {
		check();
		return ::std::move(*::std::get_if<0>(&value_));
	}
	T &operator*() & {
		return value();
	}
	const T &operator*() const & {
		return value();
	}
	T &&operator*() && {
		return ::std::move(*this).value();
	}
	T *operator->() {
		return &value();
	}
	const T *operator->() const {
		return &value();
	}
	// The error, aborting if it holds a value.
	E &error() & {
		if (value_.index() != 1) {
			detail::fail("error() of a result that holds a value");
		}
		return *::std::get_if<1>(&value_);
	}
	const E &error() const & {
		return const_cast<result *>(this)->error();
	}
	E &&error() && {
		return ::std::move(error());
	}
};

template <class E>
class result<void, E> {
	::std::optional<E> error_;

public:
	static result ok() {
		return result();
	}
	static result err(E error) {
		result r;
		r.error_.emplace(::std::move(error));
		return r;
	}

	bool has_value() const noexcept {
		return !error_.has_value();
	}
	explicit operator bool() const noexcept {
		return has_value();
	}
	// Aborts if it holds an error.
	void value() const noexcept {
		if (error_.has_value()) {
			detail::fail("value() of a result that holds an error");
		}
	}
	E &error() & {
		if (!error_.has_value()) {
			detail::fail("error() of a result that holds a value");
		}
		return *error_;
	}
	const E &error() const & {
		return const_cast<result *>(this)->error();
	}
	E &&error() && {
		return ::std::move(error());
	}
};

// Wakes a future's poller: a shared callable, which Rust may call from any thread, any
// number of times, until it lets go.
class waker {
	detail::node *node_;

	template <class F>
	struct callable final : detail::node {
		F f_;
		explicit callable(F f) : f_(::std::move(f)) {}
		void wake() noexcept override {
			f_();
		}
	};

public:
	template <class F, class = ::std::enable_if_t<!::std::is_same_v<::std::decay_t<F>, waker>>>
	explicit waker(F f) : node_(new callable<F>(::std::move(f))) {}
	waker(const waker &other) noexcept : node_(other.node_) {
		node_->ref();
	}
	waker &operator=(const waker &other) noexcept {
		other.node_->ref();
		node_->unref();
		node_ = other.node_;
		return *this;
	}
	~waker() {
		node_->unref();
	}

	void wake() const noexcept {
		node_->wake();
	}

	friend struct detail::access;
};

namespace detail {

// `f`, any future of nothing, such as a binding's `future` of `void`, as the future Rust
// polls: from any thread, with a waker of its own each time, until its `poll` returns
// true, and destroyed once Rust drops it, finished or not.
template <class F>
c_foreign_future foreign_future(F f) {
	static_assert(::std::is_same_v<decltype(f.poll(::std::declval<const waker &>())), bool>,
	              "a future Rust polls has `bool poll(const waker &)`, as a future of void does");
	struct calls {
		static bool poll(void *data, c_waker w) noexcept {
			return static_cast<F *>(data)->poll(waker(returned(w)));
		}
		static void free(void *data) noexcept {
			delete static_cast<F *>(data);
		}
	};
	return c_foreign_future{&calls::poll, &calls::free, new F(::std::move(f))};
}

} // namespace detail

// What a future poll returns: the output when ready, or nothing yet.
template <class T>
using poll_t = ::std::conditional_t<::std::is_void_v<T>, bool, ::std::optional<T>>;

class task;

namespace detail {

// A future driven on an executor, which a `task` cancels.
struct task_node : node {
	virtual void cancel() noexcept = 0;
};

template <class Op, class Exec, class F>
class driver;

template <class Op, class Exec, class F>
void spawn(task &slot, typename Op::raw *raw, Exec exec, F f);

} // namespace detail

// A future running on an executor until it calls back with the output. Destroying it
// cancels the future: once that returns, the callback has finished or never runs, and the
// executor is never called again, so it may go too.
class task {
	detail::task_node *node_ = nullptr;

	void reset() noexcept {
		if (node_ != nullptr) {
			::std::exchange(node_, nullptr)->cancel();
		}
	}

public:
	task() noexcept = default;
	task(task &&other) noexcept : node_(::std::exchange(other.node_, nullptr)) {}
	task &operator=(task &&other) noexcept {
		if (this != &other) {
			reset();
			node_ = ::std::exchange(other.node_, nullptr);
		}
		return *this;
	}
	~task() {
		reset();
	}

	template <class Op, class Exec, class F>
	friend void detail::spawn(task &, typename Op::raw *, Exec, F);
};

// A Rust future, polled from C++: move-only, so one poller holds it at a time. Destroying
// it before it finishes cancels it, which loses nothing, since a value leaves Rust only in
// the poll that finishes it.
template <class Op>
class future {
	typename Op::raw *raw_;
	// The waker Rust holds from the last poll, which it reuses when given it again.
	detail::node *last_ = nullptr;

	explicit future(typename Op::raw *raw) noexcept : raw_(raw) {}

	void poll_into(const waker &w, detail::slot<typename Op::output> &out);

	friend struct detail::access;

public:
	using output = typename Op::output;

	future(future &&other) noexcept
	    : raw_(::std::exchange(other.raw_, nullptr)), last_(::std::exchange(other.last_, nullptr)) {}
	future &operator=(future &&other) noexcept {
		::std::swap(raw_, other.raw_);
		::std::swap(last_, other.last_);
		return *this;
	}
	~future() {
		Op::free(raw_);
	}

	// Poll once: the output when ready, else nothing, and `w` is woken when it is worth
	// polling again. Polling after it is ready aborts.
	poll_t<output> poll(const waker &w) {
		detail::slot<output> out;
		poll_into(w, out);
		return out.release();
	}

	// Block the calling thread until the output is ready.
	output get() &&;

	// Poll on `exec` until ready, then call `f` there with the output. `exec` is any
	// callable that takes a `void()` callable and runs it once, later, on any thread: never
	// within the call, which may come from inside Rust. Once the returned task is
	// destroyed, `exec` is never called again.
	template <class Exec, class F>
	task then(Exec exec, F f) &&;

	// From C++20, `co_await` it, polled and resumed on `exec`, as `then` takes it.
	// Destroying the suspended coroutine cancels the future, and once that returns, `exec`
	// is never called again. Declared at every standard, so the class is one definition
	// across translation units built at different ones.
	template <class Exec>
	auto via(Exec exec) &&;
};

namespace detail {

// What generated classes keep private, for the generated code.
struct access {
	static c_buf raw(const view &v) noexcept {
		return c_buf{v.data_, v.size_, nullptr};
	}
	static buf adopt(c_buf raw) noexcept {
		return buf(raw);
	}
	static ::std::optional<buf> adopt_option(c_buf raw) noexcept {
		if (raw.ptr == nullptr) {
			return ::std::nullopt;
		}
		return buf(raw);
	}
	static node *raw(const waker &w) noexcept {
		return w.node_;
	}
	template <class T>
	static auto raw(const T &object) noexcept {
		return object.raw_.get();
	}
	// The handle of an object a call moves the value out of.
	template <class T>
	static auto own(T &object) {
		return object.raw_.own();
	}
	// `raw` owned by a `T`.
	template <class T, class R>
	static T wrap(R *raw) {
		return T(decltype(T::raw_)(raw));
	}
	// `raw`, which Rust lends for a call, as a `T` that never frees it.
	template <class T, class R>
	static T lend(R *raw) {
		return T(decltype(T::raw_)::lend(raw));
	}
	template <class T, class R>
	static const char *give_back(T &object, R *raw) noexcept {
		return object.raw_.give_back(raw);
	}
};

inline c_buf none() noexcept {
	return c_buf{nullptr, 0, nullptr};
}

// A copy of `bytes`, anything a view takes, which Rust then owns: what a closure or a
// reader C++ implements returns Rust.
inline c_buf copy_bytes(view bytes) noexcept {
	return c_buf_copy(access::raw(bytes));
}

// `r`, any reader, lent for one call, which Rust reads from any thread, one call at a time:
// `read(uint8_t *, size_t)` returns a `result` of how many bytes it read, at most the size
// it was given and none only at its end, and `seek(int64_t, std::ios_base::seekdir)` one of
// the position from the start, as `seekg` takes them; a seek from the start past `INT64_MAX`,
// as Rust's `SeekFrom::Start` takes a `uint64_t`, calls `seek_start(uint64_t)` instead, and
// fails in a reader without one. Either fails with an error anything a view takes, such as
// a `std::string`, which Rust raises as an `io::Error` of it. Its errors cross as messages,
// as an `istream`'s, which keeps only a failed bit, could not.
template <class R, class = void>
struct has_seek_start : ::std::false_type {};

template <class R>
struct has_seek_start<R, ::std::void_t<decltype(::std::declval<R &>().seek_start(::std::uint64_t{}))>>
	: ::std::true_type {};

template <class R>
c_foreign_reader foreign_reader(R &r) noexcept {
	struct calls {
		static bool read(void *data, ::std::uint8_t *buf, ::std::size_t len, ::std::size_t *n, c_buf *err) noexcept {
			auto read = static_cast<R *>(data)->read(buf, len);
			if (!read) {
				*err = copy_bytes(read.error());
				return false;
			}
			*n = *read;
			return true;
		}
		static bool seek(void *data, int whence, ::std::int64_t offset, ::std::uint64_t *position, c_buf *err) noexcept {
			::std::ios_base::seekdir dir = whence == SEEK_SET   ? ::std::ios_base::beg
			                               : whence == SEEK_CUR ? ::std::ios_base::cur
			                                                    : ::std::ios_base::end;
			// From the start, the offset's bits are unsigned, past what an int64_t holds.
			if (whence == SEEK_SET && offset < 0) {
				if constexpr (has_seek_start<R>::value) {
					auto start = static_cast<R *>(data)->seek_start(static_cast<::std::uint64_t>(offset));
					if (!start) {
						*err = copy_bytes(start.error());
						return false;
					}
					*position = *start;
					return true;
				} else {
					*err = copy_bytes("a seek past INT64_MAX of a reader with no seek_start");
					return false;
				}
			}
			auto seek = static_cast<R *>(data)->seek(offset, dir);
			if (!seek) {
				*err = copy_bytes(seek.error());
				return false;
			}
			*position = *seek;
			return true;
		}
	};
	return c_foreign_reader{&calls::read, &calls::seek, ::std::addressof(r)};
}

// The arrays a call's lowered vectors point into, kept until the call returns.
class arena {
	::std::vector<::std::shared_ptr<void>> held_;

public:
	// `items` as the C array `C`, each lowered by `lower`: never null, so an empty one is
	// told apart from a missing one. Each item is mutable where `items` is, so a handle
	// moved in can be owned alone.
	// The array is no `std::vector`, whose `bool` form holds no array, and `items` may be
	// one, whose items are proxies rather than references.
	template <class C, class R, class F>
	C list(R &items, F lower) {
		using E = ::std::remove_const_t<::std::remove_pointer_t<decltype(C{}.ptr)>>;
		static const E empty{};
		::std::size_t len = items.size();
		if (len == 0) {
			return C{&empty, 0, nullptr};
		}
		::std::shared_ptr<E[]> array(new E[len]);
		::std::size_t i = 0;
		for (auto &&item : items) {
			array[i++] = lower(item);
		}
		const E *ptr = array.get();
		held_.push_back(::std::move(array));
		return C{ptr, len, nullptr};
	}
};

// The numbers of `items` as the C array `C`, in place: never null, so an empty one is
// told apart from a missing one.
template <class C, class T>
C block(const ::std::vector<T> &items) {
	static const T empty{};
	return C{items.empty() ? &empty : items.data(), items.size(), nullptr};
}

// The C array of numbers `c` Rust returned, copied whole, then freed.
template <class T, class C, class Free>
::std::vector<T> lift_block(C c, Free free) {
	::std::vector<T> out(c.ptr, c.ptr + c.len);
	free(c);
	return out;
}

// The C array `c` Rust returned, as `T`s each lifted by `lift`, which takes what its
// element holds; `free` then frees the array alone.
template <class T, class C, class F, class Free>
::std::vector<T> lift_list(C c, F lift, Free free) {
	::std::vector<T> out;
	out.reserve(c.len);
	for (::std::size_t i = 0; i < c.len; i++) {
		out.push_back(lift(c.ptr[i]));
	}
	c.len = 0;
	free(c);
	return out;
}

// An object Rust lends a closure for one call, as its class over the lent handle, which
// it gives back unfreed. Moving out of it, or keeping a copy past the call, would keep
// the borrow past the call, so it aborts.
template <class T, class R>
class lent {
	T object_;
	R *raw_;

	static ::std::remove_const_t<R> *mutable_(R *raw) noexcept {
		return const_cast<::std::remove_const_t<R> *>(raw);
	}

public:
	explicit lent(R *raw) : object_(access::lend<T>(mutable_(raw))), raw_(raw) {}
	lent(const lent &) = delete;
	lent &operator=(const lent &) = delete;
	~lent() {
		if (const char *kept = access::give_back(object_, mutable_(raw_))) {
			fail(kept);
		}
	}

	T &get() noexcept {
		return object_;
	}
};

template <class Op, class Exec, class F>
class driver final : public task_node {
	enum : int { idle, queued, running, done };

	::std::mutex mutex_;
	// Held across each call to the executor, so `cancel` returns only once none is under
	// way, and none starts after.
	::std::mutex posting_;
	// Null once the future finished or was cancelled.
	typename Op::raw *raw_;
	Exec exec_;
	F f_;
	::std::atomic<int> state_{idle};
	// Whether a wake landed while it polled, so it polls again.
	::std::atomic<bool> woken_{false};
	// The thread running its poll and callback, if any.
	::std::atomic<::std::thread::id> on_{};
	// Whether Rust holds a reference from the first poll, which it reuses.
	bool given_ = false;

	void post() {
		::std::lock_guard<::std::mutex> lock(posting_);
		// Cancelled since the wake that queued it, so the executor may be gone.
		if (state_.load(::std::memory_order_acquire) == done) {
			return;
		}
		ref();
		exec_([self = ptr<driver>(this)]() { self->run(); });
	}

	void run() {
		::std::unique_lock<::std::mutex> lock(mutex_);
		if (raw_ == nullptr) {
			return;
		}
		on_.store(::std::this_thread::get_id(), ::std::memory_order_relaxed);
		state_.store(running, ::std::memory_order_release);
		woken_.store(false, ::std::memory_order_release);
		slot<typename Op::output> out;
		c_waker w = given_ ? lend(this) : give(this);
		given_ = true;
		Op::poll(raw_, w, out);
		if (out) {
			state_.store(done, ::std::memory_order_release);
			Op::free(::std::exchange(raw_, nullptr));
			if constexpr (::std::is_void_v<typename Op::output>) {
				f_();
			} else {
				f_(out.take());
			}
		} else {
			int expect = running;
			// A wake during the poll lands in `woken_`: poll again, after what else is queued.
			state_.compare_exchange_strong(expect, idle, ::std::memory_order_acq_rel);
			if (woken_.exchange(false, ::std::memory_order_acq_rel)) {
				wake();
			}
		}
		on_.store(::std::thread::id(), ::std::memory_order_relaxed);
	}

public:
	driver(typename Op::raw *raw, Exec exec, F f) : raw_(raw), exec_(::std::move(exec)), f_(::std::move(f)) {}

	void wake() noexcept override {
		int s = state_.load(::std::memory_order_acquire);
		for (;;) {
			switch (s) {
			case idle:
				if (state_.compare_exchange_weak(s, queued, ::std::memory_order_acq_rel)) {
					post();
					return;
				}
				break;
			case running:
				woken_.store(true, ::std::memory_order_release);
				// It may have just gone idle, missing the flag.
				if (state_.compare_exchange_strong(s, running, ::std::memory_order_acq_rel)) {
					return;
				}
				break;
			default:
				return;
			}
		}
	}

	void cancel() noexcept override {
		// From its own callback, the future is already gone.
		if (on_.load(::std::memory_order_relaxed) != ::std::this_thread::get_id()) {
			::std::lock_guard<::std::mutex> lock(mutex_);
			state_.store(done, ::std::memory_order_release);
			Op::free(::std::exchange(raw_, nullptr));
		}
		// Wait out a post under way, which may have passed its check before `done`: after
		// this, the executor is never called again.
		{
			::std::lock_guard<::std::mutex> lock(posting_);
		}
		unref();
	}
};

template <class Op, class Exec, class F>
void spawn(task &slot, typename Op::raw *raw, Exec exec, F f) {
	auto *d = new driver<Op, Exec, F>(raw, ::std::move(exec), ::std::move(f));
	slot = task();
	slot.node_ = d;
	d->wake();
}

} // namespace detail

template <class Op>
void future<Op>::poll_into(const waker &w, detail::slot<output> &out) {
	detail::node *n = detail::access::raw(w);
	detail::c_waker raw = n == last_ ? detail::lend(n) : detail::give(n);
	last_ = n;
	Op::poll(raw_, raw, out);
}

template <class Op>
typename future<Op>::output future<Op>::get() && {
	struct parker {
		::std::mutex mutex;
		::std::condition_variable cv;
		bool woken = false;
	};
	auto p = ::std::make_shared<parker>();
	waker w([p]() {
		{
			::std::lock_guard<::std::mutex> lock(p->mutex);
			p->woken = true;
		}
		p->cv.notify_one();
	});
	for (;;) {
		{
			// Before the poll, so a wake that lands between its pending and the wait is kept.
			::std::lock_guard<::std::mutex> lock(p->mutex);
			p->woken = false;
		}
		detail::slot<output> out;
		poll_into(w, out);
		if (out) {
			return out.take();
		}
		::std::unique_lock<::std::mutex> lock(p->mutex);
		p->cv.wait(lock, [&]() { return p->woken; });
	}
}

template <class Op>
template <class Exec, class F>
task future<Op>::then(Exec exec, F f) && {
	task t;
	detail::spawn<Op>(t, ::std::exchange(raw_, nullptr), ::std::move(exec), ::std::move(f));
	return t;
}

#if defined(__cpp_impl_coroutine) && __cpp_impl_coroutine >= 201902L && __has_include(<coroutine>)
namespace detail {

template <class Op, class Exec>
class awaiter {
	typename Op::raw *raw_;
	Exec exec_;
	slot<typename Op::output> out_;
	task task_;

public:
	awaiter(typename Op::raw *raw, Exec exec) : raw_(raw), exec_(::std::move(exec)) {}
	awaiter(const awaiter &) = delete;
	awaiter &operator=(const awaiter &) = delete;
	~awaiter() {
		Op::free(raw_);
	}

	bool await_ready() const noexcept {
		return false;
	}

	void await_suspend(::std::coroutine_handle<> h) {
		// The task is in place before the first poll, which may resume the coroutine and
		// destroy this from the executor's thread.
		spawn<Op>(task_, ::std::exchange(raw_, nullptr), ::std::move(exec_), [this, h](auto &&...value) {
			out_.emplace(::std::forward<decltype(value)>(value)...);
			h.resume();
		});
	}

	typename Op::output await_resume() {
		return out_.take();
	}
};

} // namespace detail

template <class Op>
template <class Exec>
auto future<Op>::via(Exec exec) && {
	return detail::awaiter<Op, Exec>(::std::exchange(raw_, nullptr), ::std::move(exec));
}
#endif
