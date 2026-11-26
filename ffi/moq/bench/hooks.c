/* hooks.h over Callgrind's client requests, issued as swole's harness issues them. Built
 * with HOOKS_JNI, it also holds the JNI functions export.kt's `Hooks` object declares. */

#include "hooks.h"

#include <stdio.h>
#include <stdlib.h>
#include <valgrind/callgrind.h>

static bool measured;
static char prefix[256];

static void fail(const char *what) {
	fprintf(stderr, "hooks: %s failed\n", what);
	exit(1);
}

bool hooks_init(const char *bench) {
	if ((size_t)snprintf(prefix, sizeof prefix, "%s", bench) >= sizeof prefix) {
		fail("a bench name this long");
	}
	measured = RUNNING_ON_VALGRIND;
	return measured;
}

void hooks_start(void) {
	if (measured) {
		CALLGRIND_ZERO_STATS;
		CALLGRIND_START_INSTRUMENTATION;
	}
}

void hooks_stop(const char *name) {
	if (!measured) {
		return;
	}
	CALLGRIND_STOP_INSTRUMENTATION;
	char uri[512];
	if ((size_t)snprintf(uri, sizeof uri, "%s::%s", prefix, name) >= sizeof uri) {
		fail("a case name this long");
	}
	CALLGRIND_DUMP_STATS_AT(uri);
}

#ifdef HOOKS_JNI

#include <jni.h>

/* A Java string's modified UTF-8, which is plain UTF-8 for the ASCII names here. */
static const char *chars(JNIEnv *env, jstring s) {
	const char *c = (*env)->GetStringUTFChars(env, s, NULL);
	if (c == NULL) {
		fail("GetStringUTFChars");
	}
	return c;
}

JNIEXPORT jboolean JNICALL Java_Hooks_init(JNIEnv *env, jobject self, jstring bench) {
	(void)self;
	const char *c = chars(env, bench);
	bool on = hooks_init(c);
	(*env)->ReleaseStringUTFChars(env, bench, c);
	return on ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT void JNICALL Java_Hooks_start(JNIEnv *env, jobject self) {
	(void)env;
	(void)self;
	hooks_start();
}

JNIEXPORT void JNICALL Java_Hooks_stop(JNIEnv *env, jobject self, jstring name) {
	(void)self;
	const char *c = chars(env, name);
	hooks_stop(c);
	(*env)->ReleaseStringUTFChars(env, name, c);
}

#endif
