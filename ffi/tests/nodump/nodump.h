/* `prctl` for the Swift test, which cannot call it: Swift imports no variadic C function. */

#include <sys/prctl.h>

/* Marks the process non-dumpable, so an expected abort writes no core and never starts the
 * host's crash handler. */
static inline int nodump(void) {
	return prctl(PR_SET_DUMPABLE, 0, 0, 0, 0);
}
