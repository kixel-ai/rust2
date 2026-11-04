#!/usr/bin/env python3
"""Rewrite a fresh translation of the codec and task fixtures into one prototype model of
a `String`, for `just ts bench --route <model>=<dir>` to time against main. Prototype only,
for moq-dev/rust2#574's decision.

usage: str-routes.py <dir> js|utf8

<dir> holds `translated/` and `task/` as `just ts run` wrote them. Each edit is what the
translator would emit for that model on the paths the `str/` and `fmt/` workloads run;
every edit must match as often as it says.

- js: a `String` as `rt.PString`, a JS string with its UTF-8 length and capacity; a `&str`
  stays a JS string.
- utf8: a `String` as an `rt.ByteVec`, a `&str` as an `rt.ByteSlice`, each literal's bytes
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


shutil.copy(here / f"str-{model}.ts", root / "translated" / "rt" / "proto.ts")
edit("rt.ts", [('export * from "./rt/time.ts";\n', 'export * from "./rt/time.ts";\nexport * from "./rt/proto.ts";\n')])

NAMES = '["demo/room-42/alice/camera", "démo/salle-42/アリス/📷"]'
TRACKS = '["video", "audio", "catalog.json", "字幕"]'
SET = re.compile(r"s = rt\.fmtWriteString\(s, (\[.*\])\)\.s;")

if utf8:
    edit("coding/encode.Encoder.ts", [("return bytes(self, rt.stringAsBytes(v));", "return bytes(self, v);")])
edit("coding/decode.Decoder.ts", [("rt.stringFromUtf8(", "rt.pStringFromUtf8(")])

bench = [
    # decode_strings
    ('let last: string = "";', "let last = new rt.ByteVec(0);" if utf8 else "let last = rt.pStringNew(0);"),
    ("rt.stringLen(last)", "last.length" if utf8 else "last.len", 2),
    # join
    (
        "let path: string = rt.stringWithCapacity(rt.usizeAdd(rt.usizeAdd(rt.stringLen(_7[_8]), 1), rt.stringLen(_30[_31])));",
        "const path = new rt.ByteVec(rt.usizeAdd(rt.usizeAdd(_7[_8].length, 1), _30[_31].length));" if utf8
        else "const path = rt.pStringNew(rt.usizeAdd(rt.usizeAdd(rt.pStrLen(_7[_8]), 1), rt.pStrLen(_30[_31])));",
    ),
    ("path = path + _7[_8];", "rt.pStringPushStr(path, _7[_8]);"),
    ('path = path + "/";', "rt.pStringPushAscii(path, 0x2f);" if utf8 else 'rt.pStringPushLit(path, "/", 1);'),
    ("path = path + _30[_31];", "rt.pStringPushStr(path, _30[_31]);"),
    ("rt.usizeAdd(total, rt.stringLen(path))", "rt.usizeAdd(total, path.length)" if utf8 else "rt.usizeAdd(total, path.len)"),
    ("rt.strStripPrefix(path, _7[_8])", "rt.pStrStripPrefix(rt.pStringAsStr(path), _7[_8])" if utf8 else "rt.strStripPrefix(path.s, _7[_8])"),
    ('rt.strStripPrefix(_53, "/")', "rt.pStrStripAscii(_53, 0x2f)" if utf8 else 'rt.strStripPrefix(_53, "/")'),
    ("rt.stringLen(_58)", "_58.length" if utf8 else "rt.pStrLen(_58)"),
    # keyed
    ("rt.fmtPad(f, part)", "rt.pFmtPadStr(f, part)" if utf8 else "rt.fmtPad(f, part)"),
    ('return rt.fmtFormat([args2[0], "/", args2[1]]);', 'return rt.pFmtFormat([args2[0], "/", args2[1]], 1);'),
    ("rt.hashNew() as rt.HashMap<string, rt.u64>", "rt.hashNew(rt.P_STR_HASHING) as rt.HashMap<string, rt.u64>" if utf8 else "rt.hashNew() as rt.HashMap<string, rt.u64>"),
    (
        "rt.hashInsert(map, _32[1], _32[0] as rt.u64);",
        "rt.hashInsert(map, rt.pStringClone(_32[1]), _32[0] as rt.u64);" if utf8 else "rt.pHashInsertString(map, rt.pStringClone(_32[1]), _32[0] as rt.u64);",
    ),
    ("rt.hashGet(map, _51)", "rt.hashGet(map, rt.pStringAsStr(_51))" if utf8 else "rt.pHashGetString(map, _51.s)"),
    # format
    ('const count2: rt.i64 = rt.i64Cast(rt.u64Min(takeU64(data)[0], 65536));\n\tlet s: string = "";',
     "const count2: rt.i64 = rt.i64Cast(rt.u64Min(takeU64(data)[0], 65536));\n\tlet s = " + ("new rt.ByteVec(0);" if utf8 else "rt.pStringNew(0);")),
    (SET, r"rt.pFmtWriteString(s, \1);"),
    ("putUsize(out, rt.stringLen(s));", "putUsize(out, s.length);" if utf8 else "putUsize(out, s.len);"),
    ("rt.strBytes(s)", "rt.iterSlice(rt.byteVecView(s))" if utf8 else "rt.strBytes(s.s)"),
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
    decl = "".join(f'const {n} = /* @__PURE__ */ rt.pLit("{s}");\n' for n, s in lits)
    at = text.index("\nexport function ")
    p.write_text(text[: at + 1] + decl + text[at + 1 :])
