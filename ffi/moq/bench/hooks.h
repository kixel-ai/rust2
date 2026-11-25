/* The binding benchmark's hooks into Callgrind: each program but the Rust floor, which counts
 * through rust2-bench-harness, calls them through its own FFI around each case it times, so
 * Valgrind counts that case alone. Outside Valgrind they do nothing. */

#ifndef HOOKS_H
#define HOOKS_H

#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Whether Valgrind measures this process, naming its cases `<bench>::<case>`, such as
 * `ffi/moq/bench/export.kt::kotlin`: the file, then the language. Call it once, first. */
bool hooks_init(const char *bench);

/* Starts counting a case. */
void hooks_start(void);

/* Stops counting, and names what was counted since hooks_start. */
void hooks_stop(const char *name);

#ifdef __cplusplus
}
#endif

#endif
