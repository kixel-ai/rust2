/* api.h */
#ifndef API_H
#define API_H
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#ifndef API_MUST_USE
#if (defined(__cplusplus) && __cplusplus >= 201703L) || (defined(_MSVC_LANG) && _MSVC_LANG >= 201703L) || \
    (defined(__STDC_VERSION__) && __STDC_VERSION__ >= 202311L)
#define API_MUST_USE [[nodiscard]]
#elif defined(__GNUC__)
#define API_MUST_USE __attribute__((warn_unused_result))
#else
#define API_MUST_USE
#endif
#endif
#ifdef __cplusplus
extern "C" {
#endif
#define API_CONTRACT ((uint64_t)0x06b4a9cf5dcdd72bULL)
uint64_t api_contract(void);
uint32_t api_target(void);
typedef int32_t api_status;
enum {
	API_STATUS_MOVED = 1,
	API_STATUS_EXCLUSIVE = 2,
	API_STATUS_BORROWED = 3,
	API_STATUS_THREAD = 4,
	API_STATUS_LENT = 5,
	API_STATUS_OUTLIVED = 6,
	API_STATUS_NOT_SEND = 7,
};
const char *api_status_message(api_status status);
typedef struct api_buf {
	const uint8_t *ptr;
	size_t len;
	void *owner;
} api_buf;
void api_buf_free(api_buf buf);
API_MUST_USE api_buf api_buf_copy(api_buf view);
API_MUST_USE api_buf api_buf_clone(api_buf buf);
typedef struct api_u128 {
	uint64_t lo;
	uint64_t hi;
} api_u128;
typedef struct api_duration {
	uint64_t secs;
	uint32_t nanos;
} api_duration;
typedef struct api_ipv4_addr {
	uint8_t octets[4];
} api_ipv4_addr;
typedef struct api_ipv6_addr {
	uint8_t octets[16];
} api_ipv6_addr;
typedef struct api_ip_addr {
	bool v6;
	union {
		api_ipv4_addr v4;
		api_ipv6_addr v6;
	} ip;
} api_ip_addr;
typedef struct api_socket_addr_v4 {
	api_ipv4_addr ip;
	uint16_t port;
} api_socket_addr_v4;
typedef struct api_socket_addr_v6 {
	api_ipv6_addr ip;
	uint16_t port;
	uint32_t flowinfo;
	uint32_t scope_id;
} api_socket_addr_v6;
typedef struct api_socket_addr {
	bool v6;
	union {
		api_socket_addr_v4 v4;
		api_socket_addr_v6 v6;
	} addr;
} api_socket_addr;
typedef struct api_waker {
	void (*wake)(void *data);
	void (*drop)(void *data);
	void *data;
} api_waker;
void api_thread_wake(api_waker wake);
void api_thread_drain(void);
typedef struct api_foreign_future {
	bool (*poll)(void *data, api_waker waker);
	void (*free)(void *data);
	void *data;
} api_foreign_future;
typedef struct api_foreign_reader {
	bool (*read)(void *data, uint8_t *buf, size_t len, size_t *n, api_buf *err);
	bool (*seek)(void *data, int whence, int64_t offset, uint64_t *position, api_buf *err);
	void *data;
} api_foreign_reader;
typedef enum api_poll {
	API_POLL_PENDING = 0,
	API_POLL_READY = 1,
	API_POLL_ERROR = 2,
} api_poll;
typedef struct api_Board api_Board;
typedef struct api_Tab api_Tab;
typedef struct api_Receipt api_Receipt;
typedef struct api_Draft api_Draft;
typedef struct api_Filed_Receipt api_Filed_Receipt;
typedef struct api_Pad_u16 api_Pad_u16;
typedef struct api_Pad_u8 api_Pad_u8;
typedef struct api_Layered api_Layered;
typedef struct api_Marker api_Marker;
typedef struct api_Batch api_Batch;
typedef struct api_Batch_2 api_Batch_2;
typedef struct api_Waiter api_Waiter;
typedef struct api_Feed api_Feed;
typedef struct api_Countdown api_Countdown;
typedef struct api_Retired api_Retired;
typedef struct api_Logger api_Logger;
typedef struct api_Journal api_Journal;
typedef struct api_Local api_Local;
typedef struct api_LocalWaiter api_LocalWaiter;
typedef struct api_Retry api_Retry;
typedef struct api_Digest api_Digest;
typedef struct api_Cursor api_Cursor;
typedef struct api_Outer_Inner_u8 api_Outer_Inner_u8;
typedef struct api_Slot_u32 api_Slot_u32;
typedef struct api_Slot_u8 api_Slot_u8;
typedef struct api_Resolving api_Resolving;
typedef struct api_Codecs api_Codecs;
typedef struct api_Der api_Der;
typedef struct api_Verbosity api_Verbosity;
typedef struct api_Site api_Site;
typedef struct api_Trail api_Trail;
typedef struct api_Worker api_Worker;
typedef struct api_Handle api_Handle;
typedef struct api_Holder api_Holder;
typedef struct api_Area api_Area;
typedef struct api_Counter api_Counter;
typedef struct api_Meter api_Meter;
typedef struct api_Meters api_Meters;
typedef struct api_Registry api_Registry;
typedef struct api_Ledger api_Ledger;
typedef struct api_Slip api_Slip;
typedef struct api_Sheet api_Sheet;
typedef struct api_Name api_Name;
typedef struct api_Channel api_Channel;
typedef struct api_Gap api_Gap;
typedef struct api_Berth api_Berth;
typedef struct api_Mark api_Mark;
typedef struct api_Message api_Message;
typedef struct api_Node api_Node;
typedef struct api_Tally api_Tally;
typedef struct api_Tallies api_Tallies;
typedef struct api_Plain api_Plain;
typedef struct api_Labeled api_Labeled;
typedef struct api_Gauge api_Gauge;
typedef struct api_Latch api_Latch;
typedef struct api_Token api_Token;
typedef struct api_Stack api_Stack;
typedef struct api_Refusal api_Refusal;
typedef struct api_Fault api_Fault;
typedef struct api_c api_c;
typedef struct api_Quantity api_Quantity;
typedef struct api_Pass api_Pass;
typedef struct api_Key api_Key;
typedef struct api_clash_Seed api_clash_Seed;
typedef struct api_clash_Empty_string api_clash_Empty_string;
typedef struct api_clash_Store api_clash_Store;
typedef struct api_tls_Session api_tls_Session;
typedef struct api_Url api_Url;
typedef struct api_Json api_Json;
typedef struct api_Instant api_Instant;
typedef struct api_std_io_Error api_std_io_Error;
typedef struct api_tls_Config api_tls_Config;
typedef struct api_tls_Cert api_tls_Cert;
typedef struct api_tls_Resolver api_tls_Resolver;
typedef struct api_tls_Codec api_tls_Codec;
typedef struct api_Spawned api_Spawned;
typedef struct api_LogThread api_LogThread;
typedef struct api_Pinned api_Pinned;
typedef struct api_Hooked api_Hooked;
typedef struct api_Forged api_Forged;
typedef struct api_Part api_Part;
typedef enum api_Shape {
	API_SHAPE_CIRCLE = 1,
	API_SHAPE_SQUARE = 4,
} api_Shape;
typedef enum api_Status_ {
	API_STATUS__MOVED = 0,
	API_STATUS__KEPT = 1,
} api_Status_;
typedef enum api_Family {
	API_FAMILY_NATIVE = 0,
	API_FAMILY_WASM = 1,
} api_Family;
typedef enum api_VecClashTone {
	API_VECCLASHTONE_ERROR = 0,
} api_VecClashTone;
typedef enum api_Tier {
	API_TIER_LOW = 0,
	API_TIER_HIGH = 1,
	API_TIER_COMPARE_TO = 2,
	API_TIER_PARTIAL_COMPARE_TO = 3,
} api_Tier;
typedef enum api_Level {
	API_LEVEL_ERROR = 0,
	API_LEVEL_WARN = 1,
	API_LEVEL_INFO = 2,
	API_LEVEL_DEBUG = 3,
	API_LEVEL_TRACE = 4,
} api_Level;
typedef enum api_bound {
	API_BOUND_UNBOUNDED = 0,
	API_BOUND_INCLUDED = 1,
	API_BOUND_EXCLUDED = 2,
} api_bound;
typedef struct api_vec_Counter {
	api_Counter *const *ptr;
	size_t len;
	void *owner;
} api_vec_Counter;
void api_vec_Counter_free(api_vec_Counter vec);
typedef struct api_Point {
	int32_t x;
	int32_t y;
} api_Point;
typedef struct api_option_Point {
	bool some;
	api_Point value;
} api_option_Point;
typedef struct api_option_duration {
	bool some;
	api_duration value;
} api_option_duration;
typedef struct api_Label {
	api_buf name;
	api_option_Point at;
	api_option_duration ttl;
} api_Label;
typedef struct api_vec_Label {
	const api_Label *ptr;
	size_t len;
	void *owner;
} api_vec_Label;
void api_vec_Label_free(api_vec_Label vec);
typedef struct api_vec_Point {
	const api_Point *ptr;
	size_t len;
	void *owner;
} api_vec_Point;
void api_vec_Point_free(api_vec_Point vec);
typedef struct api_vec_Refusal {
	api_Refusal *const *ptr;
	size_t len;
	void *owner;
} api_vec_Refusal;
void api_vec_Refusal_free(api_vec_Refusal vec);
typedef struct api_vec_Shape {
	const api_Shape *ptr;
	size_t len;
	void *owner;
} api_vec_Shape;
void api_vec_Shape_free(api_vec_Shape vec);
typedef struct api_vec_Token {
	api_Token *const *ptr;
	size_t len;
	void *owner;
} api_vec_Token;
void api_vec_Token_free(api_vec_Token vec);
typedef struct api_vec_bytes {
	const api_buf *ptr;
	size_t len;
	void *owner;
} api_vec_bytes;
void api_vec_bytes_free(api_vec_bytes vec);
typedef struct api_clash_Tone {
	uint16_t hz;
} api_clash_Tone;
typedef struct api_vec_clash_Tone {
	const api_clash_Tone *ptr;
	size_t len;
	void *owner;
} api_vec_clash_Tone;
void api_vec_clash_Tone_free(api_vec_clash_Tone vec);
typedef struct api_vec_f32 {
	const float *ptr;
	size_t len;
	void *owner;
} api_vec_f32;
void api_vec_f32_free(api_vec_f32 vec);
typedef struct api_vec_f64 {
	const double *ptr;
	size_t len;
	void *owner;
} api_vec_f64;
void api_vec_f64_free(api_vec_f64 vec);
typedef struct api_vec_i16 {
	const int16_t *ptr;
	size_t len;
	void *owner;
} api_vec_i16;
void api_vec_i16_free(api_vec_i16 vec);
typedef struct api_vec_i32 {
	const int32_t *ptr;
	size_t len;
	void *owner;
} api_vec_i32;
void api_vec_i32_free(api_vec_i32 vec);
typedef struct api_vec_i64 {
	const int64_t *ptr;
	size_t len;
	void *owner;
} api_vec_i64;
void api_vec_i64_free(api_vec_i64 vec);
typedef struct api_vec_i8 {
	const int8_t *ptr;
	size_t len;
	void *owner;
} api_vec_i8;
void api_vec_i8_free(api_vec_i8 vec);
typedef struct api_vec_option_path {
	const api_buf *ptr;
	size_t len;
	void *owner;
} api_vec_option_path;
void api_vec_option_path_free(api_vec_option_path vec);
typedef struct api_vec_path {
	const api_buf *ptr;
	size_t len;
	void *owner;
} api_vec_path;
void api_vec_path_free(api_vec_path vec);
typedef struct api_vec_socket_addr {
	const api_socket_addr *ptr;
	size_t len;
	void *owner;
} api_vec_socket_addr;
void api_vec_socket_addr_free(api_vec_socket_addr vec);
typedef struct api_vec_string {
	const api_buf *ptr;
	size_t len;
	void *owner;
} api_vec_string;
void api_vec_string_free(api_vec_string vec);
typedef struct api_vec_tls_Cert {
	api_tls_Cert *const *ptr;
	size_t len;
	void *owner;
} api_vec_tls_Cert;
void api_vec_tls_Cert_free(api_vec_tls_Cert vec);
typedef struct api_vec_tls_Codec {
	api_tls_Codec *const *ptr;
	size_t len;
	void *owner;
} api_vec_tls_Codec;
void api_vec_tls_Codec_free(api_vec_tls_Codec vec);
typedef struct api_tuple_Counter_Refusal {
	api_Counter *_0;
	api_Refusal *_1;
} api_tuple_Counter_Refusal;
typedef struct api_vec_tuple_Counter_Refusal {
	const api_tuple_Counter_Refusal *ptr;
	size_t len;
	void *owner;
} api_vec_tuple_Counter_Refusal;
void api_vec_tuple_Counter_Refusal_free(api_vec_tuple_Counter_Refusal vec);
typedef struct api_tuple_string_Counter {
	api_buf _0;
	api_Counter *_1;
} api_tuple_string_Counter;
typedef struct api_vec_tuple_string_Counter {
	const api_tuple_string_Counter *ptr;
	size_t len;
	void *owner;
} api_vec_tuple_string_Counter;
void api_vec_tuple_string_Counter_free(api_vec_tuple_string_Counter vec);
typedef struct api_tuple_string_Json {
	api_buf _0;
	api_Json *_1;
} api_tuple_string_Json;
typedef struct api_vec_tuple_string_Json {
	const api_tuple_string_Json *ptr;
	size_t len;
	void *owner;
} api_vec_tuple_string_Json;
void api_vec_tuple_string_Json_free(api_vec_tuple_string_Json vec);
typedef struct api_tuple_string_Point {
	api_buf _0;
	api_Point _1;
} api_tuple_string_Point;
typedef struct api_vec_tuple_string_Point {
	const api_tuple_string_Point *ptr;
	size_t len;
	void *owner;
} api_vec_tuple_string_Point;
void api_vec_tuple_string_Point_free(api_vec_tuple_string_Point vec);
typedef struct api_tuple_string_Shape_Point {
	api_buf _0;
	api_Shape _1;
	api_Point _2;
} api_tuple_string_Shape_Point;
typedef struct api_vec_tuple_string_Shape_Point {
	const api_tuple_string_Shape_Point *ptr;
	size_t len;
	void *owner;
} api_vec_tuple_string_Shape_Point;
void api_vec_tuple_string_Shape_Point_free(api_vec_tuple_string_Shape_Point vec);
typedef struct api_Ticket {
	uint32_t opaque[1];
} api_Ticket;
typedef struct api_tuple_string_Ticket {
	api_buf _0;
	api_Ticket _1;
} api_tuple_string_Ticket;
typedef struct api_vec_tuple_string_Ticket {
	const api_tuple_string_Ticket *ptr;
	size_t len;
	void *owner;
} api_vec_tuple_string_Ticket;
void api_vec_tuple_string_Ticket_free(api_vec_tuple_string_Ticket vec);
typedef struct api_vec_u16 {
	const uint16_t *ptr;
	size_t len;
	void *owner;
} api_vec_u16;
void api_vec_u16_free(api_vec_u16 vec);
typedef struct api_vec_u32 {
	const uint32_t *ptr;
	size_t len;
	void *owner;
} api_vec_u32;
void api_vec_u32_free(api_vec_u32 vec);
typedef struct api_vec_u64 {
	const uint64_t *ptr;
	size_t len;
	void *owner;
} api_vec_u64;
void api_vec_u64_free(api_vec_u64 vec);
typedef struct api_vec_vec_u32 {
	const api_vec_u32 *ptr;
	size_t len;
	void *owner;
} api_vec_vec_u32;
void api_vec_vec_u32_free(api_vec_vec_u32 vec);
typedef struct api_bound_u64 {
	api_bound kind;
	uint64_t value;
} api_bound_u64;
typedef struct api_bounds_u64 {
	api_bound_u64 start;
	api_bound_u64 end;
} api_bounds_u64;
typedef struct api_range_u64 {
	uint64_t start;
	uint64_t end;
} api_range_u64;
typedef struct api_array_u8_4 {
	uint8_t items[4];
} api_array_u8_4;
typedef struct api_option_Level {
	bool some;
	api_Level value;
} api_option_Level;
typedef struct api_Moment {
	uint64_t opaque[2];
} api_Moment;
typedef struct api_option_Moment {
	bool some;
	api_Moment value;
} api_option_Moment;
typedef struct api_Reading {
	bool ok;
	float level;
	double ratio;
	uint32_t mark;
	api_Shape shape;
	api_Point at;
	api_buf note;
	uint64_t count;
	int8_t offset;
	size_t len;
	uint16_t tag;
} api_Reading;
typedef struct api_option_Reading {
	bool some;
	api_Reading value;
} api_option_Reading;
typedef struct api_option_Shape {
	bool some;
	api_Shape value;
} api_option_Shape;
typedef struct api_Stamp {
	uint64_t opaque[1];
} api_Stamp;
typedef struct api_option_Stamp {
	bool some;
	api_Stamp value;
} api_option_Stamp;
typedef struct api_option_char {
	bool some;
	uint32_t value;
} api_option_char;
typedef struct api_option_non_zero_u64 {
	bool some;
	uint64_t value;
} api_option_non_zero_u64;
typedef struct api_polar_Point {
	uint8_t turns;
	uint32_t distance;
} api_polar_Point;
typedef struct api_option_polar_Point {
	bool some;
	api_polar_Point value;
} api_option_polar_Point;
typedef struct api_tuple_Counter {
	api_Counter *_0;
} api_tuple_Counter;
typedef struct api_option_tuple_Counter {
	bool some;
	api_tuple_Counter value;
} api_option_tuple_Counter;
typedef struct api_option_tuple_Counter_Refusal {
	bool some;
	api_tuple_Counter_Refusal value;
} api_option_tuple_Counter_Refusal;
typedef struct api_option_u32 {
	bool some;
	uint32_t value;
} api_option_u32;
typedef struct api_tuple_option_u32 {
	api_option_u32 _0;
} api_tuple_option_u32;
typedef struct api_option_tuple_option_u32 {
	bool some;
	api_tuple_option_u32 value;
} api_option_tuple_option_u32;
typedef struct api_tuple_string {
	api_buf _0;
} api_tuple_string;
typedef struct api_option_tuple_string {
	bool some;
	api_tuple_string value;
} api_option_tuple_string;
typedef struct api_tuple_string_Shape {
	api_buf _0;
	api_Shape _1;
} api_tuple_string_Shape;
typedef struct api_option_tuple_string_Shape {
	bool some;
	api_tuple_string_Shape value;
} api_option_tuple_string_Shape;
typedef struct api_tuple_u8_u16 {
	uint8_t _0;
	uint16_t _1;
} api_tuple_u8_u16;
typedef struct api_tuple_tuple_u8_u16_u32 {
	api_tuple_u8_u16 _0;
	uint32_t _1;
} api_tuple_tuple_u8_u16_u32;
typedef struct api_option_tuple_tuple_u8_u16_u32 {
	bool some;
	api_tuple_tuple_u8_u16_u32 value;
} api_option_tuple_tuple_u8_u16_u32;
typedef struct api_tuple_u8_u16_u32 {
	uint8_t _0;
	uint16_t _1;
	uint32_t _2;
} api_tuple_u8_u16_u32;
typedef struct api_tuple_tuple_u8_u16_u32_ {
	api_tuple_u8_u16_u32 _0;
} api_tuple_tuple_u8_u16_u32_;
typedef struct api_option_tuple_tuple_u8_u16_u32_ {
	bool some;
	api_tuple_tuple_u8_u16_u32_ value;
} api_option_tuple_tuple_u8_u16_u32_;
typedef struct api_tuple_u8_i16_u32_u64 {
	uint8_t _0;
	int16_t _1;
	uint32_t _2;
	uint64_t _3;
} api_tuple_u8_i16_u32_u64;
typedef struct api_option_tuple_u8_i16_u32_u64 {
	bool some;
	api_tuple_u8_i16_u32_u64 value;
} api_option_tuple_u8_i16_u32_u64;
typedef struct api_tuple_usize_usize {
	size_t _0;
	size_t _1;
} api_tuple_usize_usize;
typedef struct api_option_tuple_usize_usize {
	bool some;
	api_tuple_usize_usize value;
} api_option_tuple_usize_usize;
typedef struct api_option_u128 {
	bool some;
	api_u128 value;
} api_option_u128;
typedef struct api_option_u64 {
	bool some;
	uint64_t value;
} api_option_u64;
typedef struct api_Seat {
	api_Mark *mark;
	uint32_t row;
} api_Seat;
typedef enum api_Choice_tag {
	API_CHOICE_EMPTY = 0,
	API_CHOICE_PICKED = 1,
} api_Choice_tag;
typedef struct api_Choice_Picked {
	api_Mark *_0;
} api_Choice_Picked;
typedef struct api_Choice {
	api_Choice_tag tag;
	union {
		api_Choice_Picked picked;
	};
} api_Choice;
typedef struct api_tuple_Counter_Mark_Seat_Choice {
	api_Counter *_0;
	api_Mark *_1;
	api_Seat _2;
	api_Choice _3;
} api_tuple_Counter_Mark_Seat_Choice;
typedef struct api_tuple_Counter_bytes {
	api_Counter *_0;
	api_buf _1;
} api_tuple_Counter_bytes;
typedef struct api_tuple_Mark_bytes {
	api_Mark *_0;
	api_buf _1;
} api_tuple_Mark_bytes;
typedef struct api_tuple_bytes {
	api_buf _0;
} api_tuple_bytes;
typedef struct api_tuple_bytes_Refusal {
	api_buf _0;
	api_Refusal *_1;
} api_tuple_bytes_Refusal;
typedef struct api_tuple_bytes_u32 {
	api_buf _0;
	uint32_t _1;
} api_tuple_bytes_u32;
typedef struct api_tuple_string_u32 {
	api_buf _0;
	uint32_t _1;
} api_tuple_string_u32;
typedef struct api_tuple_u32_u32 {
	uint32_t _0;
	uint32_t _1;
} api_tuple_u32_u32;
typedef struct api_Parcel {
	api_Mark *mark;
	api_buf payload;
} api_Parcel;
typedef struct api_Score {
	uint64_t opaque[1];
} api_Score;
typedef struct api_Weighted {
	api_Score score;
	uint32_t weight;
} api_Weighted;
typedef struct api_Stamped {
	api_Stamp at;
	uint32_t count;
} api_Stamped;
typedef struct api_CancellationError {
	uint32_t code;
} api_CancellationError;
typedef struct api_Packet {
	uint32_t seq;
	api_buf payload;
} api_Packet;
typedef struct api_Hasher {
	uint8_t seed;
} api_Hasher;
typedef struct api_Size {
	uint32_t width;
	uint32_t height;
} api_Size;
typedef struct api_Note {
	api_buf text;
} api_Note;
typedef struct api_Packaged {
	uint32_t api;
} api_Packaged;
typedef struct api_Atlas {
	api_vec_tuple_string_Point points;
	api_vec_u32 ids;
} api_Atlas;
typedef struct api_Span {
	api_tuple_u32_u32 bounds;
	api_option_tuple_string_Shape named;
} api_Span;
typedef struct api_Window {
	api_range_u64 range;
	api_bound_u64 end;
} api_Window;
typedef struct api_LongArray {
	uint64_t opaque[2];
} api_LongArray;
typedef struct api_JvmField {
	uint32_t n;
} api_JvmField;
typedef struct api_JvmStatic {
	uint32_t n;
} api_JvmStatic;
typedef struct api_OptIn {
	uint32_t n;
} api_OptIn;
typedef struct api_Wide {
	uint64_t opaque[128];
} api_Wide;
typedef struct api_Clocked {
	api_Moment at;
	uint32_t count;
} api_Clocked;
typedef struct api_Rated {
	uint64_t opaque[2];
} api_Rated;
typedef struct api_Hits {
	uint64_t opaque[2];
} api_Hits;
typedef struct api_Extent {
	uint64_t opaque[2];
} api_Extent;
typedef struct api_Pair_Size_u8 {
	api_Size left;
	uint8_t right;
} api_Pair_Size_u8;
typedef struct api_Pair_non_zero_u16 {
	uint16_t left;
	uint16_t right;
} api_Pair_non_zero_u16;
typedef struct api_Pair_u32 {
	uint32_t left;
	uint32_t right;
} api_Pair_u32;
typedef struct api_Pair_u8_Size {
	uint8_t left;
	api_Size right;
} api_Pair_u8_Size;
typedef struct api_Right_u8 {
	uint8_t value;
} api_Right_u8;
typedef struct api_Left_Right_u8 {
	api_Right_u8 value;
} api_Left_Right_u8;
typedef struct api_Left_u8 {
	uint8_t value;
} api_Left_u8;
typedef struct api_Right_Left_u8 {
	api_Left_u8 value;
} api_Right_Left_u8;
typedef struct api_Inner_u8 {
	uint8_t value;
} api_Inner_u8;
typedef struct api_Boxed_u16 {
	uint16_t value;
} api_Boxed_u16;
typedef struct api_Boxed_u8 {
	uint8_t value;
} api_Boxed_u8;
typedef struct api_LogCount {
	uint32_t kept;
} api_LogCount;
typedef struct api_Wrap_u32 {
	uint32_t value;
} api_Wrap_u32;
typedef struct api_PolarTurn {
	uint8_t whole;
} api_PolarTurn;
typedef struct api_Peer {
	api_socket_addr addr;
	api_vec_string names;
} api_Peer;
typedef struct api_Shadowed {
	api_vec_i32 other;
	uint8_t a;
	uint8_t b;
} api_Shadowed;
typedef struct api_Numbers {
	api_vec_u16 u16s;
	api_vec_u32 u32s;
	api_vec_u64 u64s;
	api_vec_i8 i8s;
	api_vec_i16 i16s;
	api_vec_i32 i32s;
	api_vec_i64 i64s;
	api_vec_f32 f32s;
	api_vec_f64 f64s;
	api_vec_i32 some;
} api_Numbers;
typedef struct api_Measure {
	double value;
	api_Numbers numbers;
} api_Measure;
typedef struct api_Limits_bytes {
	api_buf low;
	api_buf high;
} api_Limits_bytes;
typedef struct api_Limits_duration {
	api_duration low;
	api_duration high;
} api_Limits_duration;
typedef struct api_Page {
	uint32_t size;
} api_Page;
typedef struct api_Booth {
	api_Mark *mark;
} api_Booth;
typedef struct api_Cancelled {
	uint8_t step;
} api_Cancelled;
typedef struct api_StartSeekable {
	uint64_t at;
} api_StartSeekable;
typedef struct api_DoubleArray {
	uint32_t len;
} api_DoubleArray;
typedef struct api_NonZeros {
	uint8_t u8;
	uint16_t u16;
	uint32_t u32;
	uint64_t u64;
	size_t usize;
	int8_t i8;
	int16_t i16;
	int32_t i32;
	int64_t i64;
	ptrdiff_t isize;
} api_NonZeros;
typedef struct api_a {
	uint8_t b;
} api_a;
typedef struct api_Qualifiers {
	double api;
	double kotlin;
} api_Qualifiers;
typedef struct api_Version {
	uint16_t major;
	uint16_t minor;
} api_Version;
typedef struct api_Spot {
	uint32_t x;
	uint32_t y;
} api_Spot;
typedef struct api_Deprecated {
	bool on;
	uint32_t lower;
	bool deprecated;
} api_Deprecated;
typedef struct api_Dial {
	uint64_t level;
	uint64_t before;
} api_Dial;
typedef struct api_Knob {
	uint64_t level;
} api_Knob;
typedef struct api_polar_Turn {
	uint8_t part;
} api_polar_Turn;
typedef struct api_macros_Streams {
	uint8_t stdin_;
	uint8_t stdout_;
	uint8_t stderr_;
	uint8_t errno_;
} api_macros_Streams;
typedef struct api_clash_TONE {
	uint8_t db;
} api_clash_TONE;
typedef struct api_clash_Span {
	uint32_t width;
} api_clash_Span;
typedef struct api_clash_Any {
	api_buf type_url;
} api_clash_Any;
typedef struct api_clash_Break {
	uint8_t _0;
} api_clash_Break;
typedef struct api_clash_Escaped {
	uint8_t new_;
} api_clash_Escaped;
typedef struct api_tls_policy_Lifetime {
	uint32_t seconds;
} api_tls_policy_Lifetime;
typedef struct api_Core {
	uint32_t id;
} api_Core;
typedef enum api_Array_tag {
	API_ARRAY_U8X4 = 0,
} api_Array_tag;
typedef struct api_Array_U8x4 {
	uint8_t _0;
} api_Array_U8x4;
typedef struct api_Array {
	api_Array_tag tag;
	union {
		api_Array_U8x4 u8x4;
	};
} api_Array;
typedef enum api_Platform_tag {
	API_PLATFORM_WASM = 0,
	API_PLATFORM_NATIVE = 1,
} api_Platform_tag;
typedef struct api_Platform_Wasm {
	uint32_t bits;
} api_Platform_Wasm;
typedef struct api_Platform_Native {
	uint32_t _0;
} api_Platform_Native;
typedef struct api_Platform {
	api_Platform_tag tag;
	union {
		api_Platform_Wasm wasm;
		api_Platform_Native native;
	};
} api_Platform;
typedef enum api_Placement_tag {
	API_PLACEMENT_ANYWHERE = 0,
	API_PLACEMENT_PINNED = 1,
} api_Placement_tag;
typedef struct api_Placement_Pinned {
	api_Core _0;
} api_Placement_Pinned;
typedef struct api_Placement {
	api_Placement_tag tag;
	union {
		api_Placement_Pinned pinned;
	};
} api_Placement;
typedef enum api_Event_tag {
	API_EVENT_MOVED = 0,
	API_EVENT_NAMED = 1,
	API_EVENT_DONE = 2,
} api_Event_tag;
typedef struct api_Event_Moved {
	api_Point _0;
} api_Event_Moved;
typedef struct api_Event_Named {
	api_buf name;
} api_Event_Named;
typedef struct api_Event {
	api_Event_tag tag;
	union {
		api_Event_Moved moved;
		api_Event_Named named;
	};
} api_Event;
typedef enum api_Clash_tag {
	API_CLASH_POINT = 0,
	API_CLASH_SHAPE = 1,
	API_CLASH_EVENT = 2,
	API_CLASH_DOUBLE = 3,
	API_CLASH_FLOAT = 4,
	API_CLASH_STRING = 5,
	API_CLASH_INT = 6,
	API_CLASH_BOOLEAN = 7,
	API_CLASH_NATIVE = 8,
	API_CLASH_BYTES = 9,
	API_CLASH_ANY = 10,
	API_CLASH_UNIT = 11,
	API_CLASH_WAKER = 12,
	API_CLASH_MUT = 13,
	API_CLASH_PAIR = 14,
	API_CLASH_STAMP = 15,
	API_CLASH_MOMENT = 16,
} api_Clash_tag;
typedef struct api_Clash_Point {
	api_Point _0;
} api_Clash_Point;
typedef struct api_Clash_Shape {
	api_Shape _0;
} api_Clash_Shape;
typedef struct api_Clash_Event {
	api_Event _0;
} api_Clash_Event;
typedef struct api_Clash_Double {
	double _0;
} api_Clash_Double;
typedef struct api_Clash_Float {
	float _0;
} api_Clash_Float;
typedef struct api_Clash_String {
	api_buf _0;
} api_Clash_String;
typedef struct api_Clash_Int {
	int32_t _0;
} api_Clash_Int;
typedef struct api_Clash_Boolean {
	bool _0;
} api_Clash_Boolean;
typedef struct api_Clash_Native {
	uint32_t _0;
} api_Clash_Native;
typedef struct api_Clash_Bytes {
	api_buf _0;
} api_Clash_Bytes;
typedef struct api_Clash_Pair {
	uint8_t _0;
	uint8_t _1;
} api_Clash_Pair;
typedef struct api_Clash_Stamp {
	int64_t _0;
} api_Clash_Stamp;
typedef struct api_Clash_Moment {
	api_Moment _0;
} api_Clash_Moment;
typedef struct api_Clash {
	api_Clash_tag tag;
	union {
		api_Clash_Point point;
		api_Clash_Shape shape;
		api_Clash_Event event;
		api_Clash_Double double_;
		api_Clash_Float float_;
		api_Clash_String string;
		api_Clash_Int int_;
		api_Clash_Boolean boolean;
		api_Clash_Native native;
		api_Clash_Bytes bytes;
		api_Clash_Pair pair;
		api_Clash_Stamp stamp;
		api_Clash_Moment moment;
	};
} api_Clash;
typedef enum api_Stage_tag {
	API_STAGE_INIT = 0,
	API_STAGE_DONE = 1,
} api_Stage_tag;
typedef struct api_Stage_Init {
	uint32_t _0;
} api_Stage_Init;
typedef struct api_Stage {
	api_Stage_tag tag;
	union {
		api_Stage_Init init;
	};
} api_Stage;
typedef enum api_Error_tag {
	API_ERROR_UNDERFLOW = 0,
	API_ERROR_OVERFLOW = 1,
} api_Error_tag;
typedef struct api_Error_Underflow {
	uint64_t by;
} api_Error_Underflow;
typedef struct api_Error {
	api_Error_tag tag;
	union {
		api_Error_Underflow underflow;
	};
} api_Error;
typedef enum api_Labelled_tag {
	API_LABELLED_TAGGED = 0,
	API_LABELLED_BARE = 1,
} api_Labelled_tag;
typedef struct api_Labelled_Tagged {
	uint32_t tag;
} api_Labelled_Tagged;
typedef struct api_Labelled {
	api_Labelled_tag tag;
	union {
		api_Labelled_Tagged tagged;
	};
} api_Labelled;
typedef enum api_b_tag {
	API_B_ONE = 0,
	API_B_TWO = 1,
} api_b_tag;
typedef struct api_b_Two {
	uint8_t _0;
} api_b_Two;
typedef struct api_b {
	api_b_tag tag;
	union {
		api_b_Two two;
	};
} api_b;
typedef enum api_Builtin_tag {
	API_BUILTIN_ANY = 0,
	API_BUILTIN_BOOLEAN = 1,
	API_BUILTIN_INT = 2,
	API_BUILTIN_NATIVE = 3,
} api_Builtin_tag;
typedef struct api_Builtin_Any {
	api_vec_i32 _0;
} api_Builtin_Any;
typedef struct api_Builtin_Boolean {
	double _0;
} api_Builtin_Boolean;
typedef struct api_Builtin_Int {
	double _0;
} api_Builtin_Int;
typedef struct api_Builtin_Native {
	double _0;
} api_Builtin_Native;
typedef struct api_Builtin {
	api_Builtin_tag tag;
	union {
		api_Builtin_Any any;
		api_Builtin_Boolean boolean;
		api_Builtin_Int int_;
		api_Builtin_Native native;
	};
} api_Builtin;
typedef enum api_Rank_tag {
	API_RANK_UNRANKED = 0,
	API_RANK_RANKED = 1,
} api_Rank_tag;
typedef struct api_Rank_Ranked {
	uint8_t compare;
	uint8_t partial_compare;
	uint8_t compare_to;
	uint8_t partial_compare_to;
} api_Rank_Ranked;
typedef struct api_Rank {
	api_Rank_tag tag;
	union {
		api_Rank_Ranked ranked;
	};
} api_Rank;
typedef enum api_clash_Candidate_tag {
	API_CLASH_CANDIDATE_NONE = 0,
	API_CLASH_CANDIDATE_MATCH = 1,
} api_clash_Candidate_tag;
typedef struct api_clash_Candidate_Match {
	uint8_t _0;
} api_clash_Candidate_Match;
typedef struct api_clash_Candidate {
	api_clash_Candidate_tag tag;
	union {
		api_clash_Candidate_Match match;
	};
} api_clash_Candidate;
typedef struct api_Log_vtable {
	void (*free)(void *data);
	bool (*log)(void *data, uint8_t level, api_buf line);
} api_Log_vtable;
typedef struct api_Log {
	const api_Log_vtable *vtable;
	void *data;
} api_Log;
typedef struct api_Scan_vtable {
	void (*free)(void *data);
	uint64_t (*scan)(void *data, api_buf payload, api_buf rest);
} api_Scan_vtable;
typedef struct api_Scan {
	const api_Scan_vtable *vtable;
	void *data;
} api_Scan;
typedef struct api_Beat_vtable {
	void (*free)(void *data);
	uint32_t (*beat)(void *data);
} api_Beat_vtable;
typedef struct api_Beat {
	const api_Beat_vtable *vtable;
	void *data;
} api_Beat;
typedef struct api_Turner_vtable {
	void (*free)(void *data);
	uint64_t (*by)(void *data);
	uint64_t (*once)(void *data);
} api_Turner_vtable;
typedef struct api_Turner {
	const api_Turner_vtable *vtable;
	void *data;
} api_Turner;
#define API_LIMIT ((uint64_t)1000ULL)
#define API_FAR ((int64_t)9223372036854775807LL)
#define API_URL ((uint64_t)1000ULL)
#define API_STEP ((uint64_t)1ULL)
void api_Parcel_free(api_Parcel *self);
void api_Weighted_free(api_Weighted *self);
uint64_t api_Weighted_total(const api_Weighted *self);
void api_Stamped_free(api_Stamped *self);
uint64_t api_Stamped_total(const api_Stamped *self);
void api_CancellationError_free(api_CancellationError *self);
void api_Packet_free(api_Packet *self);
void api_Hasher_free(api_Hasher *self);
void api_Size_free(api_Size *self);
bool api_Size_eq(const api_Size *a, const api_Size *b);
API_MUST_USE api_buf api_Size_debug(const api_Size *self);
void api_Point_free(api_Point *self);
bool api_Point_eq(const api_Point *a, const api_Point *b);
API_MUST_USE api_buf api_Point_debug(const api_Point *self);
api_Point api_Point_default(void);
api_Point api_Point_offset(const api_Point *self, int32_t dx, int32_t dy);
int32_t api_Point_get_x(void);
void api_Label_free(api_Label *self);
bool api_Label_eq(const api_Label *a, const api_Label *b);
API_MUST_USE api_buf api_Label_debug(const api_Label *self);
void api_Note_free(api_Note *self);
bool api_Note_eq(const api_Note *a, const api_Note *b);
API_MUST_USE api_buf api_Note_debug(const api_Note *self);
API_MUST_USE api_Note api_Note_default(void);
void api_Reading_free(api_Reading *self);
bool api_Reading_eq(const api_Reading *a, const api_Reading *b);
API_MUST_USE api_buf api_Reading_debug(const api_Reading *self);
API_MUST_USE api_option_Reading api_Reading_next(const api_Reading *self);
void api_Packaged_free(api_Packaged *self);
bool api_Packaged_eq(const api_Packaged *a, const api_Packaged *b);
API_MUST_USE api_buf api_Packaged_debug(const api_Packaged *self);
api_Point api_Packaged_point(const api_Packaged *self);
api_Shape api_Packaged_shape(const api_Packaged *self);
void api_Atlas_free(api_Atlas *self);
bool api_Atlas_eq(const api_Atlas *a, const api_Atlas *b);
API_MUST_USE api_buf api_Atlas_debug(const api_Atlas *self);
API_MUST_USE api_Atlas api_Atlas_with(const api_Atlas *self, api_buf name, api_Point at);
void api_Span_free(api_Span *self);
bool api_Span_eq(const api_Span *a, const api_Span *b);
API_MUST_USE api_buf api_Span_debug(const api_Span *self);
void api_Window_free(api_Window *self);
bool api_Window_eq(const api_Window *a, const api_Window *b);
API_MUST_USE api_buf api_Window_debug(const api_Window *self);
bool api_Window_contains(const api_Window *self, uint64_t n);
bool api_Ticket_eq(const api_Ticket *a, const api_Ticket *b);
API_MUST_USE api_buf api_Ticket_debug(const api_Ticket *self);
api_Ticket api_Ticket_new(uint32_t number);
uint32_t api_Ticket_number(const api_Ticket *self);
uint32_t api_Ticket_clones(void);
void api_Seat_free(api_Seat *self);
bool api_Seat_eq(const api_Seat *a, const api_Seat *b, api_status *status);
API_MUST_USE api_buf api_Seat_debug(const api_Seat *self, api_status *status);
uint64_t api_Seat_count(const api_Seat *self, api_status *status);
bool api_Stamp_eq(const api_Stamp *a, const api_Stamp *b);
API_MUST_USE api_buf api_Stamp_debug(const api_Stamp *self);
api_Stamp api_Stamp_from_millis(uint64_t millis);
uint64_t api_Stamp_secs(const api_Stamp *self);
api_Stamp api_Stamp_whole(const api_Stamp *self);
bool api_Stamp_later(const api_Stamp *self, api_Stamp other);
bool api_Moment_eq(const api_Moment *a, const api_Moment *b);
API_MUST_USE api_buf api_Moment_debug(const api_Moment *self);
api_option_Moment api_Moment_new(uint64_t ticks, uint64_t rate);
uint64_t api_Moment_millis(const api_Moment *self);
api_Moment api_Moment_after(const api_Moment *self, uint64_t ticks);
bool api_LongArray_eq(const api_LongArray *a, const api_LongArray *b);
API_MUST_USE api_buf api_LongArray_debug(const api_LongArray *self);
api_LongArray api_LongArray_new(uint64_t low, uint64_t high);
uint64_t api_LongArray_c_(const api_LongArray *self);
void api_LongArray_also(const api_LongArray *self, void (*f)(void *data, uint64_t), void *f_data);
void api_JvmField_free(api_JvmField *self);
bool api_JvmField_eq(const api_JvmField *a, const api_JvmField *b);
API_MUST_USE api_buf api_JvmField_debug(const api_JvmField *self);
void api_JvmStatic_free(api_JvmStatic *self);
bool api_JvmStatic_eq(const api_JvmStatic *a, const api_JvmStatic *b);
API_MUST_USE api_buf api_JvmStatic_debug(const api_JvmStatic *self);
void api_OptIn_free(api_OptIn *self);
bool api_OptIn_eq(const api_OptIn *a, const api_OptIn *b);
API_MUST_USE api_buf api_OptIn_debug(const api_OptIn *self);
bool api_Wide_eq(const api_Wide *a, const api_Wide *b);
API_MUST_USE api_buf api_Wide_debug(const api_Wide *self);
api_Wide api_Wide_new(uint64_t seed);
uint64_t api_Wide_sum(const api_Wide *self);
void api_Clocked_free(api_Clocked *self);
bool api_Clocked_eq(const api_Clocked *a, const api_Clocked *b);
API_MUST_USE api_buf api_Clocked_debug(const api_Clocked *self);
bool api_Rated_eq(const api_Rated *a, const api_Rated *b);
API_MUST_USE api_buf api_Rated_debug(const api_Rated *self);
api_Rated api_Rated_new(uint64_t count, uint64_t scale);
uint64_t api_Rated_scale(const api_Rated *self);
API_MUST_USE api_buf api_Score_debug(const api_Score *self);
api_Score api_Score_of(uint64_t count);
uint64_t api_Score_count(const api_Score *self);
void api_Score_add(api_Score *self, api_Score other);
api_Score api_Score_doubled(const api_Score *self);
uint64_t api_Score_take(api_Score *self);
uint64_t api_Score_note(api_Score *self, const api_Mark *mark, api_status *status);
void api_Score_absorb(api_Score *self, api_Score other, api_Score *other_out, uint64_t *out0, uint64_t *out1);
bool api_Hits_eq(const api_Hits *a, const api_Hits *b);
API_MUST_USE api_buf api_Hits_debug(const api_Hits *self);
api_Hits api_Hits_default(void);
uint64_t api_Hits_hits_get(const api_Hits *self);
void api_Hits_hits_set(api_Hits *self, uint64_t value);
uint64_t api_Hits_misses_get(const api_Hits *self);
void api_Hits_misses_set(api_Hits *self, uint64_t value);
api_Hits api_Hits_of(uint64_t hits, uint64_t misses);
void api_Hits_add(api_Hits *self, api_Hits other);
bool api_Hits_eq_(api_Hits *self, api_Hits other);
bool api_Extent_eq(const api_Extent *a, const api_Extent *b);
API_MUST_USE api_buf api_Extent_debug(const api_Extent *self);
uint64_t api_Extent_start_get(const api_Extent *self);
void api_Extent_start_set(api_Extent *self, uint64_t value);
api_Extent api_Extent_new(uint64_t start, uint64_t len);
uint64_t api_Extent_end(const api_Extent *self);
uint64_t api_Extent_slide(api_Extent *self, uint64_t by);
void api_Pair_Size_u8_free(api_Pair_Size_u8 *self);
bool api_Pair_Size_u8_eq(const api_Pair_Size_u8 *a, const api_Pair_Size_u8 *b);
API_MUST_USE api_buf api_Pair_Size_u8_debug(const api_Pair_Size_u8 *self);
api_Pair_u8_Size api_Pair_Size_u8_swap(const api_Pair_Size_u8 *self);
void api_Pair_non_zero_u16_free(api_Pair_non_zero_u16 *self);
bool api_Pair_non_zero_u16_eq(const api_Pair_non_zero_u16 *a, const api_Pair_non_zero_u16 *b);
API_MUST_USE api_buf api_Pair_non_zero_u16_debug(const api_Pair_non_zero_u16 *self);
api_Pair_non_zero_u16 api_Pair_non_zero_u16_swap(const api_Pair_non_zero_u16 *self);
bool api_Pair_non_zero_u16_even(const api_Pair_non_zero_u16 *self);
void api_Pair_u32_free(api_Pair_u32 *self);
bool api_Pair_u32_eq(const api_Pair_u32 *a, const api_Pair_u32 *b);
API_MUST_USE api_buf api_Pair_u32_debug(const api_Pair_u32 *self);
api_Pair_u32 api_Pair_u32_swap(const api_Pair_u32 *self);
bool api_Pair_u32_even(const api_Pair_u32 *self);
uint32_t api_Pair_u32_sum(const api_Pair_u32 *self);
api_Pair_u8_Size api_Pair_u32_beside_Size(const api_Pair_u32 *self, api_Size right);
void api_Pair_u8_Size_free(api_Pair_u8_Size *self);
bool api_Pair_u8_Size_eq(const api_Pair_u8_Size *a, const api_Pair_u8_Size *b);
API_MUST_USE api_buf api_Pair_u8_Size_debug(const api_Pair_u8_Size *self);
api_Pair_Size_u8 api_Pair_u8_Size_swap(const api_Pair_u8_Size *self);
void api_Left_Right_u8_free(api_Left_Right_u8 *self);
bool api_Left_Right_u8_eq(const api_Left_Right_u8 *a, const api_Left_Right_u8 *b);
API_MUST_USE api_buf api_Left_Right_u8_debug(const api_Left_Right_u8 *self);
void api_Left_u8_free(api_Left_u8 *self);
bool api_Left_u8_eq(const api_Left_u8 *a, const api_Left_u8 *b);
API_MUST_USE api_buf api_Left_u8_debug(const api_Left_u8 *self);
void api_Right_Left_u8_free(api_Right_Left_u8 *self);
bool api_Right_Left_u8_eq(const api_Right_Left_u8 *a, const api_Right_Left_u8 *b);
API_MUST_USE api_buf api_Right_Left_u8_debug(const api_Right_Left_u8 *self);
void api_Right_u8_free(api_Right_u8 *self);
bool api_Right_u8_eq(const api_Right_u8 *a, const api_Right_u8 *b);
API_MUST_USE api_buf api_Right_u8_debug(const api_Right_u8 *self);
void api_Inner_u8_free(api_Inner_u8 *self);
bool api_Inner_u8_eq(const api_Inner_u8 *a, const api_Inner_u8 *b);
API_MUST_USE api_buf api_Inner_u8_debug(const api_Inner_u8 *self);
void api_Boxed_u16_free(api_Boxed_u16 *self);
bool api_Boxed_u16_eq(const api_Boxed_u16 *a, const api_Boxed_u16 *b);
API_MUST_USE api_buf api_Boxed_u16_debug(const api_Boxed_u16 *self);
void api_Boxed_u8_free(api_Boxed_u8 *self);
bool api_Boxed_u8_eq(const api_Boxed_u8 *a, const api_Boxed_u8 *b);
API_MUST_USE api_buf api_Boxed_u8_debug(const api_Boxed_u8 *self);
void api_LogCount_free(api_LogCount *self);
bool api_LogCount_eq(const api_LogCount *a, const api_LogCount *b);
API_MUST_USE api_buf api_LogCount_debug(const api_LogCount *self);
void api_Wrap_u32_free(api_Wrap_u32 *self);
bool api_Wrap_u32_eq(const api_Wrap_u32 *a, const api_Wrap_u32 *b);
API_MUST_USE api_buf api_Wrap_u32_debug(const api_Wrap_u32 *self);
void api_PolarTurn_free(api_PolarTurn *self);
bool api_PolarTurn_eq(const api_PolarTurn *a, const api_PolarTurn *b);
API_MUST_USE api_buf api_PolarTurn_debug(const api_PolarTurn *self);
void api_Peer_free(api_Peer *self);
bool api_Peer_eq(const api_Peer *a, const api_Peer *b);
API_MUST_USE api_buf api_Peer_debug(const api_Peer *self);
API_MUST_USE api_vec_string api_Peer_aliases(const api_Peer *self);
API_MUST_USE api_Peer api_Peer_next(const api_Peer *self, api_buf name);
void api_Shadowed_free(api_Shadowed *self);
bool api_Shadowed_eq(const api_Shadowed *a, const api_Shadowed *b);
API_MUST_USE api_buf api_Shadowed_debug(const api_Shadowed *self);
void api_Numbers_free(api_Numbers *self);
bool api_Numbers_eq(const api_Numbers *a, const api_Numbers *b);
API_MUST_USE api_buf api_Numbers_debug(const api_Numbers *self);
API_MUST_USE api_Numbers api_Numbers_edges(void);
API_MUST_USE api_Numbers api_Numbers_zero(bool negative);
API_MUST_USE api_Numbers api_Numbers_reversed(const api_Numbers *self);
void api_Measure_free(api_Measure *self);
bool api_Measure_eq(const api_Measure *a, const api_Measure *b);
API_MUST_USE api_buf api_Measure_debug(const api_Measure *self);
API_MUST_USE api_Measure api_Measure_new(double value, bool negative);
void api_Limits_bytes_free(api_Limits_bytes *self);
bool api_Limits_bytes_eq(const api_Limits_bytes *a, const api_Limits_bytes *b);
API_MUST_USE api_buf api_Limits_bytes_debug(const api_Limits_bytes *self);
void api_Limits_duration_free(api_Limits_duration *self);
bool api_Limits_duration_eq(const api_Limits_duration *a, const api_Limits_duration *b);
API_MUST_USE api_buf api_Limits_duration_debug(const api_Limits_duration *self);
void api_Page_free(api_Page *self);
bool api_Page_eq(const api_Page *a, const api_Page *b);
API_MUST_USE api_buf api_Page_debug(const api_Page *self);
api_Page api_Page_default(void);
void api_Booth_free(api_Booth *self);
bool api_Booth_eq(const api_Booth *a, const api_Booth *b, api_status *status);
API_MUST_USE api_buf api_Booth_debug(const api_Booth *self, api_status *status);
void api_Cancelled_free(api_Cancelled *self);
bool api_Cancelled_eq(const api_Cancelled *a, const api_Cancelled *b);
API_MUST_USE api_buf api_Cancelled_debug(const api_Cancelled *self);
void api_StartSeekable_free(api_StartSeekable *self);
bool api_StartSeekable_eq(const api_StartSeekable *a, const api_StartSeekable *b);
API_MUST_USE api_buf api_StartSeekable_debug(const api_StartSeekable *self);
void api_DoubleArray_free(api_DoubleArray *self);
bool api_DoubleArray_eq(const api_DoubleArray *a, const api_DoubleArray *b);
API_MUST_USE api_buf api_DoubleArray_debug(const api_DoubleArray *self);
void api_NonZeros_free(api_NonZeros *self);
API_MUST_USE api_buf api_NonZeros_debug(const api_NonZeros *self);
void api_a_free(api_a *self);
bool api_a_eq(const api_a *a, const api_a *b);
API_MUST_USE api_buf api_a_debug(const api_a *self);
void api_Qualifiers_free(api_Qualifiers *self);
bool api_Qualifiers_eq(const api_Qualifiers *a, const api_Qualifiers *b);
void api_Version_free(api_Version *self);
bool api_Version_eq(const api_Version *a, const api_Version *b);
int8_t api_Version_partial_cmp(const api_Version *a, const api_Version *b);
int8_t api_Version_cmp(const api_Version *a, const api_Version *b);
API_MUST_USE api_buf api_Version_debug(const api_Version *self);
void api_Spot_free(api_Spot *self);
bool api_Spot_eq(const api_Spot *a, const api_Spot *b);
API_MUST_USE api_buf api_Spot_debug(const api_Spot *self);
API_MUST_USE bool api_Spot_deserialize(api_Json *deserializer, api_Spot *out, api_buf *err, api_status *status);
void api_Deprecated_free(api_Deprecated *self);
bool api_Deprecated_eq(const api_Deprecated *a, const api_Deprecated *b);
API_MUST_USE api_buf api_Deprecated_debug(const api_Deprecated *self);
api_Deprecated api_Deprecated_default(void);
void api_Dial_free(api_Dial *self);
bool api_Dial_eq(const api_Dial *a, const api_Dial *b);
API_MUST_USE api_buf api_Dial_debug(const api_Dial *self);
api_Dial api_Dial_default(void);
API_MUST_USE bool api_Dial_turn(const api_Dial *self, uint64_t by, api_Dial *out, api_Error *err);
api_Dial api_Dial_max(const api_Dial *self);
void api_Knob_free(api_Knob *self);
bool api_Knob_eq(const api_Knob *a, const api_Knob *b);
API_MUST_USE api_buf api_Knob_debug(const api_Knob *self);
api_Knob api_Knob_default(void);
uint64_t api_Knob_doubled(const api_Knob *self);
void api_polar_Point_free(api_polar_Point *self);
bool api_polar_Point_eq(const api_polar_Point *a, const api_polar_Point *b);
API_MUST_USE api_buf api_polar_Point_debug(const api_polar_Point *self);
void api_polar_Turn_free(api_polar_Turn *self);
bool api_polar_Turn_eq(const api_polar_Turn *a, const api_polar_Turn *b);
API_MUST_USE api_buf api_polar_Turn_debug(const api_polar_Turn *self);
void api_macros_Streams_free(api_macros_Streams *self);
bool api_macros_Streams_eq(const api_macros_Streams *a, const api_macros_Streams *b);
API_MUST_USE api_buf api_macros_Streams_debug(const api_macros_Streams *self);
void api_clash_Tone_free(api_clash_Tone *self);
bool api_clash_Tone_eq(const api_clash_Tone *a, const api_clash_Tone *b);
API_MUST_USE api_buf api_clash_Tone_debug(const api_clash_Tone *self);
void api_clash_TONE_free(api_clash_TONE *self);
bool api_clash_TONE_eq(const api_clash_TONE *a, const api_clash_TONE *b);
API_MUST_USE api_buf api_clash_TONE_debug(const api_clash_TONE *self);
void api_clash_Span_free(api_clash_Span *self);
bool api_clash_Span_eq(const api_clash_Span *a, const api_clash_Span *b);
API_MUST_USE api_buf api_clash_Span_to_string(const api_clash_Span *self);
API_MUST_USE api_buf api_clash_Span_debug(const api_clash_Span *self);
uint32_t api_clash_Span_width(const api_clash_Span *self);
API_MUST_USE api_buf api_clash_Span_to_string_(const api_clash_Span *self);
void api_clash_Any_free(api_clash_Any *self);
bool api_clash_Any_eq(const api_clash_Any *a, const api_clash_Any *b);
uint64_t api_clash_Any_hash(const api_clash_Any *self);
API_MUST_USE api_buf api_clash_Any_debug(const api_clash_Any *self);
void api_clash_Break_free(api_clash_Break *self);
bool api_clash_Break_eq(const api_clash_Break *a, const api_clash_Break *b);
API_MUST_USE api_buf api_clash_Break_debug(const api_clash_Break *self);
api_clash_Break api_clash_Break_Sp(void);
api_clash_Break api_clash_Break_SP(void);
void api_clash_Escaped_free(api_clash_Escaped *self);
bool api_clash_Escaped_eq(const api_clash_Escaped *a, const api_clash_Escaped *b);
API_MUST_USE api_buf api_clash_Escaped_debug(const api_clash_Escaped *self);
uint8_t api_clash_Escaped_delete(const api_clash_Escaped *self);
uint8_t api_clash_Escaped_delete_(const api_clash_Escaped *self, uint8_t new_, uint8_t new__);
void api_tls_policy_Lifetime_free(api_tls_policy_Lifetime *self);
bool api_tls_policy_Lifetime_eq(const api_tls_policy_Lifetime *a, const api_tls_policy_Lifetime *b);
API_MUST_USE api_buf api_tls_policy_Lifetime_debug(const api_tls_policy_Lifetime *self);
void api_Core_free(api_Core *self);
void api_Choice_free(api_Choice *self);
void api_Array_free(api_Array *self);
void api_Platform_free(api_Platform *self);
void api_Placement_free(api_Placement *self);
bool api_Shape_eq(const api_Shape *a, const api_Shape *b);
API_MUST_USE api_buf api_Shape_debug(const api_Shape *self);
api_option_Shape api_Shape_value_of(api_buf name);
API_MUST_USE api_vec_Shape api_Shape_ALL(void);
API_MUST_USE api_vec_Shape api_Shape_BOTH(void);
void api_Event_free(api_Event *self);
bool api_Event_eq(const api_Event *a, const api_Event *b);
API_MUST_USE api_buf api_Event_debug(const api_Event *self);
void api_Clash_free(api_Clash *self);
bool api_Clash_eq(const api_Clash *a, const api_Clash *b);
API_MUST_USE api_buf api_Clash_debug(const api_Clash *self);
api_option_Point api_Clash_point(const api_Clash *self);
API_MUST_USE api_Clash api_Clash_of(double value);
API_MUST_USE api_buf api_Clash_bytes(const api_Clash *self);
API_MUST_USE api_Reading api_Clash_reading(const api_Clash *self, uint32_t kotlin);
API_MUST_USE api_vec_vec_u32 api_Clash_listed(api_vec_u32 java, uint32_t kotlin, uint32_t *kotlin_out);
typedef struct api_Clash_after_future api_Clash_after_future;
API_MUST_USE api_Clash_after_future *api_Clash_after(api_foreign_future first, double kotlin);
API_MUST_USE api_poll api_Clash_after_start(api_foreign_future first, double kotlin, api_Clash *out, api_Clash_after_future **future);
API_MUST_USE api_poll api_Clash_after_poll(api_Clash_after_future *future, api_waker waker, api_Clash *out);
void api_Clash_after_future_free(api_Clash_after_future *future);
API_MUST_USE api_buf api_Clash_joined(const api_Clash *self, api_buf lent, api_buf owned, api_buf maybe);
typedef struct api_Clash_tagged_future api_Clash_tagged_future;
API_MUST_USE api_Clash_tagged_future *api_Clash_tagged(const api_Clash *self, api_buf tag);
API_MUST_USE api_poll api_Clash_tagged_start(const api_Clash *self, api_buf tag, api_buf *out, api_Clash_tagged_future **future);
API_MUST_USE api_poll api_Clash_tagged_poll(api_Clash_tagged_future *future, api_waker waker, api_buf *out);
void api_Clash_tagged_future_free(api_Clash_tagged_future *future);
api_Stamp api_Clash_stamp(const api_Clash *self, uint64_t kotlin);
api_Moment api_Clash_moment(const api_Clash *self, uint64_t kotlin);
API_MUST_USE api_Clash api_Clash_at(api_Point api);
API_MUST_USE api_buf api_Clash_describe(const api_Clash *self, api_buf text, uint32_t n, api_Shape shape, uint32_t out_, uint32_t *out__out);
void api_Stage_free(api_Stage *self);
bool api_Stage_eq(const api_Stage *a, const api_Stage *b);
API_MUST_USE api_buf api_Stage_debug(const api_Stage *self);
void api_Error_free(api_Error *self);
bool api_Error_eq(const api_Error *a, const api_Error *b);
API_MUST_USE api_buf api_Error_to_string(const api_Error *self);
API_MUST_USE api_buf api_Error_debug(const api_Error *self);
bool api_Status__eq(const api_Status_ *a, const api_Status_ *b);
API_MUST_USE api_buf api_Status__debug(const api_Status_ *self);
void api_Labelled_free(api_Labelled *self);
bool api_Labelled_eq(const api_Labelled *a, const api_Labelled *b);
API_MUST_USE api_buf api_Labelled_debug(const api_Labelled *self);
bool api_Family_eq(const api_Family *a, const api_Family *b);
API_MUST_USE api_buf api_Family_debug(const api_Family *self);
bool api_VecClashTone_eq(const api_VecClashTone *a, const api_VecClashTone *b);
API_MUST_USE api_buf api_VecClashTone_debug(const api_VecClashTone *self);
void api_b_free(api_b *self);
bool api_b_eq(const api_b *a, const api_b *b);
API_MUST_USE api_buf api_b_debug(const api_b *self);
void api_Builtin_free(api_Builtin *self);
bool api_Builtin_eq(const api_Builtin *a, const api_Builtin *b);
void api_Rank_free(api_Rank *self);
bool api_Rank_eq(const api_Rank *a, const api_Rank *b);
int8_t api_Rank_partial_cmp(const api_Rank *a, const api_Rank *b);
int8_t api_Rank_cmp(const api_Rank *a, const api_Rank *b);
API_MUST_USE api_buf api_Rank_debug(const api_Rank *self);
bool api_Tier_eq(const api_Tier *a, const api_Tier *b);
int8_t api_Tier_partial_cmp(const api_Tier *a, const api_Tier *b);
int8_t api_Tier_cmp(const api_Tier *a, const api_Tier *b);
API_MUST_USE api_buf api_Tier_debug(const api_Tier *self);
void api_clash_Candidate_free(api_clash_Candidate *self);
bool api_clash_Candidate_eq(const api_clash_Candidate *a, const api_clash_Candidate *b);
API_MUST_USE api_buf api_clash_Candidate_debug(const api_clash_Candidate *self);
bool api_Level_eq(const api_Level *a, const api_Level *b);
uint64_t api_Level_hash(const api_Level *self);
int8_t api_Level_partial_cmp(const api_Level *a, const api_Level *b);
int8_t api_Level_cmp(const api_Level *a, const api_Level *b);
API_MUST_USE api_buf api_Level_to_string(const api_Level *self);
void api_Board_free(api_Board *self);
void api_Board_drop(api_Board *self, api_status *status);
API_MUST_USE api_Board *api_Board_new(api_vec_tuple_string_Counter counters, api_status *status);
API_MUST_USE api_vec_tuple_string_Counter api_Board_into_counters(api_Board *self, api_status *status);
API_MUST_USE api_vec_tuple_string_Counter api_Board_ranked(const api_Board *self, api_status *status);
API_MUST_USE api_Counter *api_Board_best(const api_Board *self, api_status *status);
void api_Tab_free(api_Tab *self);
void api_Tab_drop(api_Tab *self, api_status *status);
API_MUST_USE api_Ledger *api_Tab_ledger_get(const api_Tab *self, api_status *status);
API_MUST_USE api_buf api_Tab_label_get(const api_Tab *self, api_status *status);
void api_Receipt_free(api_Receipt *self);
void api_Receipt_drop(api_Receipt *self, api_status *status);
API_MUST_USE api_Receipt *api_Receipt_clone(const api_Receipt *self, api_status *status);
uint32_t api_Receipt_count(const api_Receipt *self, api_status *status);
void api_Draft_free(api_Draft *self);
void api_Draft_drop(api_Draft *self, api_status *status);
bool api_Draft_edited(const api_Draft *self, api_status *status);
API_MUST_USE api_buf api_Draft_describe(const api_Draft *self, api_status *status);
uint32_t api_Draft_commit(api_Draft *self, api_status *status);
API_MUST_USE api_buf api_Draft_title_get(const api_Draft *self, api_status *status);
void api_Draft_title_set(api_Draft *self, api_buf value, api_status *status);
uint32_t api_Draft_lines(const api_Draft *self, api_status *status);
void api_Draft_write(api_Draft *self, api_buf line, api_status *status);
void api_Filed_Receipt_free(api_Filed_Receipt *self);
void api_Filed_Receipt_drop(api_Filed_Receipt *self, api_status *status);
API_MUST_USE api_Filed_Receipt *api_Filed_Receipt_clone(const api_Filed_Receipt *self, api_status *status);
uint32_t api_Filed_Receipt_count(const api_Filed_Receipt *self, api_status *status);
void api_Pad_u16_free(api_Pad_u16 *self);
API_MUST_USE api_Pad_u16 *api_Pad_u16_wide(void);
uint16_t api_Pad_u16_tag(const api_Pad_u16 *self);
uint16_t api_Pad_u16_title(const api_Pad_u16 *self);
API_MUST_USE api_buf api_Pad_u16_title_get(const api_Pad_u16 *self);
uint32_t api_Pad_u16_lines(const api_Pad_u16 *self);
API_MUST_USE api_buf api_Pad_u16_describe(const api_Pad_u16 *self);
void api_Pad_u8_free(api_Pad_u8 *self);
API_MUST_USE api_Pad_u8 *api_Pad_u8_small(void);
API_MUST_USE api_buf api_Pad_u8_describe(const api_Pad_u8 *self);
API_MUST_USE api_buf api_Pad_u8_title_get(const api_Pad_u8 *self);
uint32_t api_Pad_u8_lines(const api_Pad_u8 *self);
void api_Layered_free(api_Layered *self);
API_MUST_USE api_Layered *api_Layered_new(void);
API_MUST_USE api_buf api_Layered_title_get(const api_Layered *self);
uint32_t api_Layered_lines(const api_Layered *self);
void api_Marker_free(api_Marker *self);
void api_Marker_drop(api_Marker *self, api_status *status);
API_MUST_USE api_Marker *api_Marker_new(api_buf label, int32_t x, int32_t y);
API_MUST_USE api_buf api_Marker_label(const api_Marker *self, api_status *status);
int32_t api_Marker_x(const api_Marker *self, api_status *status);
int32_t api_Marker_y(void);
int32_t api_Marker_x_get(const api_Marker *self, api_status *status);
void api_Marker_x_set(api_Marker *self, int32_t value, api_status *status);
int32_t api_Marker_y_get(const api_Marker *self, api_status *status);
void api_Marker_y_set(api_Marker *self, int32_t value, api_status *status);
api_Point api_Marker_offset(const api_Marker *self, int32_t dx, int32_t dy, api_status *status);
void api_Batch_free(api_Batch *self);
void api_Batch_drop(api_Batch *self, api_status *status);
API_MUST_USE api_Batch *api_Batch_new(void);
bool api_Batch_push(api_Batch *self, uint32_t v, api_status *status);
size_t api_Batch_capacity(const api_Batch *self, api_status *status);
void api_Batch_2_free(api_Batch_2 *self);
void api_Batch_2_drop(api_Batch_2 *self, api_status *status);
API_MUST_USE api_Batch_2 *api_Batch_2_new(void);
bool api_Batch_2_push(api_Batch_2 *self, uint32_t v, api_status *status);
size_t api_Batch_2_capacity(const api_Batch_2 *self, api_status *status);
void api_Waiter_free(api_Waiter *self);
void api_Waiter_drop(api_Waiter *self, api_status *status);
API_MUST_USE api_Waiter *api_Waiter_new(api_waker waker_);
API_MUST_USE api_waker api_Waiter_waker(const api_Waiter *self, api_status *status);
API_MUST_USE api_waker api_Waiter_context(const api_Waiter *self, api_status *status);
typedef struct api_Waiter_poll_future_future api_Waiter_poll_future_future;
API_MUST_USE api_Waiter_poll_future_future *api_Waiter_poll_future(api_foreign_future future_);
API_MUST_USE api_poll api_Waiter_poll_future_start(api_foreign_future future_, api_Waiter_poll_future_future **future);
API_MUST_USE api_poll api_Waiter_poll_future_poll(api_Waiter_poll_future_future *future, api_waker waker);
void api_Waiter_poll_future_future_free(api_Waiter_poll_future_future *future);
void api_Feed_free(api_Feed *self);
void api_Feed_drop(api_Feed *self, api_status *status);
typedef struct api_Feed_next_future api_Feed_next_future;
API_MUST_USE api_Feed_next_future *api_Feed_next(api_Feed *self, api_status *status);
API_MUST_USE api_poll api_Feed_next_start(api_Feed *self, api_buf *out, api_Error *err, api_Feed_next_future **future, api_status *status);
API_MUST_USE api_poll api_Feed_next_poll(api_Feed_next_future *future, api_waker waker, api_buf *out, api_Error *err);
void api_Feed_next_future_free(api_Feed_next_future *future);
void api_Countdown_free(api_Countdown *self);
void api_Countdown_drop(api_Countdown *self, api_status *status);
API_MUST_USE api_Countdown *api_Countdown_new(uint32_t from);
API_MUST_USE api_Countdown *api_Countdown_told(uint32_t from, api_Log log);
typedef struct api_Countdown_next_future api_Countdown_next_future;
API_MUST_USE api_Countdown_next_future *api_Countdown_next(api_Countdown *self, api_status *status);
API_MUST_USE api_poll api_Countdown_next_start(api_Countdown *self, api_option_u32 *out, api_Countdown_next_future **future, api_status *status);
API_MUST_USE api_poll api_Countdown_next_poll(api_Countdown_next_future *future, api_waker waker, api_option_u32 *out);
void api_Countdown_next_future_free(api_Countdown_next_future *future);
void api_Retired_free(api_Retired *self);
void api_Retired_drop(api_Retired *self, api_status *status);
API_MUST_USE api_Retired *api_Retired_new(uint32_t from);
typedef struct api_Retired_next_future api_Retired_next_future;
API_MUST_USE api_Retired_next_future *api_Retired_next(api_Retired *self, api_status *status);
API_MUST_USE api_poll api_Retired_next_start(api_Retired *self, api_option_u32 *out, api_Retired_next_future **future, api_status *status);
API_MUST_USE api_poll api_Retired_next_poll(api_Retired_next_future *future, api_waker waker, api_option_u32 *out);
void api_Retired_next_future_free(api_Retired_next_future *future);
void api_Logger_free(api_Logger *self);
API_MUST_USE api_Logger *api_Logger_new(api_Log sink);
bool api_Logger_write(const api_Logger *self, api_buf line);
bool api_Logger_write_from_thread(const api_Logger *self, api_buf line);
void api_Journal_free(api_Journal *self);
void api_Journal_drop(api_Journal *self, api_status *status);
API_MUST_USE api_Journal *api_Journal_new(api_Log sink);
void api_Journal_redirect(api_Journal *self, api_Log sink, api_status *status);
bool api_Journal_write(const api_Journal *self, api_buf line, api_status *status);
void api_Local_free(api_Local *self);
void api_Local_drop(api_Local *self, api_status *status);
bool api_Local_eq(const api_Local *a, const api_Local *b, api_status *status);
uint64_t api_Local_hash(const api_Local *self, api_status *status);
API_MUST_USE api_Local *api_Local_default(void);
API_MUST_USE api_Local *api_Local_default_homed(void);
API_MUST_USE api_Local *api_Local_new(void);
API_MUST_USE api_Local *api_Local_new_homed(void);
uint32_t api_Local_bump(const api_Local *self, api_status *status);
API_MUST_USE api_Local *api_Local_share(const api_Local *self, api_status *status);
typedef struct api_Local_later_future api_Local_later_future;
API_MUST_USE api_Local_later_future *api_Local_later(const api_Local *self, api_status *status);
API_MUST_USE api_poll api_Local_later_start(const api_Local *self, uint32_t *out, api_Local_later_future **future, api_status *status);
API_MUST_USE api_poll api_Local_later_poll(api_Local_later_future *future, api_waker waker, uint32_t *out, api_status *status);
void api_Local_later_future_free(api_Local_later_future *future);
uint32_t api_Local_visit(const api_Local *self, uint32_t (*f)(void *data), void *f_data, api_status *status);
typedef struct api_Local_poll_count_future api_Local_poll_count_future;
API_MUST_USE api_Local_poll_count_future *api_Local_poll_count(const api_Local *self, api_status *status);
API_MUST_USE api_poll api_Local_poll_count_start(const api_Local *self, uint32_t *out, api_Local_poll_count_future **future, api_status *status);
API_MUST_USE api_poll api_Local_poll_count_poll(api_Local_poll_count_future *future, api_waker waker, uint32_t *out, api_status *status);
void api_Local_poll_count_future_free(api_Local_poll_count_future *future);
uint64_t api_Local_thread_number(const api_Local *self, api_status *status);
bool api_Local_on_own_thread(const api_Local *self, api_status *status);
void api_LocalWaiter_free(api_LocalWaiter *self);
void api_LocalWaiter_drop(api_LocalWaiter *self, api_status *status);
API_MUST_USE api_LocalWaiter *api_LocalWaiter_new(api_waker waker_);
API_MUST_USE api_LocalWaiter *api_LocalWaiter_new_homed(api_waker waker_);
typedef struct api_LocalWaiter_poll_ready_future api_LocalWaiter_poll_ready_future;
API_MUST_USE api_LocalWaiter_poll_ready_future *api_LocalWaiter_poll_ready(uint32_t n);
API_MUST_USE api_poll api_LocalWaiter_poll_ready_start(uint32_t n, uint32_t *out, api_LocalWaiter_poll_ready_future **future);
API_MUST_USE api_poll api_LocalWaiter_poll_ready_poll(api_LocalWaiter_poll_ready_future *future, api_waker waker, uint32_t *out, api_status *status);
void api_LocalWaiter_poll_ready_future_free(api_LocalWaiter_poll_ready_future *future);
API_MUST_USE api_LocalWaiter_poll_ready_future *api_LocalWaiter_poll_ready_homed(uint32_t n);
API_MUST_USE api_poll api_LocalWaiter_poll_ready_start_homed(uint32_t n, uint32_t *out, api_LocalWaiter_poll_ready_future **future);
void api_Retry_free(api_Retry *self);
void api_Retry_drop(api_Retry *self, api_status *status);
api_option_u32 api_Retry_limit_get(const api_Retry *self, api_status *status);
void api_Retry_limit_set(api_Retry *self, api_option_u32 value, api_status *status);
api_option_u32 api_Retry_delay_get(const api_Retry *self, api_status *status);
void api_Retry_delay_set(api_Retry *self, api_option_u32 value, api_status *status);
API_MUST_USE api_Retry *api_Retry_new(api_option_u32 limit);
uint32_t api_Retry_limit(const api_Retry *self, api_status *status);
uint32_t api_Retry_delay(const api_Retry *self, api_status *status);
void api_Retry_set_limit(api_Retry *self, uint32_t limit, api_status *status);
void api_Digest_free(api_Digest *self);
void api_Digest_drop(api_Digest *self, api_status *status);
api_array_u8_4 api_Digest_sum_get(const api_Digest *self, api_status *status);
void api_Digest_sum_set(api_Digest *self, api_array_u8_4 value, api_status *status);
API_MUST_USE api_Digest *api_Digest_new(void);
api_array_u8_4 api_Digest_flipped(const api_Digest *self, api_array_u8_4 mask, api_status *status);
void api_Cursor_free(api_Cursor *self);
void api_Cursor_drop(api_Cursor *self, api_status *status);
uint64_t api_Cursor_at(const api_Cursor *self, api_status *status);
void api_Outer_Inner_u8_free(api_Outer_Inner_u8 *self);
API_MUST_USE api_Outer_Inner_u8 *api_Outer_Inner_u8_new(uint8_t value);
uint8_t api_Outer_Inner_u8_get(const api_Outer_Inner_u8 *self);
void api_Slot_u32_free(api_Slot_u32 *self);
void api_Slot_u32_drop(api_Slot_u32 *self, api_status *status);
uint32_t api_Slot_u32_value_get(const api_Slot_u32 *self, api_status *status);
void api_Slot_u32_value_set(api_Slot_u32 *self, uint32_t value, api_status *status);
API_MUST_USE api_Slot_u32 *api_Slot_u32_new(uint32_t value);
void api_Slot_u8_free(api_Slot_u8 *self);
void api_Slot_u8_drop(api_Slot_u8 *self, api_status *status);
uint8_t api_Slot_u8_value_get(const api_Slot_u8 *self, api_status *status);
void api_Slot_u8_value_set(api_Slot_u8 *self, uint8_t value, api_status *status);
API_MUST_USE api_Slot_u8 *api_Slot_u8_new(uint8_t value);
uint16_t api_Slot_u8_value(const api_Slot_u8 *self, api_status *status);
void api_Resolving_free(api_Resolving *self);
void api_Resolving_drop(api_Resolving *self, api_status *status);
API_MUST_USE api_tls_Resolver *api_Resolving_0_get(const api_Resolving *self, api_status *status);
void api_Resolving_0_set(api_Resolving *self, api_tls_Resolver *value, api_status *status);
API_MUST_USE api_Resolving *api_Resolving_new(api_buf name);
void api_Codecs_free(api_Codecs *self);
void api_Codecs_drop(api_Codecs *self, api_status *status);
API_MUST_USE api_Codecs *api_Codecs_default(void);
API_MUST_USE api_vec_tls_Codec api_Codecs_list_get(const api_Codecs *self, api_status *status);
void api_Codecs_list_set(api_Codecs *self, api_vec_tls_Codec value, api_status *status);
API_MUST_USE api_Codecs *api_Codecs_new(void);
uint32_t api_Codecs_ids(api_Codecs *self, api_status *status);
void api_Der_free(api_Der *self);
API_MUST_USE api_Der *api_Der_from_slice(api_buf bytes);
API_MUST_USE api_buf api_Der_bytes(const api_Der *self);
void api_Verbosity_free(api_Verbosity *self);
void api_Verbosity_drop(api_Verbosity *self, api_status *status);
api_Level api_Verbosity_level_get(const api_Verbosity *self, api_status *status);
void api_Verbosity_level_set(api_Verbosity *self, api_Level value, api_status *status);
API_MUST_USE api_Verbosity *api_Verbosity_new(api_Level level);
api_option_Level api_Verbosity_filter(const api_Verbosity *self, api_status *status);
void api_Site_free(api_Site *self);
void api_Site_drop(api_Site *self, api_status *status);
API_MUST_USE api_Url *api_Site_home_get(const api_Site *self, api_status *status);
void api_Site_home_set(api_Site *self, api_Url *value, api_status *status);
API_MUST_USE api_Site *api_Site_new(api_Url *home, api_status *status);
void api_Trail_free(api_Trail *self);
void api_Trail_drop(api_Trail *self, api_status *status);
API_MUST_USE api_Trail *api_Trail_new(api_vec_Point points, api_vec_string names);
API_MUST_USE api_vec_Point api_Trail_points(const api_Trail *self, api_status *status);
API_MUST_USE api_vec_Label api_Trail_labels(const api_Trail *self, api_status *status);
API_MUST_USE api_vec_Point api_Trail_iter(const api_Trail *self, api_status *status);
API_MUST_USE api_vec_string api_Trail_names(const api_Trail *self, api_status *status);
API_MUST_USE api_vec_Point api_Trail_beyond(const api_Trail *self, int32_t x, api_status *status);
API_MUST_USE api_buf api_Trail_bytes(const api_Trail *self, api_status *status);
API_MUST_USE api_vec_Label api_Trail_drain(api_Trail *self, api_status *status);
void api_Worker_free(api_Worker *self);
API_MUST_USE api_Worker *api_Worker_new(uint32_t id);
uint32_t api_Worker_id(const api_Worker *self);
void api_Handle_free(api_Handle *self);
API_MUST_USE api_Handle *api_Handle_new(uint32_t count);
uint32_t api_Handle_count(const api_Handle *self);
typedef struct api_Handle_later_future api_Handle_later_future;
API_MUST_USE api_Handle_later_future *api_Handle_later(uint32_t count);
API_MUST_USE api_poll api_Handle_later_start(uint32_t count, api_Handle **out, api_Handle_later_future **future);
API_MUST_USE api_poll api_Handle_later_poll(api_Handle_later_future *future, api_waker waker, api_Handle **out);
void api_Handle_later_future_free(api_Handle_later_future *future);
void api_Holder_free(api_Holder *self);
API_MUST_USE api_Holder *api_Holder_new(uint64_t level);
uint64_t api_Holder_level_get(const api_Holder *self);
uint64_t api_Holder_doubled(const api_Holder *self);
void api_Area_free(api_Area *self);
bool api_Area_borrowed(const api_Area *self);
API_MUST_USE api_Area *api_Area_clone(const api_Area *self);
API_MUST_USE api_buf api_Area_debug(const api_Area *self);
API_MUST_USE api_Area *api_Area_new(api_Size size);
api_Size api_Area_size(const api_Area *self);
uint64_t api_Area_pixels(const api_Area *self);
uint64_t api_Area_scaled_Point(const api_Area *self, api_Point by);
void api_Counter_free(api_Counter *self);
void api_Counter_drop(api_Counter *self, api_status *status);
API_MUST_USE api_Counter *api_Counter_clone(const api_Counter *self, api_status *status);
API_MUST_USE api_buf api_Counter_debug(const api_Counter *self, api_status *status);
API_MUST_USE api_Counter *api_Counter_default(void);
API_MUST_USE api_Counter *api_Counter_new(api_buf name, uint64_t start);
uint64_t api_Counter_get(const api_Counter *self, api_status *status);
uint64_t api_Counter_offset(const api_Counter *self, uint64_t handle, uint64_t h, api_status *status);
uint64_t api_Counter_into_sum(api_Counter *self, uint64_t close, api_status *status);
API_MUST_USE api_buf api_Counter_name(const api_Counter *self, api_status *status);
API_MUST_USE bool api_Counter_add(api_Counter *self, uint64_t n, uint64_t *out, api_Error *err, api_status *status);
API_MUST_USE bool api_Counter_sub(api_Counter *self, uint64_t n, uint64_t *out, api_Error *err, api_status *status);
bool api_Counter_edited(const api_Counter *self, api_status *status);
uint64_t api_Counter_plus(const api_Counter *self, const api_Counter *other, api_status *status);
uint64_t api_Counter_clamped(const api_Counter *self, const api_Counter *low, const api_Counter *high, api_status *status);
typedef struct api_Counter_plus_later_future api_Counter_plus_later_future;
API_MUST_USE api_Counter_plus_later_future *api_Counter_plus_later(const api_Counter *self, const api_Counter *other, api_status *status);
API_MUST_USE api_poll api_Counter_plus_later_start(const api_Counter *self, const api_Counter *other, uint64_t *out, api_Counter_plus_later_future **future, api_status *status);
API_MUST_USE api_poll api_Counter_plus_later_poll(api_Counter_plus_later_future *future, api_waker waker, uint64_t *out);
void api_Counter_plus_later_future_free(api_Counter_plus_later_future *future);
bool api_Counter_add_to(const api_Counter *self, api_Counter *other, api_status *status);
API_MUST_USE api_Label api_Counter_into_label(api_Counter *self, api_option_Point at, api_status *status);
void api_Counter_rename(api_Counter *self, api_buf name, api_status *status);
void api_Counter_split(api_Counter *self, uint64_t at, api_Counter **out0, api_Counter **out1, api_status *status);
bool api_Counter_is(const api_Counter *self, api_buf name, api_status *status);
API_MUST_USE api_Counter *api_Counter_seeded_Point(api_buf name, api_Point from);
API_MUST_USE api_Counter *api_Counter_seeded_Counter(api_buf name, const api_Counter *from, api_status *status);
API_MUST_USE api_Counter *api_Counter_seeded_Size(api_buf name, api_Size from);
API_MUST_USE api_Counter *api_Counter_seeded_Wrap_u32(api_buf name, api_Wrap_u32 from);
API_MUST_USE api_Counter *api_Counter_seeded_polar_Point(api_buf name, api_polar_Point from);
void api_Meter_free(api_Meter *self);
void api_Meter_drop(api_Meter *self, api_status *status);
API_MUST_USE api_Meter *api_Meter_default(void);
uint64_t api_Meter_add(const api_Meter *self, uint64_t n, api_status *status);
uint64_t api_Meter_count(const api_Meter *self, api_status *status);
void api_Meter_reset(api_Meter *self, api_status *status);
void api_Meters_free(api_Meters *self);
void api_Meters_drop(api_Meters *self, api_status *status);
API_MUST_USE api_Meters *api_Meters_default(void);
API_MUST_USE api_Meter *api_Meters_last_get(const api_Meters *self, api_status *status);
void api_Meters_last_set(api_Meters *self, api_Meter *value, api_status *status);
API_MUST_USE api_Meter *api_Meters_spare_get(const api_Meters *self, api_status *status);
void api_Meters_spare_set(api_Meters *self, api_Meter *value, api_status *status);
API_MUST_USE api_Meters *api_Meters_new(void);
API_MUST_USE api_Meter *api_Meters_first(const api_Meters *self, api_status *status);
void api_Meters_clear(api_Meters *self, api_status *status);
void api_Registry_free(api_Registry *self);
void api_Registry_drop(api_Registry *self, api_status *status);
API_MUST_USE api_Registry *api_Registry_default(void);
API_MUST_USE api_Counter *api_Registry_first_get(const api_Registry *self, api_status *status);
void api_Registry_first_set(api_Registry *self, api_Counter *value, api_status *status);
API_MUST_USE api_Registry *api_Registry_new(api_Counter *first, api_status *status);
API_MUST_USE api_Counter *api_Registry_last(const api_Registry *self, api_status *status);
void api_Registry_push(api_Registry *self, api_Counter *counter, api_status *status);
bool api_Registry_edit(api_Registry *self, bool (*edit)(void *data, api_Counter *), void *edit_data, api_status *status);
void api_Ledger_free(api_Ledger *self);
void api_Ledger_drop(api_Ledger *self, api_status *status);
API_MUST_USE api_Ledger *api_Ledger_default(void);
API_MUST_USE api_Ledger *api_Ledger_new(void);
API_MUST_USE bool api_Ledger_edit(api_Ledger *self, api_Draft **out, api_Error *err, api_status *status);
API_MUST_USE bool api_Ledger_edit_homed(api_Ledger *self, api_Draft **out, api_Error *err, api_status *status);
uint32_t api_Ledger_published(const api_Ledger *self, api_status *status);
API_MUST_USE api_buf api_Ledger_title(const api_Ledger *self, api_status *status);
void api_Ledger_seal(api_Ledger *self, api_status *status);
API_MUST_USE api_Receipt *api_Ledger_receipt(api_Ledger *self, api_status *status);
API_MUST_USE api_Receipt *api_Ledger_noted(api_Ledger *self, api_status *status);
API_MUST_USE api_Filed_Receipt *api_Ledger_file_Receipt(api_Ledger *self, api_Receipt *item, api_status *status);
API_MUST_USE api_Tab *api_Ledger_tab(api_Ledger *self, api_status *status);
API_MUST_USE api_Slip *api_Ledger_slip(api_Ledger *self, api_status *status);
API_MUST_USE api_Slip *api_Ledger_slip_homed(api_Ledger *self, api_status *status);
void api_Slip_free(api_Slip *self);
void api_Slip_drop(api_Slip *self, api_status *status);
API_MUST_USE api_Slip *api_Slip_clone(const api_Slip *self, api_status *status);
API_MUST_USE api_Receipt *api_Slip_receipt_get(const api_Slip *self, api_status *status);
void api_Slip_receipt_set(api_Slip *self, api_Receipt *value, api_status *status);
API_MUST_USE api_Receipt *api_Slip_global_get(const api_Slip *self, api_status *status);
void api_Slip_global_set(api_Slip *self, api_Receipt *value, api_status *status);
uint32_t api_Slip_count(const api_Slip *self, api_status *status);
void api_Sheet_free(api_Sheet *self);
void api_Sheet_drop(api_Sheet *self, api_status *status);
API_MUST_USE api_Sheet *api_Sheet_clone(const api_Sheet *self, api_status *status);
API_MUST_USE api_buf api_Sheet_debug(const api_Sheet *self, api_status *status);
API_MUST_USE api_Sheet *api_Sheet_default(void);
API_MUST_USE api_buf api_Sheet_title_get(const api_Sheet *self, api_status *status);
void api_Sheet_title_set(api_Sheet *self, api_buf value, api_status *status);
API_MUST_USE api_Sheet *api_Sheet_new(void);
uint32_t api_Sheet_lines(const api_Sheet *self, api_status *status);
void api_Sheet_write(api_Sheet *self, api_buf line, api_status *status);
API_MUST_USE api_buf api_Sheet_describe(const api_Sheet *self, api_status *status);
void api_Name_free(api_Name *self);
void api_Name_drop(api_Name *self, api_status *status);
API_MUST_USE api_Name *api_Name_clone(const api_Name *self, api_status *status);
bool api_Name_eq(const api_Name *a, const api_Name *b, api_status *status);
API_MUST_USE api_buf api_Name_debug(const api_Name *self, api_status *status);
API_MUST_USE api_Name *api_Name_owned(api_buf text);
API_MUST_USE api_buf api_Name_as_str(const api_Name *self, api_status *status);
API_MUST_USE api_Name *api_Name_join(const api_Name *self, const api_Name *other, api_status *status);
API_MUST_USE api_Name *api_Name_borrowed(api_buf text);
bool api_Name_shares(const api_Name *self, const api_Name *other, api_status *status);
void api_Name_rename(api_Name *self, api_buf text, api_status *status);
void api_Channel_free(api_Channel *self);
void api_Channel_drop(api_Channel *self, api_status *status);
API_MUST_USE api_Channel *api_Channel_clone(const api_Channel *self, api_status *status);
API_MUST_USE api_Channel *api_Channel_default(void);
API_MUST_USE api_Channel *api_Channel_new(void);
void api_Channel_clear(api_Channel *self, api_status *status);
void api_Channel_send(const api_Channel *self, api_buf payload, api_status *status);
typedef struct api_Channel_recv_future api_Channel_recv_future;
API_MUST_USE api_Channel_recv_future *api_Channel_recv(const api_Channel *self, api_status *status);
API_MUST_USE api_poll api_Channel_recv_start(const api_Channel *self, api_buf *out, api_Channel_recv_future **future, api_status *status);
API_MUST_USE api_poll api_Channel_recv_poll(api_Channel_recv_future *future, api_waker waker, api_buf *out);
void api_Channel_recv_future_free(api_Channel_recv_future *future);
void api_Channel_notify(const api_Channel *self, api_waker waker_, api_status *status);
typedef struct api_Channel_send_later_future api_Channel_send_later_future;
API_MUST_USE api_Channel_send_later_future *api_Channel_send_later(const api_Channel *self, api_buf payload, api_status *status);
API_MUST_USE api_poll api_Channel_send_later_start(const api_Channel *self, api_buf payload, api_Channel_send_later_future **future, api_status *status);
API_MUST_USE api_poll api_Channel_send_later_poll(api_Channel_send_later_future *future, api_waker waker);
void api_Channel_send_later_future_free(api_Channel_send_later_future *future);
typedef struct api_Channel_tagged_future api_Channel_tagged_future;
API_MUST_USE api_Channel_tagged_future *api_Channel_tagged(const api_Channel *self, api_buf tag, api_status *status);
API_MUST_USE api_poll api_Channel_tagged_start(const api_Channel *self, api_buf tag, api_buf *out, api_Channel_tagged_future **future, api_status *status);
API_MUST_USE api_poll api_Channel_tagged_poll(api_Channel_tagged_future *future, api_waker waker, api_buf *out);
void api_Channel_tagged_future_free(api_Channel_tagged_future *future);
typedef struct api_Channel_itemized_future api_Channel_itemized_future;
API_MUST_USE api_Channel_itemized_future *api_Channel_itemized(const api_Channel *self, api_vec_u32 items, api_status *status);
API_MUST_USE api_poll api_Channel_itemized_start(const api_Channel *self, api_vec_u32 items, size_t *out, api_Channel_itemized_future **future, api_status *status);
API_MUST_USE api_poll api_Channel_itemized_poll(api_Channel_itemized_future *future, api_waker waker, size_t *out);
void api_Channel_itemized_future_free(api_Channel_itemized_future *future);
typedef struct api_Channel_poll_take_future api_Channel_poll_take_future;
API_MUST_USE api_Channel_poll_take_future *api_Channel_poll_take(const api_Channel *self, api_status *status);
API_MUST_USE api_poll api_Channel_poll_take_start(const api_Channel *self, api_buf *out, api_Channel_poll_take_future **future, api_status *status);
API_MUST_USE api_poll api_Channel_poll_take_poll(api_Channel_poll_take_future *future, api_waker waker, api_buf *out);
void api_Channel_poll_take_future_free(api_Channel_poll_take_future *future);
typedef struct api_Channel_poll_measure_future api_Channel_poll_measure_future;
API_MUST_USE api_Channel_poll_measure_future *api_Channel_poll_measure(api_Channel *self, size_t extra, api_status *status);
API_MUST_USE api_poll api_Channel_poll_measure_start(api_Channel *self, size_t extra, size_t *out, api_Error *err, api_Channel_poll_measure_future **future, api_status *status);
API_MUST_USE api_poll api_Channel_poll_measure_poll(api_Channel_poll_measure_future *future, api_waker waker, size_t *out, api_Error *err);
void api_Channel_poll_measure_future_free(api_Channel_poll_measure_future *future);
void api_Channel_listen(const api_Channel *self, api_waker cx, api_status *status);
typedef struct api_Channel_recv_sized_future api_Channel_recv_sized_future;
API_MUST_USE api_Channel_recv_sized_future *api_Channel_recv_sized(const api_Channel *self, api_status *status);
API_MUST_USE api_poll api_Channel_recv_sized_start(const api_Channel *self, api_buf *out0, size_t *out1, api_Channel_recv_sized_future **future, api_status *status);
API_MUST_USE api_poll api_Channel_recv_sized_poll(api_Channel_recv_sized_future *future, api_waker waker, api_buf *out0, size_t *out1);
void api_Channel_recv_sized_future_free(api_Channel_recv_sized_future *future);
typedef struct api_Channel_recv_nonempty_future api_Channel_recv_nonempty_future;
API_MUST_USE api_Channel_recv_nonempty_future *api_Channel_recv_nonempty(const api_Channel *self, api_status *status);
API_MUST_USE api_poll api_Channel_recv_nonempty_start(const api_Channel *self, api_buf *out, api_Error *err, api_Channel_recv_nonempty_future **future, api_status *status);
API_MUST_USE api_poll api_Channel_recv_nonempty_poll(api_Channel_recv_nonempty_future *future, api_waker waker, api_buf *out, api_Error *err);
void api_Channel_recv_nonempty_future_free(api_Channel_recv_nonempty_future *future);
API_MUST_USE api_Feed *api_Channel_feed(const api_Channel *self, api_status *status);
void api_Channel_send_from_thread(const api_Channel *self, api_buf payload, api_status *status);
void api_Gap_free(api_Gap *self);
bool api_Gap_borrowed(const api_Gap *self);
API_MUST_USE api_Gap *api_Gap_clone(const api_Gap *self);
bool api_Gap_eq(const api_Gap *a, const api_Gap *b);
API_MUST_USE api_buf api_Gap_debug(const api_Gap *self);
API_MUST_USE api_Gap *api_Gap_new(uint64_t wide, uint8_t narrow);
uint64_t api_Gap_sum(const api_Gap *self);
void api_Berth_free(api_Berth *self);
bool api_Berth_borrowed(const api_Berth *self);
API_MUST_USE api_Berth *api_Berth_clone(const api_Berth *self);
API_MUST_USE api_buf api_Berth_debug(const api_Berth *self);
API_MUST_USE api_Berth *api_Berth_new(api_option_u32 at);
api_option_u32 api_Berth_at(const api_Berth *self);
void api_Mark_free(api_Mark *self);
void api_Mark_drop(api_Mark *self, api_status *status);
API_MUST_USE api_Mark *api_Mark_clone(const api_Mark *self, api_status *status);
bool api_Mark_eq(const api_Mark *a, const api_Mark *b, api_status *status);
API_MUST_USE api_buf api_Mark_debug(const api_Mark *self, api_status *status);
API_MUST_USE api_Mark *api_Mark_of(uint64_t count);
uint64_t api_Mark_count(const api_Mark *self, api_status *status);
void api_Mark_add(api_Mark *self, uint64_t by, api_status *status);
void api_Message_free(api_Message *self);
void api_Message_drop(api_Message *self, api_status *status);
API_MUST_USE api_Message *api_Message_clone(const api_Message *self, api_status *status);
API_MUST_USE api_buf api_Message_debug(const api_Message *self, api_status *status);
api_option_Stamp api_Message_stamp_get(const api_Message *self, api_status *status);
void api_Message_stamp_set(api_Message *self, api_option_Stamp value, api_status *status);
API_MUST_USE api_buf api_Message_payload_get(const api_Message *self, api_status *status);
void api_Message_payload_set(api_Message *self, api_buf value, api_status *status);
API_MUST_USE api_Message *api_Message_new(api_option_Stamp stamp, api_buf payload);
bool api_Message_sent(const api_Message *self, api_status *status);
void api_Node_free(api_Node *self);
void api_Node_drop(api_Node *self, api_status *status);
API_MUST_USE api_buf api_Node_debug(const api_Node *self, api_status *status);
uint64_t api_Node_value_get(const api_Node *self, api_status *status);
void api_Node_value_set(api_Node *self, uint64_t value, api_status *status);
API_MUST_USE api_Node *api_Node_next_get(const api_Node *self, api_status *status);
void api_Node_next_set(api_Node *self, api_Node *value, api_status *status);
API_MUST_USE api_Node *api_Node_new(uint64_t value, api_Node *next, api_status *status);
void api_Node_push(api_Node *self, uint64_t value, api_status *status);
uint64_t api_Node_sum(const api_Node *self, api_status *status);
void api_Tally_free(api_Tally *self);
void api_Tally_drop(api_Tally *self, api_status *status);
bool api_Tally_eq(const api_Tally *a, const api_Tally *b, api_status *status);
API_MUST_USE api_buf api_Tally_debug(const api_Tally *self, api_status *status);
API_MUST_USE api_Tally *api_Tally_default(void);
API_MUST_USE api_Tally *api_Tally_new(void);
uint32_t api_Tally_bump(const api_Tally *self, api_status *status);
typedef struct api_Tally_bump_later_future api_Tally_bump_later_future;
API_MUST_USE api_Tally_bump_later_future *api_Tally_bump_later(const api_Tally *self, api_status *status);
API_MUST_USE api_poll api_Tally_bump_later_start(const api_Tally *self, uint32_t *out, api_Tally_bump_later_future **future, api_status *status);
API_MUST_USE api_poll api_Tally_bump_later_poll(api_Tally_bump_later_future *future, api_waker waker, uint32_t *out, api_status *status);
void api_Tally_bump_later_future_free(api_Tally_bump_later_future *future);
API_MUST_USE api_Tally_bump_later_future *api_Tally_bump_later_homed(const api_Tally *self, api_status *status);
API_MUST_USE api_poll api_Tally_bump_later_start_homed(const api_Tally *self, uint32_t *out, api_Tally_bump_later_future **future, api_status *status);
void api_Tallies_free(api_Tallies *self);
void api_Tallies_drop(api_Tallies *self, api_status *status);
API_MUST_USE api_Tallies *api_Tallies_default(void);
API_MUST_USE api_Tallies *api_Tallies_new(void);
API_MUST_USE api_Tally *api_Tallies_first(const api_Tallies *self, api_status *status);
void api_Plain_free(api_Plain *self);
void api_Plain_drop(api_Plain *self, api_status *status);
bool api_Plain_eq(const api_Plain *a, const api_Plain *b, api_status *status);
uint64_t api_Plain_hash(const api_Plain *self, api_status *status);
API_MUST_USE api_buf api_Plain_debug(const api_Plain *self, api_status *status);
uint64_t api_Plain_n_get(const api_Plain *self, api_status *status);
void api_Plain_n_set(api_Plain *self, uint64_t value, api_status *status);
bool api_Plain_on_get(const api_Plain *self, api_status *status);
void api_Plain_on_set(api_Plain *self, bool value, api_status *status);
API_MUST_USE api_Plain *api_Plain_new(uint64_t n, bool on);
void api_Labeled_free(api_Labeled *self);
bool api_Labeled_eq(const api_Labeled *a, const api_Labeled *b);
uint64_t api_Labeled_hash(const api_Labeled *self);
API_MUST_USE api_buf api_Labeled_debug(const api_Labeled *self);
API_MUST_USE api_Labeled *api_Labeled_new(api_buf label);
void api_Gauge_free(api_Gauge *self);
void api_Gauge_drop(api_Gauge *self, api_status *status);
bool api_Gauge_borrowed(const api_Gauge *self);
API_MUST_USE api_Gauge *api_Gauge_clone(const api_Gauge *self, api_status *status);
API_MUST_USE api_Gauge *api_Gauge_new(uint64_t level);
void api_Gauge_set(const api_Gauge *self, uint64_t level, api_status *status);
uint64_t api_Gauge_level(const api_Gauge *self, api_status *status);
uint64_t api_Gauge_into_level(api_Gauge *self, api_status *status);
typedef struct api_Gauge_later_future api_Gauge_later_future;
API_MUST_USE api_Gauge_later_future *api_Gauge_later(const api_Gauge *self, api_status *status);
API_MUST_USE api_poll api_Gauge_later_start(const api_Gauge *self, uint64_t *out, api_Gauge_later_future **future, api_status *status);
API_MUST_USE api_poll api_Gauge_later_poll(api_Gauge_later_future *future, api_waker waker, uint64_t *out);
void api_Gauge_later_future_free(api_Gauge_later_future *future);
typedef struct api_Gauge_reaches_future api_Gauge_reaches_future;
API_MUST_USE api_Gauge_reaches_future *api_Gauge_reaches(const api_Gauge *self, uint64_t this_, api_status *status);
API_MUST_USE api_poll api_Gauge_reaches_start(const api_Gauge *self, uint64_t this_, bool *out, api_Gauge_reaches_future **future, api_status *status);
API_MUST_USE api_poll api_Gauge_reaches_poll(api_Gauge_reaches_future *future, api_waker waker, bool *out);
void api_Gauge_reaches_future_free(api_Gauge_reaches_future *future);
void api_Latch_free(api_Latch *self);
API_MUST_USE api_Latch *api_Latch_clone(const api_Latch *self);
API_MUST_USE api_Latch *api_Latch_default(void);
bool api_Latch_set(const api_Latch *self, uint32_t value);
api_option_u32 api_Latch_get(const api_Latch *self);
void api_Token_free(api_Token *self);
void api_Token_drop(api_Token *self, api_status *status);
API_MUST_USE api_Token *api_Token_clone(const api_Token *self, api_status *status);
uint32_t api_Token_id_get(const api_Token *self, api_status *status);
void api_Token_id_set(api_Token *self, uint32_t value, api_status *status);
API_MUST_USE api_Token *api_Token_new(uint32_t id);
API_MUST_USE api_vec_Token api_Token_twice(const api_Token *self, api_status *status);
void api_Stack_free(api_Stack *self);
void api_Stack_drop(api_Stack *self, api_status *status);
API_MUST_USE api_buf api_Stack_debug(const api_Stack *self, api_status *status);
API_MUST_USE api_Stack *api_Stack_default(void);
API_MUST_USE api_Stack *api_Stack_new(void);
void api_Stack_push(api_Stack *self, api_Counter *item, api_status *status);
API_MUST_USE api_Counter *api_Stack_pop(api_Stack *self, api_status *status);
size_t api_Stack_depth(const api_Stack *self, api_status *status);
uint64_t api_Stack_depth_past_Point(const api_Stack *self, api_Point from, api_status *status);
uint64_t api_Stack_depth_past_Counter(const api_Stack *self, const api_Counter *from, api_status *status);
uint64_t api_Stack_depth_past_Size(const api_Stack *self, api_Size from, api_status *status);
uint64_t api_Stack_depth_past_Wrap_u32(const api_Stack *self, api_Wrap_u32 from, api_status *status);
uint64_t api_Stack_depth_past_polar_Point(const api_Stack *self, api_polar_Point from, api_status *status);
void api_Refusal_free(api_Refusal *self);
void api_Refusal_drop(api_Refusal *self, api_status *status);
API_MUST_USE api_buf api_Refusal_to_string(const api_Refusal *self, api_status *status);
API_MUST_USE api_buf api_Refusal_debug(const api_Refusal *self, api_status *status);
void api_Fault_free(api_Fault *self);
API_MUST_USE api_buf api_Fault_to_string(const api_Fault *self);
API_MUST_USE api_buf api_Fault_debug(const api_Fault *self);
void api_c_free(api_c *self);
bool api_c_eq(const api_c *a, const api_c *b);
API_MUST_USE api_buf api_c_debug(const api_c *self);
API_MUST_USE api_c *api_c_new(uint8_t value);
void api_Quantity_free(api_Quantity *self);
bool api_Quantity_eq(const api_Quantity *a, const api_Quantity *b);
int8_t api_Quantity_partial_cmp(const api_Quantity *a, const api_Quantity *b);
API_MUST_USE api_buf api_Quantity_debug(const api_Quantity *self);
API_MUST_USE api_Quantity *api_Quantity_new(int64_t value, uint32_t unit);
void api_Pass_free(api_Pass *self);
void api_Pass_drop(api_Pass *self, api_status *status);
bool api_Pass_eq(const api_Pass *a, const api_Pass *b, api_status *status);
int8_t api_Pass_partial_cmp(const api_Pass *a, const api_Pass *b, api_status *status);
int8_t api_Pass_cmp(const api_Pass *a, const api_Pass *b, api_status *status);
API_MUST_USE api_buf api_Pass_debug(const api_Pass *self, api_status *status);
API_MUST_USE api_Pass *api_Pass_new(uint32_t number);
uint32_t api_Pass_redeem(api_Pass *self, api_status *status);
void api_Key_free(api_Key *self);
bool api_Key_borrowed(const api_Key *self);
API_MUST_USE api_Key *api_Key_clone(const api_Key *self);
bool api_Key_eq(const api_Key *a, const api_Key *b);
uint64_t api_Key_hash(const api_Key *self);
API_MUST_USE api_buf api_Key_debug(const api_Key *self);
API_MUST_USE api_Key *api_Key_new(api_buf name);
uint64_t api_Key_hash_of(const api_Key *self);
void api_clash_Seed_free(api_clash_Seed *self);
API_MUST_USE api_clash_Seed *api_clash_Seed_default(void);
API_MUST_USE api_clash_Seed *api_clash_Seed_default_(void);
uint32_t api_clash_Seed_value(const api_clash_Seed *self);
bool api_clash_Seed_bool(const api_clash_Seed *self);
void api_clash_Empty_string_free(api_clash_Empty_string *self);
bool api_clash_Empty_string_is_empty(const api_clash_Empty_string *self);
void api_clash_Store_free(api_clash_Store *self);
API_MUST_USE api_clash_Store *api_clash_Store_new(uint32_t count);
uint32_t api_clash_Store_free_(const api_clash_Store *self);
uint32_t api_clash_Store_close(const api_clash_Store *self);
void api_tls_Session_free(api_tls_Session *self);
API_MUST_USE api_buf api_tls_Session_alpn(const api_tls_Session *self);
api_tls_policy_Lifetime api_tls_Session_lifetime(const api_tls_Session *self);
void api_Url_free(api_Url *self);
void api_Url_drop(api_Url *self, api_status *status);
bool api_Url_borrowed(const api_Url *self);
API_MUST_USE api_Url *api_Url_clone(const api_Url *self, api_status *status);
bool api_Url_eq(const api_Url *a, const api_Url *b, api_status *status);
uint64_t api_Url_hash(const api_Url *self, api_status *status);
int8_t api_Url_partial_cmp(const api_Url *a, const api_Url *b, api_status *status);
int8_t api_Url_cmp(const api_Url *a, const api_Url *b, api_status *status);
API_MUST_USE api_buf api_Url_to_string(const api_Url *self, api_status *status);
API_MUST_USE api_buf api_Url_debug(const api_Url *self, api_status *status);
API_MUST_USE bool api_Url_parse(api_buf input, api_Url **out, api_buf *err);
void api_Json_free(api_Json *self);
void api_Json_drop(api_Json *self, api_status *status);
API_MUST_USE bool api_Json_parse(api_buf input, api_Json **out, api_buf *err);
API_MUST_USE api_buf api_Json_to_text(const api_Json *self, api_status *status);
void api_Instant_free(api_Instant *self);
bool api_Instant_borrowed(const api_Instant *self);
API_MUST_USE api_Instant *api_Instant_clone(const api_Instant *self);
bool api_Instant_eq(const api_Instant *a, const api_Instant *b);
uint64_t api_Instant_hash(const api_Instant *self);
int8_t api_Instant_partial_cmp(const api_Instant *a, const api_Instant *b);
int8_t api_Instant_cmp(const api_Instant *a, const api_Instant *b);
API_MUST_USE api_buf api_Instant_debug(const api_Instant *self);
void api_std_io_Error_free(api_std_io_Error *self);
void api_std_io_Error_drop(api_std_io_Error *self, api_status *status);
void api_tls_Config_free(api_tls_Config *self);
void api_tls_Config_drop(api_tls_Config *self, api_status *status);
bool api_tls_Config_borrowed(const api_tls_Config *self);
API_MUST_USE api_tls_Config *api_tls_Config_clone(const api_tls_Config *self, api_status *status);
void api_tls_Cert_free(api_tls_Cert *self);
void api_tls_Cert_drop(api_tls_Cert *self, api_status *status);
void api_tls_Resolver_free(api_tls_Resolver *self);
void api_tls_Resolver_drop(api_tls_Resolver *self, api_status *status);
bool api_tls_Resolver_borrowed(const api_tls_Resolver *self);
API_MUST_USE api_tls_Resolver *api_tls_Resolver_clone(const api_tls_Resolver *self, api_status *status);
void api_tls_Codec_free(api_tls_Codec *self);
void api_tls_Codec_drop(api_tls_Codec *self, api_status *status);
void api_Spawned_free(api_Spawned *self);
void api_Spawned_drop(api_Spawned *self, api_status *status);
API_MUST_USE api_Spawned *api_Spawned_new(uint32_t (*task)(void *data), void *task_data, void (*task_free)(void *data));
uint32_t api_Spawned_join(api_Spawned *self, api_status *status);
void api_LogThread_free(api_LogThread *self);
void api_LogThread_drop(api_LogThread *self, api_status *status);
API_MUST_USE api_LogThread *api_LogThread_new(api_Log log, api_buf line);
bool api_LogThread_join(api_LogThread *self, api_status *status);
void api_Pinned_free(api_Pinned *self);
void api_Pinned_drop(api_Pinned *self, api_status *status);
uint64_t api_Pinned_value_get(const api_Pinned *self, api_status *status);
void api_Pinned_value_set(api_Pinned *self, uint64_t value, api_status *status);
API_MUST_USE api_Pinned *api_Pinned_new(uint64_t value);
API_MUST_USE api_Pinned *api_Pinned_new_homed(uint64_t value);
API_MUST_USE api_Part *api_Pinned_part(const api_Pinned *self, api_status *status);
void api_Hooked_free(api_Hooked *self);
void api_Hooked_drop(api_Hooked *self, api_status *status);
bool api_Hooked_eq(const api_Hooked *a, const api_Hooked *b, api_status *status);
API_MUST_USE api_buf api_Hooked_debug(const api_Hooked *self, api_status *status);
uint64_t api_Hooked_n_get(const api_Hooked *self, api_status *status);
void api_Hooked_n_set(api_Hooked *self, uint64_t value, api_status *status);
API_MUST_USE api_Hooked *api_Hooked_new(uint64_t n);
void api_Forged_free(api_Forged *self);
void api_Forged_drop(api_Forged *self, api_status *status);
bool api_Forged_eq(const api_Forged *a, const api_Forged *b, api_status *status);
API_MUST_USE api_buf api_Forged_debug(const api_Forged *self, api_status *status);
uint64_t api_Forged_n_get(const api_Forged *self, api_status *status);
void api_Forged_n_set(api_Forged *self, uint64_t value, api_status *status);
API_MUST_USE api_Forged *api_Forged_new(uint64_t n);
void api_Part_free(api_Part *self);
void api_Part_drop(api_Part *self, api_status *status);
bool api_Part_eq(const api_Part *a, const api_Part *b, api_status *status);
uint64_t api_Part_hash(const api_Part *self, api_status *status);
API_MUST_USE api_buf api_Part_debug(const api_Part *self, api_status *status);
uint64_t api_Part_value_get(const api_Part *self, api_status *status);
void api_Part_value_set(api_Part *self, uint64_t value, api_status *status);
bool api_has_text(api_Note note);
uint64_t api_marked(const api_Mark *api_marked, api_status *status);
API_MUST_USE api_option_Reading api_reading(uint32_t n);
API_MUST_USE api_buf api_reading_debug(uint32_t n);
API_MUST_USE api_option_Reading api_reading_echo(api_option_Reading reading);
void api_reading_bump(api_option_Reading reading, api_option_Reading *reading_out);
api_Stage api_advance(api_Stage stage);
void api_only(api_Counter *counter, api_Counter **out0, api_status *status);
void api_widths(uint64_t n, uint8_t *out0, int16_t *out1, uint32_t *out2, uint64_t *out3);
API_MUST_USE bool api_divide(uint64_t a, uint64_t b, uint64_t *out0, uint64_t *out1, api_Error *err);
API_MUST_USE bool api_corner(bool unit, api_tuple_u32_u32 *out, api_Error *err);
typedef struct api_corner_later_future api_corner_later_future;
API_MUST_USE api_corner_later_future *api_corner_later(void);
API_MUST_USE api_poll api_corner_later_start(api_tuple_u32_u32 *out, api_corner_later_future **future);
API_MUST_USE api_poll api_corner_later_poll(api_corner_later_future *future, api_waker waker, api_tuple_u32_u32 *out);
void api_corner_later_future_free(api_corner_later_future *future);
api_option_tuple_u8_i16_u32_u64 api_spread(uint64_t n);
api_option_tuple_usize_usize api_find(api_buf haystack, api_buf needle);
API_MUST_USE api_vec_tuple_string_Shape_Point api_rows(api_vec_Point points);
API_MUST_USE bool api_port(api_buf text, uint16_t *out, api_buf *err);
uint32_t api_span(uint32_t start, uint32_t end);
void api_halves(api_Span span, api_tuple_u32_u32 *out0, api_tuple_u32_u32 *out1);
api_option_tuple_option_u32 api_doubled(api_option_tuple_option_u32 value);
API_MUST_USE api_option_tuple_string api_title(bool formal);
API_MUST_USE api_option_tuple_string api_named(api_buf name);
API_MUST_USE api_tuple_bytes api_lent_back(api_tuple_bytes value);
API_MUST_USE api_tuple_bytes api_boxed_tuple(api_buf bytes);
void api_swapped(api_tuple_string_u32 pair, uint32_t *out0, api_buf *out1);
bool api_lasting(const api_Draft *draft, api_status *status);
uint32_t api_stored(api_Receipt *receipt, api_status *status);
uint32_t api_tallied(api_Receipt *receipt, api_status *status);
uint32_t api_summed_up_Receipt(api_Receipt *totaled, api_status *status);
uint32_t api_erased(api_Receipt *receipt, api_status *status);
API_MUST_USE api_Name *api_longer(api_Name *a, const api_Name *b, api_status *status);
API_MUST_USE api_Name *api_unnamed(void);
uint64_t api_total(api_range_u64 range);
uint32_t api_reach(uint32_t start, uint32_t end);
api_option_u128 api_scaled(api_u128 v, uint64_t by);
API_MUST_USE api_Batch_2 *api_pair(void);
uint64_t api_drain_4(api_Batch *batch, api_status *status);
uint64_t api_drain_2(api_Batch_2 *batch, api_status *status);
size_t api_capacities_4_4(const api_Batch *a, const api_Batch *b, api_status *status);
size_t api_capacities_4_2(const api_Batch *a, const api_Batch_2 *b, api_status *status);
size_t api_capacities_2_4(const api_Batch_2 *a, const api_Batch *b, api_status *status);
size_t api_capacities_2_2(const api_Batch_2 *a, const api_Batch_2 *b, api_status *status);
typedef struct api_fill_4_future api_fill_4_future;
API_MUST_USE api_fill_4_future *api_fill_4(api_Batch *batch, uint32_t v, api_status *status);
API_MUST_USE api_poll api_fill_4_start(api_Batch *batch, uint32_t v, api_vec_u32 *out, api_fill_4_future **future, api_status *status);
API_MUST_USE api_poll api_fill_4_poll(api_fill_4_future *future, api_waker waker, api_vec_u32 *out);
void api_fill_4_future_free(api_fill_4_future *future);
typedef struct api_fill_2_future api_fill_2_future;
API_MUST_USE api_fill_2_future *api_fill_2(api_Batch_2 *batch, uint32_t v, api_status *status);
API_MUST_USE api_poll api_fill_2_start(api_Batch_2 *batch, uint32_t v, api_vec_u32 *out, api_fill_2_future **future, api_status *status);
API_MUST_USE api_poll api_fill_2_poll(api_fill_2_future *future, api_waker waker, api_vec_u32 *out);
void api_fill_2_future_free(api_fill_2_future *future);
api_range_u64 api_widen(api_range_u64 range, uint64_t by);
size_t api_count(api_bounds_u64 range);
api_bound_u64 api_end(uint64_t limit);
void api_ends(uint64_t limit, api_bound_u64 *out0, api_bound_u64 *out1);
API_MUST_USE bool api_below(uint64_t n, uint64_t limit, uint64_t *out, api_bounds_u64 *err);
typedef struct api_drive_future api_drive_future;
API_MUST_USE api_drive_future *api_drive(api_Moment start, bool (*poll)(void *data, api_Moment, const api_Waiter *, api_option_Moment *, api_buf *), void *poll_data, void (*poll_free)(void *data));
API_MUST_USE api_poll api_drive_start(api_Moment start, bool (*poll)(void *data, api_Moment, const api_Waiter *, api_option_Moment *, api_buf *), void *poll_data, void (*poll_free)(void *data), api_buf *out, api_drive_future **future);
API_MUST_USE api_poll api_drive_poll(api_drive_future *future, api_waker waker, api_buf *out);
void api_drive_future_free(api_drive_future *future);
API_MUST_USE api_buf api_made(uint32_t n, api_buf (*make)(void *data, uint32_t), void *make_data);
API_MUST_USE api_vec_tuple_string_Ticket api_tickets(api_vec_tuple_string_Ticket entries);
api_Ticket api_reissue(api_Ticket ticket, api_Ticket (*swap)(void *data, api_Ticket), void *swap_data);
api_Ticket api_reissue_lent(api_Ticket ticket, api_Ticket (*swap)(void *data, api_Ticket), void *swap_data);
API_MUST_USE api_Seat api_seat(const api_Mark *mark, uint32_t row, api_status *status);
uint64_t api_admit(api_tuple_Counter_Mark_Seat_Choice entry, api_status *status);
uint64_t api_choose(api_Choice choice, api_status *status);
uint64_t api_heft(api_tuple_Mark_bytes marked, api_Parcel lent, api_tuple_Counter_bytes moved, api_status *status);
API_MUST_USE bool api_spend(api_Counter *e, api_Counter *it, uint64_t *out, api_buf *err, api_status *status);
bool api_is_kept(api_Status_ status);
API_MUST_USE bool api_attempt(bool (*check)(void *data, api_buf *), void *check_data, api_buf *err);
API_MUST_USE bool api_tail(api_foreign_reader file, uint64_t len, api_buf *out0, uint64_t *out1, api_buf *err);
API_MUST_USE bool api_seek_to(api_foreign_reader file, uint64_t at, uint64_t *out, api_buf *err);
typedef struct api_halved_future api_halved_future;
API_MUST_USE api_halved_future *api_halved(uint32_t n);
API_MUST_USE api_poll api_halved_start(uint32_t n, api_option_u32 *out, api_halved_future **future);
API_MUST_USE api_poll api_halved_poll(api_halved_future *future, api_waker waker, api_option_u32 *out);
void api_halved_future_free(api_halved_future *future);
typedef struct api_yielded_future api_yielded_future;
API_MUST_USE api_yielded_future *api_yielded(uint32_t value);
API_MUST_USE api_poll api_yielded_start(uint32_t value, uint32_t *out, api_yielded_future **future);
API_MUST_USE api_poll api_yielded_poll(api_yielded_future *future, api_waker waker, uint32_t *out);
void api_yielded_future_free(api_yielded_future *future);
typedef struct api_after_future api_after_future;
API_MUST_USE api_after_future *api_after(api_foreign_future first, uint32_t value);
API_MUST_USE api_poll api_after_start(api_foreign_future first, uint32_t value, uint32_t *out, api_after_future **future);
API_MUST_USE api_poll api_after_poll(api_after_future *future, api_waker waker, uint32_t *out);
void api_after_future_free(api_after_future *future);
typedef struct api_within_future api_within_future;
API_MUST_USE api_within_future *api_within(uint32_t polls, api_foreign_future future_);
API_MUST_USE api_poll api_within_start(uint32_t polls, api_foreign_future future_, api_Error *err, api_within_future **future);
API_MUST_USE api_poll api_within_poll(api_within_future *future, api_waker waker, api_Error *err);
void api_within_future_free(api_within_future *future);
API_MUST_USE bool api_moment_at(uint64_t ticks, uint64_t rate, api_Moment *out, api_buf *err);
API_MUST_USE bool api_stamp_at(uint64_t millis, api_Stamp *out, api_buf *err);
api_Moment api_shifted(api_Moment moment, api_Moment (*shift)(void *data, api_Moment), void *shift_data);
uint32_t api_jvm_sum(api_JvmField field, api_JvmStatic method, api_OptIn opt);
api_Clocked api_clocked_after(api_Clocked clocked, uint64_t ticks);
uint32_t api_emit(api_Log log, api_buf lines);
typedef struct api_counted_future api_counted_future;
API_MUST_USE api_counted_future *api_counted(uint32_t n, void (*count)(void *data, uint32_t), void *count_data);
API_MUST_USE api_poll api_counted_start(uint32_t n, void (*count)(void *data, uint32_t), void *count_data, uint32_t *out, api_Error *err, api_counted_future **future);
API_MUST_USE api_poll api_counted_poll(api_counted_future *future, api_waker waker, uint32_t *out, api_Error *err);
void api_counted_future_free(api_counted_future *future);
uint32_t api_filter(api_buf lines, bool (*keep)(void *data, uint8_t, api_buf), void *keep_data);
typedef struct api_log_later_future api_log_later_future;
API_MUST_USE api_log_later_future *api_log_later(api_Log log);
API_MUST_USE api_poll api_log_later_start(api_Log log, bool *out, api_log_later_future **future);
API_MUST_USE api_poll api_log_later_poll(api_log_later_future *future, api_waker waker, bool *out);
void api_log_later_future_free(api_log_later_future *future);
uint32_t api_log_counts(api_Log log, uint8_t level, const api_Counter *counter, api_vec_Counter more, api_status *status);
bool api_on_first_thread(void);
uint32_t api_locals_dropped(void);
API_MUST_USE api_Local *api_local_or_new(const api_Local *local, api_status *status);
API_MUST_USE api_Local *api_local_or_new_homed(const api_Local *local, api_status *status);
typedef struct api_local_or_new_later_future api_local_or_new_later_future;
API_MUST_USE api_local_or_new_later_future *api_local_or_new_later(const api_Local *local, api_status *status);
API_MUST_USE api_poll api_local_or_new_later_start(const api_Local *local, api_Local **out, api_local_or_new_later_future **future, api_status *status);
API_MUST_USE api_poll api_local_or_new_later_poll(api_local_or_new_later_future *future, api_waker waker, api_Local **out, api_status *status);
void api_local_or_new_later_future_free(api_local_or_new_later_future *future);
API_MUST_USE api_local_or_new_later_future *api_local_or_new_later_homed(const api_Local *local, api_status *status);
API_MUST_USE api_poll api_local_or_new_later_start_homed(const api_Local *local, api_Local **out, api_local_or_new_later_future **future, api_status *status);
uint32_t api_bump_either(const api_Local *first, const api_Local *second, api_status *status);
typedef struct api_local_after_future api_local_after_future;
API_MUST_USE api_local_after_future *api_local_after(void);
API_MUST_USE api_poll api_local_after_start(api_Local **out, api_local_after_future **future);
API_MUST_USE api_poll api_local_after_poll(api_local_after_future *future, api_waker waker, api_Local **out, api_status *status);
void api_local_after_future_free(api_local_after_future *future);
API_MUST_USE api_local_after_future *api_local_after_homed(void);
API_MUST_USE api_poll api_local_after_start_homed(api_Local **out, api_local_after_future **future);
typedef struct api_local_later_future api_local_later_future;
API_MUST_USE api_local_later_future *api_local_later(uint32_t n);
API_MUST_USE api_poll api_local_later_start(uint32_t n, uint32_t *out, api_local_later_future **future);
API_MUST_USE api_poll api_local_later_poll(api_local_later_future *future, api_waker waker, uint32_t *out, api_status *status);
void api_local_later_future_free(api_local_later_future *future);
API_MUST_USE api_local_later_future *api_local_later_homed(uint32_t n);
API_MUST_USE api_poll api_local_later_start_homed(uint32_t n, uint32_t *out, api_local_later_future **future);
typedef struct api_poll_tripled_future api_poll_tripled_future;
API_MUST_USE api_poll_tripled_future *api_poll_tripled(uint32_t n);
API_MUST_USE api_poll api_poll_tripled_start(uint32_t n, uint32_t *out, api_poll_tripled_future **future);
API_MUST_USE api_poll api_poll_tripled_poll(api_poll_tripled_future *future, api_waker waker, uint32_t *out, api_status *status);
void api_poll_tripled_future_free(api_poll_tripled_future *future);
API_MUST_USE api_poll_tripled_future *api_poll_tripled_homed(uint32_t n);
API_MUST_USE api_poll api_poll_tripled_start_homed(uint32_t n, uint32_t *out, api_poll_tripled_future **future);
uint64_t api_watch(const api_Gauge *gauge, uint64_t (*watch)(void *data, const api_Gauge *), void *watch_data, api_status *status);
API_MUST_USE api_buf api_token_bytes(uint32_t id, uint32_t len);
API_MUST_USE api_buf api_token_slice(uint32_t id, uint32_t len, uint32_t of);
API_MUST_USE api_Packet api_next_packet(api_Packet packet);
API_MUST_USE bool api_fingerprint(api_buf hex, api_array_u8_4 *out, api_buf *err);
uint32_t api_dropped(void);
uint32_t api_bump_both(const api_Tally *a, const api_Tally *b, api_status *status);
uint32_t api_offset(uint32_t (*by)(void *data), void *by_data_, uint32_t by_data);
uint32_t api_shift(uint32_t (*by__)(void *data), void *by___data, uint32_t c_by__);
uint64_t api_walk(uint64_t steps, uint64_t (*visit)(void *data, const api_Cursor *), void *visit_data);
uint32_t api_each(uint32_t steps, void (*visit)(void *data, uint32_t), void *visit_data);
uint32_t api_peek(const api_Tally *tally, uint32_t (*peek)(void *data, const api_Tally *), void *peek_data, api_status *status);
size_t api_len(api_buf payload);
API_MUST_USE api_buf api_hex(api_buf payload);
API_MUST_USE api_buf api_timed_hex(api_buf payload);
API_MUST_USE api_buf api_sample_hex(api_buf sample);
API_MUST_USE api_buf api_queued_hex(api_buf queued);
API_MUST_USE api_buf api_tick_name(api_Label tick);
API_MUST_USE api_buf api_rooted_name(api_Label rooted);
API_MUST_USE api_buf api_stored_text(api_buf stored);
API_MUST_USE api_buf api_layer_name(api_Label layer);
API_MUST_USE api_buf api_greet(api_buf name, api_Shape shape);
API_MUST_USE api_buf api_describe(api_Event event);
api_Pair_Size_u8 api_tag(api_Size size, uint8_t tag);
uint8_t api_cross(api_Left_Right_u8 left, api_Right_Left_u8 right);
uint64_t api_kept_Point(api_Point from);
uint64_t api_kept_Counter(api_Counter *from, api_status *status);
uint64_t api_kept_Size(api_Size from);
uint64_t api_kept_Wrap_u32(api_Wrap_u32 from);
uint64_t api_kept_polar_Point(api_polar_Point from);
int32_t api_keep_Point(api_Point kept);
int32_t api_kept_or_Point(api_option_Point kept);
int32_t api_keeper_Point(api_Point keeper);
api_Boxed_u16 api_BOXED(void);
api_Boxed_u8 api_keeps(api_Log log);
uint32_t api_weigh_Point(api_Point from);
uint32_t api_apply_non_zero_u16(api_Pair_non_zero_u16 pair, uint32_t (*f)(void *data, uint16_t), void *f_data);
uint32_t api_apply_u32(api_Pair_u32 pair, uint32_t (*f)(void *data, uint32_t), void *f_data);
uint32_t api_report_non_zero_u16(api_Pair_non_zero_u16 pair, api_Log log);
uint32_t api_report_u32(api_Pair_u32 pair, api_Log log);
uint32_t api_tally_LogCount(api_Log log);
uint32_t api_wind(api_PolarTurn whole, api_polar_Turn part);
uint64_t api_area(api_Size size);
uint64_t api_seeds_Point_Point(api_Point from);
uint64_t api_seeds_Point_Counter(const api_Counter *from, api_status *status);
uint64_t api_seeds_Point_Size(api_Size from);
uint64_t api_seeds_Point_Wrap_u32(api_Wrap_u32 from);
uint64_t api_seeds_Point_polar_Point(api_polar_Point from);
uint64_t api_seeds_Counter_Point(api_Point from);
uint64_t api_seeds_Counter_Counter(const api_Counter *from, api_status *status);
uint64_t api_seeds_Counter_Size(api_Size from);
uint64_t api_seeds_Counter_Wrap_u32(api_Wrap_u32 from);
uint64_t api_seeds_Counter_polar_Point(api_polar_Point from);
API_MUST_USE api_Area *api_grow(const api_Area *area, uint32_t by);
API_MUST_USE api_Slot_u32 *api_wide(uint32_t value);
API_MUST_USE api_buf api_host(const api_Url *url, api_status *status);
API_MUST_USE bool api_resolve(api_Url *base, api_buf path, api_Url **out, api_buf *err, api_status *status);
API_MUST_USE api_vec_u16 api_ports(api_vec_socket_addr addrs);
api_duration api_doubled_ttl(api_duration ttl);
API_MUST_USE api_buf api_file(api_buf dir, api_buf name);
API_MUST_USE api_buf api_stray(void);
void api_visit_stray(void (*visit)(void *data, api_buf), void *visit_data);
api_option_char api_next_char(uint32_t c);
api_option_duration api_extended(api_duration duration, api_duration by);
API_MUST_USE api_vec_option_path api_parents(api_vec_path paths);
API_MUST_USE api_vec_string api_words(api_buf text);
API_MUST_USE api_buf api_sentence(api_vec_string words);
void api_put_number(uint32_t value, api_buf buf, api_buf *buf_out);
api_duration api_second_later(api_duration w235);
API_MUST_USE bool api_digit(api_buf r, api_buf *r_out, uint8_t *out, api_Error *err);
uint32_t api_push_counted(api_tuple_bytes_u32 entry, uint8_t byte, api_tuple_bytes_u32 *entry_out);
void api_nudge(api_Point point, int32_t by, api_Point *point_out);
void api_nudge_by(api_Point point, const api_Refusal *refusal, api_Point *point_out, api_status *status);
API_MUST_USE api_tls_Config *api_secure(api_buf alpn);
API_MUST_USE api_buf api_alpn(api_tls_Config *config, api_status *status);
API_MUST_USE api_buf api_alpn_lent(const api_tls_Config *config, api_status *status);
API_MUST_USE api_tls_Cert *api_certify(api_buf der);
typedef struct api_byte_len_future api_byte_len_future;
API_MUST_USE api_byte_len_future *api_byte_len(api_buf bytes);
API_MUST_USE api_poll api_byte_len_start(api_buf bytes, size_t *out, api_byte_len_future **future);
API_MUST_USE api_poll api_byte_len_poll(api_byte_len_future *future, api_waker waker, size_t *out);
void api_byte_len_future_free(api_byte_len_future *future);
size_t api_cert_len(const api_tls_Cert *cert, api_status *status);
API_MUST_USE api_buf api_cert_der(api_tls_Cert *cert, api_status *status);
size_t api_chain_len(api_vec_tls_Cert chain, api_status *status);
API_MUST_USE bool api_verify(const api_tls_Cert *cert, uint32_t *out, api_buf *err, api_status *status);
API_MUST_USE bool api_parse_cert(api_buf text, uint32_t *out, api_buf *err);
typedef struct api_parse_later_future api_parse_later_future;
API_MUST_USE api_parse_later_future *api_parse_later(api_buf text);
API_MUST_USE api_poll api_parse_later_start(api_buf text, uint32_t *out, api_buf *err, api_parse_later_future **future);
API_MUST_USE api_poll api_parse_later_poll(api_parse_later_future *future, api_waker waker, uint32_t *out, api_buf *err);
void api_parse_later_future_free(api_parse_later_future *future);
API_MUST_USE api_tls_Resolver *api_resolver(api_buf name);
API_MUST_USE api_buf api_resolved(api_tls_Resolver *resolver, api_status *status);
API_MUST_USE api_std_io_Error *api_failure(api_buf message);
API_MUST_USE api_buf api_failure_message(const api_std_io_Error *error, api_status *status);
API_MUST_USE api_buf api_failure_kind(api_std_io_Error *error, api_status *status);
uint32_t api_sum_all(api_vec_u32 items);
uint64_t api_weigh_all_Counter(api_vec_Counter items, api_status *status);
size_t api_offered_len(api_vec_bytes alpns);
API_MUST_USE api_buf api_join_words(api_vec_string words);
API_MUST_USE bool api_resume(api_buf alpn, uint32_t seconds, api_tls_Session **out, api_buf *err);
size_t api_either(api_buf text);
typedef struct api_either_later_future api_either_later_future;
API_MUST_USE api_either_later_future *api_either_later(api_buf text);
API_MUST_USE api_poll api_either_later_start(api_buf text, size_t *out, api_either_later_future **future);
API_MUST_USE api_poll api_either_later_poll(api_either_later_future *future, api_waker waker, size_t *out);
void api_either_later_future_free(api_either_later_future *future);
API_MUST_USE api_buf api_prefixed(api_buf payload, api_buf prefix);
int32_t api_sum_lists(api_vec_i32 a, api_vec_i32 b, api_vec_i32 c);
int32_t api_coordinates(api_option_Point point);
API_MUST_USE api_vec_bytes api_chunks(api_buf bytes, size_t size);
API_MUST_USE api_vec_Counter api_counters(api_vec_string names);
uint64_t api_worth(api_vec_Counter some, api_vec_Counter boxed, api_status *status);
api_option_u64 api_held_count(api_option_tuple_Counter held, api_status *status);
API_MUST_USE api_Counter *api_boxed_counter(api_buf name);
API_MUST_USE api_buf api_boxed_name(api_Counter *boxed, api_status *status);
uint32_t api_less(uint32_t value, uint32_t _value);
API_MUST_USE api_vec_u32 api_sums(api_vec_vec_u32 rows);
API_MUST_USE api_vec_vec_u32 api_least(api_vec_vec_u32 sets);
void api_split_last(api_vec_i32 items, api_vec_i32 *out0, uint32_t *out1);
uint64_t api_sum_u32(api_vec_u32 items);
API_MUST_USE api_vec_i64 api_doubled_i64(api_vec_i64 items);
API_MUST_USE api_vec_u64 api_visit_chunks(api_buf payload, size_t size, uint64_t (*visit)(void *data, api_buf), void *visit_data);
uint64_t api_scanned(api_Scan scan, api_buf payload);
API_MUST_USE api_vec_f64 api_boxed_floats(api_vec_f64 items);
api_Point api_summed(api_vec_Point points);
api_duration api_breadth(api_Limits_duration limits);
API_MUST_USE api_Limits_bytes api_byte_limits(api_buf low, api_buf high);
API_MUST_USE api_buf api_joined(api_vec_bytes parts);
API_MUST_USE api_buf api_first_of(api_vec_string items);
uint64_t api_read_len(api_buf reader);
size_t api_longest(api_vec_string words);
API_MUST_USE api_buf api_encode(const api_Json *value, api_status *status);
API_MUST_USE api_Json *api_tagged(api_Json *doc, api_buf tag, api_status *status);
API_MUST_USE api_vec_tuple_string_Json api_entries(api_vec_tuple_string_Json entries, api_buf tag, api_status *status);
API_MUST_USE api_vec_tuple_string_Json api_sections(api_vec_tuple_string_Json entries, api_status *status);
API_MUST_USE api_Instant *api_now(void);
API_MUST_USE api_Instant *api_later(const api_Instant *at, api_duration by);
api_duration api_since(const api_Instant *later, const api_Instant *earlier);
bool api_has_instant(api_Instant *at);
api_option_Level api_louder(api_option_Level filter);
API_MUST_USE api_buf api_spell(api_option_Level filter);
API_MUST_USE api_vec_string api_GREETINGS(void);
API_MUST_USE api_vec_u32 api_PRIMES(void);
API_MUST_USE api_vec_Counter api_numbered(api_vec_string names);
void api_nexts(api_socket_addr_v4 v4, api_socket_addr_v6 v6, api_socket_addr_v4 *out0, api_socket_addr_v6 *out1);
void api_next_ips(api_ipv4_addr v4, api_ipv6_addr v6, api_ipv4_addr *out0, api_ipv6_addr *out1);
api_ip_addr api_flip(api_ip_addr ip);
API_MUST_USE api_buf api_reasons(api_vec_tuple_Counter_Refusal pairs, api_option_tuple_Counter_Refusal last, api_status *status);
size_t api_excuse(api_buf note, api_tuple_bytes_Refusal entry, api_status *status);
uint32_t api_relay(api_Log log, api_vec_Refusal refusals, api_status *status);
API_MUST_USE bool api_refuse(api_buf reason, uint32_t *out, api_Refusal **err);
API_MUST_USE bool api_jam(uint32_t n, uint32_t *out, api_Fault **err);
api_Platform api_platform(void);
uint32_t api_pointer_bits(api_Platform platform);
api_Labelled api_same_labelled(api_Labelled labelled);
api_Placement api_placement(void);
bool api_is_family(api_Family family);
uint32_t api_wasm_only(void);
API_MUST_USE bool api_wasm_take(api_Counter *counter, uint64_t *out, api_Error *err, api_status *status);
void api_wasm_log(api_Log log);
uint64_t api_booth_count(api_Booth booth, api_status *status);
api_Page api_page(void);
uint32_t api_beats(api_Beat beat);
int32_t api_same(double a, double b);
int32_t api_errno(void);
bool api_assert(bool holds);
uint8_t api_offsetof(uint8_t field);
API_MUST_USE bool api_return_values(api_Point point, bool fail, api_Point *point_out, api_buf *out0, api_option_tuple_u8_i16_u32_u64 *out1, api_vec_bytes *out2, api_Error *err);
API_MUST_USE bool api_return_moved(api_Counter *counter, bool fail, api_Counter **out, api_Error *err, api_status *status);
api_NonZeros api_non_zeros(void);
API_MUST_USE api_buf api_describe_non_zeros(api_NonZeros non_zeros);
api_option_non_zero_u64 api_next_hop(api_option_non_zero_u64 hop);
uint16_t api_farther_hop(api_Pair_non_zero_u16 hops);
api_duration api_same_duration(api_duration v);
uint64_t api_same_u64(uint64_t v);
size_t api_same_usize(size_t v);
ptrdiff_t api_same_isize(ptrdiff_t v);
api_u128 api_same_u128(api_u128 v);
uint32_t api_same_char(uint32_t v);
API_MUST_USE api_buf api_same_string(api_buf v);
API_MUST_USE api_buf api_same_path(api_buf v);
api_socket_addr api_same_socket_addr(api_socket_addr v);
api_socket_addr_v4 api_same_socket_addr_v4(api_socket_addr_v4 v);
api_socket_addr_v6 api_same_socket_addr_v6(api_socket_addr_v6 v);
api_ip_addr api_same_ip_addr(api_ip_addr v);
API_MUST_USE api_buf api_unescaped(api_buf text);
typedef struct api_unescaped_later_future api_unescaped_later_future;
API_MUST_USE api_unescaped_later_future *api_unescaped_later(api_buf text);
API_MUST_USE api_poll api_unescaped_later_start(api_buf text, api_buf *out, api_unescaped_later_future **future);
API_MUST_USE api_poll api_unescaped_later_poll(api_unescaped_later_future *future, api_waker waker, api_buf *out);
void api_unescaped_later_future_free(api_unescaped_later_future *future);
typedef struct api_unescaped_maybe_future api_unescaped_maybe_future;
API_MUST_USE api_unescaped_maybe_future *api_unescaped_maybe(api_buf text);
API_MUST_USE api_poll api_unescaped_maybe_start(api_buf text, api_buf *out, api_unescaped_maybe_future **future);
API_MUST_USE api_poll api_unescaped_maybe_poll(api_unescaped_maybe_future *future, api_waker waker, api_buf *out);
void api_unescaped_maybe_future_free(api_unescaped_maybe_future *future);
API_MUST_USE api_buf api_unzeroed(api_buf bytes);
uint8_t api_deprecated(uint8_t note);
api_Knob api_knob(api_Dial dial);
typedef struct api_level_later_future api_level_later_future;
API_MUST_USE api_level_later_future *api_level_later(api_Dial dial);
API_MUST_USE api_poll api_level_later_start(api_Dial dial, uint64_t *out, api_level_later_future **future);
API_MUST_USE api_poll api_level_later_poll(api_level_later_future *future, api_waker waker, uint64_t *out);
void api_level_later_future_free(api_level_later_future *future);
API_MUST_USE bool api_turned(api_Dial dial, api_Turner turner, api_Dial *out, api_Error *err);
uint64_t api_crowded(api_Reading reading, api_Stamp a, api_Stamp b, uint64_t n0, uint64_t n1, uint64_t n2, uint64_t n3, uint64_t n4, uint64_t n5, uint64_t n6, uint64_t n7, uint64_t n8, uint64_t n9, uint64_t n10, uint64_t n11, uint64_t n12, uint64_t n13, uint64_t n14, uint64_t n15, uint64_t n16, uint64_t n17, uint64_t n18, uint64_t n19, uint64_t n20, uint64_t n21, uint64_t n22, uint64_t n23, uint64_t n24, uint64_t n25, uint64_t n26, uint64_t n27, uint64_t n28, uint64_t n29, uint64_t n30, uint64_t n31, uint64_t n32, uint64_t n33, uint64_t n34, uint64_t n35, uint64_t n36, uint64_t n37, uint64_t n38, uint64_t n39, uint64_t n40, uint64_t n41, uint64_t n42, uint64_t n43, uint64_t n44, uint64_t n45, uint64_t n46, uint64_t n47, uint64_t n48, uint64_t n49, uint64_t n50, uint64_t n51, uint64_t n52, uint64_t n53, uint64_t n54, uint64_t n55, uint64_t n56, uint64_t n57, uint64_t n58, uint64_t n59, uint64_t n60, uint64_t n61, uint64_t n62, uint64_t n63, uint64_t n64, uint64_t n65, uint64_t n66, uint64_t n67, uint64_t n68, uint64_t n69, uint64_t n70, uint64_t n71, uint64_t n72, uint64_t n73, uint64_t n74, uint64_t n75, uint64_t n76, uint64_t n77, uint64_t n78, uint64_t n79, uint64_t n80, uint64_t n81, uint64_t n82, uint64_t n83, uint64_t n84, uint64_t n85, uint64_t n86, uint64_t n87, uint64_t n88, uint64_t n89, uint64_t n90, uint64_t n91, uint64_t n92, uint64_t n93, uint64_t n94, uint64_t n95, uint64_t n96, uint64_t n97, uint64_t n98, uint64_t n99, uint64_t n100, uint64_t n101, uint64_t n102, uint64_t n103, uint64_t n104, uint64_t n105, uint64_t n106, uint64_t n107, uint64_t n108, uint64_t n109, uint64_t n110, uint64_t n111, uint64_t n112, uint64_t n113, uint64_t n114, uint64_t n115, uint64_t n116, uint64_t n117, uint64_t n118);
uint64_t api_crammed(api_Moment a, api_Moment b, uint64_t n0, uint64_t n1, uint64_t n2, uint64_t n3, uint64_t n4, uint64_t n5, uint64_t n6, uint64_t n7, uint64_t n8, uint64_t n9, uint64_t n10, uint64_t n11, uint64_t n12, uint64_t n13, uint64_t n14, uint64_t n15, uint64_t n16, uint64_t n17, uint64_t n18, uint64_t n19, uint64_t n20, uint64_t n21, uint64_t n22, uint64_t n23, uint64_t n24, uint64_t n25, uint64_t n26, uint64_t n27, uint64_t n28, uint64_t n29, uint64_t n30, uint64_t n31, uint64_t n32, uint64_t n33, uint64_t n34, uint64_t n35, uint64_t n36, uint64_t n37, uint64_t n38, uint64_t n39, uint64_t n40, uint64_t n41, uint64_t n42, uint64_t n43, uint64_t n44, uint64_t n45, uint64_t n46, uint64_t n47, uint64_t n48, uint64_t n49, uint64_t n50, uint64_t n51, uint64_t n52, uint64_t n53, uint64_t n54, uint64_t n55, uint64_t n56, uint64_t n57, uint64_t n58, uint64_t n59, uint64_t n60, uint64_t n61, uint64_t n62, uint64_t n63, uint64_t n64, uint64_t n65, uint64_t n66, uint64_t n67, uint64_t n68, uint64_t n69, uint64_t n70, uint64_t n71, uint64_t n72, uint64_t n73, uint64_t n74, uint64_t n75, uint64_t n76, uint64_t n77, uint64_t n78, uint64_t n79, uint64_t n80, uint64_t n81, uint64_t n82, uint64_t n83, uint64_t n84, uint64_t n85, uint64_t n86, uint64_t n87, uint64_t n88, uint64_t n89, uint64_t n90, uint64_t n91, uint64_t n92, uint64_t n93, uint64_t n94, uint64_t n95, uint64_t n96, uint64_t n97, uint64_t n98, uint64_t n99, uint64_t n100, uint64_t n101, uint64_t n102, uint64_t n103, uint64_t n104, uint64_t n105, uint64_t n106, uint64_t n107, uint64_t n108, uint64_t n109, uint64_t n110, uint64_t n111, uint64_t n112, uint64_t n113, uint64_t n114, uint64_t n115, uint64_t n116, uint64_t n117, uint64_t n118, uint64_t n119, uint64_t n120, uint64_t n121, uint64_t n122, uint64_t n123);
uint32_t api_jni_one(void);
api_option_polar_Point api_polar_point(uint32_t distance);
uint32_t api_macros_streamed(api_macros_Streams streams, uint8_t errno_);
API_MUST_USE api_clash_Any api_clash_any(api_buf type_url);
api_clash_Candidate api_clash_candidate(uint8_t at);
uint8_t api_clash_ignore(uint8_t arg0, uint8_t arg1, uint8_t arg2);
API_MUST_USE api_clash_Empty_string *api_clash_empty(void);
API_MUST_USE bool api_clash_audible(api_clash_Tone tone, api_clash_Tone *out, api_vec_clash_Tone *err);
uint32_t api_clash_nested(api_option_tuple_tuple_u8_u16_u32 a, api_option_tuple_tuple_u8_u16_u32_ b);
uint8_t api_clash_audio_encoder_finish(void);
uint8_t api_clash_audio_encoder_finish_(void);
uint32_t api_emit_from_thread(api_Log log, api_buf lines);
typedef struct api_emit_later_future api_emit_later_future;
API_MUST_USE api_emit_later_future *api_emit_later(api_Log log, api_buf lines);
API_MUST_USE api_poll api_emit_later_start(api_Log log, api_buf lines, uint32_t *out, api_emit_later_future **future);
API_MUST_USE api_poll api_emit_later_poll(api_emit_later_future *future, api_waker waker, uint32_t *out);
void api_emit_later_future_free(api_emit_later_future *future);
typedef struct api_log_on_drop_future api_log_on_drop_future;
API_MUST_USE api_log_on_drop_future *api_log_on_drop(api_Log log);
API_MUST_USE api_poll api_log_on_drop_start(api_Log log, api_log_on_drop_future **future);
API_MUST_USE api_poll api_log_on_drop_poll(api_log_on_drop_future *future, api_waker waker);
void api_log_on_drop_future_free(api_log_on_drop_future *future);
API_MUST_USE api_waker api_log_waker(api_Log log);
void api_hook(void (*f)(void *data), void *f_data, void (*f_free)(void *data));
void api_unhook(void);
#ifdef __cplusplus
}
#endif
#endif
