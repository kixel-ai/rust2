/* Pointer-only twins of the moq.h functions node:ffi cannot call: it has no struct type, so
 * a struct crosses only through a pointer, as wasm32's C ABI lowers it for the web binding.
 * A struct of one scalar crosses as that scalar and needs no twin. This is what a glue
 * feature for a thin node:ffi binding would emit, one twin per function taking or returning
 * a struct of more than one scalar. */

#include <string.h>

#include "moq.h"

/* Scratch memory the program views as typed arrays: its argument structs and out slots. */
_Alignas(64) unsigned char nf_scratch[1 << 16];

void *nf_scratch_ptr(void) { return nf_scratch; }

bool nf_track_Producer_largest(const moq_track_Producer *self, moq_option_track_Position *out, moq_status *status) {
	*out = moq_track_Producer_largest(self, status);
	return out->some;
}

bool nf_track_Producer_append_group(const moq_track_Producer *self, const moq_Timestamp *start, moq_group_Producer **out, moq_Error **err, moq_status *status) {
	return moq_track_Producer_append_group(self, *start, out, err, status);
}

bool nf_group_Producer_write_frame(moq_group_Producer *self, const moq_Timestamp *at, const moq_buf *data, moq_Error **err, moq_status *status) {
	return moq_group_Producer_write_frame(self, *at, *data, err, status);
}

/* A payload's bytes: up to 64 copied to `copy` and freed, as the web binding holds them,
 * past it written to `out`, lent until `nf_buf_free`. Returns their length. */
size_t nf_frame_Frame_payload_get(const moq_frame_Frame *self, moq_buf *out, uint8_t *copy, moq_status *status) {
	moq_buf buf = moq_frame_Frame_payload_get(self, status);
	if (buf.len <= 64) {
		memcpy(copy, buf.ptr, buf.len);
		moq_buf_free(buf);
	} else {
		*out = buf;
	}
	return buf.len;
}

uint8_t nf_byte_at(const uint8_t *ptr, uint32_t i) { return ptr[i]; }

void nf_buf_copy(const uint8_t *ptr, uint8_t *into, size_t len) { memcpy(into, ptr, len); }

void nf_buf_free(const uint8_t *ptr, size_t len, void *owner) { moq_buf_free((moq_buf){.ptr = ptr, .len = len, .owner = owner}); }

moq_poll nf_group_Consumer_read_frame_poll(moq_group_Consumer_read_frame_future *future, const moq_waker *waker, moq_frame_Frame **out, moq_Error **err) {
	return moq_group_Consumer_read_frame_poll(future, *waker, out, err);
}

moq_poll nf_track_Subscriber_recv_group_poll(moq_track_Subscriber_recv_group_future *future, const moq_waker *waker, moq_group_Consumer **out, moq_Error **err) {
	return moq_track_Subscriber_recv_group_poll(future, *waker, out, err);
}

moq_poll nf_track_Consumer_subscribe_poll(moq_track_Consumer_subscribe_future *future, const moq_waker *waker, moq_track_Subscriber **out, moq_Error **err) {
	return moq_track_Consumer_subscribe_poll(future, *waker, out, err);
}

bool nf_broadcast_Producer_create_track(const moq_broadcast_Producer *self, const moq_buf *name, moq_track_Info *info, moq_track_Producer **out, moq_Error **err, moq_status *status) {
	return moq_broadcast_Producer_create_track(self, *name, info, out, err, status);
}

moq_track_Info *nf_track_Info_with_timescale(moq_track_Info *self, const moq_option_Timescale *timescale, moq_status *status) {
	return moq_track_Info_with_timescale(self, *timescale, status);
}
