#include <node_api.h>
#include <pthread.h>
#include <stdint.h>
#include <stdbool.h>
/* A call Rust's thread makes into JavaScript while the caller's thread waits in Rust: the
 * caller serves it, as Dart's binding serves a closure called from another thread. */
typedef int32_t (*cb)(void *data, int32_t);
typedef struct { pthread_mutex_t m; pthread_cond_t c; bool asked, answered, done; int32_t arg, ret; } serve;
static int32_t trampoline(void *data, int32_t x) {
  serve *s = data; pthread_mutex_lock(&s->m); s->arg = x; s->asked = true; pthread_cond_broadcast(&s->c);
  while (!s->answered) pthread_cond_wait(&s->c, &s->m);
  s->answered = false; int32_t r = s->ret; pthread_mutex_unlock(&s->m); return r;
}
typedef struct { cb f; void *data; int32_t r; serve *s; } job;
static void *run(void *p) { job *j = p; j->r = j->f(j->data, 21); pthread_mutex_lock(&j->s->m); j->s->done = true; pthread_cond_broadcast(&j->s->c); pthread_mutex_unlock(&j->s->m); return 0; }
static napi_value on_thread(napi_env env, napi_callback_info info) {
  size_t argc = 1; napi_value fn; napi_get_cb_info(env, info, &argc, &fn, 0, 0);
  serve s = {PTHREAD_MUTEX_INITIALIZER, PTHREAD_COND_INITIALIZER};
  job j = {trampoline, &s, 0, &s}; pthread_t t; pthread_create(&t, 0, run, &j);
  pthread_mutex_lock(&s.m);
  for (;;) {
    while (!s.asked && !s.done) pthread_cond_wait(&s.c, &s.m);
    if (s.asked) {
      s.asked = false; pthread_mutex_unlock(&s.m);
      napi_value global, arg, ret; napi_get_global(env, &global); napi_create_int32(env, s.arg, &arg);
      napi_call_function(env, global, fn, 1, &arg, &ret); int32_t r; napi_get_value_int32(env, ret, &r);
      pthread_mutex_lock(&s.m); s.ret = r; s.answered = true; pthread_cond_broadcast(&s.c); continue;
    }
    break;
  }
  pthread_mutex_unlock(&s.m); pthread_join(t, 0);
  napi_value out; napi_create_int32(env, j.r, &out); return out;
}
NAPI_MODULE_INIT() { napi_value f; napi_create_function(env, "onThread", NAPI_AUTO_LENGTH, on_thread, 0, &f); napi_set_named_property(env, exports, "onThread", f); return exports; }
