"""The outlines' edge cases, on source shaped as the generators write it."""

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import surface  # noqa: E402


def lines(text):
    return text.strip("\n").split("\n")


class Kotlin(unittest.TestCase):
    def outline(self, text):
        return lines(surface.indented(text, "kotlin"))

    def test_bodies_values_and_hidden_members_go(self):
        source = """
import api.Mark as _Mark
/** Docs. */
class Counter internal constructor(internal val raw: Long, val size: Int) : Base() {
	init {
		kotlin.require(size > 0) { "${size} is not { positive" }
	}

	constructor(size: Int) : this(0L, size.let { it })

	fun mark(at: _Mark = _Mark.ZERO): String = "${at}}"

	override fun equals(other: Any?): Boolean =
		other is Counter && raw == other.raw

	internal fun hidden() {}
	@PublishedApi internal val also: Int get() = 1
	val held: Any? get() = raw

	companion object {
		val ZERO = Counter(1)
		private const val MAX = 2
	}
}
fun interface Waker { fun wake() }
typealias Handler = (Int) -> Unit
"""
        self.assertEqual(self.outline(source), [
            "class Counter internal constructor(internal val raw: Long, val size: Int) : Base() {",
            "\tconstructor(size: Int)",
            "\tfun mark(at: api.Mark = api.Mark.ZERO): String",
            "\toverride fun equals(other: Any?): Boolean",
            "\tval held: Any?",
            "\tcompanion object {",
            "\t\tval ZERO = Counter(1)",
            "\t}",
            "}",
            "fun interface Waker {",
            "\tfun wake()",
            "}",
            "typealias Handler = (Int) -> Unit",
        ])

    def test_multiline_header_and_enum(self):
        source = """
data class Parcel(
	val mark: Int,
	val payload: String,
) {
	fun total(): Long {
		return 1
	}
}
enum class Bound {
	UNBOUNDED,
	INCLUDED,
}
"""
        self.assertEqual(self.outline(source), [
            "data class Parcel(val mark: Int, val payload: String) {",
            "\tfun total(): Long",
            "}",
            "enum class Bound {",
            "\tUNBOUNDED,",
            "\tINCLUDED,",
            "}",
        ])


class Swift(unittest.TestCase):
    def outline(self, text):
        return lines(surface.indented(text, "swift"))

    def test_access_accessors_and_conditions(self):
        source = """
import Foundation
#if compiler(>=6.4)
extension C: Equatable {}
#endif
#if canImport(FoundationEssentials)
import FoundationEssentials
#endif
@usableFromInline enum Native {
	static func check() {}
}
public struct Bytes: Hashable {
	@usableFromInline var storage: Data
	@inlinable public var count: Int {
		storage.count
	}
	@inlinable public subscript(i: Int) -> UInt8 {
		get { storage[i] }
		set { storage[i] = newValue }
	}
	@inlinable public init(_ s: String) {
		self.storage = Data("\\(s) }".utf8)
	}
	deinit {}
}
public protocol Reader {
	var position: Int { get }
	mutating func read() throws -> Int
}
public enum Bound {
	case unbounded
	case included(Int)
}
public typealias Count = Swift.UInt64
@inlinable nonisolated(nonsending) public func later() async throws -> Int {
	0
}
extension Bound {
	func hidden() {}
}
"""
        self.assertEqual(self.outline(source), [
            "#if compiler(>=6.4)",
            "extension C: Equatable {}",
            "#endif",
            "public struct Bytes: Hashable {",
            "\t@inlinable public var count: Int { get }",
            "\t@inlinable public subscript(i: Int) -> UInt8 { get set }",
            "\t@inlinable public init(_ s: String)",
            "}",
            "public protocol Reader {",
            "\tvar position: Int { get }",
            "\tmutating func read() throws -> Int",
            "}",
            "public enum Bound {",
            "\tcase unbounded",
            "\tcase included(Int)",
            "}",
            "public typealias Count = Swift.UInt64",
            "@inlinable nonisolated(nonsending) public func later() async throws -> Int",
        ])


class Cpp(unittest.TestCase):
    def outline(self, text):
        return lines(surface.braced(text, "cpp"))

    def test_access_details_and_definitions(self):
        source = """
#pragma once
namespace api {
/// A shape.
enum class Shape : int {
	circle = 1,
	square = 4,
};
inline constexpr ::std::uint64_t LIMIT = API_LIMIT;
using Choice = ::std::variant<Choice_Empty, Choice_Picked>;
class Marker {
	::api::detail::unique raw_;
	explicit Marker(decltype(raw_) raw) noexcept : raw_(::std::move(raw)) {}
	friend struct ::api::detail::access;
	friend bool operator!=(const Marker &a, const Marker &b) { return !(a == b); }
	template <class F> friend bool operator<=(const Marker &a, F b) { return true; }
	template <class F> friend void ::api::detail::spawn(Marker &, F);

public:
	friend bool operator<(const Marker &a, const Marker &b);
	Marker(Marker &&other) noexcept = default;
	Marker &operator=(Marker &&other) noexcept = default;
	static ::api::Marker new_(const char *s = "{;}");
	template <class F, class = ::std::enable_if_t<true>>
	void each(F f) const { f(1); }
};
namespace detail {
struct access {};
} // namespace detail
inline ::api::Marker Marker::new_(const char *s) {
	return {};
}
inline bool operator==(const Marker &a, const Marker &b) { return true; }
} // namespace api
"""
        self.assertEqual(self.outline(source), [
            "#pragma once",
            "namespace api {",
            "\tenum class Shape : int {",
            "\t\tcircle = 1,",
            "\t\tsquare = 4,",
            "\t};",
            "\tinline constexpr ::std::uint64_t LIMIT = API_LIMIT;",
            "\tusing Choice = ::std::variant<Choice_Empty, Choice_Picked>;",
            "\tclass Marker {",
            "\t\tfriend bool operator!=(const Marker &a, const Marker &b);",
            "\t\ttemplate <class F> friend bool operator<=(const Marker &a, F b);",
            "\t\tfriend bool operator<(const Marker &a, const Marker &b);",
            "\t\tMarker(Marker &&other) noexcept = default;",
            "\t\tMarker &operator=(Marker &&other) noexcept = default;",
            '\t\tstatic ::api::Marker new_(const char *s = "{;}");',
            "\t\ttemplate <class F, class = ::std::enable_if_t<true>> void each(F f) const;",
            "\t};",
            "\tinline bool operator==(const Marker &a, const Marker &b);",
            "}",
        ])

    def test_attributed_type(self):
        source = """
template <class T>
class [[nodiscard]] API_POINTER(T) span {
public:
	const T *data() const noexcept { return nullptr; }
};
"""
        self.assertEqual(self.outline(source), [
            "template <class T> class [[nodiscard]] API_POINTER(T) span {",
            "\tconst T *data() const noexcept;",
            "};",
        ])


class Dart(unittest.TestCase):
    def outline(self, text):
        return lines(surface.braced(text, "dart"))

    def test_private_names_and_bodies(self):
        source = """
import 'dart:ffi' as ffi;
const _file = 'api.dart';
final _ownerFree = ffi.Native.addressOf<ffi.NativeFunction<ffi.Void Function()>>(_free);
final limit = U64(5);
typedef Callback = void Function(int);
@ffi.Native<ffi.Uint64 Function()>(symbol: 'api_contract')
external int _contract();
(int, int)? _pair(int c) => null;
extension on _Arena {
	void x() {}
}
enum SeekFrom {
	start(0), end(2);
	const SeekFrom(this._c);
	final int _c;
	String debug() => '${_c}';
}
final class Duration implements Comparable<Duration> {
	final int secs;
	Duration(this.secs) : assert(secs >= 0);
	factory Duration.from(int s) {
		return Duration(s);
	}
	Duration._(this.secs);
	int get hashCode => secs;
	static const zero = Duration(0);
	final _set = {1, 2};
}
"""
        self.assertEqual(self.outline(source), [
            "final limit = U64(5);",
            "typedef Callback = void Function(int);",
            "enum SeekFrom {",
            "\tstart(0),",
            "\tend(2),",
            "\tconst SeekFrom(this._c);",
            "\tString debug();",
            "}",
            "final class Duration implements Comparable<Duration> {",
            "\tfinal int secs;",
            "\tDuration(this.secs);",
            "\tfactory Duration.from(int s);",
            "\tint get hashCode;",
            "\tstatic const zero = Duration(0);",
            "}",
        ])


class C(unittest.TestCase):
    def test_comments_go_strings_stay(self):
        source = '/* Docs. */\n#define API_NAME "/* not a comment */"\n\n// Line.\nvoid api_f(void); /* trailing */\n'
        self.assertEqual(lines(surface.c_header(source)), [
            '#define API_NAME "/* not a comment */"',
            "void api_f(void);",
        ])


class Python(unittest.TestCase):
    def test_type_checking_view(self):
        source = '''
"""Docs."""
import typing as _typing
_T = _typing.TypeVar("_T")
LIMIT = 5
if _typing.TYPE_CHECKING:
    def marked(n: int) -> int:
        """Docs."""
        ...
if not _typing.TYPE_CHECKING:
    def marked(n):
        return n
class Point:
    """Docs."""
    x: int
    def __eq__(self, other: object) -> bool:
        return True
    def _hidden(self) -> None: ...
    async def later(self, n: int = 0) -> int:
        return n
class Event:
    if _typing.TYPE_CHECKING:
        Moved: _typing.TypeAlias = "_Event_Moved"
class _Event_Moved(Event):
    _0: Point
class _Hidden:
    x: int
if _n.contract() != 1:
    raise ImportError("mismatch")
'''
        self.assertEqual(lines(surface.python_stub(source)), [
            "LIMIT = 5",
            "def marked(n: int) -> int: ...",
            "class Point:",
            "    x: int",
            "    def __eq__(self, other: object) -> bool: ...",
            "    async def later(self, n: int=0) -> int: ...",
            "class Event:",
            "    Moved: _typing.TypeAlias = '_Event_Moved'",
            "class _Event_Moved(Event):",
            "    _0: Point",
        ])


class Malformed(unittest.TestCase):
    def test_refuses_what_it_cannot_read(self):
        for text, lang in [
            ('fun f() = "open\n', "kotlin"),
            ("class A {\n", "dart"),
            ("namespace api {\nint f();\n", "cpp"),
            ("int f(int a;\n", "cpp"),
        ]:
            with self.subTest(text=text), self.assertRaises(surface.Malformed):
                (surface.braced if lang in ("dart", "cpp") else lambda t, l: surface.indented(t, l))(text, lang)


if __name__ == "__main__":
    unittest.main()
