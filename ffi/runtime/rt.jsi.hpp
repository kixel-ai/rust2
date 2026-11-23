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

// The runtime the module is installed in, and what `rt.ts` gave it. A reload installs the
// module into a new runtime, so what Rust still holds of an old one checks `alive`, and
// leaks its jsi values rather than touch a runtime that may be gone.
struct world {
	jsi::Runtime *rt;
	::std::thread::id js;
	post_t post;
	::std::atomic<bool> alive{true};
	jsi::Object kit;
	jsi::Function pending, bytes, uint8, range, type, error;
	jsi::Value none, handle, waker;
	// Calls into Rust under way on the JavaScript thread.
	int depth = 0;

	world(jsi::Runtime &rt, post_t post, jsi::Object kit)
	    : rt(&rt), js(::std::this_thread::get_id()), post(::std::move(post)), kit(::std::move(kit)),
	      pending(fn(rt, "pending")), bytes(fn(rt, "bytes")), uint8(fn(rt, "Uint8Array")),
	      range(fn(rt, "RangeError")), type(fn(rt, "TypeError")), error(fn(rt, "Error")),
	      none(this->kit.getProperty(rt, "None")),
	      handle(this->kit.getProperty(rt, "Handle")), waker(this->kit.getProperty(rt, "Waker")) {}

	jsi::Function fn(jsi::Runtime &rt, const char *name) {
		return kit.getPropertyAsFunction(rt, name);
	}

	bool here() const {
		return ::std::this_thread::get_id() == js;
	}
};

inline ::std::shared_ptr<world> &current() {
	static ::std::shared_ptr<world> w;
	return w;
}

inline world &W() {
	return *current();
}

// Errors.

[[noreturn]] inline void raise(jsi::Runtime &rt, jsi::Function &ctor, const ::std::string &what) {
	throw jsi::JSError(rt, ctor.callAsConstructor(rt, jsi::String::createFromUtf8(rt, what)));
}

[[noreturn]] inline void range(jsi::Runtime &rt, const ::std::string &what) {
	raise(rt, W().range, what);
}

[[noreturn]] inline void type(jsi::Runtime &rt, const ::std::string &what) {
	raise(rt, W().type, what);
}

[[noreturn]] inline void error(jsi::Runtime &rt, const ::std::string &what) {
	raise(rt, W().error, what);
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

// A helper of `rt.ts` by its name, called with `args`.
template <class... A>
jsi::Value call(jsi::Runtime &rt, const char *name, A &&...args) {
	return W().fn(rt, name).call(rt, ::std::forward<A>(args)...);
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
	call(rt, "int", jsi::Value(rt, v), min, max, what);
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
	call(rt, "char", jsi::Value(rt, v));
	fail("rt.char passed what the JSI module refused");
}

constexpr double safe = 9007199254740991.0;

// The halves `rt.ts`'s `name` checks and returns of `v`.
inline ::std::pair<double, double> halves(jsi::Runtime &rt, const char *name, const jsi::Value &v) {
	jsi::Array a = call(rt, name, jsi::Value(rt, v)).getObject(rt).getArray(rt);
	return {a.getValueAtIndex(rt, 0).getNumber(), a.getValueAtIndex(rt, 1).getNumber()};
}

inline ::std::uint64_t u64(jsi::Runtime &rt, const jsi::Value &v) {
	if (v.isNumber()) {
		double d = v.getNumber();
		if (integral(d) && d >= 0 && d <= safe) {
			return static_cast<::std::uint64_t>(d);
		}
	}
	auto [hi, lo] = halves(rt, "u64Halves", v);
	return static_cast<::std::uint64_t>(hi) << 32 | static_cast<::std::uint64_t>(lo);
}

inline ::std::int64_t i64(jsi::Runtime &rt, const jsi::Value &v) {
	if (v.isNumber()) {
		double d = v.getNumber();
		if (integral(d) && d >= -safe && d <= safe) {
			return static_cast<::std::int64_t>(d);
		}
	}
	auto [hi, lo] = halves(rt, "i64Halves", v);
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

inline jsi::Value u64(jsi::Runtime &, ::std::uint64_t v) {
	return static_cast<double>(v);
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
	jsi::Array a = call(rt, "u128Words", jsi::Value(rt, v)).getObject(rt).getArray(rt);
	::std::uint64_t w[4];
	for (::std::size_t i = 0; i < 4; i++) {
		w[i] = static_cast<::std::uint64_t>(a.getValueAtIndex(rt, i).getNumber());
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
	auto bits = static_cast<::std::uint32_t>(call(rt, "f32Bits", jsi::Value(rt, v)).getNumber());
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
	auto [hi, lo] = halves(rt, "f64Bits", v);
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
};

// A handle to a Rust object, held by the JavaScript object it is the state of: it counts
// the calls holding it, and is freed once disposed and no call holds it, or once the
// garbage collector finds it unreachable if never disposed.
struct cell : jsi::NativeState {
	void *p;
	const kind *k;
	::std::uint32_t h = 1;
	bool closed = false;
	bool owned;
	bool freed = false;

	cell(void *p, const kind *k, bool owned) : p(p), k(k), owned(owned) {}

	~cell() override {
		if (owned && !freed) {
			k->free(p);
		}
	}

	void release() {
		if (--h == 0 && owned && !freed) {
			freed = true;
			k->free(p);
		}
	}
};

// A new handle of `k` to `p`: owned, or else lent for a call into JavaScript.
inline jsi::Value made(jsi::Runtime &rt, void *p, const kind &k, bool owned) {
	jsi::Object o = jsi::Object::create(rt, W().handle);
	o.setNativeState(rt, ::std::make_shared<cell>(p, &k, owned));
	return o;
}

inline ::std::shared_ptr<cell> cell_of(jsi::Runtime &rt, const jsi::Value &v, const kind &k) {
	if (v.isObject()) {
		jsi::Object o = v.getObject(rt);
		if (o.hasNativeState<cell>(rt)) {
			auto c = o.getNativeState<cell>(rt);
			if (c->k == &k) {
				return c;
			}
		}
	}
	type(rt, shown(rt, v) + " is no " + k.name);
}

// Bytes a call returned, the state of an `rt.Bytes`: Rust's buffer, lent in place.
struct bytes_cell : jsi::NativeState {
	c_buf buf;
	::std::uint32_t h = 1;
	bool closed = false;

	explicit bytes_cell(c_buf buf) : buf(buf) {}

	~bytes_cell() override {
		if (buf.owner != nullptr) {
			c_buf_free(buf);
		}
	}

	void release() {
		if (--h == 0 && buf.owner != nullptr) {
			c_buf_free(buf);
			buf.owner = nullptr;
		}
	}
};

// The view an `ArrayBuffer` of returned bytes reads, which the cell frees.
struct view final : jsi::MutableBuffer {
	::std::uint8_t *ptr;
	::std::size_t len;
	::std::shared_ptr<void> keep;
	view(::std::uint8_t *ptr, ::std::size_t len, ::std::shared_ptr<void> keep)
	    : ptr(ptr), len(len), keep(::std::move(keep)) {}
	::std::size_t size() const override {
		return len;
	}
	::std::uint8_t *data() override {
		return ptr;
	}
};

// A copy of bytes, as an `ArrayBuffer` owns.
struct copy final : jsi::MutableBuffer {
	::std::vector<::std::uint8_t> v;
	explicit copy(const ::std::uint8_t *ptr, ::std::size_t len) : v(ptr, ptr + len) {}
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
	return W().uint8.callAsConstructor(rt, ab);
}

// The typed array `ctor` names of a copy of `len` elements of `size` bytes at `ptr`.
inline jsi::Value typed(jsi::Runtime &rt, const char *ctor, const void *ptr, ::std::size_t len, ::std::size_t size) {
	jsi::ArrayBuffer ab = copied(rt, static_cast<const ::std::uint8_t *>(ptr), len * size);
	return rt.global().getPropertyAsFunction(rt, ctor).callAsConstructor(rt, ab);
}

// The bytes Rust returned: lent past 64 bytes, else copied and freed, as an `rt.Bytes`.
inline jsi::Value returned(jsi::Runtime &rt, c_buf buf) {
	if (buf.len <= 64 || buf.owner == nullptr) {
		jsi::ArrayBuffer ab = copied(rt, buf.ptr, buf.len);
		if (buf.owner != nullptr) {
			c_buf_free(buf);
		}
		return W().bytes.call(rt, ab, false);
	}
	auto state = ::std::make_shared<bytes_cell>(buf);
	auto *ptr = const_cast<::std::uint8_t *>(buf.ptr);
	jsi::ArrayBuffer ab(rt, ::std::make_shared<view>(ptr, buf.len, state));
	jsi::Object b = W().bytes.call(rt, ab, true).getObject(rt);
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

// What a call holds until it returns: copies of text, arrays it lowered, and the handles
// and bytes it holds.
struct frame {
	jsi::Runtime &rt;
	::std::deque<::std::string> text;
	::std::vector<::std::shared_ptr<void>> mem;
	::std::vector<::std::shared_ptr<cell>> held;
	::std::vector<::std::shared_ptr<bytes_cell>> bytes;
	::std::deque<jsi::Value> keep;
	::std::vector<::std::shared_ptr<cell>> lent;

	explicit frame(jsi::Runtime &rt) : rt(rt) {
		W().depth++;
	}
	frame(const frame &) = delete;
	frame &operator=(const frame &) = delete;
	~frame() {
		W().depth--;
		for (auto &c : held) {
			c->release();
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
	// the call returns. Where `shared`, an `rt.Bytes` of Rust's buffer passes a second owner
	// of it, which Rust takes.
	c_buf view(const jsi::Value &v, bool shared = false) {
		if (v.isObject()) {
			jsi::Object o = v.getObject(rt);
			if (o.hasNativeState<bytes_cell>(rt)) {
				auto b = o.getNativeState<bytes_cell>(rt);
				if (b->closed) {
					error(rt, "Bytes is closed");
				}
				b->h++;
				bytes.push_back(b);
				return shared ? c_buf_clone(b->buf) : c_buf{b->buf.ptr, b->buf.len, nullptr};
			}
			jsi::Value copy = call(rt, "bytesCopy", jsi::Value(rt, v));
			if (!copy.isUndefined()) {
				// A copy `rt.Bytes` holds.
				return array(copy, "Uint8Array");
			}
			if (o.instanceOf(rt, W().uint8)) {
				return array(v, "Uint8Array");
			}
		}
		type(rt, shown(rt, v) + " is no Uint8Array");
	}

	// The bytes a typed array `v`, of the constructor `ctor`, views, held until the call
	// returns.
	c_buf array(const jsi::Value &v, const char *ctor) {
		if (!v.isObject() || !v.getObject(rt).instanceOf(rt, rt.global().getPropertyAsFunction(rt, ctor))) {
			type(rt, shown(rt, v) + " is no " + ctor);
		}
		jsi::Object o = v.getObject(rt);
		jsi::ArrayBuffer ab = o.getPropertyAsObject(rt, "buffer").getArrayBuffer(rt);
		auto offset = static_cast<::std::size_t>(o.getProperty(rt, "byteOffset").getNumber());
		auto len = static_cast<::std::size_t>(o.getProperty(rt, "byteLength").getNumber());
		const ::std::uint8_t *ptr = ab.data(rt) + offset;
		keep.emplace_back(rt, v);
		return c_buf{ptr, len, nullptr};
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
	return call(rt, "opaqueMade", W().uint8.callAsConstructor(rt, copied(rt, static_cast<const ::std::uint8_t *>(p), n)));
}

inline void opaque_put(frame &f, const jsi::Value &v, void *p, ::std::size_t n) {
	jsi::Value bytes = call(f.rt, "opaqueBytes", jsi::Value(f.rt, v));
	c_buf b = f.array(bytes, "Uint8Array");
	if (b.len != n) {
		fail("an opaque value of another size");
	}
	::std::memcpy(p, b.ptr, n);
}

// Std's values.

inline jsi::Value octets(jsi::Runtime &rt, const ::std::uint8_t *p, ::std::size_t n) {
	jsi::Object o(rt);
	o.setProperty(rt, "octets", W().uint8.callAsConstructor(rt, copied(rt, p, n)));
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
	return jsi::Value::strictEquals(rt, v, W().none);
}

// What JavaScript implements for Rust: values Rust holds, freed on the JavaScript thread.

// A JavaScript value Rust holds, freed on the JavaScript thread: in a runtime since gone,
// leaked.
struct remote {
	::std::shared_ptr<world> w;
	jsi::Value v;
	remote(jsi::Runtime &rt, const jsi::Value &v) : w(current()), v(rt, v) {}
};

// Run `job` on the JavaScript thread later, unless its runtime is gone.
inline void post(const ::std::shared_ptr<world> &w, ::std::function<void(jsi::Runtime &)> job) {
	if (w->alive.load(::std::memory_order_acquire)) {
		w->post(::std::move(job));
	}
}

// Free `r` on its thread: now if this is it, else posted, or leaked if its runtime is gone.
inline void release(remote *r) {
	if (r->w->here()) {
		delete r;
		return;
	}
	if (!r->w->alive.load(::std::memory_order_acquire)) {
		return;
	}
	auto w = r->w;
	w->post([r](jsi::Runtime &) { delete r; });
}

// Run `job` on `w`'s JavaScript thread, now if this is it, else posted and waited for: a
// call Rust makes from another thread while the JavaScript thread waits on that thread in
// a call into Rust deadlocks.
template <class T>
T on_js(const ::std::shared_ptr<world> &w, const char *what, ::std::function<T(jsi::Runtime &)> job) {
	if (w->here()) {
		return job(*w->rt);
	}
	if (!w->alive.load(::std::memory_order_acquire)) {
		fail(::std::string(what) + ": called after its JavaScript runtime was gone");
	}
	::std::mutex m;
	::std::condition_variable cv;
	bool done = false;
	::std::optional<T> out;
	w->post([&](jsi::Runtime &rt) {
		T v = job(rt);
		::std::lock_guard<::std::mutex> lock(m);
		out.emplace(::std::move(v));
		done = true;
		cv.notify_one();
	});
	::std::unique_lock<::std::mutex> lock(m);
	cv.wait(lock, [&] { return done; });
	return ::std::move(*out);
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

// A function JavaScript gave as a waker: each wake calls it on a microtask of its thread.
struct js_waker final : node {
	remote *f;
	explicit js_waker(remote *f) : f(f) {}
	~js_waker() override {
		release(f);
	}
	void wake() override {
		remote *f = this->f;
		ref();
		post(f->w, [this, f](jsi::Runtime &rt) {
			try {
				f->v.getObject(rt).asFunction(rt).call(rt);
			} catch (const jsi::JSError &e) {
				fail(::std::string("a waker: a call into JavaScript threw ") + e.getStack());
			}
			unref();
		});
	}
};

inline c_waker waker_of(jsi::Runtime &rt, const jsi::Value &v) {
	if (!v.isObject() || !v.getObject(rt).isFunction(rt)) {
		type(rt, shown(rt, v) + " is no function");
	}
	return lend(new js_waker(new remote(rt, v)));
}

// A waker Rust returned, the state of an `rt.Waker`.
struct waker_cell : jsi::NativeState {
	c_waker w;
	bool dropped = false;
	explicit waker_cell(c_waker w) : w(w) {}
	~waker_cell() override {
		drop();
	}
	void drop() {
		if (!dropped) {
			dropped = true;
			if (w.drop != nullptr) {
				w.drop(w.data);
			}
		}
	}
};

inline jsi::Value of_waker(jsi::Runtime &rt, c_waker w) {
	jsi::Object o = jsi::Object::create(rt, W().waker);
	o.setNativeState(rt, ::std::make_shared<waker_cell>(w));
	return o;
}

// A promise JavaScript gave Rust as a future: settled once, which wakes the waker of Rust's
// last poll, and polled from any thread.
struct foreign {
	::std::mutex m;
	bool done = false;
	bool subscribed = false;
	c_waker w{};
	remote *promise;
	explicit foreign(remote *promise) : promise(promise) {}

	void settle() {
		c_waker w{};
		{
			::std::lock_guard<::std::mutex> lock(m);
			if (done) {
				return;
			}
			done = true;
			w = ::std::exchange(this->w, c_waker{});
		}
		if (w.wake != nullptr) {
			w.wake(w.data);
		}
		if (w.drop != nullptr) {
			w.drop(w.data);
		}
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
			// Settled, or cancelled if the promise is a call into Rust, whose `Cancelled` it
			// stands for; any other rejection aborts.
			auto then = jsi::Function::createFromHostFunction(
			    rt, jsi::PropNameID::forAscii(rt, "settled"), 1,
			    [self](jsi::Runtime &, const jsi::Value &, const jsi::Value *, ::std::size_t) {
				    self->settle();
				    return jsi::Value::undefined();
			    });
			auto caught = jsi::Function::createFromHostFunction(
			    rt, jsi::PropNameID::forAscii(rt, "rejected"), 1,
			    [self](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
				    if (n > 0 && call(rt, "isCancelled", jsi::Value(rt, a[0])).getBool()) {
					    self->settle();
					    return jsi::Value::undefined();
				    }
				    fail("a future given to Rust: it rejected with " + (n > 0 ? shown(rt, a[0]) : ::std::string("nothing")));
			    });
			jsi::Object p = self->promise->v.getObject(rt);
			p.getPropertyAsFunction(rt, "then").callWithThis(rt, p, then, caught);
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
	remote *p = self->promise;
	// A call into Rust given back is cancelled once Rust drops it.
	post(p->w, [p](jsi::Runtime &rt) {
		call(rt, "cancelCall", jsi::Value(rt, p->v));
		delete p;
	});
}

inline c_foreign_future future_of(jsi::Runtime &rt, const jsi::Value &v) {
	if (!v.isObject() || !v.getObject(rt).hasProperty(rt, "then")) {
		type(rt, shown(rt, v) + " is no Promise");
	}
	auto *f = new ::std::shared_ptr<foreign>(::std::make_shared<foreign>(new remote(rt, v)));
	return c_foreign_future{&foreign_poll, &foreign_free, f};
}

// A reader JavaScript lends Rust for a call, read from any thread.
inline bool reader_error(jsi::Runtime &rt, const jsi::JSError &e, c_buf *err) {
	::std::string m = e.getMessage();
	*err = c_buf_copy(c_buf{reinterpret_cast<const ::std::uint8_t *>(m.data()), m.size(), nullptr});
	return false;
}

inline bool reader_read(void *data, ::std::uint8_t *buf, ::std::size_t len, ::std::size_t *n, c_buf *err) {
	auto *r = static_cast<remote *>(data);
	return out<bool>(r->w, "a reader's read", [&](jsi::Runtime &rt) -> bool {
		jsi::Object chunk = W().uint8.callAsConstructor(rt, static_cast<double>(len)).getObject(rt);
		jsi::Value got;
		try {
			jsi::Object reader = r->v.getObject(rt);
			got = reader.getPropertyAsFunction(rt, "read").callWithThis(rt, reader, chunk);
		} catch (const jsi::JSError &e) {
			return reader_error(rt, e, err);
		}
		double g = got.isNumber() ? got.getNumber() : -1;
		if (integral(g) && g > 0 && g <= static_cast<double>(len)) {
			jsi::ArrayBuffer ab = chunk.getPropertyAsObject(rt, "buffer").getArrayBuffer(rt);
			::std::memcpy(buf, ab.data(rt), static_cast<::std::size_t>(g));
		}
		*n = integral(g) && g >= 0 && g <= 4294967295.0 ? static_cast<::std::size_t>(g) : 0xffffffff;
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
			return reader_error(rt, e, err);
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

// An implementation of a trait JavaScript gave Rust, closed with `close` once Rust drops it.
inline void trait_free(void *data) {
	auto *r = static_cast<remote *>(data);
	auto w = r->w;
	if (!w->alive.load(::std::memory_order_acquire)) {
		return;
	}
	auto job = [r](jsi::Runtime &rt) {
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
	if (w->here()) {
		job(*w->rt);
	} else {
		w->post(job);
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

	pending(void *f, void (*free)(void *)) : w(current()), f(f), free_(free) {}

	void wake() override {
		if (queued.exchange(true, ::std::memory_order_acq_rel)) {
			return;
		}
		ref();
		post(w, [this](jsi::Runtime &rt) {
			queued.store(false, ::std::memory_order_release);
			if (f != nullptr && !polling) {
				step(rt);
			}
			unref();
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
			finish();
			reject->call(rt, e.value());
			return;
		}
		polling = false;
		if (done) {
			finish();
		} else if (cancelled) {
			cancel(rt);
		}
	}

	void finish() {
		if (f != nullptr) {
			free_(::std::exchange(f, nullptr));
		}
	}

	void cancel(jsi::Runtime &rt) {
		finish();
		reject->call(rt, call(rt, "cancelled"));
	}
};

// The `Call` of a future that pended at its start, `f`, which `poll` polls and `free`
// frees: its promise, whose `cancel` cancels it in Rust.
inline jsi::Value pend(jsi::Runtime &rt, void *f, void (*free)(void *),
                       ::std::function<bool(jsi::Runtime &, void *, c_waker, jsi::Function &, jsi::Function &)> poll) {
	auto *p = new pending(f, free);
	p->poll_ = ::std::move(poll);
	auto cancel = jsi::Function::createFromHostFunction(
	    rt, jsi::PropNameID::forAscii(rt, "cancel"), 0,
	    [p](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *, ::std::size_t) {
		    if (p->f != nullptr && !p->cancelled) {
			    p->cancelled = true;
			    if (!p->polling) {
				    p->cancel(rt);
			    }
		    }
		    return jsi::Value::undefined();
	    });
	jsi::Array made = W().pending.call(rt, cancel).getObject(rt).getArray(rt);
	p->resolve.emplace(made.getValueAtIndex(rt, 1).getObject(rt).asFunction(rt));
	p->reject.emplace(made.getValueAtIndex(rt, 2).getObject(rt).asFunction(rt));
	jsi::Value promise = made.getValueAtIndex(rt, 0);
	// Rust holds a reference once polled; the module's own goes now.
	p->wake();
	p->unref();
	return promise;
}

// The `Call` of a future ready at its start.
inline jsi::Value ready(jsi::Runtime &rt, jsi::Value v) {
	return call(rt, "ready", v);
}

// A value that is not `Send`, freed on another thread, drops on its own: the JavaScript
// thread drains what other threads freed of its values on a job Rust's wake posts.
struct drainer final : node {
	::std::shared_ptr<world> w;
	explicit drainer(::std::shared_ptr<world> w) : w(::std::move(w)) {}
	void wake() override {
		post(w, [](jsi::Runtime &) { c_thread_drain(); });
	}
};

// Install the runtime into `rt`, whose `kit` is what `rt.ts` gives it, replacing any
// runtime installed before.
inline void install(jsi::Runtime &rt, post_t post, jsi::Object kit) {
	if (auto &old = current()) {
		old->alive.store(false, ::std::memory_order_release);
	}
	current() = ::std::make_shared<world>(rt, ::std::move(post), ::std::move(kit));
	if (c_drains) {
		auto *d = new drainer(current());
		c_thread_wake(lend(d));
	}
}

// The functions of `rt.ts`'s handles, bytes, and wakers.
inline void natives(jsi::Runtime &rt, jsi::Object &n) {
	auto fn = [&](const char *name, unsigned argc, jsi::HostFunctionType f) {
		n.setProperty(rt, name, jsi::Function::createFromHostFunction(rt, jsi::PropNameID::forAscii(rt, name), argc, ::std::move(f)));
	};
	fn("$dispose", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		jsi::Value v(rt, arg(a, n, 0));
		auto c = v.getObject(rt).getNativeState<cell>(rt);
		if (c->closed) {
			return jsi::Value::undefined();
		}
		if (c->owned && c->k->drop != nullptr) {
			::std::int32_t s = 0;
			c->k->drop(c->p, &s);
			status(rt, s, c->k->name);
		} else if (c->h != 1) {
			error(rt, ::std::string(c->k->name) + ": a handle is already borrowed");
		}
		c->closed = true;
		c->release();
		return jsi::Value::undefined();
	});
	fn("$closed", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		return jsi::Value(arg(a, n, 0).getObject(rt).getNativeState<cell>(rt)->closed);
	});
	fn("$bytesFree", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		jsi::Object o = arg(a, n, 0).getObject(rt);
		if (o.hasNativeState<bytes_cell>(rt)) {
			auto b = o.getNativeState<bytes_cell>(rt);
			if (!b->closed) {
				b->closed = true;
				b->release();
			}
		}
		return jsi::Value::undefined();
	});
	fn("$wake", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		auto w = arg(a, n, 0).getObject(rt).getNativeState<waker_cell>(rt);
		if (w->dropped) {
			error(rt, "Waker is closed");
		}
		w->w.wake(w->w.data);
		return jsi::Value::undefined();
	});
	fn("$wakerDrop", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		arg(a, n, 0).getObject(rt).getNativeState<waker_cell>(rt)->drop();
		return jsi::Value::undefined();
	});
	fn("$encode", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		frame f(rt);
		c_buf b = f.utf8(arg(a, n, 0));
		return W().uint8.callAsConstructor(rt, copied(rt, b.ptr, b.len));
	});
	fn("$decode", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, ::std::size_t n) {
		frame f(rt);
		c_buf b = f.array(arg(a, n, 0), "Uint8Array");
		// What is not UTF-8 is replaced, as Rust's `from_utf8_lossy` does.
		::std::string s;
		for (::std::size_t i = 0; i < b.len;) {
			::std::uint8_t c = b.ptr[i];
			::std::size_t len = c < 0x80 ? 1 : (c >> 5) == 6 ? 2 : (c >> 4) == 14 ? 3 : (c >> 3) == 30 ? 4 : 0;
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
