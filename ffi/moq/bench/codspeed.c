/* codspeed.h over CodSpeed's instrument-hooks. Built with CODSPEED_JNI, it also holds the
 * JNI functions export.kt's `Codspeed` object declares. */

#define _POSIX_C_SOURCE 200112L

#include "codspeed.h"

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include "core.h"

static InstrumentHooks *hooks;
static bool measured;
static char prefix[256];

static void fail(const char *what) {
	fprintf(stderr, "codspeed: %s failed\n", what);
	exit(1);
}

bool codspeed_init(const char *bench) {
	hooks = instrument_hooks_init();
	if (hooks == NULL) {
		fail("instrument_hooks_init");
	}
	if ((size_t)snprintf(prefix, sizeof prefix, "%s", bench) >= sizeof prefix) {
		fail("a bench name this long");
	}
	measured = instrument_hooks_is_instrumented(hooks);
	if (!measured) {
		return false;
	}
	/* A case is one run, which only the simulation counts exactly; walltime would time one. */
	if (!RUNNING_ON_VALGRIND) {
		fail("a CodSpeed instrument but its CPU simulation");
	}
	if (instrument_hooks_set_integration(hooks, "rust2-ffi-bench", "0.1.0") != 0) {
		fail("instrument_hooks_set_integration");
	}
	return true;
}

void codspeed_start(void) {
	if (measured && instrument_hooks_start_benchmark(hooks) != 0) {
		fail("instrument_hooks_start_benchmark");
	}
}

void codspeed_stop(const char *name) {
	if (!measured) {
		return;
	}
	if (instrument_hooks_stop_benchmark(hooks) != 0) {
		fail("instrument_hooks_stop_benchmark");
	}
	char uri[512];
	if ((size_t)snprintf(uri, sizeof uri, "%s::%s", prefix, name) >= sizeof uri) {
		fail("a case name this long");
	}
	if (instrument_hooks_set_executed_benchmark(hooks, (int32_t)getpid(), uri) != 0) {
		fail("instrument_hooks_set_executed_benchmark");
	}
}

#ifdef CODSPEED_JNI

#include <jni.h>

/* A Java string's modified UTF-8, which is plain UTF-8 for the ASCII names here. */
static const char *chars(JNIEnv *env, jstring s) {
	const char *c = (*env)->GetStringUTFChars(env, s, NULL);
	if (c == NULL) {
		fail("GetStringUTFChars");
	}
	return c;
}

JNIEXPORT jboolean JNICALL Java_Codspeed_init(JNIEnv *env, jobject self, jstring bench) {
	(void)self;
	const char *c = chars(env, bench);
	bool on = codspeed_init(c);
	(*env)->ReleaseStringUTFChars(env, bench, c);
	return on ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT void JNICALL Java_Codspeed_start(JNIEnv *env, jobject self) {
	(void)env;
	(void)self;
	codspeed_start();
}

JNIEXPORT void JNICALL Java_Codspeed_stop(JNIEnv *env, jobject self, jstring name) {
	(void)self;
	const char *c = chars(env, name);
	codspeed_stop(c);
	(*env)->ReleaseStringUTFChars(env, name, c);
}

#endif
