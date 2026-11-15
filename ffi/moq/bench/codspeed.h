/* The binding benchmark's hooks into CodSpeed's CPU simulation, through its instrument-hooks
 * library: each program calls them through its own FFI around each case it times, so
 * CodSpeed counts that case alone. Outside CodSpeed they do nothing. */

#ifndef CODSPEED_H
#define CODSPEED_H

#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Whether CodSpeed measures this process, naming its cases `<bench>::<case>`, such as
 * `ffi/moq/bench/export.kt::kotlin`: the file, then the language. Call it once, first. */
bool codspeed_init(const char *bench);

/* Starts counting a case. */
void codspeed_start(void);

/* Stops counting, and names what was counted since codspeed_start. */
void codspeed_stop(const char *name);

#ifdef __cplusplus
}
#endif

#endif
