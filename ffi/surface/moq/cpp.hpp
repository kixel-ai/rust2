// hang.hpp
#ifndef MOQ_HPP_CRATE8
#define MOQ_HPP_CRATE8
#include "rt.hpp"
#include "kio.hpp"
#include "net.hpp"
#include "serdejson.hpp"
namespace moq {
	enum class hang_catalog_AudioCodecKind {
		aac = 0,
		opus = 1,
		unknown = 2,
		flac = 3,
		mp3 = 4,
		pcm = 5,
		ac3 = 6,
		ec3 = 7,
	};
	enum class hang_catalog_VideoCodecKind {
		h264 = 0,
		h265 = 1,
		vp8 = 2,
		vp9 = 3,
		av1 = 4,
		unknown = 5,
	};
	enum class hang_catalog_ColorRange {
		limited = 0,
		full = 1,
	};
	inline constexpr ::std::uint64_t hang_catalog_MAX_SAFE_INTEGER = MOQ_HANG_CATALOG_MAX_SAFE_INTEGER;
	class RUST2_FFI_CONSUMABLE hang_Catalog {
		RUST2_FFI_UNCONSUMED hang_Catalog(hang_Catalog &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_Catalog &operator=(hang_Catalog &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_Catalog clone() const;
		friend bool operator==(const hang_Catalog &a, const hang_Catalog &b);
		friend bool operator!=(const hang_Catalog &a, const hang_Catalog &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_Catalog default_();
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Video video_get() const;
		RUST2_FFI_CALLABLE void video_set(::moq::hang_catalog_Video value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Audio audio_get() const;
		RUST2_FFI_CALLABLE void audio_set(::moq::hang_catalog_Audio value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Timeline sync_get() const;
		RUST2_FFI_CALLABLE void sync_set(const ::moq::hang_catalog_Timeline &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Text text_get() const;
		RUST2_FFI_CALLABLE void text_set(::moq::hang_catalog_Text value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Json json_get() const;
		RUST2_FFI_CALLABLE void json_set(::moq::hang_catalog_Json value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Binary binary_get() const;
		RUST2_FFI_CALLABLE void binary_set(::moq::hang_catalog_Binary value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Stats> stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(::std::optional<::moq::hang_catalog_Stats> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Echo> echo_get() const;
		RUST2_FFI_CALLABLE void echo_set(::std::optional<::moq::hang_catalog_Echo> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_Include>> include_get() const;
		RUST2_FFI_CALLABLE void include_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_Include>> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Skipped skipped_get() const;
		RUST2_FFI_CALLABLE void skipped_set(::moq::hang_catalog_Skipped value);
		static ::moq::buf DEFAULT_NAME();
		static ::moq::track_Info default_track_info();
		static ::moq::track_Subscription default_subscription();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> check_renditions() const;
		RUST2_FFI_CALLABLE ::moq::hang_Catalog without_ext() const;
		static ::moq::result<::moq::hang_Catalog, ::moq::hang_Error> from_slice(::moq::input v);
		static ::moq::result<::moq::hang_Catalog, ::moq::hang_Error> from_reader(::moq::input reader);
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_vec() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> to_writer(::moq::input writer) const;
	};
	class RUST2_FFI_CONSUMABLE hang_Error {
		RUST2_FFI_UNCONSUMED hang_Error(hang_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_Error &operator=(hang_Error &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED hang_Error(const hang_Error &other) = default;
		hang_Error &operator=(const hang_Error &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE hang_stats_Snapshot {
		RUST2_FFI_UNCONSUMED hang_stats_Snapshot(hang_stats_Snapshot &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_stats_Snapshot &operator=(hang_stats_Snapshot &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_stats_Snapshot clone() const;
		friend bool operator==(const hang_stats_Snapshot &a, const hang_stats_Snapshot &b);
		friend bool operator!=(const hang_stats_Snapshot &a, const hang_stats_Snapshot &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_stats_Snapshot default_();
		RUST2_FFI_CALLABLE ::moq::hang_stats_Transport transport_get() const;
		RUST2_FFI_CALLABLE void transport_set(::moq::hang_stats_Transport value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_stats_Rendition>> renditions_get() const;
		RUST2_FFI_CALLABLE void renditions_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_stats_Rendition>> value);
	};
	class RUST2_FFI_CONSUMABLE hang_stats_Rendition {
		RUST2_FFI_UNCONSUMED hang_stats_Rendition(hang_stats_Rendition &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_stats_Rendition &operator=(hang_stats_Rendition &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_stats_Rendition clone() const;
		friend bool operator==(const hang_stats_Rendition &a, const hang_stats_Rendition &b);
		friend bool operator!=(const hang_stats_Rendition &a, const hang_stats_Rendition &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_stats_Rendition default_();
		RUST2_FFI_CALLABLE ::std::uint64_t frames_get() const;
		RUST2_FFI_CALLABLE void frames_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t bytes_get() const;
		RUST2_FFI_CALLABLE void bytes_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t keyframes_get() const;
		RUST2_FFI_CALLABLE void keyframes_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t skipped_get() const;
		RUST2_FFI_CALLABLE void skipped_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bitrate_get() const;
		RUST2_FFI_CALLABLE void bitrate_set(const ::std::optional<::std::uint64_t> &value);
	};
	class RUST2_FFI_CONSUMABLE hang_stats_Transport {
		RUST2_FFI_UNCONSUMED hang_stats_Transport(hang_stats_Transport &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_stats_Transport &operator=(hang_stats_Transport &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_stats_Transport clone() const;
		friend bool operator==(const hang_stats_Transport &a, const hang_stats_Transport &b);
		friend bool operator!=(const hang_stats_Transport &a, const hang_stats_Transport &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_stats_Transport default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> rtt_get() const;
		RUST2_FFI_CALLABLE void rtt_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> rate_get() const;
		RUST2_FFI_CALLABLE void rate_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> lost_bytes_get() const;
		RUST2_FFI_CALLABLE void lost_bytes_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> lost_packets_get() const;
		RUST2_FFI_CALLABLE void lost_packets_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> age_get() const;
		RUST2_FFI_CALLABLE void age_set(const ::std::optional<::moq::duration> &value);
	};
	class RUST2_FFI_CONSUMABLE hang_echo_Snapshot {
		RUST2_FFI_UNCONSUMED hang_echo_Snapshot(hang_echo_Snapshot &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_echo_Snapshot &operator=(hang_echo_Snapshot &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_echo_Snapshot clone() const;
		friend bool operator==(const hang_echo_Snapshot &a, const hang_echo_Snapshot &b);
		friend bool operator!=(const hang_echo_Snapshot &a, const hang_echo_Snapshot &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_echo_Snapshot default_();
		RUST2_FFI_CALLABLE ::moq::hang_stats_Transport transport_get() const;
		RUST2_FFI_CALLABLE void transport_set(::moq::hang_stats_Transport value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_echo_Rendition>> renditions_get() const;
		RUST2_FFI_CALLABLE void renditions_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_echo_Rendition>> value);
	};
	class RUST2_FFI_CONSUMABLE hang_echo_Rendition {
		RUST2_FFI_UNCONSUMED hang_echo_Rendition(hang_echo_Rendition &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_echo_Rendition &operator=(hang_echo_Rendition &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_echo_Rendition clone() const;
		friend bool operator==(const hang_echo_Rendition &a, const hang_echo_Rendition &b);
		friend bool operator!=(const hang_echo_Rendition &a, const hang_echo_Rendition &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_echo_Rendition default_();
		RUST2_FFI_CALLABLE ::std::uint64_t frames_get() const;
		RUST2_FFI_CALLABLE void frames_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t bytes_get() const;
		RUST2_FFI_CALLABLE void bytes_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t decoded_get() const;
		RUST2_FFI_CALLABLE void decoded_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t late_get() const;
		RUST2_FFI_CALLABLE void late_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t errors_get() const;
		RUST2_FFI_CALLABLE void errors_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t stalls_get() const;
		RUST2_FFI_CALLABLE void stalls_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t underruns_get() const;
		RUST2_FFI_CALLABLE void underruns_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::moq::duration stalled_get() const;
		RUST2_FFI_CALLABLE void stalled_set(::moq::duration value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_echo_Arrival> arrival_get() const;
		RUST2_FFI_CALLABLE void arrival_set(::std::optional<::moq::hang_echo_Arrival> value);
	};
	class RUST2_FFI_CONSUMABLE hang_echo_Arrival {
		RUST2_FFI_UNCONSUMED hang_echo_Arrival(hang_echo_Arrival &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_echo_Arrival &operator=(hang_echo_Arrival &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_echo_Arrival clone() const;
		friend bool operator==(const hang_echo_Arrival &a, const hang_echo_Arrival &b);
		friend bool operator!=(const hang_echo_Arrival &a, const hang_echo_Arrival &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::Timestamp timestamp_get() const;
		RUST2_FFI_CALLABLE void timestamp_set(const ::moq::Timestamp &value);
		RUST2_FFI_CALLABLE ::moq::clock_Wall wall_get() const;
		RUST2_FFI_CALLABLE void wall_set(const ::moq::clock_Wall &value);
		static ::moq::hang_echo_Arrival new_(const ::moq::Timestamp &timestamp, const ::moq::clock_Wall &wall);
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Skipped {
		RUST2_FFI_UNCONSUMED hang_catalog_Skipped(hang_catalog_Skipped &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Skipped &operator=(hang_catalog_Skipped &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Skipped clone() const;
		friend bool operator==(const hang_catalog_Skipped &a, const hang_catalog_Skipped &b);
		friend bool operator!=(const hang_catalog_Skipped &a, const hang_catalog_Skipped &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_catalog_Skipped default_();
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::std::uint64_t>> video_get() const;
		RUST2_FFI_CALLABLE void video_set(const ::moq::items<::std::tuple<::moq::view, ::std::uint64_t>> &value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::std::uint64_t>> audio_get() const;
		RUST2_FFI_CALLABLE void audio_set(const ::moq::items<::std::tuple<::moq::view, ::std::uint64_t>> &value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::std::uint64_t>> text_get() const;
		RUST2_FFI_CALLABLE void text_set(const ::moq::items<::std::tuple<::moq::view, ::std::uint64_t>> &value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::std::uint64_t>> json_get() const;
		RUST2_FFI_CALLABLE void json_set(const ::moq::items<::std::tuple<::moq::view, ::std::uint64_t>> &value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::std::uint64_t>> binary_get() const;
		RUST2_FFI_CALLABLE void binary_set(const ::moq::items<::std::tuple<::moq::view, ::std::uint64_t>> &value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::std::uint64_t>> include_get() const;
		RUST2_FFI_CALLABLE void include_set(const ::moq::items<::std::tuple<::moq::view, ::std::uint64_t>> &value);
		RUST2_FFI_CALLABLE bool is_empty() const;
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> version() const;
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Consumer {
		RUST2_FFI_UNCONSUMED hang_catalog_Consumer(hang_catalog_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Consumer &operator=(hang_catalog_Consumer &&other) noexcept = default;
		static ::moq::hang_catalog_Consumer new_(::moq::track_Consumer track, ::moq::duration delay);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Consumer_poll_next_future poll_next();
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Audio {
		RUST2_FFI_UNCONSUMED hang_catalog_Audio(hang_catalog_Audio &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Audio &operator=(hang_catalog_Audio &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Audio clone() const;
		friend bool operator==(const hang_catalog_Audio &a, const hang_catalog_Audio &b);
		friend bool operator!=(const hang_catalog_Audio &a, const hang_catalog_Audio &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_catalog_Audio default_();
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_AudioConfig>> renditions_get() const;
		RUST2_FFI_CALLABLE void renditions_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_AudioConfig>> value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> insert(::moq::input name, ::moq::hang_catalog_AudioConfig config);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_AudioConfig> remove(::moq::input name);
		RUST2_FFI_CALLABLE bool is_empty() const;
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_AudioConfig>> enabled() const;
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_AudioConfig>> ranked() const;
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_AudioConfig {
		RUST2_FFI_UNCONSUMED hang_catalog_AudioConfig(hang_catalog_AudioConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_AudioConfig &operator=(hang_catalog_AudioConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_AudioConfig clone() const;
		friend bool operator==(const hang_catalog_AudioConfig &a, const hang_catalog_AudioConfig &b);
		friend bool operator!=(const hang_catalog_AudioConfig &a, const hang_catalog_AudioConfig &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::path_Relative> broadcast_get() const;
		RUST2_FFI_CALLABLE void broadcast_set(::std::optional<::moq::path_Relative> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> track_get() const;
		RUST2_FFI_CALLABLE void track_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> label_get() const;
		RUST2_FFI_CALLABLE void label_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_AudioCodec codec_get() const;
		RUST2_FFI_CALLABLE void codec_set(const ::moq::hang_catalog_AudioCodec &value);
		RUST2_FFI_CALLABLE ::std::uint32_t sample_rate_get() const;
		RUST2_FFI_CALLABLE void sample_rate_set(::std::uint32_t value);
		RUST2_FFI_CALLABLE ::std::uint32_t channel_count_get() const;
		RUST2_FFI_CALLABLE void channel_count_set(::std::uint32_t value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_AudioLayout> layout_get() const;
		RUST2_FFI_CALLABLE void layout_set(const ::std::optional<::moq::hang_catalog_AudioLayout> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bitrate_get() const;
		RUST2_FFI_CALLABLE void bitrate_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE bool enabled_get() const;
		RUST2_FFI_CALLABLE void enabled_set(bool value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> description_get() const;
		RUST2_FFI_CALLABLE void description_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Container container_get() const;
		RUST2_FFI_CALLABLE void container_set(::moq::hang_catalog_Container value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> jitter_get() const;
		RUST2_FFI_CALLABLE void jitter_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> warmup_get() const;
		RUST2_FFI_CALLABLE void warmup_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::std::optional<::moq::duration> &value);
		static ::moq::hang_catalog_AudioConfig new_(const ::moq::hang_catalog_AudioCodec &codec, ::std::uint32_t sample_rate, ::std::uint32_t channel_count);
		RUST2_FFI_CALLABLE bool same_identity(const ::moq::hang_catalog_AudioConfig &other) const;
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Binary {
		RUST2_FFI_UNCONSUMED hang_catalog_Binary(hang_catalog_Binary &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Binary &operator=(hang_catalog_Binary &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Binary clone() const;
		friend bool operator==(const hang_catalog_Binary &a, const hang_catalog_Binary &b);
		friend bool operator!=(const hang_catalog_Binary &a, const hang_catalog_Binary &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_catalog_Binary default_();
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_BinaryConfig>> tracks_get() const;
		RUST2_FFI_CALLABLE void tracks_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_BinaryConfig>> value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> insert(::moq::input name, ::moq::hang_catalog_BinaryConfig config);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_BinaryConfig> remove(::moq::input name);
		RUST2_FFI_CALLABLE bool is_empty() const;
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_BinaryConfig {
		RUST2_FFI_UNCONSUMED hang_catalog_BinaryConfig(hang_catalog_BinaryConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_BinaryConfig &operator=(hang_catalog_BinaryConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_BinaryConfig clone() const;
		friend bool operator==(const hang_catalog_BinaryConfig &a, const hang_catalog_BinaryConfig &b);
		friend bool operator!=(const hang_catalog_BinaryConfig &a, const hang_catalog_BinaryConfig &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::path_Relative> broadcast_get() const;
		RUST2_FFI_CALLABLE void broadcast_set(::std::optional<::moq::path_Relative> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> track_get() const;
		RUST2_FFI_CALLABLE void track_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Mode mode_get() const;
		RUST2_FFI_CALLABLE void mode_set(const ::moq::hang_catalog_Mode &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> mime_get() const;
		RUST2_FFI_CALLABLE void mime_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bitrate_get() const;
		RUST2_FFI_CALLABLE void bitrate_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> jitter_get() const;
		RUST2_FFI_CALLABLE void jitter_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::Json>> extra_get() const;
		RUST2_FFI_CALLABLE void extra_set(::moq::taken<::std::tuple<::moq::view, ::moq::Json>> value);
		static ::moq::hang_catalog_BinaryConfig new_(const ::moq::hang_catalog_Mode &mode);
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Container {
		RUST2_FFI_UNCONSUMED hang_catalog_Container(hang_catalog_Container &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Container &operator=(hang_catalog_Container &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Container clone() const;
		friend bool operator==(const hang_catalog_Container &a, const hang_catalog_Container &b);
		friend bool operator!=(const hang_catalog_Container &a, const hang_catalog_Container &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_catalog_Container default_();
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Echo {
		RUST2_FFI_UNCONSUMED hang_catalog_Echo(hang_catalog_Echo &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Echo &operator=(hang_catalog_Echo &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Echo clone() const;
		friend bool operator==(const hang_catalog_Echo &a, const hang_catalog_Echo &b);
		friend bool operator!=(const hang_catalog_Echo &a, const hang_catalog_Echo &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::path_Relative path_get() const;
		RUST2_FFI_CALLABLE void path_set(::moq::path_Relative value);
		static ::moq::hang_catalog_Echo new_(::moq::path_Relative path);
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Include {
		RUST2_FFI_UNCONSUMED hang_catalog_Include(hang_catalog_Include &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Include &operator=(hang_catalog_Include &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Include clone() const;
		friend bool operator==(const hang_catalog_Include &a, const hang_catalog_Include &b);
		friend bool operator!=(const hang_catalog_Include &a, const hang_catalog_Include &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::path_Relative broadcast_get() const;
		RUST2_FFI_CALLABLE void broadcast_set(::moq::path_Relative value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		static ::moq::hang_catalog_Include new_(::moq::path_Relative broadcast);
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Json {
		RUST2_FFI_UNCONSUMED hang_catalog_Json(hang_catalog_Json &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Json &operator=(hang_catalog_Json &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Json clone() const;
		friend bool operator==(const hang_catalog_Json &a, const hang_catalog_Json &b);
		friend bool operator!=(const hang_catalog_Json &a, const hang_catalog_Json &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_catalog_Json default_();
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_JsonConfig>> tracks_get() const;
		RUST2_FFI_CALLABLE void tracks_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_JsonConfig>> value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> insert(::moq::input name, ::moq::hang_catalog_JsonConfig config);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_JsonConfig> remove(::moq::input name);
		RUST2_FFI_CALLABLE bool is_empty() const;
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_JsonConfig {
		RUST2_FFI_UNCONSUMED hang_catalog_JsonConfig(hang_catalog_JsonConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_JsonConfig &operator=(hang_catalog_JsonConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_JsonConfig clone() const;
		friend bool operator==(const hang_catalog_JsonConfig &a, const hang_catalog_JsonConfig &b);
		friend bool operator!=(const hang_catalog_JsonConfig &a, const hang_catalog_JsonConfig &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::path_Relative> broadcast_get() const;
		RUST2_FFI_CALLABLE void broadcast_set(::std::optional<::moq::path_Relative> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> track_get() const;
		RUST2_FFI_CALLABLE void track_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Mode mode_get() const;
		RUST2_FFI_CALLABLE void mode_set(const ::moq::hang_catalog_Mode &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> schema_get() const;
		RUST2_FFI_CALLABLE void schema_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bitrate_get() const;
		RUST2_FFI_CALLABLE void bitrate_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> jitter_get() const;
		RUST2_FFI_CALLABLE void jitter_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::Json>> extra_get() const;
		RUST2_FFI_CALLABLE void extra_set(::moq::taken<::std::tuple<::moq::view, ::moq::Json>> value);
		static ::moq::hang_catalog_JsonConfig new_(const ::moq::hang_catalog_Mode &mode);
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Stats {
		RUST2_FFI_UNCONSUMED hang_catalog_Stats(hang_catalog_Stats &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Stats &operator=(hang_catalog_Stats &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Stats clone() const;
		friend bool operator==(const hang_catalog_Stats &a, const hang_catalog_Stats &b);
		friend bool operator!=(const hang_catalog_Stats &a, const hang_catalog_Stats &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::buf track_get() const;
		RUST2_FFI_CALLABLE void track_set(::moq::input value);
		static ::moq::hang_catalog_Stats new_(::moq::input track);
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Text {
		RUST2_FFI_UNCONSUMED hang_catalog_Text(hang_catalog_Text &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Text &operator=(hang_catalog_Text &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Text clone() const;
		friend bool operator==(const hang_catalog_Text &a, const hang_catalog_Text &b);
		friend bool operator!=(const hang_catalog_Text &a, const hang_catalog_Text &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_catalog_Text default_();
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_TextConfig>> renditions_get() const;
		RUST2_FFI_CALLABLE void renditions_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_TextConfig>> value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> insert(::moq::input name, ::moq::hang_catalog_TextConfig config);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_TextConfig> remove(::moq::input name);
		RUST2_FFI_CALLABLE bool is_empty() const;
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_TextConfig {
		RUST2_FFI_UNCONSUMED hang_catalog_TextConfig(hang_catalog_TextConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_TextConfig &operator=(hang_catalog_TextConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_TextConfig clone() const;
		friend bool operator==(const hang_catalog_TextConfig &a, const hang_catalog_TextConfig &b);
		friend bool operator!=(const hang_catalog_TextConfig &a, const hang_catalog_TextConfig &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::path_Relative> broadcast_get() const;
		RUST2_FFI_CALLABLE void broadcast_set(::std::optional<::moq::path_Relative> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> track_get() const;
		RUST2_FFI_CALLABLE void track_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_TextFormat format_get() const;
		RUST2_FFI_CALLABLE void format_set(const ::moq::hang_catalog_TextFormat &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_TextRole role_get() const;
		RUST2_FFI_CALLABLE void role_set(const ::moq::hang_catalog_TextRole &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> lang_get() const;
		RUST2_FFI_CALLABLE void lang_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> label_get() const;
		RUST2_FFI_CALLABLE void label_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Container container_get() const;
		RUST2_FFI_CALLABLE void container_set(::moq::hang_catalog_Container value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> jitter_get() const;
		RUST2_FFI_CALLABLE void jitter_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::std::optional<::moq::duration> &value);
		static ::moq::hang_catalog_TextConfig new_(const ::moq::hang_catalog_TextFormat &format);
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Timeline {
		RUST2_FFI_UNCONSUMED hang_catalog_Timeline(hang_catalog_Timeline &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Timeline &operator=(hang_catalog_Timeline &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Timeline clone() const;
		friend bool operator==(const hang_catalog_Timeline &a, const hang_catalog_Timeline &b);
		friend bool operator!=(const hang_catalog_Timeline &a, const hang_catalog_Timeline &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_catalog_Timeline default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Clock> clock_get() const;
		RUST2_FFI_CALLABLE void clock_set(const ::std::optional<::moq::hang_catalog_Clock> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> interval_get() const;
		RUST2_FFI_CALLABLE void interval_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE bool is_empty() const;
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Color {
		RUST2_FFI_UNCONSUMED hang_catalog_Color(hang_catalog_Color &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Color &operator=(hang_catalog_Color &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Color clone() const;
		friend bool operator==(const hang_catalog_Color &a, const hang_catalog_Color &b);
		friend bool operator!=(const hang_catalog_Color &a, const hang_catalog_Color &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_catalog_Color default_();
		RUST2_FFI_CALLABLE ::std::optional<::std::uint8_t> primaries_get() const;
		RUST2_FFI_CALLABLE void primaries_set(const ::std::optional<::std::uint8_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint8_t> transfer_get() const;
		RUST2_FFI_CALLABLE void transfer_set(const ::std::optional<::std::uint8_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint8_t> matrix_get() const;
		RUST2_FFI_CALLABLE void matrix_set(const ::std::optional<::std::uint8_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_ColorRange> range_get() const;
		RUST2_FFI_CALLABLE void range_set(const ::std::optional<::moq::hang_catalog_ColorRange> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Mastering> mastering_get() const;
		RUST2_FFI_CALLABLE void mastering_set(const ::std::optional<::moq::hang_catalog_Mastering> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_ContentLight> content_light_get() const;
		RUST2_FFI_CALLABLE void content_light_set(const ::std::optional<::moq::hang_catalog_ContentLight> &value);
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Projection {
		RUST2_FFI_UNCONSUMED hang_catalog_Projection(hang_catalog_Projection &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Projection &operator=(hang_catalog_Projection &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Projection clone() const;
		friend bool operator==(const hang_catalog_Projection &a, const hang_catalog_Projection &b);
		friend bool operator!=(const hang_catalog_Projection &a, const hang_catalog_Projection &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_catalog_Projection default_();
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Video {
		RUST2_FFI_UNCONSUMED hang_catalog_Video(hang_catalog_Video &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Video &operator=(hang_catalog_Video &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Video clone() const;
		friend bool operator==(const hang_catalog_Video &a, const hang_catalog_Video &b);
		friend bool operator!=(const hang_catalog_Video &a, const hang_catalog_Video &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_catalog_Video default_();
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_VideoConfig>> renditions_get() const;
		RUST2_FFI_CALLABLE void renditions_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_VideoConfig>> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Display> display_get() const;
		RUST2_FFI_CALLABLE void display_set(const ::std::optional<::moq::hang_catalog_Display> &value);
		RUST2_FFI_CALLABLE ::std::optional<double> rotation_get() const;
		RUST2_FFI_CALLABLE void rotation_set(const ::std::optional<double> &value);
		RUST2_FFI_CALLABLE ::std::optional<bool> flip_get() const;
		RUST2_FFI_CALLABLE void flip_set(const ::std::optional<bool> &value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> insert(::moq::input name, ::moq::hang_catalog_VideoConfig config);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_VideoConfig> remove(::moq::input name);
		RUST2_FFI_CALLABLE bool is_empty() const;
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_VideoConfig>> enabled() const;
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_VideoConfig>> ranked() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> set_properties(::moq::hang_catalog_VideoProperties properties);
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_VideoProperties {
		RUST2_FFI_UNCONSUMED hang_catalog_VideoProperties(hang_catalog_VideoProperties &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_VideoProperties &operator=(hang_catalog_VideoProperties &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_VideoProperties clone() const;
		friend bool operator==(const hang_catalog_VideoProperties &a, const hang_catalog_VideoProperties &b);
		friend bool operator!=(const hang_catalog_VideoProperties &a, const hang_catalog_VideoProperties &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_catalog_VideoProperties default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Display> display_get() const;
		RUST2_FFI_CALLABLE void display_set(const ::std::optional<::moq::hang_catalog_Display> &value);
		RUST2_FFI_CALLABLE ::std::optional<double> rotation_get() const;
		RUST2_FFI_CALLABLE void rotation_set(const ::std::optional<double> &value);
		RUST2_FFI_CALLABLE ::std::optional<bool> flip_get() const;
		RUST2_FFI_CALLABLE void flip_set(const ::std::optional<bool> &value);
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_VideoConfig {
		RUST2_FFI_UNCONSUMED hang_catalog_VideoConfig(hang_catalog_VideoConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_VideoConfig &operator=(hang_catalog_VideoConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_VideoConfig clone() const;
		friend bool operator==(const hang_catalog_VideoConfig &a, const hang_catalog_VideoConfig &b);
		friend bool operator!=(const hang_catalog_VideoConfig &a, const hang_catalog_VideoConfig &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::path_Relative> broadcast_get() const;
		RUST2_FFI_CALLABLE void broadcast_set(::std::optional<::moq::path_Relative> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> track_get() const;
		RUST2_FFI_CALLABLE void track_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> label_get() const;
		RUST2_FFI_CALLABLE void label_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_VideoCodec codec_get() const;
		RUST2_FFI_CALLABLE void codec_set(const ::moq::hang_catalog_VideoCodec &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> description_get() const;
		RUST2_FFI_CALLABLE void description_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> coded_width_get() const;
		RUST2_FFI_CALLABLE void coded_width_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> coded_height_get() const;
		RUST2_FFI_CALLABLE void coded_height_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> display_aspect_width_get() const;
		RUST2_FFI_CALLABLE void display_aspect_width_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> display_aspect_height_get() const;
		RUST2_FFI_CALLABLE void display_aspect_height_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Color> color_get() const;
		RUST2_FFI_CALLABLE void color_set(::std::optional<::moq::hang_catalog_Color> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Projection> projection_get() const;
		RUST2_FFI_CALLABLE void projection_set(::std::optional<::moq::hang_catalog_Projection> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Stereo> stereo_get() const;
		RUST2_FFI_CALLABLE void stereo_set(const ::std::optional<::moq::hang_catalog_Stereo> &value);
		RUST2_FFI_CALLABLE bool interlaced_get() const;
		RUST2_FFI_CALLABLE void interlaced_set(bool value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bitrate_get() const;
		RUST2_FFI_CALLABLE void bitrate_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE bool enabled_get() const;
		RUST2_FFI_CALLABLE void enabled_set(bool value);
		RUST2_FFI_CALLABLE ::std::int32_t preference_get() const;
		RUST2_FFI_CALLABLE void preference_set(::std::int32_t value);
		RUST2_FFI_CALLABLE ::std::optional<double> framerate_get() const;
		RUST2_FFI_CALLABLE void framerate_set(const ::std::optional<double> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint8_t> reorder_get() const;
		RUST2_FFI_CALLABLE void reorder_set(const ::std::optional<::std::uint8_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<bool> optimize_for_latency_get() const;
		RUST2_FFI_CALLABLE void optimize_for_latency_set(const ::std::optional<bool> &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Container container_get() const;
		RUST2_FFI_CALLABLE void container_set(::moq::hang_catalog_Container value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> jitter_get() const;
		RUST2_FFI_CALLABLE void jitter_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::std::optional<::moq::duration> &value);
		static ::moq::hang_catalog_VideoConfig new_(const ::moq::hang_catalog_VideoCodec &codec);
		RUST2_FFI_CALLABLE bool same_identity(const ::moq::hang_catalog_VideoConfig &other) const;
	};
	class RUST2_FFI_CONSUMABLE moq_json_Timed_Catalog {
		RUST2_FFI_UNCONSUMED moq_json_Timed_Catalog(moq_json_Timed_Catalog &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_json_Timed_Catalog &operator=(moq_json_Timed_Catalog &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_json_Timed_Catalog clone() const;
		friend bool operator==(const moq_json_Timed_Catalog &a, const moq_json_Timed_Catalog &b);
		friend bool operator!=(const moq_json_Timed_Catalog &a, const moq_json_Timed_Catalog &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::hang_Catalog value_get() const;
		RUST2_FFI_CALLABLE void value_set(::moq::hang_Catalog value);
		RUST2_FFI_CALLABLE ::moq::Timestamp at_get() const;
		RUST2_FFI_CALLABLE void at_set(const ::moq::Timestamp &value);
		static ::moq::moq_json_Timed_Catalog new_(::moq::hang_Catalog value, const ::moq::Timestamp &at);
	};
	struct hang_catalog_AAC {
		::std::uint8_t profile;
		friend bool operator==(const hang_catalog_AAC &a, const hang_catalog_AAC &b);
		friend bool operator!=(const hang_catalog_AAC &a, const hang_catalog_AAC &b);
		::moq::buf to_string() const;
		::moq::buf debug() const;
	};
	class hang_catalog_Clock {
		friend bool operator==(const hang_catalog_Clock &a, const hang_catalog_Clock &b);
		friend bool operator!=(const hang_catalog_Clock &a, const hang_catalog_Clock &b);
		::moq::buf debug() const;
		::moq::Timestamp wall_get() const;
		void wall_set(const ::moq::Timestamp &value);
		static ::moq::result<::moq::hang_catalog_Clock, ::moq::hang_Error> new_(const ::moq::Timestamp &wall);
		::moq::result<::moq::clock_Wall, ::moq::hang_Error> wall_clock(const ::moq::Timestamp &pts) const;
	};
	class hang_catalog_Priorities {
		friend bool operator==(const hang_catalog_Priorities &a, const hang_catalog_Priorities &b);
		friend bool operator!=(const hang_catalog_Priorities &a, const hang_catalog_Priorities &b);
		::moq::buf debug() const;
		::std::int8_t catalog_get() const;
		void catalog_set(::std::int8_t value);
		::std::int8_t text_get() const;
		void text_set(::std::int8_t value);
		::std::int8_t audio_get() const;
		void audio_set(::std::int8_t value);
		::std::int8_t video_get() const;
		void video_set(::std::int8_t value);
	};
	struct hang_catalog_AV1 {
		::std::uint8_t profile;
		::std::uint8_t level;
		char32_t tier;
		::std::uint8_t bitdepth;
		bool mono_chrome;
		bool chroma_subsampling_x;
		bool chroma_subsampling_y;
		::std::uint8_t chroma_sample_position;
		::std::uint8_t color_primaries;
		::std::uint8_t transfer_characteristics;
		::std::uint8_t matrix_coefficients;
		bool full_range;
		friend bool operator==(const hang_catalog_AV1 &a, const hang_catalog_AV1 &b);
		friend bool operator!=(const hang_catalog_AV1 &a, const hang_catalog_AV1 &b);
		::moq::buf to_string() const;
		::moq::buf debug() const;
		static hang_catalog_AV1 default_();
		static ::moq::buf PREFIX();
	};
	struct hang_catalog_Chromaticity {
		double x;
		double y;
		friend bool operator==(const hang_catalog_Chromaticity &a, const hang_catalog_Chromaticity &b);
		friend bool operator!=(const hang_catalog_Chromaticity &a, const hang_catalog_Chromaticity &b);
		::moq::buf debug() const;
	};
	struct hang_catalog_Mastering {
		::moq::hang_catalog_Chromaticity red;
		::moq::hang_catalog_Chromaticity green;
		::moq::hang_catalog_Chromaticity blue;
		::moq::hang_catalog_Chromaticity white;
		double max_luminance;
		double min_luminance;
		friend bool operator==(const hang_catalog_Mastering &a, const hang_catalog_Mastering &b);
		friend bool operator!=(const hang_catalog_Mastering &a, const hang_catalog_Mastering &b);
		::moq::buf debug() const;
	};
	struct hang_catalog_ContentLight {
		::std::uint16_t max_cll;
		::std::uint16_t max_fall;
		friend bool operator==(const hang_catalog_ContentLight &a, const hang_catalog_ContentLight &b);
		friend bool operator!=(const hang_catalog_ContentLight &a, const hang_catalog_ContentLight &b);
		::moq::buf debug() const;
	};
	struct hang_catalog_H264 {
		bool inline_;
		::std::uint8_t profile;
		::std::uint8_t constraints;
		::std::uint8_t level;
		friend bool operator==(const hang_catalog_H264 &a, const hang_catalog_H264 &b);
		friend bool operator!=(const hang_catalog_H264 &a, const hang_catalog_H264 &b);
		::moq::buf to_string() const;
		::moq::buf debug() const;
	};
	struct hang_catalog_H265 {
		bool in_band;
		::std::uint8_t profile_space;
		::std::uint8_t profile_idc;
		::std::array<::std::uint8_t, 4> profile_compatibility_flags;
		bool tier_flag;
		::std::uint8_t level_idc;
		::std::array<::std::uint8_t, 6> constraint_flags;
		friend bool operator==(const hang_catalog_H265 &a, const hang_catalog_H265 &b);
		friend bool operator!=(const hang_catalog_H265 &a, const hang_catalog_H265 &b);
		::moq::buf to_string() const;
		::moq::buf debug() const;
	};
	struct hang_catalog_VP9 {
		::std::uint8_t profile;
		::std::uint8_t level;
		::std::uint8_t bit_depth;
		::std::uint8_t chroma_subsampling;
		::std::uint8_t color_primaries;
		::std::uint8_t transfer_characteristics;
		::std::uint8_t matrix_coefficients;
		bool full_range;
		friend bool operator==(const hang_catalog_VP9 &a, const hang_catalog_VP9 &b);
		friend bool operator!=(const hang_catalog_VP9 &a, const hang_catalog_VP9 &b);
		::moq::buf to_string() const;
		::moq::buf debug() const;
		static hang_catalog_VP9 default_();
	};
	struct hang_catalog_Display {
		::std::uint32_t width;
		::std::uint32_t height;
		friend bool operator==(const hang_catalog_Display &a, const hang_catalog_Display &b);
		friend bool operator!=(const hang_catalog_Display &a, const hang_catalog_Display &b);
		::moq::buf debug() const;
	};
	struct hang_container_Frame {
		::moq::Timestamp timestamp;
		::moq::buf payload;
		::moq::buf debug() const;
		::moq::result<void, ::moq::hang_Error> encode(::moq::buf &buf_) const;
		static ::moq::result<::moq::hang_container_Frame, ::moq::hang_Error> decode(::moq::input buf_);
		::moq::result<void, ::moq::hang_Error> write_to(::moq::group_Producer &group) const;
	};
	struct hang_catalog_AudioCodec_AAC {
		::moq::hang_catalog_AAC _0;
	};
	struct hang_catalog_AudioCodec_Opus {
	};
	struct hang_catalog_AudioCodec_Pcm {
	};
	struct hang_catalog_AudioCodec_Flac {
	};
	struct hang_catalog_AudioCodec_Mp3 {
	};
	struct hang_catalog_AudioCodec_Mp2 {
	};
	struct hang_catalog_AudioCodec_Ac3 {
	};
	struct hang_catalog_AudioCodec_Ec3 {
	};
	struct hang_catalog_AudioCodec_Unknown {
		::moq::buf _0;
	};
	struct hang_catalog_AudioLayout_Ambisonic {
	};
	struct hang_catalog_AudioLayout_Unknown {
		::moq::buf _0;
	};
	struct hang_catalog_Mode_Snapshot {
	};
	struct hang_catalog_Mode_Stream {
	};
	struct hang_catalog_Mode_Unknown {
		::moq::buf _0;
	};
	struct hang_catalog_TextFormat_Vtt {
	};
	struct hang_catalog_TextFormat_Ttml {
	};
	struct hang_catalog_TextFormat_Utf8 {
	};
	struct hang_catalog_TextFormat_Unknown {
		::moq::buf _0;
	};
	struct hang_catalog_TextRole_Subtitle {
	};
	struct hang_catalog_TextRole_Caption {
	};
	struct hang_catalog_TextRole_Unknown {
		::moq::buf _0;
	};
	struct hang_catalog_VideoCodec_H264 {
		::moq::hang_catalog_H264 _0;
	};
	struct hang_catalog_VideoCodec_H265 {
		::moq::hang_catalog_H265 _0;
	};
	struct hang_catalog_VideoCodec_VP9 {
		::moq::hang_catalog_VP9 _0;
	};
	struct hang_catalog_VideoCodec_AV1 {
		::moq::hang_catalog_AV1 _0;
	};
	struct hang_catalog_VideoCodec_VP8 {
	};
	struct hang_catalog_VideoCodec_Unknown {
		::moq::buf _0;
	};
	struct hang_catalog_Stereo_Mono {
	};
	struct hang_catalog_Stereo_LeftRight {
	};
	struct hang_catalog_Stereo_TopBottom {
	};
	struct hang_catalog_Stereo_Unknown {
		::moq::buf _0;
	};
	inline bool operator==(const hang_catalog_AAC &a, const hang_catalog_AAC &b);
	inline bool operator==(const hang_catalog_Clock &a, const hang_catalog_Clock &b);
	inline bool operator==(const hang_catalog_Priorities &a, const hang_catalog_Priorities &b);
	inline bool operator==(const hang_catalog_AV1 &a, const hang_catalog_AV1 &b);
	inline bool operator==(const hang_catalog_Chromaticity &a, const hang_catalog_Chromaticity &b);
	inline bool operator==(const hang_catalog_Mastering &a, const hang_catalog_Mastering &b);
	inline bool operator==(const hang_catalog_ContentLight &a, const hang_catalog_ContentLight &b);
	inline bool operator==(const hang_catalog_H264 &a, const hang_catalog_H264 &b);
	inline bool operator==(const hang_catalog_H265 &a, const hang_catalog_H265 &b);
	inline bool operator==(const hang_catalog_VP9 &a, const hang_catalog_VP9 &b);
	inline bool operator==(const hang_catalog_Display &a, const hang_catalog_Display &b);
	inline bool operator==(const ::moq::hang_catalog_AudioCodec &a, const ::moq::hang_catalog_AudioCodec &b);
	inline bool operator!=(const ::moq::hang_catalog_AudioCodec &a, const ::moq::hang_catalog_AudioCodec &b);
	inline ::moq::buf hang_catalog_AudioCodec_to_string(const ::moq::hang_catalog_AudioCodec &self);
	inline ::moq::buf hang_catalog_AudioCodec_debug(const ::moq::hang_catalog_AudioCodec &self);
	inline ::moq::hang_catalog_AudioCodecKind hang_catalog_AudioCodec_kind(const ::moq::hang_catalog_AudioCodec &self);
	inline bool operator==(::moq::hang_catalog_AudioCodecKind a, ::moq::hang_catalog_AudioCodecKind b);
	inline bool operator!=(::moq::hang_catalog_AudioCodecKind a, ::moq::hang_catalog_AudioCodecKind b);
	inline ::moq::buf hang_catalog_AudioCodecKind_debug(::moq::hang_catalog_AudioCodecKind self);
	inline bool operator==(const ::moq::hang_catalog_AudioLayout &a, const ::moq::hang_catalog_AudioLayout &b);
	inline bool operator!=(const ::moq::hang_catalog_AudioLayout &a, const ::moq::hang_catalog_AudioLayout &b);
	inline ::moq::buf hang_catalog_AudioLayout_to_string(const ::moq::hang_catalog_AudioLayout &self);
	inline ::moq::buf hang_catalog_AudioLayout_debug(const ::moq::hang_catalog_AudioLayout &self);
	inline bool operator==(const ::moq::hang_catalog_Mode &a, const ::moq::hang_catalog_Mode &b);
	inline bool operator!=(const ::moq::hang_catalog_Mode &a, const ::moq::hang_catalog_Mode &b);
	inline ::moq::buf hang_catalog_Mode_to_string(const ::moq::hang_catalog_Mode &self);
	inline ::moq::buf hang_catalog_Mode_debug(const ::moq::hang_catalog_Mode &self);
	inline bool operator==(const ::moq::hang_catalog_TextFormat &a, const ::moq::hang_catalog_TextFormat &b);
	inline bool operator!=(const ::moq::hang_catalog_TextFormat &a, const ::moq::hang_catalog_TextFormat &b);
	inline ::moq::buf hang_catalog_TextFormat_to_string(const ::moq::hang_catalog_TextFormat &self);
	inline ::moq::buf hang_catalog_TextFormat_debug(const ::moq::hang_catalog_TextFormat &self);
	inline hang_catalog_TextFormat hang_catalog_TextFormat_default();
	inline bool operator==(const ::moq::hang_catalog_TextRole &a, const ::moq::hang_catalog_TextRole &b);
	inline bool operator!=(const ::moq::hang_catalog_TextRole &a, const ::moq::hang_catalog_TextRole &b);
	inline ::moq::buf hang_catalog_TextRole_to_string(const ::moq::hang_catalog_TextRole &self);
	inline ::moq::buf hang_catalog_TextRole_debug(const ::moq::hang_catalog_TextRole &self);
	inline hang_catalog_TextRole hang_catalog_TextRole_default();
	inline bool operator==(const ::moq::hang_catalog_VideoCodec &a, const ::moq::hang_catalog_VideoCodec &b);
	inline bool operator!=(const ::moq::hang_catalog_VideoCodec &a, const ::moq::hang_catalog_VideoCodec &b);
	inline ::moq::buf hang_catalog_VideoCodec_to_string(const ::moq::hang_catalog_VideoCodec &self);
	inline ::moq::buf hang_catalog_VideoCodec_debug(const ::moq::hang_catalog_VideoCodec &self);
	inline ::moq::hang_catalog_VideoCodecKind hang_catalog_VideoCodec_kind(const ::moq::hang_catalog_VideoCodec &self);
	inline bool operator==(::moq::hang_catalog_VideoCodecKind a, ::moq::hang_catalog_VideoCodecKind b);
	inline bool operator!=(::moq::hang_catalog_VideoCodecKind a, ::moq::hang_catalog_VideoCodecKind b);
	inline ::moq::buf hang_catalog_VideoCodecKind_debug(::moq::hang_catalog_VideoCodecKind self);
	inline bool operator==(::moq::hang_catalog_ColorRange a, ::moq::hang_catalog_ColorRange b);
	inline bool operator!=(::moq::hang_catalog_ColorRange a, ::moq::hang_catalog_ColorRange b);
	inline ::moq::buf hang_catalog_ColorRange_debug(::moq::hang_catalog_ColorRange self);
	inline bool operator==(const ::moq::hang_catalog_Stereo &a, const ::moq::hang_catalog_Stereo &b);
	inline bool operator!=(const ::moq::hang_catalog_Stereo &a, const ::moq::hang_catalog_Stereo &b);
	inline ::moq::buf hang_catalog_Stereo_to_string(const ::moq::hang_catalog_Stereo &self);
	inline ::moq::buf hang_catalog_Stereo_debug(const ::moq::hang_catalog_Stereo &self);
	inline ::moq::buf hang_echo_SUFFIX();
	inline ::moq::buf hang_echo_TRACK();
	inline bool hang_echo_is_echo(const ::moq::Path &path);
	inline ::moq::hang_catalog_Priorities hang_catalog_PRIORITY();
	inline ::moq::duration hang_container_MAX_AGE();
	inline ::moq::Timescale hang_container_TIMESCALE();
	inline ::moq::track_Info hang_container_track_info(::std::int8_t priority);
}
#endif

// json.hpp
#ifndef MOQ_HPP_CRATE7
#define MOQ_HPP_CRATE7
#include "rt.hpp"
#include "net.hpp"
#include "serdejson.hpp"
namespace moq {
	class RUST2_FFI_CONSUMABLE moq_json_Timed_Json {
		RUST2_FFI_UNCONSUMED moq_json_Timed_Json(moq_json_Timed_Json &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_json_Timed_Json &operator=(moq_json_Timed_Json &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_json_Timed_Json clone() const;
		friend bool operator==(const moq_json_Timed_Json &a, const moq_json_Timed_Json &b);
		friend bool operator!=(const moq_json_Timed_Json &a, const moq_json_Timed_Json &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::Json value_get() const;
		RUST2_FFI_CALLABLE void value_set(::moq::Json value);
		RUST2_FFI_CALLABLE ::moq::Timestamp at_get() const;
		RUST2_FFI_CALLABLE void at_set(const ::moq::Timestamp &value);
		static ::moq::moq_json_Timed_Json new_(::moq::Json value, const ::moq::Timestamp &at);
	};
}
#endif

// kio.hpp
#ifndef MOQ_HPP_CRATE1
#define MOQ_HPP_CRATE1
#include "rt.hpp"
#include "std.hpp"
namespace moq {
	enum class clock_MissedTickBehavior {
		burst = 0,
		delay = 1,
		skip = 2,
	};
	class RUST2_FFI_CONSUMABLE Park {
		RUST2_FFI_UNCONSUMED Park(Park &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Park &operator=(Park &&other) noexcept = default;
		RUST2_FFI_CALLABLE Park clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static Park default_();
		static ::moq::Park new_(::moq::Waiter waiter);
		RUST2_FFI_CALLABLE ::moq::Waiter hold(const ::moq::waker &cx);
	};
	class RUST2_FFI_CONSUMABLE Waiter {
		RUST2_FFI_UNCONSUMED Waiter(Waiter &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Waiter &operator=(Waiter &&other) noexcept = default;
		RUST2_FFI_CALLABLE Waiter clone() const;
		static ::moq::Waiter new_(const ::moq::waker &waker_);
		static ::moq::Waiter noop();
		RUST2_FFI_CALLABLE void register_(::moq::WaiterList &list) const;
		RUST2_FFI_CALLABLE ::moq::waker waker() const;
		RUST2_FFI_CALLABLE ::moq::waker context() const;
		template <class F0> RUST2_FFI_CALLABLE ::moq::buf batch(F0 &&poll) const;
		template <class F0> static ::moq::Waiter_poll_future_future poll_future(F0 future_);
	};
	class RUST2_FFI_CONSUMABLE WaiterList {
		RUST2_FFI_UNCONSUMED WaiterList(WaiterList &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		WaiterList &operator=(WaiterList &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static WaiterList default_();
		static ::moq::WaiterList new_();
		RUST2_FFI_CALLABLE void register_(const ::moq::Waiter &waiter);
		RUST2_FFI_CALLABLE ::moq::WaiterList take();
		RUST2_FFI_CALLABLE bool is_empty() const;
		RUST2_FFI_CALLABLE void wake();
	};
	class RUST2_FFI_CONSUMABLE clock_Clock {
		RUST2_FFI_UNCONSUMED clock_Clock(clock_Clock &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		clock_Clock &operator=(clock_Clock &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED clock_Clock(const clock_Clock &other) = default;
		clock_Clock &operator=(const clock_Clock &other) = default;
		friend bool operator==(const clock_Clock &a, const clock_Clock &b);
		friend bool operator!=(const clock_Clock &a, const clock_Clock &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::clock_Clock current();
		RUST2_FFI_CALLABLE ::moq::clock_Instant now() const;
		RUST2_FFI_CALLABLE ::moq::clock_Wall wall() const;
	};
	class RUST2_FFI_CONSUMABLE clock_Deadline {
		RUST2_FFI_UNCONSUMED clock_Deadline(clock_Deadline &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		clock_Deadline &operator=(clock_Deadline &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::clock_Deadline new_(const ::moq::clock_Deadlines &deadlines);
		static ::moq::clock_Deadline after(const ::moq::clock_Deadlines &deadlines, ::moq::duration duration_);
		RUST2_FFI_CALLABLE void set(const ::std::optional<::moq::clock_Instant> &at);
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> deadline() const;
		RUST2_FFI_CALLABLE ::moq::clock_Deadline_poll_future poll();
	};
	class RUST2_FFI_CONSUMABLE clock_Deadlines {
		RUST2_FFI_UNCONSUMED clock_Deadlines(clock_Deadlines &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		clock_Deadlines &operator=(clock_Deadlines &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED clock_Deadlines(const clock_Deadlines &other) = default;
		clock_Deadlines &operator=(const clock_Deadlines &other) = default;
		static ::moq::clock_Deadlines new_(const ::moq::clock_Instant &now);
		RUST2_FFI_CALLABLE void advance(const ::moq::clock_Instant &now) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> park(const ::moq::Waiter &waiter) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> next() const;
		RUST2_FFI_CALLABLE ::moq::clock_Instant now() const;
		RUST2_FFI_CALLABLE ::moq::clock_Entered enter() const;
	};
	class RUST2_FFI_CONSUMABLE clock_Entered {
		RUST2_FFI_UNCONSUMED clock_Entered(clock_Entered &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		clock_Entered &operator=(clock_Entered &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE clock_Manual {
		RUST2_FFI_UNCONSUMED clock_Manual(clock_Manual &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		clock_Manual &operator=(clock_Manual &&other) noexcept = default;
		RUST2_FFI_CALLABLE clock_Manual clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static clock_Manual default_();
		static ::moq::clock_Wall WALL_START();
		static ::moq::clock_Manual new_();
		RUST2_FFI_CALLABLE ::moq::clock_Instant now() const;
		RUST2_FFI_CALLABLE ::moq::clock_Wall wall() const;
		RUST2_FFI_CALLABLE void advance(::moq::duration duration_);
		RUST2_FFI_CALLABLE void step_wall(const ::moq::clock_Wall &wall);
	};
	class RUST2_FFI_CONSUMABLE clock_Rng {
		RUST2_FFI_UNCONSUMED clock_Rng(clock_Rng &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		clock_Rng &operator=(clock_Rng &&other) noexcept = default;
	};
	class RUST2_FFI_CONSUMABLE clock_Interval {
		RUST2_FFI_UNCONSUMED clock_Interval(clock_Interval &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		clock_Interval &operator=(clock_Interval &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::clock_Interval_tick_future tick();
		RUST2_FFI_CALLABLE void set_missed_tick_behavior(::moq::clock_MissedTickBehavior missed);
		RUST2_FFI_CALLABLE ::moq::duration period() const;
	};
	class RUST2_FFI_CONSUMABLE clock_WallMap {
		RUST2_FFI_UNCONSUMED clock_WallMap(clock_WallMap &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		clock_WallMap &operator=(clock_WallMap &&other) noexcept = default;
		RUST2_FFI_CALLABLE clock_WallMap clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static clock_WallMap default_();
		static ::moq::duration TOLERANCE();
		RUST2_FFI_CALLABLE void sample(const ::moq::clock_Instant &now, const ::moq::clock_Wall &wall);
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> map(const ::moq::clock_Wall &at) const;
	};
	class RUST2_FFI_CONSUMABLE coop_Budget {
		RUST2_FFI_UNCONSUMED coop_Budget(coop_Budget &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		coop_Budget &operator=(coop_Budget &&other) noexcept = default;
		RUST2_FFI_CALLABLE coop_Budget clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::coop_Budget new_(::std::uint32_t passes);
		RUST2_FFI_CALLABLE ::moq::coop_Budget_poll_yield_future poll_yield();
	};
	class clock_Instant {
		friend bool operator==(const clock_Instant &a, const clock_Instant &b);
		friend bool operator!=(const clock_Instant &a, const clock_Instant &b);
		friend bool operator<(const clock_Instant &a, const clock_Instant &b);
		friend bool operator<=(const clock_Instant &a, const clock_Instant &b);
		friend bool operator>(const clock_Instant &a, const clock_Instant &b);
		friend bool operator>=(const clock_Instant &a, const clock_Instant &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const clock_Instant &a, const clock_Instant &b);
		#endif
		::moq::buf debug() const;
		static ::moq::clock_Instant ZERO();
		static ::moq::clock_Instant now();
		::moq::duration elapsed() const;
		::moq::duration duration_since(const ::moq::clock_Instant &earlier) const;
		::std::optional<::moq::duration> checked_duration_since(const ::moq::clock_Instant &earlier) const;
		::moq::duration saturating_duration_since(const ::moq::clock_Instant &earlier) const;
		::std::optional<::moq::clock_Instant> checked_add(::moq::duration duration_) const;
		::std::optional<::moq::clock_Instant> checked_sub(::moq::duration duration_) const;
		::moq::clock_Instant saturating_add(::moq::duration duration_) const;
		::moq::clock_Instant saturating_sub(::moq::duration duration_) const;
		::moq::Instant into_std() const;
		static ::moq::clock_Instant from_std(const ::moq::Instant &at);
	};
	class clock_Elapsed {
		friend bool operator==(const clock_Elapsed &a, const clock_Elapsed &b);
		friend bool operator!=(const clock_Elapsed &a, const clock_Elapsed &b);
		::moq::buf to_string() const;
		::moq::buf debug() const;
	};
	class clock_Wall {
		friend bool operator==(const clock_Wall &a, const clock_Wall &b);
		friend bool operator!=(const clock_Wall &a, const clock_Wall &b);
		friend bool operator<(const clock_Wall &a, const clock_Wall &b);
		friend bool operator<=(const clock_Wall &a, const clock_Wall &b);
		friend bool operator>(const clock_Wall &a, const clock_Wall &b);
		friend bool operator>=(const clock_Wall &a, const clock_Wall &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const clock_Wall &a, const clock_Wall &b);
		#endif
		::moq::buf debug() const;
		static ::moq::clock_Wall UNIX_EPOCH();
		static ::moq::clock_Wall MOQ_EPOCH();
		static ::moq::clock_Wall now();
		::std::optional<::moq::duration> checked_duration_since(const ::moq::clock_Wall &earlier) const;
		::moq::duration saturating_duration_since(const ::moq::clock_Wall &earlier) const;
		::std::optional<::moq::clock_Wall> checked_add(::moq::duration duration_) const;
		::moq::clock_Wall saturating_add(::moq::duration duration_) const;
		::std::optional<::moq::clock_Wall> checked_sub(::moq::duration duration_) const;
	};
	inline bool operator==(const clock_Instant &a, const clock_Instant &b);
	inline bool operator<(const clock_Instant &a, const clock_Instant &b);
	inline bool operator<=(const clock_Instant &a, const clock_Instant &b);
	inline bool operator>(const clock_Instant &a, const clock_Instant &b);
	inline bool operator>=(const clock_Instant &a, const clock_Instant &b);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const clock_Instant &a, const clock_Instant &b);
	#endif
	inline bool operator==(const clock_Elapsed &a, const clock_Elapsed &b);
	inline bool operator==(const clock_Wall &a, const clock_Wall &b);
	inline bool operator<(const clock_Wall &a, const clock_Wall &b);
	inline bool operator<=(const clock_Wall &a, const clock_Wall &b);
	inline bool operator>(const clock_Wall &a, const clock_Wall &b);
	inline bool operator>=(const clock_Wall &a, const clock_Wall &b);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const clock_Wall &a, const clock_Wall &b);
	#endif
	inline bool operator==(::moq::clock_MissedTickBehavior a, ::moq::clock_MissedTickBehavior b);
	inline bool operator!=(::moq::clock_MissedTickBehavior a, ::moq::clock_MissedTickBehavior b);
	inline ::moq::buf clock_MissedTickBehavior_debug(::moq::clock_MissedTickBehavior self);
	inline clock_MissedTickBehavior clock_MissedTickBehavior_default();
	inline ::moq::clock_Instant clock_now();
	inline ::moq::clock_Wall clock_wall();
	inline ::std::tuple<::moq::clock_Instant, ::moq::clock_Wall> clock_origin();
	inline ::moq::clock_Rng clock_rng();
	inline ::std::uint64_t clock_seed();
	inline void clock_host_time();
	template <class F0> inline ::moq::clock_drive_future clock_drive(F0 &&poll);
	inline ::moq::clock_Interval clock_interval(::moq::duration period);
	inline ::moq::clock_Interval clock_interval_at(const ::moq::clock_Instant &start, ::moq::duration period);
	inline ::moq::clock_sleep_future clock_sleep(::moq::duration duration_);
	inline ::moq::clock_sleep_until_future clock_sleep_until(const ::moq::clock_Instant &at);
	template <class F0> inline ::moq::clock_timeout_future clock_timeout(::moq::duration duration_, F0 future_);
	template <class F0> inline ::moq::clock_timeout_at_future clock_timeout_at(const ::moq::clock_Instant &deadline, F0 future_);
}
#endif

// mux.hpp
#ifndef MOQ_HPP_CRATE9
#define MOQ_HPP_CRATE9
#include "rt.hpp"
#include "hang.hpp"
#include "kio.hpp"
#include "json.hpp"
#include "net.hpp"
#include "serdejson.hpp"
namespace moq {
	enum class mux_catalog_CatalogFormat {
		hang = 0,
		msf = 1,
	};
	enum class mux_container_Kind {
		audio = 0,
		video = 1,
		data = 2,
	};
	enum class mux_import_AudioFormat {
		aac = 0,
		opus = 1,
		flac = 2,
		mp3 = 3,
	};
	enum class mux_import_ContainerFormat {
		fmp4 = 0,
		mkv = 1,
		ts = 2,
		flv = 3,
	};
	enum class mux_import_Kind {
		audio = 0,
		video = 1,
		container = 2,
	};
	enum class mux_import_VideoFormat {
		avc1 = 0,
		avc3 = 1,
		hvc1 = 2,
		hev1 = 3,
		av01 = 4,
		vp8 = 5,
		vp9 = 6,
	};
	enum class mux_codec_flac_Error {
		short_ = 0,
		invalid_marker = 1,
		missing_stream_info = 2,
	};
	enum class mux_codec_mp3_Error {
		header_too_short = 0,
		missing_sync = 1,
		reserved_version = 2,
		not_layer3 = 3,
		reserved_sample_rate = 4,
	};
	enum class mux_codec_vp8_Error {
		frame_too_short = 0,
		keyframe_header_too_short = 1,
		start_code_mismatch = 2,
		empty_frame = 3,
	};
	enum class mux_codec_vp9_Error {
		invalid_frame_marker = 0,
		invalid_sync_code = 1,
		truncated = 2,
		empty_frame = 3,
	};
	enum class mux_container_ts_Framing {
		section = 0,
	};
	enum class mux_container_ts_stats_Class {
		audio = 1,
		video = 2,
		data = 0,
	};
	class RUST2_FFI_CONSUMABLE hang_Catalog_Extra {
		RUST2_FFI_UNCONSUMED hang_Catalog_Extra(hang_Catalog_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_Catalog_Extra &operator=(hang_Catalog_Extra &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_Catalog_Extra clone() const;
		friend bool operator==(const hang_Catalog_Extra &a, const hang_Catalog_Extra &b);
		friend bool operator!=(const hang_Catalog_Extra &a, const hang_Catalog_Extra &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_Catalog_Extra default_();
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Video video_get() const;
		RUST2_FFI_CALLABLE void video_set(::moq::hang_catalog_Video value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Audio audio_get() const;
		RUST2_FFI_CALLABLE void audio_set(::moq::hang_catalog_Audio value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Timeline sync_get() const;
		RUST2_FFI_CALLABLE void sync_set(const ::moq::hang_catalog_Timeline &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Text text_get() const;
		RUST2_FFI_CALLABLE void text_set(::moq::hang_catalog_Text value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Json json_get() const;
		RUST2_FFI_CALLABLE void json_set(::moq::hang_catalog_Json value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Binary binary_get() const;
		RUST2_FFI_CALLABLE void binary_set(::moq::hang_catalog_Binary value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Stats> stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(::std::optional<::moq::hang_catalog_Stats> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Echo> echo_get() const;
		RUST2_FFI_CALLABLE void echo_set(::std::optional<::moq::hang_catalog_Echo> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_Include>> include_get() const;
		RUST2_FFI_CALLABLE void include_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_Include>> value);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_hang_Extra ext_get() const;
		RUST2_FFI_CALLABLE void ext_set(::moq::mux_catalog_hang_Extra value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Skipped skipped_get() const;
		RUST2_FFI_CALLABLE void skipped_set(::moq::hang_catalog_Skipped value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> check_renditions() const;
		RUST2_FFI_CALLABLE ::moq::hang_Catalog without_ext() const;
		static ::moq::result<::moq::hang_Catalog_Extra, ::moq::hang_Error> from_slice(::moq::input v);
		static ::moq::result<::moq::hang_Catalog_Extra, ::moq::hang_Error> from_reader(::moq::input reader);
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_vec() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> to_writer(::moq::input writer) const;
	};
	class RUST2_FFI_CONSUMABLE hang_Catalog_Ext {
		RUST2_FFI_UNCONSUMED hang_Catalog_Ext(hang_Catalog_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_Catalog_Ext &operator=(hang_Catalog_Ext &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_Catalog_Ext clone() const;
		friend bool operator==(const hang_Catalog_Ext &a, const hang_Catalog_Ext &b);
		friend bool operator!=(const hang_Catalog_Ext &a, const hang_Catalog_Ext &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_Catalog_Ext default_();
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Video video_get() const;
		RUST2_FFI_CALLABLE void video_set(::moq::hang_catalog_Video value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Audio audio_get() const;
		RUST2_FFI_CALLABLE void audio_set(::moq::hang_catalog_Audio value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Timeline sync_get() const;
		RUST2_FFI_CALLABLE void sync_set(const ::moq::hang_catalog_Timeline &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Text text_get() const;
		RUST2_FFI_CALLABLE void text_set(::moq::hang_catalog_Text value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Json json_get() const;
		RUST2_FFI_CALLABLE void json_set(::moq::hang_catalog_Json value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Binary binary_get() const;
		RUST2_FFI_CALLABLE void binary_set(::moq::hang_catalog_Binary value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Stats> stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(::std::optional<::moq::hang_catalog_Stats> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Echo> echo_get() const;
		RUST2_FFI_CALLABLE void echo_set(::std::optional<::moq::hang_catalog_Echo> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_Include>> include_get() const;
		RUST2_FFI_CALLABLE void include_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_Include>> value);
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Ext ext_get() const;
		RUST2_FFI_CALLABLE void ext_set(::moq::mux_container_ts_Ext value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Skipped skipped_get() const;
		RUST2_FFI_CALLABLE void skipped_set(::moq::hang_catalog_Skipped value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> check_renditions() const;
		RUST2_FFI_CALLABLE ::moq::hang_Catalog without_ext() const;
		static ::moq::result<::moq::hang_Catalog_Ext, ::moq::hang_Error> from_slice(::moq::input v);
		static ::moq::result<::moq::hang_Catalog_Ext, ::moq::hang_Error> from_reader(::moq::input reader);
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_vec() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> to_writer(::moq::input writer) const;
	};
	class RUST2_FFI_CONSUMABLE mux_Clock {
		RUST2_FFI_UNCONSUMED mux_Clock(mux_Clock &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_Clock &operator=(mux_Clock &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_Clock(const mux_Clock &other) = default;
		mux_Clock &operator=(const mux_Clock &other) = default;
		friend bool operator==(const mux_Clock &a, const mux_Clock &b);
		friend bool operator!=(const mux_Clock &a, const mux_Clock &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_Clock default_();
		static ::moq::Timescale TIMESCALE();
		static ::moq::mux_Clock new_();
		static ::moq::result<::moq::mux_Clock, ::moq::mux_Error> at(const ::moq::clock_Instant &epoch, const ::moq::clock_Wall &wall);
		RUST2_FFI_CALLABLE ::moq::Timestamp now() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> capture(const ::moq::clock_Instant &at) const;
		RUST2_FFI_CALLABLE ::moq::Timescale timescale() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Clock> wall() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::clock_Wall, ::moq::mux_Error> wall_clock(const ::moq::Timestamp &pts) const;
	};
	class RUST2_FFI_CONSUMABLE mux_Ingest {
		RUST2_FFI_UNCONSUMED mux_Ingest(mux_Ingest &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_Ingest &operator=(mux_Ingest &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_Ingest clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_Ingest default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> max_age_get() const;
		RUST2_FFI_CALLABLE void max_age_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::moq::bandwidth_Allocator bandwidth_get() const;
		RUST2_FFI_CALLABLE void bandwidth_set(::moq::bandwidth_Allocator value);
		RUST2_FFI_CALLABLE ::moq::duration linger_get() const;
		RUST2_FFI_CALLABLE void linger_set(::moq::duration value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::bandwidth_Limit> rate_get() const;
		RUST2_FFI_CALLABLE void rate_set(::std::optional<::moq::bandwidth_Limit> value);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Config catalog() const;
	};
	class RUST2_FFI_CONSUMABLE mux_Linger_unit {
		RUST2_FFI_UNCONSUMED mux_Linger_unit(mux_Linger_unit &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_Linger_unit &operator=(mux_Linger_unit &&other) noexcept = default;
		static ::moq::duration DEFAULT();
		static ::moq::mux_Linger_unit new_(::std::vector<::moq::broadcast_Producer> broadcasts, ::moq::duration duration_);
		RUST2_FFI_CONSUMES ::moq::mux_Linger_unit_wait_future wait() &&;
	};
	class RUST2_FFI_CONSUMABLE mux_Pacer {
		RUST2_FFI_UNCONSUMED mux_Pacer(mux_Pacer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_Pacer &operator=(mux_Pacer &&other) noexcept = default;
		static mux_Pacer default_();
		RUST2_FFI_CONSUMES ::moq::mux_Pacer with_lead(::moq::duration lead) &&;
		RUST2_FFI_CONSUMES ::moq::mux_Pacer with_delay(::moq::duration delay) &&;
		RUST2_FFI_CALLABLE ::moq::clock_Instant pace(const ::moq::Timestamp &ts, const ::moq::clock_Instant &now);
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> due(const ::moq::Timestamp &ts) const;
		RUST2_FFI_CALLABLE ::moq::duration slack() const;
		RUST2_FFI_CALLABLE ::moq::clock_Instant hurry(const ::moq::Timestamp &ts, const ::moq::clock_Instant &now);
	};
	class RUST2_FFI_CONSUMABLE mux_Binding {
		RUST2_FFI_UNCONSUMED mux_Binding(mux_Binding &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_Binding &operator=(mux_Binding &&other) noexcept = default;
		static ::moq::mux_Binding new_(::moq::broadcast_Consumer broadcast);
		RUST2_FFI_CALLABLE ::moq::mux_Binding_broadcast_future broadcast() const;
	};
	class RUST2_FFI_CONSUMABLE mux_Source {
		RUST2_FFI_UNCONSUMED mux_Source(mux_Source &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_Source &operator=(mux_Source &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_Source clone() const;
		static ::moq::mux_Source new_(::moq::origin_Consumer origin, ::moq::broadcast_Consumer broadcast);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_echo_Rendition>> stats() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Consumer broadcast() const;
		RUST2_FFI_CALLABLE ::moq::mux_Source_catalog_Extra_future catalog_Extra(::moq::mux_catalog_CatalogFormat format) const;
		RUST2_FFI_CALLABLE ::moq::mux_Source_catalog_Ext_future catalog_Ext(::moq::mux_catalog_CatalogFormat format) const;
		RUST2_FFI_CALLABLE ::moq::mux_Source_catalog_unit_future catalog_unit(::moq::mux_catalog_CatalogFormat format) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Path> resolve_reference(const ::moq::path_Relative *rel) const;
		RUST2_FFI_CALLABLE ::moq::mux_Source_resolve_future resolve(const ::moq::path_Relative *rel, const ::moq::Instance *instance) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_Binding, ::moq::mux_Error> bind(const ::moq::path_Relative *rel, const ::moq::Instance *instance) const;
		RUST2_FFI_CALLABLE ::moq::mux_Source_subscribe_track_future subscribe_track(const ::moq::path_Relative *rel, const ::moq::Instance *instance, ::moq::input name) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_follow_Instances, ::moq::mux_Error> follow(const ::moq::path_Relative *rel, const ::moq::Instance *instance) const;
	};
	class RUST2_FFI_CONSUMABLE mux_Error {
		RUST2_FFI_UNCONSUMED mux_Error(mux_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_Error &operator=(mux_Error &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_Error(const mux_Error &other) = default;
		mux_Error &operator=(const mux_Error &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Consumer {
		RUST2_FFI_UNCONSUMED mux_catalog_Consumer(mux_catalog_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Consumer &operator=(mux_catalog_Consumer &&other) noexcept = default;
		static ::moq::mux_catalog_Consumer_new_future new_(const ::moq::broadcast_Consumer &broadcast, ::moq::mux_catalog_CatalogFormat format);
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Consumer_Extra {
		RUST2_FFI_UNCONSUMED mux_catalog_Consumer_Extra(mux_catalog_Consumer_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Consumer_Extra &operator=(mux_catalog_Consumer_Extra &&other) noexcept = default;
		static ::moq::mux_catalog_Consumer_Extra_new_future new_(const ::moq::broadcast_Consumer &broadcast, ::moq::mux_catalog_CatalogFormat format);
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Consumer_Ext {
		RUST2_FFI_UNCONSUMED mux_catalog_Consumer_Ext(mux_catalog_Consumer_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Consumer_Ext &operator=(mux_catalog_Consumer_Ext &&other) noexcept = default;
		static ::moq::mux_catalog_Consumer_Ext_new_future new_(const ::moq::broadcast_Consumer &broadcast, ::moq::mux_catalog_CatalogFormat format);
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Estimate {
		RUST2_FFI_UNCONSUMED mux_catalog_Estimate(mux_catalog_Estimate &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Estimate &operator=(mux_catalog_Estimate &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_Estimate clone() const;
		friend bool operator==(const mux_catalog_Estimate &a, const mux_catalog_Estimate &b);
		friend bool operator!=(const mux_catalog_Estimate &a, const mux_catalog_Estimate &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_catalog_Estimate default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> jitter_get() const;
		RUST2_FFI_CALLABLE void jitter_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bitrate_get() const;
		RUST2_FFI_CALLABLE void bitrate_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<double> framerate_get() const;
		RUST2_FFI_CALLABLE void framerate_set(const ::std::optional<double> &value);
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Estimate with_jitter(const ::std::optional<::moq::duration> &jitter) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Estimate with_bitrate(const ::std::optional<::std::uint64_t> &bitrate) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Estimate with_delay(const ::std::optional<::moq::duration> &delay) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Estimate with_framerate(const ::std::optional<double> &framerate) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Config {
		RUST2_FFI_UNCONSUMED mux_catalog_Config(mux_catalog_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Config &operator=(mux_catalog_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_Config clone() const;
		static mux_catalog_Config default_();
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config_Extra with_catalog_Extra(::moq::hang_Catalog_Extra catalog) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config_Ext with_catalog_Ext(::moq::hang_Catalog_Ext catalog) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config with_catalog_unit(::moq::hang_Catalog catalog) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config with_clock(::moq::mux_Clock clock) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config with_max_age(const ::std::optional<::moq::duration> &max_age) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config with_bandwidth(::moq::bandwidth_Allocator bandwidth) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Config_Extra {
		RUST2_FFI_UNCONSUMED mux_catalog_Config_Extra(mux_catalog_Config_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Config_Extra &operator=(mux_catalog_Config_Extra &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_Config_Extra clone() const;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config_Extra with_catalog_Extra(::moq::hang_Catalog_Extra catalog) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config_Ext with_catalog_Ext(::moq::hang_Catalog_Ext catalog) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config with_catalog_unit(::moq::hang_Catalog catalog) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config_Extra with_clock(::moq::mux_Clock clock) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config_Extra with_max_age(const ::std::optional<::moq::duration> &max_age) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config_Extra with_bandwidth(::moq::bandwidth_Allocator bandwidth) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Config_Ext {
		RUST2_FFI_UNCONSUMED mux_catalog_Config_Ext(mux_catalog_Config_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Config_Ext &operator=(mux_catalog_Config_Ext &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_Config_Ext clone() const;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config_Extra with_catalog_Extra(::moq::hang_Catalog_Extra catalog) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config_Ext with_catalog_Ext(::moq::hang_Catalog_Ext catalog) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config with_catalog_unit(::moq::hang_Catalog catalog) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config_Ext with_clock(::moq::mux_Clock clock) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config_Ext with_max_age(const ::std::optional<::moq::duration> &max_age) &&;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Config_Ext with_bandwidth(::moq::bandwidth_Allocator bandwidth) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Guard {
		RUST2_FFI_UNCONSUMED mux_catalog_Guard(mux_catalog_Guard &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Guard &operator=(mux_catalog_Guard &&other) noexcept = default;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_Error> commit() &&;
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Video video_get() const;
		RUST2_FFI_CALLABLE void video_set(::moq::hang_catalog_Video value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Audio audio_get() const;
		RUST2_FFI_CALLABLE void audio_set(::moq::hang_catalog_Audio value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Timeline sync_get() const;
		RUST2_FFI_CALLABLE void sync_set(const ::moq::hang_catalog_Timeline &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Text text_get() const;
		RUST2_FFI_CALLABLE void text_set(::moq::hang_catalog_Text value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Json json_get() const;
		RUST2_FFI_CALLABLE void json_set(::moq::hang_catalog_Json value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Binary binary_get() const;
		RUST2_FFI_CALLABLE void binary_set(::moq::hang_catalog_Binary value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Stats> stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(::std::optional<::moq::hang_catalog_Stats> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Echo> echo_get() const;
		RUST2_FFI_CALLABLE void echo_set(::std::optional<::moq::hang_catalog_Echo> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_Include>> include_get() const;
		RUST2_FFI_CALLABLE void include_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_Include>> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Skipped skipped_get() const;
		RUST2_FFI_CALLABLE void skipped_set(::moq::hang_catalog_Skipped value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> check_renditions() const;
		RUST2_FFI_CALLABLE ::moq::hang_Catalog without_ext() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_vec() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> to_writer(::moq::input writer) const;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Guard_Extra {
		RUST2_FFI_UNCONSUMED mux_catalog_Guard_Extra(mux_catalog_Guard_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Guard_Extra &operator=(mux_catalog_Guard_Extra &&other) noexcept = default;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_Error> commit() &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_section(::moq::input name, ::moq::Json value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Json> remove_section(::moq::input name);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Video video_get() const;
		RUST2_FFI_CALLABLE void video_set(::moq::hang_catalog_Video value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Audio audio_get() const;
		RUST2_FFI_CALLABLE void audio_set(::moq::hang_catalog_Audio value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Timeline sync_get() const;
		RUST2_FFI_CALLABLE void sync_set(const ::moq::hang_catalog_Timeline &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Text text_get() const;
		RUST2_FFI_CALLABLE void text_set(::moq::hang_catalog_Text value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Json json_get() const;
		RUST2_FFI_CALLABLE void json_set(::moq::hang_catalog_Json value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Binary binary_get() const;
		RUST2_FFI_CALLABLE void binary_set(::moq::hang_catalog_Binary value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Stats> stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(::std::optional<::moq::hang_catalog_Stats> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Echo> echo_get() const;
		RUST2_FFI_CALLABLE void echo_set(::std::optional<::moq::hang_catalog_Echo> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_Include>> include_get() const;
		RUST2_FFI_CALLABLE void include_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_Include>> value);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_hang_Extra ext_get() const;
		RUST2_FFI_CALLABLE void ext_set(::moq::mux_catalog_hang_Extra value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Skipped skipped_get() const;
		RUST2_FFI_CALLABLE void skipped_set(::moq::hang_catalog_Skipped value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> check_renditions() const;
		RUST2_FFI_CALLABLE ::moq::hang_Catalog without_ext() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_vec() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> to_writer(::moq::input writer) const;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Guard_Ext {
		RUST2_FFI_UNCONSUMED mux_catalog_Guard_Ext(mux_catalog_Guard_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Guard_Ext &operator=(mux_catalog_Guard_Ext &&other) noexcept = default;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_Error> commit() &&;
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Video video_get() const;
		RUST2_FFI_CALLABLE void video_set(::moq::hang_catalog_Video value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Audio audio_get() const;
		RUST2_FFI_CALLABLE void audio_set(::moq::hang_catalog_Audio value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Timeline sync_get() const;
		RUST2_FFI_CALLABLE void sync_set(const ::moq::hang_catalog_Timeline &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Text text_get() const;
		RUST2_FFI_CALLABLE void text_set(::moq::hang_catalog_Text value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Json json_get() const;
		RUST2_FFI_CALLABLE void json_set(::moq::hang_catalog_Json value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Binary binary_get() const;
		RUST2_FFI_CALLABLE void binary_set(::moq::hang_catalog_Binary value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Stats> stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(::std::optional<::moq::hang_catalog_Stats> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Echo> echo_get() const;
		RUST2_FFI_CALLABLE void echo_set(::std::optional<::moq::hang_catalog_Echo> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_Include>> include_get() const;
		RUST2_FFI_CALLABLE void include_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_Include>> value);
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Ext ext_get() const;
		RUST2_FFI_CALLABLE void ext_set(::moq::mux_container_ts_Ext value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Skipped skipped_get() const;
		RUST2_FFI_CALLABLE void skipped_set(::moq::hang_catalog_Skipped value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> check_renditions() const;
		RUST2_FFI_CALLABLE ::moq::hang_Catalog without_ext() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_vec() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> to_writer(::moq::input writer) const;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Producer {
		RUST2_FFI_UNCONSUMED mux_catalog_Producer(mux_catalog_Producer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Producer &operator=(mux_catalog_Producer &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_Producer clone() const;
		static ::moq::result<::moq::mux_catalog_Producer, ::moq::mux_Error> new_(::moq::broadcast_Producer &broadcast, ::moq::mux_catalog_Config config);
		RUST2_FFI_CALLABLE ::moq::track_Info track_info(::std::int8_t priority) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		template <class F0> RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> mutate(F0 &&f);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_catalog_Guard, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::hang_Catalog snapshot() const;
		RUST2_FFI_CALLABLE ::moq::mux_Clock clock() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Timebase timebase() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Reserved reserve() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_VideoConfig, ::moq::mux_Error> video_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_VideoConfig, ::moq::mux_Error> video_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_VideoConfig, ::moq::mux_Error> video_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_VideoConfig, ::moq::mux_Error> video_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_AudioConfig, ::moq::mux_Error> audio_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_AudioConfig, ::moq::mux_Error> audio_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_AudioConfig, ::moq::mux_Error> audio_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_AudioConfig, ::moq::mux_Error> audio_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_TextConfig, ::moq::mux_Error> text_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_TextConfig, ::moq::mux_Error> text_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_TextConfig, ::moq::mux_Error> text_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_TextConfig, ::moq::mux_Error> text_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_JsonConfig, ::moq::mux_Error> track_Container_JsonConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_BinaryConfig, ::moq::mux_Error> track_Container_BinaryConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_VideoConfig, ::moq::mux_Error> track_Container_VideoConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_AudioConfig, ::moq::mux_Error> track_Container_AudioConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_TextConfig, ::moq::mux_Error> track_Container_TextConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_JsonConfig, ::moq::mux_Error> track_fmp4_Wire_JsonConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_BinaryConfig, ::moq::mux_Error> track_fmp4_Wire_BinaryConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_VideoConfig, ::moq::mux_Error> track_fmp4_Wire_VideoConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_AudioConfig, ::moq::mux_Error> track_fmp4_Wire_AudioConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_TextConfig, ::moq::mux_Error> track_fmp4_Wire_TextConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_JsonConfig, ::moq::mux_Error> track_legacy_Wire_JsonConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_BinaryConfig, ::moq::mux_Error> track_legacy_Wire_BinaryConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_VideoConfig, ::moq::mux_Error> track_legacy_Wire_VideoConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_AudioConfig, ::moq::mux_Error> track_legacy_Wire_AudioConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_TextConfig, ::moq::mux_Error> track_legacy_Wire_TextConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_JsonConfig, ::moq::mux_Error> track_loc_Wire_JsonConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_BinaryConfig, ::moq::mux_Error> track_loc_Wire_BinaryConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_VideoConfig, ::moq::mux_Error> track_loc_Wire_VideoConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_AudioConfig, ::moq::mux_Error> track_loc_Wire_AudioConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_TextConfig, ::moq::mux_Error> track_loc_Wire_TextConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE bool is_claimed_JsonConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE bool is_claimed_BinaryConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE bool is_claimed_VideoConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE bool is_claimed_AudioConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE bool is_claimed_TextConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_stats_Rendition>> stats() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_json_Snapshot_Json, ::moq::mux_Error> json_snapshot_Config(::moq::track_Producer track, ::moq::mux_json_Config config, ::moq::mux_json_Options options) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_json_Snapshot_Json, ::moq::mux_Error> json_snapshot_JsonConfig(::moq::track_Producer track, ::moq::hang_catalog_JsonConfig config, ::moq::mux_json_Options options) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_json_Stream_Json, ::moq::mux_Error> json_stream_Config(::moq::track_Producer track, ::moq::mux_json_Config config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_json_Stream_Json, ::moq::mux_Error> json_stream_JsonConfig(::moq::track_Producer track, ::moq::hang_catalog_JsonConfig config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_catalog_hang_Consumer, ::moq::Error> consume() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Producer_Extra {
		RUST2_FFI_UNCONSUMED mux_catalog_Producer_Extra(mux_catalog_Producer_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Producer_Extra &operator=(mux_catalog_Producer_Extra &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_Producer_Extra clone() const;
		static ::moq::result<::moq::mux_catalog_Producer_Extra, ::moq::mux_Error> new_(::moq::broadcast_Producer &broadcast, ::moq::mux_catalog_Config_Extra config);
		RUST2_FFI_CALLABLE ::moq::track_Info track_info(::std::int8_t priority) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		template <class F0> RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> mutate(F0 &&f);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_catalog_Guard_Extra, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::hang_Catalog_Extra snapshot() const;
		RUST2_FFI_CALLABLE ::moq::mux_Clock clock() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Timebase_Extra timebase() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Reserved_Extra reserve() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_VideoConfig, ::moq::mux_Error> video_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_VideoConfig, ::moq::mux_Error> video_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_VideoConfig, ::moq::mux_Error> video_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_VideoConfig, ::moq::mux_Error> video_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_AudioConfig, ::moq::mux_Error> audio_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_AudioConfig, ::moq::mux_Error> audio_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_AudioConfig, ::moq::mux_Error> audio_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_AudioConfig, ::moq::mux_Error> audio_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_TextConfig, ::moq::mux_Error> text_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_TextConfig, ::moq::mux_Error> text_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_TextConfig, ::moq::mux_Error> text_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_TextConfig, ::moq::mux_Error> text_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_JsonConfig, ::moq::mux_Error> track_Container_JsonConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_BinaryConfig, ::moq::mux_Error> track_Container_BinaryConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_VideoConfig, ::moq::mux_Error> track_Container_VideoConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_AudioConfig, ::moq::mux_Error> track_Container_AudioConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_TextConfig, ::moq::mux_Error> track_Container_TextConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_JsonConfig, ::moq::mux_Error> track_fmp4_Wire_JsonConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_BinaryConfig, ::moq::mux_Error> track_fmp4_Wire_BinaryConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_VideoConfig, ::moq::mux_Error> track_fmp4_Wire_VideoConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_AudioConfig, ::moq::mux_Error> track_fmp4_Wire_AudioConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_TextConfig, ::moq::mux_Error> track_fmp4_Wire_TextConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_JsonConfig, ::moq::mux_Error> track_legacy_Wire_JsonConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_BinaryConfig, ::moq::mux_Error> track_legacy_Wire_BinaryConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_VideoConfig, ::moq::mux_Error> track_legacy_Wire_VideoConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_AudioConfig, ::moq::mux_Error> track_legacy_Wire_AudioConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_TextConfig, ::moq::mux_Error> track_legacy_Wire_TextConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_JsonConfig, ::moq::mux_Error> track_loc_Wire_JsonConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_BinaryConfig, ::moq::mux_Error> track_loc_Wire_BinaryConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_VideoConfig, ::moq::mux_Error> track_loc_Wire_VideoConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_AudioConfig, ::moq::mux_Error> track_loc_Wire_AudioConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_TextConfig, ::moq::mux_Error> track_loc_Wire_TextConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE bool is_claimed_JsonConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE bool is_claimed_BinaryConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE bool is_claimed_VideoConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE bool is_claimed_AudioConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE bool is_claimed_TextConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_stats_Rendition>> stats() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_json_Snapshot_Json, ::moq::mux_Error> json_snapshot_Config(::moq::track_Producer track, ::moq::mux_json_Config config, ::moq::mux_json_Options options) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_json_Snapshot_Json, ::moq::mux_Error> json_snapshot_JsonConfig(::moq::track_Producer track, ::moq::hang_catalog_JsonConfig config, ::moq::mux_json_Options options) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_json_Stream_Json, ::moq::mux_Error> json_stream_Config(::moq::track_Producer track, ::moq::mux_json_Config config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_json_Stream_Json, ::moq::mux_Error> json_stream_JsonConfig(::moq::track_Producer track, ::moq::hang_catalog_JsonConfig config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_catalog_hang_Consumer_Extra, ::moq::Error> consume() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Producer_Ext {
		RUST2_FFI_UNCONSUMED mux_catalog_Producer_Ext(mux_catalog_Producer_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Producer_Ext &operator=(mux_catalog_Producer_Ext &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_Producer_Ext clone() const;
		static ::moq::result<::moq::mux_catalog_Producer_Ext, ::moq::mux_Error> new_(::moq::broadcast_Producer &broadcast, ::moq::mux_catalog_Config_Ext config);
		RUST2_FFI_CALLABLE ::moq::track_Info track_info(::std::int8_t priority) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		template <class F0> RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> mutate(F0 &&f);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_catalog_Guard_Ext, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::hang_Catalog_Ext snapshot() const;
		RUST2_FFI_CALLABLE ::moq::mux_Clock clock() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Timebase_Ext timebase() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Reserved_Ext reserve() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_VideoConfig, ::moq::mux_Error> video_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_VideoConfig, ::moq::mux_Error> video_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_VideoConfig, ::moq::mux_Error> video_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_VideoConfig, ::moq::mux_Error> video_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_AudioConfig, ::moq::mux_Error> audio_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_AudioConfig, ::moq::mux_Error> audio_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_AudioConfig, ::moq::mux_Error> audio_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_AudioConfig, ::moq::mux_Error> audio_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_TextConfig, ::moq::mux_Error> text_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_TextConfig, ::moq::mux_Error> text_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_TextConfig, ::moq::mux_Error> text_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_TextConfig, ::moq::mux_Error> text_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_JsonConfig, ::moq::mux_Error> track_Container_JsonConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_BinaryConfig, ::moq::mux_Error> track_Container_BinaryConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_VideoConfig, ::moq::mux_Error> track_Container_VideoConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_AudioConfig, ::moq::mux_Error> track_Container_AudioConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_TextConfig, ::moq::mux_Error> track_Container_TextConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_JsonConfig, ::moq::mux_Error> track_fmp4_Wire_JsonConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_BinaryConfig, ::moq::mux_Error> track_fmp4_Wire_BinaryConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_VideoConfig, ::moq::mux_Error> track_fmp4_Wire_VideoConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_AudioConfig, ::moq::mux_Error> track_fmp4_Wire_AudioConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_TextConfig, ::moq::mux_Error> track_fmp4_Wire_TextConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_JsonConfig, ::moq::mux_Error> track_legacy_Wire_JsonConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_BinaryConfig, ::moq::mux_Error> track_legacy_Wire_BinaryConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_VideoConfig, ::moq::mux_Error> track_legacy_Wire_VideoConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_AudioConfig, ::moq::mux_Error> track_legacy_Wire_AudioConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_TextConfig, ::moq::mux_Error> track_legacy_Wire_TextConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_JsonConfig, ::moq::mux_Error> track_loc_Wire_JsonConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_BinaryConfig, ::moq::mux_Error> track_loc_Wire_BinaryConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_VideoConfig, ::moq::mux_Error> track_loc_Wire_VideoConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_AudioConfig, ::moq::mux_Error> track_loc_Wire_AudioConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_TextConfig, ::moq::mux_Error> track_loc_Wire_TextConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE bool is_claimed_JsonConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE bool is_claimed_BinaryConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE bool is_claimed_VideoConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE bool is_claimed_AudioConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE bool is_claimed_TextConfig(::moq::input name) const;
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_stats_Rendition>> stats() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_json_Snapshot_Json, ::moq::mux_Error> json_snapshot_Config(::moq::track_Producer track, ::moq::mux_json_Config config, ::moq::mux_json_Options options) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_json_Snapshot_Json, ::moq::mux_Error> json_snapshot_JsonConfig(::moq::track_Producer track, ::moq::hang_catalog_JsonConfig config, ::moq::mux_json_Options options) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_json_Stream_Json, ::moq::mux_Error> json_stream_Config(::moq::track_Producer track, ::moq::mux_json_Config config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_json_Stream_Json, ::moq::mux_Error> json_stream_JsonConfig(::moq::track_Producer track, ::moq::hang_catalog_JsonConfig config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_catalog_hang_Consumer_Ext, ::moq::Error> consume() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Timebase {
		RUST2_FFI_UNCONSUMED mux_catalog_Timebase(mux_catalog_Timebase &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Timebase &operator=(mux_catalog_Timebase &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_Timebase clone() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Reserved reserve() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> place(const ::moq::Timestamp &pts, const ::moq::clock_Wall &wall) const;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Timebase_Extra {
		RUST2_FFI_UNCONSUMED mux_catalog_Timebase_Extra(mux_catalog_Timebase_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Timebase_Extra &operator=(mux_catalog_Timebase_Extra &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_Timebase_Extra clone() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Reserved_Extra reserve() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> place(const ::moq::Timestamp &pts, const ::moq::clock_Wall &wall) const;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Timebase_Ext {
		RUST2_FFI_UNCONSUMED mux_catalog_Timebase_Ext(mux_catalog_Timebase_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Timebase_Ext &operator=(mux_catalog_Timebase_Ext &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_Timebase_Ext clone() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Reserved_Ext reserve() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> place(const ::moq::Timestamp &pts, const ::moq::clock_Wall &wall) const;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Reserved {
		RUST2_FFI_UNCONSUMED mux_catalog_Reserved(mux_catalog_Reserved &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Reserved &operator=(mux_catalog_Reserved &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_Reserved clone() const;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Reserved with_interval(const ::std::optional<::moq::duration> &interval) &&;
		RUST2_FFI_CALLABLE ::moq::track_Info track_info(::std::int8_t priority) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_VideoConfig, ::moq::mux_Error> video_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_VideoConfig, ::moq::mux_Error> video_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_VideoConfig, ::moq::mux_Error> video_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_VideoConfig, ::moq::mux_Error> video_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_AudioConfig, ::moq::mux_Error> audio_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_AudioConfig, ::moq::mux_Error> audio_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_AudioConfig, ::moq::mux_Error> audio_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_AudioConfig, ::moq::mux_Error> audio_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_TextConfig, ::moq::mux_Error> text_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_TextConfig, ::moq::mux_Error> text_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_TextConfig, ::moq::mux_Error> text_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_TextConfig, ::moq::mux_Error> text_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_JsonConfig, ::moq::mux_Error> track_Container_JsonConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_BinaryConfig, ::moq::mux_Error> track_Container_BinaryConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_VideoConfig, ::moq::mux_Error> track_Container_VideoConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_AudioConfig, ::moq::mux_Error> track_Container_AudioConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_TextConfig, ::moq::mux_Error> track_Container_TextConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_JsonConfig, ::moq::mux_Error> track_fmp4_Wire_JsonConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_BinaryConfig, ::moq::mux_Error> track_fmp4_Wire_BinaryConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_VideoConfig, ::moq::mux_Error> track_fmp4_Wire_VideoConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_AudioConfig, ::moq::mux_Error> track_fmp4_Wire_AudioConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_TextConfig, ::moq::mux_Error> track_fmp4_Wire_TextConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_JsonConfig, ::moq::mux_Error> track_legacy_Wire_JsonConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_BinaryConfig, ::moq::mux_Error> track_legacy_Wire_BinaryConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_VideoConfig, ::moq::mux_Error> track_legacy_Wire_VideoConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_AudioConfig, ::moq::mux_Error> track_legacy_Wire_AudioConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_TextConfig, ::moq::mux_Error> track_legacy_Wire_TextConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_JsonConfig, ::moq::mux_Error> track_loc_Wire_JsonConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_BinaryConfig, ::moq::mux_Error> track_loc_Wire_BinaryConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_VideoConfig, ::moq::mux_Error> track_loc_Wire_VideoConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_AudioConfig, ::moq::mux_Error> track_loc_Wire_AudioConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_TextConfig, ::moq::mux_Error> track_loc_Wire_TextConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Reserved_Extra {
		RUST2_FFI_UNCONSUMED mux_catalog_Reserved_Extra(mux_catalog_Reserved_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Reserved_Extra &operator=(mux_catalog_Reserved_Extra &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_Reserved_Extra clone() const;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Reserved_Extra with_interval(const ::std::optional<::moq::duration> &interval) &&;
		RUST2_FFI_CALLABLE ::moq::track_Info track_info(::std::int8_t priority) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_VideoConfig, ::moq::mux_Error> video_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_VideoConfig, ::moq::mux_Error> video_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_VideoConfig, ::moq::mux_Error> video_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_VideoConfig, ::moq::mux_Error> video_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_AudioConfig, ::moq::mux_Error> audio_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_AudioConfig, ::moq::mux_Error> audio_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_AudioConfig, ::moq::mux_Error> audio_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_AudioConfig, ::moq::mux_Error> audio_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_TextConfig, ::moq::mux_Error> text_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_TextConfig, ::moq::mux_Error> text_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_TextConfig, ::moq::mux_Error> text_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_TextConfig, ::moq::mux_Error> text_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_JsonConfig, ::moq::mux_Error> track_Container_JsonConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_BinaryConfig, ::moq::mux_Error> track_Container_BinaryConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_VideoConfig, ::moq::mux_Error> track_Container_VideoConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_AudioConfig, ::moq::mux_Error> track_Container_AudioConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_TextConfig, ::moq::mux_Error> track_Container_TextConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_JsonConfig, ::moq::mux_Error> track_fmp4_Wire_JsonConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_BinaryConfig, ::moq::mux_Error> track_fmp4_Wire_BinaryConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_VideoConfig, ::moq::mux_Error> track_fmp4_Wire_VideoConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_AudioConfig, ::moq::mux_Error> track_fmp4_Wire_AudioConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_TextConfig, ::moq::mux_Error> track_fmp4_Wire_TextConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_JsonConfig, ::moq::mux_Error> track_legacy_Wire_JsonConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_BinaryConfig, ::moq::mux_Error> track_legacy_Wire_BinaryConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_VideoConfig, ::moq::mux_Error> track_legacy_Wire_VideoConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_AudioConfig, ::moq::mux_Error> track_legacy_Wire_AudioConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_TextConfig, ::moq::mux_Error> track_legacy_Wire_TextConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_JsonConfig, ::moq::mux_Error> track_loc_Wire_JsonConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_BinaryConfig, ::moq::mux_Error> track_loc_Wire_BinaryConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_VideoConfig, ::moq::mux_Error> track_loc_Wire_VideoConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_AudioConfig, ::moq::mux_Error> track_loc_Wire_AudioConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_TextConfig, ::moq::mux_Error> track_loc_Wire_TextConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_Reserved_Ext {
		RUST2_FFI_UNCONSUMED mux_catalog_Reserved_Ext(mux_catalog_Reserved_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Reserved_Ext &operator=(mux_catalog_Reserved_Ext &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_Reserved_Ext clone() const;
		RUST2_FFI_CONSUMES ::moq::mux_catalog_Reserved_Ext with_interval(const ::std::optional<::moq::duration> &interval) &&;
		RUST2_FFI_CALLABLE ::moq::track_Info track_info(::std::int8_t priority) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_VideoConfig, ::moq::mux_Error> video_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_VideoConfig, ::moq::mux_Error> video_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_VideoConfig, ::moq::mux_Error> video_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_VideoConfig, ::moq::mux_Error> video_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_AudioConfig, ::moq::mux_Error> audio_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_AudioConfig, ::moq::mux_Error> audio_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_AudioConfig, ::moq::mux_Error> audio_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_AudioConfig, ::moq::mux_Error> audio_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_TextConfig, ::moq::mux_Error> text_Container(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_TextConfig, ::moq::mux_Error> text_fmp4_Wire(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_TextConfig, ::moq::mux_Error> text_legacy_Wire(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_TextConfig, ::moq::mux_Error> text_loc_Wire(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_JsonConfig, ::moq::mux_Error> track_Container_JsonConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_BinaryConfig, ::moq::mux_Error> track_Container_BinaryConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_VideoConfig, ::moq::mux_Error> track_Container_VideoConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_AudioConfig, ::moq::mux_Error> track_Container_AudioConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_Container_TextConfig, ::moq::mux_Error> track_Container_TextConfig(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_JsonConfig, ::moq::mux_Error> track_fmp4_Wire_JsonConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_BinaryConfig, ::moq::mux_Error> track_fmp4_Wire_BinaryConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_VideoConfig, ::moq::mux_Error> track_fmp4_Wire_VideoConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_AudioConfig, ::moq::mux_Error> track_fmp4_Wire_AudioConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_fmp4_Wire_TextConfig, ::moq::mux_Error> track_fmp4_Wire_TextConfig(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_JsonConfig, ::moq::mux_Error> track_legacy_Wire_JsonConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_BinaryConfig, ::moq::mux_Error> track_legacy_Wire_BinaryConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_VideoConfig, ::moq::mux_Error> track_legacy_Wire_VideoConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_AudioConfig, ::moq::mux_Error> track_legacy_Wire_AudioConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_legacy_Wire_TextConfig, ::moq::mux_Error> track_legacy_Wire_TextConfig(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_JsonConfig, ::moq::mux_Error> track_loc_Wire_JsonConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_JsonConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_BinaryConfig, ::moq::mux_Error> track_loc_Wire_BinaryConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_BinaryConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_VideoConfig, ::moq::mux_Error> track_loc_Wire_VideoConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_VideoConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_AudioConfig, ::moq::mux_Error> track_loc_Wire_AudioConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_AudioConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Producer_loc_Wire_TextConfig, ::moq::mux_Error> track_loc_Wire_TextConfig(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container, ::std::optional<::moq::hang_catalog_TextConfig> config) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_VideoHint {
		RUST2_FFI_UNCONSUMED mux_catalog_VideoHint(mux_catalog_VideoHint &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_VideoHint &operator=(mux_catalog_VideoHint &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_VideoHint clone() const;
		friend bool operator==(const mux_catalog_VideoHint &a, const mux_catalog_VideoHint &b);
		friend bool operator!=(const mux_catalog_VideoHint &a, const mux_catalog_VideoHint &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_catalog_VideoHint default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_VideoCodec> codec_get() const;
		RUST2_FFI_CALLABLE void codec_set(const ::std::optional<::moq::hang_catalog_VideoCodec> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> coded_width_get() const;
		RUST2_FFI_CALLABLE void coded_width_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> coded_height_get() const;
		RUST2_FFI_CALLABLE void coded_height_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> display_aspect_width_get() const;
		RUST2_FFI_CALLABLE void display_aspect_width_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> display_aspect_height_get() const;
		RUST2_FFI_CALLABLE void display_aspect_height_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Color> color_get() const;
		RUST2_FFI_CALLABLE void color_set(::std::optional<::moq::hang_catalog_Color> value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bitrate_get() const;
		RUST2_FFI_CALLABLE void bitrate_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<double> framerate_get() const;
		RUST2_FFI_CALLABLE void framerate_set(const ::std::optional<double> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint8_t> reorder_get() const;
		RUST2_FFI_CALLABLE void reorder_set(const ::std::optional<::std::uint8_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<bool> optimize_for_latency_get() const;
		RUST2_FFI_CALLABLE void optimize_for_latency_set(const ::std::optional<bool> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> jitter_get() const;
		RUST2_FFI_CALLABLE void jitter_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Container container_get() const;
		RUST2_FFI_CALLABLE void container_set(::moq::hang_catalog_Container value);
		RUST2_FFI_CALLABLE ::std::int32_t preference_get() const;
		RUST2_FFI_CALLABLE void preference_set(::std::int32_t value);
		RUST2_FFI_CALLABLE void apply(::moq::hang_catalog_VideoConfig &config) const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Consumer_Container {
		RUST2_FFI_UNCONSUMED mux_container_Consumer_Container(mux_container_Consumer_Container &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Consumer_Container &operator=(mux_container_Consumer_Container &&other) noexcept = default;
		static ::moq::mux_container_Consumer_Container new_(::moq::track_Subscriber track, ::moq::mux_catalog_hang_Container format);
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> arrival() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> backlog() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Consumer_Container_read_future read();
		RUST2_FFI_CALLABLE bool is_skipped(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE void set_delay(const ::moq::track_Delay &delay);
		RUST2_FFI_CALLABLE void set_playhead(::moq::mux_sync_Consumer playhead);
	};
	class RUST2_FFI_CONSUMABLE mux_container_Consumer_fmp4_Wire {
		RUST2_FFI_UNCONSUMED mux_container_Consumer_fmp4_Wire(mux_container_Consumer_fmp4_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Consumer_fmp4_Wire &operator=(mux_container_Consumer_fmp4_Wire &&other) noexcept = default;
		static ::moq::mux_container_Consumer_fmp4_Wire new_(::moq::track_Subscriber track, ::moq::mux_container_fmp4_Wire format);
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> arrival() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> backlog() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Consumer_fmp4_Wire_read_future read();
		RUST2_FFI_CALLABLE bool is_skipped(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE void set_delay(const ::moq::track_Delay &delay);
		RUST2_FFI_CALLABLE void set_playhead(::moq::mux_sync_Consumer playhead);
	};
	class RUST2_FFI_CONSUMABLE mux_container_Consumer_legacy_Wire {
		RUST2_FFI_UNCONSUMED mux_container_Consumer_legacy_Wire(mux_container_Consumer_legacy_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Consumer_legacy_Wire &operator=(mux_container_Consumer_legacy_Wire &&other) noexcept = default;
		static ::moq::mux_container_Consumer_legacy_Wire new_(::moq::track_Subscriber track, const ::moq::mux_container_legacy_Wire &format);
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> arrival() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> backlog() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Consumer_legacy_Wire_read_future read();
		RUST2_FFI_CALLABLE bool is_skipped(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE void set_delay(const ::moq::track_Delay &delay);
		RUST2_FFI_CALLABLE void set_playhead(::moq::mux_sync_Consumer playhead);
	};
	class RUST2_FFI_CONSUMABLE mux_container_Consumer_loc_Wire {
		RUST2_FFI_UNCONSUMED mux_container_Consumer_loc_Wire(mux_container_Consumer_loc_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Consumer_loc_Wire &operator=(mux_container_Consumer_loc_Wire &&other) noexcept = default;
		static ::moq::mux_container_Consumer_loc_Wire new_(::moq::track_Subscriber track, const ::moq::mux_container_loc_Wire &format);
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> arrival() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> backlog() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Consumer_loc_Wire_read_future read();
		RUST2_FFI_CALLABLE bool is_skipped(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE void set_delay(const ::moq::track_Delay &delay);
		RUST2_FFI_CALLABLE void set_playhead(::moq::mux_sync_Consumer playhead);
	};
	class RUST2_FFI_CONSUMABLE mux_container_GroupConsumer_Container {
		RUST2_FFI_UNCONSUMED mux_container_GroupConsumer_Container(mux_container_GroupConsumer_Container &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_GroupConsumer_Container &operator=(mux_container_GroupConsumer_Container &&other) noexcept = default;
		static ::moq::mux_container_GroupConsumer_Container new_(::moq::group_Consumer group, ::moq::mux_catalog_hang_Container format);
		RUST2_FFI_CALLABLE ::std::uint64_t start() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_GroupConsumer_Container_read_future read();
	};
	class RUST2_FFI_CONSUMABLE mux_container_GroupConsumer_fmp4_Wire {
		RUST2_FFI_UNCONSUMED mux_container_GroupConsumer_fmp4_Wire(mux_container_GroupConsumer_fmp4_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_GroupConsumer_fmp4_Wire &operator=(mux_container_GroupConsumer_fmp4_Wire &&other) noexcept = default;
		static ::moq::mux_container_GroupConsumer_fmp4_Wire new_(::moq::group_Consumer group, ::moq::mux_container_fmp4_Wire format);
		RUST2_FFI_CALLABLE ::std::uint64_t start() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_GroupConsumer_fmp4_Wire_read_future read();
	};
	class RUST2_FFI_CONSUMABLE mux_container_GroupConsumer_legacy_Wire {
		RUST2_FFI_UNCONSUMED mux_container_GroupConsumer_legacy_Wire(mux_container_GroupConsumer_legacy_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_GroupConsumer_legacy_Wire &operator=(mux_container_GroupConsumer_legacy_Wire &&other) noexcept = default;
		static ::moq::mux_container_GroupConsumer_legacy_Wire new_(::moq::group_Consumer group, const ::moq::mux_container_legacy_Wire &format);
		RUST2_FFI_CALLABLE ::std::uint64_t start() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_GroupConsumer_legacy_Wire_read_future read();
	};
	class RUST2_FFI_CONSUMABLE mux_container_GroupConsumer_loc_Wire {
		RUST2_FFI_UNCONSUMED mux_container_GroupConsumer_loc_Wire(mux_container_GroupConsumer_loc_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_GroupConsumer_loc_Wire &operator=(mux_container_GroupConsumer_loc_Wire &&other) noexcept = default;
		static ::moq::mux_container_GroupConsumer_loc_Wire new_(::moq::group_Consumer group, const ::moq::mux_container_loc_Wire &format);
		RUST2_FFI_CALLABLE ::std::uint64_t start() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_GroupConsumer_loc_Wire_read_future read();
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_Container_AudioConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_Container_AudioConfig(mux_container_Finished_Container_AudioConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_Container_AudioConfig &operator=(mux_container_Finished_Container_AudioConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_Container_BinaryConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_Container_BinaryConfig(mux_container_Finished_Container_BinaryConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_Container_BinaryConfig &operator=(mux_container_Finished_Container_BinaryConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_Container_JsonConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_Container_JsonConfig(mux_container_Finished_Container_JsonConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_Container_JsonConfig &operator=(mux_container_Finished_Container_JsonConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_Container_TextConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_Container_TextConfig(mux_container_Finished_Container_TextConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_Container_TextConfig &operator=(mux_container_Finished_Container_TextConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_Container_VideoConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_Container_VideoConfig(mux_container_Finished_Container_VideoConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_Container_VideoConfig &operator=(mux_container_Finished_Container_VideoConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_Container {
		RUST2_FFI_UNCONSUMED mux_container_Finished_Container(mux_container_Finished_Container &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_Container &operator=(mux_container_Finished_Container &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_fmp4_Wire_AudioConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_fmp4_Wire_AudioConfig(mux_container_Finished_fmp4_Wire_AudioConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_fmp4_Wire_AudioConfig &operator=(mux_container_Finished_fmp4_Wire_AudioConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_fmp4_Wire_BinaryConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_fmp4_Wire_BinaryConfig(mux_container_Finished_fmp4_Wire_BinaryConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_fmp4_Wire_BinaryConfig &operator=(mux_container_Finished_fmp4_Wire_BinaryConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_fmp4_Wire_JsonConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_fmp4_Wire_JsonConfig(mux_container_Finished_fmp4_Wire_JsonConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_fmp4_Wire_JsonConfig &operator=(mux_container_Finished_fmp4_Wire_JsonConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_fmp4_Wire_TextConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_fmp4_Wire_TextConfig(mux_container_Finished_fmp4_Wire_TextConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_fmp4_Wire_TextConfig &operator=(mux_container_Finished_fmp4_Wire_TextConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_fmp4_Wire_VideoConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_fmp4_Wire_VideoConfig(mux_container_Finished_fmp4_Wire_VideoConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_fmp4_Wire_VideoConfig &operator=(mux_container_Finished_fmp4_Wire_VideoConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_fmp4_Wire {
		RUST2_FFI_UNCONSUMED mux_container_Finished_fmp4_Wire(mux_container_Finished_fmp4_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_fmp4_Wire &operator=(mux_container_Finished_fmp4_Wire &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_legacy_Wire_AudioConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_legacy_Wire_AudioConfig(mux_container_Finished_legacy_Wire_AudioConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_legacy_Wire_AudioConfig &operator=(mux_container_Finished_legacy_Wire_AudioConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_legacy_Wire_BinaryConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_legacy_Wire_BinaryConfig(mux_container_Finished_legacy_Wire_BinaryConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_legacy_Wire_BinaryConfig &operator=(mux_container_Finished_legacy_Wire_BinaryConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_legacy_Wire_JsonConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_legacy_Wire_JsonConfig(mux_container_Finished_legacy_Wire_JsonConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_legacy_Wire_JsonConfig &operator=(mux_container_Finished_legacy_Wire_JsonConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_legacy_Wire_TextConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_legacy_Wire_TextConfig(mux_container_Finished_legacy_Wire_TextConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_legacy_Wire_TextConfig &operator=(mux_container_Finished_legacy_Wire_TextConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_legacy_Wire_VideoConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_legacy_Wire_VideoConfig(mux_container_Finished_legacy_Wire_VideoConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_legacy_Wire_VideoConfig &operator=(mux_container_Finished_legacy_Wire_VideoConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_legacy_Wire {
		RUST2_FFI_UNCONSUMED mux_container_Finished_legacy_Wire(mux_container_Finished_legacy_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_legacy_Wire &operator=(mux_container_Finished_legacy_Wire &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_loc_Wire_AudioConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_loc_Wire_AudioConfig(mux_container_Finished_loc_Wire_AudioConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_loc_Wire_AudioConfig &operator=(mux_container_Finished_loc_Wire_AudioConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_loc_Wire_BinaryConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_loc_Wire_BinaryConfig(mux_container_Finished_loc_Wire_BinaryConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_loc_Wire_BinaryConfig &operator=(mux_container_Finished_loc_Wire_BinaryConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_loc_Wire_JsonConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_loc_Wire_JsonConfig(mux_container_Finished_loc_Wire_JsonConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_loc_Wire_JsonConfig &operator=(mux_container_Finished_loc_Wire_JsonConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_loc_Wire_TextConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_loc_Wire_TextConfig(mux_container_Finished_loc_Wire_TextConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_loc_Wire_TextConfig &operator=(mux_container_Finished_loc_Wire_TextConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_loc_Wire_VideoConfig {
		RUST2_FFI_UNCONSUMED mux_container_Finished_loc_Wire_VideoConfig(mux_container_Finished_loc_Wire_VideoConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_loc_Wire_VideoConfig &operator=(mux_container_Finished_loc_Wire_VideoConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Finished_loc_Wire {
		RUST2_FFI_UNCONSUMED mux_container_Finished_loc_Wire(mux_container_Finished_loc_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Finished_loc_Wire &operator=(mux_container_Finished_loc_Wire &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Guard_unit {
		RUST2_FFI_UNCONSUMED mux_container_Guard_unit(mux_container_Guard_unit &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Guard_unit &operator=(mux_container_Guard_unit &&other) noexcept = default;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_Error> commit() &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Guard_AudioConfig {
		RUST2_FFI_UNCONSUMED mux_container_Guard_AudioConfig(mux_container_Guard_AudioConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Guard_AudioConfig &operator=(mux_container_Guard_AudioConfig &&other) noexcept = default;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_Error> commit() &&;
		RUST2_FFI_CALLABLE ::std::optional<::moq::path_Relative> broadcast_get() const;
		RUST2_FFI_CALLABLE void broadcast_set(::std::optional<::moq::path_Relative> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> track_get() const;
		RUST2_FFI_CALLABLE void track_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> label_get() const;
		RUST2_FFI_CALLABLE void label_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_AudioCodec codec_get() const;
		RUST2_FFI_CALLABLE void codec_set(const ::moq::hang_catalog_AudioCodec &value);
		RUST2_FFI_CALLABLE ::std::uint32_t sample_rate_get() const;
		RUST2_FFI_CALLABLE void sample_rate_set(::std::uint32_t value);
		RUST2_FFI_CALLABLE ::std::uint32_t channel_count_get() const;
		RUST2_FFI_CALLABLE void channel_count_set(::std::uint32_t value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_AudioLayout> layout_get() const;
		RUST2_FFI_CALLABLE void layout_set(const ::std::optional<::moq::hang_catalog_AudioLayout> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bitrate_get() const;
		RUST2_FFI_CALLABLE void bitrate_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE bool enabled_get() const;
		RUST2_FFI_CALLABLE void enabled_set(bool value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> description_get() const;
		RUST2_FFI_CALLABLE void description_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Container container_get() const;
		RUST2_FFI_CALLABLE void container_set(::moq::hang_catalog_Container value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> jitter_get() const;
		RUST2_FFI_CALLABLE void jitter_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> warmup_get() const;
		RUST2_FFI_CALLABLE void warmup_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE bool same_identity(const ::moq::hang_catalog_AudioConfig &other) const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Guard_BinaryConfig {
		RUST2_FFI_UNCONSUMED mux_container_Guard_BinaryConfig(mux_container_Guard_BinaryConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Guard_BinaryConfig &operator=(mux_container_Guard_BinaryConfig &&other) noexcept = default;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_Error> commit() &&;
		RUST2_FFI_CALLABLE ::std::optional<::moq::path_Relative> broadcast_get() const;
		RUST2_FFI_CALLABLE void broadcast_set(::std::optional<::moq::path_Relative> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> track_get() const;
		RUST2_FFI_CALLABLE void track_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Mode mode_get() const;
		RUST2_FFI_CALLABLE void mode_set(const ::moq::hang_catalog_Mode &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> mime_get() const;
		RUST2_FFI_CALLABLE void mime_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bitrate_get() const;
		RUST2_FFI_CALLABLE void bitrate_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> jitter_get() const;
		RUST2_FFI_CALLABLE void jitter_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::Json>> extra_get() const;
		RUST2_FFI_CALLABLE void extra_set(::moq::taken<::std::tuple<::moq::view, ::moq::Json>> value);
	};
	class RUST2_FFI_CONSUMABLE mux_container_Guard_JsonConfig {
		RUST2_FFI_UNCONSUMED mux_container_Guard_JsonConfig(mux_container_Guard_JsonConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Guard_JsonConfig &operator=(mux_container_Guard_JsonConfig &&other) noexcept = default;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_Error> commit() &&;
		RUST2_FFI_CALLABLE ::std::optional<::moq::path_Relative> broadcast_get() const;
		RUST2_FFI_CALLABLE void broadcast_set(::std::optional<::moq::path_Relative> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> track_get() const;
		RUST2_FFI_CALLABLE void track_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Mode mode_get() const;
		RUST2_FFI_CALLABLE void mode_set(const ::moq::hang_catalog_Mode &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> schema_get() const;
		RUST2_FFI_CALLABLE void schema_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bitrate_get() const;
		RUST2_FFI_CALLABLE void bitrate_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> jitter_get() const;
		RUST2_FFI_CALLABLE void jitter_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::Json>> extra_get() const;
		RUST2_FFI_CALLABLE void extra_set(::moq::taken<::std::tuple<::moq::view, ::moq::Json>> value);
	};
	class RUST2_FFI_CONSUMABLE mux_container_Guard_TextConfig {
		RUST2_FFI_UNCONSUMED mux_container_Guard_TextConfig(mux_container_Guard_TextConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Guard_TextConfig &operator=(mux_container_Guard_TextConfig &&other) noexcept = default;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_Error> commit() &&;
		RUST2_FFI_CALLABLE ::std::optional<::moq::path_Relative> broadcast_get() const;
		RUST2_FFI_CALLABLE void broadcast_set(::std::optional<::moq::path_Relative> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> track_get() const;
		RUST2_FFI_CALLABLE void track_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_TextFormat format_get() const;
		RUST2_FFI_CALLABLE void format_set(const ::moq::hang_catalog_TextFormat &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_TextRole role_get() const;
		RUST2_FFI_CALLABLE void role_set(const ::moq::hang_catalog_TextRole &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> lang_get() const;
		RUST2_FFI_CALLABLE void lang_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> label_get() const;
		RUST2_FFI_CALLABLE void label_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Container container_get() const;
		RUST2_FFI_CALLABLE void container_set(::moq::hang_catalog_Container value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> jitter_get() const;
		RUST2_FFI_CALLABLE void jitter_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::std::optional<::moq::duration> &value);
	};
	class RUST2_FFI_CONSUMABLE mux_container_Guard_VideoConfig {
		RUST2_FFI_UNCONSUMED mux_container_Guard_VideoConfig(mux_container_Guard_VideoConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Guard_VideoConfig &operator=(mux_container_Guard_VideoConfig &&other) noexcept = default;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_Error> commit() &&;
		RUST2_FFI_CALLABLE ::std::optional<::moq::path_Relative> broadcast_get() const;
		RUST2_FFI_CALLABLE void broadcast_set(::std::optional<::moq::path_Relative> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> track_get() const;
		RUST2_FFI_CALLABLE void track_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> label_get() const;
		RUST2_FFI_CALLABLE void label_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_VideoCodec codec_get() const;
		RUST2_FFI_CALLABLE void codec_set(const ::moq::hang_catalog_VideoCodec &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> description_get() const;
		RUST2_FFI_CALLABLE void description_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> coded_width_get() const;
		RUST2_FFI_CALLABLE void coded_width_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> coded_height_get() const;
		RUST2_FFI_CALLABLE void coded_height_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> display_aspect_width_get() const;
		RUST2_FFI_CALLABLE void display_aspect_width_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> display_aspect_height_get() const;
		RUST2_FFI_CALLABLE void display_aspect_height_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Color> color_get() const;
		RUST2_FFI_CALLABLE void color_set(::std::optional<::moq::hang_catalog_Color> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Projection> projection_get() const;
		RUST2_FFI_CALLABLE void projection_set(::std::optional<::moq::hang_catalog_Projection> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Stereo> stereo_get() const;
		RUST2_FFI_CALLABLE void stereo_set(const ::std::optional<::moq::hang_catalog_Stereo> &value);
		RUST2_FFI_CALLABLE bool interlaced_get() const;
		RUST2_FFI_CALLABLE void interlaced_set(bool value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bitrate_get() const;
		RUST2_FFI_CALLABLE void bitrate_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE bool enabled_get() const;
		RUST2_FFI_CALLABLE void enabled_set(bool value);
		RUST2_FFI_CALLABLE ::std::int32_t preference_get() const;
		RUST2_FFI_CALLABLE void preference_set(::std::int32_t value);
		RUST2_FFI_CALLABLE ::std::optional<double> framerate_get() const;
		RUST2_FFI_CALLABLE void framerate_set(const ::std::optional<double> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint8_t> reorder_get() const;
		RUST2_FFI_CALLABLE void reorder_set(const ::std::optional<::std::uint8_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<bool> optimize_for_latency_get() const;
		RUST2_FFI_CALLABLE void optimize_for_latency_set(const ::std::optional<bool> &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Container container_get() const;
		RUST2_FFI_CALLABLE void container_set(::moq::hang_catalog_Container value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> jitter_get() const;
		RUST2_FFI_CALLABLE void jitter_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE bool same_identity(const ::moq::hang_catalog_VideoConfig &other) const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_Container_AudioConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_Container_AudioConfig(mux_container_Producer_Container_AudioConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_Container_AudioConfig &operator=(mux_container_Producer_Container_AudioConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_AudioConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_AudioConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_Container_AudioConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_Container_AudioConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_AudioConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_AudioConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_Container_AudioConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_Container_AudioConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_Container_BinaryConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_Container_BinaryConfig(mux_container_Producer_Container_BinaryConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_Container_BinaryConfig &operator=(mux_container_Producer_Container_BinaryConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_BinaryConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_BinaryConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_Container_BinaryConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_Container_BinaryConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_BinaryConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_BinaryConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_Container_BinaryConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_Container_BinaryConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_Container_JsonConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_Container_JsonConfig(mux_container_Producer_Container_JsonConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_Container_JsonConfig &operator=(mux_container_Producer_Container_JsonConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_JsonConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_JsonConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_Container_JsonConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_Container_JsonConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_JsonConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_JsonConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_Container_JsonConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_Container_JsonConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_Container_TextConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_Container_TextConfig(mux_container_Producer_Container_TextConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_Container_TextConfig &operator=(mux_container_Producer_Container_TextConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_TextConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_TextConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_Container_TextConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_Container_TextConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_TextConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_TextConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_Container_TextConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_Container_TextConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_Container_VideoConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_Container_VideoConfig(mux_container_Producer_Container_VideoConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_Container_VideoConfig &operator=(mux_container_Producer_Container_VideoConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_VideoConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_VideoConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_Container_VideoConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_Container_VideoConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_VideoConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_VideoConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_Container_VideoConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_Container_VideoConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_Container {
		RUST2_FFI_UNCONSUMED mux_container_Producer_Container(mux_container_Producer_Container &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_Container &operator=(mux_container_Producer_Container &&other) noexcept = default;
		static ::moq::mux_container_Producer_Container new_(::moq::track_Producer track, ::moq::mux_catalog_hang_Container container);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_unit, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_Container with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_Container with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_Container> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_Container_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_fmp4_Wire_AudioConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_fmp4_Wire_AudioConfig(mux_container_Producer_fmp4_Wire_AudioConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_fmp4_Wire_AudioConfig &operator=(mux_container_Producer_fmp4_Wire_AudioConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_AudioConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_AudioConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_fmp4_Wire_AudioConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_fmp4_Wire_AudioConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_fmp4_Wire_AudioConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_fmp4_Wire_AudioConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_fmp4_Wire_AudioConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_fmp4_Wire_AudioConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_fmp4_Wire_BinaryConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_fmp4_Wire_BinaryConfig(mux_container_Producer_fmp4_Wire_BinaryConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_fmp4_Wire_BinaryConfig &operator=(mux_container_Producer_fmp4_Wire_BinaryConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_BinaryConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_BinaryConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_fmp4_Wire_BinaryConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_fmp4_Wire_BinaryConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_fmp4_Wire_BinaryConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_fmp4_Wire_BinaryConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_fmp4_Wire_BinaryConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_fmp4_Wire_BinaryConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_fmp4_Wire_JsonConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_fmp4_Wire_JsonConfig(mux_container_Producer_fmp4_Wire_JsonConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_fmp4_Wire_JsonConfig &operator=(mux_container_Producer_fmp4_Wire_JsonConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_JsonConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_JsonConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_fmp4_Wire_JsonConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_fmp4_Wire_JsonConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_fmp4_Wire_JsonConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_fmp4_Wire_JsonConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_fmp4_Wire_JsonConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_fmp4_Wire_JsonConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_fmp4_Wire_TextConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_fmp4_Wire_TextConfig(mux_container_Producer_fmp4_Wire_TextConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_fmp4_Wire_TextConfig &operator=(mux_container_Producer_fmp4_Wire_TextConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_TextConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_TextConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_fmp4_Wire_TextConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_fmp4_Wire_TextConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_fmp4_Wire_TextConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_fmp4_Wire_TextConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_fmp4_Wire_TextConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_fmp4_Wire_TextConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_fmp4_Wire_VideoConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_fmp4_Wire_VideoConfig(mux_container_Producer_fmp4_Wire_VideoConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_fmp4_Wire_VideoConfig &operator=(mux_container_Producer_fmp4_Wire_VideoConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_VideoConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_VideoConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_fmp4_Wire_VideoConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_fmp4_Wire_VideoConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_fmp4_Wire_VideoConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_fmp4_Wire_VideoConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_fmp4_Wire_VideoConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_fmp4_Wire_VideoConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_fmp4_Wire {
		RUST2_FFI_UNCONSUMED mux_container_Producer_fmp4_Wire(mux_container_Producer_fmp4_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_fmp4_Wire &operator=(mux_container_Producer_fmp4_Wire &&other) noexcept = default;
		static ::moq::mux_container_Producer_fmp4_Wire new_(::moq::track_Producer track, ::moq::mux_container_fmp4_Wire container);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_unit, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_fmp4_Wire with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_fmp4_Wire with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_fmp4_Wire, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_fmp4_Wire, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_fmp4_Wire> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_fmp4_Wire_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_legacy_Wire_AudioConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_legacy_Wire_AudioConfig(mux_container_Producer_legacy_Wire_AudioConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_legacy_Wire_AudioConfig &operator=(mux_container_Producer_legacy_Wire_AudioConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_AudioConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_AudioConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_legacy_Wire_AudioConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_legacy_Wire_AudioConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_legacy_Wire_AudioConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_legacy_Wire_AudioConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_legacy_Wire_AudioConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_legacy_Wire_AudioConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_legacy_Wire_BinaryConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_legacy_Wire_BinaryConfig(mux_container_Producer_legacy_Wire_BinaryConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_legacy_Wire_BinaryConfig &operator=(mux_container_Producer_legacy_Wire_BinaryConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_BinaryConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_BinaryConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_legacy_Wire_BinaryConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_legacy_Wire_BinaryConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_legacy_Wire_BinaryConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_legacy_Wire_BinaryConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_legacy_Wire_BinaryConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_legacy_Wire_BinaryConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_legacy_Wire_JsonConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_legacy_Wire_JsonConfig(mux_container_Producer_legacy_Wire_JsonConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_legacy_Wire_JsonConfig &operator=(mux_container_Producer_legacy_Wire_JsonConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_JsonConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_JsonConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_legacy_Wire_JsonConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_legacy_Wire_JsonConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_legacy_Wire_JsonConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_legacy_Wire_JsonConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_legacy_Wire_JsonConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_legacy_Wire_JsonConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_legacy_Wire_TextConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_legacy_Wire_TextConfig(mux_container_Producer_legacy_Wire_TextConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_legacy_Wire_TextConfig &operator=(mux_container_Producer_legacy_Wire_TextConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_TextConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_TextConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_legacy_Wire_TextConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_legacy_Wire_TextConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_legacy_Wire_TextConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_legacy_Wire_TextConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_legacy_Wire_TextConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_legacy_Wire_TextConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_legacy_Wire_VideoConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_legacy_Wire_VideoConfig(mux_container_Producer_legacy_Wire_VideoConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_legacy_Wire_VideoConfig &operator=(mux_container_Producer_legacy_Wire_VideoConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_VideoConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_VideoConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_legacy_Wire_VideoConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_legacy_Wire_VideoConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_legacy_Wire_VideoConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_legacy_Wire_VideoConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_legacy_Wire_VideoConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_legacy_Wire_VideoConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_legacy_Wire {
		RUST2_FFI_UNCONSUMED mux_container_Producer_legacy_Wire(mux_container_Producer_legacy_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_legacy_Wire &operator=(mux_container_Producer_legacy_Wire &&other) noexcept = default;
		static ::moq::mux_container_Producer_legacy_Wire new_(::moq::track_Producer track, const ::moq::mux_container_legacy_Wire &container);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_unit, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_legacy_Wire with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_legacy_Wire with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_legacy_Wire, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_legacy_Wire, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_legacy_Wire> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_legacy_Wire_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_loc_Wire_AudioConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_loc_Wire_AudioConfig(mux_container_Producer_loc_Wire_AudioConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_loc_Wire_AudioConfig &operator=(mux_container_Producer_loc_Wire_AudioConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_AudioConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_AudioConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_loc_Wire_AudioConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_loc_Wire_AudioConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_loc_Wire_AudioConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_loc_Wire_AudioConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_loc_Wire_AudioConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_loc_Wire_AudioConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_loc_Wire_BinaryConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_loc_Wire_BinaryConfig(mux_container_Producer_loc_Wire_BinaryConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_loc_Wire_BinaryConfig &operator=(mux_container_Producer_loc_Wire_BinaryConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_BinaryConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_BinaryConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_loc_Wire_BinaryConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_loc_Wire_BinaryConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_loc_Wire_BinaryConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_loc_Wire_BinaryConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_loc_Wire_BinaryConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_loc_Wire_BinaryConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_loc_Wire_JsonConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_loc_Wire_JsonConfig(mux_container_Producer_loc_Wire_JsonConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_loc_Wire_JsonConfig &operator=(mux_container_Producer_loc_Wire_JsonConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_JsonConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_JsonConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_loc_Wire_JsonConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_loc_Wire_JsonConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_loc_Wire_JsonConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_loc_Wire_JsonConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_loc_Wire_JsonConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_loc_Wire_JsonConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_loc_Wire_TextConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_loc_Wire_TextConfig(mux_container_Producer_loc_Wire_TextConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_loc_Wire_TextConfig &operator=(mux_container_Producer_loc_Wire_TextConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_TextConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_TextConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_loc_Wire_TextConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_loc_Wire_TextConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_loc_Wire_TextConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_loc_Wire_TextConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_loc_Wire_TextConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_loc_Wire_TextConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_loc_Wire_VideoConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_loc_Wire_VideoConfig(mux_container_Producer_loc_Wire_VideoConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_loc_Wire_VideoConfig &operator=(mux_container_Producer_loc_Wire_VideoConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_VideoConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_VideoConfig, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_loc_Wire_VideoConfig with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_loc_Wire_VideoConfig with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_loc_Wire_VideoConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_loc_Wire_VideoConfig, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_loc_Wire_VideoConfig> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_loc_Wire_VideoConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_loc_Wire {
		RUST2_FFI_UNCONSUMED mux_container_Producer_loc_Wire(mux_container_Producer_loc_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_loc_Wire &operator=(mux_container_Producer_loc_Wire &&other) noexcept = default;
		static ::moq::mux_container_Producer_loc_Wire new_(::moq::track_Producer track, const ::moq::mux_container_loc_Wire &container);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::mux_container_Guard_unit, ::moq::mux_Error> modify();
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_Error> timestamp(const ::std::optional<::moq::Timestamp> &hint) const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE void history(const ::moq::Timestamp &until);
		RUST2_FFI_CALLABLE bool needs_keyframe() const;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_loc_Wire with_buffer(::moq::duration duration_) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Producer_loc_Wire with_bandwidth(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> live_edge() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> write(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_loc_Wire, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_loc_Wire, ::moq::mux_Error> fail(::moq::Error err) &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_container_Producer_loc_Wire> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_loc_Wire_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_follow_Instances {
		RUST2_FFI_UNCONSUMED mux_follow_Instances(mux_follow_Instances &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_follow_Instances &operator=(mux_follow_Instances &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::mux_follow_Instances_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_import_Container {
		RUST2_FFI_UNCONSUMED mux_import_Container(mux_import_Container &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_Container &operator=(mux_import_Container &&other) noexcept = default;
		static ::moq::result<::moq::mux_import_Container, ::moq::mux_Error> new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved reserved, const ::moq::mux_import_ContainerInit &init);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_import_container_Finished, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
	};
	class RUST2_FFI_CONSUMABLE mux_import_Container_Extra {
		RUST2_FFI_UNCONSUMED mux_import_Container_Extra(mux_import_Container_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_Container_Extra &operator=(mux_import_Container_Extra &&other) noexcept = default;
		static ::moq::result<::moq::mux_import_Container_Extra, ::moq::mux_Error> new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Extra reserved, const ::moq::mux_import_ContainerInit &init);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_import_container_Finished_Extra, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
	};
	class RUST2_FFI_CONSUMABLE mux_import_Container_Ext {
		RUST2_FFI_UNCONSUMED mux_import_Container_Ext(mux_import_Container_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_Container_Ext &operator=(mux_import_Container_Ext &&other) noexcept = default;
		static ::moq::result<::moq::mux_import_Container_Ext, ::moq::mux_Error> new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Ext reserved, const ::moq::mux_import_ContainerInit &init);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_import_container_Finished_Ext, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
	};
	class RUST2_FFI_CONSUMABLE mux_import_AudioInit {
		RUST2_FFI_UNCONSUMED mux_import_AudioInit(mux_import_AudioInit &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_AudioInit &operator=(mux_import_AudioInit &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_import_AudioInit clone() const;
		friend bool operator==(const mux_import_AudioInit &a, const mux_import_AudioInit &b);
		friend bool operator!=(const mux_import_AudioInit &a, const mux_import_AudioInit &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::mux_import_AudioFormat format_get() const;
		RUST2_FFI_CALLABLE void format_set(::moq::mux_import_AudioFormat value);
		RUST2_FFI_CALLABLE ::moq::buf data_get() const;
		RUST2_FFI_CALLABLE void data_set(::moq::input value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> label_get() const;
		RUST2_FFI_CALLABLE void label_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Container container_get() const;
		RUST2_FFI_CALLABLE void container_set(::moq::hang_catalog_Container value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bitrate_get() const;
		RUST2_FFI_CALLABLE void bitrate_set(const ::std::optional<::std::uint64_t> &value);
		static ::moq::mux_import_AudioInit new_(::moq::mux_import_AudioFormat format, ::moq::input data);
	};
	class RUST2_FFI_CONSUMABLE mux_import_ContainerInit {
		RUST2_FFI_UNCONSUMED mux_import_ContainerInit(mux_import_ContainerInit &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_ContainerInit &operator=(mux_import_ContainerInit &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_import_ContainerInit clone() const;
		friend bool operator==(const mux_import_ContainerInit &a, const mux_import_ContainerInit &b);
		friend bool operator!=(const mux_import_ContainerInit &a, const mux_import_ContainerInit &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::mux_import_ContainerFormat format_get() const;
		RUST2_FFI_CALLABLE void format_set(::moq::mux_import_ContainerFormat value);
		RUST2_FFI_CALLABLE ::moq::buf data_get() const;
		RUST2_FFI_CALLABLE void data_set(::moq::input value);
		static ::moq::mux_import_ContainerInit new_(::moq::mux_import_ContainerFormat format, ::moq::input data);
	};
	class RUST2_FFI_CONSUMABLE mux_import_VideoInit {
		RUST2_FFI_UNCONSUMED mux_import_VideoInit(mux_import_VideoInit &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_VideoInit &operator=(mux_import_VideoInit &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_import_VideoInit clone() const;
		friend bool operator==(const mux_import_VideoInit &a, const mux_import_VideoInit &b);
		friend bool operator!=(const mux_import_VideoInit &a, const mux_import_VideoInit &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::mux_import_VideoFormat format_get() const;
		RUST2_FFI_CALLABLE void format_set(::moq::mux_import_VideoFormat value);
		RUST2_FFI_CALLABLE ::moq::buf data_get() const;
		RUST2_FFI_CALLABLE void data_set(::moq::input value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> label_get() const;
		RUST2_FFI_CALLABLE void label_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_VideoHint hint_get() const;
		RUST2_FFI_CALLABLE void hint_set(::moq::mux_catalog_VideoHint value);
		RUST2_FFI_CALLABLE ::std::int8_t priority_get() const;
		RUST2_FFI_CALLABLE void priority_set(::std::int8_t value);
		static ::moq::mux_import_VideoInit new_(::moq::mux_import_VideoFormat format, ::moq::input data);
	};
	class RUST2_FFI_CONSUMABLE mux_import_Track {
		RUST2_FFI_UNCONSUMED mux_import_Track(mux_import_Track &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_Track &operator=(mux_import_Track &&other) noexcept = default;
		static ::moq::result<::moq::mux_import_Track, ::moq::mux_Error> audio_Extra(::moq::track_Request request, ::moq::mux_catalog_Reserved_Extra reserved, ::moq::mux_import_AudioInit init);
		static ::moq::result<::moq::mux_import_Track, ::moq::mux_Error> audio_Ext(::moq::track_Request request, ::moq::mux_catalog_Reserved_Ext reserved, ::moq::mux_import_AudioInit init);
		static ::moq::result<::moq::mux_import_Track, ::moq::mux_Error> audio_unit(::moq::track_Request request, ::moq::mux_catalog_Reserved reserved, ::moq::mux_import_AudioInit init);
		static ::moq::result<::moq::mux_import_Track, ::moq::mux_Error> video_Extra(::moq::track_Request request, ::moq::mux_catalog_Reserved_Extra reserved, ::moq::mux_import_VideoInit init);
		static ::moq::result<::moq::mux_import_Track, ::moq::mux_Error> video_Ext(::moq::track_Request request, ::moq::mux_catalog_Reserved_Ext reserved, ::moq::mux_import_VideoInit init);
		static ::moq::result<::moq::mux_import_Track, ::moq::mux_Error> video_unit(::moq::track_Request request, ::moq::mux_catalog_Reserved reserved, ::moq::mux_import_VideoInit init);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input frame, const ::std::optional<::moq::Timestamp> &pts);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_import_track_Finished, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_import_Track> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_enabled(bool enabled);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
	};
	class RUST2_FFI_CONSUMABLE mux_import_TrackStream {
		RUST2_FFI_UNCONSUMED mux_import_TrackStream(mux_import_TrackStream &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_TrackStream &operator=(mux_import_TrackStream &&other) noexcept = default;
		static ::moq::result<::moq::mux_import_TrackStream, ::moq::mux_Error> video_Extra(::moq::track_Request request, ::moq::mux_catalog_Reserved_Extra reserved, ::moq::mux_import_VideoInit init);
		static ::moq::result<::moq::mux_import_TrackStream, ::moq::mux_Error> video_Ext(::moq::track_Request request, ::moq::mux_catalog_Reserved_Ext reserved, ::moq::mux_import_VideoInit init);
		static ::moq::result<::moq::mux_import_TrackStream, ::moq::mux_Error> video_unit(::moq::track_Request request, ::moq::mux_catalog_Reserved reserved, ::moq::mux_import_VideoInit init);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> initialize(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_import_track_Finished, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_enabled(bool enabled);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_import_TrackStream> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
	};
	class RUST2_FFI_CONSUMABLE mux_json_Snapshot_Json {
		RUST2_FFI_UNCONSUMED mux_json_Snapshot_Json(mux_json_Snapshot_Json &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_json_Snapshot_Json &operator=(mux_json_Snapshot_Json &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::mux_json_Snapshot_Json_refresh_future refresh() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> update(const ::moq::Json &value, const ::moq::Timestamp &timestamp);
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_Error> abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_json_Snapshot_Json> abort_unwanted(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_json_Stream_Json {
		RUST2_FFI_UNCONSUMED mux_json_Stream_Json(mux_json_Stream_Json &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_json_Stream_Json &operator=(mux_json_Stream_Json &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> append(const ::moq::Json &value, const ::moq::Timestamp &timestamp);
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_Error> abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_json_Stream_Json> abort_unwanted(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_json_Consumer_Json {
		RUST2_FFI_UNCONSUMED mux_json_Consumer_Json(mux_json_Consumer_Json &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_json_Consumer_Json &operator=(mux_json_Consumer_Json &&other) noexcept = default;
		static ::moq::mux_json_Consumer_Json_from_track_future from_track(::moq::track_Consumer track, const ::moq::hang_catalog_JsonConfig &config);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Mode mode() const;
		RUST2_FFI_CALLABLE ::moq::mux_json_Consumer_Json_next_future next();
		RUST2_FFI_CALLABLE ::moq::mux_json_Consumer_Json_latest_future latest();
	};
	class RUST2_FFI_CONSUMABLE mux_json_Config {
		RUST2_FFI_UNCONSUMED mux_json_Config(mux_json_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_json_Config &operator=(mux_json_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_json_Config clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_json_Config default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> schema_get() const;
		RUST2_FFI_CALLABLE void schema_set(::std::optional<::moq::input> value);
		RUST2_FFI_CONSUMES ::moq::mux_json_Config with_schema(::moq::input schema) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_json_Options {
		RUST2_FFI_UNCONSUMED mux_json_Options(mux_json_Options &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_json_Options &operator=(mux_json_Options &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_json_Options clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_json_Options default_();
		RUST2_FFI_CALLABLE bool deltas_get() const;
		RUST2_FFI_CALLABLE void deltas_set(bool value);
		RUST2_FFI_CONSUMES ::moq::mux_json_Options with_deltas(bool deltas) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_rate_Grant {
		RUST2_FFI_UNCONSUMED mux_rate_Grant(mux_rate_Grant &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_rate_Grant &operator=(mux_rate_Grant &&other) noexcept = default;
		static ::moq::mux_rate_Grant new_(::moq::bandwidth_Consumer grant, ::moq::mux_rate_Policy policy);
		RUST2_FFI_CALLABLE void assume(const ::moq::mux_rate_Target &target);
		RUST2_FFI_CALLABLE ::moq::mux_rate_Policy policy() const;
		RUST2_FFI_CALLABLE void set_policy(::moq::mux_rate_Policy policy);
		RUST2_FFI_CALLABLE ::moq::mux_rate_Grant_next_future next();
		RUST2_FFI_CALLABLE void failed();
	};
	class RUST2_FFI_CONSUMABLE mux_rate_Policy {
		RUST2_FFI_UNCONSUMED mux_rate_Policy(mux_rate_Policy &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_rate_Policy &operator=(mux_rate_Policy &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_rate_Policy clone() const;
		friend bool operator==(const mux_rate_Policy &a, const mux_rate_Policy &b);
		friend bool operator!=(const mux_rate_Policy &a, const mux_rate_Policy &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Rate max_get() const;
		RUST2_FFI_CALLABLE void max_set(const ::moq::bandwidth_Rate &value);
		RUST2_FFI_CALLABLE ::moq::bandwidth_Rate min_get() const;
		RUST2_FFI_CALLABLE void min_set(const ::moq::bandwidth_Rate &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::bandwidth_Rate> lower_get() const;
		RUST2_FFI_CALLABLE void lower_set(const ::std::optional<::moq::bandwidth_Rate> &value);
		static ::moq::mux_rate_Policy new_(const ::moq::bandwidth_Rate &max);
	};
	class RUST2_FFI_CONSUMABLE mux_rate_Control {
		RUST2_FFI_UNCONSUMED mux_rate_Control(mux_rate_Control &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_rate_Control &operator=(mux_rate_Control &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_rate_Control clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::mux_rate_Control new_(::moq::mux_rate_Policy policy);
		RUST2_FFI_CALLABLE ::moq::mux_rate_Target target() const;
		RUST2_FFI_CALLABLE void set_probed(bool probed);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mux_rate_Target> update(const ::std::optional<::moq::bandwidth_Rate> &estimate, const ::moq::clock_Instant &now);
	};
	class RUST2_FFI_CONSUMABLE mux_select_Broadcast {
		RUST2_FFI_UNCONSUMED mux_select_Broadcast(mux_select_Broadcast &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_select_Broadcast &operator=(mux_select_Broadcast &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_select_Broadcast(const mux_select_Broadcast &other) = default;
		mux_select_Broadcast &operator=(const mux_select_Broadcast &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_select_Broadcast default_();
		RUST2_FFI_CONSUMES ::moq::mux_select_Broadcast video(::moq::mux_select_Video video) &&;
		RUST2_FFI_CONSUMES ::moq::mux_select_Broadcast audio(::moq::mux_select_Audio audio) &&;
		RUST2_FFI_CALLABLE bool has_video() const;
		RUST2_FFI_CALLABLE bool has_audio() const;
		RUST2_FFI_CALLABLE void retain_Extra(::moq::hang_Catalog_Extra &catalog) const;
		RUST2_FFI_CALLABLE void retain_Ext(::moq::hang_Catalog_Ext &catalog) const;
		RUST2_FFI_CALLABLE void retain_unit(::moq::hang_Catalog &catalog) const;
	};
	class RUST2_FFI_CONSUMABLE mux_select_Video {
		RUST2_FFI_UNCONSUMED mux_select_Video(mux_select_Video &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_select_Video &operator=(mux_select_Video &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_select_Video(const mux_select_Video &other) = default;
		mux_select_Video &operator=(const mux_select_Video &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_select_Video default_();
		RUST2_FFI_CONSUMES ::moq::mux_select_Video name(::moq::input name) &&;
		RUST2_FFI_CONSUMES ::moq::mux_select_Video codec(::moq::hang_catalog_VideoCodecKind codec) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_select_Audio {
		RUST2_FFI_UNCONSUMED mux_select_Audio(mux_select_Audio &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_select_Audio &operator=(mux_select_Audio &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_select_Audio(const mux_select_Audio &other) = default;
		mux_select_Audio &operator=(const mux_select_Audio &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_select_Audio default_();
		RUST2_FFI_CONSUMES ::moq::mux_select_Audio name(::moq::input name) &&;
		RUST2_FFI_CONSUMES ::moq::mux_select_Audio codec(::moq::hang_catalog_AudioCodecKind codec) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_sync_Track {
		RUST2_FFI_UNCONSUMED mux_sync_Track(mux_sync_Track &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_sync_Track &operator=(mux_sync_Track &&other) noexcept = default;
		RUST2_FFI_CALLABLE bool received(const ::moq::Timestamp &timestamp, const ::moq::clock_Instant &now) const;
		RUST2_FFI_CALLABLE void reset() const;
		RUST2_FFI_CALLABLE bool set_target(::moq::duration target) const;
		RUST2_FFI_CALLABLE ::moq::mux_sync_Track_delay_changed_future delay_changed(::moq::duration seen) const;
	};
	class RUST2_FFI_CONSUMABLE mux_sync_Schedule {
		RUST2_FFI_UNCONSUMED mux_sync_Schedule(mux_sync_Schedule &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_sync_Schedule &operator=(mux_sync_Schedule &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_sync_Schedule(const mux_sync_Schedule &other) = default;
		mux_sync_Schedule &operator=(const mux_sync_Schedule &other) = default;
		friend bool operator==(const mux_sync_Schedule &a, const mux_sync_Schedule &b);
		friend bool operator!=(const mux_sync_Schedule &a, const mux_sync_Schedule &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::duration delay() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> due(const ::moq::Timestamp &timestamp) const;
	};
	class RUST2_FFI_CONSUMABLE mux_sync_Producer {
		RUST2_FFI_UNCONSUMED mux_sync_Producer(mux_sync_Producer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_sync_Producer &operator=(mux_sync_Producer &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_sync_Producer(const mux_sync_Producer &other) = default;
		mux_sync_Producer &operator=(const mux_sync_Producer &other) = default;
		static ::moq::mux_sync_Producer new_(::moq::duration delay);
		RUST2_FFI_CONSUMES ::moq::mux_sync_Producer reset(::moq::duration delay) &&;
		RUST2_FFI_CALLABLE ::moq::mux_sync_Consumer consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_sync_Track track(::moq::mux_container_Kind kind) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> due(const ::moq::Timestamp &timestamp) const;
	};
	class RUST2_FFI_CONSUMABLE mux_sync_Consumer {
		RUST2_FFI_UNCONSUMED mux_sync_Consumer(mux_sync_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_sync_Consumer &operator=(mux_sync_Consumer &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_sync_Consumer(const mux_sync_Consumer &other) = default;
		mux_sync_Consumer &operator=(const mux_sync_Consumer &other) = default;
		RUST2_FFI_CALLABLE ::moq::duration delay() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> due(const ::moq::Timestamp &timestamp) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> position(const ::moq::clock_Instant &at) const;
		RUST2_FFI_CALLABLE ::moq::mux_sync_Schedule schedule() const;
		RUST2_FFI_CALLABLE ::moq::mux_sync_Consumer_changed_future changed(const ::moq::mux_sync_Schedule &seen) const;
		RUST2_FFI_CALLABLE ::std::optional<::std::size_t> present(const ::std::vector<::moq::Timestamp> &timestamps, const ::moq::clock_Instant &now) const;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_hang_Consumer {
		RUST2_FFI_UNCONSUMED mux_catalog_hang_Consumer(mux_catalog_hang_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_hang_Consumer &operator=(mux_catalog_hang_Consumer &&other) noexcept = default;
		static ::moq::mux_catalog_hang_Consumer new_(::moq::track_Consumer track);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_hang_Consumer_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_hang_Consumer_Extra {
		RUST2_FFI_UNCONSUMED mux_catalog_hang_Consumer_Extra(mux_catalog_hang_Consumer_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_hang_Consumer_Extra &operator=(mux_catalog_hang_Consumer_Extra &&other) noexcept = default;
		static ::moq::mux_catalog_hang_Consumer_Extra new_(::moq::track_Consumer track);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_hang_Consumer_Extra_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_hang_Consumer_Ext {
		RUST2_FFI_UNCONSUMED mux_catalog_hang_Consumer_Ext(mux_catalog_hang_Consumer_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_hang_Consumer_Ext &operator=(mux_catalog_hang_Consumer_Ext &&other) noexcept = default;
		static ::moq::mux_catalog_hang_Consumer_Ext new_(::moq::track_Consumer track);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_hang_Consumer_Ext_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_hang_Container {
		RUST2_FFI_UNCONSUMED mux_catalog_hang_Container(mux_catalog_hang_Container &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_hang_Container &operator=(mux_catalog_hang_Container &&other) noexcept = default;
		static ::moq::result<::moq::mux_catalog_hang_Container, ::moq::mux_Error> new_(const ::moq::hang_catalog_Container &container, ::moq::mux_container_Kind kind);
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_hang_Extra {
		RUST2_FFI_UNCONSUMED mux_catalog_hang_Extra(mux_catalog_hang_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_hang_Extra &operator=(mux_catalog_hang_Extra &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_catalog_hang_Extra clone() const;
		friend bool operator==(const mux_catalog_hang_Extra &a, const mux_catalog_hang_Extra &b);
		friend bool operator!=(const mux_catalog_hang_Extra &a, const mux_catalog_hang_Extra &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_catalog_hang_Extra default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::Json> get(::moq::input name) const;
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::Json>> iter() const;
		RUST2_FFI_CALLABLE ::std::size_t len() const;
		RUST2_FFI_CALLABLE bool is_empty() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::input name, ::moq::Json value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Json> remove(::moq::input name);
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_msf_Consumer {
		RUST2_FFI_UNCONSUMED mux_catalog_msf_Consumer(mux_catalog_msf_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_msf_Consumer &operator=(mux_catalog_msf_Consumer &&other) noexcept = default;
		static ::moq::mux_catalog_msf_Consumer new_(::moq::track_Subscriber track);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_msf_Consumer_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_msf_Consumer_Extra {
		RUST2_FFI_UNCONSUMED mux_catalog_msf_Consumer_Extra(mux_catalog_msf_Consumer_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_msf_Consumer_Extra &operator=(mux_catalog_msf_Consumer_Extra &&other) noexcept = default;
		static ::moq::mux_catalog_msf_Consumer_Extra new_(::moq::track_Subscriber track);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_msf_Consumer_Extra_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_msf_Consumer_Ext {
		RUST2_FFI_UNCONSUMED mux_catalog_msf_Consumer_Ext(mux_catalog_msf_Consumer_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_msf_Consumer_Ext &operator=(mux_catalog_msf_Consumer_Ext &&other) noexcept = default;
		static ::moq::mux_catalog_msf_Consumer_Ext new_(::moq::track_Subscriber track);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_msf_Consumer_Ext_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_codec_aac_Config {
		RUST2_FFI_UNCONSUMED mux_codec_aac_Config(mux_codec_aac_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_aac_Config &operator=(mux_codec_aac_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::std::uint8_t profile_get() const;
		RUST2_FFI_CALLABLE void profile_set(::std::uint8_t value);
		RUST2_FFI_CALLABLE ::std::uint32_t sample_rate_get() const;
		RUST2_FFI_CALLABLE void sample_rate_set(::std::uint32_t value);
		RUST2_FFI_CALLABLE ::std::uint32_t channel_count_get() const;
		RUST2_FFI_CALLABLE void channel_count_set(::std::uint32_t value);
		static ::moq::mux_codec_aac_Config new_(::std::uint8_t profile, ::std::uint32_t sample_rate, ::std::uint32_t channel_count);
		static ::moq::result<::moq::mux_codec_aac_Config, ::moq::mux_codec_aac_Error> parse(::moq::buf &buf_);
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::mux_codec_aac_Error> encode() const;
	};
	class RUST2_FFI_CONSUMABLE mux_codec_aac_Error {
		RUST2_FFI_UNCONSUMED mux_codec_aac_Error(mux_codec_aac_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_aac_Error &operator=(mux_codec_aac_Error &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_codec_aac_Error(const mux_codec_aac_Error &other) = default;
		mux_codec_aac_Error &operator=(const mux_codec_aac_Error &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE mux_codec_aac_Import {
		RUST2_FFI_UNCONSUMED mux_codec_aac_Import(mux_codec_aac_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_aac_Import &operator=(mux_codec_aac_Import &&other) noexcept = default;
		static ::moq::result<::moq::mux_codec_aac_Import, ::moq::mux_Error> new_Extra(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Extra reserved, ::moq::hang_catalog_AudioConfig config);
		static ::moq::result<::moq::mux_codec_aac_Import, ::moq::mux_Error> new_Ext(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Ext reserved, ::moq::hang_catalog_AudioConfig config);
		static ::moq::result<::moq::mux_codec_aac_Import, ::moq::mux_Error> new_unit(::moq::track_Producer track, ::moq::mux_catalog_Reserved reserved, ::moq::hang_catalog_AudioConfig config);
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_AudioConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_codec_aac_Import> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input frame, const ::std::optional<::moq::Timestamp> &pts);
	};
	class RUST2_FFI_CONSUMABLE mux_codec_annexb_Error {
		RUST2_FFI_UNCONSUMED mux_codec_annexb_Error(mux_codec_annexb_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_annexb_Error &operator=(mux_codec_annexb_Error &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_codec_annexb_Error(const mux_codec_annexb_Error &other) = default;
		mux_codec_annexb_Error &operator=(const mux_codec_annexb_Error &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE mux_codec_av1_Import {
		RUST2_FFI_UNCONSUMED mux_codec_av1_Import(mux_codec_av1_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_av1_Import &operator=(mux_codec_av1_Import &&other) noexcept = default;
		static ::moq::result<::moq::mux_codec_av1_Import, ::moq::mux_Error> new_Extra(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Extra reserved, ::moq::mux_catalog_VideoHint hint);
		static ::moq::result<::moq::mux_codec_av1_Import, ::moq::mux_Error> new_Ext(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Ext reserved, ::moq::mux_catalog_VideoHint hint);
		static ::moq::result<::moq::mux_codec_av1_Import, ::moq::mux_Error> new_unit(::moq::track_Producer track, ::moq::mux_catalog_Reserved reserved, ::moq::mux_catalog_VideoHint hint);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> initialize(::moq::input buf_);
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_VideoConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_codec_av1_Import> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_enabled(bool enabled);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(const ::std::vector<::moq::mux_container_Frame> &frames);
	};
	class RUST2_FFI_CONSUMABLE mux_codec_av1_Split {
		RUST2_FFI_UNCONSUMED mux_codec_av1_Split(mux_codec_av1_Split &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_av1_Split &operator=(mux_codec_av1_Split &&other) noexcept = default;
		static mux_codec_av1_Split default_();
		static ::moq::mux_codec_av1_Split new_();
		RUST2_FFI_CALLABLE ::moq::result<::std::vector<::moq::mux_container_Frame>, ::moq::mux_Error> decode(::moq::input data, const ::std::optional<::moq::Timestamp> &pts);
		RUST2_FFI_CALLABLE ::moq::result<::std::vector<::moq::mux_container_Frame>, ::moq::mux_Error> flush(const ::std::optional<::moq::Timestamp> &pts);
		RUST2_FFI_CALLABLE void reset();
	};
	class RUST2_FFI_CONSUMABLE mux_codec_flac_Import {
		RUST2_FFI_UNCONSUMED mux_codec_flac_Import(mux_codec_flac_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_flac_Import &operator=(mux_codec_flac_Import &&other) noexcept = default;
		static ::moq::result<::moq::mux_codec_flac_Import, ::moq::mux_Error> new_Extra(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Extra reserved, ::moq::hang_catalog_AudioConfig config);
		static ::moq::result<::moq::mux_codec_flac_Import, ::moq::mux_Error> new_Ext(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Ext reserved, ::moq::hang_catalog_AudioConfig config);
		static ::moq::result<::moq::mux_codec_flac_Import, ::moq::mux_Error> new_unit(::moq::track_Producer track, ::moq::mux_catalog_Reserved reserved, ::moq::hang_catalog_AudioConfig config);
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_AudioConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_codec_flac_Import> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input frame, const ::std::optional<::moq::Timestamp> &pts);
	};
	class RUST2_FFI_CONSUMABLE mux_codec_h264_Avc1 {
		RUST2_FFI_UNCONSUMED mux_codec_h264_Avc1(mux_codec_h264_Avc1 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_h264_Avc1 &operator=(mux_codec_h264_Avc1 &&other) noexcept = default;
		static mux_codec_h264_Avc1 default_();
		static ::moq::mux_codec_h264_Avc1 new_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> avcc() const;
		RUST2_FFI_CALLABLE ::moq::result<::std::optional<::moq::buf>, ::moq::mux_codec_h264_Error> transform(::moq::input payload);
	};
	class RUST2_FFI_CONSUMABLE mux_codec_h264_Error {
		RUST2_FFI_UNCONSUMED mux_codec_h264_Error(mux_codec_h264_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_h264_Error &operator=(mux_codec_h264_Error &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_codec_h264_Error(const mux_codec_h264_Error &other) = default;
		mux_codec_h264_Error &operator=(const mux_codec_h264_Error &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE mux_codec_h264_Avcc {
		RUST2_FFI_UNCONSUMED mux_codec_h264_Avcc(mux_codec_h264_Avcc &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_h264_Avcc &operator=(mux_codec_h264_Avcc &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_codec_h264_Avcc clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::uint8_t profile_get() const;
		RUST2_FFI_CALLABLE void profile_set(::std::uint8_t value);
		RUST2_FFI_CALLABLE ::std::uint8_t constraints_get() const;
		RUST2_FFI_CALLABLE void constraints_set(::std::uint8_t value);
		RUST2_FFI_CALLABLE ::std::uint8_t level_get() const;
		RUST2_FFI_CALLABLE void level_set(::std::uint8_t value);
		RUST2_FFI_CALLABLE ::std::size_t length_size_get() const;
		RUST2_FFI_CALLABLE void length_size_set(::std::size_t value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> sps_get() const;
		RUST2_FFI_CALLABLE void sps_set(const ::moq::items<::moq::view> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> pps_get() const;
		RUST2_FFI_CALLABLE void pps_set(const ::moq::items<::moq::view> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> coded_width_get() const;
		RUST2_FFI_CALLABLE void coded_width_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> coded_height_get() const;
		RUST2_FFI_CALLABLE void coded_height_set(const ::std::optional<::std::uint32_t> &value);
		static ::moq::result<::moq::mux_codec_h264_Avcc, ::moq::mux_codec_h264_Error> parse(::moq::input avcc);
	};
	class RUST2_FFI_CONSUMABLE mux_codec_h264_Import {
		RUST2_FFI_UNCONSUMED mux_codec_h264_Import(mux_codec_h264_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_h264_Import &operator=(mux_codec_h264_Import &&other) noexcept = default;
		static ::moq::result<::moq::mux_codec_h264_Import, ::moq::mux_Error> new_Extra(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Extra reserved, ::moq::mux_catalog_VideoHint hint);
		static ::moq::result<::moq::mux_codec_h264_Import, ::moq::mux_Error> new_Ext(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Ext reserved, ::moq::mux_catalog_VideoHint hint);
		static ::moq::result<::moq::mux_codec_h264_Import, ::moq::mux_Error> new_unit(::moq::track_Producer track, ::moq::mux_catalog_Reserved reserved, ::moq::mux_catalog_VideoHint hint);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> initialize(::moq::input buf_);
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_VideoConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_codec_h264_Import> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_enabled(bool enabled);
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> observe_reorder(const ::moq::Timestamp &reorder);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(const ::std::vector<::moq::mux_container_Frame> &frames);
	};
	class RUST2_FFI_CONSUMABLE mux_codec_h264_Split {
		RUST2_FFI_UNCONSUMED mux_codec_h264_Split(mux_codec_h264_Split &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_h264_Split &operator=(mux_codec_h264_Split &&other) noexcept = default;
		static mux_codec_h264_Split default_();
		static ::moq::mux_codec_h264_Split new_();
		RUST2_FFI_CALLABLE ::moq::result<::std::vector<::moq::mux_container_Frame>, ::moq::mux_Error> decode(::moq::input data, const ::std::optional<::moq::Timestamp> &pts);
		RUST2_FFI_CALLABLE ::moq::result<::std::vector<::moq::mux_container_Frame>, ::moq::mux_Error> flush(const ::std::optional<::moq::Timestamp> &pts);
		RUST2_FFI_CALLABLE void reset();
	};
	class RUST2_FFI_CONSUMABLE mux_codec_h265_Hvc1 {
		RUST2_FFI_UNCONSUMED mux_codec_h265_Hvc1(mux_codec_h265_Hvc1 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_h265_Hvc1 &operator=(mux_codec_h265_Hvc1 &&other) noexcept = default;
		static mux_codec_h265_Hvc1 default_();
		static ::moq::mux_codec_h265_Hvc1 new_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> hvcc() const;
		RUST2_FFI_CALLABLE ::moq::result<::std::optional<::moq::buf>, ::moq::mux_codec_h265_Error> transform(::moq::input payload);
	};
	class RUST2_FFI_CONSUMABLE mux_codec_h265_Error {
		RUST2_FFI_UNCONSUMED mux_codec_h265_Error(mux_codec_h265_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_h265_Error &operator=(mux_codec_h265_Error &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_codec_h265_Error(const mux_codec_h265_Error &other) = default;
		mux_codec_h265_Error &operator=(const mux_codec_h265_Error &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE mux_codec_h265_Hvcc {
		RUST2_FFI_UNCONSUMED mux_codec_h265_Hvcc(mux_codec_h265_Hvcc &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_h265_Hvcc &operator=(mux_codec_h265_Hvcc &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_codec_h265_Hvcc clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::size_t length_size_get() const;
		RUST2_FFI_CALLABLE void length_size_set(::std::size_t value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> vps_get() const;
		RUST2_FFI_CALLABLE void vps_set(const ::moq::items<::moq::view> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> sps_get() const;
		RUST2_FFI_CALLABLE void sps_set(const ::moq::items<::moq::view> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> pps_get() const;
		RUST2_FFI_CALLABLE void pps_set(const ::moq::items<::moq::view> &value);
		static ::moq::result<::moq::mux_codec_h265_Hvcc, ::moq::mux_codec_h265_Error> parse(::moq::input hvcc);
	};
	class RUST2_FFI_CONSUMABLE mux_codec_h265_Sps {
		RUST2_FFI_UNCONSUMED mux_codec_h265_Sps(mux_codec_h265_Sps &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_h265_Sps &operator=(mux_codec_h265_Sps &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_codec_h265_Sps(const mux_codec_h265_Sps &other) = default;
		mux_codec_h265_Sps &operator=(const mux_codec_h265_Sps &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::result<::moq::mux_codec_h265_Sps, ::moq::mux_codec_h265_Error> parse(::moq::input nal);
		RUST2_FFI_CALLABLE ::std::tuple<::std::uint32_t, ::std::uint32_t> size() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::mux_codec_h265_VideoSignal> video_signal() const;
	};
	class RUST2_FFI_CONSUMABLE mux_codec_h265_Import {
		RUST2_FFI_UNCONSUMED mux_codec_h265_Import(mux_codec_h265_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_h265_Import &operator=(mux_codec_h265_Import &&other) noexcept = default;
		static ::moq::result<::moq::mux_codec_h265_Import, ::moq::mux_Error> new_Extra(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Extra reserved, ::moq::mux_catalog_VideoHint hint);
		static ::moq::result<::moq::mux_codec_h265_Import, ::moq::mux_Error> new_Ext(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Ext reserved, ::moq::mux_catalog_VideoHint hint);
		static ::moq::result<::moq::mux_codec_h265_Import, ::moq::mux_Error> new_unit(::moq::track_Producer track, ::moq::mux_catalog_Reserved reserved, ::moq::mux_catalog_VideoHint hint);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> initialize(::moq::input buf_);
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Producer track() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_VideoConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_codec_h265_Import> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_enabled(bool enabled);
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> observe_reorder(const ::moq::Timestamp &reorder);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(const ::std::vector<::moq::mux_container_Frame> &frames);
	};
	class RUST2_FFI_CONSUMABLE mux_codec_h265_Split {
		RUST2_FFI_UNCONSUMED mux_codec_h265_Split(mux_codec_h265_Split &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_h265_Split &operator=(mux_codec_h265_Split &&other) noexcept = default;
		static mux_codec_h265_Split default_();
		static ::moq::mux_codec_h265_Split new_();
		RUST2_FFI_CALLABLE ::moq::result<::std::vector<::moq::mux_container_Frame>, ::moq::mux_Error> decode(::moq::input data, const ::std::optional<::moq::Timestamp> &pts);
		RUST2_FFI_CALLABLE ::moq::result<::std::vector<::moq::mux_container_Frame>, ::moq::mux_Error> flush(const ::std::optional<::moq::Timestamp> &pts);
		RUST2_FFI_CALLABLE void reset();
	};
	class RUST2_FFI_CONSUMABLE mux_codec_mp3_Import {
		RUST2_FFI_UNCONSUMED mux_codec_mp3_Import(mux_codec_mp3_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_mp3_Import &operator=(mux_codec_mp3_Import &&other) noexcept = default;
		static ::moq::result<::moq::mux_codec_mp3_Import, ::moq::mux_Error> new_Extra(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Extra reserved, ::moq::hang_catalog_AudioConfig config);
		static ::moq::result<::moq::mux_codec_mp3_Import, ::moq::mux_Error> new_Ext(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Ext reserved, ::moq::hang_catalog_AudioConfig config);
		static ::moq::result<::moq::mux_codec_mp3_Import, ::moq::mux_Error> new_unit(::moq::track_Producer track, ::moq::mux_catalog_Reserved reserved, ::moq::hang_catalog_AudioConfig config);
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_AudioConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_codec_mp3_Import> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input frame, const ::std::optional<::moq::Timestamp> &pts);
	};
	class RUST2_FFI_CONSUMABLE mux_codec_opus_Config {
		RUST2_FFI_UNCONSUMED mux_codec_opus_Config(mux_codec_opus_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_opus_Config &operator=(mux_codec_opus_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_codec_opus_Config clone() const;
		friend bool operator==(const mux_codec_opus_Config &a, const mux_codec_opus_Config &b);
		friend bool operator!=(const mux_codec_opus_Config &a, const mux_codec_opus_Config &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::uint32_t sample_rate_get() const;
		RUST2_FFI_CALLABLE void sample_rate_set(::std::uint32_t value);
		RUST2_FFI_CALLABLE ::std::uint32_t channel_count_get() const;
		RUST2_FFI_CALLABLE void channel_count_set(::std::uint32_t value);
		RUST2_FFI_CALLABLE ::std::uint16_t pre_skip_get() const;
		RUST2_FFI_CALLABLE void pre_skip_set(::std::uint16_t value);
		RUST2_FFI_CALLABLE ::std::int16_t output_gain_get() const;
		RUST2_FFI_CALLABLE void output_gain_set(::std::int16_t value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mux_codec_opus_Mapping> mapping_get() const;
		RUST2_FFI_CALLABLE void mapping_set(const ::std::optional<::moq::mux_codec_opus_Mapping> &value);
		static ::moq::mux_codec_opus_Config new_(::std::uint32_t sample_rate, ::std::uint32_t channel_count);
		RUST2_FFI_CALLABLE ::moq::mux_codec_opus_Config with_pre_skip(::std::uint16_t pre_skip) const;
		static ::moq::result<::moq::mux_codec_opus_Config, ::moq::mux_codec_opus_Error> parse(::moq::buf &buf_);
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::mux_codec_opus_Error> encode() const;
	};
	class RUST2_FFI_CONSUMABLE mux_codec_opus_Import {
		RUST2_FFI_UNCONSUMED mux_codec_opus_Import(mux_codec_opus_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_opus_Import &operator=(mux_codec_opus_Import &&other) noexcept = default;
		static ::moq::result<::moq::mux_codec_opus_Import, ::moq::mux_Error> new_Extra(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Extra reserved, ::moq::hang_catalog_AudioConfig config);
		static ::moq::result<::moq::mux_codec_opus_Import, ::moq::mux_Error> new_Ext(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Ext reserved, ::moq::hang_catalog_AudioConfig config);
		static ::moq::result<::moq::mux_codec_opus_Import, ::moq::mux_Error> new_unit(::moq::track_Producer track, ::moq::mux_catalog_Reserved reserved, ::moq::hang_catalog_AudioConfig config);
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_AudioConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_codec_opus_Import> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input frame, const ::std::optional<::moq::Timestamp> &pts);
	};
	class RUST2_FFI_CONSUMABLE mux_codec_vp8_Import {
		RUST2_FFI_UNCONSUMED mux_codec_vp8_Import(mux_codec_vp8_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_vp8_Import &operator=(mux_codec_vp8_Import &&other) noexcept = default;
		static ::moq::result<::moq::mux_codec_vp8_Import, ::moq::mux_Error> new_Extra(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Extra reserved, ::moq::mux_catalog_VideoHint hint);
		static ::moq::result<::moq::mux_codec_vp8_Import, ::moq::mux_Error> new_Ext(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Ext reserved, ::moq::mux_catalog_VideoHint hint);
		static ::moq::result<::moq::mux_codec_vp8_Import, ::moq::mux_Error> new_unit(::moq::track_Producer track, ::moq::mux_catalog_Reserved reserved, ::moq::mux_catalog_VideoHint hint);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> initialize(::moq::input buf_);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input frame, const ::std::optional<::moq::Timestamp> &pts);
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_VideoConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_codec_vp8_Import> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_enabled(bool enabled);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
	};
	class RUST2_FFI_CONSUMABLE mux_codec_vp9_Import {
		RUST2_FFI_UNCONSUMED mux_codec_vp9_Import(mux_codec_vp9_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_vp9_Import &operator=(mux_codec_vp9_Import &&other) noexcept = default;
		static ::moq::result<::moq::mux_codec_vp9_Import, ::moq::mux_Error> new_Extra(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Extra reserved, ::moq::mux_catalog_VideoHint hint);
		static ::moq::result<::moq::mux_codec_vp9_Import, ::moq::mux_Error> new_Ext(::moq::track_Producer track, ::moq::mux_catalog_Reserved_Ext reserved, ::moq::mux_catalog_VideoHint hint);
		static ::moq::result<::moq::mux_codec_vp9_Import, ::moq::mux_Error> new_unit(::moq::track_Producer track, ::moq::mux_catalog_Reserved reserved, ::moq::mux_catalog_VideoHint hint);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> initialize(::moq::input buf_);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input frame, const ::std::optional<::moq::Timestamp> &pts);
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_Finished_Container_VideoConfig, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::mux_codec_vp9_Import> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_enabled(bool enabled);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
	};
	class RUST2_FFI_CONSUMABLE mux_container_flv_Import {
		RUST2_FFI_UNCONSUMED mux_container_flv_Import(mux_container_flv_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_flv_Import &operator=(mux_container_flv_Import &&other) noexcept = default;
		static ::moq::mux_container_flv_Import new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_flv_Import with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_flv_Finished, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_flv_Import_Extra {
		RUST2_FFI_UNCONSUMED mux_container_flv_Import_Extra(mux_container_flv_Import_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_flv_Import_Extra &operator=(mux_container_flv_Import_Extra &&other) noexcept = default;
		static ::moq::mux_container_flv_Import_Extra new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Extra reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_flv_Import_Extra with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_flv_Finished_Extra, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_flv_Import_Ext {
		RUST2_FFI_UNCONSUMED mux_container_flv_Import_Ext(mux_container_flv_Import_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_flv_Import_Ext &operator=(mux_container_flv_Import_Ext &&other) noexcept = default;
		static ::moq::mux_container_flv_Import_Ext new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Ext reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_flv_Import_Ext with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_flv_Finished_Ext, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_flv_Finished {
		RUST2_FFI_UNCONSUMED mux_container_flv_Finished(mux_container_flv_Finished &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_flv_Finished &operator=(mux_container_flv_Finished &&other) noexcept = default;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_flv_Finished_Extra {
		RUST2_FFI_UNCONSUMED mux_container_flv_Finished_Extra(mux_container_flv_Finished_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_flv_Finished_Extra &operator=(mux_container_flv_Finished_Extra &&other) noexcept = default;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_flv_Finished_Ext {
		RUST2_FFI_UNCONSUMED mux_container_flv_Finished_Ext(mux_container_flv_Finished_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_flv_Finished_Ext &operator=(mux_container_flv_Finished_Ext &&other) noexcept = default;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Wire {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Wire(mux_container_fmp4_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Wire &operator=(mux_container_fmp4_Wire &&other) noexcept = default;
		static ::moq::result<::moq::mux_container_fmp4_Wire, ::moq::mux_container_fmp4_Error> from_init(::moq::input init_data);
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Error {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Error(mux_container_fmp4_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Error &operator=(mux_container_fmp4_Error &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Error(const mux_container_fmp4_Error &other) = default;
		mux_container_fmp4_Error &operator=(const mux_container_fmp4_Error &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Import {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Import(mux_container_fmp4_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Import &operator=(mux_container_fmp4_Import &&other) noexcept = default;
		static ::moq::mux_container_fmp4_Import new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved reserved);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut();
		RUST2_FFI_CONSUMES ::moq::mux_container_fmp4_Import with_select(::moq::mux_select_Broadcast select) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_fmp4_Import with_cmaf(bool cmaf) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_fmp4_Finished, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> rejoin();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Import_Extra {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Import_Extra(mux_container_fmp4_Import_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Import_Extra &operator=(mux_container_fmp4_Import_Extra &&other) noexcept = default;
		static ::moq::mux_container_fmp4_Import_Extra new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Extra reserved);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut();
		RUST2_FFI_CONSUMES ::moq::mux_container_fmp4_Import_Extra with_select(::moq::mux_select_Broadcast select) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_fmp4_Import_Extra with_cmaf(bool cmaf) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_fmp4_Finished_Extra, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> rejoin();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Import_Ext {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Import_Ext(mux_container_fmp4_Import_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Import_Ext &operator=(mux_container_fmp4_Import_Ext &&other) noexcept = default;
		static ::moq::mux_container_fmp4_Import_Ext new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Ext reserved);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut();
		RUST2_FFI_CONSUMES ::moq::mux_container_fmp4_Import_Ext with_select(::moq::mux_select_Broadcast select) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_fmp4_Import_Ext with_cmaf(bool cmaf) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_fmp4_Finished_Ext, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> rejoin();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Finished {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Finished(mux_container_fmp4_Finished &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Finished &operator=(mux_container_fmp4_Finished &&other) noexcept = default;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Finished_Extra {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Finished_Extra(mux_container_fmp4_Finished_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Finished_Extra &operator=(mux_container_fmp4_Finished_Extra &&other) noexcept = default;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Finished_Ext {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Finished_Ext(mux_container_fmp4_Finished_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Finished_Ext &operator=(mux_container_fmp4_Finished_Ext &&other) noexcept = default;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Muxer {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Muxer(mux_container_fmp4_Muxer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Muxer &operator=(mux_container_fmp4_Muxer &&other) noexcept = default;
		static ::moq::result<::moq::mux_container_fmp4_Muxer, ::moq::mux_Error> video(const ::moq::hang_catalog_VideoConfig &config);
		static ::moq::result<::moq::mux_container_fmp4_Muxer, ::moq::mux_Error> audio(const ::moq::hang_catalog_AudioConfig &config);
		RUST2_FFI_CALLABLE ::moq::Timescale timescale() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_fmp4_Muxer, ::moq::mux_Error> with_timescale(const ::moq::Timescale &timescale) &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_fmp4_Muxer_read_future read(::moq::group_Consumer &group);
		RUST2_FFI_CALLABLE ::moq::result<::std::optional<::std::tuple<::std::uint32_t, ::std::uint32_t>>, ::moq::mux_Error> picture_size(const ::moq::mux_container_Frame &keyframe);
		RUST2_FFI_CALLABLE ::moq::result<::std::optional<::moq::buf>, ::moq::mux_Error> init() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::mux_Error> fragment(::std::uint32_t sequence, const ::std::vector<::moq::mux_container_Frame> &frames, const ::std::optional<::moq::Timestamp> &end) const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_legacy_Audio {
		RUST2_FFI_UNCONSUMED mux_container_legacy_Audio(mux_container_legacy_Audio &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_legacy_Audio &operator=(mux_container_legacy_Audio &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_container_legacy_Audio(const mux_container_legacy_Audio &other) = default;
		mux_container_legacy_Audio &operator=(const mux_container_legacy_Audio &other) = default;
		friend bool operator==(const mux_container_legacy_Audio &a, const mux_container_legacy_Audio &b);
		friend bool operator!=(const mux_container_legacy_Audio &a, const mux_container_legacy_Audio &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::std::optional<::moq::mux_container_legacy_Audio> new_(const ::moq::hang_catalog_AudioConfig &config);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> duration(::moq::input payload) const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_mkv_Import {
		RUST2_FFI_UNCONSUMED mux_container_mkv_Import(mux_container_mkv_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_mkv_Import &operator=(mux_container_mkv_Import &&other) noexcept = default;
		static ::moq::mux_container_mkv_Import new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_mkv_Import with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_mkv_Finished, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_mkv_Import_Extra {
		RUST2_FFI_UNCONSUMED mux_container_mkv_Import_Extra(mux_container_mkv_Import_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_mkv_Import_Extra &operator=(mux_container_mkv_Import_Extra &&other) noexcept = default;
		static ::moq::mux_container_mkv_Import_Extra new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Extra reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_mkv_Import_Extra with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_mkv_Finished_Extra, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_mkv_Import_Ext {
		RUST2_FFI_UNCONSUMED mux_container_mkv_Import_Ext(mux_container_mkv_Import_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_mkv_Import_Ext &operator=(mux_container_mkv_Import_Ext &&other) noexcept = default;
		static ::moq::mux_container_mkv_Import_Ext new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Ext reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_mkv_Import_Ext with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t start);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_mkv_Finished_Ext, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_mkv_Finished {
		RUST2_FFI_UNCONSUMED mux_container_mkv_Finished(mux_container_mkv_Finished &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_mkv_Finished &operator=(mux_container_mkv_Finished &&other) noexcept = default;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_mkv_Finished_Extra {
		RUST2_FFI_UNCONSUMED mux_container_mkv_Finished_Extra(mux_container_mkv_Finished_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_mkv_Finished_Extra &operator=(mux_container_mkv_Finished_Extra &&other) noexcept = default;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_mkv_Finished_Ext {
		RUST2_FFI_UNCONSUMED mux_container_mkv_Finished_Ext(mux_container_mkv_Finished_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_mkv_Finished_Ext &operator=(mux_container_mkv_Finished_Ext &&other) noexcept = default;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_mp4_Finish {
		RUST2_FFI_UNCONSUMED mux_container_mp4_Finish(mux_container_mp4_Finish &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_mp4_Finish &operator=(mux_container_mp4_Finish &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_container_mp4_Finish(const mux_container_mp4_Finish &other) = default;
		mux_container_mp4_Finish &operator=(const mux_container_mp4_Finish &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Ext {
		RUST2_FFI_UNCONSUMED mux_container_ts_Ext(mux_container_ts_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Ext &operator=(mux_container_ts_Ext &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_container_ts_Ext clone() const;
		friend bool operator==(const mux_container_ts_Ext &a, const mux_container_ts_Ext &b);
		friend bool operator!=(const mux_container_ts_Ext &a, const mux_container_ts_Ext &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_container_ts_Ext default_();
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Mpegts mpegts_get() const;
		RUST2_FFI_CALLABLE void mpegts_set(::moq::mux_container_ts_Mpegts value);
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Mpegts {
		RUST2_FFI_UNCONSUMED mux_container_ts_Mpegts(mux_container_ts_Mpegts &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Mpegts &operator=(mux_container_ts_Mpegts &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_container_ts_Mpegts clone() const;
		friend bool operator==(const mux_container_ts_Mpegts &a, const mux_container_ts_Mpegts &b);
		friend bool operator!=(const mux_container_ts_Mpegts &a, const mux_container_ts_Mpegts &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_container_ts_Mpegts default_();
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::mux_container_ts_Track>> tracks_get() const;
		RUST2_FFI_CALLABLE void tracks_set(::moq::taken<::std::tuple<::moq::view, ::moq::mux_container_ts_Track>> value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mux_container_ts_Descriptor> program_descriptors_get() const;
		RUST2_FFI_CALLABLE void program_descriptors_set(const ::std::vector<::moq::mux_container_ts_Descriptor> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mux_container_ts_Program> program_get() const;
		RUST2_FFI_CALLABLE void program_set(::std::optional<::moq::mux_container_ts_Program> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::std::uint16_t, ::std::vector<::std::tuple<::std::uint8_t, ::moq::mux_container_ts_SiEntry>>>> si_get() const;
		RUST2_FFI_CALLABLE void si_set(::std::vector<::std::tuple<::std::uint16_t, ::std::vector<::std::tuple<::std::uint8_t, ::moq::mux_container_ts_SiEntry>>>> value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> mux_rate_get() const;
		RUST2_FFI_CALLABLE void mux_rate_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mux_container_ts_PassthroughTrack> passthrough_get() const;
		RUST2_FFI_CALLABLE void passthrough_set(::std::optional<::moq::mux_container_ts_PassthroughTrack> value);
		static ::moq::result<::moq::mux_container_ts_Mpegts, ::moq::buf> deserialize(::moq::Json __deserializer);
		RUST2_FFI_CALLABLE bool is_empty() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_PassthroughTrack {
		RUST2_FFI_UNCONSUMED mux_container_ts_PassthroughTrack(mux_container_ts_PassthroughTrack &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_PassthroughTrack &operator=(mux_container_ts_PassthroughTrack &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_container_ts_PassthroughTrack clone() const;
		friend bool operator==(const mux_container_ts_PassthroughTrack &a, const mux_container_ts_PassthroughTrack &b);
		friend bool operator!=(const mux_container_ts_PassthroughTrack &a, const mux_container_ts_PassthroughTrack &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::buf track_get() const;
		RUST2_FFI_CALLABLE void track_set(::moq::input value);
		RUST2_FFI_CALLABLE bool random_access_get() const;
		RUST2_FFI_CALLABLE void random_access_set(bool value);
		static ::moq::mux_container_ts_PassthroughTrack new_(::moq::input track);
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_PesPacket {
		RUST2_FFI_UNCONSUMED mux_container_ts_PesPacket(mux_container_ts_PesPacket &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_PesPacket &operator=(mux_container_ts_PesPacket &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_container_ts_PesPacket clone() const;
		friend bool operator==(const mux_container_ts_PesPacket &a, const mux_container_ts_PesPacket &b);
		friend bool operator!=(const mux_container_ts_PesPacket &a, const mux_container_ts_PesPacket &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_container_ts_PesPacket default_();
		RUST2_FFI_CALLABLE ::std::uint8_t stream_type_get() const;
		RUST2_FFI_CALLABLE void stream_type_set(::std::uint8_t value);
		static ::moq::mux_container_ts_PesPacket new_(::std::uint8_t stream_type);
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Program {
		RUST2_FFI_UNCONSUMED mux_container_ts_Program(mux_container_ts_Program &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Program &operator=(mux_container_ts_Program &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_container_ts_Program clone() const;
		friend bool operator==(const mux_container_ts_Program &a, const mux_container_ts_Program &b);
		friend bool operator!=(const mux_container_ts_Program &a, const mux_container_ts_Program &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_container_ts_Program default_();
		RUST2_FFI_CALLABLE ::std::uint16_t transport_stream_id_get() const;
		RUST2_FFI_CALLABLE void transport_stream_id_set(::std::uint16_t value);
		RUST2_FFI_CALLABLE ::std::uint16_t program_number_get() const;
		RUST2_FFI_CALLABLE void program_number_set(::std::uint16_t value);
		RUST2_FFI_CALLABLE ::std::uint16_t pmt_pid_get() const;
		RUST2_FFI_CALLABLE void pmt_pid_set(::std::uint16_t value);
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_SiEntry {
		RUST2_FFI_UNCONSUMED mux_container_ts_SiEntry(mux_container_ts_SiEntry &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_SiEntry &operator=(mux_container_ts_SiEntry &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_container_ts_SiEntry clone() const;
		friend bool operator==(const mux_container_ts_SiEntry &a, const mux_container_ts_SiEntry &b);
		friend bool operator!=(const mux_container_ts_SiEntry &a, const mux_container_ts_SiEntry &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_container_ts_SiEntry default_();
		RUST2_FFI_CALLABLE ::moq::buf track_get() const;
		RUST2_FFI_CALLABLE void track_set(::moq::input value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> interval_get() const;
		RUST2_FFI_CALLABLE void interval_set(const ::std::optional<::moq::duration> &value);
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Track {
		RUST2_FFI_UNCONSUMED mux_container_ts_Track(mux_container_ts_Track &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Track &operator=(mux_container_ts_Track &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_container_ts_Track clone() const;
		friend bool operator==(const mux_container_ts_Track &a, const mux_container_ts_Track &b);
		friend bool operator!=(const mux_container_ts_Track &a, const mux_container_ts_Track &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_container_ts_Track default_();
		RUST2_FFI_CALLABLE ::std::uint16_t pid_get() const;
		RUST2_FFI_CALLABLE void pid_set(::std::uint16_t value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mux_container_ts_Descriptor> descriptors_get() const;
		RUST2_FFI_CALLABLE void descriptors_set(const ::std::vector<::moq::mux_container_ts_Descriptor> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mux_container_ts_Verbatim> verbatim_get() const;
		RUST2_FFI_CALLABLE void verbatim_set(::std::optional<::moq::mux_container_ts_Verbatim> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mux_container_ts_PesPacket> pes_packet_get() const;
		RUST2_FFI_CALLABLE void pes_packet_set(::std::optional<::moq::mux_container_ts_PesPacket> value);
		static ::moq::mux_container_ts_Track new_(::std::uint16_t pid);
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Verbatim {
		RUST2_FFI_UNCONSUMED mux_container_ts_Verbatim(mux_container_ts_Verbatim &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Verbatim &operator=(mux_container_ts_Verbatim &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_container_ts_Verbatim clone() const;
		friend bool operator==(const mux_container_ts_Verbatim &a, const mux_container_ts_Verbatim &b);
		friend bool operator!=(const mux_container_ts_Verbatim &a, const mux_container_ts_Verbatim &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_container_ts_Verbatim default_();
		RUST2_FFI_CALLABLE ::std::uint8_t stream_type_get() const;
		RUST2_FFI_CALLABLE void stream_type_set(::std::uint8_t value);
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Framing framing_get() const;
		RUST2_FFI_CALLABLE void framing_set(::moq::mux_container_ts_Framing value);
		static ::moq::mux_container_ts_Verbatim new_(::std::uint8_t stream_type, ::moq::mux_container_ts_Framing framing);
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Error {
		RUST2_FFI_UNCONSUMED mux_container_ts_Error(mux_container_ts_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Error &operator=(mux_container_ts_Error &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_container_ts_Error(const mux_container_ts_Error &other) = default;
		mux_container_ts_Error &operator=(const mux_container_ts_Error &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Export_Consumer_Extra {
		RUST2_FFI_UNCONSUMED mux_container_ts_Export_Consumer_Extra(mux_container_ts_Export_Consumer_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Export_Consumer_Extra &operator=(mux_container_ts_Export_Consumer_Extra &&other) noexcept = default;
		static ::moq::mux_container_ts_Export_Consumer_Extra new_(::moq::mux_Source source, ::moq::mux_catalog_Consumer_Extra catalog, ::moq::duration delay);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Export_Consumer_Extra with_mux_rate(::std::uint64_t mux_rate) &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Export_Consumer_Extra_next_future next();
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Export stats() const;
		RUST2_FFI_CALLABLE ::std::uint64_t discontinuity() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_ts_Export_Consumer_Extra, ::moq::mux_Error> follow(::moq::broadcast_Consumer broadcast, ::moq::mux_catalog_Consumer_Extra catalog) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Export_Consumer_Ext {
		RUST2_FFI_UNCONSUMED mux_container_ts_Export_Consumer_Ext(mux_container_ts_Export_Consumer_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Export_Consumer_Ext &operator=(mux_container_ts_Export_Consumer_Ext &&other) noexcept = default;
		static ::moq::mux_container_ts_Export_Consumer_Ext new_(::moq::mux_Source source, ::moq::mux_catalog_Consumer_Ext catalog, ::moq::duration delay);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Export_Consumer_Ext with_mux_rate(::std::uint64_t mux_rate) &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Export_Consumer_Ext_next_future next();
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Export stats() const;
		RUST2_FFI_CALLABLE ::std::uint64_t discontinuity() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_ts_Export_Consumer_Ext, ::moq::mux_Error> follow(::moq::broadcast_Consumer broadcast, ::moq::mux_catalog_Consumer_Ext catalog) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Export_Consumer {
		RUST2_FFI_UNCONSUMED mux_container_ts_Export_Consumer(mux_container_ts_Export_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Export_Consumer &operator=(mux_container_ts_Export_Consumer &&other) noexcept = default;
		static ::moq::mux_container_ts_Export_Consumer new_(::moq::mux_Source source, ::moq::mux_catalog_Consumer catalog, ::moq::duration delay);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Export_Consumer with_mux_rate(::std::uint64_t mux_rate) &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Export_Consumer_next_future next();
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Export stats() const;
		RUST2_FFI_CALLABLE ::std::uint64_t discontinuity() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_ts_Export_Consumer, ::moq::mux_Error> follow(::moq::broadcast_Consumer broadcast, ::moq::mux_catalog_Consumer catalog) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Follower {
		RUST2_FFI_UNCONSUMED mux_container_ts_Follower(mux_container_ts_Follower &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Follower &operator=(mux_container_ts_Follower &&other) noexcept = default;
		static ::moq::result<::moq::mux_container_ts_Follower, ::moq::mux_Error> new_(::moq::mux_container_ts_Export_Consumer export_);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Follower with_follow(::moq::duration follow) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Follower with_stitch(bool stitch) &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Export_Consumer export_() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Follower_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Follower_Extra {
		RUST2_FFI_UNCONSUMED mux_container_ts_Follower_Extra(mux_container_ts_Follower_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Follower_Extra &operator=(mux_container_ts_Follower_Extra &&other) noexcept = default;
		static ::moq::result<::moq::mux_container_ts_Follower_Extra, ::moq::mux_Error> new_(::moq::mux_container_ts_Export_Consumer_Extra export_);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Follower_Extra with_follow(::moq::duration follow) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Follower_Extra with_stitch(bool stitch) &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Export_Consumer_Extra export_() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Follower_Extra_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Follower_Ext {
		RUST2_FFI_UNCONSUMED mux_container_ts_Follower_Ext(mux_container_ts_Follower_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Follower_Ext &operator=(mux_container_ts_Follower_Ext &&other) noexcept = default;
		static ::moq::result<::moq::mux_container_ts_Follower_Ext, ::moq::mux_Error> new_(::moq::mux_container_ts_Export_Consumer_Ext export_);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Follower_Ext with_follow(::moq::duration follow) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Follower_Ext with_stitch(bool stitch) &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Export_Consumer_Ext export_() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Follower_Ext_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Passthrough {
		RUST2_FFI_UNCONSUMED mux_container_ts_Passthrough(mux_container_ts_Passthrough &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Passthrough &operator=(mux_container_ts_Passthrough &&other) noexcept = default;
		static ::moq::result<::moq::mux_container_ts_Passthrough, ::moq::mux_Error> new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Ext reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Passthrough with_pcr_pid(::std::uint16_t pid) &&;
		RUST2_FFI_CALLABLE ::moq::buf track() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_ts_passthrough_Finished, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Passthrough_unit {
		RUST2_FFI_UNCONSUMED mux_container_ts_Passthrough_unit(mux_container_ts_Passthrough_unit &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Passthrough_unit &operator=(mux_container_ts_Passthrough_unit &&other) noexcept = default;
		static ::moq::result<::moq::mux_container_ts_Passthrough_unit, ::moq::mux_Error> new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Passthrough_unit with_pcr_pid(::std::uint16_t pid) &&;
		RUST2_FFI_CALLABLE ::moq::buf track() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_ts_passthrough_Finished_unit, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Passthrough_Extra {
		RUST2_FFI_UNCONSUMED mux_container_ts_Passthrough_Extra(mux_container_ts_Passthrough_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Passthrough_Extra &operator=(mux_container_ts_Passthrough_Extra &&other) noexcept = default;
		static ::moq::result<::moq::mux_container_ts_Passthrough_Extra, ::moq::mux_Error> new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Extra reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Passthrough_Extra with_pcr_pid(::std::uint16_t pid) &&;
		RUST2_FFI_CALLABLE ::moq::buf track() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_ts_passthrough_Finished_Extra, ::moq::mux_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Programs {
		RUST2_FFI_UNCONSUMED mux_container_ts_Programs(mux_container_ts_Programs &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Programs &operator=(mux_container_ts_Programs &&other) noexcept = default;
		static ::moq::mux_container_ts_Programs new_(::moq::origin_Producer origin, ::moq::Path name, ::moq::mux_catalog_Config config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_container_ts_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_ts_programs_Finished, ::moq::mux_container_ts_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Release {
		RUST2_FFI_UNCONSUMED mux_container_ts_Release(mux_container_ts_Release &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Release &operator=(mux_container_ts_Release &&other) noexcept = default;
		static ::moq::mux_container_ts_Release_new_future new_(::moq::mux_Source source, ::moq::mux_catalog_CatalogFormat catalog_format, ::moq::duration delay);
		RUST2_FFI_CALLABLE ::std::uint64_t dropped() const;
		RUST2_FFI_CALLABLE ::std::optional<double> drift() const;
		RUST2_FFI_CALLABLE ::std::uint64_t out_of_tolerance() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Release stats() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Release_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Republish {
		RUST2_FFI_UNCONSUMED mux_container_ts_Republish(mux_container_ts_Republish &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Republish &operator=(mux_container_ts_Republish &&other) noexcept = default;
		static ::moq::result<::moq::mux_container_ts_Republish, ::moq::mux_container_ts_Error> new_(::moq::origin_Producer origin, ::moq::Path name, ::moq::Instance instance, ::moq::mux_catalog_Config config);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Republish with_program(::std::uint16_t program) &&;
		RUST2_FFI_CALLABLE ::moq::broadcast_Producer broadcast() const;
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Producer_Ext catalog() const;
		RUST2_FFI_CALLABLE ::moq::Instance instance() const;
		RUST2_FFI_CALLABLE ::moq::result<::std::optional<::moq::mux_container_ts_Republished>, ::moq::mux_container_ts_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_ts_republish_Finished, ::moq::mux_container_ts_Error> finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Republished {
		RUST2_FFI_UNCONSUMED mux_container_ts_Republished(mux_container_ts_Republished &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Republished &operator=(mux_container_ts_Republished &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_container_ts_Republished clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Step step_get() const;
		RUST2_FFI_CALLABLE void step_set(const ::moq::mux_container_ts_Step &value);
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(::moq::mux_container_ts_stats_Snapshot value);
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Import {
		RUST2_FFI_UNCONSUMED mux_container_ts_Import(mux_container_ts_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Import &operator=(mux_container_ts_Import &&other) noexcept = default;
		static ::moq::mux_container_ts_Import new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Import with_program(::std::uint16_t program) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Import with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_container_ts_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_container_ts_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_ts_Finished, ::moq::mux_container_ts_Error> finish() &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Import_Extra {
		RUST2_FFI_UNCONSUMED mux_container_ts_Import_Extra(mux_container_ts_Import_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Import_Extra &operator=(mux_container_ts_Import_Extra &&other) noexcept = default;
		static ::moq::mux_container_ts_Import_Extra new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Extra reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Import_Extra with_program(::std::uint16_t program) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Import_Extra with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_container_ts_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_container_ts_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_ts_Finished_Extra, ::moq::mux_container_ts_Error> finish() &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Import_Ext {
		RUST2_FFI_UNCONSUMED mux_container_ts_Import_Ext(mux_container_ts_Import_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Import_Ext &operator=(mux_container_ts_Import_Ext &&other) noexcept = default;
		static ::moq::mux_container_ts_Import_Ext new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Ext reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Import_Ext with_program(::std::uint16_t program) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Import_Ext with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_container_ts_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_container_ts_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_ts_Finished_Ext, ::moq::mux_container_ts_Error> finish() &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Finished {
		RUST2_FFI_UNCONSUMED mux_container_ts_Finished(mux_container_ts_Finished &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Finished &operator=(mux_container_ts_Finished &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Finished_Extra {
		RUST2_FFI_UNCONSUMED mux_container_ts_Finished_Extra(mux_container_ts_Finished_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Finished_Extra &operator=(mux_container_ts_Finished_Extra &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Finished_Ext {
		RUST2_FFI_UNCONSUMED mux_container_ts_Finished_Ext(mux_container_ts_Finished_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Finished_Ext &operator=(mux_container_ts_Finished_Ext &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_import_container_Finished {
		RUST2_FFI_UNCONSUMED mux_import_container_Finished(mux_import_container_Finished &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_container_Finished &operator=(mux_import_container_Finished &&other) noexcept = default;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_import_container_Finished_Extra {
		RUST2_FFI_UNCONSUMED mux_import_container_Finished_Extra(mux_import_container_Finished_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_container_Finished_Extra &operator=(mux_import_container_Finished_Extra &&other) noexcept = default;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_import_container_Finished_Ext {
		RUST2_FFI_UNCONSUMED mux_import_container_Finished_Ext(mux_import_container_Finished_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_container_Finished_Ext &operator=(mux_import_container_Finished_Ext &&other) noexcept = default;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_import_track_Finished {
		RUST2_FFI_UNCONSUMED mux_import_track_Finished(mux_import_track_Finished &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_track_Finished &operator=(mux_import_track_Finished &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_codec_aac_adts_Header {
		RUST2_FFI_UNCONSUMED mux_codec_aac_adts_Header(mux_codec_aac_adts_Header &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_codec_aac_adts_Header &operator=(mux_codec_aac_adts_Header &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_codec_aac_adts_Header clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::uint8_t object_type_get() const;
		RUST2_FFI_CALLABLE void object_type_set(::std::uint8_t value);
		RUST2_FFI_CALLABLE ::std::uint32_t sample_rate_get() const;
		RUST2_FFI_CALLABLE void sample_rate_set(::std::uint32_t value);
		RUST2_FFI_CALLABLE ::std::uint8_t channel_config_get() const;
		RUST2_FFI_CALLABLE void channel_config_set(::std::uint8_t value);
		RUST2_FFI_CALLABLE ::std::uint8_t blocks_get() const;
		RUST2_FFI_CALLABLE void blocks_set(::std::uint8_t value);
		static ::moq::result<::moq::mux_codec_aac_adts_Header, ::moq::mux_codec_aac_adts_Error> parse(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::mux_codec_aac_adts_Error> payload(::moq::input packet) const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_passthrough_Finished {
		RUST2_FFI_UNCONSUMED mux_container_ts_passthrough_Finished(mux_container_ts_passthrough_Finished &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_passthrough_Finished &operator=(mux_container_ts_passthrough_Finished &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf track() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_passthrough_Finished_unit {
		RUST2_FFI_UNCONSUMED mux_container_ts_passthrough_Finished_unit(mux_container_ts_passthrough_Finished_unit &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_passthrough_Finished_unit &operator=(mux_container_ts_passthrough_Finished_unit &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf track() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_passthrough_Finished_Extra {
		RUST2_FFI_UNCONSUMED mux_container_ts_passthrough_Finished_Extra(mux_container_ts_passthrough_Finished_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_passthrough_Finished_Extra &operator=(mux_container_ts_passthrough_Finished_Extra &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf track() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_programs_Finished {
		RUST2_FFI_UNCONSUMED mux_container_ts_programs_Finished(mux_container_ts_programs_Finished &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_programs_Finished &operator=(mux_container_ts_programs_Finished &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::std::vector<::moq::broadcast_Producer> broadcasts() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_republish_Finished {
		RUST2_FFI_UNCONSUMED mux_container_ts_republish_Finished(mux_container_ts_republish_Finished &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_republish_Finished &operator=(mux_container_ts_republish_Finished &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::broadcast_Producer broadcast() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_stats_Report {
		RUST2_FFI_UNCONSUMED mux_container_ts_stats_Report(mux_container_ts_stats_Report &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_stats_Report &operator=(mux_container_ts_stats_Report &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_container_ts_stats_Report clone() const;
		friend bool operator==(const mux_container_ts_stats_Report &a, const mux_container_ts_stats_Report &b);
		friend bool operator!=(const mux_container_ts_stats_Report &a, const mux_container_ts_stats_Report &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_container_ts_stats_Report default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mux_container_ts_stats_Snapshot> mpegts_get() const;
		RUST2_FFI_CALLABLE void mpegts_set(::std::optional<::moq::mux_container_ts_stats_Snapshot> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mux_container_ts_stats_Release> release_get() const;
		RUST2_FFI_CALLABLE void release_set(::std::optional<::moq::mux_container_ts_stats_Release> value);
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_stats_Snapshot {
		RUST2_FFI_UNCONSUMED mux_container_ts_stats_Snapshot(mux_container_ts_stats_Snapshot &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_stats_Snapshot &operator=(mux_container_ts_stats_Snapshot &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_container_ts_stats_Snapshot clone() const;
		friend bool operator==(const mux_container_ts_stats_Snapshot &a, const mux_container_ts_stats_Snapshot &b);
		friend bool operator!=(const mux_container_ts_stats_Snapshot &a, const mux_container_ts_stats_Snapshot &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_container_ts_stats_Snapshot default_();
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::std::uint16_t, ::moq::mux_container_ts_stats_Stream>> streams_get() const;
		RUST2_FFI_CALLABLE void streams_set(::std::vector<::std::tuple<::std::uint16_t, ::moq::mux_container_ts_stats_Stream>> value);
		RUST2_FFI_CALLABLE ::std::uint64_t crc_error_get() const;
		RUST2_FFI_CALLABLE void crc_error_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t ts_sync_loss_get() const;
		RUST2_FFI_CALLABLE void ts_sync_loss_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t sync_byte_error_get() const;
		RUST2_FFI_CALLABLE void sync_byte_error_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t pat_error_get() const;
		RUST2_FFI_CALLABLE void pat_error_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t continuity_count_error_get() const;
		RUST2_FFI_CALLABLE void continuity_count_error_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t pmt_error_get() const;
		RUST2_FFI_CALLABLE void pmt_error_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t transport_error_get() const;
		RUST2_FFI_CALLABLE void transport_error_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t pcr_repetition_error_get() const;
		RUST2_FFI_CALLABLE void pcr_repetition_error_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t pcr_discontinuity_indicator_error_get() const;
		RUST2_FFI_CALLABLE void pcr_discontinuity_indicator_error_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t pts_error_get() const;
		RUST2_FFI_CALLABLE void pts_error_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE bool is_empty() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_stats_Stream {
		RUST2_FFI_UNCONSUMED mux_container_ts_stats_Stream(mux_container_ts_stats_Stream &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_stats_Stream &operator=(mux_container_ts_stats_Stream &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_container_ts_stats_Stream clone() const;
		friend bool operator==(const mux_container_ts_stats_Stream &a, const mux_container_ts_stats_Stream &b);
		friend bool operator!=(const mux_container_ts_stats_Stream &a, const mux_container_ts_stats_Stream &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_container_ts_stats_Stream default_();
		RUST2_FFI_CALLABLE ::moq::buf track_get() const;
		RUST2_FFI_CALLABLE void track_set(::moq::input value);
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Class class_get() const;
		RUST2_FFI_CALLABLE void class_set(::moq::mux_container_ts_stats_Class value);
		RUST2_FFI_CALLABLE ::std::uint64_t units_get() const;
		RUST2_FFI_CALLABLE void units_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> quiet_get() const;
		RUST2_FFI_CALLABLE void quiet_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::uint64_t resyncs_get() const;
		RUST2_FFI_CALLABLE void resyncs_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t discarded_get() const;
		RUST2_FFI_CALLABLE void discarded_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t unconfirmed_get() const;
		RUST2_FFI_CALLABLE void unconfirmed_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t damaged_get() const;
		RUST2_FFI_CALLABLE void damaged_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t unplaced_get() const;
		RUST2_FFI_CALLABLE void unplaced_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t refused_get() const;
		RUST2_FFI_CALLABLE void refused_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t continuity_count_error_get() const;
		RUST2_FFI_CALLABLE void continuity_count_error_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t transport_error_get() const;
		RUST2_FFI_CALLABLE void transport_error_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t pts_error_get() const;
		RUST2_FFI_CALLABLE void pts_error_set(::std::uint64_t value);
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_stats_Export {
		RUST2_FFI_UNCONSUMED mux_container_ts_stats_Export(mux_container_ts_stats_Export &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_stats_Export &operator=(mux_container_ts_stats_Export &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_container_ts_stats_Export clone() const;
		friend bool operator==(const mux_container_ts_stats_Export &a, const mux_container_ts_stats_Export &b);
		friend bool operator!=(const mux_container_ts_stats_Export &a, const mux_container_ts_stats_Export &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_container_ts_stats_Export default_();
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::std::uint16_t, ::moq::mux_container_ts_stats_Stream>> streams_get() const;
		RUST2_FFI_CALLABLE void streams_set(::std::vector<::std::tuple<::std::uint16_t, ::moq::mux_container_ts_stats_Stream>> value);
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Release release_get() const;
		RUST2_FFI_CALLABLE void release_set(::moq::mux_container_ts_stats_Release value);
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_stats_Release {
		RUST2_FFI_UNCONSUMED mux_container_ts_stats_Release(mux_container_ts_stats_Release &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_stats_Release &operator=(mux_container_ts_stats_Release &&other) noexcept = default;
		RUST2_FFI_CALLABLE mux_container_ts_stats_Release clone() const;
		friend bool operator==(const mux_container_ts_stats_Release &a, const mux_container_ts_stats_Release &b);
		friend bool operator!=(const mux_container_ts_stats_Release &a, const mux_container_ts_stats_Release &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mux_container_ts_stats_Release default_();
		RUST2_FFI_CALLABLE ::std::uint64_t dropped_get() const;
		RUST2_FFI_CALLABLE void dropped_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t trimmed_get() const;
		RUST2_FFI_CALLABLE void trimmed_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t gaps_get() const;
		RUST2_FFI_CALLABLE void gaps_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::optional<::std::int64_t> margin_get() const;
		RUST2_FFI_CALLABLE void margin_set(const ::std::optional<::std::int64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::int64_t> offset_get() const;
		RUST2_FFI_CALLABLE void offset_set(const ::std::optional<::std::int64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<double> drift_get() const;
		RUST2_FFI_CALLABLE void drift_set(const ::std::optional<double> &value);
		RUST2_FFI_CALLABLE ::std::uint64_t out_of_tolerance_get() const;
		RUST2_FFI_CALLABLE void out_of_tolerance_set(::std::uint64_t value);
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_stats_Log {
		RUST2_FFI_UNCONSUMED mux_container_ts_stats_Log(mux_container_ts_stats_Log &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_stats_Log &operator=(mux_container_ts_stats_Log &&other) noexcept = default;
		static mux_container_ts_stats_Log default_();
		static ::moq::duration INTERVAL();
		RUST2_FFI_CALLABLE void sample(::moq::mux_container_ts_stats_Snapshot latest);
		RUST2_FFI_CALLABLE void finish(const ::moq::mux_container_ts_stats_Snapshot &latest) const;
	};
	struct mux_container_Frame {
		::moq::Timestamp timestamp;
		::std::optional<::moq::Timestamp> duration;
		::moq::buf payload;
		bool keyframe;
		::moq::buf debug() const;
	};
	struct mux_container_TimestampRewind {
		::moq::Timestamp timestamp;
		::moq::Timestamp floor;
		friend bool operator==(const mux_container_TimestampRewind &a, const mux_container_TimestampRewind &b);
		friend bool operator!=(const mux_container_TimestampRewind &a, const mux_container_TimestampRewind &b);
		::moq::buf to_string() const;
		::moq::buf debug() const;
	};
	struct mux_container_Playout {
		::moq::track_Delay delay;
		::moq::duration warmup;
		friend bool operator==(const mux_container_Playout &a, const mux_container_Playout &b);
		friend bool operator!=(const mux_container_Playout &a, const mux_container_Playout &b);
		::moq::buf debug() const;
		static mux_container_Playout default_();
		static ::moq::mux_container_Playout new_(const ::moq::track_Delay &delay_, ::moq::duration warmup_);
		::moq::track_Subscription subscription(::moq::track_Subscription subscription) const;
		::moq::mux_container_Consumer_Container consumer_Container(::moq::track_Subscriber track, ::moq::mux_catalog_hang_Container format) const;
		::moq::mux_container_Consumer_fmp4_Wire consumer_fmp4_Wire(::moq::track_Subscriber track, ::moq::mux_container_fmp4_Wire format) const;
		::moq::mux_container_Consumer_legacy_Wire consumer_legacy_Wire(::moq::track_Subscriber track, const ::moq::mux_container_legacy_Wire &format) const;
		::moq::mux_container_Consumer_loc_Wire consumer_loc_Wire(::moq::track_Subscriber track, const ::moq::mux_container_loc_Wire &format) const;
	};
	struct mux_codec_flac_Config {
		::std::uint16_t min_block_size;
		::std::uint16_t max_block_size;
		::std::uint32_t min_frame_size;
		::std::uint32_t max_frame_size;
		::std::uint32_t sample_rate;
		::std::uint32_t channel_count;
		::std::uint32_t bits_per_sample;
		::std::uint64_t total_samples;
		::std::array<::std::uint8_t, 16> md5;
		friend bool operator==(const mux_codec_flac_Config &a, const mux_codec_flac_Config &b);
		friend bool operator!=(const mux_codec_flac_Config &a, const mux_codec_flac_Config &b);
		::moq::buf debug() const;
		static ::moq::result<::moq::mux_codec_flac_Config, ::moq::mux_codec_flac_Error> parse_stream_info(::moq::buf &buf_);
		static ::moq::result<::moq::mux_codec_flac_Config, ::moq::mux_codec_flac_Error> parse(::moq::buf &buf_);
		::moq::buf encode_stream_info() const;
		::moq::buf description() const;
	};
	struct mux_codec_h264_Sps {
		::std::uint8_t profile;
		::std::uint8_t constraints;
		::std::uint8_t level;
		::std::uint32_t coded_width;
		::std::uint32_t coded_height;
		::moq::buf debug() const;
		static ::moq::result<::moq::mux_codec_h264_Sps, ::moq::mux_codec_h264_Error> parse(::moq::input nal);
	};
	struct mux_codec_h265_VideoSignal {
		::std::uint8_t primaries;
		::std::uint8_t transfer;
		::std::uint8_t matrix;
		bool full_range;
		friend bool operator==(const mux_codec_h265_VideoSignal &a, const mux_codec_h265_VideoSignal &b);
		friend bool operator!=(const mux_codec_h265_VideoSignal &a, const mux_codec_h265_VideoSignal &b);
		::moq::buf debug() const;
	};
	struct mux_codec_mp3_Config {
		::std::uint32_t sample_rate;
		::std::uint32_t channel_count;
		static ::moq::result<::moq::mux_codec_mp3_Config, ::moq::mux_codec_mp3_Error> parse(::moq::input data);
	};
	class mux_codec_opus_Mapping {
		friend bool operator==(const mux_codec_opus_Mapping &a, const mux_codec_opus_Mapping &b);
		friend bool operator!=(const mux_codec_opus_Mapping &a, const mux_codec_opus_Mapping &b);
		::moq::buf debug() const;
		::std::uint8_t family() const;
		::std::uint8_t streams() const;
		::std::uint8_t coupled() const;
		::moq::buf table() const;
	};
	struct mux_container_fmp4_Fragment {
		::moq::buf data;
		bool independent;
		::moq::duration duration;
		::moq::buf debug() const;
	};
	struct mux_container_legacy_Wire {
		::moq::mux_container_Kind _0;
	};
	struct mux_container_loc_Wire {
		::moq::mux_container_Kind _0;
	};
	struct mux_container_ts_Descriptor {
		::std::uint8_t tag;
		::moq::buf data;
		friend bool operator==(const mux_container_ts_Descriptor &a, const mux_container_ts_Descriptor &b);
		friend bool operator!=(const mux_container_ts_Descriptor &a, const mux_container_ts_Descriptor &b);
		::moq::buf debug() const;
	};
	struct mux_container_Read_Frame {
		::std::uint64_t group;
		::moq::mux_container_Frame frame;
	};
	struct mux_container_Read_Gap {
		::moq::Timestamp timestamp;
		::moq::Timestamp duration;
	};
	struct mux_container_Read_End {
		::std::uint64_t group;
		::moq::Timestamp end;
	};
	struct mux_container_ReorderError_Depth {
		::std::uint8_t reorder;
		::std::uint32_t max;
		::moq::buf codec;
	};
	struct mux_container_ReorderError_Deepened {
		::std::uint8_t declared;
		::std::uint8_t reorder;
	};
	struct mux_container_ReorderError_Reordered {
		::std::size_t _0;
	};
	struct mux_container_ReorderError_Presentation {
	};
	struct mux_container_ReorderError_Tie {
	};
	struct mux_container_ReorderError_Undrained {
	};
	struct mux_import_Format_Audio {
		::moq::mux_import_AudioFormat _0;
	};
	struct mux_import_Format_Video {
		::moq::mux_import_VideoFormat _0;
	};
	struct mux_import_Format_Container {
		::moq::mux_import_ContainerFormat _0;
	};
	struct mux_rate_Target_Enabled {
		::moq::bandwidth_Rate _0;
	};
	struct mux_rate_Target_Disabled {
	};
	struct mux_catalog_msf_Error_UnresolvableNamespace {
		::moq::buf name;
		::moq::buf namespace_;
	};
	struct mux_catalog_msf_Error_InvalidUtf8 {
	};
	struct mux_catalog_msf_Error_ParseFrame {
	};
	struct mux_catalog_msf_Error_MissingCmafInit {
		::moq::buf _0;
	};
	struct mux_catalog_msf_Error_MalformedInitData {
		::moq::buf _0;
	};
	struct mux_catalog_msf_Error_MissingVideoCodec {
		::moq::buf _0;
	};
	struct mux_catalog_msf_Error_MissingAudioCodec {
		::moq::buf _0;
	};
	struct mux_catalog_msf_Error_UnsupportedPackaging {
		::moq::buf name;
		::moq::buf packaging;
	};
	struct mux_catalog_msf_Error_MissingTextCodec {
		::moq::buf _0;
	};
	struct mux_catalog_msf_Error_InvalidVideoCodec {
		::moq::buf name;
		::moq::buf codec;
	};
	struct mux_catalog_msf_Error_InvalidAudioCodec {
		::moq::buf name;
		::moq::buf codec;
	};
	struct mux_catalog_msf_Error_MissingAudioParams {
		::moq::buf _0;
	};
	struct mux_catalog_msf_Error_UnsupportedDerivationPackaging {
		::moq::buf name;
		::moq::buf packaging;
	};
	struct mux_catalog_msf_Error_MalformedAac {
		::moq::buf _0;
	};
	struct mux_catalog_msf_Error_MalformedOpus {
		::moq::buf _0;
	};
	struct mux_catalog_msf_Error_MalformedFlac {
		::moq::buf _0;
	};
	struct mux_catalog_msf_Error_OpusTrailingBytes {
		::moq::buf _0;
	};
	struct mux_catalog_msf_Error_UnsupportedDerivationCodec {
		::moq::buf _0;
	};
	struct mux_catalog_msf_Error_MalformedInitSegment {
		::moq::buf _0;
	};
	struct mux_catalog_msf_Error_MissingInitMoov {
		::moq::buf _0;
	};
	struct mux_catalog_msf_Error_MissingAudioSampleEntry {
		::moq::buf _0;
	};
	struct mux_codec_opus_Error_HeadTooShort {
	};
	struct mux_codec_opus_Error_InvalidSignature {
	};
	struct mux_codec_opus_Error_UnsupportedVersion {
		::std::uint8_t _0;
	};
	struct mux_codec_opus_Error_UnsupportedChannelCount {
		::std::uint32_t _0;
	};
	struct mux_codec_opus_Error_MappingTableTooShort {
	};
	struct mux_codec_opus_Error_InvalidMappingTable {
	};
	struct mux_codec_opus_Error_UnsupportedMappingFamily {
		::std::uint8_t _0;
	};
	struct mux_container_fmp4_Chunk_Init {
		::moq::buf _0;
	};
	struct mux_container_fmp4_Chunk_Fragment {
		::moq::mux_container_fmp4_Fragment _0;
	};
	struct mux_container_ts_Step_Rewind {
		::moq::duration by;
		bool flagged;
	};
	struct mux_container_ts_Step_Leap {
		::moq::duration by;
	};
	struct mux_container_ts_Step_Identity {
		::std::uint16_t transport_stream_id;
		::std::uint16_t program_number;
	};
	struct mux_codec_aac_adts_Error_Truncated {
	};
	struct mux_codec_aac_adts_Error_Syncword {
	};
	struct mux_codec_aac_adts_Error_SampleRate {
	};
	struct mux_codec_aac_adts_Error_FrameLength {
	};
	struct mux_codec_aac_adts_Error_PacketLength {
	};
	struct mux_codec_aac_adts_Error_ObjectType {
		::std::uint8_t _0;
	};
	struct mux_codec_aac_adts_Error_Rate {
		::std::uint32_t _0;
	};
	struct mux_codec_aac_adts_Error_ChannelConfig {
		::std::uint8_t _0;
	};
	struct mux_codec_aac_adts_Error_TooLarge {
		::std::size_t _0;
	};
	inline bool operator==(const mux_container_TimestampRewind &a, const mux_container_TimestampRewind &b);
	inline bool operator==(const mux_container_Playout &a, const mux_container_Playout &b);
	inline bool operator==(const mux_codec_flac_Config &a, const mux_codec_flac_Config &b);
	inline bool operator==(const mux_codec_h265_VideoSignal &a, const mux_codec_h265_VideoSignal &b);
	inline bool operator==(const mux_codec_opus_Mapping &a, const mux_codec_opus_Mapping &b);
	inline bool operator==(const mux_container_ts_Descriptor &a, const mux_container_ts_Descriptor &b);
	inline bool operator==(::moq::mux_catalog_CatalogFormat a, ::moq::mux_catalog_CatalogFormat b);
	inline bool operator!=(::moq::mux_catalog_CatalogFormat a, ::moq::mux_catalog_CatalogFormat b);
	inline ::moq::buf mux_catalog_CatalogFormat_debug(::moq::mux_catalog_CatalogFormat self);
	inline mux_catalog_CatalogFormat mux_catalog_CatalogFormat_default();
	inline ::moq::mux_catalog_CatalogFormat mux_catalog_CatalogFormat_DEFAULT();
	inline ::moq::buf mux_catalog_CatalogFormat_extension(::moq::mux_catalog_CatalogFormat self);
	inline ::std::optional<::moq::mux_catalog_CatalogFormat> mux_catalog_CatalogFormat_detect(::moq::input name);
	inline bool operator==(::moq::mux_container_Kind a, ::moq::mux_container_Kind b);
	inline bool operator!=(::moq::mux_container_Kind a, ::moq::mux_container_Kind b);
	inline ::moq::buf mux_container_Kind_debug(::moq::mux_container_Kind self);
	inline ::moq::buf mux_container_Read_debug(const ::moq::mux_container_Read &self);
	inline bool operator==(const ::moq::mux_container_ReorderError &a, const ::moq::mux_container_ReorderError &b);
	inline bool operator!=(const ::moq::mux_container_ReorderError &a, const ::moq::mux_container_ReorderError &b);
	inline ::moq::buf mux_container_ReorderError_to_string(const ::moq::mux_container_ReorderError &self);
	inline ::moq::buf mux_container_ReorderError_debug(const ::moq::mux_container_ReorderError &self);
	inline bool operator==(::moq::mux_import_AudioFormat a, ::moq::mux_import_AudioFormat b);
	inline bool operator!=(::moq::mux_import_AudioFormat a, ::moq::mux_import_AudioFormat b);
	inline ::moq::buf mux_import_AudioFormat_to_string(::moq::mux_import_AudioFormat self);
	inline ::moq::buf mux_import_AudioFormat_debug(::moq::mux_import_AudioFormat self);
	inline bool operator==(::moq::mux_import_ContainerFormat a, ::moq::mux_import_ContainerFormat b);
	inline bool operator!=(::moq::mux_import_ContainerFormat a, ::moq::mux_import_ContainerFormat b);
	inline ::moq::buf mux_import_ContainerFormat_to_string(::moq::mux_import_ContainerFormat self);
	inline ::moq::buf mux_import_ContainerFormat_debug(::moq::mux_import_ContainerFormat self);
	inline bool operator==(const ::moq::mux_import_Format &a, const ::moq::mux_import_Format &b);
	inline bool operator!=(const ::moq::mux_import_Format &a, const ::moq::mux_import_Format &b);
	inline ::moq::buf mux_import_Format_to_string(const ::moq::mux_import_Format &self);
	inline ::moq::buf mux_import_Format_debug(const ::moq::mux_import_Format &self);
	inline ::moq::mux_import_Kind mux_import_Format_kind(const ::moq::mux_import_Format &self);
	inline bool operator==(::moq::mux_import_Kind a, ::moq::mux_import_Kind b);
	inline bool operator!=(::moq::mux_import_Kind a, ::moq::mux_import_Kind b);
	inline ::moq::buf mux_import_Kind_debug(::moq::mux_import_Kind self);
	inline ::moq::buf mux_import_Kind_name(::moq::mux_import_Kind self);
	inline bool operator==(::moq::mux_import_VideoFormat a, ::moq::mux_import_VideoFormat b);
	inline bool operator!=(::moq::mux_import_VideoFormat a, ::moq::mux_import_VideoFormat b);
	inline ::moq::buf mux_import_VideoFormat_to_string(::moq::mux_import_VideoFormat self);
	inline ::moq::buf mux_import_VideoFormat_debug(::moq::mux_import_VideoFormat self);
	inline bool operator==(const ::moq::mux_rate_Target &a, const ::moq::mux_rate_Target &b);
	inline bool operator!=(const ::moq::mux_rate_Target &a, const ::moq::mux_rate_Target &b);
	inline ::moq::buf mux_rate_Target_debug(const ::moq::mux_rate_Target &self);
	inline ::moq::buf mux_catalog_msf_Error_to_string(const ::moq::mux_catalog_msf_Error &self);
	inline ::moq::buf mux_catalog_msf_Error_debug(const ::moq::mux_catalog_msf_Error &self);
	inline ::moq::buf mux_codec_flac_Error_to_string(::moq::mux_codec_flac_Error self);
	inline ::moq::buf mux_codec_flac_Error_debug(::moq::mux_codec_flac_Error self);
	inline ::moq::buf mux_codec_mp3_Error_to_string(::moq::mux_codec_mp3_Error self);
	inline ::moq::buf mux_codec_mp3_Error_debug(::moq::mux_codec_mp3_Error self);
	inline ::moq::buf mux_codec_opus_Error_to_string(const ::moq::mux_codec_opus_Error &self);
	inline ::moq::buf mux_codec_opus_Error_debug(const ::moq::mux_codec_opus_Error &self);
	inline ::moq::buf mux_codec_vp8_Error_to_string(::moq::mux_codec_vp8_Error self);
	inline ::moq::buf mux_codec_vp8_Error_debug(::moq::mux_codec_vp8_Error self);
	inline ::moq::buf mux_codec_vp9_Error_to_string(::moq::mux_codec_vp9_Error self);
	inline ::moq::buf mux_codec_vp9_Error_debug(::moq::mux_codec_vp9_Error self);
	inline ::moq::buf mux_container_fmp4_Chunk_debug(const ::moq::mux_container_fmp4_Chunk &self);
	inline ::moq::buf mux_container_fmp4_Chunk_data(const ::moq::mux_container_fmp4_Chunk &self);
	inline ::moq::buf mux_container_fmp4_Chunk_into_data(const ::moq::mux_container_fmp4_Chunk &self);
	inline ::std::optional<::moq::buf> mux_container_fmp4_Chunk_init(const ::moq::mux_container_fmp4_Chunk &self);
	inline ::std::optional<::moq::mux_container_fmp4_Fragment> mux_container_fmp4_Chunk_fragment(const ::moq::mux_container_fmp4_Chunk &self);
	inline bool operator==(::moq::mux_container_ts_Framing a, ::moq::mux_container_ts_Framing b);
	inline bool operator!=(::moq::mux_container_ts_Framing a, ::moq::mux_container_ts_Framing b);
	inline ::moq::buf mux_container_ts_Framing_debug(::moq::mux_container_ts_Framing self);
	inline mux_container_ts_Framing mux_container_ts_Framing_default();
	inline bool operator==(const ::moq::mux_container_ts_Step &a, const ::moq::mux_container_ts_Step &b);
	inline bool operator!=(const ::moq::mux_container_ts_Step &a, const ::moq::mux_container_ts_Step &b);
	inline ::moq::buf mux_container_ts_Step_to_string(const ::moq::mux_container_ts_Step &self);
	inline ::moq::buf mux_container_ts_Step_debug(const ::moq::mux_container_ts_Step &self);
	inline bool operator==(const ::moq::mux_codec_aac_adts_Error &a, const ::moq::mux_codec_aac_adts_Error &b);
	inline bool operator!=(const ::moq::mux_codec_aac_adts_Error &a, const ::moq::mux_codec_aac_adts_Error &b);
	inline ::moq::buf mux_codec_aac_adts_Error_to_string(const ::moq::mux_codec_aac_adts_Error &self);
	inline ::moq::buf mux_codec_aac_adts_Error_debug(const ::moq::mux_codec_aac_adts_Error &self);
	inline bool operator==(::moq::mux_container_ts_stats_Class a, ::moq::mux_container_ts_stats_Class b);
	inline bool operator!=(::moq::mux_container_ts_stats_Class a, ::moq::mux_container_ts_stats_Class b);
	inline ::moq::buf mux_container_ts_stats_Class_debug(::moq::mux_container_ts_stats_Class self);
	inline mux_container_ts_stats_Class mux_container_ts_stats_Class_default();
	inline ::moq::duration mux_container_DELAY_MAX();
	inline ::moq::duration mux_rate_RETRY();
	inline ::moq::result<::moq::hang_catalog_AudioConfig, ::moq::mux_Error> mux_codec_aac_config(::moq::input init);
	inline ::moq::buf mux_codec_annexb_START_CODE();
	inline ::moq::result<::moq::buf, ::moq::mux_codec_annexb_Error> mux_codec_annexb_from_length_prefixed(::moq::input payload, ::std::size_t length_size, ::std::optional<::moq::input> prefix);
	inline ::moq::buf mux_codec_annexb_build_prefix(const ::moq::items<::moq::view> &nals);
	inline ::moq::result<void, ::moq::mux_codec_annexb_Error> mux_codec_annexb_length_prefixed_to_annexb(::moq::input data, ::std::size_t length_size, ::moq::buf &out);
	inline ::moq::result<::std::optional<::std::size_t>, ::moq::mux_codec_annexb_Error> mux_codec_annexb_after_start_code(::moq::input b);
	inline ::std::optional<::std::tuple<::std::size_t, ::std::size_t>> mux_codec_annexb_find_start_code(::moq::input b);
	inline ::moq::result<::moq::hang_catalog_AudioConfig, ::moq::mux_Error> mux_codec_flac_config(::moq::input init);
	inline ::moq::result<::moq::hang_catalog_VideoConfig, ::moq::mux_Error> mux_codec_h264_config(::moq::input buf_);
	inline ::moq::result<::moq::hang_catalog_VideoConfig, ::moq::mux_Error> mux_codec_h265_config(::moq::input buf_);
	inline ::moq::result<::moq::hang_catalog_AudioConfig, ::moq::mux_Error> mux_codec_mp3_config(::moq::input init);
	inline ::moq::duration mux_codec_opus_WARMUP();
	inline ::std::optional<::std::uint32_t> mux_codec_opus_packet_samples(::moq::input packet);
	inline ::moq::result<::moq::hang_catalog_AudioConfig, ::moq::mux_Error> mux_codec_opus_config(::moq::input init);
	template <class F0> inline ::moq::result<::std::optional<::moq::mux_container_mp4_Finish>, ::moq::mux_Error> mux_container_mp4_repair(F0 &file);
}
#endif

// net.hpp
#ifndef MOQ_HPP_CRATE5
#define MOQ_HPP_CRATE5
#include "rt.hpp"
#include "kio.hpp"
#include "pattern.hpp"
#include "url.hpp"
namespace moq {
	enum class EncodeError {
		bounds_exceeded = 0,
		too_large = 1,
		short_ = 2,
		invalid_state = 3,
		too_many = 4,
		version = 5,
		unsupported = 6,
	};
	enum class Retry {
		same = 0,
		reroute = 1,
		never = 2,
	};
	enum class tokio_Restart {
		redial = 0,
		handoff = 1,
	};
	enum class path_Error {
		too_long = 0,
		empty_segment = 1,
		invalid_escape = 2,
	};
	enum class setup_Role {
		publisher = 0,
		subscriber = 1,
	};
	enum class reconnect_Verdict {
		retry = 0,
		unusable = 1,
		settled = 2,
	};
	enum class reconnect_ConfigError {
		empty = 0,
		healthy = 1,
	};
	enum class reconnect_Redirect {
		follow = 0,
		same_host = 1,
		ignore = 2,
	};
	enum class stats_Role {
		publisher = 0,
		subscriber = 1,
	};
	enum class stats_Message {
		session = 0,
		announce = 1,
		track_info = 2,
		subscribe = 3,
		fetch = 4,
		probe = 5,
		auth = 6,
		goaway = 7,
		other = 8,
	};
	enum class stats_Failure {
		idle_timeout = 0,
		transport = 1,
		application = 2,
		expired = 3,
	};
	enum class stats_RefusalKind {
		refused = 0,
		unavailable = 1,
		request = 2,
		forbidden = 3,
		membership = 4,
		expired = 5,
	};
	enum class stats_RouteClose {
		stalled = 0,
		overrun = 1,
		malformed = 2,
		role = 3,
		unasked = 4,
		regressed = 5,
		other = 6,
	};
	enum class stats_RouteRefusal {
		path = 0,
		scope = 1,
		looped = 2,
	};
	enum class stats_Withdrawal {
		dead = 0,
		drain = 1,
	};
	enum class transport_Address {
		uri = 0,
		setup = 1,
	};
	enum class transport_CloseKind {
		transport = 0,
		idle_timeout = 1,
		closed = 2,
		evicted = 3,
	};
	enum class broadcast_Kind {
		track = 0,
		datagrams = 1,
	};
	enum class track_Compression {
		none = 0,
		deflate = 1,
	};
	enum class track_Order {
		ascending = 0,
		descending = 1,
	};
	enum class origin_Link {
		local = 0,
		peer = 1,
		upstream = 2,
	};
	enum class origin_RouteKind {
		start = 0,
		update = 1,
		end = 2,
		offline = 3,
		restart = 4,
	};
	enum class transport_poll_Acked {
		delivered = 0,
		unsupported = 1,
		sent = 2,
	};
	inline constexpr ::std::size_t Path_MAX_PARTS = MOQ_PATH_MAX_PARTS;
	inline constexpr ::std::size_t Path_MAX_BYTES = MOQ_PATH_MAX_BYTES;
	inline constexpr ::std::uint64_t setup_Token_OUT_OF_BAND = MOQ_SETUP_TOKEN_OUT_OF_BAND;
	inline constexpr ::std::uint64_t transport_MIN_RECEIVE_WINDOW = MOQ_TRANSPORT_MIN_RECEIVE_WINDOW;
	inline constexpr ::std::uint64_t transport_SETUP_RECEIVE_WINDOW = MOQ_TRANSPORT_SETUP_RECEIVE_WINDOW;
	class RUST2_FFI_CONSUMABLE DecodeError {
		RUST2_FFI_UNCONSUMED DecodeError(DecodeError &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		DecodeError &operator=(DecodeError &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED DecodeError(const DecodeError &other) = default;
		DecodeError &operator=(const DecodeError &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE Driver {
		RUST2_FFI_UNCONSUMED Driver(Driver &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Driver &operator=(Driver &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::result<::std::optional<::moq::clock_Instant>, ::moq::Error> poll(const ::moq::clock_Instant &now, const ::moq::Waiter &waiter);
	};
	class RUST2_FFI_CONSUMABLE Instance {
		RUST2_FFI_UNCONSUMED Instance(Instance &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Instance &operator=(Instance &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Instance(const Instance &other) = default;
		Instance &operator=(const Instance &other) = default;
		friend bool operator==(const Instance &a, const Instance &b);
		friend bool operator!=(const Instance &a, const Instance &b);
		friend bool operator<(const Instance &a, const Instance &b);
		friend bool operator<=(const Instance &a, const Instance &b);
		friend bool operator>(const Instance &a, const Instance &b);
		friend bool operator>=(const Instance &a, const Instance &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const Instance &a, const Instance &b);
		#endif
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::Instance random();
		RUST2_FFI_CALLABLE ::moq::buf as_str() const;
		RUST2_FFI_CALLABLE bool is_minted() const;
	};
	class RUST2_FFI_CONSUMABLE Path {
		RUST2_FFI_UNCONSUMED Path(Path &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Path &operator=(Path &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Path(const Path &other) = default;
		Path &operator=(const Path &other) = default;
		friend bool operator==(const Path &a, const Path &b);
		friend bool operator!=(const Path &a, const Path &b);
		friend bool operator<(const Path &a, const Path &b);
		friend bool operator<=(const Path &a, const Path &b);
		friend bool operator>(const Path &a, const Path &b);
		friend bool operator>=(const Path &a, const Path &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const Path &a, const Path &b);
		#endif
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static Path default_();
		static ::moq::result<::moq::Path, ::moq::path_Error> new_(::moq::input s);
		static ::moq::Path from_static(::moq::input s);
		RUST2_FFI_CALLABLE bool has_prefix(const ::moq::Path &prefix) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Path> strip_prefix(const ::moq::Path &prefix) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Path> head(::std::size_t n) const;
		RUST2_FFI_CALLABLE ::moq::buf as_str() const;
		static ::moq::Path empty();
		RUST2_FFI_CALLABLE bool is_empty() const;
		RUST2_FFI_CALLABLE ::std::size_t len() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Path, ::moq::path_Error> join(const ::moq::Path &other) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Path, ::moq::path_Error> resolve(const ::moq::path_Relative &rel) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Path> try_resolve(const ::moq::path_Relative &rel) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::path_Relative> relative(const ::moq::Path &base) const;
	};
	class RUST2_FFI_CONSUMABLE Server {
		RUST2_FFI_UNCONSUMED Server(Server &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Server &operator=(Server &&other) noexcept = default;
		RUST2_FFI_CALLABLE Server clone() const;
		static Server default_();
		static ::moq::Server new_();
		RUST2_FFI_CONSUMES ::moq::Server with_publisher_Producer(const ::moq::origin_Producer &publish) &&;
		RUST2_FFI_CONSUMES ::moq::Server with_publisher_Consumer(const ::moq::origin_Consumer &publish) &&;
		RUST2_FFI_CONSUMES ::moq::Server with_subscriber(::moq::origin_Producer subscribe) &&;
		RUST2_FFI_CONSUMES ::moq::Server with_allocator(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CONSUMES ::moq::Server with_stats(::moq::stats_Session stats) &&;
		RUST2_FFI_CONSUMES ::moq::Server with_limits(const ::moq::session_Limits &limits) &&;
		RUST2_FFI_CONSUMES ::moq::Server with_versions(::moq::Versions versions) &&;
		RUST2_FFI_CONSUMES ::moq::Server with_implementation(::moq::input implementation) &&;
		RUST2_FFI_CONSUMES ::moq::Server with_timeout(const ::std::optional<::moq::duration> &timeout) &&;
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> deadline(const ::moq::clock_Instant &arrived) const;
		RUST2_FFI_CONSUMES ::moq::Server with_receive_window(::std::uint64_t window) &&;
		RUST2_FFI_CALLABLE ::moq::Server_accept_memory_Session_future accept_memory_Session(const ::moq::clock_Instant &now, ::moq::transport_memory_Session session) const;
		RUST2_FFI_CALLABLE ::moq::Server_accept_qmux_Session_future accept_qmux_Session(const ::moq::clock_Instant &now, ::moq::qmux_Session session) const;
		RUST2_FFI_CALLABLE ::moq::Server_handshake_memory_Session_future handshake_memory_Session(const ::moq::clock_Instant &now, ::moq::transport_memory_Session session) const;
		RUST2_FFI_CALLABLE ::moq::Server_handshake_qmux_Session_future handshake_qmux_Session(const ::moq::clock_Instant &now, ::moq::qmux_Session session) const;
	};
	class RUST2_FFI_CONSUMABLE Session {
		RUST2_FFI_UNCONSUMED Session(Session &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Session &operator=(Session &&other) noexcept = default;
		RUST2_FFI_CALLABLE Session clone() const;
		friend bool operator==(const Session &a, const Session &b);
		friend bool operator!=(const Session &a, const Session &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::Version version() const;
		RUST2_FFI_CALLABLE ::moq::Session_peer_implementation_future peer_implementation() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::bandwidth_Consumer> send_bandwidth() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Allocator allocator() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::bandwidth_Consumer> recv_bandwidth() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Viewers viewers() const;
		RUST2_FFI_CALLABLE ::moq::session_Stats stats() const;
		RUST2_FFI_CALLABLE void abort(::moq::Error err) const;
		RUST2_FFI_CONSUMES ::moq::Session_close_future close() &&;
		RUST2_FFI_CALLABLE ::moq::Session_closed_future closed() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> goaway(::moq::goaway_Goaway goaway) const;
		RUST2_FFI_CALLABLE ::moq::goaway_Consumer draining() const;
		RUST2_FFI_CALLABLE ::moq::auth_Handle auth() const;
	};
	class RUST2_FFI_CONSUMABLE Client {
		RUST2_FFI_UNCONSUMED Client(Client &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Client &operator=(Client &&other) noexcept = default;
		RUST2_FFI_CALLABLE Client clone() const;
		static Client default_();
		static ::moq::Client new_();
		RUST2_FFI_CONSUMES ::moq::Client with_publisher_Producer(const ::moq::origin_Producer &publish) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_publisher_Consumer(const ::moq::origin_Consumer &publish) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_subscriber(::moq::origin_Producer subscribe) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_stats(::moq::stats_Session stats) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_limits(const ::moq::session_Limits &limits) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_allocator(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_viewers(::moq::bandwidth_Viewers viewers) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_versions(::moq::Versions versions) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_url(::moq::Url url) &&;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Url> url() const;
		RUST2_FFI_CONSUMES ::moq::Client with_token(const ::moq::setup_Token &token) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_auth(::moq::auth_Handle auth) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_implementation(::moq::input implementation) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_cost(const ::moq::origin_Cost &cost) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_peer_node(::std::uint64_t node) &&;
		RUST2_FFI_CALLABLE ::moq::Client_connect_memory_Session_future connect_memory_Session(const ::moq::clock_Instant &now, ::moq::transport_memory_Session session) const;
		RUST2_FFI_CALLABLE ::moq::Client_connect_qmux_Session_future connect_qmux_Session(const ::moq::clock_Instant &now, ::moq::qmux_Session session) const;
	};
	class RUST2_FFI_CONSUMABLE Error {
		RUST2_FFI_UNCONSUMED Error(Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Error &operator=(Error &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Error(const Error &other) = default;
		Error &operator=(const Error &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::SessionError> session() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::StreamError> stream() const;
		static ::moq::Error from_transport_Error(::moq::Error err);
		static ::moq::Error from_transport_memory_Error(const ::moq::transport_memory_Error &err);
		RUST2_FFI_CALLABLE ::moq::Retry retry() const;
	};
	class RUST2_FFI_CONSUMABLE Version {
		RUST2_FFI_UNCONSUMED Version(Version &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Version &operator=(Version &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Version(const Version &other) = default;
		Version &operator=(const Version &other) = default;
		friend bool operator==(const Version &a, const Version &b);
		friend bool operator!=(const Version &a, const Version &b);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::std::vector<::moq::Version> ALL();
		RUST2_FFI_CALLABLE bool has_request_path() const;
		RUST2_FFI_CALLABLE ::moq::buf as_str() const;
		RUST2_FFI_CALLABLE ::moq::buf alpn() const;
		RUST2_FFI_CALLABLE bool is_lite() const;
	};
	class RUST2_FFI_CONSUMABLE Versions {
		RUST2_FFI_UNCONSUMED Versions(Versions &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Versions &operator=(Versions &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Versions(const Versions &other) = default;
		Versions &operator=(const Versions &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static Versions default_();
		static ::moq::Versions all();
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> alpns() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Versions> filter(const ::moq::Versions &other) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Version> select(const ::moq::Version &version) const;
		RUST2_FFI_CALLABLE bool contains(const ::moq::Version &version) const;
		RUST2_FFI_CALLABLE ::std::vector<::moq::Version> iter() const;
	};
	class RUST2_FFI_CONSUMABLE auth_Watch {
		RUST2_FFI_UNCONSUMED auth_Watch(auth_Watch &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		auth_Watch &operator=(auth_Watch &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::std::optional<::moq::auth_Grant> peek() const;
		RUST2_FFI_CALLABLE ::moq::auth_Watch_changed_future changed();
	};
	class RUST2_FFI_CONSUMABLE auth_Requests {
		RUST2_FFI_UNCONSUMED auth_Requests(auth_Requests &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		auth_Requests &operator=(auth_Requests &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::auth_Requests_next_future next();
	};
	class RUST2_FFI_CONSUMABLE auth_Request {
		RUST2_FFI_UNCONSUMED auth_Request(auth_Request &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		auth_Request &operator=(auth_Request &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf token() const;
		RUST2_FFI_CALLABLE ::moq::auth_Request_closed_future closed() const;
		RUST2_FFI_CONSUMES ::moq::auth_Issued accept(::moq::auth_Grant grant) &&;
		RUST2_FFI_CONSUMES void reject(const ::moq::SessionError &code, ::moq::input reason) &&;
	};
	class RUST2_FFI_CONSUMABLE auth_Issued {
		RUST2_FFI_UNCONSUMED auth_Issued(auth_Issued &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		auth_Issued &operator=(auth_Issued &&other) noexcept = default;
		RUST2_FFI_CALLABLE void update(::moq::auth_Grant grant) const;
		RUST2_FFI_CONSUMES void revoke(const ::moq::SessionError &code, ::moq::input reason) &&;
		RUST2_FFI_CALLABLE ::moq::auth_Issued_closed_future closed() const;
	};
	class RUST2_FFI_CONSUMABLE auth_Grant {
		RUST2_FFI_UNCONSUMED auth_Grant(auth_Grant &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		auth_Grant &operator=(auth_Grant &&other) noexcept = default;
		RUST2_FFI_CALLABLE auth_Grant clone() const;
		friend bool operator==(const auth_Grant &a, const auth_Grant &b);
		friend bool operator!=(const auth_Grant &a, const auth_Grant &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static auth_Grant default_();
		RUST2_FFI_CALLABLE ::moq::Patterns publish_get() const;
		RUST2_FFI_CALLABLE void publish_set(::moq::Patterns value);
		RUST2_FFI_CALLABLE ::moq::Patterns subscribe_get() const;
		RUST2_FFI_CALLABLE void subscribe_set(::moq::Patterns value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> expires_get() const;
		RUST2_FFI_CALLABLE void expires_set(const ::std::optional<::moq::clock_Instant> &value);
		static ::moq::auth_Grant all();
	};
	class RUST2_FFI_CONSUMABLE auth_Handle {
		RUST2_FFI_UNCONSUMED auth_Handle(auth_Handle &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		auth_Handle &operator=(auth_Handle &&other) noexcept = default;
		RUST2_FFI_CALLABLE auth_Handle clone() const;
		static auth_Handle default_();
		static ::moq::auth_Handle new_();
		RUST2_FFI_CALLABLE ::moq::auth_Watch grant() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::auth_Credential, ::moq::Error> add(::moq::input token) const;
		RUST2_FFI_CALLABLE void authorize(const ::moq::auth_Grant &grant) const;
		RUST2_FFI_CALLABLE void abort(::moq::Error err) const;
	};
	class RUST2_FFI_CONSUMABLE auth_Credential {
		RUST2_FFI_UNCONSUMED auth_Credential(auth_Credential &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		auth_Credential &operator=(auth_Credential &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED auth_Credential(const auth_Credential &other) = default;
		auth_Credential &operator=(const auth_Credential &other) = default;
		friend bool operator==(const auth_Credential &a, const auth_Credential &b);
		friend bool operator!=(const auth_Credential &a, const auth_Credential &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::auth_Watch grant() const;
		RUST2_FFI_CALLABLE ::moq::auth_Credential_closed_future closed() const;
		RUST2_FFI_CALLABLE ::moq::auth_Credential_answered_future answered() const;
	};
	class RUST2_FFI_CONSUMABLE auth_Token {
		RUST2_FFI_UNCONSUMED auth_Token(auth_Token &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		auth_Token &operator=(auth_Token &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED auth_Token(const auth_Token &other) = default;
		auth_Token &operator=(const auth_Token &other) = default;
		friend bool operator==(const auth_Token &a, const auth_Token &b);
		friend bool operator!=(const auth_Token &a, const auth_Token &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::buf as_str() const;
	};
	class RUST2_FFI_CONSUMABLE auth_Tokens {
		RUST2_FFI_UNCONSUMED auth_Tokens(auth_Tokens &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		auth_Tokens &operator=(auth_Tokens &&other) noexcept = default;
		RUST2_FFI_CALLABLE auth_Tokens clone() const;
		static ::moq::auth_Tokens new_(::std::vector<::moq::auth_Token> tokens);
		RUST2_FFI_CALLABLE ::moq::auth_Handle handle() const;
		RUST2_FFI_CALLABLE void set(::std::vector<::moq::auth_Token> tokens) const;
		RUST2_FFI_CALLABLE ::moq::auth_Tokens_answered_future answered() const;
	};
	class RUST2_FFI_CONSUMABLE goaway_Goaway {
		RUST2_FFI_UNCONSUMED goaway_Goaway(goaway_Goaway &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		goaway_Goaway &operator=(goaway_Goaway &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED goaway_Goaway(const goaway_Goaway &other) = default;
		goaway_Goaway &operator=(const goaway_Goaway &other) = default;
		friend bool operator==(const goaway_Goaway &a, const goaway_Goaway &b);
		friend bool operator!=(const goaway_Goaway &a, const goaway_Goaway &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static goaway_Goaway default_();
		static ::moq::goaway_Goaway new_();
		static ::moq::goaway_Goaway redirect(::moq::input uri);
		RUST2_FFI_CONSUMES ::moq::goaway_Goaway with_timeout(const ::std::optional<::moq::duration> &timeout) &&;
		RUST2_FFI_CALLABLE ::moq::buf uri() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> timeout() const;
	};
	class RUST2_FFI_CONSUMABLE goaway_Consumer {
		RUST2_FFI_UNCONSUMED goaway_Consumer(goaway_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		goaway_Consumer &operator=(goaway_Consumer &&other) noexcept = default;
		RUST2_FFI_CALLABLE goaway_Consumer clone() const;
		RUST2_FFI_CALLABLE ::moq::goaway_Consumer_poll_future poll() const;
	};
	class RUST2_FFI_CONSUMABLE path_Relative {
		RUST2_FFI_UNCONSUMED path_Relative(path_Relative &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		path_Relative &operator=(path_Relative &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED path_Relative(const path_Relative &other) = default;
		path_Relative &operator=(const path_Relative &other) = default;
		friend bool operator==(const path_Relative &a, const path_Relative &b);
		friend bool operator!=(const path_Relative &a, const path_Relative &b);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static path_Relative default_();
		static ::moq::result<::moq::path_Relative, ::moq::path_Error> new_(::moq::input s);
		RUST2_FFI_CALLABLE ::moq::buf as_str() const;
		static ::moq::path_Relative empty();
		RUST2_FFI_CALLABLE bool is_empty() const;
		RUST2_FFI_CALLABLE ::std::size_t len() const;
	};
	class RUST2_FFI_CONSUMABLE reconnect_Backoff {
		RUST2_FFI_UNCONSUMED reconnect_Backoff(reconnect_Backoff &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		reconnect_Backoff &operator=(reconnect_Backoff &&other) noexcept = default;
		RUST2_FFI_CALLABLE reconnect_Backoff clone() const;
		friend bool operator==(const reconnect_Backoff &a, const reconnect_Backoff &b);
		friend bool operator!=(const reconnect_Backoff &a, const reconnect_Backoff &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static reconnect_Backoff default_();
		RUST2_FFI_CALLABLE ::moq::duration initial_get() const;
		RUST2_FFI_CALLABLE void initial_set(::moq::duration value);
		RUST2_FFI_CALLABLE ::std::uint32_t multiplier_get() const;
		RUST2_FFI_CALLABLE void multiplier_set(::std::uint32_t value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> max_get() const;
		RUST2_FFI_CALLABLE void max_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::moq::duration healthy_get() const;
		RUST2_FFI_CALLABLE void healthy_set(::moq::duration value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> timeout_get() const;
		RUST2_FFI_CALLABLE void timeout_set(const ::std::optional<::moq::duration> &value);
	};
	class RUST2_FFI_CONSUMABLE reconnect_Handover {
		RUST2_FFI_UNCONSUMED reconnect_Handover(reconnect_Handover &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		reconnect_Handover &operator=(reconnect_Handover &&other) noexcept = default;
		RUST2_FFI_CALLABLE reconnect_Handover clone() const;
		friend bool operator==(const reconnect_Handover &a, const reconnect_Handover &b);
		friend bool operator!=(const reconnect_Handover &a, const reconnect_Handover &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static reconnect_Handover default_();
		RUST2_FFI_CALLABLE ::moq::reconnect_Redirect redirect_get() const;
		RUST2_FFI_CALLABLE void redirect_set(::moq::reconnect_Redirect value);
		RUST2_FFI_CALLABLE ::moq::duration handover_get() const;
		RUST2_FFI_CALLABLE void handover_set(::moq::duration value);
	};
	class RUST2_FFI_CONSUMABLE reconnect_Config {
		RUST2_FFI_UNCONSUMED reconnect_Config(reconnect_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		reconnect_Config &operator=(reconnect_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE reconnect_Config clone() const;
		friend bool operator==(const reconnect_Config &a, const reconnect_Config &b);
		friend bool operator!=(const reconnect_Config &a, const reconnect_Config &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static reconnect_Config default_();
		RUST2_FFI_CALLABLE ::moq::reconnect_Backoff backoff_get() const;
		RUST2_FFI_CALLABLE void backoff_set(const ::moq::reconnect_Backoff &value);
		RUST2_FFI_CALLABLE ::moq::reconnect_Handover goaway_get() const;
		RUST2_FFI_CALLABLE void goaway_set(const ::moq::reconnect_Handover &value);
		RUST2_FFI_CALLABLE ::moq::tokio_Restart restart_get() const;
		RUST2_FFI_CALLABLE void restart_set(::moq::tokio_Restart value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> timeout_get() const;
		RUST2_FFI_CALLABLE void timeout_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE bool once_get() const;
		RUST2_FFI_CALLABLE void once_set(bool value);
		RUST2_FFI_CALLABLE bool pinned_get() const;
		RUST2_FFI_CALLABLE void pinned_set(bool value);
	};
	class RUST2_FFI_CONSUMABLE server_Handshake {
		RUST2_FFI_UNCONSUMED server_Handshake(server_Handshake &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		server_Handshake &operator=(server_Handshake &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf path() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::setup_Role> role() const;
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> peer_node() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> peer_implementation() const;
		RUST2_FFI_CALLABLE ::moq::auth_Handle auth() const;
		RUST2_FFI_CALLABLE ::moq::auth_Requests requests();
		RUST2_FFI_CALLABLE ::std::optional<::moq::setup_Token> token() const;
		RUST2_FFI_CONSUMES ::moq::server_Handshake with_publisher_Producer(const ::moq::origin_Producer &publish) &&;
		RUST2_FFI_CONSUMES ::moq::server_Handshake with_publisher_Consumer(const ::moq::origin_Consumer &publish) &&;
		RUST2_FFI_CONSUMES ::moq::server_Handshake with_subscriber(::moq::origin_Producer subscribe) &&;
		RUST2_FFI_CONSUMES ::moq::server_Handshake with_peer_node(::std::uint64_t node) &&;
		RUST2_FFI_CONSUMES ::moq::server_Handshake with_stats(::moq::stats_Session stats) &&;
		RUST2_FFI_CONSUMES ::moq::server_Handshake with_limits(const ::moq::session_Limits &limits) &&;
		RUST2_FFI_CONSUMES ::moq::server_Handshake with_receive_window(::std::uint64_t window) &&;
		RUST2_FFI_CALLABLE ::moq::server_Handshake_expired_future expired();
		RUST2_FFI_CONSUMES ::moq::server_Handshake_ok_future ok() &&;
		RUST2_FFI_CONSUMES void close(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE server_Admission {
		RUST2_FFI_UNCONSUMED server_Admission(server_Admission &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		server_Admission &operator=(server_Admission &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED server_Admission(const server_Admission &other) = default;
		server_Admission &operator=(const server_Admission &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static server_Admission default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::server_Admitted> enter() const;
		RUST2_FFI_CALLABLE void drain() const;
		RUST2_FFI_CALLABLE ::moq::server_Admission_close_future close() const;
	};
	class RUST2_FFI_CONSUMABLE server_Admitted {
		RUST2_FFI_UNCONSUMED server_Admitted(server_Admitted &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		server_Admitted &operator=(server_Admitted &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE session_Stats {
		RUST2_FFI_UNCONSUMED session_Stats(session_Stats &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		session_Stats &operator=(session_Stats &&other) noexcept = default;
		RUST2_FFI_CALLABLE session_Stats clone() const;
		friend bool operator==(const session_Stats &a, const session_Stats &b);
		friend bool operator!=(const session_Stats &a, const session_Stats &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static session_Stats default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> rtt_get() const;
		RUST2_FFI_CALLABLE void rtt_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::bandwidth_Rate> estimated_send_rate_get() const;
		RUST2_FFI_CALLABLE void estimated_send_rate_set(const ::std::optional<::moq::bandwidth_Rate> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bytes_sent_get() const;
		RUST2_FFI_CALLABLE void bytes_sent_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bytes_received_get() const;
		RUST2_FFI_CALLABLE void bytes_received_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bytes_lost_get() const;
		RUST2_FFI_CALLABLE void bytes_lost_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> packets_sent_get() const;
		RUST2_FFI_CALLABLE void packets_sent_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> packets_received_get() const;
		RUST2_FFI_CALLABLE void packets_received_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> packets_lost_get() const;
		RUST2_FFI_CALLABLE void packets_lost_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> streams_evicted_get() const;
		RUST2_FFI_CALLABLE void streams_evicted_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> datagrams_unrouted_get() const;
		RUST2_FFI_CALLABLE void datagrams_unrouted_set(const ::std::optional<::std::uint64_t> &value);
	};
	class RUST2_FFI_CONSUMABLE stats_Tier {
		RUST2_FFI_UNCONSUMED stats_Tier(stats_Tier &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		stats_Tier &operator=(stats_Tier &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED stats_Tier(const stats_Tier &other) = default;
		stats_Tier &operator=(const stats_Tier &other) = default;
		friend bool operator==(const stats_Tier &a, const stats_Tier &b);
		friend bool operator!=(const stats_Tier &a, const stats_Tier &b);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static stats_Tier default_();
		static ::moq::stats_Tier new_(::moq::Path label);
		RUST2_FFI_CALLABLE ::moq::Path label() const;
		RUST2_FFI_CALLABLE ::moq::buf as_str() const;
	};
	class RUST2_FFI_CONSUMABLE stats_Snapshot {
		RUST2_FFI_UNCONSUMED stats_Snapshot(stats_Snapshot &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		stats_Snapshot &operator=(stats_Snapshot &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED stats_Snapshot(const stats_Snapshot &other) = default;
		stats_Snapshot &operator=(const stats_Snapshot &other) = default;
		friend bool operator==(const stats_Snapshot &a, const stats_Snapshot &b);
		friend bool operator!=(const stats_Snapshot &a, const stats_Snapshot &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static stats_Snapshot default_();
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::stats_Tier, ::moq::stats_Message, ::moq::stats_Totals>> control() const;
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::stats_Tier, ::moq::stats_Role, ::moq::stats_Traffic>> traffic() const;
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::stats_Tier, ::moq::stats_Presence>> sessions() const;
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::stats_Tier, ::moq::stats_Activity>> activity() const;
	};
	class RUST2_FFI_CONSUMABLE stats_Report {
		RUST2_FFI_UNCONSUMED stats_Report(stats_Report &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		stats_Report &operator=(stats_Report &&other) noexcept = default;
		RUST2_FFI_CALLABLE stats_Report clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static stats_Report default_();
		RUST2_FFI_CALLABLE ::std::vector<::moq::stats_TrafficEntry> traffic_get() const;
		RUST2_FFI_CALLABLE void traffic_set(::std::vector<::moq::stats_TrafficEntry> value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::stats_SessionEntry> sessions_get() const;
		RUST2_FFI_CALLABLE void sessions_set(::std::vector<::moq::stats_SessionEntry> value);
	};
	class RUST2_FFI_CONSUMABLE stats_TrafficEntry {
		RUST2_FFI_UNCONSUMED stats_TrafficEntry(stats_TrafficEntry &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		stats_TrafficEntry &operator=(stats_TrafficEntry &&other) noexcept = default;
		RUST2_FFI_CALLABLE stats_TrafficEntry clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::Path path_get() const;
		RUST2_FFI_CALLABLE void path_set(::moq::Path value);
		RUST2_FFI_CALLABLE ::moq::stats_Tier tier_get() const;
		RUST2_FFI_CALLABLE void tier_set(::moq::stats_Tier value);
		RUST2_FFI_CALLABLE ::moq::stats_Traffic publisher_get() const;
		RUST2_FFI_CALLABLE void publisher_set(const ::moq::stats_Traffic &value);
		RUST2_FFI_CALLABLE ::moq::stats_Traffic subscriber_get() const;
		RUST2_FFI_CALLABLE void subscriber_set(const ::moq::stats_Traffic &value);
		RUST2_FFI_CALLABLE bool closed_get() const;
		RUST2_FFI_CALLABLE void closed_set(bool value);
	};
	class RUST2_FFI_CONSUMABLE stats_SessionEntry {
		RUST2_FFI_UNCONSUMED stats_SessionEntry(stats_SessionEntry &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		stats_SessionEntry &operator=(stats_SessionEntry &&other) noexcept = default;
		RUST2_FFI_CALLABLE stats_SessionEntry clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::stats_Tier tier_get() const;
		RUST2_FFI_CALLABLE void tier_set(::moq::stats_Tier value);
		RUST2_FFI_CALLABLE ::moq::Path label_get() const;
		RUST2_FFI_CALLABLE void label_set(::moq::Path value);
		RUST2_FFI_CALLABLE ::moq::stats_Presence presence_get() const;
		RUST2_FFI_CALLABLE void presence_set(const ::moq::stats_Presence &value);
		RUST2_FFI_CALLABLE bool closed_get() const;
		RUST2_FFI_CALLABLE void closed_set(bool value);
	};
	class RUST2_FFI_CONSUMABLE stats_Config {
		RUST2_FFI_UNCONSUMED stats_Config(stats_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		stats_Config &operator=(stats_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE stats_Config clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static stats_Config default_();
		RUST2_FFI_CALLABLE ::moq::Patterns exclude_get() const;
		RUST2_FFI_CALLABLE void exclude_set(::moq::Patterns value);
		static ::moq::stats_Config new_();
		RUST2_FFI_CONSUMES ::moq::stats_Config with_exclude(::moq::Pattern pattern) &&;
	};
	class RUST2_FFI_CONSUMABLE stats_Registry {
		RUST2_FFI_UNCONSUMED stats_Registry(stats_Registry &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		stats_Registry &operator=(stats_Registry &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED stats_Registry(const stats_Registry &other) = default;
		stats_Registry &operator=(const stats_Registry &other) = default;
		static stats_Registry default_();
		static ::moq::stats_Registry new_(::moq::stats_Config config);
		static ::moq::stats_Registry disabled();
		RUST2_FFI_CALLABLE bool is_enabled() const;
		RUST2_FFI_CALLABLE ::moq::stats_Handle tier(::moq::stats_Tier tier) const;
		RUST2_FFI_CALLABLE ::moq::stats_Snapshot snapshot() const;
		RUST2_FFI_CALLABLE void report(::moq::stats_Report &report, const ::moq::clock_Instant &now) const;
	};
	class RUST2_FFI_CONSUMABLE stats_Handle {
		RUST2_FFI_UNCONSUMED stats_Handle(stats_Handle &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		stats_Handle &operator=(stats_Handle &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED stats_Handle(const stats_Handle &other) = default;
		stats_Handle &operator=(const stats_Handle &other) = default;
		static stats_Handle default_();
		RUST2_FFI_CALLABLE void refuse(::moq::stats_Refusal refusal) const;
		RUST2_FFI_CALLABLE ::moq::stats_Session session(::moq::stats_SessionInfo session) const;
		RUST2_FFI_CALLABLE ::moq::stats_Session request(::moq::Path label) const;
	};
	class RUST2_FFI_CONSUMABLE stats_Refusal {
		RUST2_FFI_UNCONSUMED stats_Refusal(stats_Refusal &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		stats_Refusal &operator=(stats_Refusal &&other) noexcept = default;
		RUST2_FFI_CALLABLE stats_Refusal clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::Path label_get() const;
		RUST2_FFI_CALLABLE void label_set(::moq::Path value);
		RUST2_FFI_CALLABLE ::moq::stats_RefusalKind reason_get() const;
		RUST2_FFI_CALLABLE void reason_set(::moq::stats_RefusalKind value);
		static ::moq::stats_Refusal new_(::moq::Path label, ::moq::stats_RefusalKind reason);
	};
	class RUST2_FFI_CONSUMABLE stats_SessionInfo {
		RUST2_FFI_UNCONSUMED stats_SessionInfo(stats_SessionInfo &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		stats_SessionInfo &operator=(stats_SessionInfo &&other) noexcept = default;
		RUST2_FFI_CALLABLE stats_SessionInfo clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::Path label_get() const;
		RUST2_FFI_CALLABLE void label_set(::moq::Path value);
		static ::moq::stats_SessionInfo new_(::moq::Path label);
	};
	class RUST2_FFI_CONSUMABLE stats_Wire {
		RUST2_FFI_UNCONSUMED stats_Wire(stats_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		stats_Wire &operator=(stats_Wire &&other) noexcept = default;
		static stats_Wire default_();
	};
	class RUST2_FFI_CONSUMABLE stats_Session {
		RUST2_FFI_UNCONSUMED stats_Session(stats_Session &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		stats_Session &operator=(stats_Session &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED stats_Session(const stats_Session &other) = default;
		stats_Session &operator=(const stats_Session &other) = default;
		static stats_Session default_();
		RUST2_FFI_CALLABLE ::moq::stats_Wire wire(::moq::transport_Wire counters) const;
		RUST2_FFI_CALLABLE void retag(::moq::stats_Tier tier, ::moq::Path label) const;
	};
	class RUST2_FFI_CONSUMABLE stats_Bill {
		RUST2_FFI_UNCONSUMED stats_Bill(stats_Bill &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		stats_Bill &operator=(stats_Bill &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED stats_Bill(const stats_Bill &other) = default;
		stats_Bill &operator=(const stats_Bill &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static stats_Bill default_();
		RUST2_FFI_CALLABLE void deliver(::std::uint64_t bytes) const;
	};
	class RUST2_FFI_CONSUMABLE transport_Wire {
		RUST2_FFI_UNCONSUMED transport_Wire(transport_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		transport_Wire &operator=(transport_Wire &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED transport_Wire(const transport_Wire &other) = default;
		transport_Wire &operator=(const transport_Wire &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static transport_Wire default_();
		static ::moq::transport_Wire new_();
		RUST2_FFI_CALLABLE ::std::uint64_t sent_bytes() const;
		RUST2_FFI_CALLABLE ::std::uint64_t received_bytes() const;
		RUST2_FFI_CALLABLE void set_sent_bytes(::std::uint64_t total) const;
		RUST2_FFI_CALLABLE void set_received_bytes(::std::uint64_t total) const;
	};
	class RUST2_FFI_CONSUMABLE transport_Probe {
		RUST2_FFI_UNCONSUMED transport_Probe(transport_Probe &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		transport_Probe &operator=(transport_Probe &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED transport_Probe(const transport_Probe &other) = default;
		transport_Probe &operator=(const transport_Probe &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static transport_Probe default_();
		static ::moq::transport_Probe new_();
		RUST2_FFI_CALLABLE ::moq::bandwidth_Rate target() const;
		RUST2_FFI_CALLABLE void set_target(const ::moq::bandwidth_Rate &rate) const;
	};
	class RUST2_FFI_CONSUMABLE bandwidth_Limit {
		RUST2_FFI_UNCONSUMED bandwidth_Limit(bandwidth_Limit &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		bandwidth_Limit &operator=(bandwidth_Limit &&other) noexcept = default;
		RUST2_FFI_CALLABLE bandwidth_Limit clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static bandwidth_Limit default_();
		static ::moq::bandwidth_Limit new_(const ::moq::bandwidth_Cap *cap);
		RUST2_FFI_CALLABLE void set(const ::moq::bandwidth_Cap *cap) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::bandwidth_Cap> get();
		RUST2_FFI_CALLABLE ::moq::bandwidth_Limit_changed_future changed();
	};
	class RUST2_FFI_CONSUMABLE bandwidth_Demand {
		RUST2_FFI_UNCONSUMED bandwidth_Demand(bandwidth_Demand &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		bandwidth_Demand &operator=(bandwidth_Demand &&other) noexcept = default;
		RUST2_FFI_CALLABLE bandwidth_Demand clone() const;
		RUST2_FFI_CALLABLE bool wanted() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Demand_wanted_changed_future wanted_changed();
	};
	class RUST2_FFI_CONSUMABLE bandwidth_Reservation {
		RUST2_FFI_UNCONSUMED bandwidth_Reservation(bandwidth_Reservation &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		bandwidth_Reservation &operator=(bandwidth_Reservation &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::std::optional<::moq::bandwidth_Rate> rate() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Consumer consume() const;
		RUST2_FFI_CALLABLE void update(const ::moq::bandwidth_Rate &max) const;
	};
	class RUST2_FFI_CONSUMABLE bandwidth_Viewer {
		RUST2_FFI_UNCONSUMED bandwidth_Viewer(bandwidth_Viewer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		bandwidth_Viewer &operator=(bandwidth_Viewer &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::std::optional<::moq::bandwidth_Rate> rate() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Consumer consume() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> update(const ::moq::bandwidth_Rate &max) const;
	};
	class RUST2_FFI_CONSUMABLE bandwidth_Cap {
		RUST2_FFI_UNCONSUMED bandwidth_Cap(bandwidth_Cap &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		bandwidth_Cap &operator=(bandwidth_Cap &&other) noexcept = default;
		RUST2_FFI_CALLABLE bandwidth_Cap clone() const;
		friend bool operator==(const bandwidth_Cap &a, const bandwidth_Cap &b);
		friend bool operator!=(const bandwidth_Cap &a, const bandwidth_Cap &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Rate rate_get() const;
		RUST2_FFI_CALLABLE void rate_set(const ::moq::bandwidth_Rate &value);
		RUST2_FFI_CALLABLE ::moq::duration burst_get() const;
		RUST2_FFI_CALLABLE void burst_set(::moq::duration value);
		static ::moq::duration DEFAULT_BURST();
		static ::moq::bandwidth_Cap new_(const ::moq::bandwidth_Rate &rate);
		RUST2_FFI_CALLABLE ::moq::bandwidth_Cap with_burst(::moq::duration burst) const;
	};
	class RUST2_FFI_CONSUMABLE bandwidth_Producer {
		RUST2_FFI_UNCONSUMED bandwidth_Producer(bandwidth_Producer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		bandwidth_Producer &operator=(bandwidth_Producer &&other) noexcept = default;
		RUST2_FFI_CALLABLE bandwidth_Producer clone() const;
		static bandwidth_Producer default_();
		static ::moq::bandwidth_Producer new_();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> set(const ::std::optional<::moq::bandwidth_Rate> &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Consumer consume() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Weak weak() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Demand demand() const;
	};
	class RUST2_FFI_CONSUMABLE bandwidth_Weak {
		RUST2_FFI_UNCONSUMED bandwidth_Weak(bandwidth_Weak &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		bandwidth_Weak &operator=(bandwidth_Weak &&other) noexcept = default;
		RUST2_FFI_CALLABLE bandwidth_Weak clone() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Consumer consume() const;
	};
	class RUST2_FFI_CONSUMABLE bandwidth_Allocator {
		RUST2_FFI_UNCONSUMED bandwidth_Allocator(bandwidth_Allocator &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		bandwidth_Allocator &operator=(bandwidth_Allocator &&other) noexcept = default;
		RUST2_FFI_CALLABLE bandwidth_Allocator clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static bandwidth_Allocator default_();
		static ::moq::bandwidth_Allocator new_(::moq::bandwidth_Consumer estimate);
		RUST2_FFI_CALLABLE void set_target(const ::std::optional<::moq::bandwidth_Rate> &target) const;
		static ::moq::bandwidth_Allocator unlimited();
		RUST2_FFI_CALLABLE ::moq::bandwidth_Reservation reserve(const ::moq::track_Demand &track, const ::moq::bandwidth_Rate &max) const;
	};
	class RUST2_FFI_CONSUMABLE bandwidth_Viewers {
		RUST2_FFI_UNCONSUMED bandwidth_Viewers(bandwidth_Viewers &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		bandwidth_Viewers &operator=(bandwidth_Viewers &&other) noexcept = default;
		RUST2_FFI_CALLABLE bandwidth_Viewers clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static bandwidth_Viewers default_();
		static ::moq::bandwidth_Rate MAX_TARGET();
		static ::moq::bandwidth_Viewers new_(::moq::bandwidth_Weak estimate);
		RUST2_FFI_CALLABLE ::moq::result<::moq::bandwidth_Viewer, ::moq::Error> watch(const ::moq::bandwidth_Rate &max) const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> set_target(const ::std::optional<::moq::bandwidth_Rate> &target) const;
	};
	class RUST2_FFI_CONSUMABLE bandwidth_Consumer {
		RUST2_FFI_UNCONSUMED bandwidth_Consumer(bandwidth_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		bandwidth_Consumer &operator=(bandwidth_Consumer &&other) noexcept = default;
		RUST2_FFI_CALLABLE bandwidth_Consumer clone() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::bandwidth_Rate> peek() const;
		RUST2_FFI_CALLABLE bool probed() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Consumer_changed_future changed();
	};
	class RUST2_FFI_CONSUMABLE broadcast_DatagramRequests {
		RUST2_FFI_UNCONSUMED broadcast_DatagramRequests(broadcast_DatagramRequests &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		broadcast_DatagramRequests &operator=(broadcast_DatagramRequests &&other) noexcept = default;
		RUST2_FFI_CALLABLE broadcast_DatagramRequests clone() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_DatagramRequests_requested_future requested();
	};
	class RUST2_FFI_CONSUMABLE broadcast_Consumer {
		RUST2_FFI_UNCONSUMED broadcast_Consumer(broadcast_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		broadcast_Consumer &operator=(broadcast_Consumer &&other) noexcept = default;
		RUST2_FFI_CALLABLE broadcast_Consumer clone() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::broadcast_Tracks, ::moq::Error> tracks() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::track_Consumer, ::moq::Error> track(::moq::input name) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::datagram_Consumer, ::moq::Error> datagrams(::moq::input name) const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Consumer_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::broadcast_Consumer &other) const;
	};
	class RUST2_FFI_CONSUMABLE broadcast_Demand {
		RUST2_FFI_UNCONSUMED broadcast_Demand(broadcast_Demand &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		broadcast_Demand &operator=(broadcast_Demand &&other) noexcept = default;
		RUST2_FFI_CALLABLE broadcast_Demand clone() const;
		RUST2_FFI_CALLABLE bool wanted() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Demand_wanted_changed_future wanted_changed();
	};
	class RUST2_FFI_CONSUMABLE broadcast_Info {
		RUST2_FFI_UNCONSUMED broadcast_Info(broadcast_Info &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		broadcast_Info &operator=(broadcast_Info &&other) noexcept = default;
		RUST2_FFI_CALLABLE broadcast_Info clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static broadcast_Info default_();
		RUST2_FFI_CALLABLE ::moq::cache_Pool pool_get() const;
		RUST2_FFI_CALLABLE void pool_set(::moq::cache_Pool value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> cache_duration_get() const;
		RUST2_FFI_CALLABLE void cache_duration_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::moq::Path path_get() const;
		RUST2_FFI_CALLABLE void path_set(::moq::Path value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		static ::moq::broadcast_Info new_();
		RUST2_FFI_CONSUMES ::moq::broadcast_Producer produce() &&;
	};
	class RUST2_FFI_CONSUMABLE broadcast_Producer {
		RUST2_FFI_UNCONSUMED broadcast_Producer(broadcast_Producer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		broadcast_Producer &operator=(broadcast_Producer &&other) noexcept = default;
		RUST2_FFI_CALLABLE broadcast_Producer clone() const;
		static ::moq::broadcast_Producer new_(::moq::broadcast_Info info);
		RUST2_FFI_CALLABLE ::moq::broadcast_Info info() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::track_Producer, ::moq::Error> create_track(::moq::input name, ::std::optional<::moq::track_Info> info) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::datagram_Producer, ::moq::Error> create_datagrams(::moq::input name, const ::moq::datagram_Info &info) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::track_Request, ::moq::Error> reserve_track(::moq::input name) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::track_Request, ::moq::Error> reserve_unlisted_track(::moq::input name) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::track_Producer, ::moq::Error> unique_track(::moq::input suffix, ::std::optional<::moq::track_Info> info) const;
		RUST2_FFI_CALLABLE ::moq::buf unique_name(::moq::input suffix) const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Requests requests() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Consumer consume() const;
		RUST2_FFI_CALLABLE void close() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::broadcast_Producer &other) const;
	};
	class RUST2_FFI_CONSUMABLE broadcast_Requests {
		RUST2_FFI_UNCONSUMED broadcast_Requests(broadcast_Requests &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		broadcast_Requests &operator=(broadcast_Requests &&other) noexcept = default;
		RUST2_FFI_CALLABLE broadcast_Requests clone() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info info() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Requests_requested_track_future requested_track();
		RUST2_FFI_CALLABLE ::moq::broadcast_Consumer consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_DatagramRequests datagrams() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::broadcast_Lister, ::moq::Error> lister() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Requests_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::broadcast_Requests &other) const;
	};
	class RUST2_FFI_CONSUMABLE broadcast_Lister {
		RUST2_FFI_UNCONSUMED broadcast_Lister(broadcast_Lister &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		broadcast_Lister &operator=(broadcast_Lister &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::broadcast_Lister_poll_wanted_future poll_wanted() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Lister_poll_unwanted_future poll_unwanted() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> add(::moq::input name, ::moq::track_Info info) const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> remove(::moq::broadcast_Kind kind, ::moq::input name) const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> finish(::moq::broadcast_Kind kind, ::moq::input name) const;
		RUST2_FFI_CALLABLE void synced() const;
		RUST2_FFI_CALLABLE void clear() const;
		RUST2_FFI_CALLABLE void end() const;
		RUST2_FFI_CALLABLE void abort(::moq::Error err) const;
	};
	class RUST2_FFI_CONSUMABLE broadcast_Tracks {
		RUST2_FFI_UNCONSUMED broadcast_Tracks(broadcast_Tracks &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		broadcast_Tracks &operator=(broadcast_Tracks &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::broadcast_Tracks_next_future next();
		RUST2_FFI_CALLABLE ::moq::broadcast_Consumer broadcast() const;
	};
	class RUST2_FFI_CONSUMABLE broadcast_TracksEvent {
		RUST2_FFI_UNCONSUMED broadcast_TracksEvent(broadcast_TracksEvent &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		broadcast_TracksEvent &operator=(broadcast_TracksEvent &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED broadcast_TracksEvent(const broadcast_TracksEvent &other) = default;
		broadcast_TracksEvent &operator=(const broadcast_TracksEvent &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE cache_Config {
		RUST2_FFI_UNCONSUMED cache_Config(cache_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		cache_Config &operator=(cache_Config &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED cache_Config(const cache_Config &other) = default;
		cache_Config &operator=(const cache_Config &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static cache_Config default_();
		RUST2_FFI_CONSUMES ::moq::cache_Config with_capacity(const ::std::optional<::std::uint64_t> &capacity) &&;
		RUST2_FFI_CONSUMES ::moq::cache_Config with_expiry(const ::std::optional<::moq::duration> &expiry) &&;
	};
	class RUST2_FFI_CONSUMABLE cache_Pool {
		RUST2_FFI_UNCONSUMED cache_Pool(cache_Pool &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		cache_Pool &operator=(cache_Pool &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED cache_Pool(const cache_Pool &other) = default;
		cache_Pool &operator=(const cache_Pool &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static cache_Pool default_();
		static ::moq::cache_Pool new_(::moq::cache_Config config);
		static ::moq::cache_Pool unbounded();
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> capacity() const;
		RUST2_FFI_CALLABLE ::std::uint64_t used() const;
		RUST2_FFI_CALLABLE void resize(const ::std::optional<::std::uint64_t> &capacity) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> expiry() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> poll(const ::moq::clock_Instant &now, const ::moq::Waiter &waiter) const;
		RUST2_FFI_CALLABLE ::moq::cache_PoolWeak downgrade() const;
	};
	class RUST2_FFI_CONSUMABLE cache_PoolWeak {
		RUST2_FFI_UNCONSUMED cache_PoolWeak(cache_PoolWeak &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		cache_PoolWeak &operator=(cache_PoolWeak &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED cache_PoolWeak(const cache_PoolWeak &other) = default;
		cache_PoolWeak &operator=(const cache_PoolWeak &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::cache_Pool> upgrade() const;
	};
	class RUST2_FFI_CONSUMABLE datagram_Producer {
		RUST2_FFI_UNCONSUMED datagram_Producer(datagram_Producer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		datagram_Producer &operator=(datagram_Producer &&other) noexcept = default;
		RUST2_FFI_CALLABLE datagram_Producer clone() const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::datagram_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::std::uint64_t, ::moq::Error> append(const ::moq::Timestamp &timestamp, ::moq::input payload);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> insert(const ::moq::datagram_Datagram &datagram);
		RUST2_FFI_CALLABLE ::moq::datagram_Consumer consume() const;
		RUST2_FFI_CONSUMES void finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::std::optional<::std::int8_t> priority() const;
		RUST2_FFI_CALLABLE ::moq::datagram_Producer_priority_changed_future priority_changed();
	};
	class RUST2_FFI_CONSUMABLE datagram_Request {
		RUST2_FFI_UNCONSUMED datagram_Request(datagram_Request &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		datagram_Request &operator=(datagram_Request &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::std::optional<::std::int8_t> priority() const;
		RUST2_FFI_CONSUMES ::moq::datagram_Producer accept(const ::moq::datagram_Info &info) &&;
		RUST2_FFI_CONSUMES void reject(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE datagram_Consumer {
		RUST2_FFI_UNCONSUMED datagram_Consumer(datagram_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		datagram_Consumer &operator=(datagram_Consumer &&other) noexcept = default;
		RUST2_FFI_CALLABLE datagram_Consumer clone() const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::datagram_Consumer_info_future info() const;
		RUST2_FFI_CALLABLE ::moq::datagram_Consumer_recv_future recv();
		RUST2_FFI_CALLABLE ::std::int8_t priority() const;
		RUST2_FFI_CALLABLE void set_priority(::std::int8_t priority);
	};
	class RUST2_FFI_CONSUMABLE datagram_Info {
		RUST2_FFI_UNCONSUMED datagram_Info(datagram_Info &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		datagram_Info &operator=(datagram_Info &&other) noexcept = default;
		RUST2_FFI_CALLABLE datagram_Info clone() const;
		friend bool operator==(const datagram_Info &a, const datagram_Info &b);
		friend bool operator!=(const datagram_Info &a, const datagram_Info &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static datagram_Info default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timescale> timescale_get() const;
		RUST2_FFI_CALLABLE void timescale_set(const ::std::optional<::moq::Timescale> &value);
		RUST2_FFI_CALLABLE ::std::int8_t priority_get() const;
		RUST2_FFI_CALLABLE void priority_set(::std::int8_t value);
		RUST2_FFI_CALLABLE ::moq::datagram_Info with_timescale(const ::std::optional<::moq::Timescale> &timescale) const;
		RUST2_FFI_CALLABLE ::moq::datagram_Info with_priority(::std::int8_t priority) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::Error> rescale(const ::moq::Timestamp &timestamp) const;
	};
	class RUST2_FFI_CONSUMABLE frame_Producer {
		RUST2_FFI_UNCONSUMED frame_Producer(frame_Producer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		frame_Producer &operator=(frame_Producer &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::group_Producer group() const;
		RUST2_FFI_CALLABLE ::std::size_t remaining() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> write(::moq::input chunk);
		RUST2_FFI_CONSUMES ::moq::result<::moq::group_Producer, ::moq::Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::Error> abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::std::uint64_t size_get() const;
		RUST2_FFI_CALLABLE ::moq::Timestamp timestamp_get() const;
	};
	class RUST2_FFI_CONSUMABLE frame_Frame {
		RUST2_FFI_UNCONSUMED frame_Frame(frame_Frame &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		frame_Frame &operator=(frame_Frame &&other) noexcept = default;
		RUST2_FFI_CALLABLE frame_Frame clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::Timestamp timestamp_get() const;
		RUST2_FFI_CALLABLE void timestamp_set(const ::moq::Timestamp &value);
		RUST2_FFI_CALLABLE ::moq::buf payload_get() const;
		RUST2_FFI_CALLABLE void payload_set(::moq::input value);
		static ::moq::frame_Frame new_(const ::moq::Timestamp &timestamp, ::moq::input payload);
		RUST2_FFI_CALLABLE ::moq::clock_Instant arrived() const;
	};
	class RUST2_FFI_CONSUMABLE frame_Consumer {
		RUST2_FFI_UNCONSUMED frame_Consumer(frame_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		frame_Consumer &operator=(frame_Consumer &&other) noexcept = default;
		RUST2_FFI_CALLABLE frame_Consumer clone() const;
		RUST2_FFI_CALLABLE ::moq::frame_Consumer_read_chunk_future read_chunk();
		RUST2_FFI_CALLABLE ::moq::frame_Consumer_read_all_future read_all();
		RUST2_FFI_CALLABLE ::std::uint64_t size_get() const;
		RUST2_FFI_CALLABLE ::moq::Timestamp timestamp_get() const;
	};
	class RUST2_FFI_CONSUMABLE group_Producer {
		RUST2_FFI_UNCONSUMED group_Producer(group_Producer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		group_Producer &operator=(group_Producer &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timescale> timescale() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> start_at(::std::uint64_t index);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> write_frame(const ::moq::Timestamp &timestamp, ::moq::input data);
		RUST2_FFI_CONSUMES ::moq::result<::moq::frame_Producer, ::moq::Error> create_frame(const ::moq::frame_Info &frame) &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::group_Finished, ::moq::Error> finish_at(::std::uint64_t end) &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::group_Finished, ::moq::Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::Error> abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::group_Producer_ready_future ready(const ::moq::frame_Info &frame) const;
		RUST2_FFI_CALLABLE ::std::size_t frame_count() const;
		RUST2_FFI_CALLABLE ::moq::group_Consumer consume() const;
		RUST2_FFI_CALLABLE ::moq::group_Producer_closed_future closed() const;
		RUST2_FFI_CALLABLE ::moq::group_Demand demand() const;
		RUST2_FFI_CALLABLE ::std::uint64_t start_get() const;
	};
	class RUST2_FFI_CONSUMABLE group_Finished {
		RUST2_FFI_UNCONSUMED group_Finished(group_Finished &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		group_Finished &operator=(group_Finished &&other) noexcept = default;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::Error> abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::std::size_t frame_count() const;
		RUST2_FFI_CALLABLE ::moq::group_Consumer consume() const;
		RUST2_FFI_CALLABLE ::std::uint64_t start_get() const;
	};
	class RUST2_FFI_CONSUMABLE group_Demand {
		RUST2_FFI_UNCONSUMED group_Demand(group_Demand &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		group_Demand &operator=(group_Demand &&other) noexcept = default;
		RUST2_FFI_CALLABLE group_Demand clone() const;
		RUST2_FFI_CALLABLE ::std::uint64_t start() const;
		RUST2_FFI_CALLABLE bool wanted() const;
		RUST2_FFI_CALLABLE ::moq::group_Demand_wanted_changed_future wanted_changed();
		RUST2_FFI_CALLABLE ::moq::group_Demand_closed_future closed() const;
	};
	class RUST2_FFI_CONSUMABLE group_Consumer {
		RUST2_FFI_UNCONSUMED group_Consumer(group_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		group_Consumer &operator=(group_Consumer &&other) noexcept = default;
		RUST2_FFI_CALLABLE group_Consumer clone() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timescale> timescale() const;
		RUST2_FFI_CALLABLE ::moq::clock_Instant arrived() const;
		RUST2_FFI_CALLABLE ::std::uint64_t index() const;
		RUST2_FFI_CONSUMES ::moq::group_Consumer with_range(const ::moq::bounds_u64 &frames) &&;
		RUST2_FFI_CALLABLE void skip_to(::std::uint64_t index);
		RUST2_FFI_CALLABLE ::std::size_t frame_count() const;
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> end() const;
		RUST2_FFI_CALLABLE ::moq::group_Consumer_next_frame_future next_frame();
		RUST2_FFI_CALLABLE ::moq::group_Consumer_read_frame_future read_frame();
		RUST2_FFI_CALLABLE ::moq::group_Consumer_finished_future finished();
		RUST2_FFI_CALLABLE ::std::uint64_t start_get() const;
	};
	class RUST2_FFI_CONSUMABLE group_Fetch {
		RUST2_FFI_UNCONSUMED group_Fetch(group_Fetch &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		group_Fetch &operator=(group_Fetch &&other) noexcept = default;
		RUST2_FFI_CALLABLE group_Fetch clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static group_Fetch default_();
		RUST2_FFI_CALLABLE ::std::int8_t priority_get() const;
		RUST2_FFI_CALLABLE void priority_set(::std::int8_t value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> max_delay_get() const;
		RUST2_FFI_CALLABLE void max_delay_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CONSUMES ::moq::group_Fetch with_priority(::std::int8_t priority) &&;
		RUST2_FFI_CONSUMES ::moq::group_Fetch with_max_delay(::moq::duration max_delay) &&;
	};
	class RUST2_FFI_CONSUMABLE track_Finished {
		RUST2_FFI_UNCONSUMED track_Finished(track_Finished &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Finished &operator=(track_Finished &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::Error> abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Finished_closed_future closed() const;
		RUST2_FFI_CALLABLE ::moq::track_Consumer consume() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
	};
	class RUST2_FFI_CONSUMABLE track_Requests {
		RUST2_FFI_UNCONSUMED track_Requests(track_Requests &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Requests &operator=(track_Requests &&other) noexcept = default;
		RUST2_FFI_CALLABLE track_Requests clone() const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests_requested_fill_future requested_fill() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
	};
	class RUST2_FFI_CONSUMABLE track_Demand {
		RUST2_FFI_UNCONSUMED track_Demand(track_Demand &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Demand &operator=(track_Demand &&other) noexcept = default;
		RUST2_FFI_CALLABLE track_Demand clone() const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE bool wanted() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand_wanted_changed_future wanted_changed();
		RUST2_FFI_CALLABLE ::moq::track_Demand_refresh_future refresh() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand_closed_future closed() const;
	};
	class RUST2_FFI_CONSUMABLE track_Subscriber {
		RUST2_FFI_UNCONSUMED track_Subscriber(track_Subscriber &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Subscriber &operator=(track_Subscriber &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::stats_Bill bill();
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Control control() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber_recv_group_future recv_group();
		RUST2_FFI_CONSUMES ::moq::track_Ordered ordered() &&;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Subscriber &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber_start_future start();
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> floor() const;
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> next_in_range(::std::uint64_t from) const;
		RUST2_FFI_CALLABLE bool is_skipped(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber_finished_future finished();
		RUST2_FFI_CONSUMES ::moq::track_Subscriber with_range(const ::moq::bounds_u64 &groups) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscription subscription() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> update(::moq::track_Subscription subscription);
		RUST2_FFI_CALLABLE ::moq::track_Subscriber_largest_future largest();
		RUST2_FFI_CALLABLE ::moq::track_Subscriber_refresh_future refresh() const;
	};
	class RUST2_FFI_CONSUMABLE track_Ordered {
		RUST2_FFI_UNCONSUMED track_Ordered(track_Ordered &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Ordered &operator=(track_Ordered &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Ordered_next_group_future next_group();
		RUST2_FFI_CALLABLE ::moq::track_Control control() const;
		RUST2_FFI_CALLABLE ::moq::track_Ordered_refresh_future refresh() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscription subscription() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> update(::moq::track_Subscription subscription);
		RUST2_FFI_CALLABLE ::moq::track_Ordered_finished_future finished();
		RUST2_FFI_CALLABLE ::moq::track_Ordered_largest_future largest();
		RUST2_FFI_CALLABLE bool is_skipped(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Ordered &other) const;
	};
	class RUST2_FFI_CONSUMABLE track_Item {
		RUST2_FFI_UNCONSUMED track_Item(track_Item &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Item &operator=(track_Item &&other) noexcept = default;
		RUST2_FFI_CONSUMES ::std::optional<::moq::group_Consumer> group() &&;
	};
	class RUST2_FFI_CONSUMABLE track_Request {
		RUST2_FFI_UNCONSUMED track_Request(track_Request &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Request &operator=(track_Request &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::track_Consumer consume() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CONSUMES ::moq::track_Producer accept(::std::optional<::moq::track_Info> info) &&;
		RUST2_FFI_CONSUMES void reject(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Request_subscription_changed_future subscription_changed();
	};
	class RUST2_FFI_CONSUMABLE track_Info {
		RUST2_FFI_UNCONSUMED track_Info(track_Info &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Info &operator=(track_Info &&other) noexcept = default;
		RUST2_FFI_CALLABLE track_Info clone() const;
		friend bool operator==(const track_Info &a, const track_Info &b);
		friend bool operator!=(const track_Info &a, const track_Info &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static track_Info default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timescale> timescale_get() const;
		RUST2_FFI_CALLABLE void timescale_set(const ::std::optional<::moq::Timescale> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> max_age_get() const;
		RUST2_FFI_CALLABLE void max_age_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::int8_t priority_get() const;
		RUST2_FFI_CALLABLE void priority_set(::std::int8_t value);
		RUST2_FFI_CALLABLE ::moq::track_Compression compression_get() const;
		RUST2_FFI_CALLABLE void compression_set(::moq::track_Compression value);
		RUST2_FFI_CALLABLE ::moq::track_Refresh refresh_get() const;
		RUST2_FFI_CALLABLE void refresh_set(const ::moq::track_Refresh &value);
		RUST2_FFI_CONSUMES ::moq::track_Info with_timescale(const ::std::optional<::moq::Timescale> &timescale) &&;
		RUST2_FFI_CONSUMES ::moq::track_Info with_max_age(const ::std::optional<::moq::duration> &max_age) &&;
		RUST2_FFI_CONSUMES ::moq::track_Info with_priority(::std::int8_t priority) &&;
		RUST2_FFI_CONSUMES ::moq::track_Info with_compression(::moq::track_Compression compression) &&;
		RUST2_FFI_CONSUMES ::moq::track_Info with_refresh(const ::moq::track_Refresh &refresh) &&;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::Error> rescale(const ::moq::Timestamp &timestamp) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::Error> rescale_in(const ::moq::Timestamp &timestamp, ::std::uint64_t key) const;
	};
	class RUST2_FFI_CONSUMABLE track_Refresh {
		RUST2_FFI_UNCONSUMED track_Refresh(track_Refresh &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Refresh &operator=(track_Refresh &&other) noexcept = default;
		RUST2_FFI_CALLABLE track_Refresh clone() const;
		friend bool operator==(const track_Refresh &a, const track_Refresh &b);
		friend bool operator!=(const track_Refresh &a, const track_Refresh &b);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static track_Refresh default_();
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> ratio_get() const;
		RUST2_FFI_CALLABLE void ratio_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> interval_get() const;
		RUST2_FFI_CALLABLE void interval_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::moq::track_Refresh with_ratio(const ::std::optional<::std::uint32_t> &ratio) const;
		RUST2_FFI_CALLABLE ::moq::track_Refresh with_interval(const ::std::optional<::moq::duration> &interval) const;
	};
	class RUST2_FFI_CONSUMABLE track_Producer {
		RUST2_FFI_UNCONSUMED track_Producer(track_Producer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Producer &operator=(track_Producer &&other) noexcept = default;
		RUST2_FFI_CALLABLE track_Producer clone() const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group(const ::moq::Timestamp &start) const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> write_frame(const ::moq::Timestamp &timestamp, ::moq::input frame);
		RUST2_FFI_CONSUMES ::moq::result<::moq::track_Finished, ::moq::Error> finish() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::track_Finished, ::moq::Error> fail(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> finish_at(const ::moq::track_Position &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> start_at(const ::std::optional<::std::uint64_t> &key);
		RUST2_FFI_CALLABLE void skip_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::Error> abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::track_Producer> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Producer_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_AbortHandle abort_handle() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Consumer consume() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Producer_subscription_changed_future subscription_changed();
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE track_AbortHandle {
		RUST2_FFI_UNCONSUMED track_AbortHandle(track_AbortHandle &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_AbortHandle &operator=(track_AbortHandle &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED track_AbortHandle(const track_AbortHandle &other) = default;
		track_AbortHandle &operator=(const track_AbortHandle &other) = default;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE void abort(::moq::Error err) const;
	};
	class RUST2_FFI_CONSUMABLE track_Consumer {
		RUST2_FFI_UNCONSUMED track_Consumer(track_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Consumer &operator=(track_Consumer &&other) noexcept = default;
		RUST2_FFI_CALLABLE track_Consumer clone() const;
		RUST2_FFI_CONSUMES ::moq::track_Consumer unmetered() &&;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Consumer_subscribe_future subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::moq::track_Consumer_fetch_group_future fetch_group(::std::uint64_t key, ::std::optional<::moq::group_Fetch> options) const;
		RUST2_FFI_CALLABLE ::moq::track_Consumer_info_future info() const;
		RUST2_FFI_CALLABLE ::moq::track_Consumer_largest_future largest();
	};
	class RUST2_FFI_CONSUMABLE track_Control {
		RUST2_FFI_UNCONSUMED track_Control(track_Control &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Control &operator=(track_Control &&other) noexcept = default;
		RUST2_FFI_CALLABLE track_Control clone() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscription subscription() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> update(::moq::track_Subscription subscription) const;
		RUST2_FFI_CALLABLE ::moq::track_Control_refresh_future refresh() const;
	};
	class RUST2_FFI_CONSUMABLE track_Subscription {
		RUST2_FFI_UNCONSUMED track_Subscription(track_Subscription &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Subscription &operator=(track_Subscription &&other) noexcept = default;
		RUST2_FFI_CALLABLE track_Subscription clone() const;
		friend bool operator==(const track_Subscription &a, const track_Subscription &b);
		friend bool operator!=(const track_Subscription &a, const track_Subscription &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static track_Subscription default_();
		RUST2_FFI_CALLABLE ::std::int8_t priority_get() const;
		RUST2_FFI_CALLABLE void priority_set(::std::int8_t value);
		RUST2_FFI_CALLABLE ::moq::track_Delay delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::moq::track_Delay &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::track_Range> ranges_get() const;
		RUST2_FFI_CALLABLE void ranges_set(const ::std::vector<::moq::track_Range> &value);
		RUST2_FFI_CALLABLE ::moq::track_Order order_get() const;
		RUST2_FFI_CALLABLE void order_set(::moq::track_Order value);
		RUST2_FFI_CONSUMES ::moq::track_Subscription with_priority(::std::int8_t priority) &&;
		RUST2_FFI_CONSUMES ::moq::track_Subscription with_delay(const ::moq::track_Delay &delay) &&;
		RUST2_FFI_CONSUMES ::moq::track_Subscription with_ranges(const ::std::vector<::moq::track_Range> &ranges) &&;
		RUST2_FFI_CONSUMES ::moq::track_Subscription with_order(::moq::track_Order order) &&;
	};
	class RUST2_FFI_CONSUMABLE track_Fill {
		RUST2_FFI_UNCONSUMED track_Fill(track_Fill &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Fill &operator=(track_Fill &&other) noexcept = default;
	};
	class RUST2_FFI_CONSUMABLE origin_Following {
		RUST2_FFI_UNCONSUMED origin_Following(origin_Following &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Following &operator=(origin_Following &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::origin_Following_available_future available();
	};
	class RUST2_FFI_CONSUMABLE origin_Claim {
		RUST2_FFI_UNCONSUMED origin_Claim(origin_Claim &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Claim &operator=(origin_Claim &&other) noexcept = default;
		RUST2_FFI_CALLABLE origin_Claim clone() const;
		RUST2_FFI_CALLABLE ::moq::Instance instance() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> announce() const;
		RUST2_FFI_CALLABLE void withdraw() const;
		RUST2_FFI_CALLABLE bool is_announced() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> set_cost(const ::moq::origin_Cost &cost) const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> set_auth(::std::optional<::moq::auth_Credential> auth) const;
		RUST2_FFI_CALLABLE ::moq::origin_Claim_requested_future requested() const;
	};
	class RUST2_FFI_CONSUMABLE origin_Config {
		RUST2_FFI_UNCONSUMED origin_Config(origin_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Config &operator=(origin_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE origin_Config clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static origin_Config default_();
		RUST2_FFI_CALLABLE ::std::uint64_t node_get() const;
		RUST2_FFI_CALLABLE void node_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::size_t max_hops_get() const;
		RUST2_FFI_CALLABLE void max_hops_set(::std::size_t value);
		RUST2_FFI_CALLABLE ::moq::cache_Pool pool_get() const;
		RUST2_FFI_CALLABLE void pool_set(::moq::cache_Pool value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> cache_duration_get() const;
		RUST2_FFI_CALLABLE void cache_duration_set(const ::std::optional<::moq::duration> &value);
		static ::moq::origin_Config new_(::std::uint64_t node);
	};
	class RUST2_FFI_CONSUMABLE origin_Consumer {
		RUST2_FFI_UNCONSUMED origin_Consumer(origin_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Consumer &operator=(origin_Consumer &&other) noexcept = default;
		RUST2_FFI_CALLABLE origin_Consumer clone() const;
		RUST2_FFI_CALLABLE ::std::uint64_t node() const;
		RUST2_FFI_CONSUMES ::moq::origin_Consumer with_hidden(bool hidden) &&;
		RUST2_FFI_CONSUMES ::moq::origin_Consumer covering() &&;
		RUST2_FFI_CALLABLE ::moq::result<::moq::origin_Consumer, ::moq::Error> exclude(::moq::input prefix) const;
		RUST2_FFI_CONSUMES ::moq::origin_Consumer with_stats(::moq::stats_Session session) &&;
		RUST2_FFI_CALLABLE ::moq::announce_Consumer announced() const;
		RUST2_FFI_CALLABLE ::moq::origin_Routes routes() const;
		RUST2_FFI_CALLABLE ::moq::origin_Consumer consume() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::origin_Following, ::moq::Error> follow(::moq::input path, ::std::optional<::moq::Instance> instance) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::origin_Consumer, ::moq::Error> scope(::moq::input root, const ::moq::Patterns &patterns) const;
		RUST2_FFI_CALLABLE ::moq::origin_Consumer_request_future request(::moq::input path, ::std::optional<::moq::Instance> instance) const;
		RUST2_FFI_CALLABLE ::moq::Path root() const;
		RUST2_FFI_CALLABLE ::moq::Patterns allowed() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Path, ::moq::Error> absolute(::moq::input path) const;
	};
	class RUST2_FFI_CONSUMABLE origin_Driver {
		RUST2_FFI_UNCONSUMED origin_Driver(origin_Driver &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Driver &operator=(origin_Driver &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<::std::optional<::moq::clock_Instant>, ::moq::Error> poll(const ::moq::clock_Instant &now, const ::moq::Waiter &waiter);
	};
	class RUST2_FFI_CONSUMABLE origin_Producer {
		RUST2_FFI_UNCONSUMED origin_Producer(origin_Producer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Producer &operator=(origin_Producer &&other) noexcept = default;
		RUST2_FFI_CALLABLE origin_Producer clone() const;
		static ::std::tuple<::moq::origin_Producer, ::moq::origin_Driver> new_(::moq::origin_Config config);
		RUST2_FFI_CONSUMES ::moq::origin_Producer with_stats(::moq::stats_Session session) &&;
		RUST2_FFI_CONSUMES ::moq::origin_Producer with_link(::moq::origin_Link link) &&;
		RUST2_FFI_CONSUMES ::moq::origin_Producer with_priority(const ::moq::origin_Priority &priority) &&;
		RUST2_FFI_CONSUMES ::moq::origin_Producer with_proxy(bool proxy) &&;
		RUST2_FFI_CALLABLE ::moq::origin_Config config() const;
		RUST2_FFI_CALLABLE ::std::uint64_t node() const;
		RUST2_FFI_CALLABLE bool is_own(::std::uint64_t id) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::origin_Route, ::moq::Error> route(::moq::input path, ::moq::origin_RouteOptions options) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::origin_Claim, ::moq::Error> claim(::moq::input prefix, ::moq::origin_RouteOptions options) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::origin_Producer, ::moq::Error> scope(::moq::input root, const ::moq::Patterns &patterns) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::origin_Producer, ::moq::Error> exclude(::moq::input prefix) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::origin_Producer, ::moq::Error> mount(::moq::input at, ::moq::input target) const;
		RUST2_FFI_CALLABLE ::moq::origin_Consumer consume() const;
		RUST2_FFI_CALLABLE ::moq::Path root() const;
		RUST2_FFI_CALLABLE ::moq::Patterns allowed() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Path, ::moq::Error> absolute(::moq::input path) const;
	};
	class RUST2_FFI_CONSUMABLE origin_Request {
		RUST2_FFI_UNCONSUMED origin_Request(origin_Request &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Request &operator=(origin_Request &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::Path path() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::origin_Route, ::moq::Error> route(::moq::origin_RouteOptions options) &&;
		RUST2_FFI_CONSUMES void serve_Producer(const ::moq::broadcast_Producer &broadcast) &&;
		RUST2_FFI_CONSUMES void serve_Consumer(const ::moq::broadcast_Consumer &broadcast) &&;
		RUST2_FFI_CONSUMES void reject(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE origin_Route {
		RUST2_FFI_UNCONSUMED origin_Route(origin_Route &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Route &operator=(origin_Route &&other) noexcept = default;
		RUST2_FFI_CALLABLE origin_Route clone() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Producer broadcast() const;
		RUST2_FFI_CALLABLE ::moq::Instance instance() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> announce() const;
		RUST2_FFI_CALLABLE void withdraw() const;
		RUST2_FFI_CALLABLE bool is_announced() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> set_cost(const ::moq::origin_Cost &cost) const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> set_auth(::std::optional<::moq::auth_Credential> auth) const;
	};
	class RUST2_FFI_CONSUMABLE origin_RouteInfo {
		RUST2_FFI_UNCONSUMED origin_RouteInfo(origin_RouteInfo &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_RouteInfo &operator=(origin_RouteInfo &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED origin_RouteInfo(const origin_RouteInfo &other) = default;
		origin_RouteInfo &operator=(const origin_RouteInfo &other) = default;
		friend bool operator==(const origin_RouteInfo &a, const origin_RouteInfo &b);
		friend bool operator!=(const origin_RouteInfo &a, const origin_RouteInfo &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::Instance instance() const;
		RUST2_FFI_CALLABLE ::moq::origin_Cost cost() const;
		RUST2_FFI_CALLABLE bool available() const;
	};
	class RUST2_FFI_CONSUMABLE origin_RouteOptions {
		RUST2_FFI_UNCONSUMED origin_RouteOptions(origin_RouteOptions &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_RouteOptions &operator=(origin_RouteOptions &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED origin_RouteOptions(const origin_RouteOptions &other) = default;
		origin_RouteOptions &operator=(const origin_RouteOptions &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static origin_RouteOptions default_();
		static ::moq::origin_RouteOptions hidden();
		RUST2_FFI_CONSUMES ::moq::origin_RouteOptions with_cost(const ::moq::origin_Cost &cost) &&;
		RUST2_FFI_CONSUMES ::moq::origin_RouteOptions with_auth(::moq::auth_Credential auth) &&;
		RUST2_FFI_CONSUMES ::moq::origin_RouteOptions with_instance(::moq::Instance instance, ::std::uint64_t epoch) &&;
	};
	class RUST2_FFI_CONSUMABLE origin_RouteEvent {
		RUST2_FFI_UNCONSUMED origin_RouteEvent(origin_RouteEvent &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_RouteEvent &operator=(origin_RouteEvent &&other) noexcept = default;
		RUST2_FFI_CALLABLE origin_RouteEvent clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::origin_RouteKind kind_get() const;
		RUST2_FFI_CALLABLE void kind_set(::moq::origin_RouteKind value);
		RUST2_FFI_CALLABLE ::moq::announce_Announce announce_get() const;
		RUST2_FFI_CALLABLE void announce_set(::moq::announce_Announce value);
	};
	class RUST2_FFI_CONSUMABLE origin_Routes {
		RUST2_FFI_UNCONSUMED origin_Routes(origin_Routes &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Routes &operator=(origin_Routes &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::origin_Routes_next_future next();
		RUST2_FFI_CALLABLE ::std::optional<::moq::origin_RouteEvent> try_next();
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::moq::origin_Routes_poll_synced_future poll_synced() const;
		RUST2_FFI_CALLABLE ::moq::Path root() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Path, ::moq::Error> absolute(::moq::input prefix) const;
	};
	class RUST2_FFI_CONSUMABLE announce_Consumer {
		RUST2_FFI_UNCONSUMED announce_Consumer(announce_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		announce_Consumer &operator=(announce_Consumer &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::announce_Consumer_next_future next();
		RUST2_FFI_CALLABLE ::std::optional<::moq::announce_Event> try_next();
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::moq::announce_Consumer_poll_synced_future poll_synced() const;
		RUST2_FFI_CALLABLE ::moq::Path root() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Path, ::moq::Error> absolute(::moq::input prefix) const;
	};
	class RUST2_FFI_CONSUMABLE announce_Event {
		RUST2_FFI_UNCONSUMED announce_Event(announce_Event &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		announce_Event &operator=(announce_Event &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED announce_Event(const announce_Event &other) = default;
		announce_Event &operator=(const announce_Event &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE announce_Announce {
		RUST2_FFI_UNCONSUMED announce_Announce(announce_Announce &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		announce_Announce &operator=(announce_Announce &&other) noexcept = default;
		RUST2_FFI_CALLABLE announce_Announce clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::Path prefix_get() const;
		RUST2_FFI_CALLABLE void prefix_set(::moq::Path value);
		RUST2_FFI_CALLABLE ::std::optional<::std::vector<::moq::Pattern>> captures_get() const;
		RUST2_FFI_CALLABLE void captures_set(::std::optional<::std::vector<::moq::Pattern>> value);
		RUST2_FFI_CALLABLE ::moq::origin_RouteInfo route_get() const;
		RUST2_FFI_CALLABLE void route_set(::moq::origin_RouteInfo value);
	};
	class RUST2_FFI_CONSUMABLE transport_memory_Config {
		RUST2_FFI_UNCONSUMED transport_memory_Config(transport_memory_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		transport_memory_Config &operator=(transport_memory_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE transport_memory_Config clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static transport_memory_Config default_();
		RUST2_FFI_CALLABLE ::std::size_t stream_buffer_get() const;
		RUST2_FFI_CALLABLE void stream_buffer_set(::std::size_t value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> protocol_get() const;
		RUST2_FFI_CALLABLE void protocol_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::std::size_t> max_concurrent_uni_streams_get() const;
		RUST2_FFI_CALLABLE void max_concurrent_uni_streams_set(const ::std::optional<::std::size_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::size_t> max_concurrent_bi_streams_get() const;
		RUST2_FFI_CALLABLE void max_concurrent_bi_streams_set(const ::std::optional<::std::size_t> &value);
		RUST2_FFI_CALLABLE ::moq::duration latency_get() const;
		RUST2_FFI_CALLABLE void latency_set(::moq::duration value);
	};
	class RUST2_FFI_CONSUMABLE transport_memory_Session {
		RUST2_FFI_UNCONSUMED transport_memory_Session(transport_memory_Session &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		transport_memory_Session &operator=(transport_memory_Session &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED transport_memory_Session(const transport_memory_Session &other) = default;
		transport_memory_Session &operator=(const transport_memory_Session &other) = default;
		RUST2_FFI_CALLABLE ::moq::transport_memory_Closer closer() const;
	};
	class RUST2_FFI_CONSUMABLE transport_memory_Closer {
		RUST2_FFI_UNCONSUMED transport_memory_Closer(transport_memory_Closer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		transport_memory_Closer &operator=(transport_memory_Closer &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED transport_memory_Closer(const transport_memory_Closer &other) = default;
		transport_memory_Closer &operator=(const transport_memory_Closer &other) = default;
		RUST2_FFI_CALLABLE void close(::std::uint32_t code, ::moq::input reason) const;
		RUST2_FFI_CALLABLE bool is_closed() const;
	};
	class transport_Metrics {
		virtual ~transport_Metrics() = default;
	};
	class TimeOverflow {
		friend bool operator==(const TimeOverflow &a, const TimeOverflow &b);
		friend bool operator!=(const TimeOverflow &a, const TimeOverflow &b);
		::moq::buf to_string() const;
		::moq::buf debug() const;
	};
	class Timescale {
		friend bool operator==(const Timescale &a, const Timescale &b);
		friend bool operator!=(const Timescale &a, const Timescale &b);
		friend bool operator<(const Timescale &a, const Timescale &b);
		friend bool operator<=(const Timescale &a, const Timescale &b);
		friend bool operator>(const Timescale &a, const Timescale &b);
		friend bool operator>=(const Timescale &a, const Timescale &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const Timescale &a, const Timescale &b);
		#endif
		::moq::buf to_string() const;
		::moq::buf debug() const;
		static ::moq::Timescale SECOND();
		static ::moq::Timescale MILLI();
		static ::moq::Timescale MICRO();
		static ::moq::Timescale NANO();
		static ::moq::result<::moq::Timescale, ::moq::TimeOverflow> new_(::std::uint64_t units_per_second);
		::std::uint64_t as_u64() const;
	};
	class Timestamp {
		friend bool operator==(const Timestamp &a, const Timestamp &b);
		friend bool operator!=(const Timestamp &a, const Timestamp &b);
		friend bool operator<(const Timestamp &a, const Timestamp &b);
		friend bool operator<=(const Timestamp &a, const Timestamp &b);
		friend bool operator>(const Timestamp &a, const Timestamp &b);
		friend bool operator>=(const Timestamp &a, const Timestamp &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::partial_ordering operator<=>(const Timestamp &a, const Timestamp &b);
		#endif
		::moq::buf debug() const;
		static ::moq::Timestamp ZERO();
		static ::moq::result<::moq::Timestamp, ::moq::TimeOverflow> new_(::std::uint64_t value, const ::moq::Timescale &scale);
		static ::moq::Timestamp unscaled(::std::uint64_t key);
		static ::moq::result<::moq::Timestamp, ::moq::TimeOverflow> from_scale(::std::uint64_t value, ::std::uint64_t units_per_second);
		static ::moq::result<::moq::Timestamp, ::moq::TimeOverflow> from_secs(::std::uint64_t seconds);
		static ::moq::result<::moq::Timestamp, ::moq::TimeOverflow> from_millis(::std::uint64_t millis);
		static ::moq::result<::moq::Timestamp, ::moq::TimeOverflow> from_micros(::std::uint64_t micros);
		static ::moq::result<::moq::Timestamp, ::moq::TimeOverflow> from_nanos(::std::uint64_t nanos);
		::std::uint64_t value() const;
		::std::optional<::moq::Timescale> scale() const;
		bool is_zero() const;
		::moq::result<::moq::Timestamp, ::moq::TimeOverflow> convert(const ::moq::Timescale &new_scale) const;
		::moq::result<::moq::u128, ::moq::TimeOverflow> as_scale(const ::moq::Timescale &target) const;
		::moq::result<::std::uint64_t, ::moq::TimeOverflow> as_secs() const;
		::moq::result<::moq::u128, ::moq::TimeOverflow> as_millis() const;
		::moq::result<::moq::u128, ::moq::TimeOverflow> as_micros() const;
		::moq::result<::moq::u128, ::moq::TimeOverflow> as_nanos() const;
		::moq::result<::moq::Timestamp, ::moq::TimeOverflow> checked_add(const ::moq::Timestamp &rhs) const;
		::moq::result<::moq::Timestamp, ::moq::TimeOverflow> checked_sub(const ::moq::Timestamp &rhs) const;
	};
	struct setup_Token {
		::std::uint64_t kind;
		::moq::buf value;
		friend bool operator==(const setup_Token &a, const setup_Token &b);
		friend bool operator!=(const setup_Token &a, const setup_Token &b);
		::moq::buf debug() const;
	};
	class session_Limits {
		friend bool operator==(const session_Limits &a, const session_Limits &b);
		friend bool operator!=(const session_Limits &a, const session_Limits &b);
		::moq::buf debug() const;
		static session_Limits default_();
		::std::size_t announces_get() const;
		void announces_set(::std::size_t value);
	};
	class stats_Content {
		friend bool operator==(const stats_Content &a, const stats_Content &b);
		friend bool operator!=(const stats_Content &a, const stats_Content &b);
		::moq::buf debug() const;
		static stats_Content default_();
		::std::uint64_t bytes_get() const;
		void bytes_set(::std::uint64_t value);
		::std::uint64_t frames_get() const;
		void frames_set(::std::uint64_t value);
		::std::uint64_t groups_get() const;
		void groups_set(::std::uint64_t value);
	};
	class stats_Traffic {
		friend bool operator==(const stats_Traffic &a, const stats_Traffic &b);
		friend bool operator!=(const stats_Traffic &a, const stats_Traffic &b);
		::moq::buf debug() const;
		static stats_Traffic default_();
		::std::uint64_t announces_started_get() const;
		void announces_started_set(::std::uint64_t value);
		::std::uint64_t announces_ended_get() const;
		void announces_ended_set(::std::uint64_t value);
		::std::uint64_t announced_bytes_get() const;
		void announced_bytes_set(::std::uint64_t value);
		::std::uint64_t broadcasts_started_get() const;
		void broadcasts_started_set(::std::uint64_t value);
		::std::uint64_t broadcasts_ended_get() const;
		void broadcasts_ended_set(::std::uint64_t value);
		::std::uint64_t subscriptions_started_get() const;
		void subscriptions_started_set(::std::uint64_t value);
		::std::uint64_t subscriptions_ended_get() const;
		void subscriptions_ended_set(::std::uint64_t value);
		::std::uint64_t fetches_get() const;
		void fetches_set(::std::uint64_t value);
		::std::uint64_t bytes_get() const;
		void bytes_set(::std::uint64_t value);
		::std::uint64_t frames_get() const;
		void frames_set(::std::uint64_t value);
		::std::uint64_t groups_get() const;
		void groups_set(::std::uint64_t value);
		::std::uint64_t datagrams_get() const;
		void datagrams_set(::std::uint64_t value);
		::std::uint64_t inflated_get() const;
		void inflated_set(::std::uint64_t value);
		::moq::stats_Content stale_get() const;
		void stale_set(const ::moq::stats_Content &value);
		void add(const ::moq::stats_Traffic &other);
		::std::uint64_t active_broadcasts() const;
		::std::uint64_t active_subscriptions() const;
		bool is_idle() const;
	};
	class stats_Presence {
		friend bool operator==(const stats_Presence &a, const stats_Presence &b);
		friend bool operator!=(const stats_Presence &a, const stats_Presence &b);
		::moq::buf debug() const;
		static stats_Presence default_();
		::moq::stats_Refusals sessions_refused_get() const;
		void sessions_refused_set(const ::moq::stats_Refusals &value);
		::moq::stats_Failures sessions_failed_get() const;
		void sessions_failed_set(const ::moq::stats_Failures &value);
		::std::uint64_t requests_get() const;
		void requests_set(::std::uint64_t value);
		::std::uint64_t sessions_started_get() const;
		void sessions_started_set(::std::uint64_t value);
		::std::uint64_t sessions_ended_get() const;
		void sessions_ended_set(::std::uint64_t value);
		::std::uint64_t sent_bytes_get() const;
		void sent_bytes_set(::std::uint64_t value);
		::std::uint64_t received_bytes_get() const;
		void received_bytes_set(::std::uint64_t value);
		::std::uint64_t announces_peak_get() const;
		void announces_peak_set(::std::uint64_t value);
		::std::uint64_t subscriptions_peak_get() const;
		void subscriptions_peak_set(::std::uint64_t value);
		::std::uint64_t wire_tx_get() const;
		void wire_tx_set(::std::uint64_t value);
		::std::uint64_t wire_rx_get() const;
		void wire_rx_set(::std::uint64_t value);
		::std::uint64_t routes_refused_get() const;
		void routes_refused_set(::std::uint64_t value);
		::moq::stats_Routes routes_get() const;
		void routes_set(const ::moq::stats_Routes &value);
		void add(const ::moq::stats_Presence &other);
		::std::uint64_t active() const;
	};
	class stats_Activity {
		friend bool operator==(const stats_Activity &a, const stats_Activity &b);
		friend bool operator!=(const stats_Activity &a, const stats_Activity &b);
		::moq::buf debug() const;
		static stats_Activity default_();
		::std::uint64_t idle_get() const;
		void idle_set(::std::uint64_t value);
		::std::uint64_t active_get() const;
		void active_set(::std::uint64_t value);
	};
	class stats_Totals {
		friend bool operator==(const stats_Totals &a, const stats_Totals &b);
		friend bool operator!=(const stats_Totals &a, const stats_Totals &b);
		::moq::buf debug() const;
		static stats_Totals default_();
		::std::uint64_t tx_get() const;
		void tx_set(::std::uint64_t value);
		::std::uint64_t rx_get() const;
		void rx_set(::std::uint64_t value);
		::std::uint64_t tx_messages_get() const;
		void tx_messages_set(::std::uint64_t value);
		::std::uint64_t rx_messages_get() const;
		void rx_messages_set(::std::uint64_t value);
	};
	class stats_Failures {
		friend bool operator==(const stats_Failures &a, const stats_Failures &b);
		friend bool operator!=(const stats_Failures &a, const stats_Failures &b);
		::moq::buf debug() const;
		static stats_Failures default_();
		::std::uint64_t idle_timeout_get() const;
		void idle_timeout_set(::std::uint64_t value);
		::std::uint64_t transport_get() const;
		void transport_set(::std::uint64_t value);
		::std::uint64_t application_get() const;
		void application_set(::std::uint64_t value);
		::std::uint64_t expired_get() const;
		void expired_set(::std::uint64_t value);
		::std::uint64_t count(::moq::stats_Failure kind) const;
		void add(const ::moq::stats_Failures &other);
		bool regressed(const ::moq::stats_Failures &previous) const;
	};
	class stats_Refusals {
		friend bool operator==(const stats_Refusals &a, const stats_Refusals &b);
		friend bool operator!=(const stats_Refusals &a, const stats_Refusals &b);
		::moq::buf debug() const;
		static stats_Refusals default_();
		::std::uint64_t refused_get() const;
		void refused_set(::std::uint64_t value);
		::std::uint64_t unavailable_get() const;
		void unavailable_set(::std::uint64_t value);
		::std::uint64_t request_get() const;
		void request_set(::std::uint64_t value);
		::std::uint64_t forbidden_get() const;
		void forbidden_set(::std::uint64_t value);
		::std::uint64_t membership_get() const;
		void membership_set(::std::uint64_t value);
		::std::uint64_t expired_get() const;
		void expired_set(::std::uint64_t value);
		::std::uint64_t count(::moq::stats_RefusalKind kind) const;
		void add(const ::moq::stats_Refusals &other);
		bool regressed(const ::moq::stats_Refusals &previous) const;
	};
	class stats_RouteCloses {
		friend bool operator==(const stats_RouteCloses &a, const stats_RouteCloses &b);
		friend bool operator!=(const stats_RouteCloses &a, const stats_RouteCloses &b);
		::moq::buf debug() const;
		static stats_RouteCloses default_();
		::std::uint64_t stalled_get() const;
		void stalled_set(::std::uint64_t value);
		::std::uint64_t overrun_get() const;
		void overrun_set(::std::uint64_t value);
		::std::uint64_t malformed_get() const;
		void malformed_set(::std::uint64_t value);
		::std::uint64_t role_get() const;
		void role_set(::std::uint64_t value);
		::std::uint64_t unasked_get() const;
		void unasked_set(::std::uint64_t value);
		::std::uint64_t regressed_get() const;
		void regressed_set(::std::uint64_t value);
		::std::uint64_t other_get() const;
		void other_set(::std::uint64_t value);
		::std::uint64_t count(::moq::stats_RouteClose kind) const;
		::std::uint64_t total() const;
		void add(const ::moq::stats_RouteCloses &other);
		bool regressed(const ::moq::stats_RouteCloses &previous) const;
	};
	class stats_RouteRefusals {
		friend bool operator==(const stats_RouteRefusals &a, const stats_RouteRefusals &b);
		friend bool operator!=(const stats_RouteRefusals &a, const stats_RouteRefusals &b);
		::moq::buf debug() const;
		static stats_RouteRefusals default_();
		::std::uint64_t path_get() const;
		void path_set(::std::uint64_t value);
		::std::uint64_t scope_get() const;
		void scope_set(::std::uint64_t value);
		::std::uint64_t looped_get() const;
		void looped_set(::std::uint64_t value);
		::std::uint64_t count(::moq::stats_RouteRefusal kind) const;
		::std::uint64_t total() const;
		void add(const ::moq::stats_RouteRefusals &other);
		bool regressed(const ::moq::stats_RouteRefusals &previous) const;
	};
	class stats_Routes {
		friend bool operator==(const stats_Routes &a, const stats_Routes &b);
		friend bool operator!=(const stats_Routes &a, const stats_Routes &b);
		::moq::buf debug() const;
		static stats_Routes default_();
		::std::uint64_t changed_get() const;
		void changed_set(::std::uint64_t value);
		::std::uint64_t rewritten_get() const;
		void rewritten_set(::std::uint64_t value);
		::moq::stats_Withdrawals sent_get() const;
		void sent_set(const ::moq::stats_Withdrawals &value);
		::moq::stats_Withdrawals received_get() const;
		void received_set(const ::moq::stats_Withdrawals &value);
		::moq::stats_RouteRefusals refused_get() const;
		void refused_set(const ::moq::stats_RouteRefusals &value);
		::moq::stats_RouteCloses closed_get() const;
		void closed_set(const ::moq::stats_RouteCloses &value);
		void add(const ::moq::stats_Routes &other);
		bool regressed(const ::moq::stats_Routes &previous) const;
	};
	class stats_Withdrawals {
		friend bool operator==(const stats_Withdrawals &a, const stats_Withdrawals &b);
		friend bool operator!=(const stats_Withdrawals &a, const stats_Withdrawals &b);
		::moq::buf debug() const;
		static stats_Withdrawals default_();
		::std::uint64_t dead_get() const;
		void dead_set(::std::uint64_t value);
		::std::uint64_t drain_get() const;
		void drain_set(::std::uint64_t value);
		::std::uint64_t count(::moq::stats_Withdrawal kind) const;
		::std::uint64_t total() const;
		void add(const ::moq::stats_Withdrawals &other);
		bool regressed(const ::moq::stats_Withdrawals &previous) const;
	};
	class transport_ReceiveWindow {
		friend bool operator==(const transport_ReceiveWindow &a, const transport_ReceiveWindow &b);
		friend bool operator!=(const transport_ReceiveWindow &a, const transport_ReceiveWindow &b);
		::moq::buf debug() const;
		::std::optional<::std::uint64_t> connection_get() const;
		void connection_set(const ::std::optional<::std::uint64_t> &value);
		::std::uint64_t stream_get() const;
		void stream_set(::std::uint64_t value);
		static ::moq::transport_ReceiveWindow new_(const ::std::optional<::std::uint64_t> &connection, ::std::uint64_t stream);
	};
	class transport_Limits {
		friend bool operator==(const transport_Limits &a, const transport_Limits &b);
		friend bool operator!=(const transport_Limits &a, const transport_Limits &b);
		::moq::buf debug() const;
		static transport_Limits default_();
		::std::optional<::std::uint64_t> receive_window_get() const;
		void receive_window_set(const ::std::optional<::std::uint64_t> &value);
	};
	class bandwidth_Rate {
		friend bool operator==(const bandwidth_Rate &a, const bandwidth_Rate &b);
		friend bool operator!=(const bandwidth_Rate &a, const bandwidth_Rate &b);
		friend bool operator<(const bandwidth_Rate &a, const bandwidth_Rate &b);
		friend bool operator<=(const bandwidth_Rate &a, const bandwidth_Rate &b);
		friend bool operator>(const bandwidth_Rate &a, const bandwidth_Rate &b);
		friend bool operator>=(const bandwidth_Rate &a, const bandwidth_Rate &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const bandwidth_Rate &a, const bandwidth_Rate &b);
		#endif
		::moq::buf debug() const;
		static bandwidth_Rate default_();
		static ::moq::bandwidth_Rate ZERO();
		static ::moq::bandwidth_Rate from_bps(::std::uint64_t bps);
		static ::moq::bandwidth_Rate from_kbps(::std::uint64_t kbps);
		static ::moq::bandwidth_Rate from_mbps(::std::uint64_t mbps);
		::std::uint64_t as_bps() const;
		::moq::bandwidth_Rate scaled(double factor) const;
		::moq::bandwidth_Rate abs_diff(const ::moq::bandwidth_Rate &other) const;
	};
	struct datagram_Datagram {
		::std::uint64_t sequence;
		::moq::Timestamp timestamp;
		::moq::buf payload;
		friend bool operator==(const datagram_Datagram &a, const datagram_Datagram &b);
		friend bool operator!=(const datagram_Datagram &a, const datagram_Datagram &b);
		::moq::buf debug() const;
	};
	struct frame_Info {
		::std::uint64_t size;
		::moq::Timestamp timestamp;
		::moq::buf debug() const;
	};
	struct group_Info {
		::std::uint64_t start;
		friend bool operator==(const group_Info &a, const group_Info &b);
		friend bool operator!=(const group_Info &a, const group_Info &b);
		friend bool operator<(const group_Info &a, const group_Info &b);
		friend bool operator<=(const group_Info &a, const group_Info &b);
		friend bool operator>(const group_Info &a, const group_Info &b);
		friend bool operator>=(const group_Info &a, const group_Info &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const group_Info &a, const group_Info &b);
		#endif
		::moq::buf debug() const;
	};
	struct track_Delay {
		::std::optional<::moq::duration> min;
		::std::optional<::moq::duration> max;
		friend bool operator==(const track_Delay &a, const track_Delay &b);
		friend bool operator!=(const track_Delay &a, const track_Delay &b);
		::moq::buf debug() const;
		static track_Delay default_();
		::moq::track_Delay with_min(const ::std::optional<::moq::duration> &min_) const;
		::moq::track_Delay with_max(const ::std::optional<::moq::duration> &max_) const;
	};
	class track_Position {
		friend bool operator==(const track_Position &a, const track_Position &b);
		friend bool operator!=(const track_Position &a, const track_Position &b);
		friend bool operator<(const track_Position &a, const track_Position &b);
		friend bool operator<=(const track_Position &a, const track_Position &b);
		friend bool operator>(const track_Position &a, const track_Position &b);
		friend bool operator>=(const track_Position &a, const track_Position &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const track_Position &a, const track_Position &b);
		#endif
		::moq::buf debug() const;
		static track_Position default_();
		::std::uint64_t group_get() const;
		void group_set(::std::uint64_t value);
		::std::uint64_t frame_get() const;
		void frame_set(::std::uint64_t value);
		static ::moq::track_Position new_(::std::uint64_t group, ::std::uint64_t frame);
		static ::moq::track_Position group(::std::uint64_t group);
	};
	struct track_Anchor_Relative {
		::std::uint64_t _0;
	};
	struct track_Anchor_Oldest {
	};
	struct track_Anchor_Time {
		::std::uint64_t _0;
	};
	struct track_Range {
		::moq::track_Anchor start;
		::std::optional<::moq::track_Anchor> end;
		friend bool operator==(const track_Range &a, const track_Range &b);
		friend bool operator!=(const track_Range &a, const track_Range &b);
		friend bool operator<(const track_Range &a, const track_Range &b);
		friend bool operator<=(const track_Range &a, const track_Range &b);
		friend bool operator>(const track_Range &a, const track_Range &b);
		friend bool operator>=(const track_Range &a, const track_Range &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const track_Range &a, const track_Range &b);
		#endif
		::moq::buf debug() const;
		static ::moq::track_Range live();
		static ::moq::track_Range oldest();
	};
	class origin_Cost {
		friend bool operator==(const origin_Cost &a, const origin_Cost &b);
		friend bool operator!=(const origin_Cost &a, const origin_Cost &b);
		friend bool operator<(const origin_Cost &a, const origin_Cost &b);
		friend bool operator<=(const origin_Cost &a, const origin_Cost &b);
		friend bool operator>(const origin_Cost &a, const origin_Cost &b);
		friend bool operator>=(const origin_Cost &a, const origin_Cost &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const origin_Cost &a, const origin_Cost &b);
		#endif
		::moq::buf debug() const;
		static origin_Cost default_();
		static ::moq::origin_Cost new_(::std::uint64_t cost);
		::std::uint64_t value() const;
	};
	class origin_Priority {
		friend bool operator==(const origin_Priority &a, const origin_Priority &b);
		friend bool operator!=(const origin_Priority &a, const origin_Priority &b);
		friend bool operator<(const origin_Priority &a, const origin_Priority &b);
		friend bool operator<=(const origin_Priority &a, const origin_Priority &b);
		friend bool operator>(const origin_Priority &a, const origin_Priority &b);
		friend bool operator>=(const origin_Priority &a, const origin_Priority &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const origin_Priority &a, const origin_Priority &b);
		#endif
		::moq::buf debug() const;
		static origin_Priority default_();
		static ::moq::origin_Priority new_(::std::int8_t priority);
		::std::int8_t value() const;
	};
	struct SessionError_Cancel {
	};
	struct SessionError_Internal {
	};
	struct SessionError_Unauthorized {
	};
	struct SessionError_Expired {
	};
	struct SessionError_ProtocolViolation {
	};
	struct SessionError_TooManyRequests {
	};
	struct SessionError_GoawayTimeout {
	};
	struct SessionError_Timeout {
	};
	struct SessionError_Version {
	};
	struct SessionError_MalformedAuthToken {
	};
	struct SessionError_App {
		::std::uint16_t _0;
	};
	struct SessionError_Unknown {
		::std::uint32_t _0;
	};
	struct SessionError_Unsupported {
	};
	struct StreamError_Session {
		::moq::SessionError _0;
	};
	struct StreamError_Internal {
	};
	struct StreamError_Cancel {
	};
	struct StreamError_DeliveryTimeout {
	};
	struct StreamError_ControlTimeout {
	};
	struct StreamError_GoingAway {
	};
	struct StreamError_TooFarBehind {
	};
	struct StreamError_MalformedTrack {
	};
	struct StreamError_NotFound {
	};
	struct StreamError_Unroutable {
	};
	struct StreamError_Old {
	};
	struct StreamError_Evicted {
	};
	struct StreamError_WrongSize {
	};
	struct StreamError_FrameTooLarge {
	};
	struct StreamError_GroupTooLarge {
	};
	struct StreamError_TimestampMismatch {
	};
	struct StreamError_Unauthorized {
	};
	struct StreamError_App {
		::std::uint16_t _0;
	};
	struct StreamError_Unknown {
		::std::uint32_t _0;
	};
	struct transport_LimitsError_Unsupported {
	};
	struct transport_LimitsError_Refused {
		::moq::buf _0;
	};
	struct transport_memory_Error_Session {
		::std::uint32_t code;
		::moq::buf reason;
	};
	struct transport_memory_Error_Reset {
		::std::uint32_t _0;
	};
	struct transport_memory_Error_Stopped {
		::std::uint32_t _0;
	};
	struct transport_memory_Error_Closed {
	};
	inline bool operator==(const TimeOverflow &a, const TimeOverflow &b);
	inline bool operator==(const Timescale &a, const Timescale &b);
	inline bool operator<(const Timescale &a, const Timescale &b);
	inline bool operator<=(const Timescale &a, const Timescale &b);
	inline bool operator>(const Timescale &a, const Timescale &b);
	inline bool operator>=(const Timescale &a, const Timescale &b);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const Timescale &a, const Timescale &b);
	#endif
	inline bool operator==(const Timestamp &a, const Timestamp &b);
	inline bool operator<(const Timestamp &a, const Timestamp &b);
	inline bool operator<=(const Timestamp &a, const Timestamp &b);
	inline bool operator>(const Timestamp &a, const Timestamp &b);
	inline bool operator>=(const Timestamp &a, const Timestamp &b);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::partial_ordering operator<=>(const Timestamp &a, const Timestamp &b);
	#endif
	inline bool operator==(const setup_Token &a, const setup_Token &b);
	inline bool operator==(const session_Limits &a, const session_Limits &b);
	inline bool operator==(const stats_Content &a, const stats_Content &b);
	inline bool operator==(const stats_Traffic &a, const stats_Traffic &b);
	inline bool operator==(const stats_Presence &a, const stats_Presence &b);
	inline bool operator==(const stats_Activity &a, const stats_Activity &b);
	inline bool operator==(const stats_Totals &a, const stats_Totals &b);
	inline bool operator==(const stats_Failures &a, const stats_Failures &b);
	inline bool operator==(const stats_Refusals &a, const stats_Refusals &b);
	inline bool operator==(const stats_RouteCloses &a, const stats_RouteCloses &b);
	inline bool operator==(const stats_RouteRefusals &a, const stats_RouteRefusals &b);
	inline bool operator==(const stats_Routes &a, const stats_Routes &b);
	inline bool operator==(const stats_Withdrawals &a, const stats_Withdrawals &b);
	inline bool operator==(const transport_ReceiveWindow &a, const transport_ReceiveWindow &b);
	inline bool operator==(const transport_Limits &a, const transport_Limits &b);
	inline bool operator==(const bandwidth_Rate &a, const bandwidth_Rate &b);
	inline bool operator<(const bandwidth_Rate &a, const bandwidth_Rate &b);
	inline bool operator<=(const bandwidth_Rate &a, const bandwidth_Rate &b);
	inline bool operator>(const bandwidth_Rate &a, const bandwidth_Rate &b);
	inline bool operator>=(const bandwidth_Rate &a, const bandwidth_Rate &b);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const bandwidth_Rate &a, const bandwidth_Rate &b);
	#endif
	inline bool operator==(const datagram_Datagram &a, const datagram_Datagram &b);
	inline bool operator==(const group_Info &a, const group_Info &b);
	inline bool operator<(const group_Info &a, const group_Info &b);
	inline bool operator<=(const group_Info &a, const group_Info &b);
	inline bool operator>(const group_Info &a, const group_Info &b);
	inline bool operator>=(const group_Info &a, const group_Info &b);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const group_Info &a, const group_Info &b);
	#endif
	inline bool operator==(const track_Delay &a, const track_Delay &b);
	inline bool operator==(const track_Position &a, const track_Position &b);
	inline bool operator<(const track_Position &a, const track_Position &b);
	inline bool operator<=(const track_Position &a, const track_Position &b);
	inline bool operator>(const track_Position &a, const track_Position &b);
	inline bool operator>=(const track_Position &a, const track_Position &b);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const track_Position &a, const track_Position &b);
	#endif
	inline bool operator==(const ::moq::track_Anchor &a, const ::moq::track_Anchor &b);
	inline bool operator!=(const ::moq::track_Anchor &a, const ::moq::track_Anchor &b);
	inline bool operator<(const ::moq::track_Anchor &a, const ::moq::track_Anchor &b);
	inline bool operator<=(const ::moq::track_Anchor &a, const ::moq::track_Anchor &b);
	inline bool operator>(const ::moq::track_Anchor &a, const ::moq::track_Anchor &b);
	inline bool operator>=(const ::moq::track_Anchor &a, const ::moq::track_Anchor &b);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const ::moq::track_Anchor &a, const ::moq::track_Anchor &b);
	#endif
	inline ::moq::buf track_Anchor_debug(const ::moq::track_Anchor &self);
	inline bool operator==(const track_Range &a, const track_Range &b);
	inline bool operator<(const track_Range &a, const track_Range &b);
	inline bool operator<=(const track_Range &a, const track_Range &b);
	inline bool operator>(const track_Range &a, const track_Range &b);
	inline bool operator>=(const track_Range &a, const track_Range &b);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const track_Range &a, const track_Range &b);
	#endif
	inline bool operator==(const origin_Cost &a, const origin_Cost &b);
	inline bool operator<(const origin_Cost &a, const origin_Cost &b);
	inline bool operator<=(const origin_Cost &a, const origin_Cost &b);
	inline bool operator>(const origin_Cost &a, const origin_Cost &b);
	inline bool operator>=(const origin_Cost &a, const origin_Cost &b);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const origin_Cost &a, const origin_Cost &b);
	#endif
	inline bool operator==(const origin_Priority &a, const origin_Priority &b);
	inline bool operator<(const origin_Priority &a, const origin_Priority &b);
	inline bool operator<=(const origin_Priority &a, const origin_Priority &b);
	inline bool operator>(const origin_Priority &a, const origin_Priority &b);
	inline bool operator>=(const origin_Priority &a, const origin_Priority &b);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(const origin_Priority &a, const origin_Priority &b);
	#endif
	inline ::moq::buf EncodeError_to_string(::moq::EncodeError self);
	inline ::moq::buf EncodeError_debug(::moq::EncodeError self);
	inline bool operator==(const ::moq::SessionError &a, const ::moq::SessionError &b);
	inline bool operator!=(const ::moq::SessionError &a, const ::moq::SessionError &b);
	inline ::moq::buf SessionError_to_string(const ::moq::SessionError &self);
	inline ::moq::buf SessionError_debug(const ::moq::SessionError &self);
	inline ::std::uint32_t SessionError_to_code(const ::moq::SessionError &self);
	inline ::moq::SessionError SessionError_from_code(::std::uint32_t code);
	inline bool operator==(const ::moq::StreamError &a, const ::moq::StreamError &b);
	inline bool operator!=(const ::moq::StreamError &a, const ::moq::StreamError &b);
	inline ::moq::buf StreamError_to_string(const ::moq::StreamError &self);
	inline ::moq::buf StreamError_debug(const ::moq::StreamError &self);
	inline ::std::uint32_t StreamError_to_code(const ::moq::StreamError &self);
	inline ::moq::StreamError StreamError_from_code(::std::uint32_t code);
	inline bool operator==(::moq::Retry a, ::moq::Retry b);
	inline bool operator!=(::moq::Retry a, ::moq::Retry b);
	inline ::moq::buf Retry_debug(::moq::Retry self);
	inline bool operator==(::moq::tokio_Restart a, ::moq::tokio_Restart b);
	inline bool operator!=(::moq::tokio_Restart a, ::moq::tokio_Restart b);
	inline ::moq::buf tokio_Restart_debug(::moq::tokio_Restart self);
	inline tokio_Restart tokio_Restart_default();
	inline bool operator==(::moq::path_Error a, ::moq::path_Error b);
	inline bool operator!=(::moq::path_Error a, ::moq::path_Error b);
	inline ::moq::buf path_Error_to_string(::moq::path_Error self);
	inline ::moq::buf path_Error_debug(::moq::path_Error self);
	inline bool operator==(::moq::setup_Role a, ::moq::setup_Role b);
	inline bool operator!=(::moq::setup_Role a, ::moq::setup_Role b);
	inline ::moq::buf setup_Role_to_string(::moq::setup_Role self);
	inline ::moq::buf setup_Role_debug(::moq::setup_Role self);
	inline ::moq::buf setup_Role_as_str(::moq::setup_Role self);
	inline bool operator==(::moq::reconnect_Verdict a, ::moq::reconnect_Verdict b);
	inline bool operator!=(::moq::reconnect_Verdict a, ::moq::reconnect_Verdict b);
	inline ::moq::buf reconnect_Verdict_debug(::moq::reconnect_Verdict self);
	inline bool operator==(::moq::reconnect_ConfigError a, ::moq::reconnect_ConfigError b);
	inline bool operator!=(::moq::reconnect_ConfigError a, ::moq::reconnect_ConfigError b);
	inline ::moq::buf reconnect_ConfigError_to_string(::moq::reconnect_ConfigError self);
	inline ::moq::buf reconnect_ConfigError_debug(::moq::reconnect_ConfigError self);
	inline bool operator==(::moq::reconnect_Redirect a, ::moq::reconnect_Redirect b);
	inline bool operator!=(::moq::reconnect_Redirect a, ::moq::reconnect_Redirect b);
	inline ::moq::buf reconnect_Redirect_debug(::moq::reconnect_Redirect self);
	inline reconnect_Redirect reconnect_Redirect_default();
	inline bool operator==(::moq::stats_Role a, ::moq::stats_Role b);
	inline bool operator!=(::moq::stats_Role a, ::moq::stats_Role b);
	inline ::moq::buf stats_Role_debug(::moq::stats_Role self);
	inline ::moq::buf stats_Role_as_str(::moq::stats_Role self);
	inline bool operator==(::moq::stats_Message a, ::moq::stats_Message b);
	inline bool operator!=(::moq::stats_Message a, ::moq::stats_Message b);
	inline ::moq::buf stats_Message_debug(::moq::stats_Message self);
	inline ::moq::buf stats_Message_as_str(::moq::stats_Message self);
	inline bool operator==(::moq::stats_Failure a, ::moq::stats_Failure b);
	inline bool operator!=(::moq::stats_Failure a, ::moq::stats_Failure b);
	inline ::moq::buf stats_Failure_debug(::moq::stats_Failure self);
	inline ::std::vector<::moq::stats_Failure> stats_Failure_ALL();
	inline ::moq::buf stats_Failure_as_str(::moq::stats_Failure self);
	inline bool operator==(::moq::stats_RefusalKind a, ::moq::stats_RefusalKind b);
	inline bool operator!=(::moq::stats_RefusalKind a, ::moq::stats_RefusalKind b);
	inline ::moq::buf stats_RefusalKind_debug(::moq::stats_RefusalKind self);
	inline ::std::vector<::moq::stats_RefusalKind> stats_RefusalKind_ALL();
	inline ::moq::buf stats_RefusalKind_as_str(::moq::stats_RefusalKind self);
	inline bool operator==(::moq::stats_RouteClose a, ::moq::stats_RouteClose b);
	inline bool operator!=(::moq::stats_RouteClose a, ::moq::stats_RouteClose b);
	inline ::moq::buf stats_RouteClose_debug(::moq::stats_RouteClose self);
	inline ::std::vector<::moq::stats_RouteClose> stats_RouteClose_ALL();
	inline ::moq::buf stats_RouteClose_as_str(::moq::stats_RouteClose self);
	inline bool operator==(::moq::stats_RouteRefusal a, ::moq::stats_RouteRefusal b);
	inline bool operator!=(::moq::stats_RouteRefusal a, ::moq::stats_RouteRefusal b);
	inline ::moq::buf stats_RouteRefusal_debug(::moq::stats_RouteRefusal self);
	inline ::std::vector<::moq::stats_RouteRefusal> stats_RouteRefusal_ALL();
	inline ::moq::buf stats_RouteRefusal_as_str(::moq::stats_RouteRefusal self);
	inline bool operator==(::moq::stats_Withdrawal a, ::moq::stats_Withdrawal b);
	inline bool operator!=(::moq::stats_Withdrawal a, ::moq::stats_Withdrawal b);
	inline ::moq::buf stats_Withdrawal_debug(::moq::stats_Withdrawal self);
	inline ::std::vector<::moq::stats_Withdrawal> stats_Withdrawal_ALL();
	inline ::moq::buf stats_Withdrawal_as_str(::moq::stats_Withdrawal self);
	inline bool operator==(::moq::transport_Address a, ::moq::transport_Address b);
	inline bool operator!=(::moq::transport_Address a, ::moq::transport_Address b);
	inline ::moq::buf transport_Address_debug(::moq::transport_Address self);
	inline bool operator==(const ::moq::transport_LimitsError &a, const ::moq::transport_LimitsError &b);
	inline bool operator!=(const ::moq::transport_LimitsError &a, const ::moq::transport_LimitsError &b);
	inline ::moq::buf transport_LimitsError_to_string(const ::moq::transport_LimitsError &self);
	inline ::moq::buf transport_LimitsError_debug(const ::moq::transport_LimitsError &self);
	inline bool operator==(::moq::transport_CloseKind a, ::moq::transport_CloseKind b);
	inline bool operator!=(::moq::transport_CloseKind a, ::moq::transport_CloseKind b);
	inline ::moq::buf transport_CloseKind_debug(::moq::transport_CloseKind self);
	inline transport_CloseKind transport_CloseKind_default();
	inline bool operator==(::moq::broadcast_Kind a, ::moq::broadcast_Kind b);
	inline bool operator!=(::moq::broadcast_Kind a, ::moq::broadcast_Kind b);
	inline ::moq::buf broadcast_Kind_debug(::moq::broadcast_Kind self);
	inline bool operator==(::moq::track_Compression a, ::moq::track_Compression b);
	inline bool operator!=(::moq::track_Compression a, ::moq::track_Compression b);
	inline ::moq::buf track_Compression_debug(::moq::track_Compression self);
	inline track_Compression track_Compression_default();
	inline bool operator==(::moq::track_Order a, ::moq::track_Order b);
	inline bool operator!=(::moq::track_Order a, ::moq::track_Order b);
	inline ::moq::buf track_Order_debug(::moq::track_Order self);
	inline track_Order track_Order_default();
	inline bool operator==(::moq::origin_Link a, ::moq::origin_Link b);
	inline bool operator!=(::moq::origin_Link a, ::moq::origin_Link b);
	inline ::moq::buf origin_Link_debug(::moq::origin_Link self);
	inline origin_Link origin_Link_default();
	inline bool operator==(::moq::origin_RouteKind a, ::moq::origin_RouteKind b);
	inline bool operator!=(::moq::origin_RouteKind a, ::moq::origin_RouteKind b);
	inline ::moq::buf origin_RouteKind_debug(::moq::origin_RouteKind self);
	inline ::moq::buf transport_memory_Error_to_string(const ::moq::transport_memory_Error &self);
	inline ::moq::buf transport_memory_Error_debug(const ::moq::transport_memory_Error &self);
	inline bool operator==(::moq::transport_poll_Acked a, ::moq::transport_poll_Acked b);
	inline bool operator!=(::moq::transport_poll_Acked a, ::moq::transport_poll_Acked b);
	inline ::moq::buf transport_poll_Acked_debug(::moq::transport_poll_Acked self);
	inline ::moq::buf IMPLEMENTATION();
	inline ::std::vector<::moq::buf> ALPNS();
	inline ::moq::duration server_DEFAULT_TIMEOUT();
	inline ::moq::time_run_Driver_future time_run_Driver(::moq::Driver driver);
	inline ::moq::time_run_origin_Driver_future time_run_origin_Driver(::moq::origin_Driver driver);
	inline ::moq::result<::std::uint64_t, ::moq::DecodeError> varint_decode_quic(::moq::buf &r);
	inline ::moq::result<void, ::moq::EncodeError> varint_encode_quic(::std::uint64_t value, ::moq::buf &w);
	inline ::moq::duration cache_DEFAULT_EXPIRY();
	inline ::std::tuple<::moq::transport_memory_Session, ::moq::transport_memory_Session> transport_memory_pair(::moq::transport_memory_Config config);
}
namespace std {
	template <> struct hash<::moq::Instance> {
		size_t operator()(const ::moq::Instance &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::moq::Path> {
		size_t operator()(const ::moq::Path &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::moq::Version> {
		size_t operator()(const ::moq::Version &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::moq::path_Relative> {
		size_t operator()(const ::moq::path_Relative &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::moq::stats_Tier> {
		size_t operator()(const ::moq::stats_Tier &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::moq::track_Refresh> {
		size_t operator()(const ::moq::track_Refresh &value) const noexcept;
	};
}
#endif

// pattern.hpp
#ifndef MOQ_HPP_CRATE3
#define MOQ_HPP_CRATE3
#include "rt.hpp"
namespace moq {
	enum class IntersectionError {
		too_many_patterns = 0,
	};
	inline constexpr ::std::size_t Pattern_MAX_SEGMENTS = MOQ_PATTERN_MAX_SEGMENTS;
	inline constexpr ::std::size_t Pattern_MAX_INTERSECTIONS = MOQ_PATTERN_MAX_INTERSECTIONS;
	class RUST2_FFI_CONSUMABLE Pattern {
		RUST2_FFI_UNCONSUMED Pattern(Pattern &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Pattern &operator=(Pattern &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Pattern(const Pattern &other) = default;
		Pattern &operator=(const Pattern &other) = default;
		friend bool operator==(const Pattern &a, const Pattern &b);
		friend bool operator!=(const Pattern &a, const Pattern &b);
		friend bool operator<(const Pattern &a, const Pattern &b);
		friend bool operator<=(const Pattern &a, const Pattern &b);
		friend bool operator>(const Pattern &a, const Pattern &b);
		friend bool operator>=(const Pattern &a, const Pattern &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const Pattern &a, const Pattern &b);
		#endif
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static Pattern default_();
		static ::moq::result<::moq::Pattern, ::moq::InvalidPattern> new_(const ::std::vector<::moq::Segment> &segments);
		static ::moq::Pattern all();
		static ::moq::result<::moq::Pattern, ::moq::InvalidPattern> literal(::moq::input path);
		static ::moq::result<::moq::Pattern, ::moq::InvalidPattern> subtree(::moq::input path);
		RUST2_FFI_CALLABLE ::moq::buf as_str() const;
		RUST2_FFI_CALLABLE ::std::vector<::moq::Segment> segments() const;
		RUST2_FFI_CALLABLE ::moq::buf head() const;
		RUST2_FFI_CALLABLE bool is_literal() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> as_prefix() const;
		RUST2_FFI_CALLABLE bool has_globstar() const;
		RUST2_FFI_CALLABLE bool matches(::moq::input path) const;
		RUST2_FFI_CALLABLE bool contains(const ::moq::Pattern &other) const;
		RUST2_FFI_CALLABLE bool overlaps(const ::moq::Pattern &other) const;
		RUST2_FFI_CALLABLE ::moq::Specificity specificity() const;
		RUST2_FFI_CALLABLE ::moq::Patterns rebase(::moq::input root) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Patterns, ::moq::IntersectionError> intersect(const ::moq::Pattern &other) const;
		RUST2_FFI_CALLABLE ::std::optional<::std::vector<::moq::Pattern>> captures(const ::moq::Pattern &matched) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Pattern, ::moq::InvalidPattern> rooted(::moq::input root) const;
	};
	class RUST2_FFI_CONSUMABLE Patterns {
		RUST2_FFI_UNCONSUMED Patterns(Patterns &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Patterns &operator=(Patterns &&other) noexcept = default;
		RUST2_FFI_CALLABLE Patterns clone() const;
		friend bool operator==(const Patterns &a, const Patterns &b);
		friend bool operator!=(const Patterns &a, const Patterns &b);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static Patterns default_();
		static ::moq::Patterns new_();
		RUST2_FFI_CALLABLE bool insert(::moq::Pattern pattern);
		RUST2_FFI_CALLABLE bool matches(::moq::input path) const;
		RUST2_FFI_CALLABLE bool contains(const ::moq::Pattern &pattern) const;
		RUST2_FFI_CALLABLE bool covers(const ::moq::Patterns &other) const;
		RUST2_FFI_CALLABLE bool overlaps(const ::moq::Pattern &pattern) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Patterns, ::moq::IntersectionError> intersect(const ::moq::Patterns &other) const;
		RUST2_FFI_CALLABLE ::moq::Patterns rebase(::moq::input root) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Patterns, ::moq::InvalidPattern> rooted(::moq::input root) const;
		RUST2_FFI_CALLABLE ::std::vector<::moq::Pattern> iter() const;
		RUST2_FFI_CALLABLE ::std::size_t len() const;
		RUST2_FFI_CALLABLE bool is_empty() const;
	};
	class RUST2_FFI_CONSUMABLE Specificity {
		RUST2_FFI_UNCONSUMED Specificity(Specificity &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Specificity &operator=(Specificity &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Specificity(const Specificity &other) = default;
		Specificity &operator=(const Specificity &other) = default;
		friend bool operator==(const Specificity &a, const Specificity &b);
		friend bool operator!=(const Specificity &a, const Specificity &b);
		friend bool operator<(const Specificity &a, const Specificity &b);
		friend bool operator<=(const Specificity &a, const Specificity &b);
		friend bool operator>(const Specificity &a, const Specificity &b);
		friend bool operator>=(const Specificity &a, const Specificity &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const Specificity &a, const Specificity &b);
		#endif
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	struct InvalidPattern_EmptySegment {
	};
	struct InvalidPattern_InvalidSegment {
		::moq::buf _0;
	};
	struct InvalidPattern_MultipleGlobstars {
	};
	struct InvalidPattern_TooManySegments {
	};
	struct Segment_Literal {
		::moq::buf _0;
	};
	struct Segment_Wildcard {
	};
	struct Segment_Partial {
		::moq::buf prefix;
		::moq::buf suffix;
	};
	struct Segment_Globstar {
	};
	inline bool operator==(const ::moq::InvalidPattern &a, const ::moq::InvalidPattern &b);
	inline bool operator!=(const ::moq::InvalidPattern &a, const ::moq::InvalidPattern &b);
	inline ::moq::buf InvalidPattern_to_string(const ::moq::InvalidPattern &self);
	inline ::moq::buf InvalidPattern_debug(const ::moq::InvalidPattern &self);
	inline bool operator==(::moq::IntersectionError a, ::moq::IntersectionError b);
	inline bool operator!=(::moq::IntersectionError a, ::moq::IntersectionError b);
	inline ::moq::buf IntersectionError_to_string(::moq::IntersectionError self);
	inline ::moq::buf IntersectionError_debug(::moq::IntersectionError self);
	inline bool operator==(const ::moq::Segment &a, const ::moq::Segment &b);
	inline bool operator!=(const ::moq::Segment &a, const ::moq::Segment &b);
	inline ::moq::buf Segment_to_string(const ::moq::Segment &self);
	inline ::moq::buf Segment_debug(const ::moq::Segment &self);
	inline ::moq::buf escape(::moq::input content);
	inline ::moq::buf unescape(::moq::input segment);
}
namespace std {
	template <> struct hash<::moq::Pattern> {
		size_t operator()(const ::moq::Pattern &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::moq::Patterns> {
		size_t operator()(const ::moq::Patterns &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::moq::Specificity> {
		size_t operator()(const ::moq::Specificity &value) const noexcept;
	};
}
#endif

// qmux.hpp
#ifndef MOQ_HPP_CRATE14
#define MOQ_HPP_CRATE14
#include "rt.hpp"
#include "kio.hpp"
#include "net.hpp"
#include "quic.hpp"
#include "rustlspkitypes.hpp"
namespace moq {
	class RUST2_FFI_CONSUMABLE qmux_Session {
		RUST2_FFI_UNCONSUMED qmux_Session(qmux_Session &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		qmux_Session &operator=(qmux_Session &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED qmux_Session(const qmux_Session &other) = default;
		qmux_Session &operator=(const qmux_Session &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> protocol() const;
		RUST2_FFI_CALLABLE ::moq::moq_quic_connection_mux_Version version() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> set_limits(const ::moq::moq_quic_Limits &limits) const;
		RUST2_FFI_CALLABLE ::std::optional<::std::vector<::moq::rustls_pki_types_CertificateDer>> peer_certificates() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> server_name() const;
	};
}
#endif

// quic.hpp
#ifndef MOQ_HPP_CRATE13
#define MOQ_HPP_CRATE13
#include "rt.hpp"
namespace moq {
	enum class moq_quic_connection_mux_Version {
		q_mux00 = 0,
		q_mux01 = 1,
		q_mux02 = 2,
	};
	class RUST2_FFI_CONSUMABLE moq_quic_Limits {
		RUST2_FFI_UNCONSUMED moq_quic_Limits(moq_quic_Limits &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_quic_Limits &operator=(moq_quic_Limits &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_quic_Limits clone() const;
		friend bool operator==(const moq_quic_Limits &a, const moq_quic_Limits &b);
		friend bool operator!=(const moq_quic_Limits &a, const moq_quic_Limits &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_quic_Limits default_();
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> max_streams_get() const;
		RUST2_FFI_CALLABLE void max_streams_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> receive_window_get() const;
		RUST2_FFI_CALLABLE void receive_window_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> stream_receive_window_get() const;
		RUST2_FFI_CALLABLE void stream_receive_window_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> send_window_get() const;
		RUST2_FFI_CALLABLE void send_window_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_quic_Rate> receive_rate_get() const;
		RUST2_FFI_CALLABLE void receive_rate_set(const ::std::optional<::moq::moq_quic_Rate> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_quic_Rate> send_rate_get() const;
		RUST2_FFI_CALLABLE void send_rate_set(const ::std::optional<::moq::moq_quic_Rate> &value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> validate() const;
	};
	struct moq_quic_Rate_Unlimited {
	};
	struct moq_quic_Rate_BytesPerSecond {
		::std::uint64_t _0;
	};
	inline bool operator==(const ::moq::moq_quic_Rate &a, const ::moq::moq_quic_Rate &b);
	inline bool operator!=(const ::moq::moq_quic_Rate &a, const ::moq::moq_quic_Rate &b);
	inline ::moq::buf moq_quic_Rate_debug(const ::moq::moq_quic_Rate &self);
	inline bool operator==(::moq::moq_quic_connection_mux_Version a, ::moq::moq_quic_connection_mux_Version b);
	inline bool operator!=(::moq::moq_quic_connection_mux_Version a, ::moq::moq_quic_connection_mux_Version b);
	inline ::moq::buf moq_quic_connection_mux_Version_debug(::moq::moq_quic_connection_mux_Version self);
	inline ::std::vector<::moq::moq_quic_connection_mux_Version> moq_quic_connection_mux_Version_ALL();
	inline bool moq_quic_connection_mux_Version_uses_records(::moq::moq_quic_connection_mux_Version self);
	inline ::moq::buf moq_quic_connection_mux_Version_alpn(::moq::moq_quic_connection_mux_Version self);
	inline ::moq::buf moq_quic_connection_mux_Version_prefix(::moq::moq_quic_connection_mux_Version self);
}
#endif

// rt.hpp
#ifndef MOQ_HPP_RT
#define MOQ_HPP_RT
#include "../moq.h"
#include <array>
#include <atomic>
#include <condition_variable>
#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <functional>
#include <initializer_list>
#include <ios>
#include <memory>
#include <mutex>
#include <optional>
#include <string_view>
#include <thread>
#include <tuple>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>
#if defined(__cpp_impl_coroutine) && __cpp_impl_coroutine >= 201902L && __has_include(<coroutine>)
#include <coroutine>
#endif
#if defined(__cpp_impl_three_way_comparison) && __cpp_impl_three_way_comparison >= 201907L && __has_include(<compare>)
#include <compare>
#endif
namespace moq {
	#if defined(__clang__)
	#define RUST2_FFI_OWNER(T) [[gsl::Owner(T)]]
	#define RUST2_FFI_POINTER(T) [[gsl::Pointer(T)]]
	#define RUST2_FFI_LIFETIMEBOUND [[clang::lifetimebound]]
	#define RUST2_FFI_CONSUMABLE [[clang::consumable(unconsumed)]]
	#define RUST2_FFI_CALLABLE [[clang::callable_when("unconsumed", "unknown")]]
	#define RUST2_FFI_CONSUMES [[clang::callable_when("unconsumed", "unknown"), clang::set_typestate(consumed)]]
	#define RUST2_FFI_UNCONSUMED [[clang::return_typestate(unconsumed)]]
	#define RUST2_FFI_MOVED_FROM [[clang::return_typestate(consumed)]]
	#else
	#define RUST2_FFI_OWNER(T)
	#define RUST2_FFI_POINTER(T)
	#define RUST2_FFI_LIFETIMEBOUND
	#define RUST2_FFI_CONSUMABLE
	#define RUST2_FFI_CALLABLE
	#define RUST2_FFI_CONSUMES
	#define RUST2_FFI_UNCONSUMED
	#define RUST2_FFI_MOVED_FROM
	#endif
	class buf;
	class input;
	template <class T> class borrow {
		explicit borrow(T &object) noexcept : object_(object);
		borrow(const borrow &) = delete;
		borrow &operator=(const borrow &) = delete;
		T *operator->() const noexcept;
	};
	class RUST2_FFI_POINTER(::std::uint8_t) view {
		view() noexcept : data_(detail::bytes(nullptr, 0)), size_(0);
		view(const void *data, ::std::size_t size) noexcept : data_(detail::bytes(data, size)), size_(size);
		view(const char *text) noexcept : view(text, ::std::strlen(text));
		template <class R, class = ::std::enable_if_t<sizeof(*::std::declval<const R &>().data()) == 1 && !::std::is_pointer_v<::std::decay_t<R>>>, class = decltype(::std::declval<const R &>().size())> view(const R &range RUST2_FFI_LIFETIMEBOUND) noexcept : view(range.data(), range.size());
		const ::std::uint8_t *data() const noexcept;
		::std::size_t size() const noexcept;
		bool empty() const noexcept;
		const ::std::uint8_t *begin() const noexcept;
		const ::std::uint8_t *end() const noexcept;
		::std::string_view str() const noexcept;
	};
	#if defined(__GNUC__) && !defined(__clang__)
	#pragma GCC diagnostic push
	#pragma GCC diagnostic ignored "-Winit-list-lifetime"
	#endif
	template <class T> class RUST2_FFI_POINTER(T) slice {
		slice() noexcept : data_(nullptr), size_(0);
		slice(const T *data, ::std::size_t size) noexcept : data_(data), size_(size);
		template <::std::size_t N> slice(const T (&array RUST2_FFI_LIFETIMEBOUND)[N]) noexcept : data_(array), size_(N);
		slice(::std::initializer_list<T> list RUST2_FFI_LIFETIMEBOUND) noexcept : data_(list.begin()), size_(list.size());
		template <class R, class = ::std::enable_if_t<::std::is_same_v< ::std::remove_cv_t<::std::remove_pointer_t<decltype(::std::declval<const R &>().data())>>, T>>, class = decltype(::std::declval<const R &>().size())> slice(const R &range RUST2_FFI_LIFETIMEBOUND) noexcept : data_(range.data()), size_(range.size());
		const T *data() const noexcept;
		::std::size_t size() const noexcept;
		bool empty() const noexcept;
		const T *begin() const noexcept;
		const T *end() const noexcept;
	};
	#if defined(__GNUC__) && !defined(__clang__)
	#pragma GCC diagnostic pop
	#endif
	class [[nodiscard]] RUST2_FFI_OWNER(::std::uint8_t) buf {
		buf() noexcept;
		explicit buf(view bytes) : buf();
		buf(const buf &other) : buf();
		buf(buf &&other) noexcept : raw_(::std::exchange(other.raw_, detail::c_buf{nullptr, 0, nullptr}));
		buf &operator=(const buf &other);
		buf &operator=(buf &&other) noexcept;
		~buf();
		const ::std::uint8_t *data() const noexcept RUST2_FFI_LIFETIMEBOUND;
		::std::size_t size() const noexcept;
		bool empty() const noexcept;
		const ::std::uint8_t *begin() const noexcept;
		const ::std::uint8_t *end() const noexcept;
		::std::string_view str() const noexcept RUST2_FFI_LIFETIMEBOUND;
	};
	class input {
		input(const char *text) noexcept : input(view(text));
		template <class R, class = ::std::enable_if_t<::std::is_constructible_v<view, const R &> && !::std::is_pointer_v<::std::decay_t<R>>>> input(const R &range RUST2_FFI_LIFETIMEBOUND) noexcept;
		input(buf &&bytes) noexcept;
		input(input &&other) noexcept : raw_(::std::exchange(other.raw_, detail::c_buf{nullptr, 0, nullptr})), held_(::std::move(other.held_));
		input(const input &) = delete;
		input &operator=(const input &) = delete;
		input &operator=(input &&) = delete;
		~input();
	};
	template <class T> class RUST2_FFI_POINTER(T) items {
		items(const ::std::vector<T> &vector RUST2_FFI_LIFETIMEBOUND) noexcept : data_(vector.data()), size_(vector.size());
		items(::std::initializer_list<T> list) : converted_(list), data_(converted_.data()), size_(converted_.size());
		template <class R, class = ::std::enable_if_t<!::std::is_same_v<::std::decay_t<R>, ::std::vector<T>> && !::std::is_same_v<::std::decay_t<R>, items> && detail::lasting<const R, T> && ::std::is_convertible_v<decltype(*::std::begin(::std::declval<const R &>())), T>>> items(const R &range RUST2_FFI_LIFETIMEBOUND);
		template <class U, class = ::std::enable_if_t<!::std::is_same_v<U, T> && ::std::is_convertible_v<U &&, T>>> items(::std::vector<U> &&vector RUST2_FFI_LIFETIMEBOUND);
		items(const items &) = delete;
		items &operator=(const items &) = delete;
		items(items &&other) noexcept = default;
		const T *begin() const noexcept;
		const T *end() const noexcept;
		::std::size_t size() const noexcept;
		bool empty() const noexcept;
	};
	template <class T> class taken {
		taken() noexcept = default;
		taken(::std::vector<T> &&vector) noexcept : items_(::std::move(vector));
		template <class R, class = ::std::enable_if_t<!::std::is_reference_v<R> && !::std::is_same_v<R, ::std::vector<T>> && !::std::is_same_v<R, taken> && detail::lasting<R, T> && ::std::is_convertible_v<decltype(::std::move(*::std::begin(::std::declval<R &>()))), T>>> taken(R &&range RUST2_FFI_LIFETIMEBOUND);
		T *begin() noexcept;
		T *end() noexcept;
		::std::size_t size() const noexcept;
		bool empty() const noexcept;
	};
	template <class T, class E> class [[nodiscard]] result {
		static result ok(T value);
		static result err(E error);
		bool has_value() const noexcept;
		explicit operator bool() const noexcept;
		T &value() &;
		const T &value() const &;
		T &&value() &&;
		T &operator*() &;
		const T &operator*() const &;
		T &&operator*() &&;
		T *operator->();
		const T *operator->() const;
		E &error() &;
		const E &error() const &;
		E &&error() &&;
	};
	template <class E> class [[nodiscard]] result<void, E> {
		static result ok();
		static result err(E error);
		bool has_value() const noexcept;
		explicit operator bool() const noexcept;
		void value() const noexcept;
		E &error() &;
		const E &error() const &;
		E &&error() &&;
	};
	class waker {
		template <class F, class = ::std::enable_if_t<!::std::is_same_v<::std::decay_t<F>, waker>>> explicit waker(F f) : node_(new callable<F>(::std::move(f)));
		waker(const waker &other) noexcept : node_(other.node_);
		waker &operator=(const waker &other) noexcept;
		~waker();
		void wake() const noexcept;
	};
	template <class T> using poll_t = ::std::conditional_t<::std::is_void_v<T>, bool, ::std::optional<T>>;
	class task;
	class [[nodiscard]] task {
		task() noexcept = default;
		task(task &&other) noexcept : node_(::std::exchange(other.node_, nullptr));
		task &operator=(task &&other) noexcept;
		~task();
	};
	template <class Op> class [[nodiscard]] future {
		using output = typename Op::output;
		future(future &&other) noexcept : raw_(::std::exchange(other.raw_, nullptr)), last_(::std::exchange(other.last_, nullptr));
		future &operator=(future &&other) noexcept;
		~future();
		poll_t<output> poll(const waker &w);
		output get() &&;
		template <class Exec, class F> task then(Exec exec, F f) &&;
		template <class Exec> auto via(Exec exec) &&;
	};
	using u128 = ::moq_u128;
	using duration = ::moq_duration;
	using socket_addr = ::moq_socket_addr;
	using socket_addr_v4 = ::moq_socket_addr_v4;
	using socket_addr_v6 = ::moq_socket_addr_v6;
	using ip_addr = ::moq_ip_addr;
	using ipv4_addr = ::moq_ipv4_addr;
	using ipv6_addr = ::moq_ipv6_addr;
	using bound = ::moq_bound;
	using bound_u64 = ::moq_bound_u64;
	using bounds_u64 = ::moq_bounds_u64;
	using range_u64 = ::moq_range_u64;
	enum class EncodeError;
	enum class Retry;
	enum class IntersectionError;
	enum class clock_MissedTickBehavior;
	enum class tokio_ConnectError;
	enum class tokio_Restart;
	enum class tokio_Status;
	enum class tokio_Transport;
	enum class path_Error;
	enum class setup_Role;
	enum class reconnect_Verdict;
	enum class reconnect_ConfigError;
	enum class reconnect_Redirect;
	enum class stats_Role;
	enum class stats_Message;
	enum class stats_Failure;
	enum class stats_RefusalKind;
	enum class stats_RouteClose;
	enum class stats_RouteRefusal;
	enum class stats_Withdrawal;
	enum class transport_Address;
	enum class transport_CloseKind;
	enum class broadcast_Kind;
	enum class track_Compression;
	enum class track_Order;
	enum class origin_Link;
	enum class origin_RouteKind;
	enum class hang_catalog_AudioCodecKind;
	enum class hang_catalog_VideoCodecKind;
	enum class hang_catalog_ColorRange;
	enum class mux_catalog_CatalogFormat;
	enum class mux_container_Kind;
	enum class mux_import_AudioFormat;
	enum class mux_import_ContainerFormat;
	enum class mux_import_Kind;
	enum class mux_import_VideoFormat;
	enum class transport_poll_Acked;
	enum class mux_codec_flac_Error;
	enum class mux_codec_mp3_Error;
	enum class mux_codec_vp8_Error;
	enum class mux_codec_vp9_Error;
	enum class mux_container_ts_Framing;
	enum class mux_container_ts_stats_Class;
	enum class moq_tokio_quic_CongestionControl;
	enum class moq_tokio_accept_Failure;
	enum class moq_tokio_Redirect;
	enum class rustls_client_Tls12Resumption;
	enum class moq_quic_connection_mux_Version;
	enum class moq_sock_demux_Discard;
	enum class moq_sock_udp_EcnCodepoint;
	enum class Level;
	class TimeOverflow;
	class Timescale;
	class Timestamp;
	class clock_Instant;
	class clock_Elapsed;
	class clock_Wall;
	struct setup_Token;
	class session_Limits;
	class stats_Content;
	class stats_Traffic;
	class stats_Presence;
	class stats_Activity;
	class stats_Totals;
	class stats_Failures;
	class stats_Refusals;
	class stats_RouteCloses;
	class stats_RouteRefusals;
	class stats_Routes;
	class stats_Withdrawals;
	class transport_ReceiveWindow;
	class transport_Limits;
	class bandwidth_Rate;
	struct datagram_Datagram;
	struct frame_Info;
	struct group_Info;
	struct track_Delay;
	class track_Position;
	struct track_Range;
	class origin_Cost;
	class origin_Priority;
	struct hang_catalog_AAC;
	class hang_catalog_Clock;
	class hang_catalog_Priorities;
	struct hang_catalog_AV1;
	struct hang_catalog_Mastering;
	struct hang_catalog_Chromaticity;
	struct hang_catalog_ContentLight;
	struct hang_catalog_H264;
	struct hang_catalog_H265;
	struct hang_catalog_VP9;
	struct hang_catalog_Display;
	struct hang_container_Frame;
	struct mux_container_Frame;
	struct mux_container_TimestampRewind;
	struct mux_container_Playout;
	struct mux_codec_flac_Config;
	struct mux_codec_h264_Sps;
	struct mux_codec_h265_VideoSignal;
	struct mux_codec_mp3_Config;
	class mux_codec_opus_Mapping;
	struct mux_container_fmp4_Fragment;
	struct mux_container_legacy_Wire;
	struct mux_container_loc_Wire;
	struct mux_container_ts_Descriptor;
	struct rustls_client_TicketRequest;
	struct rustls_crypto_hpke_HpkePublicKey;
	class Park;
	class Waiter;
	class WaiterList;
	class DecodeError;
	class Driver;
	class Instance;
	class Path;
	class Pattern;
	class Patterns;
	class Server;
	class Session;
	class Client;
	class Error;
	class Version;
	class Versions;
	class Specificity;
	class clock_Clock;
	class clock_Deadline;
	class clock_Deadlines;
	class clock_Entered;
	class clock_Manual;
	class clock_Rng;
	class clock_Interval;
	class clock_WallMap;
	class coop_Budget;
	class hang_Catalog;
	class hang_Catalog_Extra;
	class hang_Catalog_Ext;
	class hang_Error;
	class mux_Clock;
	class mux_Ingest;
	class mux_Linger_unit;
	class mux_Pacer;
	class mux_Binding;
	class mux_Source;
	class mux_Error;
	class tokio_Duration;
	class tokio_Backoff;
	class tokio_Client;
	class tokio_Connection;
	class tokio_Error;
	class tokio_Log;
	class tokio_Server;
	class auth_Watch;
	class auth_Requests;
	class auth_Request;
	class auth_Issued;
	class auth_Grant;
	class auth_Handle;
	class auth_Credential;
	class auth_Token;
	class auth_Tokens;
	class goaway_Goaway;
	class goaway_Consumer;
	class path_Relative;
	class reconnect_Backoff;
	class reconnect_Handover;
	class reconnect_Config;
	class server_Handshake;
	class server_Admission;
	class server_Admitted;
	class session_Stats;
	class stats_Tier;
	class stats_Snapshot;
	class stats_Report;
	class stats_TrafficEntry;
	class stats_SessionEntry;
	class stats_Config;
	class stats_Registry;
	class stats_Handle;
	class stats_Refusal;
	class stats_SessionInfo;
	class stats_Wire;
	class stats_Session;
	class stats_Bill;
	class transport_Wire;
	class transport_Probe;
	class bandwidth_Limit;
	class bandwidth_Demand;
	class bandwidth_Reservation;
	class bandwidth_Viewer;
	class bandwidth_Cap;
	class bandwidth_Producer;
	class bandwidth_Weak;
	class bandwidth_Allocator;
	class bandwidth_Viewers;
	class bandwidth_Consumer;
	class broadcast_DatagramRequests;
	class broadcast_Consumer;
	class broadcast_Demand;
	class broadcast_Info;
	class broadcast_Producer;
	class broadcast_Requests;
	class broadcast_Lister;
	class broadcast_Tracks;
	class broadcast_TracksEvent;
	class cache_Config;
	class cache_Pool;
	class cache_PoolWeak;
	class datagram_Producer;
	class datagram_Request;
	class datagram_Consumer;
	class datagram_Info;
	class frame_Producer;
	class frame_Frame;
	class frame_Consumer;
	class group_Producer;
	class group_Finished;
	class group_Demand;
	class group_Consumer;
	class group_Fetch;
	class track_Finished;
	class track_Requests;
	class track_Demand;
	class track_Subscriber;
	class track_Ordered;
	class track_Item;
	class track_Request;
	class track_Info;
	class track_Refresh;
	class track_Producer;
	class track_AbortHandle;
	class track_Consumer;
	class track_Control;
	class track_Subscription;
	class track_Fill;
	class origin_Following;
	class origin_Claim;
	class origin_Config;
	class origin_Consumer;
	class origin_Driver;
	class origin_Producer;
	class origin_Request;
	class origin_Route;
	class origin_RouteInfo;
	class origin_RouteOptions;
	class origin_RouteEvent;
	class origin_Routes;
	class announce_Consumer;
	class announce_Event;
	class announce_Announce;
	class hang_stats_Snapshot;
	class hang_stats_Rendition;
	class hang_stats_Transport;
	class hang_echo_Snapshot;
	class hang_echo_Rendition;
	class hang_echo_Arrival;
	class hang_catalog_Skipped;
	class hang_catalog_Consumer;
	class hang_catalog_Audio;
	class hang_catalog_AudioConfig;
	class hang_catalog_Binary;
	class hang_catalog_BinaryConfig;
	class hang_catalog_Container;
	class hang_catalog_Echo;
	class hang_catalog_Include;
	class hang_catalog_Json;
	class hang_catalog_JsonConfig;
	class hang_catalog_Stats;
	class hang_catalog_Text;
	class hang_catalog_TextConfig;
	class hang_catalog_Timeline;
	class hang_catalog_Color;
	class hang_catalog_Projection;
	class hang_catalog_Video;
	class hang_catalog_VideoProperties;
	class hang_catalog_VideoConfig;
	class mux_catalog_Consumer;
	class mux_catalog_Consumer_Extra;
	class mux_catalog_Consumer_Ext;
	class mux_catalog_Estimate;
	class mux_catalog_Config;
	class mux_catalog_Config_Extra;
	class mux_catalog_Config_Ext;
	class mux_catalog_Guard;
	class mux_catalog_Guard_Extra;
	class mux_catalog_Guard_Ext;
	class mux_catalog_Producer;
	class mux_catalog_Producer_Extra;
	class mux_catalog_Producer_Ext;
	class mux_catalog_Timebase;
	class mux_catalog_Timebase_Extra;
	class mux_catalog_Timebase_Ext;
	class mux_catalog_Reserved;
	class mux_catalog_Reserved_Extra;
	class mux_catalog_Reserved_Ext;
	class mux_catalog_VideoHint;
	class mux_container_Consumer_Container;
	class mux_container_Consumer_fmp4_Wire;
	class mux_container_Consumer_legacy_Wire;
	class mux_container_Consumer_loc_Wire;
	class mux_container_GroupConsumer_Container;
	class mux_container_GroupConsumer_fmp4_Wire;
	class mux_container_GroupConsumer_legacy_Wire;
	class mux_container_GroupConsumer_loc_Wire;
	class mux_container_Finished_Container_AudioConfig;
	class mux_container_Finished_Container_BinaryConfig;
	class mux_container_Finished_Container_JsonConfig;
	class mux_container_Finished_Container_TextConfig;
	class mux_container_Finished_Container_VideoConfig;
	class mux_container_Finished_Container;
	class mux_container_Finished_fmp4_Wire_AudioConfig;
	class mux_container_Finished_fmp4_Wire_BinaryConfig;
	class mux_container_Finished_fmp4_Wire_JsonConfig;
	class mux_container_Finished_fmp4_Wire_TextConfig;
	class mux_container_Finished_fmp4_Wire_VideoConfig;
	class mux_container_Finished_fmp4_Wire;
	class mux_container_Finished_legacy_Wire_AudioConfig;
	class mux_container_Finished_legacy_Wire_BinaryConfig;
	class mux_container_Finished_legacy_Wire_JsonConfig;
	class mux_container_Finished_legacy_Wire_TextConfig;
	class mux_container_Finished_legacy_Wire_VideoConfig;
	class mux_container_Finished_legacy_Wire;
	class mux_container_Finished_loc_Wire_AudioConfig;
	class mux_container_Finished_loc_Wire_BinaryConfig;
	class mux_container_Finished_loc_Wire_JsonConfig;
	class mux_container_Finished_loc_Wire_TextConfig;
	class mux_container_Finished_loc_Wire_VideoConfig;
	class mux_container_Finished_loc_Wire;
	class mux_container_Guard_unit;
	class mux_container_Guard_AudioConfig;
	class mux_container_Guard_BinaryConfig;
	class mux_container_Guard_JsonConfig;
	class mux_container_Guard_TextConfig;
	class mux_container_Guard_VideoConfig;
	class mux_container_Producer_Container_AudioConfig;
	class mux_container_Producer_Container_BinaryConfig;
	class mux_container_Producer_Container_JsonConfig;
	class mux_container_Producer_Container_TextConfig;
	class mux_container_Producer_Container_VideoConfig;
	class mux_container_Producer_Container;
	class mux_container_Producer_fmp4_Wire_AudioConfig;
	class mux_container_Producer_fmp4_Wire_BinaryConfig;
	class mux_container_Producer_fmp4_Wire_JsonConfig;
	class mux_container_Producer_fmp4_Wire_TextConfig;
	class mux_container_Producer_fmp4_Wire_VideoConfig;
	class mux_container_Producer_fmp4_Wire;
	class mux_container_Producer_legacy_Wire_AudioConfig;
	class mux_container_Producer_legacy_Wire_BinaryConfig;
	class mux_container_Producer_legacy_Wire_JsonConfig;
	class mux_container_Producer_legacy_Wire_TextConfig;
	class mux_container_Producer_legacy_Wire_VideoConfig;
	class mux_container_Producer_legacy_Wire;
	class mux_container_Producer_loc_Wire_AudioConfig;
	class mux_container_Producer_loc_Wire_BinaryConfig;
	class mux_container_Producer_loc_Wire_JsonConfig;
	class mux_container_Producer_loc_Wire_TextConfig;
	class mux_container_Producer_loc_Wire_VideoConfig;
	class mux_container_Producer_loc_Wire;
	class mux_follow_Instances;
	class mux_import_Container;
	class mux_import_Container_Extra;
	class mux_import_Container_Ext;
	class mux_import_AudioInit;
	class mux_import_ContainerInit;
	class mux_import_VideoInit;
	class mux_import_Track;
	class mux_import_TrackStream;
	class mux_json_Snapshot_Json;
	class mux_json_Stream_Json;
	class mux_json_Consumer_Json;
	class mux_json_Config;
	class mux_json_Options;
	class mux_rate_Grant;
	class mux_rate_Policy;
	class mux_rate_Control;
	class mux_select_Broadcast;
	class mux_select_Video;
	class mux_select_Audio;
	class mux_sync_Track;
	class mux_sync_Schedule;
	class mux_sync_Producer;
	class mux_sync_Consumer;
	class tokio_tls_Error;
	class tokio_tls_Identity;
	class tokio_tls_Connect;
	class tokio_tls_Listen;
	class tokio_tls_Serve;
	class tokio_tls_PeerIdentity;
	class tokio_tls_Certificates;
	class tokio_tls_KeyLog;
	class tokio_client_Config;
	class tokio_connect_Config;
	class tokio_connection_Monitor;
	class tokio_listen_Config;
	class tokio_quic_Config;
	class tokio_server_Config;
	class tokio_server_Listener;
	class tokio_server_Request;
	class transport_memory_Config;
	class transport_memory_Session;
	class transport_memory_Closer;
	class mux_catalog_hang_Consumer;
	class mux_catalog_hang_Consumer_Extra;
	class mux_catalog_hang_Consumer_Ext;
	class mux_catalog_hang_Container;
	class mux_catalog_hang_Extra;
	class mux_catalog_msf_Consumer;
	class mux_catalog_msf_Consumer_Extra;
	class mux_catalog_msf_Consumer_Ext;
	class mux_codec_aac_Config;
	class mux_codec_aac_Error;
	class mux_codec_aac_Import;
	class mux_codec_annexb_Error;
	class mux_codec_av1_Import;
	class mux_codec_av1_Split;
	class mux_codec_flac_Import;
	class mux_codec_h264_Avc1;
	class mux_codec_h264_Error;
	class mux_codec_h264_Avcc;
	class mux_codec_h264_Import;
	class mux_codec_h264_Split;
	class mux_codec_h265_Hvc1;
	class mux_codec_h265_Error;
	class mux_codec_h265_Hvcc;
	class mux_codec_h265_Sps;
	class mux_codec_h265_Import;
	class mux_codec_h265_Split;
	class mux_codec_mp3_Import;
	class mux_codec_opus_Config;
	class mux_codec_opus_Import;
	class mux_codec_vp8_Import;
	class mux_codec_vp9_Import;
	class mux_container_flv_Import;
	class mux_container_flv_Import_Extra;
	class mux_container_flv_Import_Ext;
	class mux_container_flv_Finished;
	class mux_container_flv_Finished_Extra;
	class mux_container_flv_Finished_Ext;
	class mux_container_fmp4_Wire;
	class mux_container_fmp4_Error;
	class mux_container_fmp4_Import;
	class mux_container_fmp4_Import_Extra;
	class mux_container_fmp4_Import_Ext;
	class mux_container_fmp4_Finished;
	class mux_container_fmp4_Finished_Extra;
	class mux_container_fmp4_Finished_Ext;
	class mux_container_fmp4_Muxer;
	class mux_container_legacy_Audio;
	class mux_container_mkv_Import;
	class mux_container_mkv_Import_Extra;
	class mux_container_mkv_Import_Ext;
	class mux_container_mkv_Finished;
	class mux_container_mkv_Finished_Extra;
	class mux_container_mkv_Finished_Ext;
	class mux_container_mp4_Finish;
	class mux_container_ts_Ext;
	class mux_container_ts_Mpegts;
	class mux_container_ts_PassthroughTrack;
	class mux_container_ts_PesPacket;
	class mux_container_ts_Program;
	class mux_container_ts_SiEntry;
	class mux_container_ts_Track;
	class mux_container_ts_Verbatim;
	class mux_container_ts_Error;
	class mux_container_ts_Export_Consumer_Extra;
	class mux_container_ts_Export_Consumer_Ext;
	class mux_container_ts_Export_Consumer;
	class mux_container_ts_Follower;
	class mux_container_ts_Follower_Extra;
	class mux_container_ts_Follower_Ext;
	class mux_container_ts_Passthrough;
	class mux_container_ts_Passthrough_unit;
	class mux_container_ts_Passthrough_Extra;
	class mux_container_ts_Programs;
	class mux_container_ts_Release;
	class mux_container_ts_Republish;
	class mux_container_ts_Republished;
	class mux_container_ts_Import;
	class mux_container_ts_Import_Extra;
	class mux_container_ts_Import_Ext;
	class mux_container_ts_Finished;
	class mux_container_ts_Finished_Extra;
	class mux_container_ts_Finished_Ext;
	class mux_import_container_Finished;
	class mux_import_container_Finished_Extra;
	class mux_import_container_Finished_Ext;
	class mux_import_track_Finished;
	class mux_codec_aac_adts_Header;
	class mux_container_ts_passthrough_Finished;
	class mux_container_ts_passthrough_Finished_unit;
	class mux_container_ts_passthrough_Finished_Extra;
	class mux_container_ts_programs_Finished;
	class mux_container_ts_republish_Finished;
	class mux_container_ts_stats_Report;
	class mux_container_ts_stats_Snapshot;
	class mux_container_ts_stats_Stream;
	class mux_container_ts_stats_Export;
	class mux_container_ts_stats_Release;
	class mux_container_ts_stats_Log;
	class moq_tokio_Addrs;
	class moq_tokio_cli_Deprecated;
	class moq_tokio_cli_Bound;
	class moq_tokio_tcp_TlsAcceptor;
	class moq_tokio_accept_Health;
	class moq_json_Timed_Catalog;
	class moq_json_Timed_Json;
	class rustls_ClientConfig;
	class rustls_RootCertStore;
	class rustls_pki_types_CertificateDer;
	class moq_tokio_connect_Resolved;
	class moq_tokio_connection_Handover;
	class moq_tokio_websocket_Config;
	class moq_tokio_connection_Observation;
	class moq_tokio_connection_Snapshot;
	class moq_tokio_listen_Resolved;
	class moq_tokio_tcp_Config;
	class moq_tokio_unix_Config;
	class moq_tokio_listen_LoadBalancer;
	class moq_tokio_listen_Reset;
	class moq_tokio_quic_Resolved;
	class moq_tokio_websocket_Listener;
	class moq_tokio_quic_demux_Demux;
	class moq_tokio_server_Streams;
	class moq_tokio_server_QuicLimits;
	class moq_quic_Limits;
	class moq_tokio_connect_Addr;
	class rustls_pki_types_TrustAnchor;
	class moq_tokio_connection_Phase;
	class moq_tokio_quic_ServerId;
	class moq_tokio_listen_Forward;
	class rustls_client_danger_DangerousClientConfig;
	class rustls_client_Resumption;
	class rustls_DistinguishedName;
	class moq_tokio_websocket_Resolved;
	class moq_tokio_unix_Allow;
	class moq_tokio_websocket_Error;
	class qmux_Session;
	class moq_sock_demux_Counters;
	class moq_tokio_quic_demux_WebRtc;
	class rustls_pki_types_Der;
	class moq_tokio_quic_demux_Datagram;
	class moq_sock_demux_Pin;
	class Url;
	class Json;
	class Instant;
	class rustls_ServerConfig;
	class rustls_ResolvesClientCert;
	class rustls_KeyLog;
	class rustls_TimeProvider;
	class rustls_CertDecompressor;
	class rustls_CertCompressor;
	class rustls_CompressionCache;
	class rustls_CryptoProvider;
	class transport_Metrics;
	struct InvalidPattern_EmptySegment;
	struct InvalidPattern_InvalidSegment;
	struct InvalidPattern_MultipleGlobstars;
	struct InvalidPattern_TooManySegments;
	using InvalidPattern = ::std::variant<InvalidPattern_EmptySegment, InvalidPattern_InvalidSegment, InvalidPattern_MultipleGlobstars, InvalidPattern_TooManySegments>;
	struct SessionError_Cancel;
	struct SessionError_Internal;
	struct SessionError_Unauthorized;
	struct SessionError_Expired;
	struct SessionError_ProtocolViolation;
	struct SessionError_TooManyRequests;
	struct SessionError_GoawayTimeout;
	struct SessionError_Timeout;
	struct SessionError_Version;
	struct SessionError_MalformedAuthToken;
	struct SessionError_App;
	struct SessionError_Unknown;
	struct SessionError_Unsupported;
	using SessionError = ::std::variant<SessionError_Cancel, SessionError_Internal, SessionError_Unauthorized, SessionError_Expired, SessionError_ProtocolViolation, SessionError_TooManyRequests, SessionError_GoawayTimeout, SessionError_Timeout, SessionError_Version, SessionError_MalformedAuthToken, SessionError_App, SessionError_Unknown, SessionError_Unsupported>;
	struct StreamError_Session;
	struct StreamError_Internal;
	struct StreamError_Cancel;
	struct StreamError_DeliveryTimeout;
	struct StreamError_ControlTimeout;
	struct StreamError_GoingAway;
	struct StreamError_TooFarBehind;
	struct StreamError_MalformedTrack;
	struct StreamError_NotFound;
	struct StreamError_Unroutable;
	struct StreamError_Old;
	struct StreamError_Evicted;
	struct StreamError_WrongSize;
	struct StreamError_FrameTooLarge;
	struct StreamError_GroupTooLarge;
	struct StreamError_TimestampMismatch;
	struct StreamError_Unauthorized;
	struct StreamError_App;
	struct StreamError_Unknown;
	using StreamError = ::std::variant<StreamError_Session, StreamError_Internal, StreamError_Cancel, StreamError_DeliveryTimeout, StreamError_ControlTimeout, StreamError_GoingAway, StreamError_TooFarBehind, StreamError_MalformedTrack, StreamError_NotFound, StreamError_Unroutable, StreamError_Old, StreamError_Evicted, StreamError_WrongSize, StreamError_FrameTooLarge, StreamError_GroupTooLarge, StreamError_TimestampMismatch, StreamError_Unauthorized, StreamError_App, StreamError_Unknown>;
	struct Segment_Literal;
	struct Segment_Wildcard;
	struct Segment_Partial;
	struct Segment_Globstar;
	using Segment = ::std::variant<Segment_Literal, Segment_Wildcard, Segment_Partial, Segment_Globstar>;
	struct transport_LimitsError_Unsupported;
	struct transport_LimitsError_Refused;
	using transport_LimitsError = ::std::variant<transport_LimitsError_Unsupported, transport_LimitsError_Refused>;
	struct track_Anchor_Relative;
	struct track_Anchor_Oldest;
	struct track_Anchor_Time;
	using track_Anchor = ::std::variant<track_Anchor_Relative, track_Anchor_Oldest, track_Anchor_Time>;
	struct hang_catalog_AudioCodec_AAC;
	struct hang_catalog_AudioCodec_Opus;
	struct hang_catalog_AudioCodec_Pcm;
	struct hang_catalog_AudioCodec_Flac;
	struct hang_catalog_AudioCodec_Mp3;
	struct hang_catalog_AudioCodec_Mp2;
	struct hang_catalog_AudioCodec_Ac3;
	struct hang_catalog_AudioCodec_Ec3;
	struct hang_catalog_AudioCodec_Unknown;
	using hang_catalog_AudioCodec = ::std::variant<hang_catalog_AudioCodec_AAC, hang_catalog_AudioCodec_Opus, hang_catalog_AudioCodec_Pcm, hang_catalog_AudioCodec_Flac, hang_catalog_AudioCodec_Mp3, hang_catalog_AudioCodec_Mp2, hang_catalog_AudioCodec_Ac3, hang_catalog_AudioCodec_Ec3, hang_catalog_AudioCodec_Unknown>;
	struct hang_catalog_AudioLayout_Ambisonic;
	struct hang_catalog_AudioLayout_Unknown;
	using hang_catalog_AudioLayout = ::std::variant<hang_catalog_AudioLayout_Ambisonic, hang_catalog_AudioLayout_Unknown>;
	struct hang_catalog_Mode_Snapshot;
	struct hang_catalog_Mode_Stream;
	struct hang_catalog_Mode_Unknown;
	using hang_catalog_Mode = ::std::variant<hang_catalog_Mode_Snapshot, hang_catalog_Mode_Stream, hang_catalog_Mode_Unknown>;
	struct hang_catalog_TextFormat_Vtt;
	struct hang_catalog_TextFormat_Ttml;
	struct hang_catalog_TextFormat_Utf8;
	struct hang_catalog_TextFormat_Unknown;
	using hang_catalog_TextFormat = ::std::variant<hang_catalog_TextFormat_Vtt, hang_catalog_TextFormat_Ttml, hang_catalog_TextFormat_Utf8, hang_catalog_TextFormat_Unknown>;
	struct hang_catalog_TextRole_Subtitle;
	struct hang_catalog_TextRole_Caption;
	struct hang_catalog_TextRole_Unknown;
	using hang_catalog_TextRole = ::std::variant<hang_catalog_TextRole_Subtitle, hang_catalog_TextRole_Caption, hang_catalog_TextRole_Unknown>;
	struct hang_catalog_VideoCodec_H264;
	struct hang_catalog_VideoCodec_H265;
	struct hang_catalog_VideoCodec_VP9;
	struct hang_catalog_VideoCodec_AV1;
	struct hang_catalog_VideoCodec_VP8;
	struct hang_catalog_VideoCodec_Unknown;
	using hang_catalog_VideoCodec = ::std::variant<hang_catalog_VideoCodec_H264, hang_catalog_VideoCodec_H265, hang_catalog_VideoCodec_VP9, hang_catalog_VideoCodec_AV1, hang_catalog_VideoCodec_VP8, hang_catalog_VideoCodec_Unknown>;
	struct hang_catalog_Stereo_Mono;
	struct hang_catalog_Stereo_LeftRight;
	struct hang_catalog_Stereo_TopBottom;
	struct hang_catalog_Stereo_Unknown;
	using hang_catalog_Stereo = ::std::variant<hang_catalog_Stereo_Mono, hang_catalog_Stereo_LeftRight, hang_catalog_Stereo_TopBottom, hang_catalog_Stereo_Unknown>;
	struct mux_container_Read_Frame;
	struct mux_container_Read_Gap;
	struct mux_container_Read_End;
	using mux_container_Read = ::std::variant<mux_container_Read_Frame, mux_container_Read_Gap, mux_container_Read_End>;
	struct mux_container_ReorderError_Depth;
	struct mux_container_ReorderError_Deepened;
	struct mux_container_ReorderError_Reordered;
	struct mux_container_ReorderError_Presentation;
	struct mux_container_ReorderError_Tie;
	struct mux_container_ReorderError_Undrained;
	using mux_container_ReorderError = ::std::variant<mux_container_ReorderError_Depth, mux_container_ReorderError_Deepened, mux_container_ReorderError_Reordered, mux_container_ReorderError_Presentation, mux_container_ReorderError_Tie, mux_container_ReorderError_Undrained>;
	struct mux_import_Format_Audio;
	struct mux_import_Format_Video;
	struct mux_import_Format_Container;
	using mux_import_Format = ::std::variant<mux_import_Format_Audio, mux_import_Format_Video, mux_import_Format_Container>;
	struct mux_rate_Target_Enabled;
	struct mux_rate_Target_Disabled;
	using mux_rate_Target = ::std::variant<mux_rate_Target_Enabled, mux_rate_Target_Disabled>;
	struct tokio_listen_Bind_Addr;
	struct tokio_listen_Bind_Host;
	using tokio_listen_Bind = ::std::variant<tokio_listen_Bind_Addr, tokio_listen_Bind_Host>;
	struct transport_memory_Error_Session;
	struct transport_memory_Error_Reset;
	struct transport_memory_Error_Stopped;
	struct transport_memory_Error_Closed;
	using transport_memory_Error = ::std::variant<transport_memory_Error_Session, transport_memory_Error_Reset, transport_memory_Error_Stopped, transport_memory_Error_Closed>;
	struct mux_catalog_msf_Error_UnresolvableNamespace;
	struct mux_catalog_msf_Error_InvalidUtf8;
	struct mux_catalog_msf_Error_ParseFrame;
	struct mux_catalog_msf_Error_MissingCmafInit;
	struct mux_catalog_msf_Error_MalformedInitData;
	struct mux_catalog_msf_Error_MissingVideoCodec;
	struct mux_catalog_msf_Error_MissingAudioCodec;
	struct mux_catalog_msf_Error_UnsupportedPackaging;
	struct mux_catalog_msf_Error_MissingTextCodec;
	struct mux_catalog_msf_Error_InvalidVideoCodec;
	struct mux_catalog_msf_Error_InvalidAudioCodec;
	struct mux_catalog_msf_Error_MissingAudioParams;
	struct mux_catalog_msf_Error_UnsupportedDerivationPackaging;
	struct mux_catalog_msf_Error_MalformedAac;
	struct mux_catalog_msf_Error_MalformedOpus;
	struct mux_catalog_msf_Error_MalformedFlac;
	struct mux_catalog_msf_Error_OpusTrailingBytes;
	struct mux_catalog_msf_Error_UnsupportedDerivationCodec;
	struct mux_catalog_msf_Error_MalformedInitSegment;
	struct mux_catalog_msf_Error_MissingInitMoov;
	struct mux_catalog_msf_Error_MissingAudioSampleEntry;
	using mux_catalog_msf_Error = ::std::variant<mux_catalog_msf_Error_UnresolvableNamespace, mux_catalog_msf_Error_InvalidUtf8, mux_catalog_msf_Error_ParseFrame, mux_catalog_msf_Error_MissingCmafInit, mux_catalog_msf_Error_MalformedInitData, mux_catalog_msf_Error_MissingVideoCodec, mux_catalog_msf_Error_MissingAudioCodec, mux_catalog_msf_Error_UnsupportedPackaging, mux_catalog_msf_Error_MissingTextCodec, mux_catalog_msf_Error_InvalidVideoCodec, mux_catalog_msf_Error_InvalidAudioCodec, mux_catalog_msf_Error_MissingAudioParams, mux_catalog_msf_Error_UnsupportedDerivationPackaging, mux_catalog_msf_Error_MalformedAac, mux_catalog_msf_Error_MalformedOpus, mux_catalog_msf_Error_MalformedFlac, mux_catalog_msf_Error_OpusTrailingBytes, mux_catalog_msf_Error_UnsupportedDerivationCodec, mux_catalog_msf_Error_MalformedInitSegment, mux_catalog_msf_Error_MissingInitMoov, mux_catalog_msf_Error_MissingAudioSampleEntry>;
	struct mux_codec_opus_Error_HeadTooShort;
	struct mux_codec_opus_Error_InvalidSignature;
	struct mux_codec_opus_Error_UnsupportedVersion;
	struct mux_codec_opus_Error_UnsupportedChannelCount;
	struct mux_codec_opus_Error_MappingTableTooShort;
	struct mux_codec_opus_Error_InvalidMappingTable;
	struct mux_codec_opus_Error_UnsupportedMappingFamily;
	using mux_codec_opus_Error = ::std::variant<mux_codec_opus_Error_HeadTooShort, mux_codec_opus_Error_InvalidSignature, mux_codec_opus_Error_UnsupportedVersion, mux_codec_opus_Error_UnsupportedChannelCount, mux_codec_opus_Error_MappingTableTooShort, mux_codec_opus_Error_InvalidMappingTable, mux_codec_opus_Error_UnsupportedMappingFamily>;
	struct mux_container_fmp4_Chunk_Init;
	struct mux_container_fmp4_Chunk_Fragment;
	using mux_container_fmp4_Chunk = ::std::variant<mux_container_fmp4_Chunk_Init, mux_container_fmp4_Chunk_Fragment>;
	struct mux_container_ts_Step_Rewind;
	struct mux_container_ts_Step_Leap;
	struct mux_container_ts_Step_Identity;
	using mux_container_ts_Step = ::std::variant<mux_container_ts_Step_Rewind, mux_container_ts_Step_Leap, mux_container_ts_Step_Identity>;
	struct mux_codec_aac_adts_Error_Truncated;
	struct mux_codec_aac_adts_Error_Syncword;
	struct mux_codec_aac_adts_Error_SampleRate;
	struct mux_codec_aac_adts_Error_FrameLength;
	struct mux_codec_aac_adts_Error_PacketLength;
	struct mux_codec_aac_adts_Error_ObjectType;
	struct mux_codec_aac_adts_Error_Rate;
	struct mux_codec_aac_adts_Error_ChannelConfig;
	struct mux_codec_aac_adts_Error_TooLarge;
	using mux_codec_aac_adts_Error = ::std::variant<mux_codec_aac_adts_Error_Truncated, mux_codec_aac_adts_Error_Syncword, mux_codec_aac_adts_Error_SampleRate, mux_codec_aac_adts_Error_FrameLength, mux_codec_aac_adts_Error_PacketLength, mux_codec_aac_adts_Error_ObjectType, mux_codec_aac_adts_Error_Rate, mux_codec_aac_adts_Error_ChannelConfig, mux_codec_aac_adts_Error_TooLarge>;
	struct moq_quic_Rate_Unlimited;
	struct moq_quic_Rate_BytesPerSecond;
	using moq_quic_Rate = ::std::variant<moq_quic_Rate_Unlimited, moq_quic_Rate_BytesPerSecond>;
	struct moq_tokio_connect_Error_EmptyAddresses;
	struct moq_tokio_connect_Error_UnsupportedScheme;
	struct moq_tokio_connect_Error_MissingHost;
	using moq_tokio_connect_Error = ::std::variant<moq_tokio_connect_Error_EmptyAddresses, moq_tokio_connect_Error_UnsupportedScheme, moq_tokio_connect_Error_MissingHost>;
	struct rustls_ProtocolVersion_SSLv2;
	struct rustls_ProtocolVersion_SSLv3;
	struct rustls_ProtocolVersion_TLSv1_0;
	struct rustls_ProtocolVersion_TLSv1_1;
	struct rustls_ProtocolVersion_TLSv1_2;
	struct rustls_ProtocolVersion_TLSv1_3;
	struct rustls_ProtocolVersion_DTLSv1_0;
	struct rustls_ProtocolVersion_DTLSv1_2;
	struct rustls_ProtocolVersion_DTLSv1_3;
	struct rustls_ProtocolVersion_Unknown;
	using rustls_ProtocolVersion = ::std::variant<rustls_ProtocolVersion_SSLv2, rustls_ProtocolVersion_SSLv3, rustls_ProtocolVersion_TLSv1_0, rustls_ProtocolVersion_TLSv1_1, rustls_ProtocolVersion_TLSv1_2, rustls_ProtocolVersion_TLSv1_3, rustls_ProtocolVersion_DTLSv1_0, rustls_ProtocolVersion_DTLSv1_2, rustls_ProtocolVersion_DTLSv1_3, rustls_ProtocolVersion_Unknown>;
	using Waiter_poll_future_future = ::moq::future<detail::Waiter_poll_future_future_op>;
	using Server_accept_memory_Session_future = ::moq::future<detail::Server_accept_memory_Session_future_op>;
	using Server_accept_qmux_Session_future = ::moq::future<detail::Server_accept_qmux_Session_future_op>;
	using Server_handshake_memory_Session_future = ::moq::future<detail::Server_handshake_memory_Session_future_op>;
	using Server_handshake_qmux_Session_future = ::moq::future<detail::Server_handshake_qmux_Session_future_op>;
	using Session_peer_implementation_future = ::moq::future<detail::Session_peer_implementation_future_op>;
	using Session_close_future = ::moq::future<detail::Session_close_future_op>;
	using Session_closed_future = ::moq::future<detail::Session_closed_future_op>;
	using Client_connect_memory_Session_future = ::moq::future<detail::Client_connect_memory_Session_future_op>;
	using Client_connect_qmux_Session_future = ::moq::future<detail::Client_connect_qmux_Session_future_op>;
	using clock_Deadline_poll_future = ::moq::future<detail::clock_Deadline_poll_future_op>;
	using clock_Interval_tick_future = ::moq::future<detail::clock_Interval_tick_future_op>;
	using coop_Budget_poll_yield_future = ::moq::future<detail::coop_Budget_poll_yield_future_op>;
	using mux_Linger_unit_wait_future = ::moq::future<detail::mux_Linger_unit_wait_future_op>;
	using mux_Binding_broadcast_future = ::moq::future<detail::mux_Binding_broadcast_future_op>;
	using mux_Source_catalog_Extra_future = ::moq::future<detail::mux_Source_catalog_Extra_future_op>;
	using mux_Source_catalog_Ext_future = ::moq::future<detail::mux_Source_catalog_Ext_future_op>;
	using mux_Source_catalog_unit_future = ::moq::future<detail::mux_Source_catalog_unit_future_op>;
	using mux_Source_resolve_future = ::moq::future<detail::mux_Source_resolve_future_op>;
	using mux_Source_subscribe_track_future = ::moq::future<detail::mux_Source_subscribe_track_future_op>;
	using tokio_Client_close_future = ::moq::future<detail::tokio_Client_close_future_op>;
	using tokio_Connection_close_future = ::moq::future<detail::tokio_Connection_close_future_op>;
	using tokio_Connection_established_future = ::moq::future<detail::tokio_Connection_established_future_op>;
	using tokio_Connection_status_future = ::moq::future<detail::tokio_Connection_status_future_op>;
	using tokio_Connection_closed_future = ::moq::future<detail::tokio_Connection_closed_future_op>;
	using tokio_Server_serve_publish_future = ::moq::future<detail::tokio_Server_serve_publish_future_op>;
	using tokio_Server_bind_future = ::moq::future<detail::tokio_Server_bind_future_op>;
	using tokio_Server_listen_future = ::moq::future<detail::tokio_Server_listen_future_op>;
	using auth_Watch_changed_future = ::moq::future<detail::auth_Watch_changed_future_op>;
	using auth_Requests_next_future = ::moq::future<detail::auth_Requests_next_future_op>;
	using auth_Request_closed_future = ::moq::future<detail::auth_Request_closed_future_op>;
	using auth_Issued_closed_future = ::moq::future<detail::auth_Issued_closed_future_op>;
	using auth_Credential_closed_future = ::moq::future<detail::auth_Credential_closed_future_op>;
	using auth_Credential_answered_future = ::moq::future<detail::auth_Credential_answered_future_op>;
	using auth_Tokens_answered_future = ::moq::future<detail::auth_Tokens_answered_future_op>;
	using goaway_Consumer_poll_future = ::moq::future<detail::goaway_Consumer_poll_future_op>;
	using server_Handshake_expired_future = ::moq::future<detail::server_Handshake_expired_future_op>;
	using server_Handshake_ok_future = ::moq::future<detail::server_Handshake_ok_future_op>;
	using server_Admission_close_future = ::moq::future<detail::server_Admission_close_future_op>;
	using bandwidth_Limit_changed_future = ::moq::future<detail::bandwidth_Limit_changed_future_op>;
	using bandwidth_Demand_wanted_changed_future = ::moq::future<detail::bandwidth_Demand_wanted_changed_future_op>;
	using bandwidth_Consumer_changed_future = ::moq::future<detail::bandwidth_Consumer_changed_future_op>;
	using broadcast_DatagramRequests_requested_future = ::moq::future<detail::broadcast_DatagramRequests_requested_future_op>;
	using broadcast_Consumer_closed_future = ::moq::future<detail::broadcast_Consumer_closed_future_op>;
	using broadcast_Demand_wanted_changed_future = ::moq::future<detail::broadcast_Demand_wanted_changed_future_op>;
	using broadcast_Requests_requested_track_future = ::moq::future<detail::broadcast_Requests_requested_track_future_op>;
	using broadcast_Requests_closed_future = ::moq::future<detail::broadcast_Requests_closed_future_op>;
	using broadcast_Lister_poll_wanted_future = ::moq::future<detail::broadcast_Lister_poll_wanted_future_op>;
	using broadcast_Lister_poll_unwanted_future = ::moq::future<detail::broadcast_Lister_poll_unwanted_future_op>;
	using broadcast_Tracks_next_future = ::moq::future<detail::broadcast_Tracks_next_future_op>;
	using datagram_Producer_priority_changed_future = ::moq::future<detail::datagram_Producer_priority_changed_future_op>;
	using datagram_Consumer_info_future = ::moq::future<detail::datagram_Consumer_info_future_op>;
	using datagram_Consumer_recv_future = ::moq::future<detail::datagram_Consumer_recv_future_op>;
	using frame_Consumer_read_chunk_future = ::moq::future<detail::frame_Consumer_read_chunk_future_op>;
	using frame_Consumer_read_all_future = ::moq::future<detail::frame_Consumer_read_all_future_op>;
	using group_Producer_ready_future = ::moq::future<detail::group_Producer_ready_future_op>;
	using group_Producer_closed_future = ::moq::future<detail::group_Producer_closed_future_op>;
	using group_Demand_wanted_changed_future = ::moq::future<detail::group_Demand_wanted_changed_future_op>;
	using group_Demand_closed_future = ::moq::future<detail::group_Demand_closed_future_op>;
	using group_Consumer_next_frame_future = ::moq::future<detail::group_Consumer_next_frame_future_op>;
	using group_Consumer_read_frame_future = ::moq::future<detail::group_Consumer_read_frame_future_op>;
	using group_Consumer_finished_future = ::moq::future<detail::group_Consumer_finished_future_op>;
	using track_Finished_closed_future = ::moq::future<detail::track_Finished_closed_future_op>;
	using track_Requests_requested_fill_future = ::moq::future<detail::track_Requests_requested_fill_future_op>;
	using track_Demand_wanted_changed_future = ::moq::future<detail::track_Demand_wanted_changed_future_op>;
	using track_Demand_refresh_future = ::moq::future<detail::track_Demand_refresh_future_op>;
	using track_Demand_closed_future = ::moq::future<detail::track_Demand_closed_future_op>;
	using track_Subscriber_recv_group_future = ::moq::future<detail::track_Subscriber_recv_group_future_op>;
	using track_Subscriber_start_future = ::moq::future<detail::track_Subscriber_start_future_op>;
	using track_Subscriber_finished_future = ::moq::future<detail::track_Subscriber_finished_future_op>;
	using track_Subscriber_largest_future = ::moq::future<detail::track_Subscriber_largest_future_op>;
	using track_Subscriber_refresh_future = ::moq::future<detail::track_Subscriber_refresh_future_op>;
	using track_Ordered_next_group_future = ::moq::future<detail::track_Ordered_next_group_future_op>;
	using track_Ordered_refresh_future = ::moq::future<detail::track_Ordered_refresh_future_op>;
	using track_Ordered_finished_future = ::moq::future<detail::track_Ordered_finished_future_op>;
	using track_Ordered_largest_future = ::moq::future<detail::track_Ordered_largest_future_op>;
	using track_Request_subscription_changed_future = ::moq::future<detail::track_Request_subscription_changed_future_op>;
	using track_Producer_closed_future = ::moq::future<detail::track_Producer_closed_future_op>;
	using track_Producer_subscription_changed_future = ::moq::future<detail::track_Producer_subscription_changed_future_op>;
	using track_Consumer_subscribe_future = ::moq::future<detail::track_Consumer_subscribe_future_op>;
	using track_Consumer_fetch_group_future = ::moq::future<detail::track_Consumer_fetch_group_future_op>;
	using track_Consumer_info_future = ::moq::future<detail::track_Consumer_info_future_op>;
	using track_Consumer_largest_future = ::moq::future<detail::track_Consumer_largest_future_op>;
	using track_Control_refresh_future = ::moq::future<detail::track_Control_refresh_future_op>;
	using origin_Following_available_future = ::moq::future<detail::origin_Following_available_future_op>;
	using origin_Claim_requested_future = ::moq::future<detail::origin_Claim_requested_future_op>;
	using origin_Consumer_request_future = ::moq::future<detail::origin_Consumer_request_future_op>;
	using origin_Routes_next_future = ::moq::future<detail::origin_Routes_next_future_op>;
	using origin_Routes_poll_synced_future = ::moq::future<detail::origin_Routes_poll_synced_future_op>;
	using announce_Consumer_next_future = ::moq::future<detail::announce_Consumer_next_future_op>;
	using announce_Consumer_poll_synced_future = ::moq::future<detail::announce_Consumer_poll_synced_future_op>;
	using hang_catalog_Consumer_poll_next_future = ::moq::future<detail::hang_catalog_Consumer_poll_next_future_op>;
	using mux_catalog_Consumer_new_future = ::moq::future<detail::mux_catalog_Consumer_new_future_op>;
	using mux_catalog_Consumer_Extra_new_future = ::moq::future<detail::mux_catalog_Consumer_Extra_new_future_op>;
	using mux_catalog_Consumer_Ext_new_future = ::moq::future<detail::mux_catalog_Consumer_Ext_new_future_op>;
	using mux_container_Consumer_Container_read_future = ::moq::future<detail::mux_container_Consumer_Container_read_future_op>;
	using mux_container_Consumer_fmp4_Wire_read_future = ::moq::future<detail::mux_container_Consumer_fmp4_Wire_read_future_op>;
	using mux_container_Consumer_legacy_Wire_read_future = ::moq::future<detail::mux_container_Consumer_legacy_Wire_read_future_op>;
	using mux_container_Consumer_loc_Wire_read_future = ::moq::future<detail::mux_container_Consumer_loc_Wire_read_future_op>;
	using mux_container_GroupConsumer_Container_read_future = ::moq::future<detail::mux_container_GroupConsumer_Container_read_future_op>;
	using mux_container_GroupConsumer_fmp4_Wire_read_future = ::moq::future<detail::mux_container_GroupConsumer_fmp4_Wire_read_future_op>;
	using mux_container_GroupConsumer_legacy_Wire_read_future = ::moq::future<detail::mux_container_GroupConsumer_legacy_Wire_read_future_op>;
	using mux_container_GroupConsumer_loc_Wire_read_future = ::moq::future<detail::mux_container_GroupConsumer_loc_Wire_read_future_op>;
	using mux_container_Producer_Container_AudioConfig_closed_future = ::moq::future<detail::mux_container_Producer_Container_AudioConfig_closed_future_op>;
	using mux_container_Producer_Container_BinaryConfig_closed_future = ::moq::future<detail::mux_container_Producer_Container_BinaryConfig_closed_future_op>;
	using mux_container_Producer_Container_JsonConfig_closed_future = ::moq::future<detail::mux_container_Producer_Container_JsonConfig_closed_future_op>;
	using mux_container_Producer_Container_TextConfig_closed_future = ::moq::future<detail::mux_container_Producer_Container_TextConfig_closed_future_op>;
	using mux_container_Producer_Container_VideoConfig_closed_future = ::moq::future<detail::mux_container_Producer_Container_VideoConfig_closed_future_op>;
	using mux_container_Producer_Container_closed_future = ::moq::future<detail::mux_container_Producer_Container_closed_future_op>;
	using mux_container_Producer_fmp4_Wire_AudioConfig_closed_future = ::moq::future<detail::mux_container_Producer_fmp4_Wire_AudioConfig_closed_future_op>;
	using mux_container_Producer_fmp4_Wire_BinaryConfig_closed_future = ::moq::future<detail::mux_container_Producer_fmp4_Wire_BinaryConfig_closed_future_op>;
	using mux_container_Producer_fmp4_Wire_JsonConfig_closed_future = ::moq::future<detail::mux_container_Producer_fmp4_Wire_JsonConfig_closed_future_op>;
	using mux_container_Producer_fmp4_Wire_TextConfig_closed_future = ::moq::future<detail::mux_container_Producer_fmp4_Wire_TextConfig_closed_future_op>;
	using mux_container_Producer_fmp4_Wire_VideoConfig_closed_future = ::moq::future<detail::mux_container_Producer_fmp4_Wire_VideoConfig_closed_future_op>;
	using mux_container_Producer_fmp4_Wire_closed_future = ::moq::future<detail::mux_container_Producer_fmp4_Wire_closed_future_op>;
	using mux_container_Producer_legacy_Wire_AudioConfig_closed_future = ::moq::future<detail::mux_container_Producer_legacy_Wire_AudioConfig_closed_future_op>;
	using mux_container_Producer_legacy_Wire_BinaryConfig_closed_future = ::moq::future<detail::mux_container_Producer_legacy_Wire_BinaryConfig_closed_future_op>;
	using mux_container_Producer_legacy_Wire_JsonConfig_closed_future = ::moq::future<detail::mux_container_Producer_legacy_Wire_JsonConfig_closed_future_op>;
	using mux_container_Producer_legacy_Wire_TextConfig_closed_future = ::moq::future<detail::mux_container_Producer_legacy_Wire_TextConfig_closed_future_op>;
	using mux_container_Producer_legacy_Wire_VideoConfig_closed_future = ::moq::future<detail::mux_container_Producer_legacy_Wire_VideoConfig_closed_future_op>;
	using mux_container_Producer_legacy_Wire_closed_future = ::moq::future<detail::mux_container_Producer_legacy_Wire_closed_future_op>;
	using mux_container_Producer_loc_Wire_AudioConfig_closed_future = ::moq::future<detail::mux_container_Producer_loc_Wire_AudioConfig_closed_future_op>;
	using mux_container_Producer_loc_Wire_BinaryConfig_closed_future = ::moq::future<detail::mux_container_Producer_loc_Wire_BinaryConfig_closed_future_op>;
	using mux_container_Producer_loc_Wire_JsonConfig_closed_future = ::moq::future<detail::mux_container_Producer_loc_Wire_JsonConfig_closed_future_op>;
	using mux_container_Producer_loc_Wire_TextConfig_closed_future = ::moq::future<detail::mux_container_Producer_loc_Wire_TextConfig_closed_future_op>;
	using mux_container_Producer_loc_Wire_VideoConfig_closed_future = ::moq::future<detail::mux_container_Producer_loc_Wire_VideoConfig_closed_future_op>;
	using mux_container_Producer_loc_Wire_closed_future = ::moq::future<detail::mux_container_Producer_loc_Wire_closed_future_op>;
	using mux_follow_Instances_next_future = ::moq::future<detail::mux_follow_Instances_next_future_op>;
	using mux_json_Snapshot_Json_refresh_future = ::moq::future<detail::mux_json_Snapshot_Json_refresh_future_op>;
	using mux_json_Consumer_Json_from_track_future = ::moq::future<detail::mux_json_Consumer_Json_from_track_future_op>;
	using mux_json_Consumer_Json_next_future = ::moq::future<detail::mux_json_Consumer_Json_next_future_op>;
	using mux_json_Consumer_Json_latest_future = ::moq::future<detail::mux_json_Consumer_Json_latest_future_op>;
	using mux_rate_Grant_next_future = ::moq::future<detail::mux_rate_Grant_next_future_op>;
	using mux_sync_Track_delay_changed_future = ::moq::future<detail::mux_sync_Track_delay_changed_future_op>;
	using mux_sync_Consumer_changed_future = ::moq::future<detail::mux_sync_Consumer_changed_future_op>;
	using tokio_connection_Monitor_state_changed_future = ::moq::future<detail::tokio_connection_Monitor_state_changed_future_op>;
	using tokio_connection_Monitor_poll_presence_future = ::moq::future<detail::tokio_connection_Monitor_poll_presence_future_op>;
	using tokio_connection_Monitor_presence_changed_future = ::moq::future<detail::tokio_connection_Monitor_presence_changed_future_op>;
	using tokio_server_Listener_accept_future = ::moq::future<detail::tokio_server_Listener_accept_future_op>;
	using tokio_server_Listener_close_future = ::moq::future<detail::tokio_server_Listener_close_future_op>;
	using tokio_server_Listener_close_streams_future = ::moq::future<detail::tokio_server_Listener_close_streams_future_op>;
	using tokio_server_Request_reject_future = ::moq::future<detail::tokio_server_Request_reject_future_op>;
	using tokio_server_Request_expired_future = ::moq::future<detail::tokio_server_Request_expired_future_op>;
	using tokio_server_Request_ok_future = ::moq::future<detail::tokio_server_Request_ok_future_op>;
	using mux_catalog_hang_Consumer_next_future = ::moq::future<detail::mux_catalog_hang_Consumer_next_future_op>;
	using mux_catalog_hang_Consumer_Extra_next_future = ::moq::future<detail::mux_catalog_hang_Consumer_Extra_next_future_op>;
	using mux_catalog_hang_Consumer_Ext_next_future = ::moq::future<detail::mux_catalog_hang_Consumer_Ext_next_future_op>;
	using mux_catalog_msf_Consumer_next_future = ::moq::future<detail::mux_catalog_msf_Consumer_next_future_op>;
	using mux_catalog_msf_Consumer_Extra_next_future = ::moq::future<detail::mux_catalog_msf_Consumer_Extra_next_future_op>;
	using mux_catalog_msf_Consumer_Ext_next_future = ::moq::future<detail::mux_catalog_msf_Consumer_Ext_next_future_op>;
	using mux_container_fmp4_Muxer_read_future = ::moq::future<detail::mux_container_fmp4_Muxer_read_future_op>;
	using mux_container_ts_Export_Consumer_Extra_next_future = ::moq::future<detail::mux_container_ts_Export_Consumer_Extra_next_future_op>;
	using mux_container_ts_Export_Consumer_Ext_next_future = ::moq::future<detail::mux_container_ts_Export_Consumer_Ext_next_future_op>;
	using mux_container_ts_Export_Consumer_next_future = ::moq::future<detail::mux_container_ts_Export_Consumer_next_future_op>;
	using mux_container_ts_Follower_next_future = ::moq::future<detail::mux_container_ts_Follower_next_future_op>;
	using mux_container_ts_Follower_Extra_next_future = ::moq::future<detail::mux_container_ts_Follower_Extra_next_future_op>;
	using mux_container_ts_Follower_Ext_next_future = ::moq::future<detail::mux_container_ts_Follower_Ext_next_future_op>;
	using mux_container_ts_Release_new_future = ::moq::future<detail::mux_container_ts_Release_new_future_op>;
	using mux_container_ts_Release_next_future = ::moq::future<detail::mux_container_ts_Release_next_future_op>;
	using moq_tokio_websocket_Listener_bind_future = ::moq::future<detail::moq_tokio_websocket_Listener_bind_future_op>;
	using moq_tokio_websocket_Listener_accept_future = ::moq::future<detail::moq_tokio_websocket_Listener_accept_future_op>;
	using moq_tokio_quic_demux_WebRtc_recv_future = ::moq::future<detail::moq_tokio_quic_demux_WebRtc_recv_future_op>;
	using clock_drive_future = ::moq::future<detail::clock_drive_future_op>;
	using clock_sleep_future = ::moq::future<detail::clock_sleep_future_op>;
	using clock_sleep_until_future = ::moq::future<detail::clock_sleep_until_future_op>;
	using clock_timeout_future = ::moq::future<detail::clock_timeout_future_op>;
	using clock_timeout_at_future = ::moq::future<detail::clock_timeout_at_future_op>;
	using time_run_Driver_future = ::moq::future<detail::time_run_Driver_future_op>;
	using time_run_origin_Driver_future = ::moq::future<detail::time_run_origin_Driver_future_op>;
}
#endif

// rustls.hpp
#ifndef MOQ_HPP_CRATE12
#define MOQ_HPP_CRATE12
#include "rt.hpp"
#include "rustlspkitypes.hpp"
namespace moq {
	enum class rustls_client_Tls12Resumption {
		disabled = 0,
		session_id_only = 1,
		session_id_or_tickets = 2,
	};
	class RUST2_FFI_CONSUMABLE rustls_ClientConfig {
		RUST2_FFI_UNCONSUMED rustls_ClientConfig(rustls_ClientConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_ClientConfig &operator=(rustls_ClientConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE rustls_ClientConfig clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> alpn_protocols_get() const;
		RUST2_FFI_CALLABLE void alpn_protocols_set(const ::moq::items<::moq::view> &value);
		RUST2_FFI_CALLABLE bool check_selected_alpn_get() const;
		RUST2_FFI_CALLABLE void check_selected_alpn_set(bool value);
		RUST2_FFI_CALLABLE ::moq::rustls_client_Resumption resumption_get() const;
		RUST2_FFI_CALLABLE void resumption_set(::moq::rustls_client_Resumption value);
		RUST2_FFI_CALLABLE ::std::optional<::std::size_t> max_fragment_size_get() const;
		RUST2_FFI_CALLABLE void max_fragment_size_set(const ::std::optional<::std::size_t> &value);
		RUST2_FFI_CALLABLE ::moq::rustls_ResolvesClientCert client_auth_cert_resolver_get() const;
		RUST2_FFI_CALLABLE void client_auth_cert_resolver_set(::moq::rustls_ResolvesClientCert value);
		RUST2_FFI_CALLABLE bool enable_sni_get() const;
		RUST2_FFI_CALLABLE void enable_sni_set(bool value);
		RUST2_FFI_CALLABLE ::moq::rustls_KeyLog key_log_get() const;
		RUST2_FFI_CALLABLE void key_log_set(::moq::rustls_KeyLog value);
		RUST2_FFI_CALLABLE bool enable_secret_extraction_get() const;
		RUST2_FFI_CALLABLE void enable_secret_extraction_set(bool value);
		RUST2_FFI_CALLABLE bool enable_early_data_get() const;
		RUST2_FFI_CALLABLE void enable_early_data_set(bool value);
		RUST2_FFI_CALLABLE bool require_ems_get() const;
		RUST2_FFI_CALLABLE void require_ems_set(bool value);
		RUST2_FFI_CALLABLE ::moq::rustls_TimeProvider time_provider_get() const;
		RUST2_FFI_CALLABLE void time_provider_set(::moq::rustls_TimeProvider value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::rustls_CertDecompressor> cert_decompressors_get() const;
		RUST2_FFI_CALLABLE void cert_decompressors_set(::std::vector<::moq::rustls_CertDecompressor> value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::rustls_CertCompressor> cert_compressors_get() const;
		RUST2_FFI_CALLABLE void cert_compressors_set(::std::vector<::moq::rustls_CertCompressor> value);
		RUST2_FFI_CALLABLE ::moq::rustls_CompressionCache cert_compression_cache_get() const;
		RUST2_FFI_CALLABLE void cert_compression_cache_set(::moq::rustls_CompressionCache value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::rustls_client_TicketRequest> send_ticket_request_get() const;
		RUST2_FFI_CALLABLE void send_ticket_request_set(const ::std::optional<::moq::rustls_client_TicketRequest> &value);
		RUST2_FFI_CALLABLE bool fips() const;
		RUST2_FFI_CALLABLE ::moq::rustls_CryptoProvider crypto_provider() const;
		RUST2_FFI_CALLABLE ::moq::rustls_client_danger_DangerousClientConfig dangerous();
	};
	class RUST2_FFI_CONSUMABLE rustls_RootCertStore {
		RUST2_FFI_UNCONSUMED rustls_RootCertStore(rustls_RootCertStore &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_RootCertStore &operator=(rustls_RootCertStore &&other) noexcept = default;
		RUST2_FFI_CALLABLE rustls_RootCertStore clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::vector<::moq::rustls_pki_types_TrustAnchor> roots_get() const;
		RUST2_FFI_CALLABLE void roots_set(::std::vector<::moq::rustls_pki_types_TrustAnchor> value);
		static ::moq::rustls_RootCertStore empty();
		RUST2_FFI_CALLABLE ::std::tuple<::std::size_t, ::std::size_t> add_parsable_certificates(::std::vector<::moq::rustls_pki_types_CertificateDer> der_certs);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> add(::moq::rustls_pki_types_CertificateDer der);
		RUST2_FFI_CALLABLE ::std::vector<::moq::rustls_DistinguishedName> subjects() const;
		RUST2_FFI_CALLABLE bool is_empty() const;
		RUST2_FFI_CALLABLE ::std::size_t len() const;
	};
	class RUST2_FFI_CONSUMABLE rustls_client_danger_DangerousClientConfig {
		RUST2_FFI_UNCONSUMED rustls_client_danger_DangerousClientConfig(rustls_client_danger_DangerousClientConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_client_danger_DangerousClientConfig &operator=(rustls_client_danger_DangerousClientConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::rustls_ClientConfig cfg_get() const;
	};
	class RUST2_FFI_CONSUMABLE rustls_client_Resumption {
		RUST2_FFI_UNCONSUMED rustls_client_Resumption(rustls_client_Resumption &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_client_Resumption &operator=(rustls_client_Resumption &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED rustls_client_Resumption(const rustls_client_Resumption &other) = default;
		rustls_client_Resumption &operator=(const rustls_client_Resumption &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static rustls_client_Resumption default_();
		static ::moq::rustls_client_Resumption in_memory_sessions(::std::size_t num);
		static ::moq::rustls_client_Resumption disabled();
		RUST2_FFI_CONSUMES ::moq::rustls_client_Resumption tls12_resumption(::moq::rustls_client_Tls12Resumption tls12) &&;
	};
	class RUST2_FFI_CONSUMABLE rustls_DistinguishedName {
		RUST2_FFI_UNCONSUMED rustls_DistinguishedName(rustls_DistinguishedName &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_DistinguishedName &operator=(rustls_DistinguishedName &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED rustls_DistinguishedName(const rustls_DistinguishedName &other) = default;
		rustls_DistinguishedName &operator=(const rustls_DistinguishedName &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::rustls_DistinguishedName in_sequence(::moq::input bytes);
	};
	class RUST2_FFI_CONSUMABLE rustls_ServerConfig {
		RUST2_FFI_UNCONSUMED rustls_ServerConfig(rustls_ServerConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_ServerConfig &operator=(rustls_ServerConfig &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED rustls_ServerConfig(const rustls_ServerConfig &other) = default;
		rustls_ServerConfig &operator=(const rustls_ServerConfig &other) = default;
	};
	class RUST2_FFI_CONSUMABLE rustls_ResolvesClientCert {
		RUST2_FFI_UNCONSUMED rustls_ResolvesClientCert(rustls_ResolvesClientCert &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_ResolvesClientCert &operator=(rustls_ResolvesClientCert &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED rustls_ResolvesClientCert(const rustls_ResolvesClientCert &other) = default;
		rustls_ResolvesClientCert &operator=(const rustls_ResolvesClientCert &other) = default;
	};
	class RUST2_FFI_CONSUMABLE rustls_KeyLog {
		RUST2_FFI_UNCONSUMED rustls_KeyLog(rustls_KeyLog &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_KeyLog &operator=(rustls_KeyLog &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED rustls_KeyLog(const rustls_KeyLog &other) = default;
		rustls_KeyLog &operator=(const rustls_KeyLog &other) = default;
	};
	class RUST2_FFI_CONSUMABLE rustls_TimeProvider {
		RUST2_FFI_UNCONSUMED rustls_TimeProvider(rustls_TimeProvider &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_TimeProvider &operator=(rustls_TimeProvider &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED rustls_TimeProvider(const rustls_TimeProvider &other) = default;
		rustls_TimeProvider &operator=(const rustls_TimeProvider &other) = default;
	};
	class RUST2_FFI_CONSUMABLE rustls_CertDecompressor {
		RUST2_FFI_UNCONSUMED rustls_CertDecompressor(rustls_CertDecompressor &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_CertDecompressor &operator=(rustls_CertDecompressor &&other) noexcept = default;
	};
	class RUST2_FFI_CONSUMABLE rustls_CertCompressor {
		RUST2_FFI_UNCONSUMED rustls_CertCompressor(rustls_CertCompressor &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_CertCompressor &operator=(rustls_CertCompressor &&other) noexcept = default;
	};
	class RUST2_FFI_CONSUMABLE rustls_CompressionCache {
		RUST2_FFI_UNCONSUMED rustls_CompressionCache(rustls_CompressionCache &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_CompressionCache &operator=(rustls_CompressionCache &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED rustls_CompressionCache(const rustls_CompressionCache &other) = default;
		rustls_CompressionCache &operator=(const rustls_CompressionCache &other) = default;
	};
	class RUST2_FFI_CONSUMABLE rustls_CryptoProvider {
		RUST2_FFI_UNCONSUMED rustls_CryptoProvider(rustls_CryptoProvider &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_CryptoProvider &operator=(rustls_CryptoProvider &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED rustls_CryptoProvider(const rustls_CryptoProvider &other) = default;
		rustls_CryptoProvider &operator=(const rustls_CryptoProvider &other) = default;
	};
	struct rustls_client_TicketRequest {
		::std::uint8_t new_session_count;
		::std::uint8_t resumption_count;
		friend bool operator==(const rustls_client_TicketRequest &a, const rustls_client_TicketRequest &b);
		friend bool operator!=(const rustls_client_TicketRequest &a, const rustls_client_TicketRequest &b);
		::moq::buf debug() const;
	};
	struct rustls_crypto_hpke_HpkePublicKey {
		::moq::buf _0;
		::moq::buf debug() const;
	};
	struct rustls_ProtocolVersion_SSLv2 {
	};
	struct rustls_ProtocolVersion_SSLv3 {
	};
	struct rustls_ProtocolVersion_TLSv1_0 {
	};
	struct rustls_ProtocolVersion_TLSv1_1 {
	};
	struct rustls_ProtocolVersion_TLSv1_2 {
	};
	struct rustls_ProtocolVersion_TLSv1_3 {
	};
	struct rustls_ProtocolVersion_DTLSv1_0 {
	};
	struct rustls_ProtocolVersion_DTLSv1_2 {
	};
	struct rustls_ProtocolVersion_DTLSv1_3 {
	};
	struct rustls_ProtocolVersion_Unknown {
		::std::uint16_t _0;
	};
	inline bool operator==(const rustls_client_TicketRequest &a, const rustls_client_TicketRequest &b);
	inline bool operator==(const ::moq::rustls_ProtocolVersion &a, const ::moq::rustls_ProtocolVersion &b);
	inline bool operator!=(const ::moq::rustls_ProtocolVersion &a, const ::moq::rustls_ProtocolVersion &b);
	inline ::moq::buf rustls_ProtocolVersion_debug(const ::moq::rustls_ProtocolVersion &self);
	inline ::std::array<::std::uint8_t, 2> rustls_ProtocolVersion_to_array(const ::moq::rustls_ProtocolVersion &self);
	inline ::std::optional<::moq::buf> rustls_ProtocolVersion_as_str(const ::moq::rustls_ProtocolVersion &self);
	inline bool operator==(::moq::rustls_client_Tls12Resumption a, ::moq::rustls_client_Tls12Resumption b);
	inline bool operator!=(::moq::rustls_client_Tls12Resumption a, ::moq::rustls_client_Tls12Resumption b);
	inline ::moq::buf rustls_client_Tls12Resumption_debug(::moq::rustls_client_Tls12Resumption self);
}
#endif

// rustlspkitypes.hpp
#ifndef MOQ_HPP_CRATE11
#define MOQ_HPP_CRATE11
#include "rt.hpp"
namespace moq {
	class RUST2_FFI_CONSUMABLE rustls_pki_types_CertificateDer {
		RUST2_FFI_UNCONSUMED rustls_pki_types_CertificateDer(rustls_pki_types_CertificateDer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_pki_types_CertificateDer &operator=(rustls_pki_types_CertificateDer &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED rustls_pki_types_CertificateDer(const rustls_pki_types_CertificateDer &other) = default;
		rustls_pki_types_CertificateDer &operator=(const rustls_pki_types_CertificateDer &other) = default;
		friend bool operator==(const rustls_pki_types_CertificateDer &a, const rustls_pki_types_CertificateDer &b);
		friend bool operator!=(const rustls_pki_types_CertificateDer &a, const rustls_pki_types_CertificateDer &b);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::rustls_pki_types_CertificateDer from_slice(::moq::input bytes);
		RUST2_FFI_CONSUMES ::moq::rustls_pki_types_CertificateDer into_owned() &&;
	};
	class RUST2_FFI_CONSUMABLE rustls_pki_types_TrustAnchor {
		RUST2_FFI_UNCONSUMED rustls_pki_types_TrustAnchor(rustls_pki_types_TrustAnchor &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_pki_types_TrustAnchor &operator=(rustls_pki_types_TrustAnchor &&other) noexcept = default;
		RUST2_FFI_CALLABLE rustls_pki_types_TrustAnchor clone() const;
		friend bool operator==(const rustls_pki_types_TrustAnchor &a, const rustls_pki_types_TrustAnchor &b);
		friend bool operator!=(const rustls_pki_types_TrustAnchor &a, const rustls_pki_types_TrustAnchor &b);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::rustls_pki_types_Der subject_get() const;
		RUST2_FFI_CALLABLE void subject_set(::moq::rustls_pki_types_Der value);
		RUST2_FFI_CALLABLE ::moq::rustls_pki_types_Der subject_public_key_info_get() const;
		RUST2_FFI_CALLABLE void subject_public_key_info_set(::moq::rustls_pki_types_Der value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::rustls_pki_types_Der> name_constraints_get() const;
		RUST2_FFI_CALLABLE void name_constraints_set(::std::optional<::moq::rustls_pki_types_Der> value);
		RUST2_FFI_CALLABLE ::moq::rustls_pki_types_TrustAnchor to_owned() const;
	};
	class RUST2_FFI_CONSUMABLE rustls_pki_types_Der {
		RUST2_FFI_UNCONSUMED rustls_pki_types_Der(rustls_pki_types_Der &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		rustls_pki_types_Der &operator=(rustls_pki_types_Der &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED rustls_pki_types_Der(const rustls_pki_types_Der &other) = default;
		rustls_pki_types_Der &operator=(const rustls_pki_types_Der &other) = default;
		friend bool operator==(const rustls_pki_types_Der &a, const rustls_pki_types_Der &b);
		friend bool operator!=(const rustls_pki_types_Der &a, const rustls_pki_types_Der &b);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::rustls_pki_types_Der from_slice(::moq::input der);
	};
}
namespace std {
	template <> struct hash<::moq::rustls_pki_types_CertificateDer> {
		size_t operator()(const ::moq::rustls_pki_types_CertificateDer &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::moq::rustls_pki_types_TrustAnchor> {
		size_t operator()(const ::moq::rustls_pki_types_TrustAnchor &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::moq::rustls_pki_types_Der> {
		size_t operator()(const ::moq::rustls_pki_types_Der &value) const noexcept;
	};
}
#endif

// serdejson.hpp
#ifndef MOQ_HPP_CRATE6
#define MOQ_HPP_CRATE6
#include "rt.hpp"
namespace moq {
	class RUST2_FFI_CONSUMABLE Json {
		RUST2_FFI_UNCONSUMED Json(Json &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Json &operator=(Json &&other) noexcept = default;
		static ::moq::result<::moq::Json, ::moq::buf> parse(::moq::input input_);
		RUST2_FFI_CALLABLE ::moq::buf to_text() const;
	};
}
#endif

// sock.hpp
#ifndef MOQ_HPP_CRATE10
#define MOQ_HPP_CRATE10
#include "rt.hpp"
namespace moq {
	enum class moq_sock_demux_Discard {
		stun = 0,
		unknown = 1,
		unclaimed = 2,
		overflow = 3,
	};
	enum class moq_sock_udp_EcnCodepoint {
		ect0 = 2,
		ect1 = 1,
		ce = 3,
	};
	class RUST2_FFI_CONSUMABLE moq_sock_demux_Counters {
		RUST2_FFI_UNCONSUMED moq_sock_demux_Counters(moq_sock_demux_Counters &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_sock_demux_Counters &operator=(moq_sock_demux_Counters &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED moq_sock_demux_Counters(const moq_sock_demux_Counters &other) = default;
		moq_sock_demux_Counters &operator=(const moq_sock_demux_Counters &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_sock_demux_Counters default_();
		RUST2_FFI_CALLABLE void discard(::moq::moq_sock_demux_Discard reason) const;
		RUST2_FFI_CALLABLE ::std::uint64_t get(::moq::moq_sock_demux_Discard reason) const;
	};
	class RUST2_FFI_CONSUMABLE moq_sock_demux_Pin {
		RUST2_FFI_UNCONSUMED moq_sock_demux_Pin(moq_sock_demux_Pin &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_sock_demux_Pin &operator=(moq_sock_demux_Pin &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	inline bool operator==(::moq::moq_sock_demux_Discard a, ::moq::moq_sock_demux_Discard b);
	inline bool operator!=(::moq::moq_sock_demux_Discard a, ::moq::moq_sock_demux_Discard b);
	inline ::moq::buf moq_sock_demux_Discard_debug(::moq::moq_sock_demux_Discard self);
	inline ::std::vector<::moq::moq_sock_demux_Discard> moq_sock_demux_Discard_ALL();
	inline ::moq::buf moq_sock_demux_Discard_as_str(::moq::moq_sock_demux_Discard self);
	inline bool operator==(::moq::moq_sock_udp_EcnCodepoint a, ::moq::moq_sock_udp_EcnCodepoint b);
	inline bool operator!=(::moq::moq_sock_udp_EcnCodepoint a, ::moq::moq_sock_udp_EcnCodepoint b);
	inline ::moq::buf moq_sock_udp_EcnCodepoint_debug(::moq::moq_sock_udp_EcnCodepoint self);
	inline ::std::optional<::moq::moq_sock_udp_EcnCodepoint> moq_sock_udp_EcnCodepoint_from_bits(::std::uint8_t x);
}
#endif

// std.hpp
#ifndef MOQ_HPP_CRATE0
#define MOQ_HPP_CRATE0
#include "rt.hpp"
namespace moq {
	class RUST2_FFI_CONSUMABLE Instant {
		RUST2_FFI_UNCONSUMED Instant(Instant &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Instant &operator=(Instant &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Instant(const Instant &other) = default;
		Instant &operator=(const Instant &other) = default;
		friend bool operator==(const Instant &a, const Instant &b);
		friend bool operator!=(const Instant &a, const Instant &b);
		friend bool operator<(const Instant &a, const Instant &b);
		friend bool operator<=(const Instant &a, const Instant &b);
		friend bool operator>(const Instant &a, const Instant &b);
		friend bool operator>=(const Instant &a, const Instant &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const Instant &a, const Instant &b);
		#endif
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
}
namespace std {
	template <> struct hash<::moq::Instant> {
		size_t operator()(const ::moq::Instant &value) const noexcept;
	};
}
#endif

// tokio.hpp
#ifndef MOQ_HPP_CRATE15
#define MOQ_HPP_CRATE15
#include "rt.hpp"
#include "kio.hpp"
#include "net.hpp"
#include "quic.hpp"
#include "sock.hpp"
#include "qmux.hpp"
#include "rustls.hpp"
#include "rustlspkitypes.hpp"
#include "tracingcore.hpp"
#include "url.hpp"
namespace moq {
	enum class tokio_ConnectError {
		unauthorized = 0,
		forbidden = 1,
	};
	enum class tokio_Status {
		connected = 0,
		disconnected = 1,
		migrating = 2,
	};
	enum class tokio_Transport {
		quic = 0,
		web_socket = 1,
		tcp = 2,
		unix_ = 3,
		web_transport = 4,
		tls = 5,
	};
	enum class moq_tokio_quic_CongestionControl {
		bbr = 0,
		cubic = 1,
		unlimited = 2,
	};
	enum class moq_tokio_accept_Failure {
		connection = 0,
		exhausted = 1,
		unknown = 2,
	};
	enum class moq_tokio_Redirect {
		follow = 0,
		same_host = 1,
		ignore = 2,
	};
	class RUST2_FFI_CONSUMABLE tokio_Duration {
		RUST2_FFI_UNCONSUMED tokio_Duration(tokio_Duration &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_Duration &operator=(tokio_Duration &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED tokio_Duration(const tokio_Duration &other) = default;
		tokio_Duration &operator=(const tokio_Duration &other) = default;
		friend bool operator==(const tokio_Duration &a, const tokio_Duration &b);
		friend bool operator!=(const tokio_Duration &a, const tokio_Duration &b);
		friend bool operator<(const tokio_Duration &a, const tokio_Duration &b);
		friend bool operator<=(const tokio_Duration &a, const tokio_Duration &b);
		friend bool operator>(const tokio_Duration &a, const tokio_Duration &b);
		friend bool operator>=(const tokio_Duration &a, const tokio_Duration &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const tokio_Duration &a, const tokio_Duration &b);
		#endif
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static tokio_Duration default_();
	};
	class RUST2_FFI_CONSUMABLE tokio_Backoff {
		RUST2_FFI_UNCONSUMED tokio_Backoff(tokio_Backoff &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_Backoff &operator=(tokio_Backoff &&other) noexcept = default;
		RUST2_FFI_CALLABLE tokio_Backoff clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static tokio_Backoff default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Duration> initial_get() const;
		RUST2_FFI_CALLABLE void initial_set(const ::moq::tokio_Duration *value);
		RUST2_FFI_CALLABLE ::std::uint32_t multiplier_get() const;
		RUST2_FFI_CALLABLE void multiplier_set(::std::uint32_t value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_tokio_cli_Bound> max_get() const;
		RUST2_FFI_CALLABLE void max_set(const ::moq::moq_tokio_cli_Bound *value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Duration> healthy_get() const;
		RUST2_FFI_CALLABLE void healthy_set(const ::moq::tokio_Duration *value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_tokio_cli_Bound> timeout_get() const;
		RUST2_FFI_CALLABLE void timeout_set(const ::moq::moq_tokio_cli_Bound *value);
		RUST2_FFI_CALLABLE ::moq::duration initial() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> max() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> timeout() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_Client {
		RUST2_FFI_UNCONSUMED tokio_Client(tokio_Client &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_Client &operator=(tokio_Client &&other) noexcept = default;
		RUST2_FFI_CALLABLE tokio_Client clone() const;
		static ::moq::result<::moq::tokio_Client, ::moq::tokio_Error> new_(::moq::tokio_client_Config config);
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_publisher_Producer(const ::moq::origin_Producer &publish) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_publisher_Consumer(const ::moq::origin_Consumer &publish) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_subscriber(::moq::origin_Producer subscribe) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_limits(const ::moq::session_Limits &limits) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_stats(::moq::stats_Session stats) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_cost(::std::uint64_t cost) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_peer_node(::std::uint64_t node) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_token(const ::moq::setup_Token &token) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_reconnect(bool reconnect) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_restart(::moq::tokio_Restart restart) &&;
		RUST2_FFI_CALLABLE ::moq::tokio_Connection connect(::moq::moq_tokio_Addrs addrs) const;
		RUST2_FFI_CONSUMES ::moq::tokio_Client_close_future close() &&;
		RUST2_FFI_CONSUMES ::std::optional<::moq::tokio_Connection> publish(::moq::origin_Consumer origin) &&;
		RUST2_FFI_CONSUMES ::std::optional<::moq::tokio_Connection> consume(::moq::origin_Producer origin) &&;
	};
	class RUST2_FFI_CONSUMABLE tokio_Connection {
		RUST2_FFI_UNCONSUMED tokio_Connection(tokio_Connection &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_Connection &operator=(tokio_Connection &&other) noexcept = default;
		RUST2_FFI_CALLABLE tokio_Connection clone() const;
		RUST2_FFI_CALLABLE void abort(::moq::Error err) const;
		RUST2_FFI_CONSUMES ::moq::tokio_Connection_close_future close() &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Connection_established_future established() &&;
		RUST2_FFI_CALLABLE ::moq::tokio_Connection_status_future status();
		RUST2_FFI_CALLABLE bool connected() const;
		RUST2_FFI_CALLABLE ::std::uint64_t connects() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Version> version() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Transport> transport() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::goaway_Consumer> draining() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Consumer send_bandwidth() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Allocator allocator() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Consumer recv_bandwidth() const;
		RUST2_FFI_CALLABLE ::moq::bandwidth_Viewers viewers() const;
		RUST2_FFI_CALLABLE ::moq::tokio_Connection_closed_future closed() const;
		RUST2_FFI_CALLABLE ::moq::auth_Handle auth() const;
		RUST2_FFI_CALLABLE ::moq::auth_Tokens tokens() const;
		RUST2_FFI_CALLABLE ::moq::tokio_connection_Monitor monitor() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_Error {
		RUST2_FFI_UNCONSUMED tokio_Error(tokio_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_Error &operator=(tokio_Error &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED tokio_Error(const tokio_Error &other) = default;
		tokio_Error &operator=(const tokio_Error &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_ConnectError> connect_error() const;
		RUST2_FFI_CALLABLE bool is_auth() const;
		RUST2_FFI_CALLABLE ::std::optional<::std::uint16_t> status() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_Log {
		RUST2_FFI_UNCONSUMED tokio_Log(tokio_Log &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_Log &operator=(tokio_Log &&other) noexcept = default;
		RUST2_FFI_CALLABLE tokio_Log clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static tokio_Log default_();
		RUST2_FFI_CALLABLE ::moq::Level level_get() const;
		RUST2_FFI_CALLABLE void level_set(::moq::Level value);
		static ::moq::tokio_Log new_(::moq::Level level);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Level> level() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::tokio_Error> init() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_Server {
		RUST2_FFI_UNCONSUMED tokio_Server(tokio_Server &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_Server &operator=(tokio_Server &&other) noexcept = default;
		static ::moq::result<::moq::tokio_Server, ::moq::tokio_Error> new_(::moq::tokio_server_Config config);
		RUST2_FFI_CONSUMES ::moq::tokio_Server_serve_publish_future serve_publish(::moq::origin_Consumer origin) &&;
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_tokio_tcp_TlsAcceptor> tls_acceptor() const;
		RUST2_FFI_CALLABLE ::moq::tokio_tls_Certificates certificates() const;
		RUST2_FFI_CALLABLE ::std::vector<::moq::moq_tokio_accept_Health> accept_health() const;
		RUST2_FFI_CONSUMES ::moq::tokio_Server_bind_future bind() &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Server_listen_future listen() &&;
		RUST2_FFI_CALLABLE ::moq::result<::moq::socket_addr, ::moq::tokio_Error> local_addr() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::socket_addr> websocket_local_addr() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_tls_Error {
		RUST2_FFI_UNCONSUMED tokio_tls_Error(tokio_tls_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_tls_Error &operator=(tokio_tls_Error &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_tls_Identity {
		RUST2_FFI_UNCONSUMED tokio_tls_Identity(tokio_tls_Identity &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_tls_Identity &operator=(tokio_tls_Identity &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED tokio_tls_Identity(const tokio_tls_Identity &other) = default;
		tokio_tls_Identity &operator=(const tokio_tls_Identity &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::result<::moq::tokio_tls_Identity, ::moq::tokio_tls_Error> generate(const ::moq::items<::moq::view> &hostnames);
		RUST2_FFI_CALLABLE ::moq::buf fingerprint() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_tls_Connect {
		RUST2_FFI_UNCONSUMED tokio_tls_Connect(tokio_tls_Connect &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_tls_Connect &operator=(tokio_tls_Connect &&other) noexcept = default;
		RUST2_FFI_CALLABLE tokio_tls_Connect clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static tokio_tls_Connect default_();
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> root_get() const;
		RUST2_FFI_CALLABLE void root_set(const ::moq::items<::moq::view> &value);
		RUST2_FFI_CALLABLE ::std::optional<bool> system_roots_get() const;
		RUST2_FFI_CALLABLE void system_roots_set(const ::std::optional<bool> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> fingerprint_get() const;
		RUST2_FFI_CALLABLE void fingerprint_set(const ::moq::items<::moq::view> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_tls_Identity> identity_get() const;
		RUST2_FFI_CALLABLE void identity_set(::std::optional<::moq::tokio_tls_Identity> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> cert_get() const;
		RUST2_FFI_CALLABLE void cert_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> key_get() const;
		RUST2_FFI_CALLABLE void key_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<bool> insecure_get() const;
		RUST2_FFI_CALLABLE void insecure_set(const ::std::optional<bool> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> host_name_get() const;
		RUST2_FFI_CALLABLE void host_name_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_cli_Deprecated deprecated() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::rustls_ClientConfig, ::moq::tokio_tls_Error> build() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_tls_Listen {
		RUST2_FFI_UNCONSUMED tokio_tls_Listen(tokio_tls_Listen &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_tls_Listen &operator=(tokio_tls_Listen &&other) noexcept = default;
		RUST2_FFI_CALLABLE tokio_tls_Listen clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static tokio_tls_Listen default_();
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> cert_get() const;
		RUST2_FFI_CALLABLE void cert_set(const ::moq::items<::moq::view> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> key_get() const;
		RUST2_FFI_CALLABLE void key_set(const ::moq::items<::moq::view> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> generate_get() const;
		RUST2_FFI_CALLABLE void generate_set(const ::moq::items<::moq::view> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_tls_Identity> identity_get() const;
		RUST2_FFI_CALLABLE void identity_set(::std::optional<::moq::tokio_tls_Identity> value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> root_get() const;
		RUST2_FFI_CALLABLE void root_set(const ::moq::items<::moq::view> &value);
		RUST2_FFI_CALLABLE ::moq::result<::moq::tokio_tls_Serve, ::moq::tokio_tls_Error> load() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_cli_Deprecated deprecated() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::rustls_RootCertStore, ::moq::tokio_tls_Error> load_roots() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::rustls_ServerConfig, ::moq::tokio_tls_Error> server_config(const ::moq::items<::moq::view> &alpn) const;
	};
	class RUST2_FFI_CONSUMABLE tokio_tls_Serve {
		RUST2_FFI_UNCONSUMED tokio_tls_Serve(tokio_tls_Serve &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_tls_Serve &operator=(tokio_tls_Serve &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED tokio_tls_Serve(const tokio_tls_Serve &other) = default;
		tokio_tls_Serve &operator=(const tokio_tls_Serve &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::tokio_tls_Certificates certificates() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::rustls_ServerConfig, ::moq::tokio_tls_Error> config(const ::moq::items<::moq::view> &alpn) const;
		RUST2_FFI_CALLABLE ::moq::tokio_tls_PeerIdentity peer(::std::vector<::moq::rustls_pki_types_CertificateDer> chain) const;
	};
	class RUST2_FFI_CONSUMABLE tokio_tls_PeerIdentity {
		RUST2_FFI_UNCONSUMED tokio_tls_PeerIdentity(tokio_tls_PeerIdentity &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_tls_PeerIdentity &operator=(tokio_tls_PeerIdentity &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED tokio_tls_PeerIdentity(const tokio_tls_PeerIdentity &other) = default;
		tokio_tls_PeerIdentity &operator=(const tokio_tls_PeerIdentity &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::vector<::moq::rustls_pki_types_CertificateDer> chain() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> fingerprint() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> name() const;
		RUST2_FFI_CALLABLE ::moq::result<::std::vector<::moq::buf>, ::moq::tokio_tls_Error> lineage() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> issuer() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Wall> expiry() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_tls_Certificates {
		RUST2_FFI_UNCONSUMED tokio_tls_Certificates(tokio_tls_Certificates &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_tls_Certificates &operator=(tokio_tls_Certificates &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED tokio_tls_Certificates(const tokio_tls_Certificates &other) = default;
		tokio_tls_Certificates &operator=(const tokio_tls_Certificates &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::result<::moq::tokio_tls_Certificates, ::moq::tokio_tls_Error> from_pem(::moq::input chain);
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> fingerprints() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_tls_KeyLog {
		RUST2_FFI_UNCONSUMED tokio_tls_KeyLog(tokio_tls_KeyLog &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_tls_KeyLog &operator=(tokio_tls_KeyLog &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::result<::moq::tokio_tls_KeyLog, ::moq::tokio_tls_Error> open(::moq::input path);
	};
	class RUST2_FFI_CONSUMABLE tokio_client_Config {
		RUST2_FFI_UNCONSUMED tokio_client_Config(tokio_client_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_client_Config &operator=(tokio_client_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE tokio_client_Config clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static tokio_client_Config default_();
		RUST2_FFI_CALLABLE ::moq::tokio_connect_Config connect_get() const;
		RUST2_FFI_CALLABLE void connect_set(::moq::tokio_connect_Config value);
		RUST2_FFI_CALLABLE ::moq::tokio_quic_Config quic_get() const;
		RUST2_FFI_CALLABLE void quic_set(::moq::tokio_quic_Config value);
		RUST2_FFI_CONSUMES ::moq::result<::moq::tokio_Client, ::moq::tokio_Error> init() &&;
	};
	class RUST2_FFI_CONSUMABLE tokio_connect_Config {
		RUST2_FFI_UNCONSUMED tokio_connect_Config(tokio_connect_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_connect_Config &operator=(tokio_connect_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE tokio_connect_Config clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static tokio_connect_Config default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::Url> url_get() const;
		RUST2_FFI_CALLABLE void url_set(::std::optional<::moq::Url> value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::auth_Token> tokens_get() const;
		RUST2_FFI_CALLABLE void tokens_set(::std::vector<::moq::auth_Token> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::socket_addr> bind_get() const;
		RUST2_FFI_CALLABLE void bind_set(const ::std::optional<::moq::socket_addr> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Duration> race_get() const;
		RUST2_FFI_CALLABLE void race_set(const ::moq::tokio_Duration *value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Duration> resolution_delay_get() const;
		RUST2_FFI_CALLABLE void resolution_delay_set(const ::moq::tokio_Duration *value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Duration> timeout_get() const;
		RUST2_FFI_CALLABLE void timeout_set(const ::moq::tokio_Duration *value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::Version> version_get() const;
		RUST2_FFI_CALLABLE void version_set(const ::std::vector<::moq::Version> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> implementation_get() const;
		RUST2_FFI_CALLABLE void implementation_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::tokio_tls_Connect tls_get() const;
		RUST2_FFI_CALLABLE void tls_set(::moq::tokio_tls_Connect value);
		RUST2_FFI_CALLABLE ::moq::tokio_Backoff backoff_get() const;
		RUST2_FFI_CALLABLE void backoff_set(::moq::tokio_Backoff value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_connection_Handover goaway_get() const;
		RUST2_FFI_CALLABLE void goaway_set(::moq::moq_tokio_connection_Handover value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_websocket_Config websocket_get() const;
		RUST2_FFI_CALLABLE void websocket_set(::moq::moq_tokio_websocket_Config value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_cli_Deprecated deprecated() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::tokio_Client, ::moq::tokio_Error> init(::moq::tokio_quic_Config quic) &&;
		RUST2_FFI_CALLABLE ::moq::Versions versions() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_connect_Resolved resolve() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_connection_Monitor {
		RUST2_FFI_UNCONSUMED tokio_connection_Monitor(tokio_connection_Monitor &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_connection_Monitor &operator=(tokio_connection_Monitor &&other) noexcept = default;
		RUST2_FFI_CALLABLE tokio_connection_Monitor clone() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_connection_Observation state() const;
		RUST2_FFI_CALLABLE ::moq::tokio_connection_Monitor_state_changed_future state_changed(::std::uint64_t after) const;
		RUST2_FFI_CALLABLE ::moq::stats_Presence presence() const;
		RUST2_FFI_CALLABLE ::moq::tokio_connection_Monitor_poll_presence_future poll_presence();
		RUST2_FFI_CALLABLE ::moq::tokio_connection_Monitor_presence_changed_future presence_changed();
		RUST2_FFI_CALLABLE ::std::optional<::moq::session_Stats> stats() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Error> failure() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_tokio_connection_Snapshot> snapshot() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_listen_Config {
		RUST2_FFI_UNCONSUMED tokio_listen_Config(tokio_listen_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_listen_Config &operator=(tokio_listen_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE tokio_listen_Config clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static tokio_listen_Config default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_listen_Bind> bind_get() const;
		RUST2_FFI_CALLABLE void bind_set(const ::std::optional<::moq::tokio_listen_Bind> &value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_tcp_Config tcp_get() const;
		RUST2_FFI_CALLABLE void tcp_set(::moq::moq_tokio_tcp_Config value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_unix_Config unix_get() const;
		RUST2_FFI_CALLABLE void unix_set(::moq::moq_tokio_unix_Config value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::Version> version_get() const;
		RUST2_FFI_CALLABLE void version_set(const ::std::vector<::moq::Version> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> implementation_get() const;
		RUST2_FFI_CALLABLE void implementation_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Duration> timeout_get() const;
		RUST2_FFI_CALLABLE void timeout_set(const ::moq::tokio_Duration *value);
		RUST2_FFI_CALLABLE ::moq::tokio_tls_Listen tls_get() const;
		RUST2_FFI_CALLABLE void tls_set(::moq::tokio_tls_Listen value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::socket_addr_v4> preferred_v4_get() const;
		RUST2_FFI_CALLABLE void preferred_v4_set(const ::std::optional<::moq::socket_addr_v4> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::socket_addr_v6> preferred_v6_get() const;
		RUST2_FFI_CALLABLE void preferred_v6_set(const ::std::optional<::moq::socket_addr_v6> &value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_listen_LoadBalancer lb_get() const;
		RUST2_FFI_CALLABLE void lb_set(::moq::moq_tokio_listen_LoadBalancer value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_listen_Reset reset_get() const;
		RUST2_FFI_CALLABLE void reset_set(::moq::moq_tokio_listen_Reset value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_cli_Deprecated deprecated() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> resolved_timeout() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::moq_tokio_listen_Resolved, ::moq::tokio_Error> resolve() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::tokio_Server, ::moq::tokio_Error> init(::moq::tokio_quic_Config quic) &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::tokio_Server, ::moq::tokio_Error> init_streams() &&;
		RUST2_FFI_CALLABLE ::moq::Versions versions() const;
		RUST2_FFI_CALLABLE ::moq::Server server() const;
		RUST2_FFI_CALLABLE bool has_explicit_bind() const;
		RUST2_FFI_CALLABLE bool serves_quic() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_quic_Config {
		RUST2_FFI_UNCONSUMED tokio_quic_Config(tokio_quic_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_quic_Config &operator=(tokio_quic_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE tokio_quic_Config clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static tokio_quic_Config default_();
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> max_streams_get() const;
		RUST2_FFI_CALLABLE void max_streams_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<bool> gso_get() const;
		RUST2_FFI_CALLABLE void gso_set(const ::std::optional<bool> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Duration> idle_timeout_get() const;
		RUST2_FFI_CALLABLE void idle_timeout_set(const ::moq::tokio_Duration *value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Duration> keep_alive_get() const;
		RUST2_FFI_CALLABLE void keep_alive_set(const ::moq::tokio_Duration *value);
		RUST2_FFI_CALLABLE ::std::optional<bool> mtu_discovery_get() const;
		RUST2_FFI_CALLABLE void mtu_discovery_set(const ::std::optional<bool> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_tokio_quic_CongestionControl> congestion_control_get() const;
		RUST2_FFI_CALLABLE void congestion_control_set(const ::std::optional<::moq::moq_tokio_quic_CongestionControl> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> receive_window_get() const;
		RUST2_FFI_CALLABLE void receive_window_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> stream_receive_window_get() const;
		RUST2_FFI_CALLABLE void stream_receive_window_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> send_window_get() const;
		RUST2_FFI_CALLABLE void send_window_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> qlog_get() const;
		RUST2_FFI_CALLABLE void qlog_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> keylog_get() const;
		RUST2_FFI_CALLABLE void keylog_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_cli_Deprecated deprecated() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::tokio_Error> validate() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_Resolved resolve() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_server_Config {
		RUST2_FFI_UNCONSUMED tokio_server_Config(tokio_server_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_server_Config &operator=(tokio_server_Config &&other) noexcept = default;
		static tokio_server_Config default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_tls_Serve> tls_get() const;
		RUST2_FFI_CALLABLE void tls_set(::std::optional<::moq::tokio_tls_Serve> value);
		RUST2_FFI_CALLABLE ::moq::server_Admission admission_get() const;
		RUST2_FFI_CALLABLE void admission_set(::moq::server_Admission value);
		RUST2_FFI_CALLABLE ::moq::tokio_listen_Config listen_get() const;
		RUST2_FFI_CALLABLE void listen_set(::moq::tokio_listen_Config value);
		RUST2_FFI_CALLABLE ::moq::tokio_quic_Config quic_get() const;
		RUST2_FFI_CALLABLE void quic_set(::moq::tokio_quic_Config value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_tokio_websocket_Listener> websocket_get() const;
		RUST2_FFI_CALLABLE void websocket_set(::std::optional<::moq::moq_tokio_websocket_Listener> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::origin_Consumer> publisher_get() const;
		RUST2_FFI_CALLABLE void publisher_set(::std::optional<::moq::origin_Consumer> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::origin_Producer> subscriber_get() const;
		RUST2_FFI_CALLABLE void subscriber_set(::std::optional<::moq::origin_Producer> value);
		RUST2_FFI_CALLABLE ::moq::stats_Session stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(::moq::stats_Session value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_demux_Demux demux_get() const;
		RUST2_FFI_CALLABLE void demux_set(::moq::moq_tokio_quic_demux_Demux value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_server_Streams streams_get() const;
		RUST2_FFI_CALLABLE void streams_set(::moq::moq_tokio_server_Streams value);
		RUST2_FFI_CONSUMES ::moq::result<::moq::tokio_Server, ::moq::tokio_Error> init() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::tokio_Server, ::moq::tokio_Error> init_streams() &&;
	};
	class RUST2_FFI_CONSUMABLE tokio_server_Listener {
		RUST2_FFI_UNCONSUMED tokio_server_Listener(tokio_server_Listener &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_server_Listener &operator=(tokio_server_Listener &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::tokio_server_Listener_accept_future accept();
		RUST2_FFI_CONSUMES ::moq::tokio_server_Listener_close_future close() &&;
		RUST2_FFI_CALLABLE ::moq::tokio_server_Listener_close_streams_future close_streams();
		RUST2_FFI_CALLABLE ::moq::result<::moq::socket_addr, ::moq::tokio_Error> local_addr() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::socket_addr> websocket_local_addr() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::socket_addr> tcp_local_addr() const;
		RUST2_FFI_CALLABLE ::moq::tokio_tls_Certificates certificates() const;
		RUST2_FFI_CALLABLE ::std::vector<::moq::moq_tokio_accept_Health> accept_health() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_server_Request {
		RUST2_FFI_UNCONSUMED tokio_server_Request(tokio_server_Request &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_server_Request &operator=(tokio_server_Request &&other) noexcept = default;
		RUST2_FFI_CONSUMES ::moq::tokio_server_Request_reject_future reject(const ::moq::SessionError &code) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_server_Request with_publisher_Producer(const ::moq::origin_Producer &publish) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_server_Request with_publisher_Consumer(const ::moq::origin_Consumer &publish) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_server_Request with_subscriber(::moq::origin_Producer subscribe) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_server_Request with_peer_node(::std::uint64_t node) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_server_Request with_stats(::moq::stats_Session stats) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_server_Request with_limits(const ::moq::session_Limits &limits) &&;
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_tokio_server_QuicLimits> quic_limits() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::tokio_server_Request, ::moq::buf> with_quic_limits(const ::moq::moq_quic_Limits &limits) &&;
		RUST2_FFI_CALLABLE ::moq::tokio_server_Request_expired_future expired();
		RUST2_FFI_CONSUMES ::moq::tokio_server_Request_ok_future ok() &&;
		RUST2_FFI_CALLABLE ::moq::tokio_Transport transport() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Url> url() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> authority() const;
		RUST2_FFI_CALLABLE ::moq::buf path() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> query() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::socket_addr> remote_addr() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::socket_addr> local_addr() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> server_name() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> alpn() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::setup_Role> role() const;
		RUST2_FFI_CALLABLE ::moq::auth_Handle auth() const;
		RUST2_FFI_CALLABLE ::moq::auth_Requests requests();
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> peer_node() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> peer_implementation() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::setup_Token> token() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_tls_PeerIdentity> peer_identity() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_Addrs {
		RUST2_FFI_UNCONSUMED moq_tokio_Addrs(moq_tokio_Addrs &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_Addrs &operator=(moq_tokio_Addrs &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED moq_tokio_Addrs(const moq_tokio_Addrs &other) = default;
		moq_tokio_Addrs &operator=(const moq_tokio_Addrs &other) = default;
		friend bool operator==(const moq_tokio_Addrs &a, const moq_tokio_Addrs &b);
		friend bool operator!=(const moq_tokio_Addrs &a, const moq_tokio_Addrs &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::moq_tokio_Addrs new_(::moq::moq_tokio_connect_Addr addr);
		RUST2_FFI_CONSUMES ::moq::moq_tokio_Addrs or_(::moq::moq_tokio_connect_Addr addr) &&;
		static ::std::optional<::moq::moq_tokio_Addrs> collect(::std::vector<::moq::moq_tokio_connect_Addr> urls);
		RUST2_FFI_CALLABLE ::std::vector<::moq::moq_tokio_connect_Addr> as_slice() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_cli_Deprecated {
		RUST2_FFI_UNCONSUMED moq_tokio_cli_Deprecated(moq_tokio_cli_Deprecated &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_cli_Deprecated &operator=(moq_tokio_cli_Deprecated &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_cli_Deprecated clone() const;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_tokio_cli_Deprecated default_();
		RUST2_FFI_CALLABLE bool is_empty() const;
		RUST2_FFI_CALLABLE void extend(::moq::moq_tokio_cli_Deprecated other);
		RUST2_FFI_CALLABLE void flag(::moq::input old, ::std::optional<::moq::input> env, ::moq::input new_);
		RUST2_FFI_CALLABLE void changed(::moq::input old, ::std::optional<::moq::input> env, ::moq::input new_, ::moq::input note);
		RUST2_FFI_CALLABLE void removed(::moq::input old, ::std::optional<::moq::input> env, ::moq::input note);
		RUST2_FFI_CALLABLE void toml(::moq::input old, ::moq::input new_, ::std::optional<::moq::input> note);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_cli_Bound {
		RUST2_FFI_UNCONSUMED moq_tokio_cli_Bound(moq_tokio_cli_Bound &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_cli_Bound &operator=(moq_tokio_cli_Bound &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED moq_tokio_cli_Bound(const moq_tokio_cli_Bound &other) = default;
		moq_tokio_cli_Bound &operator=(const moq_tokio_cli_Bound &other) = default;
		friend bool operator==(const moq_tokio_cli_Bound &a, const moq_tokio_cli_Bound &b);
		friend bool operator!=(const moq_tokio_cli_Bound &a, const moq_tokio_cli_Bound &b);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::moq_tokio_cli_Bound NONE();
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_tcp_TlsAcceptor {
		RUST2_FFI_UNCONSUMED moq_tokio_tcp_TlsAcceptor(moq_tokio_tcp_TlsAcceptor &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_tcp_TlsAcceptor &operator=(moq_tokio_tcp_TlsAcceptor &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_tcp_TlsAcceptor clone() const;
		RUST2_FFI_CALLABLE ::moq::rustls_ServerConfig config() const;
		RUST2_FFI_CALLABLE bool offered(const ::moq::items<::moq::view> &alpns) const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_accept_Health {
		RUST2_FFI_UNCONSUMED moq_tokio_accept_Health(moq_tokio_accept_Health &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_accept_Health &operator=(moq_tokio_accept_Health &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED moq_tokio_accept_Health(const moq_tokio_accept_Health &other) = default;
		moq_tokio_accept_Health &operator=(const moq_tokio_accept_Health &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::moq_tokio_accept_Health new_(::moq::input listener);
		RUST2_FFI_CALLABLE ::moq::buf listener() const;
		RUST2_FFI_CALLABLE void accepted() const;
		RUST2_FFI_CALLABLE ::std::uint64_t failures(::moq::moq_tokio_accept_Failure failure) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> stalled() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_connect_Resolved {
		RUST2_FFI_UNCONSUMED moq_tokio_connect_Resolved(moq_tokio_connect_Resolved &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_connect_Resolved &operator=(moq_tokio_connect_Resolved &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_connect_Resolved clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::socket_addr bind_get() const;
		RUST2_FFI_CALLABLE void bind_set(::moq::socket_addr value);
		RUST2_FFI_CALLABLE ::moq::duration race_get() const;
		RUST2_FFI_CALLABLE void race_set(::moq::duration value);
		RUST2_FFI_CALLABLE ::moq::duration resolution_delay_get() const;
		RUST2_FFI_CALLABLE void resolution_delay_set(::moq::duration value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> timeout_get() const;
		RUST2_FFI_CALLABLE void timeout_set(const ::std::optional<::moq::duration> &value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_connection_Handover {
		RUST2_FFI_UNCONSUMED moq_tokio_connection_Handover(moq_tokio_connection_Handover &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_connection_Handover &operator=(moq_tokio_connection_Handover &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_connection_Handover clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_tokio_connection_Handover default_();
		RUST2_FFI_CALLABLE ::moq::moq_tokio_Redirect redirect_get() const;
		RUST2_FFI_CALLABLE void redirect_set(::moq::moq_tokio_Redirect value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Duration> handover_get() const;
		RUST2_FFI_CALLABLE void handover_set(const ::moq::tokio_Duration *value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_websocket_Config {
		RUST2_FFI_UNCONSUMED moq_tokio_websocket_Config(moq_tokio_websocket_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_websocket_Config &operator=(moq_tokio_websocket_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_websocket_Config clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_tokio_websocket_Config default_();
		RUST2_FFI_CALLABLE ::std::optional<bool> enabled_get() const;
		RUST2_FFI_CALLABLE void enabled_set(const ::std::optional<bool> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Duration> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::moq::tokio_Duration *value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_websocket_Resolved resolve() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_connection_Observation {
		RUST2_FFI_UNCONSUMED moq_tokio_connection_Observation(moq_tokio_connection_Observation &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_connection_Observation &operator=(moq_tokio_connection_Observation &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_connection_Observation clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::uint64_t revision_get() const;
		RUST2_FFI_CALLABLE void revision_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t connects_get() const;
		RUST2_FFI_CALLABLE void connects_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> last_connected_get() const;
		RUST2_FFI_CALLABLE void last_connected_set(const ::std::optional<::moq::clock_Instant> &value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_connection_Phase phase_get() const;
		RUST2_FFI_CALLABLE void phase_set(::moq::moq_tokio_connection_Phase value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_connection_Snapshot {
		RUST2_FFI_UNCONSUMED moq_tokio_connection_Snapshot(moq_tokio_connection_Snapshot &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_connection_Snapshot &operator=(moq_tokio_connection_Snapshot &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::session_Stats stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(const ::moq::session_Stats &value);
		RUST2_FFI_CALLABLE ::moq::Version version_get() const;
		RUST2_FFI_CALLABLE void version_set(const ::moq::Version &value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_listen_Resolved {
		RUST2_FFI_UNCONSUMED moq_tokio_listen_Resolved(moq_tokio_listen_Resolved &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_listen_Resolved &operator=(moq_tokio_listen_Resolved &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_listen_Resolved clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_listen_Bind> bind_get() const;
		RUST2_FFI_CALLABLE void bind_set(const ::std::optional<::moq::tokio_listen_Bind> &value);
		RUST2_FFI_CALLABLE ::moq::Versions versions_get() const;
		RUST2_FFI_CALLABLE void versions_set(::moq::Versions value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> implementation_get() const;
		RUST2_FFI_CALLABLE void implementation_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> timeout_get() const;
		RUST2_FFI_CALLABLE void timeout_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::moq::tokio_tls_Listen tls_get() const;
		RUST2_FFI_CALLABLE void tls_set(::moq::tokio_tls_Listen value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::socket_addr_v4> preferred_v4_get() const;
		RUST2_FFI_CALLABLE void preferred_v4_set(const ::std::optional<::moq::socket_addr_v4> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::socket_addr_v6> preferred_v6_get() const;
		RUST2_FFI_CALLABLE void preferred_v6_set(const ::std::optional<::moq::socket_addr_v6> &value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_listen_LoadBalancer lb_get() const;
		RUST2_FFI_CALLABLE void lb_set(::moq::moq_tokio_listen_LoadBalancer value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> reset_secret_get() const;
		RUST2_FFI_CALLABLE void reset_secret_set(::std::optional<::moq::input> value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_tcp_Config {
		RUST2_FFI_UNCONSUMED moq_tokio_tcp_Config(moq_tokio_tcp_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_tcp_Config &operator=(moq_tokio_tcp_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_tcp_Config clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_tokio_tcp_Config default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::socket_addr> bind_get() const;
		RUST2_FFI_CALLABLE void bind_set(const ::std::optional<::moq::socket_addr> &value);
		RUST2_FFI_CALLABLE ::std::optional<bool> tls_get() const;
		RUST2_FFI_CALLABLE void tls_set(const ::std::optional<bool> &value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_unix_Config {
		RUST2_FFI_UNCONSUMED moq_tokio_unix_Config(moq_tokio_unix_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_unix_Config &operator=(moq_tokio_unix_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_unix_Config clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_tokio_unix_Config default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> bind_get() const;
		RUST2_FFI_CALLABLE void bind_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_unix_Allow allow_get() const;
		RUST2_FFI_CALLABLE void allow_set(::moq::moq_tokio_unix_Allow value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_listen_LoadBalancer {
		RUST2_FFI_UNCONSUMED moq_tokio_listen_LoadBalancer(moq_tokio_listen_LoadBalancer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_listen_LoadBalancer &operator=(moq_tokio_listen_LoadBalancer &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_listen_LoadBalancer clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_tokio_listen_LoadBalancer default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_tokio_quic_ServerId> id_get() const;
		RUST2_FFI_CALLABLE void id_set(::std::optional<::moq::moq_tokio_quic_ServerId> value);
		RUST2_FFI_CALLABLE ::std::optional<::std::size_t> nonce_get() const;
		RUST2_FFI_CALLABLE void nonce_set(const ::std::optional<::std::size_t> &value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_listen_Forward forward_get() const;
		RUST2_FFI_CALLABLE void forward_set(::moq::moq_tokio_listen_Forward value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_listen_Reset {
		RUST2_FFI_UNCONSUMED moq_tokio_listen_Reset(moq_tokio_listen_Reset &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_listen_Reset &operator=(moq_tokio_listen_Reset &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_listen_Reset clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_tokio_listen_Reset default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> secret_get() const;
		RUST2_FFI_CALLABLE void secret_set(::std::optional<::moq::input> value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_quic_Resolved {
		RUST2_FFI_UNCONSUMED moq_tokio_quic_Resolved(moq_tokio_quic_Resolved &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_quic_Resolved &operator=(moq_tokio_quic_Resolved &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_quic_Resolved clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_tokio_quic_Resolved default_();
		RUST2_FFI_CALLABLE ::std::uint64_t max_streams_get() const;
		RUST2_FFI_CALLABLE void max_streams_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::optional<bool> gso_get() const;
		RUST2_FFI_CALLABLE void gso_set(const ::std::optional<bool> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> idle_timeout_get() const;
		RUST2_FFI_CALLABLE void idle_timeout_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> keep_alive_get() const;
		RUST2_FFI_CALLABLE void keep_alive_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE bool mtu_discovery_get() const;
		RUST2_FFI_CALLABLE void mtu_discovery_set(bool value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_CongestionControl congestion_control_get() const;
		RUST2_FFI_CALLABLE void congestion_control_set(::moq::moq_tokio_quic_CongestionControl value);
		RUST2_FFI_CALLABLE ::std::uint64_t receive_window_get() const;
		RUST2_FFI_CALLABLE void receive_window_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t stream_receive_window_get() const;
		RUST2_FFI_CALLABLE void stream_receive_window_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t send_window_get() const;
		RUST2_FFI_CALLABLE void send_window_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> qlog_get() const;
		RUST2_FFI_CALLABLE void qlog_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> keylog_get() const;
		RUST2_FFI_CALLABLE void keylog_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::duration keep_alive_interval() const;
		RUST2_FFI_CALLABLE ::moq::moq_quic_Limits limits() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_websocket_Listener {
		RUST2_FFI_UNCONSUMED moq_tokio_websocket_Listener(moq_tokio_websocket_Listener &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_websocket_Listener &operator=(moq_tokio_websocket_Listener &&other) noexcept = default;
		static ::moq::moq_tokio_websocket_Listener_bind_future bind(::moq::socket_addr addr);
		RUST2_FFI_CONSUMES ::moq::result<::moq::moq_tokio_websocket_Listener, ::moq::moq_tokio_websocket_Error> with_protocols(const ::moq::items<::moq::view> &protocols) &&;
		RUST2_FFI_CONSUMES ::moq::moq_tokio_websocket_Listener with_quic(const ::moq::moq_tokio_quic_Resolved &quic) &&;
		RUST2_FFI_CALLABLE ::moq::result<::moq::socket_addr, ::moq::moq_tokio_websocket_Error> local_addr() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_accept_Health accept_health() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_websocket_Listener_accept_future accept() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_quic_demux_Demux {
		RUST2_FFI_UNCONSUMED moq_tokio_quic_demux_Demux(moq_tokio_quic_demux_Demux &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_quic_demux_Demux &operator=(moq_tokio_quic_demux_Demux &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_quic_demux_Demux clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_tokio_quic_demux_Demux default_();
		RUST2_FFI_CALLABLE ::moq::moq_sock_demux_Counters counters() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_demux_WebRtc webrtc() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_server_Streams {
		RUST2_FFI_UNCONSUMED moq_tokio_server_Streams(moq_tokio_server_Streams &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_server_Streams &operator=(moq_tokio_server_Streams &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_tokio_server_Streams default_();
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_server_QuicLimits {
		RUST2_FFI_UNCONSUMED moq_tokio_server_QuicLimits(moq_tokio_server_QuicLimits &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_server_QuicLimits &operator=(moq_tokio_server_QuicLimits &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_server_QuicLimits clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> set(const ::moq::moq_quic_Limits &limits) const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_connect_Addr {
		RUST2_FFI_UNCONSUMED moq_tokio_connect_Addr(moq_tokio_connect_Addr &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_connect_Addr &operator=(moq_tokio_connect_Addr &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED moq_tokio_connect_Addr(const moq_tokio_connect_Addr &other) = default;
		moq_tokio_connect_Addr &operator=(const moq_tokio_connect_Addr &other) = default;
		friend bool operator==(const moq_tokio_connect_Addr &a, const moq_tokio_connect_Addr &b);
		friend bool operator!=(const moq_tokio_connect_Addr &a, const moq_tokio_connect_Addr &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::moq_tokio_connect_Addr new_(::moq::Url url);
		static ::moq::result<::moq::moq_tokio_connect_Addr, ::moq::moq_tokio_connect_Error> pinned(::moq::Url url, const ::std::vector<::moq::socket_addr> &addrs);
		RUST2_FFI_CALLABLE ::moq::Url url() const;
		RUST2_FFI_CALLABLE ::std::optional<::std::vector<::moq::socket_addr>> addresses() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_connection_Phase {
		RUST2_FFI_UNCONSUMED moq_tokio_connection_Phase(moq_tokio_connection_Phase &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_connection_Phase &operator=(moq_tokio_connection_Phase &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED moq_tokio_connection_Phase(const moq_tokio_connection_Phase &other) = default;
		moq_tokio_connection_Phase &operator=(const moq_tokio_connection_Phase &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_quic_ServerId {
		RUST2_FFI_UNCONSUMED moq_tokio_quic_ServerId(moq_tokio_quic_ServerId &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_quic_ServerId &operator=(moq_tokio_quic_ServerId &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED moq_tokio_quic_ServerId(const moq_tokio_quic_ServerId &other) = default;
		moq_tokio_quic_ServerId &operator=(const moq_tokio_quic_ServerId &other) = default;
		friend bool operator==(const moq_tokio_quic_ServerId &a, const moq_tokio_quic_ServerId &b);
		friend bool operator!=(const moq_tokio_quic_ServerId &a, const moq_tokio_quic_ServerId &b);
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_listen_Forward {
		RUST2_FFI_UNCONSUMED moq_tokio_listen_Forward(moq_tokio_listen_Forward &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_listen_Forward &operator=(moq_tokio_listen_Forward &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_listen_Forward clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_tokio_listen_Forward default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::socket_addr> bind_get() const;
		RUST2_FFI_CALLABLE void bind_set(const ::std::optional<::moq::socket_addr> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> key_get() const;
		RUST2_FFI_CALLABLE void key_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> peers_get() const;
		RUST2_FFI_CALLABLE void peers_set(::std::optional<::moq::input> value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_websocket_Resolved {
		RUST2_FFI_UNCONSUMED moq_tokio_websocket_Resolved(moq_tokio_websocket_Resolved &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_websocket_Resolved &operator=(moq_tokio_websocket_Resolved &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_websocket_Resolved clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE bool enabled_get() const;
		RUST2_FFI_CALLABLE void enabled_set(bool value);
		RUST2_FFI_CALLABLE ::moq::duration delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(::moq::duration value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_unix_Allow {
		RUST2_FFI_UNCONSUMED moq_tokio_unix_Allow(moq_tokio_unix_Allow &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_unix_Allow &operator=(moq_tokio_unix_Allow &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_unix_Allow clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_tokio_unix_Allow default_();
		RUST2_FFI_CALLABLE ::std::vector<::std::uint32_t> uid_get() const;
		RUST2_FFI_CALLABLE void uid_set(::moq::slice<::std::uint32_t> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::uint32_t> gid_get() const;
		RUST2_FFI_CALLABLE void gid_set(::moq::slice<::std::uint32_t> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::int32_t> pid_get() const;
		RUST2_FFI_CALLABLE void pid_set(::moq::slice<::std::int32_t> value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_websocket_Error {
		RUST2_FFI_UNCONSUMED moq_tokio_websocket_Error(moq_tokio_websocket_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_websocket_Error &operator=(moq_tokio_websocket_Error &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_quic_demux_WebRtc {
		RUST2_FFI_UNCONSUMED moq_tokio_quic_demux_WebRtc(moq_tokio_quic_demux_WebRtc &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_quic_demux_WebRtc &operator=(moq_tokio_quic_demux_WebRtc &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_demux_WebRtc_recv_future recv() const;
		RUST2_FFI_CALLABLE ::moq::moq_sock_demux_Pin pin(::moq::socket_addr remote) const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_quic_demux_Datagram {
		RUST2_FFI_UNCONSUMED moq_tokio_quic_demux_Datagram(moq_tokio_quic_demux_Datagram &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_quic_demux_Datagram &operator=(moq_tokio_quic_demux_Datagram &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_quic_demux_Datagram clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::socket_addr remote_get() const;
		RUST2_FFI_CALLABLE void remote_set(::moq::socket_addr value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::ip_addr> local_get() const;
		RUST2_FFI_CALLABLE void local_set(const ::std::optional<::moq::ip_addr> &value);
		RUST2_FFI_CALLABLE ::moq::buf data_get() const;
		RUST2_FFI_CALLABLE void data_set(::moq::input value);
	};
	struct tokio_listen_Bind_Addr {
		::moq::socket_addr _0;
	};
	struct tokio_listen_Bind_Host {
		::moq::buf _0;
		::std::uint16_t _1;
	};
	struct moq_tokio_connect_Error_EmptyAddresses {
	};
	struct moq_tokio_connect_Error_UnsupportedScheme {
		::moq::buf _0;
	};
	struct moq_tokio_connect_Error_MissingHost {
	};
	inline bool operator==(::moq::tokio_ConnectError a, ::moq::tokio_ConnectError b);
	inline bool operator!=(::moq::tokio_ConnectError a, ::moq::tokio_ConnectError b);
	inline ::moq::buf tokio_ConnectError_to_string(::moq::tokio_ConnectError self);
	inline ::moq::buf tokio_ConnectError_debug(::moq::tokio_ConnectError self);
	inline bool tokio_ConnectError_is_auth(::moq::tokio_ConnectError self);
	inline bool operator==(::moq::tokio_Status a, ::moq::tokio_Status b);
	inline bool operator!=(::moq::tokio_Status a, ::moq::tokio_Status b);
	inline ::moq::buf tokio_Status_debug(::moq::tokio_Status self);
	inline bool operator==(::moq::tokio_Transport a, ::moq::tokio_Transport b);
	inline bool operator!=(::moq::tokio_Transport a, ::moq::tokio_Transport b);
	inline ::moq::buf tokio_Transport_to_string(::moq::tokio_Transport self);
	inline ::moq::buf tokio_Transport_debug(::moq::tokio_Transport self);
	inline ::moq::buf tokio_Transport_as_str(::moq::tokio_Transport self);
	inline bool operator==(const ::moq::tokio_listen_Bind &a, const ::moq::tokio_listen_Bind &b);
	inline bool operator!=(const ::moq::tokio_listen_Bind &a, const ::moq::tokio_listen_Bind &b);
	inline ::moq::buf tokio_listen_Bind_to_string(const ::moq::tokio_listen_Bind &self);
	inline ::moq::buf tokio_listen_Bind_debug(const ::moq::tokio_listen_Bind &self);
	inline ::moq::result<::moq::socket_addr, ::moq::buf> tokio_listen_Bind_resolve(const ::moq::tokio_listen_Bind &self);
	inline bool operator==(::moq::moq_tokio_quic_CongestionControl a, ::moq::moq_tokio_quic_CongestionControl b);
	inline bool operator!=(::moq::moq_tokio_quic_CongestionControl a, ::moq::moq_tokio_quic_CongestionControl b);
	inline ::moq::buf moq_tokio_quic_CongestionControl_debug(::moq::moq_tokio_quic_CongestionControl self);
	inline moq_tokio_quic_CongestionControl moq_tokio_quic_CongestionControl_default();
	inline bool operator==(::moq::moq_tokio_accept_Failure a, ::moq::moq_tokio_accept_Failure b);
	inline bool operator!=(::moq::moq_tokio_accept_Failure a, ::moq::moq_tokio_accept_Failure b);
	inline ::moq::buf moq_tokio_accept_Failure_to_string(::moq::moq_tokio_accept_Failure self);
	inline ::moq::buf moq_tokio_accept_Failure_debug(::moq::moq_tokio_accept_Failure self);
	inline ::std::vector<::moq::moq_tokio_accept_Failure> moq_tokio_accept_Failure_ALL();
	inline ::moq::buf moq_tokio_accept_Failure_as_str(::moq::moq_tokio_accept_Failure self);
	inline bool operator==(::moq::moq_tokio_Redirect a, ::moq::moq_tokio_Redirect b);
	inline bool operator!=(::moq::moq_tokio_Redirect a, ::moq::moq_tokio_Redirect b);
	inline ::moq::buf moq_tokio_Redirect_debug(::moq::moq_tokio_Redirect self);
	inline moq_tokio_Redirect moq_tokio_Redirect_default();
	inline bool operator==(const ::moq::moq_tokio_connect_Error &a, const ::moq::moq_tokio_connect_Error &b);
	inline bool operator!=(const ::moq::moq_tokio_connect_Error &a, const ::moq::moq_tokio_connect_Error &b);
	inline ::moq::buf moq_tokio_connect_Error_to_string(const ::moq::moq_tokio_connect_Error &self);
	inline ::moq::buf moq_tokio_connect_Error_debug(const ::moq::moq_tokio_connect_Error &self);
	inline ::moq::result<::std::array<::std::uint8_t, 32>, ::moq::tokio_tls_Error> tokio_tls_parse_fingerprint(::moq::input value);
	inline ::moq::origin_Producer tokio_origin_spawn();
}
namespace std {
	template <> struct hash<::moq::tokio_Duration> {
		size_t operator()(const ::moq::tokio_Duration &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::moq::moq_tokio_cli_Bound> {
		size_t operator()(const ::moq::moq_tokio_cli_Bound &value) const noexcept;
	};
}
namespace std {
	template <> struct hash<::moq::moq_tokio_quic_ServerId> {
		size_t operator()(const ::moq::moq_tokio_quic_ServerId &value) const noexcept;
	};
}
#endif

// tracingcore.hpp
#ifndef MOQ_HPP_CRATE4
#define MOQ_HPP_CRATE4
#include "rt.hpp"
namespace moq {
	enum class Level {
		error = 0,
		warn = 1,
		info = 2,
		debug = 3,
		trace = 4,
	};
	inline bool operator==(::moq::Level a, ::moq::Level b);
	inline bool operator!=(::moq::Level a, ::moq::Level b);
	inline bool operator<(::moq::Level a, ::moq::Level b);
	inline bool operator<=(::moq::Level a, ::moq::Level b);
	inline bool operator>(::moq::Level a, ::moq::Level b);
	inline bool operator>=(::moq::Level a, ::moq::Level b);
	#if defined(__cpp_lib_three_way_comparison)
	inline ::std::weak_ordering operator<=>(::moq::Level a, ::moq::Level b);
	#endif
	inline ::moq::buf Level_to_string(::moq::Level self);
}
#endif

// url.hpp
#ifndef MOQ_HPP_CRATE2
#define MOQ_HPP_CRATE2
#include "rt.hpp"
namespace moq {
	class RUST2_FFI_CONSUMABLE Url {
		RUST2_FFI_UNCONSUMED Url(Url &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Url &operator=(Url &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED Url(const Url &other) = default;
		Url &operator=(const Url &other) = default;
		friend bool operator==(const Url &a, const Url &b);
		friend bool operator!=(const Url &a, const Url &b);
		friend bool operator<(const Url &a, const Url &b);
		friend bool operator<=(const Url &a, const Url &b);
		friend bool operator>(const Url &a, const Url &b);
		friend bool operator>=(const Url &a, const Url &b);
		#if defined(__cpp_lib_three_way_comparison)
		friend ::std::weak_ordering operator<=>(const Url &a, const Url &b);
		#endif
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::result<::moq::Url, ::moq::buf> parse(::moq::input input_);
	};
}
namespace std {
	template <> struct hash<::moq::Url> {
		size_t operator()(const ::moq::Url &value) const noexcept;
	};
}
#endif
