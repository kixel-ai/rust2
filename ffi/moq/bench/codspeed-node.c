/* codspeed.h as a Node addon, which export.ts loads to call CodSpeed's hooks around each
 * case, as each other program calls them through its own FFI: `init(bench)`, `start()`, and
 * `stop(name)`. */

#include <node_api.h>
#include <stdio.h>
#include <stdlib.h>

#include "codspeed.h"

static void fail(napi_env env, napi_status status, const char *what) {
	if (status != napi_ok) {
		fprintf(stderr, "codspeed-node: %s failed\n", what);
		exit(1);
	}
	(void)env;
}

/* The one string argument of a call, into `buf`. */
static void text(napi_env env, napi_callback_info info, char *buf, size_t len) {
	size_t argc = 1;
	napi_value arg;
	fail(env, napi_get_cb_info(env, info, &argc, &arg, NULL, NULL), "an argument");
	if (argc != 1) {
		fail(env, napi_invalid_arg, "one argument");
	}
	size_t written;
	fail(env, napi_get_value_string_utf8(env, arg, buf, len, &written), "a string argument");
	if (written + 1 >= len) {
		fail(env, napi_invalid_arg, "a name this short");
	}
}

static napi_value init(napi_env env, napi_callback_info info) {
	char bench[256];
	text(env, info, bench, sizeof bench);
	napi_value measured;
	fail(env, napi_get_boolean(env, codspeed_init(bench), &measured), "a boolean");
	return measured;
}

static napi_value start(napi_env env, napi_callback_info info) {
	(void)env;
	(void)info;
	codspeed_start();
	return NULL;
}

static napi_value stop(napi_env env, napi_callback_info info) {
	char name[256];
	text(env, info, name, sizeof name);
	codspeed_stop(name);
	return NULL;
}

NAPI_MODULE_INIT(/* napi_env env, napi_value exports */) {
	const napi_property_descriptor functions[] = {
		{"init", NULL, init, NULL, NULL, NULL, napi_default, NULL},
		{"start", NULL, start, NULL, NULL, NULL, napi_default, NULL},
		{"stop", NULL, stop, NULL, NULL, NULL, napi_default, NULL},
	};
	fail(env, napi_define_properties(env, exports, 3, functions), "the addon's functions");
	return exports;
}
