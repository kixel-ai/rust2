/* node:ffi probe: a callback called from another thread, which node:ffi aborts on. */
#include <pthread.h>
#include <stdint.h>
typedef int32_t (*cb)(int32_t);
static void *run(void *f) { return (void *)(intptr_t)((cb)f)(21); }
int32_t on_thread(cb f) { pthread_t t; void *r; pthread_create(&t, 0, run, f); pthread_join(t, &r); return (int32_t)(intptr_t)r; }
int32_t here(cb f) { return f(21); }
