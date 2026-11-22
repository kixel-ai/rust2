// What api.swift leaves out because it depends on the export's targets, which the release
// builder's export differs in from the tests': it has no wasm32, so what only wasm32 has throws
// on the host's library here, wherever Swift lets it, and a path that is not UTF-8, which
// Windows lacks, is fallible there.
//
// With an argument, it instead runs a call the host's library lacks, where Swift cannot
// throw, which must fail.

import Api
import Foundation
#if os(Linux)
import NoDump
#endif

func check(_ ok: Bool, _ what: String) {
	if !ok {
		FileHandle.standardError.write(Data("failed: \(what)\n".utf8))
		exit(1)
	}
}

/// Check that `body` throws the CheckError of `function`, which the library's target lacks.
func lacks(_ function: String, _ body: () throws -> Void) {
	do {
		try body()
		check(false, "\(function) throws off its target")
	} catch let e as CheckError {
		check(
			e.function == function && e.message == "it is not on this library's target",
			"\(function) threw \(e)"
		)
	} catch {
		check(false, "\(function) threw \(error)")
	}
}

/// Each calls an item the host's library lacks, which must fail with a message naming it.
func broken(_ which: String) {
	#if os(Linux)
	// Non-dumpable, an expected abort writes no core.
	if nodump() != 0 {
		perror("prctl")
		exit(1)
	}
	#endif
	switch which {
	case "variant":
		_ = pointerBits(.wasm(bits: 32))
	case "plain variant":
		_ = isFamily(.wasm)
	default:
		check(false, "no case \(which)")
	}
}

@main
struct Targets {
	static func main() {
		switch CommandLine.arguments.dropFirst().first {
		case nil:
			lacks("api_wasm_only") { _ = try wasmOnly() }
			lacks("api_Page_default") { _ = try Page.default() }
			let booth = Booth(mark: Mark.of(1))
			lacks("api_Booth_debug") { _ = try booth.debugDescription }
			lacks("api_Booth_eq") { _ = try booth == booth }
			// Unix and WASI have a path that is not UTF-8, Windows does not, so the builder's
			// export serves it with a throw.
			let strayPath = stray()
			check(strayPath == PathBuf(bytes: Bytes([0xff])), "a path that is not UTF-8 crosses with its bytes")
			visitStray { path in check(path == strayPath, "a lent path that is not UTF-8 keeps its bytes") }
			print("targets ok")
		case let which?:
			broken(which)
		}
	}
}
