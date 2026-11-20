// The runtime's UTF-8 decode and encode of each byte run on stdin, one in hex a line, and
// `char::encode_utf8` of each char after them, which `runtime.rs` compares with std's.
//
// usage: node utf8.ts < runs

import { readFileSync } from "node:fs";
import * as rs from "../runtime/rs.ts";

const hex = (v: Uint8Array) => Buffer.from(v).toString("hex");
const view = (s: rs.ByteSlice) => s.data.subarray(s.start, s.start + s.length);
const vecView = (v: rs.ByteVec) => v.data.subarray(0, v.length);

const out: string[] = [];
// Each line ends in a newline, so the last split is empty; an empty run is an empty line.
for (const line of readFileSync(0, "utf8").split("\n").slice(0, -1)) {
	if (line.startsWith("char ")) {
		const c = Number.parseInt(line.slice(5), 16) as rs.char;
		const dst = rs.byteSliceFrom(new Uint8Array(6).fill(0xaa));
		const s = rs.charEncodeUtf8(c, dst);
		out.push(`${hex(view(dst))} ${hex(view(rs.stringAsBytes(s.v)))}`);
		continue;
	}
	const bytes = new Uint8Array(Buffer.from(line, "hex"));
	const lossy = hex(view(rs.stringAsBytes(rs.strCowStr(rs.stringFromUtf8Lossy(rs.byteSliceFrom(bytes))))));
	const r = rs.strFromUtf8(rs.byteSliceFrom(bytes));
	const owned = rs.stringFromUtf8(rs.byteVecFrom(bytes));
	if (r.ok !== owned.ok) throw new Error(`str::from_utf8 and String::from_utf8 disagree on ${line}`);
	if (r.ok && owned.ok) {
		const s = r.value;
		out.push(`ok ${hex(view(rs.stringAsBytes(s)))} ${hex(vecView(rs.strToBytes(s)))} ${hex(vecView(rs.stringIntoBytes(owned.value)))} ${rs.stringLen(s)} ${lossy}`);
	} else if (!r.ok) {
		out.push(`err ${r.error.validUpTo} ${r.error.errorLen ?? "-"} ${lossy}`);
	}
}
process.stdout.write(`${out.join("\n")}\n`);
