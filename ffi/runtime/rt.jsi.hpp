// The runtime every React Native binding's JSI module shares: values crossing between
// JavaScript and the C ABI, handles, bytes, futures as promises, and what JavaScript
// implements for Rust. rust2-ffi copies it into each module, inside the export's namespace,
// after the module names the export's C items: `c_buf` for `<prefix>_buf`, and so on.
//
// JavaScript runs on one thread, the runtime's, which every jsi value is touched on: a
// call Rust makes into JavaScript from another thread posts to it through the
// `CallInvoker` React Native gives the module, and waits for its result.

namespace r {

namespace jsi = facebook::jsi;

// Run a job on the JavaScript thread, later.
using post_t = ::std::function<void(::std::function<void(jsi::Runtime &)>)>;

[[noreturn]] inline void fail(const ::std::string &what) noexcept {
#if defined(__ANDROID__)
	__android_log_write(ANDROID_LOG_FATAL, "rust2", what.c_str());
#endif
	::std::fputs(what.c_str(), stderr);
	::std::fputc('\n', stderr);
	::std::abort();
}

// What `rt.ts` gave the module, which are values of its runtime: released as that runtime
// is destroyed, while its heap still is, since a jsi value freed after would touch freed
// memory.
struct values {
	jsi::Object kit, arrays;
	jsi::Function pending, bytes, uint8, range, type, error, typed_tag, int_, char_, u64_halves, i64_halves,
	    u128_words, f32_bits, f64_bits, bytes_copy, opaque_made, opaque_bytes, is_cancelled, cancel_call, cancelled,
	    ready;
	jsi::Value none, handle, waker;

	values(jsi::Runtime &rt, jsi::Object k)
	    : kit(::std::move(k)), arrays(kit.getPropertyAsObject(rt, "typedArrays")), pending(fn(rt, "pending")), bytes(fn(rt, "bytes")), uint8(fn(rt, "Uint8Array")),
	      range(fn(rt, "RangeError")), type(fn(rt, "TypeError")), error(fn(rt, "Error")), typed_tag(fn(rt, "typedTag")),
	      int_(fn(rt, "int")), char_(fn(rt, "char")), u64_halves(fn(rt, "u64Halves")), i64_halves(fn(rt, "i64Halves")),
	      u128_words(fn(rt, "u128Words")), f32_bits(fn(rt, "f32Bits")), f64_bits(fn(rt, "f64Bits")),
	      bytes_copy(fn(rt, "bytesCopy")), opaque_made(fn(rt, "opaqueMade")), opaque_bytes(fn(rt, "opaqueBytes")),
	      is_cancelled(fn(rt, "isCancelled")), cancel_call(fn(rt, "cancelCall")), cancelled(fn(rt, "cancelled")),
	      ready(fn(rt, "ready")), none(kit.getProperty(rt, "None")), handle(kit.getProperty(rt, "Handle")),
	      waker(kit.getProperty(rt, "Waker")) {}

	jsi::Function fn(jsi::Runtime &rt, const char *name) {
		return kit.getPropertyAsFunction(rt, name);
	}
};

// The runtime the module is installed in. A reload installs the module into a new one, and
// the old one's teardown marks it gone, so what Rust still holds of it checks `alive`, and
// leaks its jsi values rather than touch a runtime that is gone.
struct world {
	jsi::Runtime *rt;
	::std::thread::id js;
	post_t post;
	::std::atomic<bool> alive{true};
	::std::unique_ptr<values> v;
	// Who holds the runtime, a stack of threads: the JavaScript thread at its bottom, and above
	// it each thread that took it while the one below waited in a call into Rust. `depth` is
	// how many took it, and `free` whether the top one waits in a call into Rust now.
	::std::mutex m;
	::std::condition_variable cv;
	bool free = false;
	::std::size_t depth = 0;

	world(jsi::Runtime &rt, post_t post, jsi::Object kit)
	    : rt(&rt), js(::std::this_thread::get_id()), post(::std::move(post)),
	      v(::std::make_unique<values>(rt, ::std::move(kit))) {}

	bool here() const {
		return ::std::this_thread::get_id() == js;
	}
};

// What a runtime holds of the module until it is destroyed, which then marks its world gone
// and releases its values while it still can.
struct teardown {
	::std::shared_ptr<world> w;
	explicit teardown(::std::shared_ptr<world> w) : w(::std::move(w)) {}
	teardown(const teardown &) = delete;
	teardown &operator=(const teardown &) = delete;
	~teardown() {
		{
			::std::lock_guard<::std::mutex> lock(w->m);
			w->alive.store(false, ::std::memory_order_release);
		}
		// A thread waiting for the runtime to run a call into JavaScript never gets it now.
		w->cv.notify_all();
		w->v.reset();
	}
};

// The key a runtime holds its `teardown` under: this module's own, by an address no other
// module's copy of this runtime shares.
inline jsi::UUID key() {
	static const char self = 0;
	auto at = static_cast<::std::uint64_t>(reinterpret_cast<::std::uintptr_t>(&self));
	return jsi::UUID(0x72757374, 0x0032, 0x4a53, 0x4900, at & 0xffffffffffffULL);
}

// The world of `rt`, which may be one of several runtimes alive at once: this thread's last,
// unless it was another's, or is gone and its address reused.
inline const ::std::shared_ptr<world> &of(jsi::Runtime &rt) {
	thread_local ::std::shared_ptr<world> last;
	if (last == nullptr || last->rt != &rt || !last->alive.load(::std::memory_order_acquire)) {
		auto t = ::std::static_pointer_cast<teardown>(rt.getRuntimeData(key()));
		if (t == nullptr) {
			fail("the JSI module is not installed in this runtime");
		}
		last = t->w;
	}
	return last;
}

inline values &W(jsi::Runtime &rt) {
	return *of(rt)->v;
}

// The world whose JavaScript this thread runs now, outside any call into Rust, or null: a
// drop Rust makes meanwhile, as the garbage collector's sweep may, posts what it calls of
// JavaScript.
inline thread_local world *running = nullptr;

// Who runs JavaScript, which one thread does at a time: the JavaScript thread, but while
// it waits in a call into Rust, which frees the runtime for a call Rust makes into
// JavaScript from any thread meanwhile, as a lent closure may be called on another thread
// until its call returns. Each thread that takes it runs on top of the one it took it from,
// as nested calls do, so a call into Rust takes the runtime back only once each thread
// that took it meanwhile gave it back, even one that waits in a call into Rust itself.

// Run the call into Rust `f` with the runtime freed, taking it back once `f` returns and
// every thread that took it meanwhile gave it back.
template <class F>
decltype(auto) unlocked(jsi::Runtime &rt, F &&f) {
	world &w = *of(rt);
	::std::size_t at;
	{
		::std::lock_guard<::std::mutex> lock(w.m);
		at = w.depth;
		w.free = true;
	}
	w.cv.notify_all();
	struct relock {
		world &w;
		::std::size_t at;
		world *was;
		~relock() {
			::std::unique_lock<::std::mutex> lock(w.m);
			w.cv.wait(lock, [&] { return w.free && w.depth == at; });
			w.free = false;
			running = was;
		}
	} back{w, at, ::std::exchange(running, nullptr)};
	return f();
}

// Take the runtime a call into Rust freed, holding `lock` on `w.m`.
inline void taken(world &w) {
	w.free = false;
	w.depth++;
}

// Give back the runtime this thread took.
struct given {
	world &w;
	world *was = ::std::exchange(running, &w);
	~given() {
		running = was;
		{
			::std::lock_guard<::std::mutex> lock(w.m);
			w.depth--;
			w.free = true;
		}
		w.cv.notify_all();
	}
};

// Run `drop`, a drop of Rust's that the collector made, on a posted job: a collection runs
// inside JavaScript, which a drop calling into JavaScript must not reenter. Once the runtime
// is gone it is leaked, as nothing of JavaScript is left to run it with.
inline void collected(const ::std::shared_ptr<world> &w, ::std::function<void()> drop) {
	if (w->alive.load(::std::memory_order_acquire)) {
		w->post([drop = ::std::move(drop)](jsi::Runtime &rt) { unlocked(rt, drop); });
	}
}

// Errors.

[[noreturn]] inline void raise(jsi::Runtime &rt, jsi::Function &ctor, const ::std::string &what) {
	throw jsi::JSError(rt, ctor.callAsConstructor(rt, jsi::String::createFromUtf8(rt, what)));
}

[[noreturn]] inline void range(jsi::Runtime &rt, const ::std::string &what) {
	raise(rt, W(rt).range, what);
}

[[noreturn]] inline void type(jsi::Runtime &rt, const ::std::string &what) {
	raise(rt, W(rt).type, what);
}

[[noreturn]] inline void error(jsi::Runtime &rt, const ::std::string &what) {
	raise(rt, W(rt).error, what);
}

// Throw unless the call `what` wrote no status: a run-time check failed.
inline void status(jsi::Runtime &rt, ::std::int32_t s, const char *what) {
	static const char *const statuses[] = {
	    "a handle's value was moved out",
	    "a handle is borrowed exclusively",
	    "a handle is already borrowed",
	    "a handle is borrowed on another thread",
	    "a handle a call lent is borrowed exclusively or moved out",
	    "a handle lent from another outlived it: the other was borrowed exclusively, moved out, or freed",
	    "a value that is not Send is used on another thread than the one that made it",
	};
	if (s == 0) {
		return;
	}
	const char *text = s >= 1 && s <= 7 ? statuses[s - 1] : "an unknown status";
	error(rt, ::std::string(what) + ": " + text);
}

// Throw that the library's target lacks `what`, unless it is one of `on`'s.
inline void lacks(jsi::Runtime &rt, ::std::uint64_t on, const char *what) {
	if ((on >> c_target() & 1) == 0) {
		error(rt, ::std::string(what) + ": the library's target lacks it");
	}
}

// The `k`th argument, or `undefined` past those passed.
inline const jsi::Value &arg(const jsi::Value *a, ::std::size_t n, ::std::size_t k) {
	static const jsi::Value undefined;
	return k < n ? a[k] : undefined;
}

// What `v` is, for an error: its `String`.
inline ::std::string shown(jsi::Runtime &rt, const jsi::Value &v) {
	if (v.isString()) {
		return v.getString(rt).utf8(rt);
	}
	if (v.isObject()) {
		return "an object";
	}
	return v.toString(rt).utf8(rt);
}

// `v` as an object, or a TypeError naming `what` it should be.
inline jsi::Object obj(jsi::Runtime &rt, const jsi::Value &v, const char *what) {
	if (!v.isObject()) {
		type(rt, shown(rt, v) + " is no " + what);
	}
	return v.getObject(rt);
}

inline jsi::Value get(jsi::Runtime &rt, const jsi::Object &o, const char *name) {
	return o.getProperty(rt, name);
}

// Numbers. A value the fast path refuses goes to `rt.ts`'s own check, so each error reads
// as the web binding's does.

inline bool integral(double d) {
	return ::std::isfinite(d) && ::std::trunc(d) == d;
}

template <class T>
T int_(jsi::Runtime &rt, const jsi::Value &v, double min, double max, const char *what) {
	if (v.isNumber()) {
		double d = v.getNumber();
		if (integral(d) && d >= min && d <= max) {
			return static_cast<T>(d);
		}
	}
	W(rt).int_.call(rt, jsi::Value(rt, v), min, max, what);
	fail("rt.int passed what the JSI module refused");
}

template <class T>
T nonzero(jsi::Runtime &rt, T v) {
	if (v == 0) {
		range(rt, "a NonZero is zero");
	}
	return v;
}

inline ::std::uint32_t char_(jsi::Runtime &rt, const jsi::Value &v) {
	if (v.isNumber()) {
		double d = v.getNumber();
		if (integral(d) && d >= 0 && d <= 0x10ffff && (d < 0xd800 || d > 0xdfff)) {
			return static_cast<::std::uint32_t>(d);
		}
	}
	W(rt).char_.call(rt, jsi::Value(rt, v));
	fail("rt.char passed what the JSI module refused");
}

constexpr double safe = 9007199254740991.0;

// What a helper of `rt.ts` returned as a 32-bit word, from `lo` up: checked, since whatever
// JavaScript calls `install` gives the kit, and JSI checks no value's type but in a debug build.
inline double word(jsi::Runtime &rt, const jsi::Value &v, double lo = 0) {
	if (!v.isNumber() || !integral(v.getNumber()) || v.getNumber() < lo || v.getNumber() > lo + 4294967295.0) {
		type(rt, "the binding's runtime returned " + shown(rt, v) + " for a 32-bit word");
	}
	return v.getNumber();
}

// The `n` words `helper` checks and returns of `v`, the first from `first` up.
template <::std::size_t n>
::std::array<double, n> words(jsi::Runtime &rt, jsi::Function &helper, const jsi::Value &v, double first = 0) {
	jsi::Value got = helper.call(rt, jsi::Value(rt, v));
	if (!got.isObject() || !got.getObject(rt).isArray(rt) || got.getObject(rt).getArray(rt).size(rt) != n) {
		type(rt, "the binding's runtime returned " + shown(rt, got) + " for " + ::std::to_string(n) + " words");
	}
	jsi::Array a = got.getObject(rt).getArray(rt);
	::std::array<double, n> out;
	for (::std::size_t i = 0; i < n; i++) {
		out[i] = word(rt, a.getValueAtIndex(rt, i), i == 0 ? first : 0);
	}
	return out;
}

inline ::std::uint64_t u64(jsi::Runtime &rt, const jsi::Value &v) {
	if (v.isNumber()) {
		double d = v.getNumber();
		if (integral(d) && d >= 0 && d <= safe) {
			return static_cast<::std::uint64_t>(d);
		}
	}
	auto [hi, lo] = words<2>(rt, W(rt).u64_halves, v);
	return static_cast<::std::uint64_t>(hi) << 32 | static_cast<::std::uint64_t>(lo);
}

inline ::std::int64_t i64(jsi::Runtime &rt, const jsi::Value &v) {
	if (v.isNumber()) {
		double d = v.getNumber();
		if (integral(d) && d >= -safe && d <= safe) {
			return static_cast<::std::int64_t>(d);
		}
	}
	auto [hi, lo] = words<2>(rt, W(rt).i64_halves, v, -2147483648.0);
	auto bits = static_cast<::std::uint64_t>(static_cast<::std::int64_t>(hi)) << 32 | static_cast<::std::uint64_t>(lo);
	return static_cast<::std::int64_t>(bits);
}

inline jsi::Value limbs(jsi::Runtime &rt, ::std::initializer_list<::std::pair<const char *, ::std::uint32_t>> words) {
	jsi::Object o(rt);
	for (auto &[name, word] : words) {
		o.setProperty(rt, name, static_cast<double>(word));
	}
	return o;
}

inline jsi::Value of_u64(jsi::Runtime &rt, ::std::uint64_t v) {
	if (v <= static_cast<::std::uint64_t>(safe)) {
		return static_cast<double>(v);
	}
	return limbs(rt, {{"hi", static_cast<::std::uint32_t>(v >> 32)}, {"lo", static_cast<::std::uint32_t>(v)}});
}

inline jsi::Value of_i64(jsi::Runtime &rt, ::std::int64_t v) {
	if (v >= -static_cast<::std::int64_t>(safe) && v <= static_cast<::std::int64_t>(safe)) {
		return static_cast<double>(v);
	}
	jsi::Object o(rt);
	o.setProperty(rt, "hi", static_cast<double>(static_cast<::std::int32_t>(v >> 32)));
	o.setProperty(rt, "lo", static_cast<double>(static_cast<::std::uint32_t>(v)));
	return o;
}

inline c_u128 u128(jsi::Runtime &rt, const jsi::Value &v) {
	if (v.isNumber()) {
		double d = v.getNumber();
		if (integral(d) && d >= 0 && d <= safe) {
			return c_u128{static_cast<::std::uint64_t>(d), 0};
		}
	}
	auto a = words<4>(rt, W(rt).u128_words, v);
	::std::uint64_t w[4];
	for (::std::size_t i = 0; i < 4; i++) {
		w[i] = static_cast<::std::uint64_t>(a[i]);
	}
	return c_u128{w[1] << 32 | w[0], w[3] << 32 | w[2]};
}

inline jsi::Value of_u128(jsi::Runtime &rt, c_u128 v) {
	if (v.hi == 0 && v.lo <= static_cast<::std::uint64_t>(safe)) {
		return static_cast<double>(v.lo);
	}
	return limbs(rt, {{"hi", static_cast<::std::uint32_t>(v.hi >> 32)},
	                  {"w2", static_cast<::std::uint32_t>(v.hi)},
	                  {"w1", static_cast<::std::uint32_t>(v.lo >> 32)},
	                  {"lo", static_cast<::std::uint32_t>(v.lo)}});
}

constexpr ::std::uint32_t f32_nan = 0x7fc00000;
constexpr ::std::uint64_t f64_nan = 0x7ff8000000000000;

inline float f32(jsi::Runtime &rt, const jsi::Value &v) {
	if (v.isNumber()) {
		double d = v.getNumber();
		if (d == d && static_cast<double>(static_cast<float>(d)) == d) {
			return static_cast<float>(d);
		}
	}
	auto bits = static_cast<::std::uint32_t>(word(rt, W(rt).f32_bits.call(rt, jsi::Value(rt, v))));
	float f;
	::std::memcpy(&f, &bits, 4);
	return f;
}

inline jsi::Value of_f32(jsi::Runtime &rt, float f) {
	::std::uint32_t bits;
	::std::memcpy(&bits, &f, 4);
	if ((bits & 0x7fffffff) <= 0x7f800000) {
		return static_cast<double>(f);
	}
	if (bits == f32_nan) {
		return ::std::numeric_limits<double>::quiet_NaN();
	}
	return limbs(rt, {{"bits", bits}});
}

inline double f64(jsi::Runtime &rt, const jsi::Value &v) {
	if (v.isNumber() && v.getNumber() == v.getNumber()) {
		return v.getNumber();
	}
	auto [hi, lo] = words<2>(rt, W(rt).f64_bits, v);
	::std::uint64_t bits = static_cast<::std::uint64_t>(hi) << 32 | static_cast<::std::uint64_t>(lo);
	double d;
	::std::memcpy(&d, &bits, 8);
	return d;
}

inline jsi::Value of_f64(jsi::Runtime &rt, double d) {
	::std::uint64_t bits;
	::std::memcpy(&bits, &d, 8);
	if ((bits & 0x7fffffffffffffff) <= 0x7ff0000000000000) {
		return d;
	}
	if (bits == f64_nan) {
		return ::std::numeric_limits<double>::quiet_NaN();
	}
	return limbs(rt, {{"hi", static_cast<::std::uint32_t>(bits >> 32)}, {"lo", static_cast<::std::uint32_t>(bits)}});
}

inline bool bool_(jsi::Runtime &rt, const jsi::Value &v) {
	if (!v.isBool()) {
		type(rt, shown(rt, v) + " is no boolean");
	}
	return v.getBool();
}

// A `usize`, which past a 32-bit target's range throws.
inline ::std::size_t usize(jsi::Runtime &rt, const jsi::Value &v) {
	::std::uint64_t u = u64(rt, v);
	if (u > SIZE_MAX) {
		range(rt, ::std::to_string(u) + " is no usize");
	}
	return static_cast<::std::size_t>(u);
}

inline ::std::ptrdiff_t isize(jsi::Runtime &rt, const jsi::Value &v) {
	::std::int64_t i = i64(rt, v);
	if (i > PTRDIFF_MAX || i < PTRDIFF_MIN) {
		range(rt, ::std::to_string(i) + " is no isize");
	}
	return static_cast<::std::ptrdiff_t>(i);
}

// Rust's `Result`, as `rt.Ok` and `rt.Err` make it.
inline jsi::Value ok(jsi::Runtime &rt, jsi::Value v) {
	jsi::Object o(rt);
	o.setProperty(rt, "ok", true);
	o.setProperty(rt, "value", v);
	return o;
}

inline jsi::Value err(jsi::Runtime &rt, jsi::Value e) {
	jsi::Object o(rt);
	o.setProperty(rt, "ok", false);
	o.setProperty(rt, "error", e);
	return o;
}

// An array of `items`, as a tuple crosses.
template <class... V>
jsi::Value array(jsi::Runtime &rt, V &&...items) {
	jsi::Array out(rt, sizeof...(items));
	::std::size_t i = 0;
	(out.setValueAtIndex(rt, i++, ::std::forward<V>(items)), ...);
	return out;
}

// `v` as a function, or a TypeError.
inline jsi::Function function(jsi::Runtime &rt, const jsi::Value &v) {
	if (!v.isObject() || !v.getObject(rt).isFunction(rt)) {
		type(rt, shown(rt, v) + " is no function");
	}
	return v.getObject(rt).getFunction(rt);
}

// `v` as an array, or a TypeError.
inline jsi::Array arr(jsi::Runtime &rt, const jsi::Value &v) {
	if (!v.isObject() || !v.getObject(rt).isArray(rt)) {
		type(rt, shown(rt, v) + " is no array");
	}
	return v.getObject(rt).getArray(rt);
}

// A tagged object's tag, or a TypeError naming what it should be.
inline ::std::string tag_of(jsi::Runtime &rt, const jsi::Value &v, const char *what) {
	jsi::Value t = obj(rt, v, what).getProperty(rt, "tag");
	return t.isString() ? t.getString(rt).utf8(rt) : ::std::string();
}

// A string's text, or a TypeError naming what it should be.
inline ::std::string text(jsi::Runtime &rt, const jsi::Value &v) {
	return v.isString() ? v.getString(rt).utf8(rt) : ::std::string();
}

inline jsi::Value ascii(jsi::Runtime &rt, const char *s) {
	return jsi::String::createFromAscii(rt, s);
}

// Handles.

// What an object type's handles do: its name, and its free and, if Rust checks it, drop.
struct kind {
	const char *name;
	void (*free)(void *);
	void (*drop)(void *, ::std::int32_t *);
	// The C function `drop` calls, which a failed status names.
	const char *dropping;
};

// A handle to a Rust object, held by the JavaScript object it is the state of: it counts
// the calls holding it, and is freed once disposed and no call holds it, or once the
// garbage collector finds it unreachable if never disposed.
struct cell : jsi::NativeState {
	::std::shared_ptr<world> home;
	void *p;
	const kind *k;
	::std::uint32_t h = 1;
	bool closed = false;
	bool owned;
	bool freed = false;

	cell(::std::shared_ptr<world> home, void *p, const kind *k, bool owned)
	    : home(::std::move(home)), p(p), k(k), owned(owned) {}

	~cell() override {
		if (owned && !freed) {
			collected(home, [p = p, k = k] { k->free(p); });
		}
	}

	void release(jsi::Runtime &rt) {
		if (--h == 0 && owned && !freed) {
			freed = true;
			unlocked(rt, [&] { k->free(p); });
		}
	}
};

// A new handle of `k` to `p`: owned, or else lent for a call into JavaScript.
inline jsi::Value made(jsi::Runtime &rt, void *p, const kind &k, bool owned) {
	jsi::Object o = jsi::Object::create(rt, W(rt).handle);
	o.setNativeState(rt, ::std::make_shared<cell>(of(rt), p, &k, owned));
	return o;
}

// The state `T` of `v`, or a TypeError naming what it should be: JSI's `getNativeState`
// checks no type but in a debug build.
template <class T>
::std::shared_ptr<T> state_of(jsi::Runtime &rt, const jsi::Value &v, const char *what) {
	if (v.isObject()) {
		jsi::Object o = v.getObject(rt);
		if (o.hasNativeState<T>(rt)) {
			return o.getNativeState<T>(rt);
		}
	}
	type(rt, shown(rt, v) + " is no " + what);
}

inline ::std::shared_ptr<cell> cell_of(jsi::Runtime &rt, const jsi::Value &v, const kind &k) {
	auto c = state_of<cell>(rt, v, k.name);
	if (c->k != &k) {
		type(rt, shown(rt, v) + " is no " + k.name);
	}
	return c;
}

// Bytes a call returned, the state of an `rt.Bytes`: Rust's buffer, lent in place.
struct bytes_cell : jsi::NativeState {
	::std::shared_ptr<world> home;
	c_buf buf;
	::std::uint32_t h = 1;
	bool closed = false;

	bytes_cell(::std::shared_ptr<world> home, c_buf buf) : home(::std::move(home)), buf(buf) {}

	~bytes_cell() override {
		if (buf.owner != nullptr) {
			collected(home, [buf = buf] { c_buf_free(buf); });
		}
	}

	void release() {
		if (--h == 0 && buf.owner != nullptr) {
			c_buf_free(buf);
			buf.owner = nullptr;
		}
	}
};

// A copy of bytes, as an `ArrayBuffer` owns.
struct copy final : jsi::MutableBuffer {
	::std::vector<::std::uint8_t> v;
	// Of `len` bytes at `ptr`, or zeros where it is null.
	explicit copy(const ::std::uint8_t *ptr, ::std::size_t len) : v(ptr == nullptr ? ::std::vector<::std::uint8_t>(len) : ::std::vector<::std::uint8_t>(ptr, ptr + len)) {}
	::std::size_t size() const override {
		return v.size();
	}
	::std::uint8_t *data() override {
		return v.data();
	}
};

inline jsi::ArrayBuffer copied(jsi::Runtime &rt, const ::std::uint8_t *ptr, ::std::size_t len) {
	return jsi::ArrayBuffer(rt, ::std::make_shared<copy>(ptr, len));
}

// A `Uint8Array` of a copy of `buf`'s bytes, which it frees where `own`.
inline jsi::Value u8s(jsi::Runtime &rt, c_buf buf, bool own) {
	jsi::ArrayBuffer ab = copied(rt, buf.ptr, buf.len);
	if (own) {
		c_buf_free(buf);
	}
	return W(rt).uint8.callAsConstructor(rt, ab);
}

// The typed array `ctor` names of a copy of `len` elements of `size` bytes at `ptr`.
inline jsi::Value typed(jsi::Runtime &rt, const char *ctor, const void *ptr, ::std::size_t len, ::std::size_t size) {
	jsi::ArrayBuffer ab = copied(rt, static_cast<const ::std::uint8_t *>(ptr), len * size);
	return W(rt).arrays.getPropertyAsFunction(rt, ctor).callAsConstructor(rt, ab);
}

// The bytes Rust returned: lent past 64 bytes, else copied and freed, as an `rt.Bytes`.
// Lent, they are no `ArrayBuffer`, which JavaScript could keep past their free: the module
// reads them, through the state of the `rt.Bytes`.
inline jsi::Value returned(jsi::Runtime &rt, c_buf buf) {
	if (buf.len <= 64 || buf.owner == nullptr) {
		jsi::ArrayBuffer ab = copied(rt, buf.ptr, buf.len);
		if (buf.owner != nullptr) {
			c_buf_free(buf);
		}
		return W(rt).bytes.call(rt, ab, false, static_cast<double>(buf.len));
	}
	auto state = ::std::make_shared<bytes_cell>(of(rt), buf);
	jsi::Object b = obj(rt, W(rt).bytes.call(rt, jsi::Value::undefined(), true, static_cast<double>(buf.len)), "Bytes");
	b.setNativeState(rt, state);
	return b;
}

// A string of Rust's UTF-8, which it frees where `own`.
inline jsi::Value str(jsi::Runtime &rt, c_buf buf, bool own) {
	jsi::String s = jsi::String::createFromUtf8(rt, buf.ptr, buf.len);
	if (own) {
		c_buf_free(buf);
	}
	return s;
}

struct remote;

// What a call holds until it returns: copies of text, arrays it lowered, and the handles
// and bytes it holds.
struct frame {
	jsi::Runtime &rt;
	::std::deque<::std::string> text;
	::std::vector<::std::shared_ptr<void>> mem;
	::std::vector<::std::shared_ptr<cell>> held;
	::std::vector<::std::shared_ptr<bytes_cell>> bytes;
	::std::vector<::std::shared_ptr<cell>> lent;
	// How to take back each value the call gives Rust, run if a later argument throws
	// before Rust took them.
	::std::vector<::std::function<void()>> staged;

	explicit frame(jsi::Runtime &rt) : rt(rt) {}
	frame(const frame &) = delete;
	frame &operator=(const frame &) = delete;
	~frame() {
		for (auto u = staged.rbegin(); u != staged.rend(); ++u) {
			(*u)();
		}
		for (auto &c : held) {
			c->release(rt);
		}
		for (auto &b : bytes) {
			b->release();
		}
		// A handle lent for a call into JavaScript closes once it returns.
		for (auto &c : lent) {
			c->closed = true;
		}
	}

	// Storage for `n` values of `T`, held until the call returns.
	template <class T>
	T *alloc(::std::size_t n) {
		auto p = ::std::shared_ptr<T[]>(new T[n == 0 ? 1 : n]());
		mem.push_back(p);
		return p.get();
	}

	// The pointer of the handle `v` of `k`, held until the call returns.
	void *hold(const jsi::Value &v, const kind &k) {
		auto c = cell_of(rt, v, k);
		if (c->closed) {
			error(rt, ::std::string(k.name) + " is closed");
		}
		c->h++;
		held.push_back(c);
		return c->p;
	}

	void *hold_or(const jsi::Value &v, const kind &k) {
		return v.isUndefined() ? nullptr : hold(v, k);
	}

	// A handle Rust lends for a call into JavaScript, closed after it.
	jsi::Value lend(void *p, const kind &k) {
		jsi::Value v = made(rt, p, k, false);
		lent.push_back(v.getObject(rt).getNativeState<cell>(rt));
		return v;
	}

	// A view of `text`'s UTF-8, held until the call returns: a lone surrogate throws.
	c_buf utf8(const jsi::Value &v) {
		if (!v.isString()) {
			type(rt, shown(rt, v) + " is no string");
		}
		::std::u16string u = v.getString(rt).utf16(rt);
		::std::string &s = text.emplace_back();
		s.reserve(u.size() * 3);
		for (::std::size_t i = 0; i < u.size(); i++) {
			::std::uint32_t c = u[i];
			if (c >= 0xd800 && c <= 0xdbff && i + 1 < u.size() && u[i + 1] >= 0xdc00 && u[i + 1] <= 0xdfff) {
				c = 0x10000 + ((c - 0xd800) << 10) + (u[++i] - 0xdc00);
			} else if (c >= 0xd800 && c <= 0xdfff) {
				range(rt, "a string holds a lone surrogate, which no Rust string does");
			}
			if (c < 0x80) {
				s.push_back(static_cast<char>(c));
			} else if (c < 0x800) {
				s.push_back(static_cast<char>(0xc0 | c >> 6));
				s.push_back(static_cast<char>(0x80 | (c & 0x3f)));
			} else if (c < 0x10000) {
				s.push_back(static_cast<char>(0xe0 | c >> 12));
				s.push_back(static_cast<char>(0x80 | (c >> 6 & 0x3f)));
				s.push_back(static_cast<char>(0x80 | (c & 0x3f)));
			} else {
				s.push_back(static_cast<char>(0xf0 | c >> 18));
				s.push_back(static_cast<char>(0x80 | (c >> 12 & 0x3f)));
				s.push_back(static_cast<char>(0x80 | (c >> 6 & 0x3f)));
				s.push_back(static_cast<char>(0x80 | (c & 0x3f)));
			}
		}
		return c_buf{reinterpret_cast<const ::std::uint8_t *>(s.data()), s.size(), nullptr};
	}

	// Rust's own copy of `text`, as a closure returns it.
	c_buf utf8_owned(const jsi::Value &v) {
		return c_buf_copy(utf8(v));
	}

	// A view of the bytes of a `Uint8Array`, or of an `rt.Bytes` a call returned, held until
	// the call returns.
	c_buf view(const jsi::Value &v) {
		if (v.isObject()) {
			jsi::Object o = v.getObject(rt);
			if (o.hasNativeState<bytes_cell>(rt)) {
				auto b = o.getNativeState<bytes_cell>(rt);
				if (b->closed) {
					error(rt, "Bytes is closed");
				}
				b->h++;
				bytes.push_back(b);
				return c_buf{b->buf.ptr, b->buf.len, nullptr};
			}
			jsi::Value copy = W(rt).bytes_copy.call(rt, jsi::Value(rt, v));
			if (!copy.isUndefined()) {
				// A copy `rt.Bytes` holds.
				return array(copy, "Uint8Array");
			}
			if (o.instanceOf(rt, W(rt).uint8)) {
				return array(v, "Uint8Array");
			}
		}
		type(rt, shown(rt, v) + " is no Uint8Array");
	}

	// What Rust takes by value of the bytes `view` made: of an `rt.Bytes` of Rust's buffer,
	// a second owner of it, even once disposed meanwhile, which only frees the first; of
	// others, the view, which Rust copies.
	c_buf give(c_buf viewed) {
		for (auto &b : bytes) {
			if (viewed.ptr != nullptr && b->buf.ptr == viewed.ptr && b->buf.len == viewed.len) {
				c_buf owner = c_buf_clone(b->buf);
				staged.emplace_back([owner] { c_buf_free(owner); });
				return owner;
			}
		}
		return viewed;
	}

	// A JavaScript value Rust keeps once the call is made.
	remote *keep(const jsi::Value &v);
	// The waker of the function `v`, which Rust drops once the call is made.
	c_waker waker(const jsi::Value &v);
	// The future of the promise `v`, which Rust frees once the call is made.
	c_foreign_future future(const jsi::Value &v);

	// Rust took what the call gave it: the call is made.
	void passed() {
		staged.clear();
	}

	// A copy of the bytes a typed array `v`, of the constructor `ctor`, views, held until the
	// call returns. Its kind, buffer, offset, and length are the engine's own, which no property
	// or prototype a caller sets can change: a `Uint8Array`'s through JSI, and another
	// kind's from `%TypedArray%.prototype`'s own `Symbol.toStringTag` getter.
	c_buf array(const jsi::Value &v, const char *ctor) {
		// The engine's own check that it is a typed array, as the kit `install` took may be
		// another's than the binding's: another kind's name alone comes from the kit, which
		// read wrong reads the array's own bytes as other items.
		bool is = v.isObject() && v.getObject(rt).isTypedArray(rt);
		if (is && ::std::strcmp(ctor, "Uint8Array") == 0) {
			is = v.getObject(rt).isUint8Array(rt);
		} else if (is) {
			jsi::Value tag = W(rt).typed_tag.callWithThis(rt, v.getObject(rt));
			is = tag.isString() && tag.getString(rt).utf8(rt) == ctor;
		}
		if (!is) {
			type(rt, shown(rt, v) + " is no " + ctor);
		}
		jsi::TypedArray ta = v.getObject(rt).getTypedArray(rt);
		jsi::ArrayBuffer ab = ta.buffer(rt);
		::std::size_t offset = ta.byteOffset(rt), len = ta.byteLength(rt), size = ab.size(rt);
		if (offset > size || len > size - offset) {
			range(rt, ::std::string(ctor) + " is outside its buffer");
		}
		// An empty array's buffer may have no data, and a null pointer is C's `None`: an
		// empty one points at bytes aligned for any item, as Rust's empty slice does.
		alignas(16) static const ::std::uint8_t empty[16] = {};
		if (len == 0) {
			return c_buf{empty, 0, nullptr};
		}
		// A copy, as the web's is: Rust holds the array as a slice no one writes while it
		// does, but JavaScript a closure the call runs, or another thread the call frees the
		// runtime for, may write the array meanwhile.
		struct alignas(16) block {
			::std::uint8_t b[16];
		};
		auto p = ::std::shared_ptr<block[]>(new block[(len + 15) / 16]);
		::std::memcpy(p.get(), ab.data(rt) + offset, len);
		mem.push_back(p);
		return c_buf{reinterpret_cast<const ::std::uint8_t *>(p.get()), len, nullptr};
	}

	// The `[u8; N]` of a `Uint8Array` of exactly `n` bytes, written to `out`.
	void fixed(const jsi::Value &v, ::std::uint8_t *out, ::std::size_t n) {
		c_buf b = view(v);
		if (b.len != n) {
			range(rt, "a Uint8Array of " + ::std::to_string(b.len) + " bytes is no [u8; " + ::std::to_string(n) + "]");
		}
		::std::memcpy(out, b.ptr, n);
	}
};

// What a call returns as text or bytes, as Rust's own copy: what JavaScript returns Rust.
inline c_buf owned_bytes(frame &f, const jsi::Value &v) {
	return c_buf_copy(f.view(v));
}

// Opaque values.

inline jsi::Value opaque(jsi::Runtime &rt, const void *p, ::std::size_t n) {
	return W(rt).opaque_made.call(rt, W(rt).uint8.callAsConstructor(rt, copied(rt, static_cast<const ::std::uint8_t *>(p), n)));
}

inline void opaque_put(frame &f, const jsi::Value &v, void *p, ::std::size_t n) {
	jsi::Value bytes = W(f.rt).opaque_bytes.call(f.rt, jsi::Value(f.rt, v));
	c_buf b = f.array(bytes, "Uint8Array");
	if (b.len != n) {
		range(f.rt, "a value Rust made is no other's");
	}
	::std::memcpy(p, b.ptr, n);
}

// Std's values.

inline jsi::Value octets(jsi::Runtime &rt, const ::std::uint8_t *p, ::std::size_t n) {
	jsi::Object o(rt);
	o.setProperty(rt, "octets", W(rt).uint8.callAsConstructor(rt, copied(rt, p, n)));
	return o;
}

inline jsi::Value tagged(jsi::Runtime &rt, const char *tag, jsi::Value v) {
	jsi::Object o(rt);
	o.setProperty(rt, "tag", jsi::String::createFromAscii(rt, tag));
	o.setProperty(rt, "_0", v);
	return o;
}

inline ::std::string tag(jsi::Runtime &rt, const jsi::Object &o) {
	jsi::Value t = o.getProperty(rt, "tag");
	return t.isString() ? t.getString(rt).utf8(rt) : ::std::string();
}

inline jsi::Value of_ipv4(jsi::Runtime &rt, const c_ipv4_addr &v) {
	return octets(rt, v.octets, 4);
}

inline jsi::Value of_ipv6(jsi::Runtime &rt, const c_ipv6_addr &v) {
	return octets(rt, v.octets, 16);
}

inline jsi::Value of_ip(jsi::Runtime &rt, const c_ip_addr &v) {
	return v.v6 ? tagged(rt, "V6", of_ipv6(rt, v.ip.v6)) : tagged(rt, "V4", of_ipv4(rt, v.ip.v4));
}

inline jsi::Value of_socket_v4(jsi::Runtime &rt, const c_socket_addr_v4 &v) {
	jsi::Object o(rt);
	o.setProperty(rt, "ip", of_ipv4(rt, v.ip));
	o.setProperty(rt, "port", static_cast<double>(v.port));
	return o;
}

inline jsi::Value of_socket_v6(jsi::Runtime &rt, const c_socket_addr_v6 &v) {
	jsi::Object o(rt);
	o.setProperty(rt, "ip", of_ipv6(rt, v.ip));
	o.setProperty(rt, "port", static_cast<double>(v.port));
	o.setProperty(rt, "flowinfo", static_cast<double>(v.flowinfo));
	o.setProperty(rt, "scopeId", static_cast<double>(v.scope_id));
	return o;
}

inline jsi::Value of_socket(jsi::Runtime &rt, const c_socket_addr &v) {
	return v.v6 ? tagged(rt, "V6", of_socket_v6(rt, v.addr.v6)) : tagged(rt, "V4", of_socket_v4(rt, v.addr.v4));
}

inline void ipv4(frame &f, const jsi::Value &v, c_ipv4_addr &out) {
	f.fixed(get(f.rt, obj(f.rt, v, "Ipv4Addr"), "octets"), out.octets, 4);
}

inline void ipv6(frame &f, const jsi::Value &v, c_ipv6_addr &out) {
	f.fixed(get(f.rt, obj(f.rt, v, "Ipv6Addr"), "octets"), out.octets, 16);
}

inline void ip(frame &f, const jsi::Value &v, c_ip_addr &out) {
	jsi::Object o = obj(f.rt, v, "IpAddr");
	::std::string t = tag(f.rt, o);
	out = c_ip_addr{};
	if (t == "V4") {
		ipv4(f, get(f.rt, o, "_0"), out.ip.v4);
	} else if (t == "V6") {
		out.v6 = true;
		ipv6(f, get(f.rt, o, "_0"), out.ip.v6);
	} else {
		range(f.rt, "an IpAddr is V4 or V6");
	}
}

inline void socket_v4(frame &f, const jsi::Value &v, c_socket_addr_v4 &out) {
	jsi::Object o = obj(f.rt, v, "SocketAddrV4");
	ipv4(f, get(f.rt, o, "ip"), out.ip);
	out.port = int_<::std::uint16_t>(f.rt, get(f.rt, o, "port"), 0, 0xffff, "u16");
}

inline void socket_v6(frame &f, const jsi::Value &v, c_socket_addr_v6 &out) {
	jsi::Object o = obj(f.rt, v, "SocketAddrV6");
	ipv6(f, get(f.rt, o, "ip"), out.ip);
	out.port = int_<::std::uint16_t>(f.rt, get(f.rt, o, "port"), 0, 0xffff, "u16");
	out.flowinfo = int_<::std::uint32_t>(f.rt, get(f.rt, o, "flowinfo"), 0, 0xffffffff, "u32");
	out.scope_id = int_<::std::uint32_t>(f.rt, get(f.rt, o, "scopeId"), 0, 0xffffffff, "u32");
}

inline void socket(frame &f, const jsi::Value &v, c_socket_addr &out) {
	jsi::Object o = obj(f.rt, v, "SocketAddr");
	::std::string t = tag(f.rt, o);
	out = c_socket_addr{};
	if (t == "V4") {
		socket_v4(f, get(f.rt, o, "_0"), out.addr.v4);
	} else if (t == "V6") {
		out.v6 = true;
		socket_v6(f, get(f.rt, o, "_0"), out.addr.v6);
	} else {
		range(f.rt, "a SocketAddr is V4 or V6");
	}
}

inline jsi::Value of_duration(jsi::Runtime &rt, const c_duration &v) {
	jsi::Object o(rt);
	o.setProperty(rt, "secs", of_u64(rt, v.secs));
	o.setProperty(rt, "nanos", static_cast<double>(v.nanos));
	return o;
}

inline void duration(frame &f, const jsi::Value &v, c_duration &out) {
	jsi::Object o = obj(f.rt, v, "Duration");
	out.secs = u64(f.rt, get(f.rt, o, "secs"));
	out.nanos = int_<::std::uint32_t>(f.rt, get(f.rt, o, "nanos"), 0, 999999999, "Duration's nanos");
}

inline jsi::Value of_path(jsi::Runtime &rt, c_buf buf, bool own) {
	jsi::Object o(rt);
	o.setProperty(rt, "bytes", u8s(rt, buf, own));
	return o;
}

inline c_buf path(frame &f, const jsi::Value &v) {
	return f.view(get(f.rt, obj(f.rt, v, "PathBuf"), "bytes"));
}

inline bool none(jsi::Runtime &rt, const jsi::Value &v) {
	return jsi::Value::strictEquals(rt, v, W(rt).none);
}

// What JavaScript implements for Rust: values Rust holds, freed on the JavaScript thread.

// A JavaScript value Rust holds, freed on the JavaScript thread: in a runtime since gone,
// leaked.
struct remote {
	::std::shared_ptr<world> w;
	jsi::Value v;
	remote(jsi::Runtime &rt, const jsi::Value &v) : w(of(rt)), v(rt, v) {}
};

// Run `job` on the JavaScript thread later, unless its runtime is gone.
inline void post(const ::std::shared_ptr<world> &w, ::std::function<void(jsi::Runtime &)> job) {
	if (w->alive.load(::std::memory_order_acquire)) {
		w->post(::std::move(job));
	}
}

// Free `r` on its thread, later, as a drop may run inside the garbage collector's sweep;
// in a runtime since gone, leaked.
inline void release(remote *r) {
	auto w = r->w;
	post(w, [r](jsi::Runtime &) { delete r; });
}

// Run `job` for Rust: on the JavaScript thread, which takes the runtime from the call into
// Rust it waits in; or from another thread, posted to the JavaScript thread, or run on this
// one once a call into Rust frees the runtime, whichever comes first.
template <class T>
T on_js(const ::std::shared_ptr<world> &w, const char *what, ::std::function<T(jsi::Runtime &)> job) {
	auto take = [&]() -> T {
		given back{*w};
		return job(*w->rt);
	};
	if (!w->alive.load(::std::memory_order_acquire)) {
		fail(::std::string(what) + ": called after its JavaScript runtime was gone");
	}
	if (running == w.get()) {
		// This thread runs this runtime's JavaScript already.
		return job(*w->rt);
	}
	if (w->here()) {
		{
			::std::unique_lock<::std::mutex> lock(w->m);
			w->cv.wait(lock, [&] { return w->free; });
			taken(*w);
		}
		return take();
	}
	struct state {
		::std::atomic<bool> taken{false};
		bool done = false;
		::std::optional<T> out;
	};
	auto s = ::std::make_shared<state>();
	auto *f = &job;
	w->post([s, f, w](jsi::Runtime &rt) {
		// Run here only if the other thread still waits for it, which then keeps `f` alive.
		if (s->taken.exchange(true)) {
			return;
		}
		T v = (*f)(rt);
		{
			::std::lock_guard<::std::mutex> lock(w->m);
			s->out.emplace(::std::move(v));
			s->done = true;
		}
		w->cv.notify_all();
	});
	::std::unique_lock<::std::mutex> lock(w->m);
	w->cv.wait(lock, [&] { return s->done || w->free || !w->alive.load(::std::memory_order_relaxed); });
	if (!s->done && !w->alive.load(::std::memory_order_relaxed) && !s->taken.exchange(true)) {
		fail(::std::string(what) + ": its JavaScript runtime was gone before the call ran");
	}
	if (!s->done && !s->taken.exchange(true)) {
		taken(*w);
		lock.unlock();
		return take();
	}
	w->cv.wait(lock, [&] { return s->done; });
	return ::std::move(*s->out);
}

// What a call into JavaScript `what` does for Rust: what it throws aborts, as Rust has
// nowhere to unwind to.
template <class T>
T out(const ::std::shared_ptr<world> &w, const char *what, ::std::function<T(jsi::Runtime &)> job) {
	return on_js<T>(w, what, [&](jsi::Runtime &rt) -> T {
		try {
			return job(rt);
		} catch (const jsi::JSError &e) {
			fail(::std::string(what) + ": a call into JavaScript threw " + e.getStack());
		} catch (const ::std::exception &e) {
			fail(::std::string(what) + ": a call into JavaScript threw " + e.what());
		}
	});
}

// A count of references, held by the module and, once given a waker, by Rust.
struct node {
	::std::atomic<::std::size_t> refs{1};
	virtual ~node() = default;
	virtual void wake() = 0;
	void ref() {
		refs.fetch_add(1, ::std::memory_order_relaxed);
	}
	void unref() {
		if (refs.fetch_sub(1, ::std::memory_order_acq_rel) == 1) {
			delete this;
		}
	}
};

inline void wake_node(void *data) {
	static_cast<node *>(data)->wake();
}

inline void drop_node(void *data) {
	static_cast<node *>(data)->unref();
}

// `n` as a waker that borrows its reference, which Rust reuses when given it again.
inline c_waker lend(node *n) {
	return c_waker{&wake_node, &drop_node, n};
}

// `n` as a waker holding a new reference, which Rust drops.
inline c_waker give(node *n) {
	n->ref();
	return lend(n);
}

// A new reference to `n` for a posted job, released once the job is destroyed, whether it
// ran or `post` discarded it with its runtime gone.
inline ::std::shared_ptr<node> held(node *n) {
	n->ref();
	return ::std::shared_ptr<node>(n, [](node *n) { n->unref(); });
}

// A function JavaScript gave as a waker: each wake calls it on a microtask of its thread.
struct js_waker final : node {
	remote *f;
	explicit js_waker(remote *f) : f(f) {}
	~js_waker() override {
		release(f);
	}
	void wake() override {
		remote *f = this->f;
		post(f->w, [self = held(this), f](jsi::Runtime &rt) {
			try {
				f->v.getObject(rt).asFunction(rt).call(rt);
			} catch (const jsi::JSError &e) {
				fail(::std::string("a waker: a call into JavaScript threw ") + e.getStack());
			}
		});
	}
};

inline c_waker waker_of(jsi::Runtime &rt, const jsi::Value &v) {
	if (!v.isObject() || !v.getObject(rt).isFunction(rt)) {
		type(rt, shown(rt, v) + " is no function");
	}
	return lend(new js_waker(new remote(rt, v)));
}

// A waker Rust returned, the state of an `rt.Waker`: disposed, and dropped in Rust once no
// wake of it runs, as JavaScript a wake calls may dispose it.
struct waker_cell : jsi::NativeState {
	::std::shared_ptr<world> home;
	c_waker w;
	bool closed = false;
	bool dropped = false;
	::std::uint32_t waking = 0;
	waker_cell(::std::shared_ptr<world> home, c_waker w) : home(::std::move(home)), w(w) {}
	~waker_cell() override {
		if (!dropped && w.drop != nullptr) {
			collected(home, [w = w] { w.drop(w.data); });
		}
	}
	void wake(jsi::Runtime &rt) {
		if (closed) {
			error(rt, "Waker is closed");
		}
		waking++;
		unlocked(rt, [&] { w.wake(w.data); });
		waking--;
		settle(rt);
	}
	void dispose(jsi::Runtime &rt) {
		closed = true;
		settle(rt);
	}
	// Drop Rust's waker once disposed and no wake runs.
	void settle(jsi::Runtime &rt) {
		if (closed && waking == 0 && !dropped) {
			dropped = true;
			if (w.drop != nullptr) {
				unlocked(rt, [&] { w.drop(w.data); });
			}
		}
	}
};

inline jsi::Value of_waker(jsi::Runtime &rt, c_waker w) {
	jsi::Object o = jsi::Object::create(rt, W(rt).waker);
	o.setNativeState(rt, ::std::make_shared<waker_cell>(of(rt), w));
	return o;
}

// A promise JavaScript gave Rust as a future: settled once, which wakes the waker of Rust's
// last poll, and polled from any thread.
struct foreign {
	::std::mutex m;
	bool done = false;
	bool subscribed = false;
	c_waker w{};
	// The promise, and its `then` as the conversion read it.
	remote *promise, *then;
	foreign(remote *promise, remote *then) : promise(promise), then(then) {}

	void settle(jsi::Runtime &rt) {
		c_waker w{};
		{
			::std::lock_guard<::std::mutex> lock(m);
			if (done) {
				return;
			}
			done = true;
			w = ::std::exchange(this->w, c_waker{});
		}
		unlocked(rt, [&] {
			if (w.wake != nullptr) {
				w.wake(w.data);
			}
			if (w.drop != nullptr) {
				w.drop(w.data);
			}
		});
	}

	// Whether it settled, or Rust dropped it.
	bool over() {
		::std::lock_guard<::std::mutex> lock(m);
		return done;
	}
};

inline bool foreign_poll(void *data, c_waker w) {
	auto *f = static_cast<::std::shared_ptr<foreign> *>(data);
	auto self = *f;
	c_waker old{};
	bool subscribe = false;
	{
		::std::lock_guard<::std::mutex> lock(self->m);
		if (self->done) {
			old = w;
		} else {
			old = ::std::exchange(self->w, w);
			subscribe = !::std::exchange(self->subscribed, true);
		}
	}
	if (old.drop != nullptr) {
		old.drop(old.data);
	}
	if (subscribe) {
		post(self->promise->w, [self](jsi::Runtime &rt) {
			// Dropped by Rust since, and cancelled: nothing waits for it.
			if (self->over()) {
				return;
			}
			// Settled, or cancelled if the promise is a call into Rust, whose `Cancelled` it
			// stands for; any other rejection aborts.
			auto then = jsi::Function::createFromHostFunction(
			    rt, jsi::PropNameID::forAscii(rt, "settled"), 1,
			    [self](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *, ::std::size_t) {
				    self->settle(rt);
				    return jsi::Value::undefined();
			    });
			auto caught = jsi::Function::createFromHostFunction(
			    rt, jsi::PropNameID::forAscii(rt, "rejected"), 1,
			    [self](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
				    // Once Rust dropped it, nothing waits for what it settles to.
				    if (self->over() || (n > 0 && bool_(rt, W(rt).is_cancelled.call(rt, jsi::Value(rt, a[0]))))) {
					    self->settle(rt);
					    return jsi::Value::undefined();
				    }
				    fail("a future given to Rust: it rejected with " + (n > 0 ? shown(rt, a[0]) : ::std::string("nothing")));
			    });
			jsi::Object p = self->promise->v.getObject(rt);
			self->then->v.getObject(rt).asFunction(rt).callWithThis(rt, p, then, caught);
		});
	}
	{
		::std::lock_guard<::std::mutex> lock(self->m);
		return self->done;
	}
}

inline void foreign_free(void *data) {
	auto *f = static_cast<::std::shared_ptr<foreign> *>(data);
	auto self = *f;
	delete f;
	c_waker w{};
	{
		::std::lock_guard<::std::mutex> lock(self->m);
		self->done = true;
		w = ::std::exchange(self->w, c_waker{});
	}
	if (w.drop != nullptr) {
		w.drop(w.data);
	}
	remote *p = self->promise, *t = self->then;
	// A call into Rust given back is cancelled once Rust drops it, at the drop, as the web's
	// is: cancelled later, its own poll, already posted, could take what it waited for.
	auto cancel = [p, t](jsi::Runtime &rt) {
		try {
			W(rt).cancel_call.call(rt, jsi::Value(rt, p->v));
		} catch (const jsi::JSError &e) {
			fail(::std::string("a future's drop: a call into JavaScript threw ") + e.getStack());
		}
		// Later, after a subscription its first poll posted, which reads them.
		release(p);
		release(t);
	};
	// Unless this thread runs JavaScript now, as a collection's drop does, or it is gone.
	auto home = p->w;
	if (running != nullptr || !home->alive.load(::std::memory_order_acquire)) {
		post(home, cancel);
	} else {
		on_js<bool>(home, "a future's drop", [&](jsi::Runtime &rt) {
			cancel(rt);
			return true;
		});
	}
}

// A thenable whose `then` is a function, checked here rather than at Rust's first poll,
// which runs later on a posted job.
inline c_foreign_future future_of(jsi::Runtime &rt, const jsi::Value &v) {
	jsi::Value then = v.isObject() ? v.getObject(rt).getProperty(rt, "then") : jsi::Value::undefined();
	if (!then.isObject() || !then.getObject(rt).isFunction(rt)) {
		type(rt, shown(rt, v) + " is no Promise");
	}
	auto *f = new ::std::shared_ptr<foreign>(::std::make_shared<foreign>(new remote(rt, v), new remote(rt, then)));
	return c_foreign_future{&foreign_poll, &foreign_free, f};
}

// Free `future` on the JavaScript thread before Rust took it, as a call whose later
// argument threw never made it: nothing polled it, so nothing was subscribed or cancelled.
inline void future_discard(c_foreign_future future) {
	auto *f = static_cast<::std::shared_ptr<foreign> *>(future.data);
	delete (*f)->promise;
	delete (*f)->then;
	delete f;
}

inline remote *frame::keep(const jsi::Value &v) {
	auto *r = new remote(rt, v);
	staged.emplace_back([r] { delete r; });
	return r;
}

inline c_waker frame::waker(const jsi::Value &v) {
	c_waker w = waker_of(rt, v);
	staged.emplace_back([w] { w.drop(w.data); });
	return w;
}

inline c_foreign_future frame::future(const jsi::Value &v) {
	c_foreign_future fut = future_of(rt, v);
	staged.emplace_back([fut] { future_discard(fut); });
	return fut;
}

// A reader JavaScript lends Rust for a call, read from any thread.
inline bool reader_error(const jsi::JSError &e, c_buf *err) {
	::std::string m = e.getMessage();
	*err = c_buf_copy(c_buf{reinterpret_cast<const ::std::uint8_t *>(m.data()), m.size(), nullptr});
	return false;
}

inline bool reader_read(void *data, ::std::uint8_t *buf, ::std::size_t len, ::std::size_t *n, c_buf *err) {
	auto *r = static_cast<remote *>(data);
	return out<bool>(r->w, "a reader's read", [&](jsi::Runtime &rt) -> bool {
		// The chunk's bytes are this buffer's, which the call reads back itself: none of the
		// chunk's properties, which `read` may redefine, nor its `ArrayBuffer`, which `read` may
		// detach, says where they are.
		auto into = ::std::make_shared<copy>(nullptr, len);
		jsi::Object chunk = obj(rt, W(rt).uint8.callAsConstructor(rt, jsi::ArrayBuffer(rt, into)), "Uint8Array");
		jsi::Value got;
		try {
			jsi::Object reader = r->v.getObject(rt);
			got = reader.getPropertyAsFunction(rt, "read").callWithThis(rt, reader, chunk);
		} catch (const jsi::JSError &e) {
			return reader_error(e, err);
		}
		double g = got.isNumber() ? got.getNumber() : -1;
		if (integral(g) && g > 0 && g <= static_cast<double>(len)) {
			::std::memcpy(buf, into->data(), static_cast<::std::size_t>(g));
		}
		// Past `len`, which Rust refuses, where it is no count.
		*n = integral(g) && g >= 0 && g <= static_cast<double>(len) ? static_cast<::std::size_t>(g) : SIZE_MAX;
		return true;
	});
}

inline bool reader_seek(void *data, int whence, ::std::int64_t offset, ::std::uint64_t *position, c_buf *err) {
	auto *r = static_cast<remote *>(data);
	return out<bool>(r->w, "a reader's seek", [&](jsi::Runtime &rt) -> bool {
		const char *from = whence == 0 ? "Start" : whence == 1 ? "Current" : "End";
		jsi::Value at = whence == 0 ? of_u64(rt, static_cast<::std::uint64_t>(offset)) : of_i64(rt, offset);
		jsi::Value got;
		try {
			jsi::Object reader = r->v.getObject(rt);
			got = reader.getPropertyAsFunction(rt, "seek").callWithThis(rt, reader, at, jsi::String::createFromAscii(rt, from));
		} catch (const jsi::JSError &e) {
			return reader_error(e, err);
		}
		*position = u64(rt, got);
		return true;
	});
}

inline c_foreign_reader reader_of(frame &f, const jsi::Value &v) {
	obj(f.rt, v, "Reader");
	auto *r = new remote(f.rt, v);
	f.mem.push_back(::std::shared_ptr<remote>(r));
	return c_foreign_reader{&reader_read, &reader_seek, r};
}

// An implementation of a trait JavaScript gave Rust, closed with `close` once Rust drops it,
// later on the JavaScript thread.
inline void trait_free(void *data) {
	auto *r = static_cast<remote *>(data);
	auto close = [r](jsi::Runtime &rt) {
		try {
			jsi::Object o = r->v.getObject(rt);
			jsi::Value close = o.getProperty(rt, "close");
			if (close.isObject()) {
				close.getObject(rt).asFunction(rt).callWithThis(rt, o);
			}
		} catch (const jsi::JSError &e) {
			fail(::std::string("a drop: a call into JavaScript threw ") + e.getStack());
		}
		delete r;
	};
	// At the drop, as Rust's own runs, unless this thread runs JavaScript now.
	if (running != nullptr || !r->w->alive.load(::std::memory_order_acquire)) {
		post(r->w, close);
	} else {
		auto w = r->w;
		on_js<bool>(w, "a drop", [&](jsi::Runtime &rt) {
			close(rt);
			return true;
		});
	}
}

// A closure JavaScript gave Rust to keep, freed once Rust drops it.
inline void closure_free(void *data) {
	release(static_cast<remote *>(data));
}

// The implementation `data` holds, a trait's or a closure's.
inline jsi::Object found(jsi::Runtime &rt, void *data) {
	return static_cast<remote *>(data)->v.getObject(rt);
}

// Futures: a call into Rust's async API polls its future once as it is made, and again on
// the JavaScript thread each time Rust wakes it, never inside the wake.
struct pending final : node {
	::std::shared_ptr<world> w;
	void *f;
	void (*free_)(void *);
	// Polls the future with a waker, settling the promise once it is ready: whether it was.
	::std::function<bool(jsi::Runtime &, void *, c_waker, jsi::Function &, jsi::Function &)> poll_;
	::std::optional<jsi::Function> resolve, reject;
	::std::atomic<bool> queued{false};
	bool given = false;
	bool polling = false;
	bool cancelled = false;

	pending(jsi::Runtime &rt, void *f, void (*free)(void *)) : w(of(rt)), f(f), free_(free) {}
	// Once nothing can poll it: its promise collected, and Rust's waker dropped, which may be
	// on any thread, even once its runtime is gone. On the JavaScript thread, as the collector
	// drops it, its future is freed on a posted job. A promise never settled leaves its
	// functions to free on the JavaScript thread, which `post` leaks once the runtime is gone.
	~pending() override {
		if (f != nullptr && w->here()) {
			collected(w, [free = free_, f = f] { free(f); });
		} else if (f != nullptr) {
			free_(f);
		}
		if (resolve.has_value()) {
			auto *fs = new ::std::pair<jsi::Function, jsi::Function>(::std::move(*resolve), ::std::move(*reject));
			post(w, [fs](jsi::Runtime &) { delete fs; });
		}
	}

	void wake() override {
		if (queued.exchange(true, ::std::memory_order_acq_rel)) {
			return;
		}
		post(w, [this, self = held(this)](jsi::Runtime &rt) {
			queued.store(false, ::std::memory_order_release);
			if (f != nullptr && !polling) {
				step(rt);
			}
		});
	}

	void step(jsi::Runtime &rt) {
		polling = true;
		c_waker waker = given ? lend(this) : give(this);
		given = true;
		bool done;
		try {
			done = poll_(rt, f, waker, *resolve, *reject);
		} catch (const jsi::JSError &e) {
			polling = false;
			finish(rt);
			reject->call(rt, e.value());
			settled();
			return;
		}
		polling = false;
		if (done) {
			finish(rt);
			settled();
		} else if (cancelled) {
			cancel(rt);
		}
	}

	// Its promise's functions go once it settled, which hold its promise, whose `cancel`
	// holds this.
	void settled() {
		resolve.reset();
		reject.reset();
	}

	// `f` is detached while this thread holds the runtime, so a `cancel` from a thread that
	// takes it during the free finds it gone.
	void finish(jsi::Runtime &rt) {
		if (void *g = ::std::exchange(f, nullptr)) {
			unlocked(rt, [&] { free_(g); });
		}
	}

	void cancel(jsi::Runtime &rt) {
		finish(rt);
		reject->call(rt, W(rt).cancelled.call(rt));
		settled();
	}
};

// The `Call` of a future that pended at its start, `f`, which `poll` polls and `free`
// frees: its promise, whose `cancel` cancels it in Rust.
inline jsi::Value pend(jsi::Runtime &rt, void *f, void (*free)(void *),
                       ::std::function<bool(jsi::Runtime &, void *, c_waker, jsi::Function &, jsi::Function &)> poll) {
	auto *p = new pending(rt, f, free);
	p->poll_ = ::std::move(poll);
	// The promise's `cancel` holds a reference, for a call however late.
	p->ref();
	::std::shared_ptr<pending> held(p, [](pending *p) { p->unref(); });
	auto cancel = jsi::Function::createFromHostFunction(
	    rt, jsi::PropNameID::forAscii(rt, "cancel"), 0,
	    [held](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *, ::std::size_t) {
		    pending *p = held.get();
		    if (p->f != nullptr && !p->cancelled) {
			    p->cancelled = true;
			    if (!p->polling) {
				    p->cancel(rt);
			    }
		    }
		    return jsi::Value::undefined();
	    });
	jsi::Array made = arr(rt, W(rt).pending.call(rt, cancel));
	p->resolve.emplace(function(rt, made.getValueAtIndex(rt, 1)));
	p->reject.emplace(function(rt, made.getValueAtIndex(rt, 2)));
	jsi::Value promise = made.getValueAtIndex(rt, 0);
	// Rust holds a reference once polled; the module's own goes now.
	p->wake();
	p->unref();
	return promise;
}

// The `Call` of a future ready at its start.
inline jsi::Value ready(jsi::Runtime &rt, jsi::Value v) {
	return W(rt).ready.call(rt, v);
}

// A value that is not `Send`, freed on another thread, drops on its own: the JavaScript
// thread drains what other threads freed of its values on a job Rust's wake posts.
struct drainer final : node {
	::std::shared_ptr<world> w;
	explicit drainer(::std::shared_ptr<world> w) : w(::std::move(w)) {}
	void wake() override {
		post(w, [](jsi::Runtime &rt) { unlocked(rt, [] { c_thread_drain(); }); });
	}
};

// Install the runtime into `rt`, whose `kit` is what `rt.ts` gives it. `rt` holds its world
// until it is destroyed.
inline void install(jsi::Runtime &rt, post_t post, jsi::Object kit) {
	// Once only, so what Rust holds of the world stays the runtime's, and no kit but the first,
	// the binding's own, which `load` installs before any call, reaches the module.
	if (rt.getRuntimeData(key()) != nullptr) {
		throw jsi::JSError(rt, "the JSI module is installed in this runtime already");
	}
	auto w = ::std::make_shared<world>(rt, ::std::move(post), ::std::move(kit));
	rt.setRuntimeData(key(), ::std::make_shared<teardown>(w));
	running = w.get();
	if (c_drains) {
		auto *d = new drainer(w);
		c_thread_wake(lend(d));
	}
}

// The functions of `rt.ts`'s handles, bytes, and wakers.
inline void natives(jsi::Runtime &rt, jsi::Object &n) {
	auto fn = [&](const char *name, unsigned argc, jsi::HostFunctionType f) {
		n.setProperty(rt, name, jsi::Function::createFromHostFunction(rt, jsi::PropNameID::forAscii(rt, name), argc, ::std::move(f)));
	};
	fn("$dispose", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		auto c = state_of<cell>(rt, arg(a, n, 0), "handle");
		if (c->closed) {
			return jsi::Value::undefined();
		}
		if (c->owned && c->k->drop != nullptr) {
			::std::int32_t s = 0;
			unlocked(rt, [&] { c->k->drop(c->p, &s); });
			status(rt, s, c->k->dropping);
		} else if (c->h != 1) {
			error(rt, ::std::string(c->k->name) + ": a handle is already borrowed");
		}
		c->closed = true;
		c->release(rt);
		return jsi::Value::undefined();
	});
	fn("$bytesFree", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		auto b = state_of<bytes_cell>(rt, arg(a, n, 0), "Bytes of Rust's buffer");
		if (!b->closed) {
			b->closed = true;
			b->release();
		}
		return jsi::Value::undefined();
	});
	// The byte at `i`, checked open and in bounds: `at`'s, whose index it converted.
	fn("$bytesAt", 2, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		auto b = state_of<bytes_cell>(rt, arg(a, n, 0), "Bytes of Rust's buffer");
		if (b->closed) {
			error(rt, "Bytes is closed");
		}
		const jsi::Value &v = arg(a, n, 1);
		double d = v.isNumber() ? v.getNumber() : -1;
		if (!(d >= 0 && d < static_cast<double>(b->buf.len) && d == ::std::floor(d))) {
			type(rt, shown(rt, v) + " is no index of the bytes");
		}
		return jsi::Value(static_cast<double>(b->buf.ptr[static_cast<::std::size_t>(d)]));
	});
	// A copy of the bytes from `from` up to `to`, checked open and read at once, so no
	// JavaScript runs between: `slice`'s, whose bounds it clamped.
	fn("$bytesSlice", 3, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		auto b = state_of<bytes_cell>(rt, arg(a, n, 0), "Bytes of Rust's buffer");
		if (b->closed) {
			error(rt, "Bytes is closed");
		}
		auto bound = [&](const jsi::Value &v) {
			double d = v.isNumber() ? v.getNumber() : -1;
			if (!(d >= 0 && d <= static_cast<double>(b->buf.len) && d == ::std::floor(d))) {
				type(rt, shown(rt, v) + " is no bound of the bytes");
			}
			return static_cast<::std::size_t>(d);
		};
		::std::size_t from = bound(arg(a, n, 1));
		::std::size_t to = bound(arg(a, n, 2));
		if (to < from) {
			type(rt, "a slice ends before it starts");
		}
		return W(rt).uint8.callAsConstructor(rt, copied(rt, b->buf.ptr + from, to - from));
	});
	fn("$wake", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		state_of<waker_cell>(rt, arg(a, n, 0), "Waker")->wake(rt);
		return jsi::Value::undefined();
	});
	fn("$wakerDrop", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		state_of<waker_cell>(rt, arg(a, n, 0), "Waker")->dispose(rt);
		return jsi::Value::undefined();
	});
	fn("$encode", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		frame f(rt);
		c_buf b = f.utf8(arg(a, n, 0));
		return W(rt).uint8.callAsConstructor(rt, copied(rt, b.ptr, b.len));
	});
	fn("$decode", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		frame f(rt);
		c_buf b = f.array(arg(a, n, 0), "Uint8Array");
		// What is not UTF-8 is replaced, as Rust's `from_utf8_lossy` does.
		::std::string s;
		for (::std::size_t i = 0; i < b.len;) {
			::std::uint8_t c = b.ptr[i];
			// A lead that starts only overlong or out-of-range sequences, 0xc0, 0xc1, and from
			// 0xf5, is invalid alone, never consuming what follows.
			::std::size_t len = c < 0x80 ? 1 : c < 0xc2 ? 0 : c < 0xe0 ? 2 : c < 0xf0 ? 3 : c < 0xf5 ? 4 : 0;
			bool valid = len != 0 && i + len <= b.len;
			::std::uint32_t cp = len == 1 ? c : len == 2 ? c & 0x1f : len == 3 ? c & 0x0f : c & 0x07;
			for (::std::size_t k = 1; valid && k < len; k++) {
				valid = (b.ptr[i + k] & 0xc0) == 0x80;
				cp = cp << 6 | (b.ptr[i + k] & 0x3f);
			}
			valid = valid && !(len == 2 && cp < 0x80) && !(len == 3 && (cp < 0x800 || (cp >= 0xd800 && cp <= 0xdfff))) &&
			        !(len == 4 && (cp < 0x10000 || cp > 0x10ffff));
			if (valid) {
				s.append(reinterpret_cast<const char *>(b.ptr + i), len);
				i += len;
			} else {
				s.append("\xef\xbf\xbd");
				// Past the longest prefix of a valid sequence, as the WHATWG decoder does.
				::std::size_t k = 1;
				while (len > 1 && k < len && i + k < b.len && (b.ptr[i + k] & 0xc0) == 0x80) {
					::std::uint8_t lo = 0x80, hi = 0xbf;
					if (k == 1 && c == 0xe0) lo = 0xa0;
					if (k == 1 && c == 0xed) hi = 0x9f;
					if (k == 1 && c == 0xf0) lo = 0x90;
					if (k == 1 && c == 0xf4) hi = 0x8f;
					if (b.ptr[i + k] < lo || b.ptr[i + k] > hi) break;
					k++;
				}
				i += k;
			}
		}
		return jsi::String::createFromUtf8(rt, reinterpret_cast<const ::std::uint8_t *>(s.data()), s.size());
	});
}

} // namespace r
