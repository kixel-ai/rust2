"""The API an export renders in each language, as the surface files CI compares with.

usage: surface.py [--check] <export> <dir> [<file>...]

Writes into <dir> one file per language the export generated, or only the files named: the
declarations a caller of that language sees, without bodies, comments, or anything private,
so a change to what a generator renders shows in the file's diff. With --check, it writes
nothing and fails with the diff where <dir> differs.

Each comes from the language's own tool where one reads source: Python's `ast` for the
stub a type checker reads, `go doc -all` for Go, and `tsc` for the declarations of the web
binding, React Native's, and Node's. C, C++, Kotlin, Swift, Dart, and C# have none that runs
without a build, so an outline of the generated source keeps each public declaration up to its body.
"""

import ast
import difflib
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path


class Malformed(Exception):
    """Source the outline cannot read: the generator wrote what this does not expect."""


# Masking: each source as code with comments blanked, and as a mask that also blanks the
# contents of every string, so depth and cut points come from code alone.


def mask(text, lang):
    """`text` with comments blanked, and the same with string contents blanked too."""
    code = list(text)
    masked = list(text)
    i = scan(text, 0, lang, code, masked, None)
    if i != len(text):
        raise Malformed(f"unbalanced `{text[i]}` at offset {i}")
    return "".join(code), "".join(masked)


def blank(chars, start, end):
    for k in range(start, end):
        if chars[k] != "\n":
            chars[k] = " "


def scan(text, i, lang, code, masked, close):
    """Scan code from `i` until `close` at depth zero, or the end; return where it stopped."""
    depth = 0
    n = len(text)
    while i < n:
        c = text[i]
        if text.startswith("//", i):
            end = text.find("\n", i)
            end = n if end < 0 else end
            blank(code, i, end)
            blank(masked, i, end)
            i = end
        elif text.startswith("/*", i):
            end = block_comment(text, i, lang)
            blank(code, i, end)
            blank(masked, i, end)
            i = end
        elif c == '"' or (c == "'" and lang in ("dart", "cpp", "c", "kotlin", "csharp")):
            i = string(text, i, lang, code, masked)
        elif close is not None and c == close and depth == 0:
            return i
        else:
            if c in "({[":
                depth += 1
            elif c in ")}]":
                depth -= 1
                if depth < 0:
                    return i
            i += 1
    return i


def block_comment(text, i, lang):
    """The end of the block comment at `i`, nested where the language nests them."""
    nests = lang in ("kotlin", "swift", "dart")
    depth = 0
    while i < len(text):
        if text.startswith("/*", i):
            depth += 1
            i += 2
        elif text.startswith("*/", i):
            depth -= 1
            i += 2
            if depth == 0 or not nests:
                return i
        else:
            i += 1
    raise Malformed("an unterminated block comment")


def string(text, i, lang, code, masked):
    """Scan the string literal at `i`, masking its contents; return the offset past it."""
    if lang in ("c", "cpp") and i > 0 and (text[i - 1].isalnum() or text[i - 1] == "_") and text[i - 1] == "R":
        raise Malformed("a C++ raw string")
    if lang == "csharp" and (text[max(i - 1, 0):i] == "@" or text[max(i - 2, 0):i] == "@$" or text.startswith('"""', i)):
        raise Malformed("a C# verbatim or raw string")
    quote = text[i]
    raw = lang == "dart" and i > 0 and text[i - 1] == "r"
    triple = lang in ("kotlin", "swift", "dart") and text.startswith(quote * 3, i)
    delim = quote * 3 if triple else quote
    start = i + len(delim)
    i = start
    while i < len(text):
        if text.startswith(delim, i):
            masked[start:i] = ["_" if ch != "\n" else ch for ch in masked[start:i]]
            return i + len(delim)
        c = text[i]
        if c == "\n" and not triple:
            raise Malformed("a string literal runs past its line")
        if c == "\\" and not raw and not (lang == "kotlin" and triple):
            if lang == "swift" and text.startswith("\\(", i):
                i = interpolation(text, i + 2, lang, code, masked, ")")
                continue
            i += 2
            continue
        if c == "$" and lang in ("kotlin", "dart") and not raw and text.startswith("${", i):
            i = interpolation(text, i + 2, lang, code, masked, "}")
            continue
        i += 1
    raise Malformed("an unterminated string literal")


def interpolation(text, i, lang, code, masked, close):
    end = scan(text, i, lang, code, masked, close)
    if end >= len(text) or text[end] != close:
        raise Malformed("an unterminated string interpolation")
    return end + 1


# Cutting a declaration at its body.

ASSIGN = re.compile(r"(?<![=!<>+\-*/%&|^~?:.])=(?![=>])")


def depth0(masked):
    """Each offset of `masked` at bracket depth zero."""
    depth = 0
    for k, c in enumerate(masked):
        if c in "([{":
            if depth == 0:
                yield k
            depth += 1
        elif c in ")]}":
            depth -= 1
            if depth == 0:
                yield k
        elif depth == 0:
            yield k


def first(masked, pattern, start=0):
    """The first match of `pattern` at depth zero in `masked`, from `start`, or None."""
    top = set(depth0(masked))
    for m in pattern.finditer(masked, start):
        if m.start() in top:
            return m.start()
    return None


def template(masked):
    """The end of a C++ declaration's `template <...>` prefix, or 0."""
    m = re.match(r"\s*template\s*<", masked)
    if not m:
        return 0
    depth = 0
    for k in range(m.end() - 1, len(masked)):
        depth += {"<": 1, ">": -1}.get(masked[k], 0)
        if depth == 0:
            return k + 1
    raise Malformed("an unclosed template parameter list")


def friended(masked):
    """A C++ declaration after its `template <...>` prefix, if it is a friend, or None."""
    rest = squash(masked[template(masked):])
    return rest if rest.startswith("friend ") else None


def assignment(masked):
    """Where a declaration's value or expression body starts, if it has one."""
    top = set(depth0(masked))
    for m in ASSIGN.finditer(masked, template(masked)):
        k = m.start()
        if k in top and not re.search(r"\boperator\s*\S*$", masked[:k]):
            return k
    # Dart's expression body, not C++'s `operator<=>`.
    for m in re.finditer(r"(?<!<)=>", masked):
        if m.start() in top:
            return m.start()
    return None


def squash(text):
    text = re.sub(r"\s+", " ", text).strip()
    return re.sub(r",? ([)\]])", r"\1", re.sub(r"([(\[]) ", r"\1", text))


# C: the header is all declarations; only its comments go.


def c_header(text):
    code, _ = mask(text, "c")
    return "".join(line.rstrip() + "\n" for line in code.splitlines() if line.strip())


# C++ and Dart: declarations end at `;` or a body's `}`, at any indentation.

CPP_TYPE = re.compile(r"^(?:template\s*<.*?>\s*)?(class|struct|union|enum|namespace)\b(?!\s*\w+\s*\*)")
DART_TYPE = re.compile(
    r"^(?:@\S+\s+)*(?:(?:abstract|sealed|final|base|interface|mixin)\s+)*(class|mixin|enum|extension)\b")
ACCESS = re.compile(r"^(public|private|protected)$")
# A C# declaration's modifiers, after its attributes, then a type's keyword.
CSHARP_MODIFIERS = re.compile(
    r"^(?:(?:public|internal|private|protected|static|sealed|abstract|unsafe|partial|readonly|ref|new|file|"
    r"override|virtual|extern|const|implicit|explicit|async|volatile)\s+)*")
CSHARP_TYPE = re.compile(r"(class|struct|interface|enum|namespace|record)\b")
ATTRIBUTE = re.compile(r"\[\[.*?\]\]|\b[A-Z][A-Z0-9_]*\([^()]*\)")


def braced(text, lang):
    code, masked = mask(text, lang)
    out = []
    Braced(code, masked, lang, out).scope(0, len(code), 0, "file", True)
    return "".join(line + "\n" for line in prune(out))


class Braced:
    def __init__(self, code, masked, lang, out):
        self.code, self.masked, self.lang, self.out = code, masked, lang, out

    def close(self, i):
        """The `}` matching the `{` at `i`."""
        depth = 0
        for k in range(i, len(self.masked)):
            c = self.masked[k]
            if c in "([{":
                depth += 1
            elif c in ")]}":
                depth -= 1
                if depth == 0:
                    if c != "}":
                        raise Malformed(f"`{self.masked[i]}` closed by `{c}`")
                    return k
        raise Malformed("an unclosed `{`")

    def scope(self, start, end, level, kind, public):
        """Outline the declarations in `[start, end)`, a scope of `kind`."""
        code, masked = self.code, self.masked
        hs = start
        depth = 0
        i = start
        while i < end:
            c = masked[i]
            if c == "#" and not masked[hs:i].strip():
                # A preprocessor line, with its continuations, stands alone.
                j = i
                while True:
                    j = code.find("\n", j)
                    if j < 0 or j >= end:
                        j = end
                        break
                    if code[:j].rstrip().endswith("\\"):
                        j += 1
                        continue
                    break
                line = squash(code[i:j].replace("\\\n", " "))
                # A C# warning's pragma is no API.
                if not (self.lang == "csharp" and line.startswith("#pragma")):
                    self.emit(level, line)
                i = hs = j
                continue
            if c in "([" or c == "{" and depth > 0:
                depth += 1
            elif c in ")]" or c == "}" and depth > 0:
                depth -= 1
            elif depth == 0 and c == ";":
                if public or self.friend(masked[hs:i]):
                    self.declaration(code[hs:i], masked[hs:i], level, kind)
                i = hs = i + 1
                continue
            elif depth == 0 and c == ":" and self.lang == "cpp" and ACCESS.match(masked[hs:i].strip()) \
                    and masked[i + 1] != ":":
                public = masked[hs:i].strip() == "public"
                i = hs = i + 1
                continue
            elif depth == 0 and c == "{":
                j = self.close(i)
                header, hmask = code[hs:i], masked[hs:i]
                if assignment(hmask) is not None and not self.initializers(hmask):
                    # A value, such as a lambda or a set literal, that the `;` ends.
                    i = j + 1
                    continue
                nested = self.nested(hmask)
                if nested:
                    if public:
                        self.typed(header, hmask, nested, i, j, level)
                    i = hs = j + 1
                    # A C++ type's own `;`.
                    while i < end and masked[i].isspace():
                        i += 1
                    if self.lang == "cpp" and i < end and masked[i] == ";":
                        i = hs = i + 1
                    continue
                if public or self.friend(hmask):
                    self.declaration(header, hmask, level, kind, masked[i + 1:j])
                i = hs = j + 1
                continue
            elif c in "}":
                raise Malformed("an unopened `}`")
            i += 1
        if masked[hs:end].strip():
            raise Malformed(f"a declaration without its end: {squash(code[hs:end])[:80]}")

    def initializers(self, hmask):
        """Whether a Dart header is a constructor with an initializer list (`A(x) : _x = x`),
        whose `{` opens a body rather than a value."""
        return self.lang == "dart" and re.search(r"\)\s*:[^;]*$", hmask) is not None

    def friend(self, hmask):
        """Whether a C++ declaration declares a function as a friend, which a caller finds by
        its arguments whatever access section declares it, unlike a friend type or a function
        declared elsewhere, such as `detail::spawn`."""
        if self.lang != "cpp" or not friended(hmask):
            return False
        paren = first(hmask, re.compile(r"\("))
        return paren is not None and re.match(r"friend\s+(?:struct|class|union)\b", friended(hmask)) is None \
            and "::" not in hmask[:paren].split()[-1]

    def nested(self, hmask):
        """The kind of scope a `{` after `hmask` opens, or None for a body."""
        head = squash(hmask)
        if self.lang == "csharp":
            m = CSHARP_TYPE.match(cs_unmodified(head))
            return m and m.group(1)
        if self.lang == "cpp":
            if head.startswith('extern "') or head == "extern _":
                return "namespace"
            m = CPP_TYPE.match(head)
            # A type's attributes, such as `[[nodiscard]]` or a macro's `RUST2_FFI_OWNER(T)`,
            # hold the only parentheses its header may.
            bare = ATTRIBUTE.sub(" ", hmask)
            if m and first(bare, re.compile(r"\(")) is None or (m and m.group(1) == "namespace"):
                return m.group(1)
            return None
        m = DART_TYPE.match(head)
        return m and m.group(1)

    def typed(self, header, hmask, kind, i, j, level):
        head = squash(header)
        if self.lang == "csharp":
            if kind != "namespace" and not cs_visible(squash(hmask)):
                return
            public = True
        elif self.lang == "cpp":
            if kind == "namespace" and re.search(r"\b(detail|)$", head.split("{")[0]):
                if re.fullmatch(r"namespace(\s+\w+)*\s+detail|namespace", head):
                    return
            public = kind in ("struct", "union", "namespace", "enum")
        else:
            if private_dart(head, hmask):
                return
            public = True
        self.emit(level, head + " {")
        start = i + 1
        if kind == "enum":
            # Its enumerators, up to a Dart enum's members.
            body = self.masked[start:j]
            ends = [k for k in depth0(body) if body[k] == ";"]
            stop = start + ends[0] if ends else j
            items = self.masked[start:stop]
            cuts = [start - 1] + [start + k for k in depth0(items) if items[k] == ","] + [stop]
            for a, b in zip(cuts, cuts[1:]):
                if squash(self.code[a + 1:b]):
                    self.emit(level + 1, squash(self.code[a + 1:b]) + ",")
            start = stop + 1 if ends else j
        self.scope(start, j, level + 1, kind, public)
        self.emit(level, "};" if self.lang == "cpp" and kind != "namespace" else "}")

    def declaration(self, header, hmask, level, kind, body=None):
        head = squash(header)
        if not head:
            return
        if self.lang == "csharp":
            if head.startswith("using ") or kind != "interface" and not cs_visible(squash(hmask)):
                return
            cut = assignment(hmask)
            end = len(header) if cut is None or re.search(r"\bconst\b", cs_attributed(squash(hmask))) else cut
            init = first(hmask[:end], re.compile(r"\)\s*:"))
            if init is not None:
                # A constructor's `: base(...)` or `: this(...)`.
                end = init + 1
            line = squash(header[:end])
            # A property, by its accessors: a body of them, or an expression body's getter.
            if not cs_parameters(hmask[:end]) and kind not in ("enum",):
                if body is not None:
                    # Its accessors, at its body's depth zero, each with the modifiers that keep it
                    # visible outside the assembly.
                    top = set(depth0(body))
                    flat = "".join(c if k in top else " " for k, c in enumerate(body))
                    # An accessor's expression body runs to its `;`, so a `set` inside it is no accessor.
                    flat = re.sub(r"=>[^;]*", lambda m: "=>" + " " * (len(m.group()) - 2), flat)
                    accessors = [" ".join(v.split() + [a]) for v, a in re.findall(
                        r"(?:^|[;}\]\s])((?:(?:public|private|protected|internal)\s+)*)(get|set|init)\s*(?=[;{]|=>)", flat)
                        if "private" not in v.split() and v.split() != ["internal"]]
                    if not accessors:
                        raise Malformed(f"a C# body that is no type, method, or property: {line[:80]}")
                    line += " { " + " ".join(a + ";" for a in accessors) + " }"
                    self.emit(level, line)
                    return
                if cut is not None and hmask[cut:cut + 2] == "=>":
                    self.emit(level, line + " { get; }")
                    return
            self.emit(level, line + ";")
            return
        if self.lang == "cpp":
            if (friended(hmask) and not self.friend(hmask)) or head.startswith("static_assert"):
                return
            if kind == "namespace" and qualified(hmask):
                return
            cut = assignment(hmask)
            # A constant's value, which C's header spells too, and what an alias names.
            constant = "constexpr" in head and first(hmask, re.compile(r"\(")) is None or \
                re.match(r"(?:template\s*<.*?>\s*)?using\b", head) is not None
            if cut is not None and squash(hmask[cut + 1:]) not in ("default", "delete", "0") and not constant:
                header = header[:cut]
            elif cut is None:
                init = initializer(hmask)
                if init is not None:
                    header = header[:init]
        else:
            if head.startswith("import "):
                return
            cut = assignment(hmask)
            end = len(header) if cut is None else cut
            init = constructor_initializer(hmask[:end])
            if init is not None:
                end = init
            if private_dart(squash(header[:end]), hmask[:end]):
                return
            if cut is not None and end == cut and head.startswith("typedef "):
                end = len(header)
            elif cut is not None and end == cut and re.fullmatch(
                    r"(?:static )?(?:late )?(?:final|const|var) [\w$]+ ?", squash(unannotated(hmask[:cut]))):
                # A value whose type is inferred: the value is all it declares of its type.
                end = len(header)
            header = header[:end]
        self.emit(level, squash(header) + ";")

    def emit(self, level, line):
        self.out.append("\t" * level + line)


def cs_attributed(head):
    """A C# declaration's masked head without its attributes, each to its balanced `]`."""
    i = 0
    while head.startswith("[", i):
        depth = 0
        for k in range(i, len(head)):
            depth += {"[": 1, "]": -1}.get(head[k], 0)
            if depth == 0:
                break
        else:
            raise Malformed(f"an unclosed C# attribute: {head[:80]}")
        i = k + 1
        while i < len(head) and head[i].isspace():
            i += 1
    return head[i:]


def cs_unmodified(head):
    """A C# declaration's head without its attributes and modifiers."""
    head = cs_attributed(head)
    return head[CSHARP_MODIFIERS.match(head).end():]


def cs_parameters(hmask):
    """Whether a C# member's header has a parameter list: a `(` at depth zero right after its
    name, type parameters, or operator, where a tuple type's follows a space, `<`, or `,`."""
    top = set(depth0(hmask))
    return any(c == "(" and k in top and k > 0 and (
        hmask[k - 1].isalnum() or hmask[k - 1] in "_>" or re.search(r"\boperator\s*\S+$", hmask[:k]))
        for k, c in enumerate(hmask))


def cs_visible(head):
    """Whether a C# declaration's modifiers make it visible outside its assembly."""
    head = cs_attributed(head)
    modifiers = head[:CSHARP_MODIFIERS.match(head).end()]
    # `private protected` reaches only subclasses in its own assembly.
    return re.search(r"\b(public|protected)\b", modifiers) is not None and re.search(r"\bprivate\b", modifiers) is None


def qualified(hmask):
    """Whether a namespace-level declaration defines a member declared in its class."""
    paren = first(hmask, re.compile(r"\("))
    if paren is None:
        return False
    before = hmask[:paren].rstrip()
    m = re.search(r"([\w:~]+(?:<[^<>]*>)?(?:::[\w~]+)*|operator\s*\S+)$", before)
    name = m.group(1) if m else ""
    return "::" in name.lstrip(":") and not name.startswith("operator")


def initializer(hmask):
    """Where a C++ constructor's member initializer list starts, after its parameters."""
    top = set(depth0(hmask))
    close = hmask.rfind(")")
    for m in re.finditer(r"(?<!:):(?!:)", hmask):
        if m.start() in top and m.start() > close >= 0:
            return m.start()
    return None


def constructor_initializer(hmask):
    """Where a Dart constructor's initializer list starts."""
    for m in re.finditer(r"\s:\s", hmask):
        if m.start() in set(depth0(hmask)) and ")" in hmask[:m.start()]:
            return m.start()
    return None


def private_dart(head, hmask):
    """Whether a Dart declaration's name is library-private."""
    hmask = hmask[annotations(hmask):]
    head = unannotated(head)
    parens = [k for k in depth0(hmask) if hmask[k] == "("]
    paren = parens[-1] if parens else None
    m = DART_TYPE.match(head)
    if m:
        name = re.match(r"\s*(?:type\s+)?([\w$]+)?", head[m.end():]).group(1) or ""
        if m.group(1) == "extension" and name == "on":
            # An unnamed extension applies only in its own library.
            return True
    elif paren is not None:
        if re.search(r"operator\s*\S+\s*$", hmask[:paren]):
            return False
        name = last_name(hmask[:paren])
    else:
        cut = assignment(hmask)
        name = last_name(hmask[:cut] if cut is not None else hmask)
    return name.startswith("_")


def last_name(text):
    """The identifier `text` ends with, past any type arguments after it."""
    text = text.rstrip()
    if text.endswith(">"):
        depth = 0
        for k in range(len(text) - 1, -1, -1):
            depth += {">": 1, "<": -1}.get(text[k], 0)
            if depth == 0:
                text = text[:k].rstrip()
                break
    m = re.search(r"[\w$]+$", text)
    return m.group(0) if m else ""


# Kotlin and Swift: declarations end at a line, so each is a line and those indented past it.

KOTLIN_TYPE = re.compile(r"\b(class|interface|object)\b")
SWIFT_TYPE = re.compile(r"^(?:@\S+\s+)*(?:[\w()]+\s+)*?(class|struct|enum|protocol|extension|actor)\b")
SWIFT_PUBLIC = re.compile(r"\b(public|open)\b")
KOTLIN_HIDDEN = re.compile(r"\b(private|internal|protected)\b")
ANNOTATION = re.compile(r"^@[\w.:]+(?:\(.*\))?$")


def indented(text, lang):
    code, masked = mask(text, lang)
    lines = [(c, m) for c, m in zip(code.split("\n"), masked.split("\n")) if c.strip()]
    out = []
    Indented(lang, out).scope(lines, 0, "file", False)
    out = prune(out)
    # Kotlin's generated file names types through private aliases, such as `_Mark` for
    # `api.Mark`: a caller sees the type.
    aliases = dict((alias, path) for path, alias in re.findall(r"^import\s+([\w.]+)\s+as\s+(\w+)\s*$", code, re.M))
    if aliases:
        names = re.compile(r"(?<![\w.])(" + "|".join(map(re.escape, sorted(aliases, key=len, reverse=True))) + r")\b")
        out = [names.sub(lambda m: aliases[m.group(1)], line) for line in out]
    return "".join(line + "\n" for line in out)


def indent(line):
    return len(line) - len(line.lstrip())


class Indented:
    def __init__(self, lang, out):
        self.lang, self.out = lang, out

    def members(self, lines):
        """Split `lines` into members: a line, and those after it indented past it or in its brackets."""
        members = []
        k = 0
        pending = []
        while k < len(lines):
            base = indent(lines[k][0])
            start = k
            depth = 0
            while True:
                depth += bracket_depth(lines[k][1])
                k += 1
                if k >= len(lines):
                    break
                if depth > 0 or indent(lines[k][0]) > base:
                    continue
                if depth == 0 and indent(lines[k][0]) == base and lines[k][1].lstrip()[:1] in ")]}" \
                        and lines[k][1].strip()[:1] and not lines[start][1].strip().startswith("#"):
                    continue
                break
            if depth != 0:
                raise Malformed(f"unbalanced brackets in: {squash(lines[start][0])[:80]}")
            member = lines[start:k]
            if len(member) == 1 and ANNOTATION.match(member[0][0].strip()):
                pending.extend(member)
                continue
            members.append(pending + member)
            pending = []
        if pending:
            raise Malformed(f"an annotation on nothing: {pending[0][0].strip()}")
        return members

    def scope(self, lines, level, kind, public_ext):
        for member in self.members(lines):
            self.member(member, level, kind, public_ext)

    def member(self, member, level, kind, public_ext):
        code = "\n".join(c for c, _ in member)
        masked = "\n".join(m for _, m in member)
        head = squash(masked)
        if head.startswith("#"):
            self.emit(level, squash(code))
            return
        if re.match(r"(import|package)\b", head) and self.lang == "kotlin" and head.startswith("import"):
            return
        if re.match(r"(@_?\w+\s+)*import\b", head):
            if self.lang == "swift" and "@_exported" not in head:
                return
            if self.lang == "kotlin":
                return
        if head.startswith("@file:"):
            return
        brace = first(masked, re.compile(r"\{"))
        cut = assignment(masked)
        if cut is not None and (brace is None or cut < brace):
            brace = None
        nested = self.nested(masked[:brace] if brace is not None else masked)
        if not self.visible(squash(masked[:brace] if brace is not None else masked), kind, public_ext, nested):
            return
        if nested and brace is not None:
            header = code[:brace]
            # The body: the lines between the one opening it and its own closing line.
            line = masked.count("\n", 0, brace)
            after = masked[brace + 1:].split("\n", 1)[0]
            body = member[line + 1:]
            if not after.strip() and body and squash(body[-1][1]) == "}":
                body = body[:-1]
            elif body or "}" not in after:
                raise Malformed(f"a type body not on lines of its own: {squash(code)[:80]}")
            else:
                # A body on its header's line, such as `fun interface Waker { fun wake() }`.
                close = masked.rindex("}")
                inner = (code[brace + 1:close], masked[brace + 1:close])
                body = [inner] if inner[1].strip() else []
            out_start = len(self.out)
            self.emit(level, squash(header) + " {")
            ext_public = nested == "extension" and SWIFT_PUBLIC.search(squash(header)) is not None
            self.scope(body, level + 1, nested, ext_public)
            if self.lang == "swift" and nested == "extension" and len(self.out) == out_start + 1 \
                    and ":" not in squash(masked[:brace]):
                # An extension adding nothing public and conforming to nothing.
                del self.out[out_start:]
                return
            if len(self.out) == out_start + 1:
                self.out[-1] = self.out[-1][:-2] + " {}"
            else:
                self.emit(level, "}")
            return
        end = len(masked)
        alias = re.match(r"(?:\w+ )*typealias\b", squash(unannotated(masked)))
        if cut is not None and not alias and not re.fullmatch(r"(?:\w+ )*(?:val|var|let) [\w`]+ ?", squash(unannotated(masked[:cut]))):
            # A value or expression body, unless it is all a property declares of its type.
            end = cut
        if brace is not None and brace < end:
            # A body, unless it is a protocol's accessor list, which is the declaration.
            accessors = masked[brace + 1:masked.rfind("}")]
            if kind == "protocol" and re.fullmatch(r"\s*(get|set|async|throws|\s)*", accessors):
                end = len(masked)
            else:
                end = brace
        if self.lang == "kotlin":
            getter = first(masked[:end], re.compile(r"\b[gs]et\("))
            if getter is not None and re.match(r"(?:\w+\s+)*(?:val|var)\b", unannotated(head)):
                end = getter
            delegated = re.match(r"^((?:@\S+\s+)*(?:\w+\s+)*)constructor\b", head)
            if delegated:
                init = first(masked[:end], re.compile(r"\)\s*:"))
                if init is not None:
                    end = init + 1
            if re.match(r"(?:\w+\s+)*init$", squash(masked[:end])):
                return
        if self.lang == "swift" and re.match(r"(?:@\S+\s+)*deinit\b", head):
            return
        line = squash(code[:end])
        if self.lang == "swift" and brace is not None and end == brace and \
                re.match(r"(?:\w+(?:\(\w+\))? )*(var|subscript)\b", unannotated(head)):
            # A computed property's accessors: whether it sets is its API, its getter implied.
            base = indent(member[0][0])
            sets = any(indent(c) == base + 1 and re.match(r"(?:\w+ )*(set|_modify)\b", m.strip())
                       for c, m in member[1:])
            line += " { get set }" if sets else " { get }"
        self.emit(level, line)

    def nested(self, hmask):
        head = unannotated(squash(hmask))
        if self.lang == "kotlin":
            words = re.sub(r"\(.*", "", head)
            words = re.sub(r"@\S+\s+", "", words)
            m = KOTLIN_TYPE.search(words)
            if m and not re.match(r"(?:\w+\s+)*(fun(?! interface)|val|var)\b", words):
                return "enum" if re.search(r"\benum\s+class\b", words) else m.group(1)
            return None
        m = SWIFT_TYPE.match(re.sub(r"\(.*", "", head))
        return m and m.group(1)

    def visible(self, head, kind, public_ext, nested):
        head = unannotated(head)
        if self.lang == "kotlin":
            # The modifiers before its keyword, not a primary constructor's own.
            modifiers = re.split(r"\b(?:class|interface|object|fun|val|var|constructor|typealias|init)\b", head)[0]
            return KOTLIN_HIDDEN.search(modifiers) is None
        # Its modifiers, such as `nonisolated(nonsending) public`, before its keyword.
        modifiers = re.split(r"\b(?:func|var|let|init|subscript|case|struct|enum|protocol|extension|actor|"
                             r"typealias|associatedtype|deinit)\b|\bclass\b(?!\s+(?:func|var)\b)", head)[0]
        if kind == "protocol" or public_ext:
            return not re.search(r"\b(private|fileprivate|internal)\b(?!\(set\))", modifiers)
        if kind == "enum" and head.startswith("case "):
            return True
        if nested == "extension":
            return True
        return SWIFT_PUBLIC.search(modifiers) is not None

    def emit(self, level, line):
        self.out.append("\t" * level + line)


def unannotated(head):
    """`head` without its leading annotations and attributes, arguments included."""
    return head[annotations(head):].lstrip()


def annotations(text):
    """The offset past the annotations, with their type and value arguments, that start `text`."""
    k = len(text) - len(text.lstrip())
    while True:
        m = re.compile(r"@[\w.:$]+").match(text, k)
        if not m:
            return k
        k = m.end()
        for open_, close in (("<", ">"), ("(", ")")):
            if text.startswith(open_, k):
                depth = 0
                for j in range(k, len(text)):
                    depth += {open_: 1, close: -1}.get(text[j], 0)
                    if depth == 0:
                        k = j + 1
                        break
        k += len(text[k:]) - len(text[k:].lstrip())


def bracket_depth(masked):
    depth = 0
    for c in masked:
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
    return depth


def prune(lines):
    """Drop conditional groups left empty once their contents went."""
    changed = True
    while changed:
        changed = False
        for k in range(len(lines) - 1):
            a, b = lines[k].strip(), lines[k + 1].strip()
            if re.match(r"#if\b", a) and b.startswith("#endif") or re.match(r"#(else|elif|elseif)\b", a) and (
                    b.startswith("#endif") or re.match(r"#(else|elif|elseif)\b", b)):
                drop = 2 if a.startswith("#if") else 1
                del lines[k:k + drop]
                changed = True
                break
    return lines


# Python: the stub a type checker reads, from the module's own syntax tree.


def python_stub(text):
    tree = ast.parse(text)
    # A private class a public alias names, such as `Event.Moved` for `_Event_Moved`, is
    # public through it.
    aliased = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.AnnAssign) and alias(node):
            value = node.value
            if isinstance(value, ast.Constant) and isinstance(value.value, str):
                aliased.add(value.value)
            elif isinstance(value, ast.Name):
                aliased.add(value.id)
    out = []
    stub(tree.body, 0, out, aliased)
    return "".join(line + "\n" for line in out)


def alias(node):
    return node.value is not None and ast.unparse(node.annotation).endswith("TypeAlias")


def public_name(name):
    # `_0` is a tuple payload's field.
    return not name.startswith("_") or (name.startswith("__") and name.endswith("__")) or \
        re.fullmatch(r"_\d+", name) is not None


def stub(body, level, out, aliased):
    pad = "    " * level
    for node in body:
        if isinstance(node, ast.If):
            test = ast.unparse(node.test)
            if test.endswith("TYPE_CHECKING") and not test.startswith("not "):
                stub(node.body, level, out, aliased)
                stub(node.orelse, level, out, aliased)
            elif test.startswith("not ") and test.endswith("TYPE_CHECKING"):
                stub(node.orelse, level, out, aliased)
            elif any(isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef))
                     for b in (node.body, node.orelse) for n in b):
                raise Malformed(f"a declaration under `if {test}`")
        elif isinstance(node, ast.ClassDef):
            if not public_name(node.name) and node.name not in aliased:
                continue
            for d in node.decorator_list:
                out.append(f"{pad}@{ast.unparse(d)}")
            bases = [ast.unparse(b) for b in node.bases] + [ast.unparse(k) for k in node.keywords]
            out.append(f"{pad}class {node.name}{'[' + ', '.join(ast.unparse(p) for p in node.type_params) + ']' if node.type_params else ''}"
                       f"{'(' + ', '.join(bases) + ')' if bases else ''}:")
            mark = len(out)
            stub(node.body, level + 1, out, aliased)
            if len(out) == mark:
                out.append(f"{pad}    ...")
        elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            if not public_name(node.name):
                continue
            for d in node.decorator_list:
                out.append(f"{pad}@{ast.unparse(d)}")
            header = ast.unparse(ast.FunctionDef(
                name=node.name, args=node.args, body=[ast.Expr(ast.Constant(...))],
                decorator_list=[], returns=node.returns, type_params=node.type_params, lineno=0)).split("\n")[0]
            prefix = "async " if isinstance(node, ast.AsyncFunctionDef) else ""
            out.append(f"{pad}{prefix}{header} ...")
        elif isinstance(node, ast.AnnAssign):
            if isinstance(node.target, ast.Name) and public_name(node.target.id):
                value = f" = {ast.unparse(node.value)}" if alias(node) else ""
                out.append(f"{pad}{ast.unparse(node.target)}: {ast.unparse(node.annotation)}{value}")
        elif isinstance(node, ast.Assign):
            names = [t.id for t in node.targets if isinstance(t, ast.Name)]
            if names and len(names) == len(node.targets) and all(public_name(n) for n in names) \
                    and not all(n.startswith("__") for n in names):
                out.append(f"{pad}{ast.unparse(node)}")
            elif names == ["__all__"]:
                out.append(f"{pad}{ast.unparse(node)}")
        elif isinstance(node, ast.TypeAlias):
            if public_name(node.name.id):
                out.append(f"{pad}{ast.unparse(node)}")


# Go: `go doc -all` of each package, without its documentation.


def go_doc(root):
    packages = sorted({p.parent for p in root.rglob("*.go")})
    parts = []
    for package in packages:
        rel = package.relative_to(root).as_posix()
        result = subprocess.run(["go", "doc", "-all", f"./{rel}"], cwd=root, capture_output=True, text=True,
                                env={**os.environ, "GOFLAGS": "-mod=mod", "GOWORK": "off"})
        if result.returncode != 0:
            raise Malformed(f"go doc ./{rel}: {result.stderr.strip()}")
        parts.append(undocumented(result.stdout))
    return "".join(parts)


def undocumented(text):
    """`go doc -all` output with its prose gone: the package's, and each declaration's."""
    lines = text.split("\n")
    out = [lines[0]]
    section = False
    for line in lines[1:]:
        if re.fullmatch(r"(CONSTANTS|VARIABLES|FUNCTIONS|TYPES)", line):
            section = True
            out.append("")
            out.append(line)
            continue
        if not section or not line.strip() or line.startswith("    "):
            continue
        if line.lstrip().startswith("//"):
            continue
        out.append(line)
    return "\n".join(out).strip("\n") + "\n\n"


# The web binding, React Native's, and Node's: tsc's declarations of their modules.

# What React Native's runtime imports of `react-native`, off a React Native app.
REACT_NATIVE = """declare module "react-native" {
  export type TurboModule = object;
  export const TurboModuleRegistry: { getEnforcing<T>(name: string): T };
}
"""


def web_declarations(root):
    sources = sorted(p for p in root.glob("*.ts") if not p.name.endswith(".d.ts"))
    with tempfile.TemporaryDirectory(dir=root.parent) as tmp:
        ambient = Path(tmp) / "ambient" / "react-native.d.ts"
        ambient.parent.mkdir()
        ambient.write_text(REACT_NATIVE, encoding="utf-8")
        result = subprocess.run(
            ["tsc", "--declaration", "--emitDeclarationOnly", "--removeComments", "--strict", "--skipLibCheck",
             "--target", "es2024", "--lib", "es2024,esnext.disposable,dom", "--module", "esnext",
             "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--types", "",
             "--rootDir", str(root), "--outDir", tmp, str(ambient), *map(str, sources)],
            capture_output=True, text=True)
        if result.returncode != 0:
            raise Malformed(f"tsc: {result.stdout}{result.stderr}")
        parts = []
        for source in sources:
            declared = (Path(tmp) / f"{source.stem}.d.ts").read_text(encoding="utf-8")
            parts.append(f"// {source.name}\n{declared.rstrip()}\n")
        return "\n".join(parts)


# Each language's file, from the export's layout.


def files(export):
    """The surface files of `export`, by name, each rendered when called."""
    surfaces = {}
    include = export / "include"
    headers = sorted(include.glob("*.h"))
    if headers:
        surfaces["c.h"] = lambda: joined(headers, c_header, "/* {} */")
    hpp = sorted(include.glob("*/*.hpp"))
    if hpp:
        surfaces["cpp.hpp"] = lambda: joined(hpp, lambda t: braced(t, "cpp"), "// {}")
    for lang, glob, name, outline, label in [
        ("kotlin", "kotlin/*.kt", "kotlin.kt", lambda t: indented(t, "kotlin"), "// {}"),
        ("swift", "swift/*.swift", "swift.swift", lambda t: indented(t, "swift"), "// {}"),
        ("dart", "dart/lib/*.dart", "dart.dart", lambda t: braced(t, "dart"), "// {}"),
        ("python", "python/*.py", "python.pyi", python_stub, "# {}"),
        ("csharp", "csharp/*.cs", "csharp.cs", lambda t: braced(t, "csharp"), "// {}"),
    ]:
        sources = sorted(export.glob(glob))
        if sources:
            surfaces[name] = lambda sources=sources, outline=outline, label=label: joined(sources, outline, label)
    if (export / "go" / "go.mod").exists():
        surfaces["go.txt"] = lambda: go_doc(export / "go")
    if (export / "web").is_dir():
        surfaces["web.d.ts"] = lambda: web_declarations(export / "web")
    if (export / "react-native").is_dir():
        surfaces["react-native.d.ts"] = lambda: web_declarations(export / "react-native")
    if (export / "node").is_dir():
        surfaces["node.d.ts"] = lambda: web_declarations(export / "node")
    return surfaces


def joined(sources, outline, label):
    parts = []
    for source in sources:
        try:
            text = outline(source.read_text(encoding="utf-8"))
        except Malformed as err:
            raise SystemExit(f"{source}: {err}") from None
        parts.append(label.format(source.name) + "\n" + text)
    return "\n".join(parts)


def main():
    args = [a for a in sys.argv[1:] if a != "--check"]
    check = "--check" in sys.argv[1:]
    if len(args) < 2:
        raise SystemExit(__doc__.split("\n\n")[1])
    export, out, only = Path(args[0]), Path(args[1]), set(args[2:])
    surfaces = files(export)
    if only:
        if only - set(surfaces):
            raise SystemExit(f"{export} renders no {', '.join(sorted(only - set(surfaces)))}")
        surfaces = {name: render for name, render in surfaces.items() if name in only}
    if not surfaces:
        raise SystemExit(f"{export} holds no binding")
    # Only the files asked for render, so another's malformed source fails no check of these.
    surfaces = {name: render() for name, render in surfaces.items()}
    existing = {p.name for p in out.iterdir()} if out.is_dir() else set()
    # Files named leave the others alone, which another export may render.
    stale = [] if only else sorted(existing - set(surfaces))
    if check:
        failed = False
        for name, text in sorted(surfaces.items()):
            path = out / name
            old = path.read_text(encoding="utf-8") if path.exists() else ""
            if old != text:
                failed = True
                sys.stdout.writelines(difflib.unified_diff(
                    old.splitlines(keepends=True), text.splitlines(keepends=True), str(path), f"{path} (rendered)"))
        for name in stale:
            failed = True
            print(f"{out / name}: no language renders it")
        if failed:
            raise SystemExit(f"{out} is not the API the generators render: run `just ffi surface` and commit it")
        return
    out.mkdir(parents=True, exist_ok=True)
    for name in stale:
        (out / name).unlink()
    for name, text in surfaces.items():
        (out / name).write_text(text, encoding="utf-8")


if __name__ == "__main__":
    main()
