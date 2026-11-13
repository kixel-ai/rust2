#!/usr/bin/env python3
"""Replace only chain 8 in a copied benchmark route's translated/bench.ts."""
import pathlib
import sys

path = pathlib.Path(sys.argv[1]) / "translated" / "bench.ts"
source = path.read_text()
start = source.index('\t} else if (chain === 8) {')
end = source.index('\t} else {', start)
source = source[:start] + '''\t} else if (chain === 8) {
		const left = rs.iterSlice(rs.byteSliceView(data));
		const right = rs.iterSlice(rs.byteSliceView(data));
		let count = 0;
		// Both leaves have the same exact length. Read left before right.
		for (let i = 0; i < data.length; i++) {
			const a = left.next() as rs.u8;
			const b = right.next() as rs.u8;
			if ((a >> 1) < (b ^ 3)) count++;
		}
		putUsize(out, count);
''' + source[end:]
path.write_text(source)
