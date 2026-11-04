#!/usr/bin/env python3
"""Rewrite a fresh translation of the codec and task fixtures into one prototype model of
a `String`, for `just ts bench --route <model>=<dir>` to time against main. Prototype only,
for moq-dev/rust2#574's decision.

usage: str-routes.py <dir> js|utf8

<dir> holds `translated/` and `task/` as `just ts run` wrote them. Each edit is what the
translator would emit for that model on the paths the `str/` and `fmt/` workloads run;
every edit must match as often as it says.

- js: a `String` as `rs.PString`, a JS string with its UTF-8 length and capacity; a `&str`
  stays a JS string.
- utf8: a `String` as an `rs.ByteVec`, a `&str` as an `rs.ByteSlice`, each literal's bytes
  made once where its module loads.
"""
import re
import shutil
import sys
from pathlib import Path

root, model = Path(sys.argv[1]), sys.argv[2]
here = Path(__file__).parent
utf8 = model == "utf8"
assert model in ("js", "utf8"), model


def edit(path, pairs):
    p = root / "translated" / path
    text = p.read_text()
    for old, new, *n in pairs:
        want = n[0] if n else 1
        got = len(re.findall(old, text)) if isinstance(old, re.Pattern) else text.count(old)
        if got != want:
            sys.exit(f"{path}: {got} matches, not {want}, for {old!r}")
        text = old.sub(new, text) if isinstance(old, re.Pattern) else text.replace(old, new)
    p.write_text(text)


shutil.copy(here / f"str-{model}.ts", root / "translated" / "rs" / "proto.ts")
edit("rs.ts", [('export * from "./rs/time.ts";\n', 'export * from "./rs/time.ts";\nexport * from "./rs/proto.ts";\n')])

NAMES = '["demo/room-42/alice/camera", "démo/salle-42/アリス/📷"]'
TRACKS = '["video", "audio", "catalog.json", "字幕"]'
SET = re.compile(r"s = rs\.fmtWriteString\(s, (\[.*\])\)\.s;")

if utf8:
    edit("coding/encode.Encoder.ts", [("return bytes(self, rs.stringAsBytes(v));", "return bytes(self, v);")])
edit("coding/decode.Decoder.ts", [("rs.stringFromUtf8(", "rs.pStringFromUtf8(")])

bench = [
    # decode_strings
    ('let last: string = "";', "let last = new rs.ByteVec(0);" if utf8 else "let last = rs.pStringNew(0);"),
    ("rs.stringLen(last)", "last.length" if utf8 else "last.len", 2),
    # join
    (
        "let path: string = rs.stringWithCapacity(rs.usizeAdd(rs.usizeAdd(rs.stringLen(_7[_8]), 1), rs.stringLen(_30[_31])));",
        "const path = new rs.ByteVec(rs.usizeAdd(rs.usizeAdd(_7[_8].length, 1), _30[_31].length));" if utf8
        else "const path = rs.pStringNew(rs.usizeAdd(rs.usizeAdd(rs.pStrLen(_7[_8]), 1), rs.pStrLen(_30[_31])));",
    ),
    ("path = path + _7[_8];", "rs.pStringPushStr(path, _7[_8]);"),
    ('path = path + "/";', "rs.pStringPushAscii(path, 0x2f);" if utf8 else 'rs.pStringPushLit(path, "/", 1);'),
    ("path = path + _30[_31];", "rs.pStringPushStr(path, _30[_31]);"),
    ("rs.usizeAdd(total, rs.stringLen(path))", "rs.usizeAdd(total, path.length)" if utf8 else "rs.usizeAdd(total, path.len)"),
    ("rs.strStripPrefix(path, _7[_8])", "rs.pStrStripPrefix(rs.pStringAsStr(path), _7[_8])" if utf8 else "rs.strStripPrefix(path.s, _7[_8])"),
    ('rs.strStripPrefix(_53, "/")', "rs.pStrStripAscii(_53, 0x2f)" if utf8 else 'rs.strStripPrefix(_53, "/")'),
    ("rs.stringLen(_58)", "_58.length" if utf8 else "rs.pStrLen(_58)"),
    # keyed
    ("rs.fmtPad(f, part)", "rs.pFmtPadStr(f, part)" if utf8 else "rs.fmtPad(f, part)"),
    ('return rs.fmtFormat([args2[0], "/", args2[1]]);', 'return rs.pFmtFormat([args2[0], "/", args2[1]], 1);'),
    ("rs.hashNew() as rs.HashMap<string, rs.u64>", "rs.hashNew(rs.P_STR_HASHING) as rs.HashMap<string, rs.u64>" if utf8 else "rs.hashNew() as rs.HashMap<string, rs.u64>"),
    (
        "rs.hashInsert(map, _32[1], _32[0] as rs.u64);",
        "rs.hashInsert(map, rs.pStringClone(_32[1]), _32[0] as rs.u64);" if utf8 else "rs.pHashInsertString(map, rs.pStringClone(_32[1]), _32[0] as rs.u64);",
    ),
    ("rs.hashGet(map, _51)", "rs.hashGet(map, rs.pStringAsStr(_51))" if utf8 else "rs.pHashGetString(map, _51.s)"),
    # format
    ('const count2: rs.i64 = rs.i64Cast(rs.u64Min(takeU64(data)[0], 65536));\n\tlet s: string = "";',
     "const count2: rs.i64 = rs.i64Cast(rs.u64Min(takeU64(data)[0], 65536));\n\tlet s = " + ("new rs.ByteVec(0);" if utf8 else "rs.pStringNew(0);")),
    (SET, r"rs.pFmtWriteString(s, \1);"),
    ("putUsize(out, rs.stringLen(s));", "putUsize(out, s.length);" if utf8 else "putUsize(out, s.len);"),
    ("rs.strBytes(s)", "rs.iterSlice(rs.byteVecView(s))" if utf8 else "rs.strBytes(s.s)"),
]
if utf8:
    # Each literal's bytes, made once where the module loads.
    bench += [(NAMES, "[P_NAME0, P_NAME1]", 6), (TRACKS, "[P_TRACK0, P_TRACK1, P_TRACK2, P_TRACK3]", 2)]
edit("bench.ts", bench)
if utf8:
    p = root / "translated" / "bench.ts"
    text = p.read_text()
    lits = [("P_NAME0", "demo/room-42/alice/camera"), ("P_NAME1", "démo/salle-42/アリス/📷"),
            ("P_TRACK0", "video"), ("P_TRACK1", "audio"), ("P_TRACK2", "catalog.json"), ("P_TRACK3", "字幕")]
    decl = "".join(f'const {n} = /* @__PURE__ */ rs.pLit("{s}");\n' for n, s in lits)
    at = text.index("\nexport function ")
    p.write_text(text[: at + 1] + decl + text[at + 1 :])
