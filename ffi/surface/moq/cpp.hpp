// congestion.hpp
#ifndef MOQ_HPP_CRATE11
#define MOQ_HPP_CRATE11
#include "rt.hpp"
namespace moq {
	class RUST2_FFI_CONSUMABLE moq_congestion_Factory {
		RUST2_FFI_UNCONSUMED moq_congestion_Factory(moq_congestion_Factory &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_congestion_Factory &operator=(moq_congestion_Factory &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED moq_congestion_Factory(const moq_congestion_Factory &other) = default;
		moq_congestion_Factory &operator=(const moq_congestion_Factory &other) = default;
	};
}
#endif

// hang.hpp
#ifndef MOQ_HPP_CRATE7
#define MOQ_HPP_CRATE7
#include "rt.hpp"
#include "kio.hpp"
#include "net.hpp"
#include "serdejson.hpp"
namespace moq {
	enum class hang_health_Verdict {
		unknown = 0,
		healthy = 1,
		degraded = 2,
		unhealthy = 3,
	};
	enum class hang_health_Role {
		publisher = 0,
		viewer = 1,
	};
	enum class hang_health_Reset {
		decrease = 0,
		replaced = 1,
	};
	enum class hang_health_Input {
		rtt = 0,
		loss = 1,
		headroom = 2,
		skipped = 3,
		late = 4,
		errors = 5,
		stalled = 6,
	};
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
	enum class hang_catalog_Range {
		limited = 0,
		full = 1,
	};
	inline constexpr ::std::uint64_t hang_catalog_MOQ_EPOCH_UNIX_MILLIS = MOQ_HANG_CATALOG_MOQ_EPOCH_UNIX_MILLIS;
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
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Media media_get() const;
		RUST2_FFI_CALLABLE void media_set(const ::moq::hang_catalog_Media &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Text text_get() const;
		RUST2_FFI_CALLABLE void text_set(::moq::hang_catalog_Text value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Json json_get() const;
		RUST2_FFI_CALLABLE void json_set(::moq::hang_catalog_Json value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Binary binary_get() const;
		RUST2_FFI_CALLABLE void binary_set(::moq::hang_catalog_Binary value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_M2ts> m2ts_get() const;
		RUST2_FFI_CALLABLE void m2ts_set(::std::optional<::moq::hang_catalog_M2ts> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Stats> stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(::std::optional<::moq::hang_catalog_Stats> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Echo> echo_get() const;
		RUST2_FFI_CALLABLE void echo_set(::std::optional<::moq::hang_catalog_Echo> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_Contribution>> contributions_get() const;
		RUST2_FFI_CALLABLE void contributions_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_Contribution>> value);
		static ::moq::buf DEFAULT_NAME();
		static ::moq::track_Info default_track_info();
		static ::moq::track_Subscription default_subscription();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> check_renditions() const;
		RUST2_FFI_CALLABLE ::moq::hang_Catalog media() const;
		static ::moq::hang_Catalog_subscribe_future subscribe(const ::moq::broadcast_Consumer &broadcast);
		static ::moq::result<::moq::hang_Catalog, ::moq::hang_Error> from_slice(::moq::input v);
		static ::moq::result<::moq::hang_Catalog, ::moq::hang_Error> from_reader(::moq::input reader);
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json_pretty() const;
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
	class RUST2_FFI_CONSUMABLE hang_stats_Snapshot_string {
		RUST2_FFI_UNCONSUMED hang_stats_Snapshot_string(hang_stats_Snapshot_string &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_stats_Snapshot_string &operator=(hang_stats_Snapshot_string &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_stats_Snapshot_string clone() const;
		friend bool operator==(const hang_stats_Snapshot_string &a, const hang_stats_Snapshot_string &b);
		friend bool operator!=(const hang_stats_Snapshot_string &a, const hang_stats_Snapshot_string &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_stats_Snapshot_string default_();
		RUST2_FFI_CALLABLE ::moq::hang_stats_Transport transport_get() const;
		RUST2_FFI_CALLABLE void transport_set(::moq::hang_stats_Transport value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_stats_Rendition>> renditions_get() const;
		RUST2_FFI_CALLABLE void renditions_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_stats_Rendition>> value);
		RUST2_FFI_CALLABLE ::moq::buf ext_get() const;
		RUST2_FFI_CALLABLE void ext_set(::moq::input value);
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
	class RUST2_FFI_CONSUMABLE hang_echo_Snapshot_string {
		RUST2_FFI_UNCONSUMED hang_echo_Snapshot_string(hang_echo_Snapshot_string &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_echo_Snapshot_string &operator=(hang_echo_Snapshot_string &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_echo_Snapshot_string clone() const;
		friend bool operator==(const hang_echo_Snapshot_string &a, const hang_echo_Snapshot_string &b);
		friend bool operator!=(const hang_echo_Snapshot_string &a, const hang_echo_Snapshot_string &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_echo_Snapshot_string default_();
		RUST2_FFI_CALLABLE ::moq::hang_stats_Transport transport_get() const;
		RUST2_FFI_CALLABLE void transport_set(::moq::hang_stats_Transport value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_echo_Rendition>> renditions_get() const;
		RUST2_FFI_CALLABLE void renditions_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_echo_Rendition>> value);
		RUST2_FFI_CALLABLE ::moq::buf ext_get() const;
		RUST2_FFI_CALLABLE void ext_set(::moq::input value);
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
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> latency_get() const;
		RUST2_FFI_CALLABLE void latency_set(const ::std::optional<::moq::duration> &value);
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
	class RUST2_FFI_CONSUMABLE hang_health_Observer {
		RUST2_FFI_UNCONSUMED hang_health_Observer(hang_health_Observer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_health_Observer &operator=(hang_health_Observer &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_health_Observer clone() const;
		friend bool operator==(const hang_health_Observer &a, const hang_health_Observer &b);
		friend bool operator!=(const hang_health_Observer &a, const hang_health_Observer &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::Path path_get() const;
		RUST2_FFI_CALLABLE void path_set(::moq::Path value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		static ::moq::hang_health_Observer new_(::moq::Path path, ::std::optional<::moq::Instance> instance);
	};
	class RUST2_FFI_CONSUMABLE hang_health_Report_echo_Snapshot_string {
		RUST2_FFI_UNCONSUMED hang_health_Report_echo_Snapshot_string(hang_health_Report_echo_Snapshot_string &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_health_Report_echo_Snapshot_string &operator=(hang_health_Report_echo_Snapshot_string &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_health_Report_echo_Snapshot_string clone() const;
		friend bool operator==(const hang_health_Report_echo_Snapshot_string &a, const hang_health_Report_echo_Snapshot_string &b);
		friend bool operator!=(const hang_health_Report_echo_Snapshot_string &a, const hang_health_Report_echo_Snapshot_string &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::hang_health_Observer observer_get() const;
		RUST2_FFI_CALLABLE void observer_set(::moq::hang_health_Observer value);
		RUST2_FFI_CALLABLE ::moq::hang_echo_Snapshot_string snapshot_get() const;
		RUST2_FFI_CALLABLE void snapshot_set(::moq::hang_echo_Snapshot_string value);
		static ::moq::hang_health_Report_echo_Snapshot_string new_(::moq::hang_health_Observer observer, ::moq::hang_echo_Snapshot_string snapshot);
	};
	class RUST2_FFI_CONSUMABLE hang_health_Report_stats_Snapshot_string {
		RUST2_FFI_UNCONSUMED hang_health_Report_stats_Snapshot_string(hang_health_Report_stats_Snapshot_string &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_health_Report_stats_Snapshot_string &operator=(hang_health_Report_stats_Snapshot_string &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_health_Report_stats_Snapshot_string clone() const;
		friend bool operator==(const hang_health_Report_stats_Snapshot_string &a, const hang_health_Report_stats_Snapshot_string &b);
		friend bool operator!=(const hang_health_Report_stats_Snapshot_string &a, const hang_health_Report_stats_Snapshot_string &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::hang_health_Observer observer_get() const;
		RUST2_FFI_CALLABLE void observer_set(::moq::hang_health_Observer value);
		RUST2_FFI_CALLABLE ::moq::hang_stats_Snapshot_string snapshot_get() const;
		RUST2_FFI_CALLABLE void snapshot_set(::moq::hang_stats_Snapshot_string value);
		static ::moq::hang_health_Report_stats_Snapshot_string new_(::moq::hang_health_Observer observer, ::moq::hang_stats_Snapshot_string snapshot);
	};
	class RUST2_FFI_CONSUMABLE hang_health_Thresholds {
		RUST2_FFI_UNCONSUMED hang_health_Thresholds(hang_health_Thresholds &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_health_Thresholds &operator=(hang_health_Thresholds &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_health_Thresholds clone() const;
		friend bool operator==(const hang_health_Thresholds &a, const hang_health_Thresholds &b);
		friend bool operator!=(const hang_health_Thresholds &a, const hang_health_Thresholds &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_health_Thresholds default_();
		RUST2_FFI_CALLABLE ::moq::duration stale_get() const;
		RUST2_FFI_CALLABLE void stale_set(::moq::duration value);
		RUST2_FFI_CALLABLE ::moq::hang_health_TransportThresholds transport_get() const;
		RUST2_FFI_CALLABLE void transport_set(::moq::hang_health_TransportThresholds value);
		RUST2_FFI_CALLABLE ::moq::hang_health_PublisherThresholds publisher_get() const;
		RUST2_FFI_CALLABLE void publisher_set(::moq::hang_health_PublisherThresholds value);
		RUST2_FFI_CALLABLE ::moq::hang_health_ViewerThresholds viewer_get() const;
		RUST2_FFI_CALLABLE void viewer_set(::moq::hang_health_ViewerThresholds value);
	};
	class RUST2_FFI_CONSUMABLE hang_health_TransportThresholds {
		RUST2_FFI_UNCONSUMED hang_health_TransportThresholds(hang_health_TransportThresholds &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_health_TransportThresholds &operator=(hang_health_TransportThresholds &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_health_TransportThresholds clone() const;
		friend bool operator==(const hang_health_TransportThresholds &a, const hang_health_TransportThresholds &b);
		friend bool operator!=(const hang_health_TransportThresholds &a, const hang_health_TransportThresholds &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::hang_health_Limits_duration rtt_get() const;
		RUST2_FFI_CALLABLE void rtt_set(const ::moq::hang_health_Limits_duration &value);
		RUST2_FFI_CALLABLE ::moq::hang_health_Limits_f64 loss_get() const;
		RUST2_FFI_CALLABLE void loss_set(const ::moq::hang_health_Limits_f64 &value);
		RUST2_FFI_CALLABLE ::moq::hang_health_Limits_f64 headroom_get() const;
		RUST2_FFI_CALLABLE void headroom_set(const ::moq::hang_health_Limits_f64 &value);
	};
	class RUST2_FFI_CONSUMABLE hang_health_PublisherThresholds {
		RUST2_FFI_UNCONSUMED hang_health_PublisherThresholds(hang_health_PublisherThresholds &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_health_PublisherThresholds &operator=(hang_health_PublisherThresholds &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_health_PublisherThresholds clone() const;
		friend bool operator==(const hang_health_PublisherThresholds &a, const hang_health_PublisherThresholds &b);
		friend bool operator!=(const hang_health_PublisherThresholds &a, const hang_health_PublisherThresholds &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::hang_health_Limits_f64 skipped_get() const;
		RUST2_FFI_CALLABLE void skipped_set(const ::moq::hang_health_Limits_f64 &value);
	};
	class RUST2_FFI_CONSUMABLE hang_health_ViewerThresholds {
		RUST2_FFI_UNCONSUMED hang_health_ViewerThresholds(hang_health_ViewerThresholds &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_health_ViewerThresholds &operator=(hang_health_ViewerThresholds &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_health_ViewerThresholds clone() const;
		friend bool operator==(const hang_health_ViewerThresholds &a, const hang_health_ViewerThresholds &b);
		friend bool operator!=(const hang_health_ViewerThresholds &a, const hang_health_ViewerThresholds &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::hang_health_Limits_f64 late_get() const;
		RUST2_FFI_CALLABLE void late_set(const ::moq::hang_health_Limits_f64 &value);
		RUST2_FFI_CALLABLE ::moq::hang_health_Limits_f64 errors_get() const;
		RUST2_FFI_CALLABLE void errors_set(const ::moq::hang_health_Limits_f64 &value);
		RUST2_FFI_CALLABLE ::moq::hang_health_Limits_f64 stalled_get() const;
		RUST2_FFI_CALLABLE void stalled_set(const ::moq::hang_health_Limits_f64 &value);
	};
	class RUST2_FFI_CONSUMABLE hang_health_Evidence {
		RUST2_FFI_UNCONSUMED hang_health_Evidence(hang_health_Evidence &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_health_Evidence &operator=(hang_health_Evidence &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_health_Evidence clone() const;
		friend bool operator==(const hang_health_Evidence &a, const hang_health_Evidence &b);
		friend bool operator!=(const hang_health_Evidence &a, const hang_health_Evidence &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::hang_health_Input input_get() const;
		RUST2_FFI_CALLABLE void input_set(::moq::hang_health_Input value);
		RUST2_FFI_CALLABLE ::std::optional<double> value_get() const;
		RUST2_FFI_CALLABLE void value_set(const ::std::optional<double> &value);
		RUST2_FFI_CALLABLE ::moq::hang_health_Verdict verdict_get() const;
		RUST2_FFI_CALLABLE void verdict_set(::moq::hang_health_Verdict value);
	};
	class RUST2_FFI_CONSUMABLE hang_health_Connection {
		RUST2_FFI_UNCONSUMED hang_health_Connection(hang_health_Connection &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_health_Connection &operator=(hang_health_Connection &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_health_Connection clone() const;
		friend bool operator==(const hang_health_Connection &a, const hang_health_Connection &b);
		friend bool operator!=(const hang_health_Connection &a, const hang_health_Connection &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_health_Connection default_();
		RUST2_FFI_CALLABLE ::moq::hang_health_Verdict verdict_get() const;
		RUST2_FFI_CALLABLE void verdict_set(::moq::hang_health_Verdict value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::hang_health_Evidence> evidence_get() const;
		RUST2_FFI_CALLABLE void evidence_set(::std::vector<::moq::hang_health_Evidence> value);
		RUST2_FFI_CALLABLE bool stale_get() const;
		RUST2_FFI_CALLABLE void stale_set(bool value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> lost_bytes_get() const;
		RUST2_FFI_CALLABLE void lost_bytes_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> lost_packets_get() const;
		RUST2_FFI_CALLABLE void lost_packets_set(const ::std::optional<::std::uint64_t> &value);
	};
	class RUST2_FFI_CONSUMABLE hang_health_Rendition {
		RUST2_FFI_UNCONSUMED hang_health_Rendition(hang_health_Rendition &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_health_Rendition &operator=(hang_health_Rendition &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_health_Rendition clone() const;
		friend bool operator==(const hang_health_Rendition &a, const hang_health_Rendition &b);
		friend bool operator!=(const hang_health_Rendition &a, const hang_health_Rendition &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_health_Rendition default_();
		RUST2_FFI_CALLABLE ::moq::hang_health_Verdict verdict_get() const;
		RUST2_FFI_CALLABLE void verdict_set(::moq::hang_health_Verdict value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::hang_health_Evidence> evidence_get() const;
		RUST2_FFI_CALLABLE void evidence_set(::std::vector<::moq::hang_health_Evidence> value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> frames_get() const;
		RUST2_FFI_CALLABLE void frames_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bytes_get() const;
		RUST2_FFI_CALLABLE void bytes_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bitrate_get() const;
		RUST2_FFI_CALLABLE void bitrate_set(const ::std::optional<::std::uint64_t> &value);
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Consumer {
		RUST2_FFI_UNCONSUMED hang_catalog_Consumer(hang_catalog_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Consumer &operator=(hang_catalog_Consumer &&other) noexcept = default;
		static ::moq::hang_catalog_Consumer new_(::moq::track_Subscriber track);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Consumer_next_future next();
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
	class RUST2_FFI_CONSUMABLE hang_catalog_Contribution {
		RUST2_FFI_UNCONSUMED hang_catalog_Contribution(hang_catalog_Contribution &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Contribution &operator=(hang_catalog_Contribution &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Contribution clone() const;
		friend bool operator==(const hang_catalog_Contribution &a, const hang_catalog_Contribution &b);
		friend bool operator!=(const hang_catalog_Contribution &a, const hang_catalog_Contribution &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::path_Relative broadcast_get() const;
		RUST2_FFI_CALLABLE void broadcast_set(::moq::path_Relative value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		static ::moq::hang_catalog_Contribution new_(::moq::path_Relative broadcast);
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
	class RUST2_FFI_CONSUMABLE hang_catalog_M2ts {
		RUST2_FFI_UNCONSUMED hang_catalog_M2ts(hang_catalog_M2ts &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_M2ts &operator=(hang_catalog_M2ts &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_M2ts clone() const;
		friend bool operator==(const hang_catalog_M2ts &a, const hang_catalog_M2ts &b);
		friend bool operator!=(const hang_catalog_M2ts &a, const hang_catalog_M2ts &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::buf track_get() const;
		RUST2_FFI_CALLABLE void track_set(::moq::input value);
		RUST2_FFI_CALLABLE bool random_access_get() const;
		RUST2_FFI_CALLABLE void random_access_set(bool value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> mux_rate_get() const;
		RUST2_FFI_CALLABLE void mux_rate_set(const ::std::optional<::std::uint64_t> &value);
		static ::moq::hang_catalog_M2ts new_(::moq::input track);
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Media {
		RUST2_FFI_UNCONSUMED hang_catalog_Media(hang_catalog_Media &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Media &operator=(hang_catalog_Media &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Media clone() const;
		friend bool operator==(const hang_catalog_Media &a, const hang_catalog_Media &b);
		friend bool operator!=(const hang_catalog_Media &a, const hang_catalog_Media &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static hang_catalog_Media default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Clock> clock_get() const;
		RUST2_FFI_CALLABLE void clock_set(const ::std::optional<::moq::hang_catalog_Clock> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Join> join_get() const;
		RUST2_FFI_CALLABLE void join_set(const ::moq::hang_catalog_Join *value);
		RUST2_FFI_CALLABLE bool is_empty() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> interval() const;
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Join {
		RUST2_FFI_UNCONSUMED hang_catalog_Join(hang_catalog_Join &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Join &operator=(hang_catalog_Join &&other) noexcept = default;
		RUST2_FFI_CALLABLE hang_catalog_Join clone() const;
		friend bool operator==(const hang_catalog_Join &a, const hang_catalog_Join &b);
		friend bool operator!=(const hang_catalog_Join &a, const hang_catalog_Join &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::duration interval_get() const;
		RUST2_FFI_CALLABLE void interval_set(::moq::duration value);
		static ::std::optional<::moq::hang_catalog_Join> new_(::moq::duration interval);
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
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Range> range_get() const;
		RUST2_FFI_CALLABLE void range_set(const ::std::optional<::moq::hang_catalog_Range> &value);
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
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> warmup_get() const;
		RUST2_FFI_CALLABLE void warmup_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> delay_get() const;
		RUST2_FFI_CALLABLE void delay_set(const ::std::optional<::moq::duration> &value);
		static ::moq::hang_catalog_VideoConfig new_(const ::moq::hang_catalog_VideoCodec &codec);
		RUST2_FFI_CALLABLE bool same_identity(const ::moq::hang_catalog_VideoConfig &other) const;
	};
	struct hang_health_Limits_f64 {
		double degraded;
		double unhealthy;
		friend bool operator==(const hang_health_Limits_f64 &a, const hang_health_Limits_f64 &b);
		friend bool operator!=(const hang_health_Limits_f64 &a, const hang_health_Limits_f64 &b);
		::moq::buf debug() const;
	};
	struct hang_health_Limits_duration {
		::moq::duration degraded;
		::moq::duration unhealthy;
		friend bool operator==(const hang_health_Limits_duration &a, const hang_health_Limits_duration &b);
		friend bool operator!=(const hang_health_Limits_duration &a, const hang_health_Limits_duration &b);
		::moq::buf debug() const;
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
	inline bool operator==(const hang_health_Limits_f64 &a, const hang_health_Limits_f64 &b);
	inline bool operator==(const hang_health_Limits_duration &a, const hang_health_Limits_duration &b);
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
	inline bool operator==(::moq::hang_health_Verdict a, ::moq::hang_health_Verdict b);
	inline bool operator!=(::moq::hang_health_Verdict a, ::moq::hang_health_Verdict b);
	inline ::moq::buf hang_health_Verdict_debug(::moq::hang_health_Verdict self);
	inline hang_health_Verdict hang_health_Verdict_default();
	inline bool operator==(::moq::hang_health_Role a, ::moq::hang_health_Role b);
	inline bool operator!=(::moq::hang_health_Role a, ::moq::hang_health_Role b);
	inline ::moq::buf hang_health_Role_debug(::moq::hang_health_Role self);
	inline bool operator==(::moq::hang_health_Reset a, ::moq::hang_health_Reset b);
	inline bool operator!=(::moq::hang_health_Reset a, ::moq::hang_health_Reset b);
	inline ::moq::buf hang_health_Reset_debug(::moq::hang_health_Reset self);
	inline bool operator==(::moq::hang_health_Input a, ::moq::hang_health_Input b);
	inline bool operator!=(::moq::hang_health_Input a, ::moq::hang_health_Input b);
	inline ::moq::buf hang_health_Input_debug(::moq::hang_health_Input self);
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
	inline bool operator==(::moq::hang_catalog_Range a, ::moq::hang_catalog_Range b);
	inline bool operator!=(::moq::hang_catalog_Range a, ::moq::hang_catalog_Range b);
	inline ::moq::buf hang_catalog_Range_debug(::moq::hang_catalog_Range self);
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
		::moq::buf debug() const;
		static ::moq::clock_Instant ZERO();
		static ::moq::clock_Instant now();
		::moq::duration elapsed() const;
		::moq::duration duration_since(const ::moq::clock_Instant &earlier) const;
		::std::optional<::moq::duration> checked_duration_since(const ::moq::clock_Instant &earlier) const;
		::moq::duration saturating_duration_since(const ::moq::clock_Instant &earlier) const;
		::std::optional<::moq::clock_Instant> checked_add(::moq::duration duration_) const;
		::std::optional<::moq::clock_Instant> checked_sub(::moq::duration duration_) const;
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
		::moq::buf debug() const;
		static ::moq::clock_Wall UNIX_EPOCH();
		static ::moq::clock_Wall now();
		::std::optional<::moq::duration> checked_duration_since(const ::moq::clock_Wall &earlier) const;
		::moq::duration saturating_duration_since(const ::moq::clock_Wall &earlier) const;
		::std::optional<::moq::clock_Wall> checked_add(::moq::duration duration_) const;
		::moq::clock_Wall saturating_add(::moq::duration duration_) const;
		::std::optional<::moq::clock_Wall> checked_sub(::moq::duration duration_) const;
	};
	inline bool operator==(const clock_Instant &a, const clock_Instant &b);
	inline bool operator==(const clock_Elapsed &a, const clock_Elapsed &b);
	inline bool operator==(const clock_Wall &a, const clock_Wall &b);
	inline bool operator==(::moq::clock_MissedTickBehavior a, ::moq::clock_MissedTickBehavior b);
	inline bool operator!=(::moq::clock_MissedTickBehavior a, ::moq::clock_MissedTickBehavior b);
	inline ::moq::buf clock_MissedTickBehavior_debug(::moq::clock_MissedTickBehavior self);
	inline clock_MissedTickBehavior clock_MissedTickBehavior_default();
	inline ::moq::clock_Instant clock_now();
	inline ::moq::clock_Wall clock_wall();
	inline ::std::tuple<::moq::clock_Instant, ::moq::clock_Wall> clock_origin();
	inline ::moq::clock_Rng clock_rng();
	inline ::std::uint64_t clock_seed();
	template <class F0> inline ::moq::clock_drive_future clock_drive(F0 &&poll);
	inline ::moq::clock_Interval clock_interval(::moq::duration period);
	inline ::moq::clock_Interval clock_interval_at(const ::moq::clock_Instant &start, ::moq::duration period);
	inline ::moq::clock_sleep_future clock_sleep(::moq::duration duration_);
	inline ::moq::clock_sleep_until_future clock_sleep_until(const ::moq::clock_Instant &at);
	template <class F0> inline ::moq::clock_timeout_future clock_timeout(::moq::duration duration_, F0 future_);
	template <class F0> inline ::moq::clock_timeout_at_future clock_timeout_at(const ::moq::clock_Instant &deadline, F0 future_);
}
#endif

// mp4atom.hpp
#ifndef MOQ_HPP_CRATE8
#define MOQ_HPP_CRATE8
#include "rt.hpp"
namespace moq {
	enum class mp4_atom_SencBoxVersion {
		v0 = 0,
		v1 = 1,
		v2 = 2,
	};
	enum class mp4_atom_ClockType {
		unknown = 0,
		does_not_sync = 1,
		can_sync = 2,
		reserved = 3,
	};
	enum class mp4_atom_ReferenceTime {
		input = 0,
		output = 1,
		finalised = 2,
		written = 3,
		consistent = 4,
		real_time = 5,
	};
	enum class mp4_atom_VexuProjKind {
		rectilinear = 0,
		equirectangular = 1,
		half_equirectangular = 2,
		fisheye = 3,
		parametric_immersive = 4,
	};
	enum class mp4_atom_AudioChannelPosition {
		front_left = 0,
		front_right = 1,
		front_center = 2,
		lfe1 = 3,
		left_surround = 4,
		right_surround = 5,
		front_left_of_center = 6,
		front_right_of_center = 7,
		rear_left = 8,
		rear_right = 9,
		rear_center = 10,
		surround_left = 11,
		surround_right = 12,
		side_left = 13,
		side_right = 14,
		front_left_wide = 15,
		front_right_wide = 16,
		top_front_left = 17,
		top_front_right = 18,
		top_front_center = 19,
		top_rear_left = 20,
		top_rear_right = 21,
		top_rear_center = 22,
		top_side_left = 23,
		top_side_right = 24,
		top_center = 25,
		lfe2 = 26,
		bottom_front_left = 27,
		bottom_front_right = 28,
		bottom_front_center = 29,
		top_surround_left = 30,
		top_surround_right = 31,
		lfe3 = 36,
		leos = 37,
		reos = 38,
		hwbcal = 39,
		hwbcar = 40,
		lbs = 41,
		rbs = 42,
		left_ear = 43,
		right_ear = 44,
		unknown = 127,
	};
	inline constexpr ::std::uint8_t mp4_atom_St3d_MONO = MOQ_MP4_ATOM_ST3D_MONO;
	inline constexpr ::std::uint8_t mp4_atom_St3d_TOP_BOTTOM = MOQ_MP4_ATOM_ST3D_TOP_BOTTOM;
	inline constexpr ::std::uint8_t mp4_atom_St3d_LEFT_RIGHT = MOQ_MP4_ATOM_ST3D_LEFT_RIGHT;
	inline constexpr ::std::uint8_t mp4_atom_St3d_STEREO_CUSTOM = MOQ_MP4_ATOM_ST3D_STEREO_CUSTOM;
	inline constexpr ::std::uint8_t mp4_atom_St3d_RIGHT_LEFT = MOQ_MP4_ATOM_ST3D_RIGHT_LEFT;
	inline constexpr ::std::uint32_t mp4_atom_Hfov_DENOMINATOR = MOQ_MP4_ATOM_HFOV_DENOMINATOR;
	inline constexpr ::std::uint8_t mp4_atom_esds_EsDescriptor_TAG = MOQ_MP4_ATOM_ESDS_ESDESCRIPTOR_TAG;
	inline constexpr ::std::uint8_t mp4_atom_esds_DecoderConfig_TAG = MOQ_MP4_ATOM_ESDS_DECODERCONFIG_TAG;
	inline constexpr ::std::uint8_t mp4_atom_esds_SLConfig_TAG = MOQ_MP4_ATOM_ESDS_SLCONFIG_TAG;
	inline constexpr ::std::uint32_t mp4_atom_u24_MAX = MOQ_MP4_ATOM_U24_MAX;
	inline constexpr ::std::uint8_t mp4_atom_Hero_NONE = MOQ_MP4_ATOM_HERO_NONE;
	inline constexpr ::std::uint8_t mp4_atom_Hero_LEFT = MOQ_MP4_ATOM_HERO_LEFT;
	inline constexpr ::std::uint8_t mp4_atom_Hero_RIGHT = MOQ_MP4_ATOM_HERO_RIGHT;
	inline constexpr ::std::uint8_t mp4_atom_esds_DecoderSpecific_TAG = MOQ_MP4_ATOM_ESDS_DECODERSPECIFIC_TAG;
	inline constexpr ::std::int32_t mp4_atom_Dadj_DENOMINATOR = MOQ_MP4_ATOM_DADJ_DENOMINATOR;
	class RUST2_FFI_CONSUMABLE mp4_atom_Codec {
		RUST2_FFI_UNCONSUMED mp4_atom_Codec(mp4_atom_Codec &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Codec &operator=(mp4_atom_Codec &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mp4_atom_Codec(const mp4_atom_Codec &other) = default;
		mp4_atom_Codec &operator=(const mp4_atom_Codec &other) = default;
		friend bool operator==(const mp4_atom_Codec &a, const mp4_atom_Codec &b);
		friend bool operator!=(const mp4_atom_Codec &a, const mp4_atom_Codec &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Visual> visual() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Audio> audio() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_MetaData> metadata() const;
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Trak {
		RUST2_FFI_UNCONSUMED mp4_atom_Trak(mp4_atom_Trak &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Trak &operator=(mp4_atom_Trak &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Trak clone() const;
		friend bool operator==(const mp4_atom_Trak &a, const mp4_atom_Trak &b);
		friend bool operator!=(const mp4_atom_Trak &a, const mp4_atom_Trak &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Trak default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Tkhd tkhd_get() const;
		RUST2_FFI_CALLABLE void tkhd_set(const ::moq::mp4_atom_Tkhd &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Edts> edts_get() const;
		RUST2_FFI_CALLABLE void edts_set(const ::std::optional<::moq::mp4_atom_Edts> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Meta> meta_get() const;
		RUST2_FFI_CALLABLE void meta_set(::std::optional<::moq::mp4_atom_Meta> value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Mdia mdia_get() const;
		RUST2_FFI_CALLABLE void mdia_set(::moq::mp4_atom_Mdia value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Senc> senc_get() const;
		RUST2_FFI_CALLABLE void senc_set(const ::std::optional<::moq::mp4_atom_Senc> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Tref> tref_get() const;
		RUST2_FFI_CALLABLE void tref_set(const ::std::optional<::moq::mp4_atom_Tref> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Udta> udta_get() const;
		RUST2_FFI_CALLABLE void udta_set(::std::optional<::moq::mp4_atom_Udta> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Avc1 {
		RUST2_FFI_UNCONSUMED mp4_atom_Avc1(mp4_atom_Avc1 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Avc1 &operator=(mp4_atom_Avc1 &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Avc1 clone() const;
		friend bool operator==(const mp4_atom_Avc1 &a, const mp4_atom_Avc1 &b);
		friend bool operator!=(const mp4_atom_Avc1 &a, const mp4_atom_Avc1 &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Avc1 default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Visual visual_get() const;
		RUST2_FFI_CALLABLE void visual_set(::moq::mp4_atom_Visual value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Avcc avcc_get() const;
		RUST2_FFI_CALLABLE void avcc_set(const ::moq::mp4_atom_Avcc &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Btrt> btrt_get() const;
		RUST2_FFI_CALLABLE void btrt_set(const ::std::optional<::moq::mp4_atom_Btrt> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Colr> colr_get() const;
		RUST2_FFI_CALLABLE void colr_set(const ::std::optional<::moq::mp4_atom_Colr> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Pasp> pasp_get() const;
		RUST2_FFI_CALLABLE void pasp_set(const ::std::optional<::moq::mp4_atom_Pasp> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Taic> taic_get() const;
		RUST2_FFI_CALLABLE void taic_set(const ::std::optional<::moq::mp4_atom_Taic> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Fiel> fiel_get() const;
		RUST2_FFI_CALLABLE void fiel_set(const ::std::optional<::moq::mp4_atom_Fiel> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Spatial> spatial_get() const;
		RUST2_FFI_CALLABLE void spatial_set(::std::optional<::moq::mp4_atom_Spatial> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Avc3 {
		RUST2_FFI_UNCONSUMED mp4_atom_Avc3(mp4_atom_Avc3 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Avc3 &operator=(mp4_atom_Avc3 &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Avc3 clone() const;
		friend bool operator==(const mp4_atom_Avc3 &a, const mp4_atom_Avc3 &b);
		friend bool operator!=(const mp4_atom_Avc3 &a, const mp4_atom_Avc3 &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Avc3 default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Visual visual_get() const;
		RUST2_FFI_CALLABLE void visual_set(::moq::mp4_atom_Visual value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Avcc avcc_get() const;
		RUST2_FFI_CALLABLE void avcc_set(const ::moq::mp4_atom_Avcc &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Btrt> btrt_get() const;
		RUST2_FFI_CALLABLE void btrt_set(const ::std::optional<::moq::mp4_atom_Btrt> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Colr> colr_get() const;
		RUST2_FFI_CALLABLE void colr_set(const ::std::optional<::moq::mp4_atom_Colr> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Pasp> pasp_get() const;
		RUST2_FFI_CALLABLE void pasp_set(const ::std::optional<::moq::mp4_atom_Pasp> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Taic> taic_get() const;
		RUST2_FFI_CALLABLE void taic_set(const ::std::optional<::moq::mp4_atom_Taic> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Fiel> fiel_get() const;
		RUST2_FFI_CALLABLE void fiel_set(const ::std::optional<::moq::mp4_atom_Fiel> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Spatial> spatial_get() const;
		RUST2_FFI_CALLABLE void spatial_set(::std::optional<::moq::mp4_atom_Spatial> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Jpeg {
		RUST2_FFI_UNCONSUMED mp4_atom_Jpeg(mp4_atom_Jpeg &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Jpeg &operator=(mp4_atom_Jpeg &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Jpeg clone() const;
		friend bool operator==(const mp4_atom_Jpeg &a, const mp4_atom_Jpeg &b);
		friend bool operator!=(const mp4_atom_Jpeg &a, const mp4_atom_Jpeg &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Jpeg default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Visual visual_get() const;
		RUST2_FFI_CALLABLE void visual_set(::moq::mp4_atom_Visual value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Btrt> btrt_get() const;
		RUST2_FFI_CALLABLE void btrt_set(const ::std::optional<::moq::mp4_atom_Btrt> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Colr> colr_get() const;
		RUST2_FFI_CALLABLE void colr_set(const ::std::optional<::moq::mp4_atom_Colr> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Pasp> pasp_get() const;
		RUST2_FFI_CALLABLE void pasp_set(const ::std::optional<::moq::mp4_atom_Pasp> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Fiel> fiel_get() const;
		RUST2_FFI_CALLABLE void fiel_set(const ::std::optional<::moq::mp4_atom_Fiel> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Spatial> spatial_get() const;
		RUST2_FFI_CALLABLE void spatial_set(::std::optional<::moq::mp4_atom_Spatial> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Hev1 {
		RUST2_FFI_UNCONSUMED mp4_atom_Hev1(mp4_atom_Hev1 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Hev1 &operator=(mp4_atom_Hev1 &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Hev1 clone() const;
		friend bool operator==(const mp4_atom_Hev1 &a, const mp4_atom_Hev1 &b);
		friend bool operator!=(const mp4_atom_Hev1 &a, const mp4_atom_Hev1 &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Hev1 default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Visual visual_get() const;
		RUST2_FFI_CALLABLE void visual_set(::moq::mp4_atom_Visual value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Hvcc hvcc_get() const;
		RUST2_FFI_CALLABLE void hvcc_set(const ::moq::mp4_atom_Hvcc &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Lhvc> lhvc_get() const;
		RUST2_FFI_CALLABLE void lhvc_set(const ::std::optional<::moq::mp4_atom_Lhvc> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Btrt> btrt_get() const;
		RUST2_FFI_CALLABLE void btrt_set(const ::std::optional<::moq::mp4_atom_Btrt> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Colr> colr_get() const;
		RUST2_FFI_CALLABLE void colr_set(const ::std::optional<::moq::mp4_atom_Colr> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Pasp> pasp_get() const;
		RUST2_FFI_CALLABLE void pasp_set(const ::std::optional<::moq::mp4_atom_Pasp> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Taic> taic_get() const;
		RUST2_FFI_CALLABLE void taic_set(const ::std::optional<::moq::mp4_atom_Taic> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Fiel> fiel_get() const;
		RUST2_FFI_CALLABLE void fiel_set(const ::std::optional<::moq::mp4_atom_Fiel> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Spatial> spatial_get() const;
		RUST2_FFI_CALLABLE void spatial_set(::std::optional<::moq::mp4_atom_Spatial> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Hvc1 {
		RUST2_FFI_UNCONSUMED mp4_atom_Hvc1(mp4_atom_Hvc1 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Hvc1 &operator=(mp4_atom_Hvc1 &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Hvc1 clone() const;
		friend bool operator==(const mp4_atom_Hvc1 &a, const mp4_atom_Hvc1 &b);
		friend bool operator!=(const mp4_atom_Hvc1 &a, const mp4_atom_Hvc1 &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Hvc1 default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Visual visual_get() const;
		RUST2_FFI_CALLABLE void visual_set(::moq::mp4_atom_Visual value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Hvcc hvcc_get() const;
		RUST2_FFI_CALLABLE void hvcc_set(const ::moq::mp4_atom_Hvcc &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Lhvc> lhvc_get() const;
		RUST2_FFI_CALLABLE void lhvc_set(const ::std::optional<::moq::mp4_atom_Lhvc> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Btrt> btrt_get() const;
		RUST2_FFI_CALLABLE void btrt_set(const ::std::optional<::moq::mp4_atom_Btrt> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Colr> colr_get() const;
		RUST2_FFI_CALLABLE void colr_set(const ::std::optional<::moq::mp4_atom_Colr> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Pasp> pasp_get() const;
		RUST2_FFI_CALLABLE void pasp_set(const ::std::optional<::moq::mp4_atom_Pasp> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Taic> taic_get() const;
		RUST2_FFI_CALLABLE void taic_set(const ::std::optional<::moq::mp4_atom_Taic> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Fiel> fiel_get() const;
		RUST2_FFI_CALLABLE void fiel_set(const ::std::optional<::moq::mp4_atom_Fiel> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Ccst> ccst_get() const;
		RUST2_FFI_CALLABLE void ccst_set(const ::std::optional<::moq::mp4_atom_Ccst> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Spatial> spatial_get() const;
		RUST2_FFI_CALLABLE void spatial_set(::std::optional<::moq::mp4_atom_Spatial> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Vp08 {
		RUST2_FFI_UNCONSUMED mp4_atom_Vp08(mp4_atom_Vp08 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Vp08 &operator=(mp4_atom_Vp08 &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Vp08 clone() const;
		friend bool operator==(const mp4_atom_Vp08 &a, const mp4_atom_Vp08 &b);
		friend bool operator!=(const mp4_atom_Vp08 &a, const mp4_atom_Vp08 &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Vp08 default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Visual visual_get() const;
		RUST2_FFI_CALLABLE void visual_set(::moq::mp4_atom_Visual value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_VpcC vpcc_get() const;
		RUST2_FFI_CALLABLE void vpcc_set(const ::moq::mp4_atom_VpcC &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Btrt> btrt_get() const;
		RUST2_FFI_CALLABLE void btrt_set(const ::std::optional<::moq::mp4_atom_Btrt> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Colr> colr_get() const;
		RUST2_FFI_CALLABLE void colr_set(const ::std::optional<::moq::mp4_atom_Colr> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Pasp> pasp_get() const;
		RUST2_FFI_CALLABLE void pasp_set(const ::std::optional<::moq::mp4_atom_Pasp> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Spatial> spatial_get() const;
		RUST2_FFI_CALLABLE void spatial_set(::std::optional<::moq::mp4_atom_Spatial> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Vp09 {
		RUST2_FFI_UNCONSUMED mp4_atom_Vp09(mp4_atom_Vp09 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Vp09 &operator=(mp4_atom_Vp09 &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Vp09 clone() const;
		friend bool operator==(const mp4_atom_Vp09 &a, const mp4_atom_Vp09 &b);
		friend bool operator!=(const mp4_atom_Vp09 &a, const mp4_atom_Vp09 &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Vp09 default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Visual visual_get() const;
		RUST2_FFI_CALLABLE void visual_set(::moq::mp4_atom_Visual value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_VpcC vpcc_get() const;
		RUST2_FFI_CALLABLE void vpcc_set(const ::moq::mp4_atom_VpcC &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Btrt> btrt_get() const;
		RUST2_FFI_CALLABLE void btrt_set(const ::std::optional<::moq::mp4_atom_Btrt> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Colr> colr_get() const;
		RUST2_FFI_CALLABLE void colr_set(const ::std::optional<::moq::mp4_atom_Colr> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Pasp> pasp_get() const;
		RUST2_FFI_CALLABLE void pasp_set(const ::std::optional<::moq::mp4_atom_Pasp> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Spatial> spatial_get() const;
		RUST2_FFI_CALLABLE void spatial_set(::std::optional<::moq::mp4_atom_Spatial> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Av01 {
		RUST2_FFI_UNCONSUMED mp4_atom_Av01(mp4_atom_Av01 &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Av01 &operator=(mp4_atom_Av01 &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Av01 clone() const;
		friend bool operator==(const mp4_atom_Av01 &a, const mp4_atom_Av01 &b);
		friend bool operator!=(const mp4_atom_Av01 &a, const mp4_atom_Av01 &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Av01 default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Visual visual_get() const;
		RUST2_FFI_CALLABLE void visual_set(::moq::mp4_atom_Visual value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Av1c av1c_get() const;
		RUST2_FFI_CALLABLE void av1c_set(const ::moq::mp4_atom_Av1c &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Btrt> btrt_get() const;
		RUST2_FFI_CALLABLE void btrt_set(const ::std::optional<::moq::mp4_atom_Btrt> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Ccst> ccst_get() const;
		RUST2_FFI_CALLABLE void ccst_set(const ::std::optional<::moq::mp4_atom_Ccst> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Colr> colr_get() const;
		RUST2_FFI_CALLABLE void colr_set(const ::std::optional<::moq::mp4_atom_Colr> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Pasp> pasp_get() const;
		RUST2_FFI_CALLABLE void pasp_set(const ::std::optional<::moq::mp4_atom_Pasp> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Taic> taic_get() const;
		RUST2_FFI_CALLABLE void taic_set(const ::std::optional<::moq::mp4_atom_Taic> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Spatial> spatial_get() const;
		RUST2_FFI_CALLABLE void spatial_set(::std::optional<::moq::mp4_atom_Spatial> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Tx3g {
		RUST2_FFI_UNCONSUMED mp4_atom_Tx3g(mp4_atom_Tx3g &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Tx3g &operator=(mp4_atom_Tx3g &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Tx3g clone() const;
		friend bool operator==(const mp4_atom_Tx3g &a, const mp4_atom_Tx3g &b);
		friend bool operator!=(const mp4_atom_Tx3g &a, const mp4_atom_Tx3g &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Tx3g default_();
		RUST2_FFI_CALLABLE ::std::uint16_t data_reference_index_get() const;
		RUST2_FFI_CALLABLE void data_reference_index_set(::std::uint16_t value);
		RUST2_FFI_CALLABLE ::std::uint32_t display_flags_get() const;
		RUST2_FFI_CALLABLE void display_flags_set(::std::uint32_t value);
		RUST2_FFI_CALLABLE ::std::int8_t horizontal_justification_get() const;
		RUST2_FFI_CALLABLE void horizontal_justification_set(::std::int8_t value);
		RUST2_FFI_CALLABLE ::std::int8_t vertical_justification_get() const;
		RUST2_FFI_CALLABLE void vertical_justification_set(::std::int8_t value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_RgbaColor bg_color_rgba_get() const;
		RUST2_FFI_CALLABLE void bg_color_rgba_set(const ::moq::mp4_atom_RgbaColor &value);
		RUST2_FFI_CALLABLE ::std::array<::std::uint8_t, 12> style_record_get() const;
		RUST2_FFI_CALLABLE void style_record_set(const ::std::array<::std::uint8_t, 12> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Ftab> ftab_get() const;
		RUST2_FFI_CALLABLE void ftab_set(const ::std::optional<::moq::mp4_atom_Ftab> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Disp> disp_get() const;
		RUST2_FFI_CALLABLE void disp_set(const ::std::optional<::moq::mp4_atom_Disp> &value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Meta {
		RUST2_FFI_UNCONSUMED mp4_atom_Meta(mp4_atom_Meta &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Meta &operator=(mp4_atom_Meta &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Meta clone() const;
		friend bool operator==(const mp4_atom_Meta &a, const mp4_atom_Meta &b);
		friend bool operator!=(const mp4_atom_Meta &a, const mp4_atom_Meta &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Hdlr hdlr_get() const;
		RUST2_FFI_CALLABLE void hdlr_set(const ::moq::mp4_atom_Hdlr &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Any> items_get() const;
		RUST2_FFI_CALLABLE void items_set(::std::vector<::moq::mp4_atom_Any> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Pitm> get_Pitm() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Dinf> get_Dinf() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Iloc> get_Iloc() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Iinf> get_Iinf() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Iprp> get_Iprp() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Iref> get_Iref() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Idat> get_Idat() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Ilst> get_Ilst() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Keys> get_Keys() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Pitm> get_mut_Pitm();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Dinf> get_mut_Dinf();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Iloc> get_mut_Iloc();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Iinf> get_mut_Iinf();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Iprp> get_mut_Iprp();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Iref> get_mut_Iref();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Idat> get_mut_Idat();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Ilst> get_mut_Ilst();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Keys> get_mut_Keys();
		RUST2_FFI_CALLABLE void push_Pitm(const ::moq::mp4_atom_Pitm &atom);
		RUST2_FFI_CALLABLE void push_Dinf(const ::moq::mp4_atom_Dinf &atom);
		RUST2_FFI_CALLABLE void push_Iloc(const ::moq::mp4_atom_Iloc &atom);
		RUST2_FFI_CALLABLE void push_Iinf(const ::moq::mp4_atom_Iinf &atom);
		RUST2_FFI_CALLABLE void push_Iprp(::moq::mp4_atom_Iprp atom);
		RUST2_FFI_CALLABLE void push_Iref(const ::moq::mp4_atom_Iref &atom);
		RUST2_FFI_CALLABLE void push_Idat(const ::moq::mp4_atom_Idat &atom);
		RUST2_FFI_CALLABLE void push_Ilst(const ::moq::mp4_atom_Ilst &atom);
		RUST2_FFI_CALLABLE void push_Keys(const ::moq::mp4_atom_Keys &atom);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Pitm> remove_Pitm();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Dinf> remove_Dinf();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Iloc> remove_Iloc();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Iinf> remove_Iinf();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Iprp> remove_Iprp();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Iref> remove_Iref();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Idat> remove_Idat();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Ilst> remove_Ilst();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Keys> remove_Keys();
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Mdia {
		RUST2_FFI_UNCONSUMED mp4_atom_Mdia(mp4_atom_Mdia &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Mdia &operator=(mp4_atom_Mdia &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Mdia clone() const;
		friend bool operator==(const mp4_atom_Mdia &a, const mp4_atom_Mdia &b);
		friend bool operator!=(const mp4_atom_Mdia &a, const mp4_atom_Mdia &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Mdia default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Mdhd mdhd_get() const;
		RUST2_FFI_CALLABLE void mdhd_set(const ::moq::mp4_atom_Mdhd &value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Hdlr hdlr_get() const;
		RUST2_FFI_CALLABLE void hdlr_set(const ::moq::mp4_atom_Hdlr &value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Minf minf_get() const;
		RUST2_FFI_CALLABLE void minf_set(::moq::mp4_atom_Minf value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Udta {
		RUST2_FFI_UNCONSUMED mp4_atom_Udta(mp4_atom_Udta &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Udta &operator=(mp4_atom_Udta &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Udta clone() const;
		friend bool operator==(const mp4_atom_Udta &a, const mp4_atom_Udta &b);
		friend bool operator!=(const mp4_atom_Udta &a, const mp4_atom_Udta &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Udta default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Chpl> chpl_get() const;
		RUST2_FFI_CALLABLE void chpl_set(const ::std::optional<::moq::mp4_atom_Chpl> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Cprt> cprt_get() const;
		RUST2_FFI_CALLABLE void cprt_set(const ::std::vector<::moq::mp4_atom_Cprt> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Kind> kind_get() const;
		RUST2_FFI_CALLABLE void kind_set(const ::std::vector<::moq::mp4_atom_Kind> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Meta> meta_get() const;
		RUST2_FFI_CALLABLE void meta_set(::std::optional<::moq::mp4_atom_Meta> value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Rtng> rtng_get() const;
		RUST2_FFI_CALLABLE void rtng_set(const ::std::vector<::moq::mp4_atom_Rtng> &value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Visual {
		RUST2_FFI_UNCONSUMED mp4_atom_Visual(mp4_atom_Visual &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Visual &operator=(mp4_atom_Visual &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Visual clone() const;
		friend bool operator==(const mp4_atom_Visual &a, const mp4_atom_Visual &b);
		friend bool operator!=(const mp4_atom_Visual &a, const mp4_atom_Visual &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Visual default_();
		RUST2_FFI_CALLABLE ::std::uint16_t data_reference_index_get() const;
		RUST2_FFI_CALLABLE void data_reference_index_set(::std::uint16_t value);
		RUST2_FFI_CALLABLE ::std::uint16_t width_get() const;
		RUST2_FFI_CALLABLE void width_set(::std::uint16_t value);
		RUST2_FFI_CALLABLE ::std::uint16_t height_get() const;
		RUST2_FFI_CALLABLE void height_set(::std::uint16_t value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_FixedPoint_u16 horizresolution_get() const;
		RUST2_FFI_CALLABLE void horizresolution_set(const ::moq::mp4_atom_FixedPoint_u16 &value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_FixedPoint_u16 vertresolution_get() const;
		RUST2_FFI_CALLABLE void vertresolution_set(const ::moq::mp4_atom_FixedPoint_u16 &value);
		RUST2_FFI_CALLABLE ::std::uint16_t frame_count_get() const;
		RUST2_FFI_CALLABLE void frame_count_set(::std::uint16_t value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Compressor compressor_get() const;
		RUST2_FFI_CALLABLE void compressor_set(::moq::mp4_atom_Compressor value);
		RUST2_FFI_CALLABLE ::std::uint16_t depth_get() const;
		RUST2_FFI_CALLABLE void depth_set(::std::uint16_t value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Spatial {
		RUST2_FFI_UNCONSUMED mp4_atom_Spatial(mp4_atom_Spatial &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Spatial &operator=(mp4_atom_Spatial &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Spatial clone() const;
		friend bool operator==(const mp4_atom_Spatial &a, const mp4_atom_Spatial &b);
		friend bool operator!=(const mp4_atom_Spatial &a, const mp4_atom_Spatial &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Spatial default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_St3d> st3d_get() const;
		RUST2_FFI_CALLABLE void st3d_set(const ::std::optional<::moq::mp4_atom_St3d> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Sv3d> sv3d_get() const;
		RUST2_FFI_CALLABLE void sv3d_set(::std::optional<::moq::mp4_atom_Sv3d> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Vexu> vexu_get() const;
		RUST2_FFI_CALLABLE void vexu_set(::std::optional<::moq::mp4_atom_Vexu> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Hfov> hfov_get() const;
		RUST2_FFI_CALLABLE void hfov_set(const ::std::optional<::moq::mp4_atom_Hfov> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Any> unknown_get() const;
		RUST2_FFI_CALLABLE void unknown_set(::std::vector<::moq::mp4_atom_Any> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Any {
		RUST2_FFI_UNCONSUMED mp4_atom_Any(mp4_atom_Any &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Any &operator=(mp4_atom_Any &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mp4_atom_Any(const mp4_atom_Any &other) = default;
		mp4_atom_Any &operator=(const mp4_atom_Any &other) = default;
		friend bool operator==(const mp4_atom_Any &a, const mp4_atom_Any &b);
		friend bool operator!=(const mp4_atom_Any &a, const mp4_atom_Any &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::mp4_atom_FourCC kind() const;
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Minf {
		RUST2_FFI_UNCONSUMED mp4_atom_Minf(mp4_atom_Minf &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Minf &operator=(mp4_atom_Minf &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Minf clone() const;
		friend bool operator==(const mp4_atom_Minf &a, const mp4_atom_Minf &b);
		friend bool operator!=(const mp4_atom_Minf &a, const mp4_atom_Minf &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Minf default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Vmhd> vmhd_get() const;
		RUST2_FFI_CALLABLE void vmhd_set(const ::std::optional<::moq::mp4_atom_Vmhd> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Smhd> smhd_get() const;
		RUST2_FFI_CALLABLE void smhd_set(const ::std::optional<::moq::mp4_atom_Smhd> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Nmhd> nmhd_get() const;
		RUST2_FFI_CALLABLE void nmhd_set(::std::optional<::moq::mp4_atom_Nmhd> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Sthd> sthd_get() const;
		RUST2_FFI_CALLABLE void sthd_set(::std::optional<::moq::mp4_atom_Sthd> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Hmhd> hmhd_get() const;
		RUST2_FFI_CALLABLE void hmhd_set(const ::std::optional<::moq::mp4_atom_Hmhd> &value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Dinf dinf_get() const;
		RUST2_FFI_CALLABLE void dinf_set(const ::moq::mp4_atom_Dinf &value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Stbl stbl_get() const;
		RUST2_FFI_CALLABLE void stbl_set(::moq::mp4_atom_Stbl value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Compressor {
		RUST2_FFI_UNCONSUMED mp4_atom_Compressor(mp4_atom_Compressor &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Compressor &operator=(mp4_atom_Compressor &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mp4_atom_Compressor(const mp4_atom_Compressor &other) = default;
		mp4_atom_Compressor &operator=(const mp4_atom_Compressor &other) = default;
		friend bool operator==(const mp4_atom_Compressor &a, const mp4_atom_Compressor &b);
		friend bool operator!=(const mp4_atom_Compressor &a, const mp4_atom_Compressor &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Compressor default_();
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Sv3d {
		RUST2_FFI_UNCONSUMED mp4_atom_Sv3d(mp4_atom_Sv3d &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Sv3d &operator=(mp4_atom_Sv3d &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Sv3d clone() const;
		friend bool operator==(const mp4_atom_Sv3d &a, const mp4_atom_Sv3d &b);
		friend bool operator!=(const mp4_atom_Sv3d &a, const mp4_atom_Sv3d &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Sv3d default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Svhd> svhd_get() const;
		RUST2_FFI_CALLABLE void svhd_set(const ::std::optional<::moq::mp4_atom_Svhd> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Proj> proj_get() const;
		RUST2_FFI_CALLABLE void proj_set(::std::optional<::moq::mp4_atom_Proj> value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Any> unknown_get() const;
		RUST2_FFI_CALLABLE void unknown_set(::std::vector<::moq::mp4_atom_Any> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Vexu {
		RUST2_FFI_UNCONSUMED mp4_atom_Vexu(mp4_atom_Vexu &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Vexu &operator=(mp4_atom_Vexu &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Vexu clone() const;
		friend bool operator==(const mp4_atom_Vexu &a, const mp4_atom_Vexu &b);
		friend bool operator!=(const mp4_atom_Vexu &a, const mp4_atom_Vexu &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Vexu default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Must> must_get() const;
		RUST2_FFI_CALLABLE void must_set(const ::std::optional<::moq::mp4_atom_Must> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_VexuProj> proj_get() const;
		RUST2_FFI_CALLABLE void proj_set(::std::optional<::moq::mp4_atom_VexuProj> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Eyes> eyes_get() const;
		RUST2_FFI_CALLABLE void eyes_set(::std::optional<::moq::mp4_atom_Eyes> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Pack> pack_get() const;
		RUST2_FFI_CALLABLE void pack_set(::std::optional<::moq::mp4_atom_Pack> value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Any> unknown_get() const;
		RUST2_FFI_CALLABLE void unknown_set(::std::vector<::moq::mp4_atom_Any> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Iprp {
		RUST2_FFI_UNCONSUMED mp4_atom_Iprp(mp4_atom_Iprp &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Iprp &operator=(mp4_atom_Iprp &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Iprp clone() const;
		friend bool operator==(const mp4_atom_Iprp &a, const mp4_atom_Iprp &b);
		friend bool operator!=(const mp4_atom_Iprp &a, const mp4_atom_Iprp &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Iprp default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Ipco ipco_get() const;
		RUST2_FFI_CALLABLE void ipco_set(::moq::mp4_atom_Ipco value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Ipma> ipma_get() const;
		RUST2_FFI_CALLABLE void ipma_set(const ::std::vector<::moq::mp4_atom_Ipma> &value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Ipco {
		RUST2_FFI_UNCONSUMED mp4_atom_Ipco(mp4_atom_Ipco &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Ipco &operator=(mp4_atom_Ipco &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Ipco clone() const;
		friend bool operator==(const mp4_atom_Ipco &a, const mp4_atom_Ipco &b);
		friend bool operator!=(const mp4_atom_Ipco &a, const mp4_atom_Ipco &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Ipco default_();
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Any> properties_get() const;
		RUST2_FFI_CALLABLE void properties_set(::std::vector<::moq::mp4_atom_Any> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Moov {
		RUST2_FFI_UNCONSUMED mp4_atom_Moov(mp4_atom_Moov &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Moov &operator=(mp4_atom_Moov &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Moov clone() const;
		friend bool operator==(const mp4_atom_Moov &a, const mp4_atom_Moov &b);
		friend bool operator!=(const mp4_atom_Moov &a, const mp4_atom_Moov &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Moov default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Mvhd mvhd_get() const;
		RUST2_FFI_CALLABLE void mvhd_set(const ::moq::mp4_atom_Mvhd &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Meta> meta_get() const;
		RUST2_FFI_CALLABLE void meta_set(::std::optional<::moq::mp4_atom_Meta> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Mvex> mvex_get() const;
		RUST2_FFI_CALLABLE void mvex_set(const ::std::optional<::moq::mp4_atom_Mvex> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Trak> trak_get() const;
		RUST2_FFI_CALLABLE void trak_set(::std::vector<::moq::mp4_atom_Trak> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Udta> udta_get() const;
		RUST2_FFI_CALLABLE void udta_set(::std::optional<::moq::mp4_atom_Udta> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Ainf> ainf_get() const;
		RUST2_FFI_CALLABLE void ainf_set(const ::std::optional<::moq::mp4_atom_Ainf> &value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Stbl {
		RUST2_FFI_UNCONSUMED mp4_atom_Stbl(mp4_atom_Stbl &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Stbl &operator=(mp4_atom_Stbl &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Stbl clone() const;
		friend bool operator==(const mp4_atom_Stbl &a, const mp4_atom_Stbl &b);
		friend bool operator!=(const mp4_atom_Stbl &a, const mp4_atom_Stbl &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Stbl default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Stsd stsd_get() const;
		RUST2_FFI_CALLABLE void stsd_set(::moq::mp4_atom_Stsd value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Stts stts_get() const;
		RUST2_FFI_CALLABLE void stts_set(const ::moq::mp4_atom_Stts &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Ctts> ctts_get() const;
		RUST2_FFI_CALLABLE void ctts_set(const ::std::optional<::moq::mp4_atom_Ctts> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Stss> stss_get() const;
		RUST2_FFI_CALLABLE void stss_set(const ::std::optional<::moq::mp4_atom_Stss> &value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Stsc stsc_get() const;
		RUST2_FFI_CALLABLE void stsc_set(const ::moq::mp4_atom_Stsc &value);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Stsz stsz_get() const;
		RUST2_FFI_CALLABLE void stsz_set(const ::moq::mp4_atom_Stsz &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Stco> stco_get() const;
		RUST2_FFI_CALLABLE void stco_set(const ::std::optional<::moq::mp4_atom_Stco> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Co64> co64_get() const;
		RUST2_FFI_CALLABLE void co64_set(const ::std::optional<::moq::mp4_atom_Co64> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Sbgp> sbgp_get() const;
		RUST2_FFI_CALLABLE void sbgp_set(const ::std::vector<::moq::mp4_atom_Sbgp> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Sgpd> sgpd_get() const;
		RUST2_FFI_CALLABLE void sgpd_set(const ::std::vector<::moq::mp4_atom_Sgpd> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Subs> subs_get() const;
		RUST2_FFI_CALLABLE void subs_set(const ::std::vector<::moq::mp4_atom_Subs> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Saiz> saiz_get() const;
		RUST2_FFI_CALLABLE void saiz_set(const ::std::vector<::moq::mp4_atom_Saiz> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Saio> saio_get() const;
		RUST2_FFI_CALLABLE void saio_set(const ::std::vector<::moq::mp4_atom_Saio> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Cslg> cslg_get() const;
		RUST2_FFI_CALLABLE void cslg_set(const ::std::optional<::moq::mp4_atom_Cslg> &value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Stsd {
		RUST2_FFI_UNCONSUMED mp4_atom_Stsd(mp4_atom_Stsd &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Stsd &operator=(mp4_atom_Stsd &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Stsd clone() const;
		friend bool operator==(const mp4_atom_Stsd &a, const mp4_atom_Stsd &b);
		friend bool operator!=(const mp4_atom_Stsd &a, const mp4_atom_Stsd &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Stsd default_();
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Codec> codecs_get() const;
		RUST2_FFI_CALLABLE void codecs_set(::std::vector<::moq::mp4_atom_Codec> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Nmhd {
		RUST2_FFI_UNCONSUMED mp4_atom_Nmhd(mp4_atom_Nmhd &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Nmhd &operator=(mp4_atom_Nmhd &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mp4_atom_Nmhd(const mp4_atom_Nmhd &other) = default;
		mp4_atom_Nmhd &operator=(const mp4_atom_Nmhd &other) = default;
		friend bool operator==(const mp4_atom_Nmhd &a, const mp4_atom_Nmhd &b);
		friend bool operator!=(const mp4_atom_Nmhd &a, const mp4_atom_Nmhd &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Nmhd default_();
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Sthd {
		RUST2_FFI_UNCONSUMED mp4_atom_Sthd(mp4_atom_Sthd &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Sthd &operator=(mp4_atom_Sthd &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mp4_atom_Sthd(const mp4_atom_Sthd &other) = default;
		mp4_atom_Sthd &operator=(const mp4_atom_Sthd &other) = default;
		friend bool operator==(const mp4_atom_Sthd &a, const mp4_atom_Sthd &b);
		friend bool operator!=(const mp4_atom_Sthd &a, const mp4_atom_Sthd &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Sthd default_();
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Moof {
		RUST2_FFI_UNCONSUMED mp4_atom_Moof(mp4_atom_Moof &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Moof &operator=(mp4_atom_Moof &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Moof clone() const;
		friend bool operator==(const mp4_atom_Moof &a, const mp4_atom_Moof &b);
		friend bool operator!=(const mp4_atom_Moof &a, const mp4_atom_Moof &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Moof default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Mfhd mfhd_get() const;
		RUST2_FFI_CALLABLE void mfhd_set(const ::moq::mp4_atom_Mfhd &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Traf> traf_get() const;
		RUST2_FFI_CALLABLE void traf_set(::std::vector<::moq::mp4_atom_Traf> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Traf {
		RUST2_FFI_UNCONSUMED mp4_atom_Traf(mp4_atom_Traf &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Traf &operator=(mp4_atom_Traf &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Traf clone() const;
		friend bool operator==(const mp4_atom_Traf &a, const mp4_atom_Traf &b);
		friend bool operator!=(const mp4_atom_Traf &a, const mp4_atom_Traf &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Traf default_();
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Tfhd tfhd_get() const;
		RUST2_FFI_CALLABLE void tfhd_set(const ::moq::mp4_atom_Tfhd &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Tfdt> tfdt_get() const;
		RUST2_FFI_CALLABLE void tfdt_set(const ::std::optional<::moq::mp4_atom_Tfdt> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Trun> trun_get() const;
		RUST2_FFI_CALLABLE void trun_set(const ::std::vector<::moq::mp4_atom_Trun> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Sbgp> sbgp_get() const;
		RUST2_FFI_CALLABLE void sbgp_set(const ::std::vector<::moq::mp4_atom_Sbgp> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Sgpd> sgpd_get() const;
		RUST2_FFI_CALLABLE void sgpd_set(const ::std::vector<::moq::mp4_atom_Sgpd> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Subs> subs_get() const;
		RUST2_FFI_CALLABLE void subs_set(const ::std::vector<::moq::mp4_atom_Subs> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Saiz> saiz_get() const;
		RUST2_FFI_CALLABLE void saiz_set(const ::std::vector<::moq::mp4_atom_Saiz> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Saio> saio_get() const;
		RUST2_FFI_CALLABLE void saio_set(const ::std::vector<::moq::mp4_atom_Saio> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Meta> meta_get() const;
		RUST2_FFI_CALLABLE void meta_set(::std::optional<::moq::mp4_atom_Meta> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Senc> senc_get() const;
		RUST2_FFI_CALLABLE void senc_set(const ::std::optional<::moq::mp4_atom_Senc> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Udta> udta_get() const;
		RUST2_FFI_CALLABLE void udta_set(::std::optional<::moq::mp4_atom_Udta> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Proj {
		RUST2_FFI_UNCONSUMED mp4_atom_Proj(mp4_atom_Proj &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Proj &operator=(mp4_atom_Proj &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Proj clone() const;
		friend bool operator==(const mp4_atom_Proj &a, const mp4_atom_Proj &b);
		friend bool operator!=(const mp4_atom_Proj &a, const mp4_atom_Proj &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Proj default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Prhd> prhd_get() const;
		RUST2_FFI_CALLABLE void prhd_set(const ::std::optional<::moq::mp4_atom_Prhd> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_ProjKind> kind_get() const;
		RUST2_FFI_CALLABLE void kind_set(const ::std::optional<::moq::mp4_atom_ProjKind> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Any> unknown_get() const;
		RUST2_FFI_CALLABLE void unknown_set(::std::vector<::moq::mp4_atom_Any> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_VexuProj {
		RUST2_FFI_UNCONSUMED mp4_atom_VexuProj(mp4_atom_VexuProj &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_VexuProj &operator=(mp4_atom_VexuProj &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_VexuProj clone() const;
		friend bool operator==(const mp4_atom_VexuProj &a, const mp4_atom_VexuProj &b);
		friend bool operator!=(const mp4_atom_VexuProj &a, const mp4_atom_VexuProj &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_VexuProj default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Must> must_get() const;
		RUST2_FFI_CALLABLE void must_set(const ::std::optional<::moq::mp4_atom_Must> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Prji> prji_get() const;
		RUST2_FFI_CALLABLE void prji_set(const ::std::optional<::moq::mp4_atom_Prji> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_VexuProjKind> kind_get() const;
		RUST2_FFI_CALLABLE void kind_set(const ::std::optional<::moq::mp4_atom_VexuProjKind> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Any> unknown_get() const;
		RUST2_FFI_CALLABLE void unknown_set(::std::vector<::moq::mp4_atom_Any> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Eyes {
		RUST2_FFI_UNCONSUMED mp4_atom_Eyes(mp4_atom_Eyes &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Eyes &operator=(mp4_atom_Eyes &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Eyes clone() const;
		friend bool operator==(const mp4_atom_Eyes &a, const mp4_atom_Eyes &b);
		friend bool operator!=(const mp4_atom_Eyes &a, const mp4_atom_Eyes &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Eyes default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Must> must_get() const;
		RUST2_FFI_CALLABLE void must_set(const ::std::optional<::moq::mp4_atom_Must> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Stri> stri_get() const;
		RUST2_FFI_CALLABLE void stri_set(const ::std::optional<::moq::mp4_atom_Stri> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Hero> hero_get() const;
		RUST2_FFI_CALLABLE void hero_set(const ::std::optional<::moq::mp4_atom_Hero> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Cams> cams_get() const;
		RUST2_FFI_CALLABLE void cams_set(::std::optional<::moq::mp4_atom_Cams> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Cmfy> cmfy_get() const;
		RUST2_FFI_CALLABLE void cmfy_set(::std::optional<::moq::mp4_atom_Cmfy> value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Any> unknown_get() const;
		RUST2_FFI_CALLABLE void unknown_set(::std::vector<::moq::mp4_atom_Any> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Pack {
		RUST2_FFI_UNCONSUMED mp4_atom_Pack(mp4_atom_Pack &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Pack &operator=(mp4_atom_Pack &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Pack clone() const;
		friend bool operator==(const mp4_atom_Pack &a, const mp4_atom_Pack &b);
		friend bool operator!=(const mp4_atom_Pack &a, const mp4_atom_Pack &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Pack default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Must> must_get() const;
		RUST2_FFI_CALLABLE void must_set(const ::std::optional<::moq::mp4_atom_Must> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Pkin> pkin_get() const;
		RUST2_FFI_CALLABLE void pkin_set(const ::std::optional<::moq::mp4_atom_Pkin> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Any> unknown_get() const;
		RUST2_FFI_CALLABLE void unknown_set(::std::vector<::moq::mp4_atom_Any> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Cams {
		RUST2_FFI_UNCONSUMED mp4_atom_Cams(mp4_atom_Cams &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Cams &operator=(mp4_atom_Cams &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Cams clone() const;
		friend bool operator==(const mp4_atom_Cams &a, const mp4_atom_Cams &b);
		friend bool operator!=(const mp4_atom_Cams &a, const mp4_atom_Cams &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Cams default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Must> must_get() const;
		RUST2_FFI_CALLABLE void must_set(const ::std::optional<::moq::mp4_atom_Must> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Blin> blin_get() const;
		RUST2_FFI_CALLABLE void blin_set(const ::std::optional<::moq::mp4_atom_Blin> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Any> unknown_get() const;
		RUST2_FFI_CALLABLE void unknown_set(::std::vector<::moq::mp4_atom_Any> value);
	};
	class RUST2_FFI_CONSUMABLE mp4_atom_Cmfy {
		RUST2_FFI_UNCONSUMED mp4_atom_Cmfy(mp4_atom_Cmfy &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mp4_atom_Cmfy &operator=(mp4_atom_Cmfy &&other) noexcept = default;
		RUST2_FFI_CALLABLE mp4_atom_Cmfy clone() const;
		friend bool operator==(const mp4_atom_Cmfy &a, const mp4_atom_Cmfy &b);
		friend bool operator!=(const mp4_atom_Cmfy &a, const mp4_atom_Cmfy &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static mp4_atom_Cmfy default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Must> must_get() const;
		RUST2_FFI_CALLABLE void must_set(const ::std::optional<::moq::mp4_atom_Must> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::mp4_atom_Dadj> dadj_get() const;
		RUST2_FFI_CALLABLE void dadj_set(const ::std::optional<::moq::mp4_atom_Dadj> &value);
		RUST2_FFI_CALLABLE ::std::vector<::moq::mp4_atom_Any> unknown_get() const;
		RUST2_FFI_CALLABLE void unknown_set(::std::vector<::moq::mp4_atom_Any> value);
	};
	class mp4_atom_FourCC {
		friend bool operator==(const mp4_atom_FourCC &a, const mp4_atom_FourCC &b);
		friend bool operator!=(const mp4_atom_FourCC &a, const mp4_atom_FourCC &b);
		::moq::buf to_string() const;
		::moq::buf debug() const;
		static ::moq::mp4_atom_FourCC new_(const ::std::array<::std::uint8_t, 4> &value);
	};
	class mp4_atom_FixedPoint_u16 {
		friend bool operator==(const mp4_atom_FixedPoint_u16 &a, const mp4_atom_FixedPoint_u16 &b);
		friend bool operator!=(const mp4_atom_FixedPoint_u16 &a, const mp4_atom_FixedPoint_u16 &b);
		::moq::buf debug() const;
		static mp4_atom_FixedPoint_u16 default_();
		static ::moq::mp4_atom_FixedPoint_u16 new_(::std::uint16_t int_, ::std::uint16_t dec);
		::std::uint16_t integer() const;
		::std::uint16_t decimal() const;
	};
	struct mp4_atom_Audio {
		::std::uint16_t data_reference_index;
		::std::uint16_t channel_count;
		::std::uint16_t sample_size;
		::moq::mp4_atom_FixedPoint_u16 sample_rate;
		friend bool operator==(const mp4_atom_Audio &a, const mp4_atom_Audio &b);
		friend bool operator!=(const mp4_atom_Audio &a, const mp4_atom_Audio &b);
		::moq::buf debug() const;
	};
	class mp4_atom_u24 {
		friend bool operator==(const mp4_atom_u24 &a, const mp4_atom_u24 &b);
		friend bool operator!=(const mp4_atom_u24 &a, const mp4_atom_u24 &b);
		::moq::buf debug() const;
		static mp4_atom_u24 default_();
	};
	struct mp4_atom_esds_DecoderSpecific {
		::std::uint8_t profile;
		::std::uint8_t freq_index;
		::std::uint8_t chan_conf;
		::moq::buf raw;
		friend bool operator==(const mp4_atom_esds_DecoderSpecific &a, const mp4_atom_esds_DecoderSpecific &b);
		friend bool operator!=(const mp4_atom_esds_DecoderSpecific &a, const mp4_atom_esds_DecoderSpecific &b);
		::moq::buf debug() const;
		static mp4_atom_esds_DecoderSpecific default_();
	};
	struct mp4_atom_esds_DecoderConfig {
		::std::uint8_t object_type_indication;
		::std::uint8_t stream_type;
		::std::uint8_t up_stream;
		::moq::mp4_atom_u24 buffer_size_db;
		::std::uint32_t max_bitrate;
		::std::uint32_t avg_bitrate;
		::std::optional<::moq::mp4_atom_esds_DecoderSpecific> dec_specific;
		friend bool operator==(const mp4_atom_esds_DecoderConfig &a, const mp4_atom_esds_DecoderConfig &b);
		friend bool operator!=(const mp4_atom_esds_DecoderConfig &a, const mp4_atom_esds_DecoderConfig &b);
		::moq::buf debug() const;
		static mp4_atom_esds_DecoderConfig default_();
	};
	class mp4_atom_esds_SLConfig {
		friend bool operator==(const mp4_atom_esds_SLConfig &a, const mp4_atom_esds_SLConfig &b);
		friend bool operator!=(const mp4_atom_esds_SLConfig &a, const mp4_atom_esds_SLConfig &b);
		::moq::buf debug() const;
		static mp4_atom_esds_SLConfig default_();
	};
	struct mp4_atom_esds_EsDescriptor {
		::std::uint16_t es_id;
		::std::optional<::std::uint16_t> depends_on_es_id;
		::std::optional<::moq::buf> url;
		::std::optional<::std::uint16_t> ocr_es_id;
		::std::uint8_t stream_priority;
		::moq::mp4_atom_esds_DecoderConfig dec_config;
		::moq::mp4_atom_esds_SLConfig sl_config;
		friend bool operator==(const mp4_atom_esds_EsDescriptor &a, const mp4_atom_esds_EsDescriptor &b);
		friend bool operator!=(const mp4_atom_esds_EsDescriptor &a, const mp4_atom_esds_EsDescriptor &b);
		::moq::buf debug() const;
		static mp4_atom_esds_EsDescriptor default_();
	};
	struct mp4_atom_Esds {
		::moq::mp4_atom_esds_EsDescriptor es_desc;
		friend bool operator==(const mp4_atom_Esds &a, const mp4_atom_Esds &b);
		friend bool operator!=(const mp4_atom_Esds &a, const mp4_atom_Esds &b);
		::moq::buf debug() const;
		static mp4_atom_Esds default_();
	};
	struct mp4_atom_Btrt {
		::std::uint32_t buffer_size_db;
		::std::uint32_t max_bitrate;
		::std::uint32_t avg_bitrate;
		friend bool operator==(const mp4_atom_Btrt &a, const mp4_atom_Btrt &b);
		friend bool operator!=(const mp4_atom_Btrt &a, const mp4_atom_Btrt &b);
		::moq::buf debug() const;
		static mp4_atom_Btrt default_();
		static ::moq::result<::moq::mp4_atom_Btrt, ::moq::buf> new_(::std::uint32_t buffer_size_db_, ::std::uint32_t max_bitrate_, ::std::uint32_t avg_bitrate_);
	};
	struct mp4_atom_Taic {
		::std::uint64_t time_uncertainty;
		::std::uint32_t clock_resolution;
		::std::int32_t clock_drift_rate;
		::moq::mp4_atom_ClockType clock_type;
		friend bool operator==(const mp4_atom_Taic &a, const mp4_atom_Taic &b);
		friend bool operator!=(const mp4_atom_Taic &a, const mp4_atom_Taic &b);
		::moq::buf debug() const;
		static mp4_atom_Taic default_();
		static ::moq::result<::moq::mp4_atom_Taic, ::moq::buf> new_(::std::uint64_t time_uncertainty_, ::std::uint32_t clock_resolution_, ::std::int32_t clock_drift_rate_, ::moq::mp4_atom_ClockType clock_type_);
	};
	struct mp4_atom_Mp4a {
		::moq::mp4_atom_Audio audio;
		::moq::mp4_atom_Esds esds;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		::std::optional<::moq::mp4_atom_Taic> taic;
		friend bool operator==(const mp4_atom_Mp4a &a, const mp4_atom_Mp4a &b);
		friend bool operator!=(const mp4_atom_Mp4a &a, const mp4_atom_Mp4a &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Mp3 {
		::moq::mp4_atom_Audio audio;
		friend bool operator==(const mp4_atom_Mp3 &a, const mp4_atom_Mp3 &b);
		friend bool operator!=(const mp4_atom_Mp3 &a, const mp4_atom_Mp3 &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_OpusChannelMapping {
		::std::uint8_t family;
		::std::uint8_t stream_count;
		::std::uint8_t coupled_count;
		::moq::buf channel_mapping;
		friend bool operator==(const mp4_atom_OpusChannelMapping &a, const mp4_atom_OpusChannelMapping &b);
		friend bool operator!=(const mp4_atom_OpusChannelMapping &a, const mp4_atom_OpusChannelMapping &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Dops {
		::std::uint8_t output_channel_count;
		::std::uint16_t pre_skip;
		::std::uint32_t input_sample_rate;
		::std::int16_t output_gain;
		::std::optional<::moq::mp4_atom_OpusChannelMapping> channel_mapping;
		friend bool operator==(const mp4_atom_Dops &a, const mp4_atom_Dops &b);
		friend bool operator!=(const mp4_atom_Dops &a, const mp4_atom_Dops &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Opus {
		::moq::mp4_atom_Audio audio;
		::moq::mp4_atom_Dops dops;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_Opus &a, const mp4_atom_Opus &b);
		friend bool operator!=(const mp4_atom_Opus &a, const mp4_atom_Opus &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_FlacMetadataBlock_StreamInfo {
		::std::uint16_t minimum_block_size;
		::std::uint16_t maximum_block_size;
		::moq::mp4_atom_u24 minimum_frame_size;
		::moq::mp4_atom_u24 maximum_frame_size;
		::std::uint32_t sample_rate;
		::std::uint8_t num_channels_minus_one;
		::std::uint8_t bits_per_sample_minus_one;
		::std::uint64_t number_of_interchannel_samples;
		::moq::buf md5_checksum;
	};
	struct mp4_atom_FlacMetadataBlock_Padding {
	};
	struct mp4_atom_FlacMetadataBlock_Application {
	};
	struct mp4_atom_FlacMetadataBlock_SeekTable {
	};
	struct mp4_atom_FlacMetadataBlock_VorbisComment {
		::moq::buf vendor_string;
		::std::vector<::moq::buf> comments;
	};
	struct mp4_atom_FlacMetadataBlock_CueSheet {
	};
	struct mp4_atom_FlacMetadataBlock_Picture {
	};
	struct mp4_atom_FlacMetadataBlock_Reserved {
	};
	struct mp4_atom_FlacMetadataBlock_Forbidden {
	};
	struct mp4_atom_Dfla {
		::std::vector<::moq::mp4_atom_FlacMetadataBlock> blocks;
		friend bool operator==(const mp4_atom_Dfla &a, const mp4_atom_Dfla &b);
		friend bool operator!=(const mp4_atom_Dfla &a, const mp4_atom_Dfla &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Flac {
		::moq::mp4_atom_Audio audio;
		::moq::mp4_atom_Dfla dfla;
		friend bool operator==(const mp4_atom_Flac &a, const mp4_atom_Flac &b);
		friend bool operator!=(const mp4_atom_Flac &a, const mp4_atom_Flac &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Ac3SpecificBox {
		::std::uint8_t fscod;
		::std::uint8_t bsid;
		::std::uint8_t bsmod;
		::std::uint8_t acmod;
		bool lfeon;
		::std::uint8_t bit_rate_code;
		friend bool operator==(const mp4_atom_Ac3SpecificBox &a, const mp4_atom_Ac3SpecificBox &b);
		friend bool operator!=(const mp4_atom_Ac3SpecificBox &a, const mp4_atom_Ac3SpecificBox &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Ac3 {
		::moq::mp4_atom_Audio audio;
		::moq::mp4_atom_Ac3SpecificBox dac3;
		friend bool operator==(const mp4_atom_Ac3 &a, const mp4_atom_Ac3 &b);
		friend bool operator!=(const mp4_atom_Ac3 &a, const mp4_atom_Ac3 &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Ec3IndependentSubstream {
		::std::uint8_t fscod;
		::std::uint8_t bsid;
		bool asvc;
		::std::uint8_t bsmod;
		::std::uint8_t acmod;
		bool lfeon;
		::std::uint8_t num_dep_sub;
		::std::optional<::std::uint16_t> chan_loc;
		friend bool operator==(const mp4_atom_Ec3IndependentSubstream &a, const mp4_atom_Ec3IndependentSubstream &b);
		friend bool operator!=(const mp4_atom_Ec3IndependentSubstream &a, const mp4_atom_Ec3IndependentSubstream &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Ec3SpecificBox {
		::std::uint16_t data_rate;
		::std::vector<::moq::mp4_atom_Ec3IndependentSubstream> substreams;
		friend bool operator==(const mp4_atom_Ec3SpecificBox &a, const mp4_atom_Ec3SpecificBox &b);
		friend bool operator!=(const mp4_atom_Ec3SpecificBox &a, const mp4_atom_Ec3SpecificBox &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Eac3 {
		::moq::mp4_atom_Audio audio;
		::moq::mp4_atom_Ec3SpecificBox dec3;
		friend bool operator==(const mp4_atom_Eac3 &a, const mp4_atom_Eac3 &b);
		friend bool operator!=(const mp4_atom_Eac3 &a, const mp4_atom_Eac3 &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_PcmC {
		bool big_endian;
		::std::uint8_t sample_size;
		friend bool operator==(const mp4_atom_PcmC &a, const mp4_atom_PcmC &b);
		friend bool operator!=(const mp4_atom_PcmC &a, const mp4_atom_PcmC &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_ExplicitSpeakerPosition {
		::std::int16_t azimuth;
		::std::int8_t elevation;
		friend bool operator==(const mp4_atom_ExplicitSpeakerPosition &a, const mp4_atom_ExplicitSpeakerPosition &b);
		friend bool operator!=(const mp4_atom_ExplicitSpeakerPosition &a, const mp4_atom_ExplicitSpeakerPosition &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_SpeakerPosition_Standard {
		::moq::mp4_atom_AudioChannelPosition _0;
	};
	struct mp4_atom_SpeakerPosition_Explicit {
		::moq::mp4_atom_ExplicitSpeakerPosition _0;
	};
	struct mp4_atom_ChannelStructure_ExplicitPositions {
		::std::vector<::moq::mp4_atom_SpeakerPosition> positions;
	};
	struct mp4_atom_ChannelStructure_DefinedLayout {
		::std::uint8_t layout;
		::std::optional<::std::uint64_t> omitted_channels_map;
		::std::optional<::std::uint8_t> channel_order_definition;
	};
	struct mp4_atom_Chnl {
		::std::optional<::moq::mp4_atom_ChannelStructure> channel_structure;
		::std::optional<::std::uint8_t> object_count;
		::std::optional<::std::uint8_t> format_ordering;
		::std::optional<::std::uint8_t> base_channel_count;
		friend bool operator==(const mp4_atom_Chnl &a, const mp4_atom_Chnl &b);
		friend bool operator!=(const mp4_atom_Chnl &a, const mp4_atom_Chnl &b);
		::moq::buf debug() const;
		static ::moq::result<::moq::mp4_atom_Chnl, ::moq::buf> decode_body_with_channel_count(::moq::buf &buf_, ::std::uint16_t channel_count);
	};
	struct mp4_atom_Ipcm {
		::moq::mp4_atom_Audio audio;
		::std::optional<::moq::mp4_atom_PcmC> pcmc;
		::std::optional<::moq::mp4_atom_Chnl> chnl;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_Ipcm &a, const mp4_atom_Ipcm &b);
		friend bool operator!=(const mp4_atom_Ipcm &a, const mp4_atom_Ipcm &b);
		::moq::buf debug() const;
		::std::optional<::moq::mp4_atom_PcmFormat> format() const;
	};
	struct mp4_atom_Fpcm {
		::moq::mp4_atom_Audio audio;
		::std::optional<::moq::mp4_atom_PcmC> pcmc;
		::std::optional<::moq::mp4_atom_Chnl> chnl;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_Fpcm &a, const mp4_atom_Fpcm &b);
		friend bool operator!=(const mp4_atom_Fpcm &a, const mp4_atom_Fpcm &b);
		::moq::buf debug() const;
		::std::optional<::moq::mp4_atom_PcmFormat> format() const;
	};
	struct mp4_atom_Sowt {
		::moq::mp4_atom_Audio audio;
		::std::optional<::moq::mp4_atom_PcmC> pcmc;
		::std::optional<::moq::mp4_atom_Chnl> chnl;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_Sowt &a, const mp4_atom_Sowt &b);
		friend bool operator!=(const mp4_atom_Sowt &a, const mp4_atom_Sowt &b);
		::moq::buf debug() const;
		::std::optional<::moq::mp4_atom_PcmFormat> format() const;
	};
	struct mp4_atom_Twos {
		::moq::mp4_atom_Audio audio;
		::std::optional<::moq::mp4_atom_PcmC> pcmc;
		::std::optional<::moq::mp4_atom_Chnl> chnl;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_Twos &a, const mp4_atom_Twos &b);
		friend bool operator!=(const mp4_atom_Twos &a, const mp4_atom_Twos &b);
		::moq::buf debug() const;
		::std::optional<::moq::mp4_atom_PcmFormat> format() const;
	};
	struct mp4_atom_Lpcm {
		::moq::mp4_atom_Audio audio;
		::std::optional<::moq::mp4_atom_PcmC> pcmc;
		::std::optional<::moq::mp4_atom_Chnl> chnl;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_Lpcm &a, const mp4_atom_Lpcm &b);
		friend bool operator!=(const mp4_atom_Lpcm &a, const mp4_atom_Lpcm &b);
		::moq::buf debug() const;
		::std::optional<::moq::mp4_atom_PcmFormat> format() const;
	};
	struct mp4_atom_In24 {
		::moq::mp4_atom_Audio audio;
		::std::optional<::moq::mp4_atom_PcmC> pcmc;
		::std::optional<::moq::mp4_atom_Chnl> chnl;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_In24 &a, const mp4_atom_In24 &b);
		friend bool operator!=(const mp4_atom_In24 &a, const mp4_atom_In24 &b);
		::moq::buf debug() const;
		::std::optional<::moq::mp4_atom_PcmFormat> format() const;
	};
	struct mp4_atom_In32 {
		::moq::mp4_atom_Audio audio;
		::std::optional<::moq::mp4_atom_PcmC> pcmc;
		::std::optional<::moq::mp4_atom_Chnl> chnl;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_In32 &a, const mp4_atom_In32 &b);
		friend bool operator!=(const mp4_atom_In32 &a, const mp4_atom_In32 &b);
		::moq::buf debug() const;
		::std::optional<::moq::mp4_atom_PcmFormat> format() const;
	};
	struct mp4_atom_Fl32 {
		::moq::mp4_atom_Audio audio;
		::std::optional<::moq::mp4_atom_PcmC> pcmc;
		::std::optional<::moq::mp4_atom_Chnl> chnl;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_Fl32 &a, const mp4_atom_Fl32 &b);
		friend bool operator!=(const mp4_atom_Fl32 &a, const mp4_atom_Fl32 &b);
		::moq::buf debug() const;
		::std::optional<::moq::mp4_atom_PcmFormat> format() const;
	};
	struct mp4_atom_Fl64 {
		::moq::mp4_atom_Audio audio;
		::std::optional<::moq::mp4_atom_PcmC> pcmc;
		::std::optional<::moq::mp4_atom_Chnl> chnl;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_Fl64 &a, const mp4_atom_Fl64 &b);
		friend bool operator!=(const mp4_atom_Fl64 &a, const mp4_atom_Fl64 &b);
		::moq::buf debug() const;
		::std::optional<::moq::mp4_atom_PcmFormat> format() const;
	};
	struct mp4_atom_S16l {
		::moq::mp4_atom_Audio audio;
		::std::optional<::moq::mp4_atom_PcmC> pcmc;
		::std::optional<::moq::mp4_atom_Chnl> chnl;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_S16l &a, const mp4_atom_S16l &b);
		friend bool operator!=(const mp4_atom_S16l &a, const mp4_atom_S16l &b);
		::moq::buf debug() const;
		::std::optional<::moq::mp4_atom_PcmFormat> format() const;
	};
	struct mp4_atom_PlainText {
		::std::uint16_t data_reference_index;
		friend bool operator==(const mp4_atom_PlainText &a, const mp4_atom_PlainText &b);
		friend bool operator!=(const mp4_atom_PlainText &a, const mp4_atom_PlainText &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_VttC {
		::moq::buf config;
		friend bool operator==(const mp4_atom_VttC &a, const mp4_atom_VttC &b);
		friend bool operator!=(const mp4_atom_VttC &a, const mp4_atom_VttC &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Vlab {
		::moq::buf source_label;
		friend bool operator==(const mp4_atom_Vlab &a, const mp4_atom_Vlab &b);
		friend bool operator!=(const mp4_atom_Vlab &a, const mp4_atom_Vlab &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Wvtt {
		::moq::mp4_atom_PlainText plaintext;
		::moq::mp4_atom_VttC config;
		::std::optional<::moq::mp4_atom_Vlab> label;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_Wvtt &a, const mp4_atom_Wvtt &b);
		friend bool operator!=(const mp4_atom_Wvtt &a, const mp4_atom_Wvtt &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_AmrSampleEntry {
		::std::uint16_t data_reference_index;
		::std::uint16_t timescale;
		friend bool operator==(const mp4_atom_AmrSampleEntry &a, const mp4_atom_AmrSampleEntry &b);
		friend bool operator!=(const mp4_atom_AmrSampleEntry &a, const mp4_atom_AmrSampleEntry &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Damr {
		::moq::mp4_atom_FourCC vendor;
		::std::uint8_t decoder_version;
		::std::uint16_t mode_set;
		::std::uint8_t mode_change_period;
		::std::uint8_t frames_per_sample;
		friend bool operator==(const mp4_atom_Damr &a, const mp4_atom_Damr &b);
		friend bool operator!=(const mp4_atom_Damr &a, const mp4_atom_Damr &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Samr {
		::moq::mp4_atom_AmrSampleEntry amrsampleentry;
		::moq::mp4_atom_Damr damr;
		friend bool operator==(const mp4_atom_Samr &a, const mp4_atom_Samr &b);
		friend bool operator!=(const mp4_atom_Samr &a, const mp4_atom_Samr &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_MetaData {
		::std::uint16_t data_reference_index;
		friend bool operator==(const mp4_atom_MetaData &a, const mp4_atom_MetaData &b);
		friend bool operator!=(const mp4_atom_MetaData &a, const mp4_atom_MetaData &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_TxtC {
		::moq::buf text_config;
		friend bool operator==(const mp4_atom_TxtC &a, const mp4_atom_TxtC &b);
		friend bool operator!=(const mp4_atom_TxtC &a, const mp4_atom_TxtC &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Mett {
		::moq::mp4_atom_MetaData metadata;
		::moq::buf content_encoding;
		::moq::buf mime_format;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		::std::optional<::moq::mp4_atom_TxtC> txtc;
		friend bool operator==(const mp4_atom_Mett &a, const mp4_atom_Mett &b);
		friend bool operator!=(const mp4_atom_Mett &a, const mp4_atom_Mett &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Metx {
		::moq::mp4_atom_MetaData metadata;
		::moq::buf content_encoding;
		::moq::buf namespace_;
		::moq::buf schema_location;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_Metx &a, const mp4_atom_Metx &b);
		friend bool operator!=(const mp4_atom_Metx &a, const mp4_atom_Metx &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Uri {
		::moq::buf the_uri;
		friend bool operator==(const mp4_atom_Uri &a, const mp4_atom_Uri &b);
		friend bool operator!=(const mp4_atom_Uri &a, const mp4_atom_Uri &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_UriI {
		::moq::buf uri_initialization_data;
		friend bool operator==(const mp4_atom_UriI &a, const mp4_atom_UriI &b);
		friend bool operator!=(const mp4_atom_UriI &a, const mp4_atom_UriI &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Urim {
		::moq::mp4_atom_MetaData metadata;
		::moq::mp4_atom_Uri the_label;
		::std::optional<::moq::mp4_atom_UriI> init;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_Urim &a, const mp4_atom_Urim &b);
		friend bool operator!=(const mp4_atom_Urim &a, const mp4_atom_Urim &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Camm {
		::moq::mp4_atom_MetaData metadata;
		friend bool operator==(const mp4_atom_Camm &a, const mp4_atom_Camm &b);
		friend bool operator!=(const mp4_atom_Camm &a, const mp4_atom_Camm &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_SchemeIdEntry {
		::moq::buf scheme_id_uri;
		::moq::buf value;
		bool atleast_once;
		friend bool operator==(const mp4_atom_SchemeIdEntry &a, const mp4_atom_SchemeIdEntry &b);
		friend bool operator!=(const mp4_atom_SchemeIdEntry &a, const mp4_atom_SchemeIdEntry &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Silb {
		::std::vector<::moq::mp4_atom_SchemeIdEntry> schemes;
		bool other_schemes;
		friend bool operator==(const mp4_atom_Silb &a, const mp4_atom_Silb &b);
		friend bool operator!=(const mp4_atom_Silb &a, const mp4_atom_Silb &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Evte {
		::moq::mp4_atom_MetaData metadata;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		::std::optional<::moq::mp4_atom_Silb> silb;
		friend bool operator==(const mp4_atom_Evte &a, const mp4_atom_Evte &b);
		friend bool operator!=(const mp4_atom_Evte &a, const mp4_atom_Evte &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Keyd {
		::moq::mp4_atom_FourCC key_namespace;
		::moq::buf key_value;
		friend bool operator==(const mp4_atom_Keyd &a, const mp4_atom_Keyd &b);
		friend bool operator!=(const mp4_atom_Keyd &a, const mp4_atom_Keyd &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_DataType_WellKnown {
		::std::uint32_t _0;
	};
	struct mp4_atom_DataType_Custom {
		::moq::buf _0;
	};
	struct mp4_atom_DataType_Unknown {
		::std::uint32_t _0;
		::moq::buf _1;
	};
	struct mp4_atom_Dtyp {
		::moq::mp4_atom_DataType data_type;
		friend bool operator==(const mp4_atom_Dtyp &a, const mp4_atom_Dtyp &b);
		friend bool operator!=(const mp4_atom_Dtyp &a, const mp4_atom_Dtyp &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Loca {
		::moq::buf locale;
		friend bool operator==(const mp4_atom_Loca &a, const mp4_atom_Loca &b);
		friend bool operator!=(const mp4_atom_Loca &a, const mp4_atom_Loca &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Setu {
		::moq::buf data;
		friend bool operator==(const mp4_atom_Setu &a, const mp4_atom_Setu &b);
		friend bool operator!=(const mp4_atom_Setu &a, const mp4_atom_Setu &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_MebxKey {
		::moq::mp4_atom_FourCC local_key_id;
		::moq::mp4_atom_Keyd keyd;
		::std::optional<::moq::mp4_atom_Dtyp> dtyp;
		::std::optional<::moq::mp4_atom_Loca> loca;
		::std::optional<::moq::mp4_atom_Setu> setu;
		friend bool operator==(const mp4_atom_MebxKey &a, const mp4_atom_MebxKey &b);
		friend bool operator!=(const mp4_atom_MebxKey &a, const mp4_atom_MebxKey &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Mebx {
		::moq::mp4_atom_MetaData metadata;
		::std::vector<::moq::mp4_atom_MebxKey> keys;
		::std::optional<::moq::mp4_atom_Btrt> btrt;
		friend bool operator==(const mp4_atom_Mebx &a, const mp4_atom_Mebx &b);
		friend bool operator!=(const mp4_atom_Mebx &a, const mp4_atom_Mebx &b);
		::moq::buf debug() const;
	};
	class mp4_atom_FixedPoint_u8 {
		friend bool operator==(const mp4_atom_FixedPoint_u8 &a, const mp4_atom_FixedPoint_u8 &b);
		friend bool operator!=(const mp4_atom_FixedPoint_u8 &a, const mp4_atom_FixedPoint_u8 &b);
		::moq::buf debug() const;
		static mp4_atom_FixedPoint_u8 default_();
		static ::moq::mp4_atom_FixedPoint_u8 new_(::std::uint8_t int_, ::std::uint8_t dec);
		::std::uint8_t integer() const;
		::std::uint8_t decimal() const;
	};
	struct mp4_atom_Matrix {
		::std::int32_t a;
		::std::int32_t b;
		::std::int32_t u;
		::std::int32_t c;
		::std::int32_t d;
		::std::int32_t v;
		::std::int32_t x;
		::std::int32_t y;
		::std::int32_t w;
		friend bool operator==(const mp4_atom_Matrix &a, const mp4_atom_Matrix &b);
		friend bool operator!=(const mp4_atom_Matrix &a, const mp4_atom_Matrix &b);
		::moq::buf to_string() const;
		::moq::buf debug() const;
		static mp4_atom_Matrix default_();
	};
	struct mp4_atom_Tkhd {
		::std::uint64_t creation_time;
		::std::uint64_t modification_time;
		::std::uint32_t track_id;
		::std::uint64_t duration;
		::std::uint16_t layer;
		::std::uint16_t alternate_group;
		bool enabled;
		bool in_movie;
		bool size_is_aspect_ratio;
		::moq::mp4_atom_FixedPoint_u8 volume;
		::moq::mp4_atom_Matrix matrix;
		::moq::mp4_atom_FixedPoint_u16 width;
		::moq::mp4_atom_FixedPoint_u16 height;
		friend bool operator==(const mp4_atom_Tkhd &a, const mp4_atom_Tkhd &b);
		friend bool operator!=(const mp4_atom_Tkhd &a, const mp4_atom_Tkhd &b);
		::moq::buf debug() const;
		static mp4_atom_Tkhd default_();
	};
	class mp4_atom_FixedPoint_i16 {
		friend bool operator==(const mp4_atom_FixedPoint_i16 &a, const mp4_atom_FixedPoint_i16 &b);
		friend bool operator!=(const mp4_atom_FixedPoint_i16 &a, const mp4_atom_FixedPoint_i16 &b);
		::moq::buf debug() const;
		static mp4_atom_FixedPoint_i16 default_();
		static ::moq::mp4_atom_FixedPoint_i16 new_(::std::int16_t int_, ::std::int16_t dec);
		::std::int16_t integer() const;
		::std::int16_t decimal() const;
	};
	struct mp4_atom_ElstEntry {
		::std::uint64_t segment_duration;
		::std::optional<::std::uint64_t> media_time;
		::moq::mp4_atom_FixedPoint_i16 media_rate;
		friend bool operator==(const mp4_atom_ElstEntry &a, const mp4_atom_ElstEntry &b);
		friend bool operator!=(const mp4_atom_ElstEntry &a, const mp4_atom_ElstEntry &b);
		::moq::buf debug() const;
		static mp4_atom_ElstEntry default_();
	};
	struct mp4_atom_Elst {
		::std::vector<::moq::mp4_atom_ElstEntry> entries;
		friend bool operator==(const mp4_atom_Elst &a, const mp4_atom_Elst &b);
		friend bool operator!=(const mp4_atom_Elst &a, const mp4_atom_Elst &b);
		::moq::buf debug() const;
		static mp4_atom_Elst default_();
	};
	struct mp4_atom_Edts {
		::std::optional<::moq::mp4_atom_Elst> elst;
		friend bool operator==(const mp4_atom_Edts &a, const mp4_atom_Edts &b);
		friend bool operator!=(const mp4_atom_Edts &a, const mp4_atom_Edts &b);
		::moq::buf debug() const;
		static mp4_atom_Edts default_();
	};
	struct mp4_atom_Senc {
		::moq::mp4_atom_SencBoxVersion version;
		bool use_subsamples;
		::moq::buf data;
		friend bool operator==(const mp4_atom_Senc &a, const mp4_atom_Senc &b);
		friend bool operator!=(const mp4_atom_Senc &a, const mp4_atom_Senc &b);
		::moq::buf debug() const;
		static mp4_atom_Senc default_();
	};
	struct mp4_atom_TrackReferenceTypeBox {
		::moq::mp4_atom_FourCC reference_type;
		::std::vector<::std::uint32_t> track_ids;
		friend bool operator==(const mp4_atom_TrackReferenceTypeBox &a, const mp4_atom_TrackReferenceTypeBox &b);
		friend bool operator!=(const mp4_atom_TrackReferenceTypeBox &a, const mp4_atom_TrackReferenceTypeBox &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Tref {
		::std::vector<::moq::mp4_atom_TrackReferenceTypeBox> track_reference_type_boxes;
		friend bool operator==(const mp4_atom_Tref &a, const mp4_atom_Tref &b);
		friend bool operator!=(const mp4_atom_Tref &a, const mp4_atom_Tref &b);
		::moq::buf debug() const;
		static mp4_atom_Tref default_();
	};
	struct mp4_atom_AvccExt {
		::std::uint8_t chroma_format;
		::std::uint8_t bit_depth_luma;
		::std::uint8_t bit_depth_chroma;
		::std::vector<::moq::buf> sequence_parameter_sets_ext;
		friend bool operator==(const mp4_atom_AvccExt &a, const mp4_atom_AvccExt &b);
		friend bool operator!=(const mp4_atom_AvccExt &a, const mp4_atom_AvccExt &b);
		::moq::buf debug() const;
		static mp4_atom_AvccExt default_();
	};
	struct mp4_atom_Avcc {
		::std::uint8_t configuration_version;
		::std::uint8_t avc_profile_indication;
		::std::uint8_t profile_compatibility;
		::std::uint8_t avc_level_indication;
		::std::uint8_t length_size;
		::std::vector<::moq::buf> sequence_parameter_sets;
		::std::vector<::moq::buf> picture_parameter_sets;
		::std::optional<::moq::mp4_atom_AvccExt> ext;
		friend bool operator==(const mp4_atom_Avcc &a, const mp4_atom_Avcc &b);
		friend bool operator!=(const mp4_atom_Avcc &a, const mp4_atom_Avcc &b);
		::moq::buf debug() const;
		static mp4_atom_Avcc default_();
		static ::moq::result<::moq::mp4_atom_Avcc, ::moq::buf> new_(::moq::input sps, ::moq::input pps);
	};
	struct mp4_atom_Pasp {
		::std::uint32_t h_spacing;
		::std::uint32_t v_spacing;
		friend bool operator==(const mp4_atom_Pasp &a, const mp4_atom_Pasp &b);
		friend bool operator!=(const mp4_atom_Pasp &a, const mp4_atom_Pasp &b);
		::moq::buf debug() const;
		static mp4_atom_Pasp default_();
		static ::moq::result<::moq::mp4_atom_Pasp, ::moq::buf> new_(::std::uint32_t h_spacing_, ::std::uint32_t v_spacing_);
	};
	struct mp4_atom_Fiel {
		::std::uint8_t field_count;
		::std::uint8_t field_order;
		friend bool operator==(const mp4_atom_Fiel &a, const mp4_atom_Fiel &b);
		friend bool operator!=(const mp4_atom_Fiel &a, const mp4_atom_Fiel &b);
		::moq::buf debug() const;
		static mp4_atom_Fiel default_();
		static ::moq::result<::moq::mp4_atom_Fiel, ::moq::buf> new_(::std::uint8_t field_count_, ::std::uint8_t field_order_);
	};
	struct mp4_atom_HvcCArray {
		bool completeness;
		::std::uint8_t nal_unit_type;
		::std::vector<::moq::buf> nalus;
		friend bool operator==(const mp4_atom_HvcCArray &a, const mp4_atom_HvcCArray &b);
		friend bool operator!=(const mp4_atom_HvcCArray &a, const mp4_atom_HvcCArray &b);
		::moq::buf debug() const;
		static mp4_atom_HvcCArray default_();
	};
	struct mp4_atom_Hvcc {
		::std::uint8_t configuration_version;
		::std::uint8_t general_profile_space;
		bool general_tier_flag;
		::std::uint8_t general_profile_idc;
		::std::array<::std::uint8_t, 4> general_profile_compatibility_flags;
		::std::array<::std::uint8_t, 6> general_constraint_indicator_flags;
		::std::uint8_t general_level_idc;
		::std::uint16_t min_spatial_segmentation_idc;
		::std::uint8_t parallelism_type;
		::std::uint8_t chroma_format_idc;
		::std::uint8_t bit_depth_luma_minus8;
		::std::uint8_t bit_depth_chroma_minus8;
		::std::uint16_t avg_frame_rate;
		::std::uint8_t constant_frame_rate;
		::std::uint8_t num_temporal_layers;
		bool temporal_id_nested;
		::std::uint8_t length_size_minus_one;
		::std::vector<::moq::mp4_atom_HvcCArray> arrays;
		friend bool operator==(const mp4_atom_Hvcc &a, const mp4_atom_Hvcc &b);
		friend bool operator!=(const mp4_atom_Hvcc &a, const mp4_atom_Hvcc &b);
		::moq::buf debug() const;
		static mp4_atom_Hvcc default_();
		static ::moq::mp4_atom_Hvcc new_();
	};
	struct mp4_atom_Lhvc {
		::std::uint8_t configuration_version;
		::std::uint16_t min_spatial_segmentation_idc;
		::std::uint8_t parallelism_type;
		::std::uint8_t num_temporal_layers;
		bool temporal_id_nested;
		::std::uint8_t length_size_minus_one;
		::std::vector<::moq::mp4_atom_HvcCArray> arrays;
		friend bool operator==(const mp4_atom_Lhvc &a, const mp4_atom_Lhvc &b);
		friend bool operator!=(const mp4_atom_Lhvc &a, const mp4_atom_Lhvc &b);
		::moq::buf debug() const;
		static mp4_atom_Lhvc default_();
	};
	struct mp4_atom_Ccst {
		bool all_ref_pics_intra;
		bool intra_pred_used;
		::std::uint8_t max_ref_per_pic;
		friend bool operator==(const mp4_atom_Ccst &a, const mp4_atom_Ccst &b);
		friend bool operator!=(const mp4_atom_Ccst &a, const mp4_atom_Ccst &b);
		::moq::buf debug() const;
		static mp4_atom_Ccst default_();
	};
	struct mp4_atom_VpcC {
		::std::uint8_t profile;
		::std::uint8_t level;
		::std::uint8_t bit_depth;
		::std::uint8_t chroma_subsampling;
		bool video_full_range_flag;
		::std::uint8_t color_primaries;
		::std::uint8_t transfer_characteristics;
		::std::uint8_t matrix_coefficients;
		::moq::buf codec_initialization_data;
		friend bool operator==(const mp4_atom_VpcC &a, const mp4_atom_VpcC &b);
		friend bool operator!=(const mp4_atom_VpcC &a, const mp4_atom_VpcC &b);
		::moq::buf debug() const;
		static mp4_atom_VpcC default_();
	};
	struct mp4_atom_Av1c {
		::std::uint8_t seq_profile;
		::std::uint8_t seq_level_idx_0;
		bool seq_tier_0;
		bool high_bitdepth;
		bool twelve_bit;
		bool monochrome;
		bool chroma_subsampling_x;
		bool chroma_subsampling_y;
		::std::uint8_t chroma_sample_position;
		::std::optional<::std::uint8_t> initial_presentation_delay;
		::moq::buf config_obus;
		friend bool operator==(const mp4_atom_Av1c &a, const mp4_atom_Av1c &b);
		friend bool operator!=(const mp4_atom_Av1c &a, const mp4_atom_Av1c &b);
		::moq::buf debug() const;
		static mp4_atom_Av1c default_();
	};
	struct mp4_atom_RgbaColor {
		::std::uint8_t red;
		::std::uint8_t green;
		::std::uint8_t blue;
		::std::uint8_t alpha;
		friend bool operator==(const mp4_atom_RgbaColor &a, const mp4_atom_RgbaColor &b);
		friend bool operator!=(const mp4_atom_RgbaColor &a, const mp4_atom_RgbaColor &b);
		::moq::buf debug() const;
		static mp4_atom_RgbaColor default_();
	};
	struct mp4_atom_FontEntry {
		::std::uint16_t font_id;
		::moq::buf font;
		friend bool operator==(const mp4_atom_FontEntry &a, const mp4_atom_FontEntry &b);
		friend bool operator!=(const mp4_atom_FontEntry &a, const mp4_atom_FontEntry &b);
		::moq::buf debug() const;
		static mp4_atom_FontEntry default_();
	};
	struct mp4_atom_Ftab {
		::std::vector<::moq::mp4_atom_FontEntry> font_entries;
		friend bool operator==(const mp4_atom_Ftab &a, const mp4_atom_Ftab &b);
		friend bool operator!=(const mp4_atom_Ftab &a, const mp4_atom_Ftab &b);
		::moq::buf debug() const;
		static mp4_atom_Ftab default_();
	};
	struct mp4_atom_Disp {
		::std::int16_t disparity_shift_in_16th_pel;
		friend bool operator==(const mp4_atom_Disp &a, const mp4_atom_Disp &b);
		friend bool operator!=(const mp4_atom_Disp &a, const mp4_atom_Disp &b);
		::moq::buf debug() const;
		static mp4_atom_Disp default_();
	};
	struct mp4_atom_Component {
		::std::uint16_t component_type;
		::std::optional<::moq::buf> component_type_uri;
		friend bool operator==(const mp4_atom_Component &a, const mp4_atom_Component &b);
		friend bool operator!=(const mp4_atom_Component &a, const mp4_atom_Component &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Cmpd {
		::std::vector<::moq::mp4_atom_Component> components;
		friend bool operator==(const mp4_atom_Cmpd &a, const mp4_atom_Cmpd &b);
		friend bool operator!=(const mp4_atom_Cmpd &a, const mp4_atom_Cmpd &b);
		::moq::buf debug() const;
		static mp4_atom_Cmpd default_();
	};
	class mp4_atom_FixedPoint_i8 {
		friend bool operator==(const mp4_atom_FixedPoint_i8 &a, const mp4_atom_FixedPoint_i8 &b);
		friend bool operator!=(const mp4_atom_FixedPoint_i8 &a, const mp4_atom_FixedPoint_i8 &b);
		::moq::buf debug() const;
		static mp4_atom_FixedPoint_i8 default_();
		static ::moq::mp4_atom_FixedPoint_i8 new_(::std::int8_t int_, ::std::int8_t dec);
		::std::int8_t integer() const;
		::std::int8_t decimal() const;
	};
	struct mp4_atom_Hdlr {
		::moq::mp4_atom_FourCC handler;
		::moq::buf name;
		friend bool operator==(const mp4_atom_Hdlr &a, const mp4_atom_Hdlr &b);
		friend bool operator!=(const mp4_atom_Hdlr &a, const mp4_atom_Hdlr &b);
		::moq::buf debug() const;
		static mp4_atom_Hdlr default_();
	};
	struct mp4_atom_Mdhd {
		::std::uint64_t creation_time;
		::std::uint64_t modification_time;
		::std::uint32_t timescale;
		::std::uint64_t duration;
		::moq::buf language;
		friend bool operator==(const mp4_atom_Mdhd &a, const mp4_atom_Mdhd &b);
		friend bool operator!=(const mp4_atom_Mdhd &a, const mp4_atom_Mdhd &b);
		::moq::buf debug() const;
		static mp4_atom_Mdhd default_();
	};
	struct mp4_atom_ChplEntry {
		::std::uint64_t start;
		::moq::buf title;
		friend bool operator==(const mp4_atom_ChplEntry &a, const mp4_atom_ChplEntry &b);
		friend bool operator!=(const mp4_atom_ChplEntry &a, const mp4_atom_ChplEntry &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Chpl {
		::std::vector<::moq::mp4_atom_ChplEntry> entries;
		friend bool operator==(const mp4_atom_Chpl &a, const mp4_atom_Chpl &b);
		friend bool operator!=(const mp4_atom_Chpl &a, const mp4_atom_Chpl &b);
		::moq::buf debug() const;
		static mp4_atom_Chpl default_();
	};
	struct mp4_atom_Cprt {
		::moq::buf language;
		::moq::buf notice;
		friend bool operator==(const mp4_atom_Cprt &a, const mp4_atom_Cprt &b);
		friend bool operator!=(const mp4_atom_Cprt &a, const mp4_atom_Cprt &b);
		::moq::buf debug() const;
		static mp4_atom_Cprt default_();
	};
	struct mp4_atom_Kind {
		::moq::buf scheme_uri;
		::moq::buf value;
		friend bool operator==(const mp4_atom_Kind &a, const mp4_atom_Kind &b);
		friend bool operator!=(const mp4_atom_Kind &a, const mp4_atom_Kind &b);
		::moq::buf debug() const;
		static mp4_atom_Kind default_();
	};
	struct mp4_atom_Rtng {
		::moq::mp4_atom_FourCC entity;
		::moq::mp4_atom_FourCC criteria;
		::moq::buf language;
		::moq::buf rating_info;
		friend bool operator==(const mp4_atom_Rtng &a, const mp4_atom_Rtng &b);
		friend bool operator!=(const mp4_atom_Rtng &a, const mp4_atom_Rtng &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_PcmFormat {
		bool big_endian;
		::std::uint16_t sample_size;
		bool float_;
		friend bool operator==(const mp4_atom_PcmFormat &a, const mp4_atom_PcmFormat &b);
		friend bool operator!=(const mp4_atom_PcmFormat &a, const mp4_atom_PcmFormat &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_St3d {
		::std::uint8_t stereo_mode;
		friend bool operator==(const mp4_atom_St3d &a, const mp4_atom_St3d &b);
		friend bool operator!=(const mp4_atom_St3d &a, const mp4_atom_St3d &b);
		::moq::buf debug() const;
		static mp4_atom_St3d default_();
	};
	struct mp4_atom_Hfov {
		::std::uint32_t field_of_view;
		friend bool operator==(const mp4_atom_Hfov &a, const mp4_atom_Hfov &b);
		friend bool operator!=(const mp4_atom_Hfov &a, const mp4_atom_Hfov &b);
		::moq::buf debug() const;
		static mp4_atom_Hfov default_();
	};
	struct mp4_atom_UncompressedComponent {
		::std::uint16_t component_index;
		::std::uint8_t component_bit_depth_minus_one;
		::std::uint8_t component_format;
		::std::uint8_t component_align_size;
		friend bool operator==(const mp4_atom_UncompressedComponent &a, const mp4_atom_UncompressedComponent &b);
		friend bool operator!=(const mp4_atom_UncompressedComponent &a, const mp4_atom_UncompressedComponent &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Ftyp {
		::moq::mp4_atom_FourCC major_brand;
		::std::uint32_t minor_version;
		::std::vector<::moq::mp4_atom_FourCC> compatible_brands;
		friend bool operator==(const mp4_atom_Ftyp &a, const mp4_atom_Ftyp &b);
		friend bool operator!=(const mp4_atom_Ftyp &a, const mp4_atom_Ftyp &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Styp {
		::moq::mp4_atom_FourCC major_brand;
		::std::uint32_t minor_version;
		::std::vector<::moq::mp4_atom_FourCC> compatible_brands;
		friend bool operator==(const mp4_atom_Styp &a, const mp4_atom_Styp &b);
		friend bool operator!=(const mp4_atom_Styp &a, const mp4_atom_Styp &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Pitm {
		::std::uint32_t item_id;
		friend bool operator==(const mp4_atom_Pitm &a, const mp4_atom_Pitm &b);
		friend bool operator!=(const mp4_atom_Pitm &a, const mp4_atom_Pitm &b);
		::moq::buf debug() const;
		static mp4_atom_Pitm default_();
	};
	struct mp4_atom_ItemLocationExtent {
		::std::uint64_t item_reference_index;
		::std::uint64_t offset;
		::std::uint64_t length;
		friend bool operator==(const mp4_atom_ItemLocationExtent &a, const mp4_atom_ItemLocationExtent &b);
		friend bool operator!=(const mp4_atom_ItemLocationExtent &a, const mp4_atom_ItemLocationExtent &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_ItemLocation {
		::std::uint32_t item_id;
		::std::uint8_t construction_method;
		::std::uint16_t data_reference_index;
		::std::uint64_t base_offset;
		::std::vector<::moq::mp4_atom_ItemLocationExtent> extents;
		friend bool operator==(const mp4_atom_ItemLocation &a, const mp4_atom_ItemLocation &b);
		friend bool operator!=(const mp4_atom_ItemLocation &a, const mp4_atom_ItemLocation &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Iloc {
		::std::vector<::moq::mp4_atom_ItemLocation> item_locations;
		friend bool operator==(const mp4_atom_Iloc &a, const mp4_atom_Iloc &b);
		friend bool operator!=(const mp4_atom_Iloc &a, const mp4_atom_Iloc &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_ItemInfoEntry {
		::std::uint32_t item_id;
		::std::uint16_t item_protection_index;
		::std::optional<::moq::mp4_atom_FourCC> item_type;
		::moq::buf item_name;
		::std::optional<::moq::buf> content_type;
		::std::optional<::moq::buf> content_encoding;
		::std::optional<::moq::buf> item_uri_type;
		bool item_not_in_presentation;
		friend bool operator==(const mp4_atom_ItemInfoEntry &a, const mp4_atom_ItemInfoEntry &b);
		friend bool operator!=(const mp4_atom_ItemInfoEntry &a, const mp4_atom_ItemInfoEntry &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Iinf {
		::std::vector<::moq::mp4_atom_ItemInfoEntry> item_infos;
		friend bool operator==(const mp4_atom_Iinf &a, const mp4_atom_Iinf &b);
		friend bool operator!=(const mp4_atom_Iinf &a, const mp4_atom_Iinf &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Auxc {
		::moq::buf aux_type;
		::moq::buf aux_subtype;
		friend bool operator==(const mp4_atom_Auxc &a, const mp4_atom_Auxc &b);
		friend bool operator!=(const mp4_atom_Auxc &a, const mp4_atom_Auxc &b);
		::moq::buf debug() const;
		static mp4_atom_Auxc default_();
	};
	struct mp4_atom_Clap {
		::std::uint32_t clean_aperture_width_n;
		::std::uint32_t clean_aperture_width_d;
		::std::uint32_t clean_aperture_height_n;
		::std::uint32_t clean_aperture_height_d;
		::std::int32_t horiz_off_n;
		::std::uint32_t horiz_off_d;
		::std::int32_t vert_off_n;
		::std::uint32_t vert_off_d;
		friend bool operator==(const mp4_atom_Clap &a, const mp4_atom_Clap &b);
		friend bool operator!=(const mp4_atom_Clap &a, const mp4_atom_Clap &b);
		::moq::buf debug() const;
		static mp4_atom_Clap default_();
	};
	struct mp4_atom_Imir {
		::std::uint8_t axis;
		friend bool operator==(const mp4_atom_Imir &a, const mp4_atom_Imir &b);
		friend bool operator!=(const mp4_atom_Imir &a, const mp4_atom_Imir &b);
		::moq::buf debug() const;
		static mp4_atom_Imir default_();
	};
	struct mp4_atom_Irot {
		::std::uint8_t angle;
		friend bool operator==(const mp4_atom_Irot &a, const mp4_atom_Irot &b);
		friend bool operator!=(const mp4_atom_Irot &a, const mp4_atom_Irot &b);
		::moq::buf debug() const;
		static mp4_atom_Irot default_();
	};
	struct mp4_atom_Iscl {
		::std::uint16_t target_width_numerator;
		::std::uint16_t target_width_denominator;
		::std::uint16_t target_height_numerator;
		::std::uint16_t target_height_denominator;
		friend bool operator==(const mp4_atom_Iscl &a, const mp4_atom_Iscl &b);
		friend bool operator!=(const mp4_atom_Iscl &a, const mp4_atom_Iscl &b);
		::moq::buf debug() const;
		static mp4_atom_Iscl default_();
	};
	struct mp4_atom_Ispe {
		::std::uint32_t width;
		::std::uint32_t height;
		friend bool operator==(const mp4_atom_Ispe &a, const mp4_atom_Ispe &b);
		friend bool operator!=(const mp4_atom_Ispe &a, const mp4_atom_Ispe &b);
		::moq::buf debug() const;
		static mp4_atom_Ispe default_();
	};
	struct mp4_atom_Pixi {
		::moq::buf bits_per_channel;
		friend bool operator==(const mp4_atom_Pixi &a, const mp4_atom_Pixi &b);
		friend bool operator!=(const mp4_atom_Pixi &a, const mp4_atom_Pixi &b);
		::moq::buf debug() const;
		static mp4_atom_Pixi default_();
	};
	struct mp4_atom_Rref {
		::std::vector<::moq::mp4_atom_FourCC> reference_types;
		friend bool operator==(const mp4_atom_Rref &a, const mp4_atom_Rref &b);
		friend bool operator!=(const mp4_atom_Rref &a, const mp4_atom_Rref &b);
		::moq::buf debug() const;
		static mp4_atom_Rref default_();
	};
	struct mp4_atom_PropertyAssociation {
		bool essential;
		::std::uint16_t property_index;
		friend bool operator==(const mp4_atom_PropertyAssociation &a, const mp4_atom_PropertyAssociation &b);
		friend bool operator!=(const mp4_atom_PropertyAssociation &a, const mp4_atom_PropertyAssociation &b);
		::moq::buf debug() const;
		static mp4_atom_PropertyAssociation default_();
	};
	struct mp4_atom_PropertyAssociations {
		::std::uint32_t item_id;
		::std::vector<::moq::mp4_atom_PropertyAssociation> associations;
		friend bool operator==(const mp4_atom_PropertyAssociations &a, const mp4_atom_PropertyAssociations &b);
		friend bool operator!=(const mp4_atom_PropertyAssociations &a, const mp4_atom_PropertyAssociations &b);
		::moq::buf debug() const;
		static mp4_atom_PropertyAssociations default_();
	};
	struct mp4_atom_Ipma {
		::std::vector<::moq::mp4_atom_PropertyAssociations> item_properties;
		friend bool operator==(const mp4_atom_Ipma &a, const mp4_atom_Ipma &b);
		friend bool operator!=(const mp4_atom_Ipma &a, const mp4_atom_Ipma &b);
		::moq::buf debug() const;
		static mp4_atom_Ipma default_();
	};
	struct mp4_atom_Reference {
		::moq::mp4_atom_FourCC reference_type;
		::std::uint32_t from_item_id;
		::std::vector<::std::uint32_t> to_item_ids;
		friend bool operator==(const mp4_atom_Reference &a, const mp4_atom_Reference &b);
		friend bool operator!=(const mp4_atom_Reference &a, const mp4_atom_Reference &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Iref {
		::std::vector<::moq::mp4_atom_Reference> references;
		friend bool operator==(const mp4_atom_Iref &a, const mp4_atom_Iref &b);
		friend bool operator!=(const mp4_atom_Iref &a, const mp4_atom_Iref &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Idat {
		::moq::buf data;
		friend bool operator==(const mp4_atom_Idat &a, const mp4_atom_Idat &b);
		friend bool operator!=(const mp4_atom_Idat &a, const mp4_atom_Idat &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Name {
		::moq::buf _0;
		friend bool operator==(const mp4_atom_Name &a, const mp4_atom_Name &b);
		friend bool operator!=(const mp4_atom_Name &a, const mp4_atom_Name &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Year {
		::moq::buf _0;
		friend bool operator==(const mp4_atom_Year &a, const mp4_atom_Year &b);
		friend bool operator!=(const mp4_atom_Year &a, const mp4_atom_Year &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Covr {
		::moq::buf _0;
		friend bool operator==(const mp4_atom_Covr &a, const mp4_atom_Covr &b);
		friend bool operator!=(const mp4_atom_Covr &a, const mp4_atom_Covr &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Desc {
		::std::uint16_t country_indicator;
		::std::uint16_t language_indicator;
		::moq::buf text;
		friend bool operator==(const mp4_atom_Desc &a, const mp4_atom_Desc &b);
		friend bool operator!=(const mp4_atom_Desc &a, const mp4_atom_Desc &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Tool {
		::std::uint16_t country_indicator;
		::std::uint16_t language_indicator;
		::moq::buf text;
		friend bool operator==(const mp4_atom_Tool &a, const mp4_atom_Tool &b);
		friend bool operator!=(const mp4_atom_Tool &a, const mp4_atom_Tool &b);
		::moq::buf debug() const;
		static ::moq::result<::moq::mp4_atom_Tool, ::moq::buf> new_(::std::uint16_t country_indicator_, ::std::uint16_t language_indicator_, ::moq::input text_);
	};
	struct mp4_atom_Copyright {
		::std::uint16_t country_indicator;
		::std::uint16_t language_indicator;
		::moq::buf text;
		friend bool operator==(const mp4_atom_Copyright &a, const mp4_atom_Copyright &b);
		friend bool operator!=(const mp4_atom_Copyright &a, const mp4_atom_Copyright &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_IlstDataValue_Reserved {
		::moq::buf _0;
	};
	struct mp4_atom_IlstDataValue_Utf8 {
		::moq::buf _0;
	};
	struct mp4_atom_IlstDataValue_Utf16 {
		::moq::buf _0;
	};
	struct mp4_atom_IlstDataValue_Jpeg {
		::moq::buf _0;
	};
	struct mp4_atom_IlstDataValue_Png {
		::moq::buf _0;
	};
	struct mp4_atom_IlstDataValue_Bmp {
		::moq::buf _0;
	};
	struct mp4_atom_IlstDataValue_BeSignedInt {
		::std::int64_t _0;
	};
	struct mp4_atom_IlstDataValue_BeUnsignedInt {
		::std::uint64_t _0;
	};
	struct mp4_atom_IlstDataValue_BeFloat32 {
		float _0;
	};
	struct mp4_atom_IlstDataValue_BeFloat64 {
		double _0;
	};
	struct mp4_atom_IlstDataValue_Unknown {
		::std::uint32_t _0;
		::moq::buf _1;
	};
	struct mp4_atom_IlstData {
		::std::uint16_t country_indicator;
		::std::uint16_t language_indicator;
		::moq::mp4_atom_IlstDataValue value;
		friend bool operator==(const mp4_atom_IlstData &a, const mp4_atom_IlstData &b);
		friend bool operator!=(const mp4_atom_IlstData &a, const mp4_atom_IlstData &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Ilst {
		::std::optional<::moq::mp4_atom_Name> name;
		::std::optional<::moq::mp4_atom_Year> year;
		::std::optional<::moq::mp4_atom_Covr> covr;
		::std::optional<::moq::mp4_atom_Desc> desc;
		::std::optional<::moq::mp4_atom_Tool> ctoo;
		::std::optional<::moq::mp4_atom_Copyright> cprt;
		::std::vector<::std::tuple<::std::uint32_t, ::moq::mp4_atom_IlstData>> mdta;
		friend bool operator==(const mp4_atom_Ilst &a, const mp4_atom_Ilst &b);
		friend bool operator!=(const mp4_atom_Ilst &a, const mp4_atom_Ilst &b);
		::moq::buf debug() const;
		static mp4_atom_Ilst default_();
	};
	struct mp4_atom_KeyEntry {
		::moq::mp4_atom_FourCC key_namespace;
		::moq::buf key_value;
		friend bool operator==(const mp4_atom_KeyEntry &a, const mp4_atom_KeyEntry &b);
		friend bool operator!=(const mp4_atom_KeyEntry &a, const mp4_atom_KeyEntry &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Keys {
		::std::vector<::moq::mp4_atom_KeyEntry> entries;
		friend bool operator==(const mp4_atom_Keys &a, const mp4_atom_Keys &b);
		friend bool operator!=(const mp4_atom_Keys &a, const mp4_atom_Keys &b);
		::moq::buf debug() const;
		static mp4_atom_Keys default_();
	};
	struct mp4_atom_Mvhd {
		::std::uint64_t creation_time;
		::std::uint64_t modification_time;
		::std::uint32_t timescale;
		::std::uint64_t duration;
		::moq::mp4_atom_FixedPoint_u16 rate;
		::moq::mp4_atom_FixedPoint_u8 volume;
		::moq::mp4_atom_Matrix matrix;
		::std::uint32_t next_track_id;
		friend bool operator==(const mp4_atom_Mvhd &a, const mp4_atom_Mvhd &b);
		friend bool operator!=(const mp4_atom_Mvhd &a, const mp4_atom_Mvhd &b);
		::moq::buf debug() const;
		static mp4_atom_Mvhd default_();
	};
	struct mp4_atom_Ainf {
		bool hidden;
		::std::uint32_t profile_version;
		::moq::buf apid;
		friend bool operator==(const mp4_atom_Ainf &a, const mp4_atom_Ainf &b);
		friend bool operator!=(const mp4_atom_Ainf &a, const mp4_atom_Ainf &b);
		::moq::buf debug() const;
		static mp4_atom_Ainf default_();
	};
	struct mp4_atom_Zeroed {
		::std::size_t size;
		friend bool operator==(const mp4_atom_Zeroed &a, const mp4_atom_Zeroed &b);
		friend bool operator!=(const mp4_atom_Zeroed &a, const mp4_atom_Zeroed &b);
		::moq::buf debug() const;
		static mp4_atom_Zeroed default_();
		static ::moq::mp4_atom_Zeroed new_(::std::size_t size_);
	};
	struct mp4_atom_Skip {
		::moq::mp4_atom_Zeroed zeroed;
		friend bool operator==(const mp4_atom_Skip &a, const mp4_atom_Skip &b);
		friend bool operator!=(const mp4_atom_Skip &a, const mp4_atom_Skip &b);
		::moq::buf debug() const;
		static mp4_atom_Skip default_();
	};
	struct mp4_atom_SttsEntry {
		::std::uint32_t sample_count;
		::std::uint32_t sample_delta;
		friend bool operator==(const mp4_atom_SttsEntry &a, const mp4_atom_SttsEntry &b);
		friend bool operator!=(const mp4_atom_SttsEntry &a, const mp4_atom_SttsEntry &b);
		::moq::buf debug() const;
		static mp4_atom_SttsEntry default_();
	};
	struct mp4_atom_Stts {
		::std::vector<::moq::mp4_atom_SttsEntry> entries;
		friend bool operator==(const mp4_atom_Stts &a, const mp4_atom_Stts &b);
		friend bool operator!=(const mp4_atom_Stts &a, const mp4_atom_Stts &b);
		::moq::buf debug() const;
		static mp4_atom_Stts default_();
	};
	struct mp4_atom_StscEntry {
		::std::uint32_t first_chunk;
		::std::uint32_t samples_per_chunk;
		::std::uint32_t sample_description_index;
		friend bool operator==(const mp4_atom_StscEntry &a, const mp4_atom_StscEntry &b);
		friend bool operator!=(const mp4_atom_StscEntry &a, const mp4_atom_StscEntry &b);
		::moq::buf debug() const;
		static mp4_atom_StscEntry default_();
	};
	struct mp4_atom_Stsc {
		::std::vector<::moq::mp4_atom_StscEntry> entries;
		friend bool operator==(const mp4_atom_Stsc &a, const mp4_atom_Stsc &b);
		friend bool operator!=(const mp4_atom_Stsc &a, const mp4_atom_Stsc &b);
		::moq::buf debug() const;
		static mp4_atom_Stsc default_();
	};
	struct mp4_atom_StszSamples_Identical {
		::std::uint32_t count;
		::std::uint32_t size;
	};
	struct mp4_atom_StszSamples_Different {
		::std::vector<::std::uint32_t> sizes;
	};
	struct mp4_atom_Stsz {
		::moq::mp4_atom_StszSamples samples;
		friend bool operator==(const mp4_atom_Stsz &a, const mp4_atom_Stsz &b);
		friend bool operator!=(const mp4_atom_Stsz &a, const mp4_atom_Stsz &b);
		::moq::buf debug() const;
		static mp4_atom_Stsz default_();
	};
	struct mp4_atom_Stss {
		::std::vector<::std::uint32_t> entries;
		friend bool operator==(const mp4_atom_Stss &a, const mp4_atom_Stss &b);
		friend bool operator!=(const mp4_atom_Stss &a, const mp4_atom_Stss &b);
		::moq::buf debug() const;
		static mp4_atom_Stss default_();
	};
	struct mp4_atom_Stco {
		::std::vector<::std::uint32_t> entries;
		friend bool operator==(const mp4_atom_Stco &a, const mp4_atom_Stco &b);
		friend bool operator!=(const mp4_atom_Stco &a, const mp4_atom_Stco &b);
		::moq::buf debug() const;
		static mp4_atom_Stco default_();
	};
	struct mp4_atom_Co64 {
		::std::vector<::std::uint64_t> entries;
		friend bool operator==(const mp4_atom_Co64 &a, const mp4_atom_Co64 &b);
		friend bool operator!=(const mp4_atom_Co64 &a, const mp4_atom_Co64 &b);
		::moq::buf debug() const;
		static mp4_atom_Co64 default_();
	};
	struct mp4_atom_Cslg {
		::std::int64_t composition_to_dts_shift;
		::std::int64_t least_decode_to_display_shift;
		::std::int64_t greatest_decode_to_display_delta;
		::std::int64_t composition_start_time;
		::std::int64_t composition_end_time;
		friend bool operator==(const mp4_atom_Cslg &a, const mp4_atom_Cslg &b);
		friend bool operator!=(const mp4_atom_Cslg &a, const mp4_atom_Cslg &b);
		::moq::buf debug() const;
		static mp4_atom_Cslg default_();
	};
	struct mp4_atom_CttsEntry {
		::std::uint32_t sample_count;
		::std::int64_t sample_offset;
		friend bool operator==(const mp4_atom_CttsEntry &a, const mp4_atom_CttsEntry &b);
		friend bool operator!=(const mp4_atom_CttsEntry &a, const mp4_atom_CttsEntry &b);
		::moq::buf debug() const;
		static mp4_atom_CttsEntry default_();
	};
	struct mp4_atom_Ctts {
		::std::vector<::moq::mp4_atom_CttsEntry> entries;
		friend bool operator==(const mp4_atom_Ctts &a, const mp4_atom_Ctts &b);
		friend bool operator!=(const mp4_atom_Ctts &a, const mp4_atom_Ctts &b);
		::moq::buf debug() const;
		static mp4_atom_Ctts default_();
	};
	struct mp4_atom_SbgpEntry {
		::std::uint32_t sample_count;
		::std::uint32_t group_description_index;
		friend bool operator==(const mp4_atom_SbgpEntry &a, const mp4_atom_SbgpEntry &b);
		friend bool operator!=(const mp4_atom_SbgpEntry &a, const mp4_atom_SbgpEntry &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Sbgp {
		::moq::mp4_atom_FourCC grouping_type;
		::std::optional<::std::uint32_t> grouping_type_parameter;
		::std::vector<::moq::mp4_atom_SbgpEntry> entries;
		friend bool operator==(const mp4_atom_Sbgp &a, const mp4_atom_Sbgp &b);
		friend bool operator!=(const mp4_atom_Sbgp &a, const mp4_atom_Sbgp &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_AnySampleGroupEntry_DirectReferenceSampleList {
		::std::uint32_t _0;
		::std::vector<::std::uint32_t> _1;
	};
	struct mp4_atom_AnySampleGroupEntry_UnknownGroupingType {
		::moq::mp4_atom_FourCC _0;
		::moq::buf _1;
	};
	struct mp4_atom_SgpdEntry {
		::std::optional<::std::uint32_t> description_length;
		::moq::mp4_atom_AnySampleGroupEntry entry;
		friend bool operator==(const mp4_atom_SgpdEntry &a, const mp4_atom_SgpdEntry &b);
		friend bool operator!=(const mp4_atom_SgpdEntry &a, const mp4_atom_SgpdEntry &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Sgpd {
		::moq::mp4_atom_FourCC grouping_type;
		::std::optional<::std::uint32_t> default_length;
		::std::optional<::std::uint32_t> default_group_description_index;
		bool static_group_description;
		bool static_mapping;
		bool essential;
		::std::vector<::moq::mp4_atom_SgpdEntry> entries;
		friend bool operator==(const mp4_atom_Sgpd &a, const mp4_atom_Sgpd &b);
		friend bool operator!=(const mp4_atom_Sgpd &a, const mp4_atom_Sgpd &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_SubsSubsampleSize_U16 {
		::std::uint16_t _0;
	};
	struct mp4_atom_SubsSubsampleSize_U32 {
		::std::uint32_t _0;
	};
	struct mp4_atom_SubsSubsample {
		::moq::mp4_atom_SubsSubsampleSize size;
		::std::uint8_t priority;
		bool discardable;
		::moq::buf codec_specific_parameters;
		friend bool operator==(const mp4_atom_SubsSubsample &a, const mp4_atom_SubsSubsample &b);
		friend bool operator!=(const mp4_atom_SubsSubsample &a, const mp4_atom_SubsSubsample &b);
		::moq::buf debug() const;
		static mp4_atom_SubsSubsample default_();
	};
	struct mp4_atom_SubsEntry {
		::std::uint32_t sample_delta;
		::std::vector<::moq::mp4_atom_SubsSubsample> subsamples;
		friend bool operator==(const mp4_atom_SubsEntry &a, const mp4_atom_SubsEntry &b);
		friend bool operator!=(const mp4_atom_SubsEntry &a, const mp4_atom_SubsEntry &b);
		::moq::buf debug() const;
		static mp4_atom_SubsEntry default_();
	};
	struct mp4_atom_Subs {
		::std::array<::std::uint8_t, 3> flags;
		::std::vector<::moq::mp4_atom_SubsEntry> entries;
		friend bool operator==(const mp4_atom_Subs &a, const mp4_atom_Subs &b);
		friend bool operator!=(const mp4_atom_Subs &a, const mp4_atom_Subs &b);
		::moq::buf debug() const;
		static mp4_atom_Subs default_();
	};
	struct mp4_atom_AuxInfo {
		::moq::mp4_atom_FourCC aux_info_type;
		::std::uint32_t aux_info_type_parameter;
		friend bool operator==(const mp4_atom_AuxInfo &a, const mp4_atom_AuxInfo &b);
		friend bool operator!=(const mp4_atom_AuxInfo &a, const mp4_atom_AuxInfo &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Saio {
		::std::optional<::moq::mp4_atom_AuxInfo> aux_info;
		::std::vector<::std::uint64_t> offsets;
		friend bool operator==(const mp4_atom_Saio &a, const mp4_atom_Saio &b);
		friend bool operator!=(const mp4_atom_Saio &a, const mp4_atom_Saio &b);
		::moq::buf debug() const;
		static mp4_atom_Saio default_();
	};
	struct mp4_atom_Saiz {
		::std::optional<::moq::mp4_atom_AuxInfo> aux_info;
		::std::uint8_t default_sample_info_size;
		::std::uint32_t sample_count;
		::moq::buf sample_info_size;
		friend bool operator==(const mp4_atom_Saiz &a, const mp4_atom_Saiz &b);
		friend bool operator!=(const mp4_atom_Saiz &a, const mp4_atom_Saiz &b);
		::moq::buf debug() const;
		static mp4_atom_Saiz default_();
	};
	struct mp4_atom_Url {
		::moq::buf location;
		friend bool operator==(const mp4_atom_Url &a, const mp4_atom_Url &b);
		friend bool operator!=(const mp4_atom_Url &a, const mp4_atom_Url &b);
		::moq::buf debug() const;
		static mp4_atom_Url default_();
	};
	struct mp4_atom_Urn {
		::moq::buf name;
		::moq::buf location;
		friend bool operator==(const mp4_atom_Urn &a, const mp4_atom_Urn &b);
		friend bool operator!=(const mp4_atom_Urn &a, const mp4_atom_Urn &b);
		::moq::buf debug() const;
		static mp4_atom_Urn default_();
	};
	struct mp4_atom_DrefEntry_Url {
		::moq::mp4_atom_Url _0;
	};
	struct mp4_atom_DrefEntry_Urn {
		::moq::mp4_atom_Urn _0;
	};
	struct mp4_atom_DrefEntry_Unknown {
		::moq::mp4_atom_FourCC _0;
		::moq::buf _1;
	};
	struct mp4_atom_Dref {
		::std::vector<::moq::mp4_atom_DrefEntry> entries;
		friend bool operator==(const mp4_atom_Dref &a, const mp4_atom_Dref &b);
		friend bool operator!=(const mp4_atom_Dref &a, const mp4_atom_Dref &b);
		::moq::buf debug() const;
		static mp4_atom_Dref default_();
	};
	struct mp4_atom_Dinf {
		::moq::mp4_atom_Dref dref;
		friend bool operator==(const mp4_atom_Dinf &a, const mp4_atom_Dinf &b);
		friend bool operator!=(const mp4_atom_Dinf &a, const mp4_atom_Dinf &b);
		::moq::buf debug() const;
		static mp4_atom_Dinf default_();
	};
	struct mp4_atom_Hmhd {
		::std::uint16_t max_pdu_size;
		::std::uint16_t avg_pdu_size;
		::std::uint32_t max_bitrate;
		::std::uint32_t avg_bitrate;
		friend bool operator==(const mp4_atom_Hmhd &a, const mp4_atom_Hmhd &b);
		friend bool operator!=(const mp4_atom_Hmhd &a, const mp4_atom_Hmhd &b);
		::moq::buf debug() const;
		static mp4_atom_Hmhd default_();
	};
	struct mp4_atom_Smhd {
		::moq::mp4_atom_FixedPoint_i8 balance;
		friend bool operator==(const mp4_atom_Smhd &a, const mp4_atom_Smhd &b);
		friend bool operator!=(const mp4_atom_Smhd &a, const mp4_atom_Smhd &b);
		::moq::buf debug() const;
		static mp4_atom_Smhd default_();
	};
	struct mp4_atom_RgbColor {
		::std::uint16_t red;
		::std::uint16_t green;
		::std::uint16_t blue;
		friend bool operator==(const mp4_atom_RgbColor &a, const mp4_atom_RgbColor &b);
		friend bool operator!=(const mp4_atom_RgbColor &a, const mp4_atom_RgbColor &b);
		::moq::buf debug() const;
		static mp4_atom_RgbColor default_();
	};
	struct mp4_atom_Vmhd {
		::std::uint16_t graphics_mode;
		::moq::mp4_atom_RgbColor op_color;
		friend bool operator==(const mp4_atom_Vmhd &a, const mp4_atom_Vmhd &b);
		friend bool operator!=(const mp4_atom_Vmhd &a, const mp4_atom_Vmhd &b);
		::moq::buf debug() const;
		static mp4_atom_Vmhd default_();
	};
	struct mp4_atom_Mehd {
		::std::uint64_t fragment_duration;
		friend bool operator==(const mp4_atom_Mehd &a, const mp4_atom_Mehd &b);
		friend bool operator!=(const mp4_atom_Mehd &a, const mp4_atom_Mehd &b);
		::moq::buf debug() const;
		static mp4_atom_Mehd default_();
	};
	struct mp4_atom_Trex {
		::std::uint32_t track_id;
		::std::uint32_t default_sample_description_index;
		::std::uint32_t default_sample_duration;
		::std::uint32_t default_sample_size;
		::std::uint32_t default_sample_flags;
		friend bool operator==(const mp4_atom_Trex &a, const mp4_atom_Trex &b);
		friend bool operator!=(const mp4_atom_Trex &a, const mp4_atom_Trex &b);
		::moq::buf debug() const;
		static mp4_atom_Trex default_();
	};
	struct mp4_atom_Mvex {
		::std::optional<::moq::mp4_atom_Mehd> mehd;
		::std::vector<::moq::mp4_atom_Trex> trex;
		friend bool operator==(const mp4_atom_Mvex &a, const mp4_atom_Mvex &b);
		friend bool operator!=(const mp4_atom_Mvex &a, const mp4_atom_Mvex &b);
		::moq::buf debug() const;
		static mp4_atom_Mvex default_();
	};
	struct mp4_atom_EmsgTimestamp_Relative {
		::std::uint32_t _0;
	};
	struct mp4_atom_EmsgTimestamp_Absolute {
		::std::uint64_t _0;
	};
	struct mp4_atom_Emsg {
		::std::uint32_t timescale;
		::moq::mp4_atom_EmsgTimestamp presentation_time;
		::std::uint32_t event_duration;
		::std::uint32_t id;
		::moq::buf scheme_id_uri;
		::moq::buf value;
		::moq::buf message_data;
		friend bool operator==(const mp4_atom_Emsg &a, const mp4_atom_Emsg &b);
		friend bool operator!=(const mp4_atom_Emsg &a, const mp4_atom_Emsg &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Mfhd {
		::std::uint32_t sequence_number;
		friend bool operator==(const mp4_atom_Mfhd &a, const mp4_atom_Mfhd &b);
		friend bool operator!=(const mp4_atom_Mfhd &a, const mp4_atom_Mfhd &b);
		::moq::buf debug() const;
		static mp4_atom_Mfhd default_();
	};
	struct mp4_atom_Tfhd {
		::std::uint32_t track_id;
		::std::optional<::std::uint64_t> base_data_offset;
		::std::optional<::std::uint32_t> sample_description_index;
		::std::optional<::std::uint32_t> default_sample_duration;
		::std::optional<::std::uint32_t> default_sample_size;
		::std::optional<::std::uint32_t> default_sample_flags;
		bool duration_is_empty;
		bool default_base_is_moof;
		friend bool operator==(const mp4_atom_Tfhd &a, const mp4_atom_Tfhd &b);
		friend bool operator!=(const mp4_atom_Tfhd &a, const mp4_atom_Tfhd &b);
		::moq::buf debug() const;
		static mp4_atom_Tfhd default_();
	};
	struct mp4_atom_Tfdt {
		::std::uint64_t base_media_decode_time;
		friend bool operator==(const mp4_atom_Tfdt &a, const mp4_atom_Tfdt &b);
		friend bool operator!=(const mp4_atom_Tfdt &a, const mp4_atom_Tfdt &b);
		::moq::buf debug() const;
		static mp4_atom_Tfdt default_();
	};
	struct mp4_atom_TrunEntry {
		::std::optional<::std::uint32_t> duration;
		::std::optional<::std::uint32_t> size;
		::std::optional<::std::uint32_t> flags;
		::std::optional<::std::int64_t> cts;
		friend bool operator==(const mp4_atom_TrunEntry &a, const mp4_atom_TrunEntry &b);
		friend bool operator!=(const mp4_atom_TrunEntry &a, const mp4_atom_TrunEntry &b);
		::moq::buf debug() const;
		static mp4_atom_TrunEntry default_();
	};
	struct mp4_atom_TrunSamples_Inherited {
		::std::uint32_t count;
		::std::optional<::std::uint32_t> first_flags;
	};
	struct mp4_atom_TrunSamples_Explicit {
		::std::vector<::moq::mp4_atom_TrunEntry> _0;
	};
	struct mp4_atom_Trun {
		::std::optional<::std::int32_t> data_offset;
		::moq::mp4_atom_TrunSamples samples;
		friend bool operator==(const mp4_atom_Trun &a, const mp4_atom_Trun &b);
		friend bool operator!=(const mp4_atom_Trun &a, const mp4_atom_Trun &b);
		::moq::buf debug() const;
		static mp4_atom_Trun default_();
	};
	struct mp4_atom_Mdat {
		::moq::buf data;
		friend bool operator==(const mp4_atom_Mdat &a, const mp4_atom_Mdat &b);
		friend bool operator!=(const mp4_atom_Mdat &a, const mp4_atom_Mdat &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Free {
		::moq::mp4_atom_Zeroed zeroed;
		friend bool operator==(const mp4_atom_Free &a, const mp4_atom_Free &b);
		friend bool operator!=(const mp4_atom_Free &a, const mp4_atom_Free &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_SegmentReference {
		bool reference_type;
		::std::uint32_t referenced_size;
		::std::uint32_t subsegment_duration;
		bool starts_with_sap;
		::std::uint8_t sap_type;
		::std::uint32_t sap_delta_time;
		friend bool operator==(const mp4_atom_SegmentReference &a, const mp4_atom_SegmentReference &b);
		friend bool operator!=(const mp4_atom_SegmentReference &a, const mp4_atom_SegmentReference &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Sidx {
		::std::uint32_t reference_id;
		::std::uint32_t timescale;
		::std::uint64_t earliest_presentation_time;
		::std::uint64_t first_offset;
		::std::vector<::moq::mp4_atom_SegmentReference> references;
		friend bool operator==(const mp4_atom_Sidx &a, const mp4_atom_Sidx &b);
		friend bool operator!=(const mp4_atom_Sidx &a, const mp4_atom_Sidx &b);
		::moq::buf debug() const;
		static mp4_atom_Sidx default_();
	};
	struct mp4_atom_Prft {
		::std::uint32_t reference_track_id;
		::std::uint64_t ntp_timestamp;
		::std::uint64_t media_time;
		::moq::mp4_atom_ReferenceTime utc_time_semantics;
		friend bool operator==(const mp4_atom_Prft &a, const mp4_atom_Prft &b);
		friend bool operator!=(const mp4_atom_Prft &a, const mp4_atom_Prft &b);
		::moq::buf debug() const;
		static mp4_atom_Prft default_();
	};
	struct mp4_atom_FragmentInfo {
		::std::uint64_t time;
		::std::uint64_t moof_offset;
		::std::uint32_t traf_number;
		::std::uint32_t trun_number;
		::std::uint32_t sample_delta;
		friend bool operator==(const mp4_atom_FragmentInfo &a, const mp4_atom_FragmentInfo &b);
		friend bool operator!=(const mp4_atom_FragmentInfo &a, const mp4_atom_FragmentInfo &b);
		::moq::buf debug() const;
		static mp4_atom_FragmentInfo default_();
	};
	struct mp4_atom_Tfra {
		::std::uint32_t track_id;
		::std::vector<::moq::mp4_atom_FragmentInfo> entries;
		friend bool operator==(const mp4_atom_Tfra &a, const mp4_atom_Tfra &b);
		friend bool operator!=(const mp4_atom_Tfra &a, const mp4_atom_Tfra &b);
		::moq::buf debug() const;
		static mp4_atom_Tfra default_();
	};
	struct mp4_atom_Mfro {
		::std::uint32_t parent_size;
		friend bool operator==(const mp4_atom_Mfro &a, const mp4_atom_Mfro &b);
		friend bool operator!=(const mp4_atom_Mfro &a, const mp4_atom_Mfro &b);
		::moq::buf debug() const;
		static mp4_atom_Mfro default_();
	};
	struct mp4_atom_Mfra {
		::std::vector<::moq::mp4_atom_Tfra> tfra;
		::moq::mp4_atom_Mfro mfro;
		friend bool operator==(const mp4_atom_Mfra &a, const mp4_atom_Mfra &b);
		friend bool operator!=(const mp4_atom_Mfra &a, const mp4_atom_Mfra &b);
		::moq::buf debug() const;
		static mp4_atom_Mfra default_();
	};
	struct mp4_atom_Svhd {
		::moq::buf metadata_source;
		friend bool operator==(const mp4_atom_Svhd &a, const mp4_atom_Svhd &b);
		friend bool operator!=(const mp4_atom_Svhd &a, const mp4_atom_Svhd &b);
		::moq::buf debug() const;
		static mp4_atom_Svhd default_();
	};
	struct mp4_atom_Must {
		::std::vector<::moq::mp4_atom_FourCC> required_box_types;
		friend bool operator==(const mp4_atom_Must &a, const mp4_atom_Must &b);
		friend bool operator!=(const mp4_atom_Must &a, const mp4_atom_Must &b);
		::moq::buf debug() const;
		static mp4_atom_Must default_();
	};
	struct mp4_atom_C2pa {
		::moq::buf box_purpose;
		::moq::buf data;
		friend bool operator==(const mp4_atom_C2pa &a, const mp4_atom_C2pa &b);
		friend bool operator!=(const mp4_atom_C2pa &a, const mp4_atom_C2pa &b);
		::moq::buf debug() const;
	};
	class mp4_atom_ExtendedType {
		friend bool operator==(const mp4_atom_ExtendedType &a, const mp4_atom_ExtendedType &b);
		friend bool operator!=(const mp4_atom_ExtendedType &a, const mp4_atom_ExtendedType &b);
		::moq::buf to_string() const;
		::moq::buf debug() const;
		static mp4_atom_ExtendedType default_();
		static ::moq::mp4_atom_ExtendedType new_(const ::std::array<::std::uint8_t, 16> &value);
	};
	struct mp4_atom_Prhd {
		::std::int32_t pose_yaw_degrees;
		::std::int32_t pose_pitch_degrees;
		::std::int32_t pose_roll_degrees;
		friend bool operator==(const mp4_atom_Prhd &a, const mp4_atom_Prhd &b);
		friend bool operator!=(const mp4_atom_Prhd &a, const mp4_atom_Prhd &b);
		::moq::buf debug() const;
		static mp4_atom_Prhd default_();
	};
	struct mp4_atom_Prji {
		::moq::mp4_atom_FourCC projection_kind;
		friend bool operator==(const mp4_atom_Prji &a, const mp4_atom_Prji &b);
		friend bool operator!=(const mp4_atom_Prji &a, const mp4_atom_Prji &b);
		::moq::buf debug() const;
		static ::moq::mp4_atom_FourCC RECTILINEAR();
		static ::moq::mp4_atom_FourCC EQUIRECTANGULAR();
		static ::moq::mp4_atom_FourCC HALF_EQUIRECTANGULAR();
		static ::moq::mp4_atom_FourCC FISHEYE();
		static ::moq::mp4_atom_FourCC PARAMETRIC_IMMERSIVE();
	};
	struct mp4_atom_Stri {
		bool eye_views_reversed;
		bool has_additional_views;
		bool has_right_eye_view;
		bool has_left_eye_view;
		friend bool operator==(const mp4_atom_Stri &a, const mp4_atom_Stri &b);
		friend bool operator!=(const mp4_atom_Stri &a, const mp4_atom_Stri &b);
		::moq::buf debug() const;
		static mp4_atom_Stri default_();
	};
	struct mp4_atom_Hero {
		::std::uint8_t hero_eye_indicator;
		friend bool operator==(const mp4_atom_Hero &a, const mp4_atom_Hero &b);
		friend bool operator!=(const mp4_atom_Hero &a, const mp4_atom_Hero &b);
		::moq::buf debug() const;
		static mp4_atom_Hero default_();
	};
	struct mp4_atom_Pkin {
		::moq::mp4_atom_FourCC view_packing_kind;
		friend bool operator==(const mp4_atom_Pkin &a, const mp4_atom_Pkin &b);
		friend bool operator!=(const mp4_atom_Pkin &a, const mp4_atom_Pkin &b);
		::moq::buf debug() const;
		static ::moq::mp4_atom_FourCC SIDE_BY_SIDE();
		static ::moq::mp4_atom_FourCC OVER_UNDER();
	};
	struct mp4_atom_Equi {
		::std::uint32_t projection_bounds_top;
		::std::uint32_t projection_bounds_bottom;
		::std::uint32_t projection_bounds_left;
		::std::uint32_t projection_bounds_right;
		friend bool operator==(const mp4_atom_Equi &a, const mp4_atom_Equi &b);
		friend bool operator!=(const mp4_atom_Equi &a, const mp4_atom_Equi &b);
		::moq::buf debug() const;
		static mp4_atom_Equi default_();
	};
	struct mp4_atom_Cbmp {
		::std::uint32_t layout;
		::std::uint32_t padding;
		friend bool operator==(const mp4_atom_Cbmp &a, const mp4_atom_Cbmp &b);
		friend bool operator!=(const mp4_atom_Cbmp &a, const mp4_atom_Cbmp &b);
		::moq::buf debug() const;
		static mp4_atom_Cbmp default_();
	};
	struct mp4_atom_Mshp {
		::std::uint32_t crc;
		::moq::mp4_atom_FourCC encoding_four_cc;
		::moq::buf data;
		friend bool operator==(const mp4_atom_Mshp &a, const mp4_atom_Mshp &b);
		friend bool operator!=(const mp4_atom_Mshp &a, const mp4_atom_Mshp &b);
		::moq::buf debug() const;
	};
	struct mp4_atom_Blin {
		::std::uint32_t baseline_value;
		friend bool operator==(const mp4_atom_Blin &a, const mp4_atom_Blin &b);
		friend bool operator!=(const mp4_atom_Blin &a, const mp4_atom_Blin &b);
		::moq::buf debug() const;
		static mp4_atom_Blin default_();
	};
	struct mp4_atom_Dadj {
		::std::int32_t disparity_adjustment;
		friend bool operator==(const mp4_atom_Dadj &a, const mp4_atom_Dadj &b);
		friend bool operator!=(const mp4_atom_Dadj &a, const mp4_atom_Dadj &b);
		::moq::buf debug() const;
		static mp4_atom_Dadj default_();
	};
	struct mp4_atom_Colr_Nclx {
		::std::uint16_t colour_primaries;
		::std::uint16_t transfer_characteristics;
		::std::uint16_t matrix_coefficients;
		bool full_range_flag;
	};
	struct mp4_atom_Colr_Nclc {
		::std::uint16_t colour_primaries;
		::std::uint16_t transfer_characteristics;
		::std::uint16_t matrix_coefficients;
	};
	struct mp4_atom_Colr_Ricc {
		::moq::buf profile;
	};
	struct mp4_atom_Colr_Prof {
		::moq::buf profile;
	};
	struct mp4_atom_UncC_V1 {
		::moq::mp4_atom_FourCC profile;
	};
	struct mp4_atom_UncC_V0 {
		::moq::mp4_atom_FourCC profile;
		::std::vector<::moq::mp4_atom_UncompressedComponent> components;
		::std::uint8_t sampling_type;
		::std::uint8_t interleave_type;
		::std::uint8_t block_size;
		bool components_little_endian;
		bool block_pad_lsb;
		bool block_little_endian;
		bool block_reversed;
		bool pad_unknown;
		::std::uint32_t pixel_size;
		::std::uint32_t row_align_size;
		::std::uint32_t tile_align_size;
		::std::uint32_t num_tile_cols_minus_one;
		::std::uint32_t num_tile_rows_minus_one;
	};
	struct mp4_atom_Uuid_C2pa {
		::moq::mp4_atom_C2pa _0;
	};
	struct mp4_atom_Uuid_Unknown {
		::moq::mp4_atom_ExtendedType _0;
		::moq::buf _1;
	};
	struct mp4_atom_ProjKind_Equi {
		::moq::mp4_atom_Equi _0;
	};
	struct mp4_atom_ProjKind_Cbmp {
		::moq::mp4_atom_Cbmp _0;
	};
	struct mp4_atom_ProjKind_Mshp {
		::moq::mp4_atom_Mshp _0;
	};
	inline bool operator==(const mp4_atom_FourCC &a, const mp4_atom_FourCC &b);
	inline bool operator==(const mp4_atom_FixedPoint_u16 &a, const mp4_atom_FixedPoint_u16 &b);
	inline bool operator==(const mp4_atom_Audio &a, const mp4_atom_Audio &b);
	inline bool operator==(const mp4_atom_u24 &a, const mp4_atom_u24 &b);
	inline bool operator==(const mp4_atom_esds_DecoderSpecific &a, const mp4_atom_esds_DecoderSpecific &b);
	inline bool operator==(const mp4_atom_esds_DecoderConfig &a, const mp4_atom_esds_DecoderConfig &b);
	inline bool operator==(const mp4_atom_esds_SLConfig &a, const mp4_atom_esds_SLConfig &b);
	inline bool operator==(const mp4_atom_esds_EsDescriptor &a, const mp4_atom_esds_EsDescriptor &b);
	inline bool operator==(const mp4_atom_Esds &a, const mp4_atom_Esds &b);
	inline bool operator==(const mp4_atom_Btrt &a, const mp4_atom_Btrt &b);
	inline bool operator==(const mp4_atom_Taic &a, const mp4_atom_Taic &b);
	inline bool operator==(const mp4_atom_Mp4a &a, const mp4_atom_Mp4a &b);
	inline bool operator==(const mp4_atom_Mp3 &a, const mp4_atom_Mp3 &b);
	inline bool operator==(const mp4_atom_OpusChannelMapping &a, const mp4_atom_OpusChannelMapping &b);
	inline bool operator==(const mp4_atom_Dops &a, const mp4_atom_Dops &b);
	inline bool operator==(const mp4_atom_Opus &a, const mp4_atom_Opus &b);
	inline bool operator==(const ::moq::mp4_atom_FlacMetadataBlock &a, const ::moq::mp4_atom_FlacMetadataBlock &b);
	inline bool operator!=(const ::moq::mp4_atom_FlacMetadataBlock &a, const ::moq::mp4_atom_FlacMetadataBlock &b);
	inline ::moq::buf mp4_atom_FlacMetadataBlock_debug(const ::moq::mp4_atom_FlacMetadataBlock &self);
	inline bool operator==(const mp4_atom_Dfla &a, const mp4_atom_Dfla &b);
	inline bool operator==(const mp4_atom_Flac &a, const mp4_atom_Flac &b);
	inline bool operator==(const mp4_atom_Ac3SpecificBox &a, const mp4_atom_Ac3SpecificBox &b);
	inline bool operator==(const mp4_atom_Ac3 &a, const mp4_atom_Ac3 &b);
	inline bool operator==(const mp4_atom_Ec3IndependentSubstream &a, const mp4_atom_Ec3IndependentSubstream &b);
	inline bool operator==(const mp4_atom_Ec3SpecificBox &a, const mp4_atom_Ec3SpecificBox &b);
	inline bool operator==(const mp4_atom_Eac3 &a, const mp4_atom_Eac3 &b);
	inline bool operator==(const mp4_atom_PcmC &a, const mp4_atom_PcmC &b);
	inline bool operator==(const mp4_atom_ExplicitSpeakerPosition &a, const mp4_atom_ExplicitSpeakerPosition &b);
	inline bool operator==(const ::moq::mp4_atom_SpeakerPosition &a, const ::moq::mp4_atom_SpeakerPosition &b);
	inline bool operator!=(const ::moq::mp4_atom_SpeakerPosition &a, const ::moq::mp4_atom_SpeakerPosition &b);
	inline ::moq::buf mp4_atom_SpeakerPosition_debug(const ::moq::mp4_atom_SpeakerPosition &self);
	inline bool operator==(const ::moq::mp4_atom_ChannelStructure &a, const ::moq::mp4_atom_ChannelStructure &b);
	inline bool operator!=(const ::moq::mp4_atom_ChannelStructure &a, const ::moq::mp4_atom_ChannelStructure &b);
	inline ::moq::buf mp4_atom_ChannelStructure_debug(const ::moq::mp4_atom_ChannelStructure &self);
	inline bool operator==(const mp4_atom_Chnl &a, const mp4_atom_Chnl &b);
	inline bool operator==(const mp4_atom_Ipcm &a, const mp4_atom_Ipcm &b);
	inline bool operator==(const mp4_atom_Fpcm &a, const mp4_atom_Fpcm &b);
	inline bool operator==(const mp4_atom_Sowt &a, const mp4_atom_Sowt &b);
	inline bool operator==(const mp4_atom_Twos &a, const mp4_atom_Twos &b);
	inline bool operator==(const mp4_atom_Lpcm &a, const mp4_atom_Lpcm &b);
	inline bool operator==(const mp4_atom_In24 &a, const mp4_atom_In24 &b);
	inline bool operator==(const mp4_atom_In32 &a, const mp4_atom_In32 &b);
	inline bool operator==(const mp4_atom_Fl32 &a, const mp4_atom_Fl32 &b);
	inline bool operator==(const mp4_atom_Fl64 &a, const mp4_atom_Fl64 &b);
	inline bool operator==(const mp4_atom_S16l &a, const mp4_atom_S16l &b);
	inline bool operator==(const mp4_atom_PlainText &a, const mp4_atom_PlainText &b);
	inline bool operator==(const mp4_atom_VttC &a, const mp4_atom_VttC &b);
	inline bool operator==(const mp4_atom_Vlab &a, const mp4_atom_Vlab &b);
	inline bool operator==(const mp4_atom_Wvtt &a, const mp4_atom_Wvtt &b);
	inline bool operator==(const mp4_atom_AmrSampleEntry &a, const mp4_atom_AmrSampleEntry &b);
	inline bool operator==(const mp4_atom_Damr &a, const mp4_atom_Damr &b);
	inline bool operator==(const mp4_atom_Samr &a, const mp4_atom_Samr &b);
	inline bool operator==(const mp4_atom_MetaData &a, const mp4_atom_MetaData &b);
	inline bool operator==(const mp4_atom_TxtC &a, const mp4_atom_TxtC &b);
	inline bool operator==(const mp4_atom_Mett &a, const mp4_atom_Mett &b);
	inline bool operator==(const mp4_atom_Metx &a, const mp4_atom_Metx &b);
	inline bool operator==(const mp4_atom_Uri &a, const mp4_atom_Uri &b);
	inline bool operator==(const mp4_atom_UriI &a, const mp4_atom_UriI &b);
	inline bool operator==(const mp4_atom_Urim &a, const mp4_atom_Urim &b);
	inline bool operator==(const mp4_atom_Camm &a, const mp4_atom_Camm &b);
	inline bool operator==(const mp4_atom_SchemeIdEntry &a, const mp4_atom_SchemeIdEntry &b);
	inline bool operator==(const mp4_atom_Silb &a, const mp4_atom_Silb &b);
	inline bool operator==(const mp4_atom_Evte &a, const mp4_atom_Evte &b);
	inline bool operator==(const mp4_atom_Keyd &a, const mp4_atom_Keyd &b);
	inline bool operator==(const ::moq::mp4_atom_DataType &a, const ::moq::mp4_atom_DataType &b);
	inline bool operator!=(const ::moq::mp4_atom_DataType &a, const ::moq::mp4_atom_DataType &b);
	inline ::moq::buf mp4_atom_DataType_debug(const ::moq::mp4_atom_DataType &self);
	inline bool operator==(const mp4_atom_Dtyp &a, const mp4_atom_Dtyp &b);
	inline bool operator==(const mp4_atom_Loca &a, const mp4_atom_Loca &b);
	inline bool operator==(const mp4_atom_Setu &a, const mp4_atom_Setu &b);
	inline bool operator==(const mp4_atom_MebxKey &a, const mp4_atom_MebxKey &b);
	inline bool operator==(const mp4_atom_Mebx &a, const mp4_atom_Mebx &b);
	inline bool operator==(const mp4_atom_FixedPoint_u8 &a, const mp4_atom_FixedPoint_u8 &b);
	inline bool operator==(const mp4_atom_Matrix &a, const mp4_atom_Matrix &b);
	inline bool operator==(const mp4_atom_Tkhd &a, const mp4_atom_Tkhd &b);
	inline bool operator==(const mp4_atom_FixedPoint_i16 &a, const mp4_atom_FixedPoint_i16 &b);
	inline bool operator==(const mp4_atom_ElstEntry &a, const mp4_atom_ElstEntry &b);
	inline bool operator==(const mp4_atom_Elst &a, const mp4_atom_Elst &b);
	inline bool operator==(const mp4_atom_Edts &a, const mp4_atom_Edts &b);
	inline bool operator==(const mp4_atom_Senc &a, const mp4_atom_Senc &b);
	inline bool operator==(const mp4_atom_TrackReferenceTypeBox &a, const mp4_atom_TrackReferenceTypeBox &b);
	inline bool operator==(const mp4_atom_Tref &a, const mp4_atom_Tref &b);
	inline bool operator==(const mp4_atom_AvccExt &a, const mp4_atom_AvccExt &b);
	inline bool operator==(const mp4_atom_Avcc &a, const mp4_atom_Avcc &b);
	inline bool operator==(const mp4_atom_Pasp &a, const mp4_atom_Pasp &b);
	inline bool operator==(const mp4_atom_Fiel &a, const mp4_atom_Fiel &b);
	inline bool operator==(const mp4_atom_HvcCArray &a, const mp4_atom_HvcCArray &b);
	inline bool operator==(const mp4_atom_Hvcc &a, const mp4_atom_Hvcc &b);
	inline bool operator==(const mp4_atom_Lhvc &a, const mp4_atom_Lhvc &b);
	inline bool operator==(const mp4_atom_Ccst &a, const mp4_atom_Ccst &b);
	inline bool operator==(const mp4_atom_VpcC &a, const mp4_atom_VpcC &b);
	inline bool operator==(const mp4_atom_Av1c &a, const mp4_atom_Av1c &b);
	inline bool operator==(const mp4_atom_RgbaColor &a, const mp4_atom_RgbaColor &b);
	inline bool operator==(const mp4_atom_FontEntry &a, const mp4_atom_FontEntry &b);
	inline bool operator==(const mp4_atom_Ftab &a, const mp4_atom_Ftab &b);
	inline bool operator==(const mp4_atom_Disp &a, const mp4_atom_Disp &b);
	inline bool operator==(const mp4_atom_Component &a, const mp4_atom_Component &b);
	inline bool operator==(const mp4_atom_Cmpd &a, const mp4_atom_Cmpd &b);
	inline bool operator==(const mp4_atom_FixedPoint_i8 &a, const mp4_atom_FixedPoint_i8 &b);
	inline bool operator==(const mp4_atom_Hdlr &a, const mp4_atom_Hdlr &b);
	inline bool operator==(const mp4_atom_Mdhd &a, const mp4_atom_Mdhd &b);
	inline bool operator==(const mp4_atom_ChplEntry &a, const mp4_atom_ChplEntry &b);
	inline bool operator==(const mp4_atom_Chpl &a, const mp4_atom_Chpl &b);
	inline bool operator==(const mp4_atom_Cprt &a, const mp4_atom_Cprt &b);
	inline bool operator==(const mp4_atom_Kind &a, const mp4_atom_Kind &b);
	inline bool operator==(const mp4_atom_Rtng &a, const mp4_atom_Rtng &b);
	inline bool operator==(const mp4_atom_PcmFormat &a, const mp4_atom_PcmFormat &b);
	inline bool operator==(const mp4_atom_St3d &a, const mp4_atom_St3d &b);
	inline bool operator==(const mp4_atom_Hfov &a, const mp4_atom_Hfov &b);
	inline bool operator==(const mp4_atom_UncompressedComponent &a, const mp4_atom_UncompressedComponent &b);
	inline bool operator==(const mp4_atom_Ftyp &a, const mp4_atom_Ftyp &b);
	inline bool operator==(const mp4_atom_Styp &a, const mp4_atom_Styp &b);
	inline bool operator==(const mp4_atom_Pitm &a, const mp4_atom_Pitm &b);
	inline bool operator==(const mp4_atom_ItemLocationExtent &a, const mp4_atom_ItemLocationExtent &b);
	inline bool operator==(const mp4_atom_ItemLocation &a, const mp4_atom_ItemLocation &b);
	inline bool operator==(const mp4_atom_Iloc &a, const mp4_atom_Iloc &b);
	inline bool operator==(const mp4_atom_ItemInfoEntry &a, const mp4_atom_ItemInfoEntry &b);
	inline bool operator==(const mp4_atom_Iinf &a, const mp4_atom_Iinf &b);
	inline bool operator==(const mp4_atom_Auxc &a, const mp4_atom_Auxc &b);
	inline bool operator==(const mp4_atom_Clap &a, const mp4_atom_Clap &b);
	inline bool operator==(const mp4_atom_Imir &a, const mp4_atom_Imir &b);
	inline bool operator==(const mp4_atom_Irot &a, const mp4_atom_Irot &b);
	inline bool operator==(const mp4_atom_Iscl &a, const mp4_atom_Iscl &b);
	inline bool operator==(const mp4_atom_Ispe &a, const mp4_atom_Ispe &b);
	inline bool operator==(const mp4_atom_Pixi &a, const mp4_atom_Pixi &b);
	inline bool operator==(const mp4_atom_Rref &a, const mp4_atom_Rref &b);
	inline bool operator==(const mp4_atom_PropertyAssociation &a, const mp4_atom_PropertyAssociation &b);
	inline bool operator==(const mp4_atom_PropertyAssociations &a, const mp4_atom_PropertyAssociations &b);
	inline bool operator==(const mp4_atom_Ipma &a, const mp4_atom_Ipma &b);
	inline bool operator==(const mp4_atom_Reference &a, const mp4_atom_Reference &b);
	inline bool operator==(const mp4_atom_Iref &a, const mp4_atom_Iref &b);
	inline bool operator==(const mp4_atom_Idat &a, const mp4_atom_Idat &b);
	inline bool operator==(const mp4_atom_Name &a, const mp4_atom_Name &b);
	inline bool operator==(const mp4_atom_Year &a, const mp4_atom_Year &b);
	inline bool operator==(const mp4_atom_Covr &a, const mp4_atom_Covr &b);
	inline bool operator==(const mp4_atom_Desc &a, const mp4_atom_Desc &b);
	inline bool operator==(const mp4_atom_Tool &a, const mp4_atom_Tool &b);
	inline bool operator==(const mp4_atom_Copyright &a, const mp4_atom_Copyright &b);
	inline bool operator==(const ::moq::mp4_atom_IlstDataValue &a, const ::moq::mp4_atom_IlstDataValue &b);
	inline bool operator!=(const ::moq::mp4_atom_IlstDataValue &a, const ::moq::mp4_atom_IlstDataValue &b);
	inline ::moq::buf mp4_atom_IlstDataValue_debug(const ::moq::mp4_atom_IlstDataValue &self);
	inline bool operator==(const mp4_atom_IlstData &a, const mp4_atom_IlstData &b);
	inline bool operator==(const mp4_atom_Ilst &a, const mp4_atom_Ilst &b);
	inline bool operator==(const mp4_atom_KeyEntry &a, const mp4_atom_KeyEntry &b);
	inline bool operator==(const mp4_atom_Keys &a, const mp4_atom_Keys &b);
	inline bool operator==(const mp4_atom_Mvhd &a, const mp4_atom_Mvhd &b);
	inline bool operator==(const mp4_atom_Ainf &a, const mp4_atom_Ainf &b);
	inline bool operator==(const mp4_atom_Zeroed &a, const mp4_atom_Zeroed &b);
	inline bool operator==(const mp4_atom_Skip &a, const mp4_atom_Skip &b);
	inline bool operator==(const mp4_atom_SttsEntry &a, const mp4_atom_SttsEntry &b);
	inline bool operator==(const mp4_atom_Stts &a, const mp4_atom_Stts &b);
	inline bool operator==(const mp4_atom_StscEntry &a, const mp4_atom_StscEntry &b);
	inline bool operator==(const mp4_atom_Stsc &a, const mp4_atom_Stsc &b);
	inline bool operator==(const ::moq::mp4_atom_StszSamples &a, const ::moq::mp4_atom_StszSamples &b);
	inline bool operator!=(const ::moq::mp4_atom_StszSamples &a, const ::moq::mp4_atom_StszSamples &b);
	inline ::moq::buf mp4_atom_StszSamples_debug(const ::moq::mp4_atom_StszSamples &self);
	inline mp4_atom_StszSamples mp4_atom_StszSamples_default();
	inline bool operator==(const mp4_atom_Stsz &a, const mp4_atom_Stsz &b);
	inline bool operator==(const mp4_atom_Stss &a, const mp4_atom_Stss &b);
	inline bool operator==(const mp4_atom_Stco &a, const mp4_atom_Stco &b);
	inline bool operator==(const mp4_atom_Co64 &a, const mp4_atom_Co64 &b);
	inline bool operator==(const mp4_atom_Cslg &a, const mp4_atom_Cslg &b);
	inline bool operator==(const mp4_atom_CttsEntry &a, const mp4_atom_CttsEntry &b);
	inline bool operator==(const mp4_atom_Ctts &a, const mp4_atom_Ctts &b);
	inline bool operator==(const mp4_atom_SbgpEntry &a, const mp4_atom_SbgpEntry &b);
	inline bool operator==(const mp4_atom_Sbgp &a, const mp4_atom_Sbgp &b);
	inline bool operator==(const ::moq::mp4_atom_AnySampleGroupEntry &a, const ::moq::mp4_atom_AnySampleGroupEntry &b);
	inline bool operator!=(const ::moq::mp4_atom_AnySampleGroupEntry &a, const ::moq::mp4_atom_AnySampleGroupEntry &b);
	inline ::moq::buf mp4_atom_AnySampleGroupEntry_debug(const ::moq::mp4_atom_AnySampleGroupEntry &self);
	inline bool operator==(const mp4_atom_SgpdEntry &a, const mp4_atom_SgpdEntry &b);
	inline bool operator==(const mp4_atom_Sgpd &a, const mp4_atom_Sgpd &b);
	inline bool operator==(const ::moq::mp4_atom_SubsSubsampleSize &a, const ::moq::mp4_atom_SubsSubsampleSize &b);
	inline bool operator!=(const ::moq::mp4_atom_SubsSubsampleSize &a, const ::moq::mp4_atom_SubsSubsampleSize &b);
	inline ::moq::buf mp4_atom_SubsSubsampleSize_debug(const ::moq::mp4_atom_SubsSubsampleSize &self);
	inline mp4_atom_SubsSubsampleSize mp4_atom_SubsSubsampleSize_default();
	inline ::std::uint32_t mp4_atom_SubsSubsampleSize_value(const ::moq::mp4_atom_SubsSubsampleSize &self);
	inline bool operator==(const mp4_atom_SubsSubsample &a, const mp4_atom_SubsSubsample &b);
	inline bool operator==(const mp4_atom_SubsEntry &a, const mp4_atom_SubsEntry &b);
	inline bool operator==(const mp4_atom_Subs &a, const mp4_atom_Subs &b);
	inline bool operator==(const mp4_atom_AuxInfo &a, const mp4_atom_AuxInfo &b);
	inline bool operator==(const mp4_atom_Saio &a, const mp4_atom_Saio &b);
	inline bool operator==(const mp4_atom_Saiz &a, const mp4_atom_Saiz &b);
	inline bool operator==(const mp4_atom_Url &a, const mp4_atom_Url &b);
	inline bool operator==(const mp4_atom_Urn &a, const mp4_atom_Urn &b);
	inline bool operator==(const ::moq::mp4_atom_DrefEntry &a, const ::moq::mp4_atom_DrefEntry &b);
	inline bool operator!=(const ::moq::mp4_atom_DrefEntry &a, const ::moq::mp4_atom_DrefEntry &b);
	inline ::moq::buf mp4_atom_DrefEntry_debug(const ::moq::mp4_atom_DrefEntry &self);
	inline ::moq::mp4_atom_FourCC mp4_atom_DrefEntry_kind(const ::moq::mp4_atom_DrefEntry &self);
	inline bool operator==(const mp4_atom_Dref &a, const mp4_atom_Dref &b);
	inline bool operator==(const mp4_atom_Dinf &a, const mp4_atom_Dinf &b);
	inline bool operator==(const mp4_atom_Hmhd &a, const mp4_atom_Hmhd &b);
	inline bool operator==(const mp4_atom_Smhd &a, const mp4_atom_Smhd &b);
	inline bool operator==(const mp4_atom_RgbColor &a, const mp4_atom_RgbColor &b);
	inline bool operator==(const mp4_atom_Vmhd &a, const mp4_atom_Vmhd &b);
	inline bool operator==(const mp4_atom_Mehd &a, const mp4_atom_Mehd &b);
	inline bool operator==(const mp4_atom_Trex &a, const mp4_atom_Trex &b);
	inline bool operator==(const mp4_atom_Mvex &a, const mp4_atom_Mvex &b);
	inline bool operator==(const ::moq::mp4_atom_EmsgTimestamp &a, const ::moq::mp4_atom_EmsgTimestamp &b);
	inline bool operator!=(const ::moq::mp4_atom_EmsgTimestamp &a, const ::moq::mp4_atom_EmsgTimestamp &b);
	inline ::moq::buf mp4_atom_EmsgTimestamp_debug(const ::moq::mp4_atom_EmsgTimestamp &self);
	inline bool operator==(const mp4_atom_Emsg &a, const mp4_atom_Emsg &b);
	inline bool operator==(const mp4_atom_Mfhd &a, const mp4_atom_Mfhd &b);
	inline bool operator==(const mp4_atom_Tfhd &a, const mp4_atom_Tfhd &b);
	inline bool operator==(const mp4_atom_Tfdt &a, const mp4_atom_Tfdt &b);
	inline bool operator==(const mp4_atom_TrunEntry &a, const mp4_atom_TrunEntry &b);
	inline bool operator==(const ::moq::mp4_atom_TrunSamples &a, const ::moq::mp4_atom_TrunSamples &b);
	inline bool operator!=(const ::moq::mp4_atom_TrunSamples &a, const ::moq::mp4_atom_TrunSamples &b);
	inline ::moq::buf mp4_atom_TrunSamples_debug(const ::moq::mp4_atom_TrunSamples &self);
	inline mp4_atom_TrunSamples mp4_atom_TrunSamples_default();
	inline ::std::size_t mp4_atom_TrunSamples_len(const ::moq::mp4_atom_TrunSamples &self);
	inline bool mp4_atom_TrunSamples_is_empty(const ::moq::mp4_atom_TrunSamples &self);
	inline ::std::optional<::moq::mp4_atom_TrunEntry> mp4_atom_TrunSamples_get(const ::moq::mp4_atom_TrunSamples &self, ::std::size_t index);
	inline bool operator==(const mp4_atom_Trun &a, const mp4_atom_Trun &b);
	inline bool operator==(const mp4_atom_Mdat &a, const mp4_atom_Mdat &b);
	inline bool operator==(const mp4_atom_Free &a, const mp4_atom_Free &b);
	inline bool operator==(const mp4_atom_SegmentReference &a, const mp4_atom_SegmentReference &b);
	inline bool operator==(const mp4_atom_Sidx &a, const mp4_atom_Sidx &b);
	inline bool operator==(const mp4_atom_Prft &a, const mp4_atom_Prft &b);
	inline bool operator==(const mp4_atom_FragmentInfo &a, const mp4_atom_FragmentInfo &b);
	inline bool operator==(const mp4_atom_Tfra &a, const mp4_atom_Tfra &b);
	inline bool operator==(const mp4_atom_Mfro &a, const mp4_atom_Mfro &b);
	inline bool operator==(const mp4_atom_Mfra &a, const mp4_atom_Mfra &b);
	inline bool operator==(const mp4_atom_Svhd &a, const mp4_atom_Svhd &b);
	inline bool operator==(const mp4_atom_Must &a, const mp4_atom_Must &b);
	inline bool operator==(const mp4_atom_C2pa &a, const mp4_atom_C2pa &b);
	inline bool operator==(const mp4_atom_ExtendedType &a, const mp4_atom_ExtendedType &b);
	inline bool operator==(const mp4_atom_Prhd &a, const mp4_atom_Prhd &b);
	inline bool operator==(const mp4_atom_Prji &a, const mp4_atom_Prji &b);
	inline bool operator==(const mp4_atom_Stri &a, const mp4_atom_Stri &b);
	inline bool operator==(const mp4_atom_Hero &a, const mp4_atom_Hero &b);
	inline bool operator==(const mp4_atom_Pkin &a, const mp4_atom_Pkin &b);
	inline bool operator==(const mp4_atom_Equi &a, const mp4_atom_Equi &b);
	inline bool operator==(const mp4_atom_Cbmp &a, const mp4_atom_Cbmp &b);
	inline bool operator==(const mp4_atom_Mshp &a, const mp4_atom_Mshp &b);
	inline bool operator==(const mp4_atom_Blin &a, const mp4_atom_Blin &b);
	inline bool operator==(const mp4_atom_Dadj &a, const mp4_atom_Dadj &b);
	inline bool operator==(const ::moq::mp4_atom_Colr &a, const ::moq::mp4_atom_Colr &b);
	inline bool operator!=(const ::moq::mp4_atom_Colr &a, const ::moq::mp4_atom_Colr &b);
	inline ::moq::buf mp4_atom_Colr_debug(const ::moq::mp4_atom_Colr &self);
	inline mp4_atom_Colr mp4_atom_Colr_default();
	inline ::moq::result<::moq::mp4_atom_Colr, ::moq::buf> mp4_atom_Colr_new(::std::uint16_t colour_primaries, ::std::uint16_t transfer_characteristics, ::std::uint16_t matrix_coefficients, bool full_range_flag);
	inline bool operator==(const ::moq::mp4_atom_UncC &a, const ::moq::mp4_atom_UncC &b);
	inline bool operator!=(const ::moq::mp4_atom_UncC &a, const ::moq::mp4_atom_UncC &b);
	inline ::moq::buf mp4_atom_UncC_debug(const ::moq::mp4_atom_UncC &self);
	inline bool operator==(::moq::mp4_atom_SencBoxVersion a, ::moq::mp4_atom_SencBoxVersion b);
	inline bool operator!=(::moq::mp4_atom_SencBoxVersion a, ::moq::mp4_atom_SencBoxVersion b);
	inline ::moq::buf mp4_atom_SencBoxVersion_debug(::moq::mp4_atom_SencBoxVersion self);
	inline mp4_atom_SencBoxVersion mp4_atom_SencBoxVersion_default();
	inline bool operator==(::moq::mp4_atom_ClockType a, ::moq::mp4_atom_ClockType b);
	inline bool operator!=(::moq::mp4_atom_ClockType a, ::moq::mp4_atom_ClockType b);
	inline ::moq::buf mp4_atom_ClockType_debug(::moq::mp4_atom_ClockType self);
	inline mp4_atom_ClockType mp4_atom_ClockType_default();
	inline bool operator==(const ::moq::mp4_atom_Uuid &a, const ::moq::mp4_atom_Uuid &b);
	inline bool operator!=(const ::moq::mp4_atom_Uuid &a, const ::moq::mp4_atom_Uuid &b);
	inline ::moq::buf mp4_atom_Uuid_debug(const ::moq::mp4_atom_Uuid &self);
	inline bool operator==(::moq::mp4_atom_ReferenceTime a, ::moq::mp4_atom_ReferenceTime b);
	inline bool operator!=(::moq::mp4_atom_ReferenceTime a, ::moq::mp4_atom_ReferenceTime b);
	inline ::moq::buf mp4_atom_ReferenceTime_debug(::moq::mp4_atom_ReferenceTime self);
	inline mp4_atom_ReferenceTime mp4_atom_ReferenceTime_default();
	inline bool operator==(const ::moq::mp4_atom_ProjKind &a, const ::moq::mp4_atom_ProjKind &b);
	inline bool operator!=(const ::moq::mp4_atom_ProjKind &a, const ::moq::mp4_atom_ProjKind &b);
	inline ::moq::buf mp4_atom_ProjKind_debug(const ::moq::mp4_atom_ProjKind &self);
	inline bool operator==(::moq::mp4_atom_VexuProjKind a, ::moq::mp4_atom_VexuProjKind b);
	inline bool operator!=(::moq::mp4_atom_VexuProjKind a, ::moq::mp4_atom_VexuProjKind b);
	inline ::moq::buf mp4_atom_VexuProjKind_debug(::moq::mp4_atom_VexuProjKind self);
	inline ::moq::mp4_atom_FourCC mp4_atom_VexuProjKind_kind(::moq::mp4_atom_VexuProjKind self);
	inline bool operator==(::moq::mp4_atom_AudioChannelPosition a, ::moq::mp4_atom_AudioChannelPosition b);
	inline bool operator!=(::moq::mp4_atom_AudioChannelPosition a, ::moq::mp4_atom_AudioChannelPosition b);
	inline ::moq::buf mp4_atom_AudioChannelPosition_debug(::moq::mp4_atom_AudioChannelPosition self);
	inline ::std::optional<::moq::mp4_atom_AudioChannelPosition> mp4_atom_AudioChannelPosition_from_u8(::std::uint8_t value);
}
#endif

// mux.hpp
#ifndef MOQ_HPP_CRATE9
#define MOQ_HPP_CRATE9
#include "rt.hpp"
#include "hang.hpp"
#include "kio.hpp"
#include "net.hpp"
#include "mp4atom.hpp"
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
	enum class mux_timeline_Kind {
		video = 0,
		audio = 1,
	};
	enum class mux_timeline_Error {
		overflow = 0,
		before_origin = 1,
		delay_overflow = 2,
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
	enum class mux_codec_aac_adts_Error {
		truncated = 0,
		syncword = 1,
		sample_rate = 2,
		frame_length = 3,
		packet_length = 4,
	};
	enum class mux_container_fmp4_fragment_MissingDuration {
		reject = 0,
		infer_from_presentation_time = 1,
	};
	enum class mux_container_ts_stats_Class {
		audio = 1,
		video = 2,
		data = 0,
	};
	inline constexpr ::std::size_t mux_container_DEFAULT_MAX_BUFFER = MOQ_MUX_CONTAINER_DEFAULT_MAX_BUFFER;
	inline constexpr ::std::size_t mux_container_fmp4_MAX_HELD_BYTES = MOQ_MUX_CONTAINER_FMP4_MAX_HELD_BYTES;
	inline constexpr ::std::size_t mux_container_fmp4_MAX_METADATA_BYTES = MOQ_MUX_CONTAINER_FMP4_MAX_METADATA_BYTES;
	inline constexpr ::std::uint64_t mux_container_fmp4_MAX_FRAGMENT_SAMPLES = MOQ_MUX_CONTAINER_FMP4_MAX_FRAGMENT_SAMPLES;
	inline constexpr ::std::size_t mux_container_fmp4_MAX_TRACKS = MOQ_MUX_CONTAINER_FMP4_MAX_TRACKS;
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
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Media media_get() const;
		RUST2_FFI_CALLABLE void media_set(const ::moq::hang_catalog_Media &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Text text_get() const;
		RUST2_FFI_CALLABLE void text_set(::moq::hang_catalog_Text value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Json json_get() const;
		RUST2_FFI_CALLABLE void json_set(::moq::hang_catalog_Json value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Binary binary_get() const;
		RUST2_FFI_CALLABLE void binary_set(::moq::hang_catalog_Binary value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_M2ts> m2ts_get() const;
		RUST2_FFI_CALLABLE void m2ts_set(::std::optional<::moq::hang_catalog_M2ts> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Stats> stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(::std::optional<::moq::hang_catalog_Stats> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Echo> echo_get() const;
		RUST2_FFI_CALLABLE void echo_set(::std::optional<::moq::hang_catalog_Echo> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_Contribution>> contributions_get() const;
		RUST2_FFI_CALLABLE void contributions_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_Contribution>> value);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_hang_Extra ext_get() const;
		RUST2_FFI_CALLABLE void ext_set(::moq::mux_catalog_hang_Extra value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> check_renditions() const;
		RUST2_FFI_CALLABLE ::moq::hang_Catalog media() const;
		static ::moq::hang_Catalog_Extra_subscribe_future subscribe(const ::moq::broadcast_Consumer &broadcast);
		static ::moq::result<::moq::hang_Catalog_Extra, ::moq::hang_Error> from_slice(::moq::input v);
		static ::moq::result<::moq::hang_Catalog_Extra, ::moq::hang_Error> from_reader(::moq::input reader);
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json_pretty() const;
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
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Media media_get() const;
		RUST2_FFI_CALLABLE void media_set(const ::moq::hang_catalog_Media &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Text text_get() const;
		RUST2_FFI_CALLABLE void text_set(::moq::hang_catalog_Text value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Json json_get() const;
		RUST2_FFI_CALLABLE void json_set(::moq::hang_catalog_Json value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Binary binary_get() const;
		RUST2_FFI_CALLABLE void binary_set(::moq::hang_catalog_Binary value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_M2ts> m2ts_get() const;
		RUST2_FFI_CALLABLE void m2ts_set(::std::optional<::moq::hang_catalog_M2ts> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Stats> stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(::std::optional<::moq::hang_catalog_Stats> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Echo> echo_get() const;
		RUST2_FFI_CALLABLE void echo_set(::std::optional<::moq::hang_catalog_Echo> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_Contribution>> contributions_get() const;
		RUST2_FFI_CALLABLE void contributions_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_Contribution>> value);
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Ext ext_get() const;
		RUST2_FFI_CALLABLE void ext_set(::moq::mux_container_ts_Ext value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> check_renditions() const;
		RUST2_FFI_CALLABLE ::moq::hang_Catalog media() const;
		static ::moq::hang_Catalog_Ext_subscribe_future subscribe(const ::moq::broadcast_Consumer &broadcast);
		static ::moq::result<::moq::hang_Catalog_Ext, ::moq::hang_Error> from_slice(::moq::input v);
		static ::moq::result<::moq::hang_Catalog_Ext, ::moq::hang_Error> from_reader(::moq::input reader);
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json_pretty() const;
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
	class RUST2_FFI_CONSUMABLE hang_catalog_Consumer_Extra {
		RUST2_FFI_UNCONSUMED hang_catalog_Consumer_Extra(hang_catalog_Consumer_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Consumer_Extra &operator=(hang_catalog_Consumer_Extra &&other) noexcept = default;
		static ::moq::hang_catalog_Consumer_Extra new_(::moq::track_Subscriber track);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Consumer_Extra_next_future next();
	};
	class RUST2_FFI_CONSUMABLE hang_catalog_Consumer_Ext {
		RUST2_FFI_UNCONSUMED hang_catalog_Consumer_Ext(hang_catalog_Consumer_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		hang_catalog_Consumer_Ext &operator=(hang_catalog_Consumer_Ext &&other) noexcept = default;
		static ::moq::hang_catalog_Consumer_Ext new_(::moq::track_Subscriber track);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Consumer_Ext_next_future next();
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
	class RUST2_FFI_CONSUMABLE mux_catalog_Estimator {
		RUST2_FFI_UNCONSUMED mux_catalog_Estimator(mux_catalog_Estimator &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_Estimator &operator=(mux_catalog_Estimator &&other) noexcept = default;
		static mux_catalog_Estimator default_();
		static ::moq::mux_catalog_Estimator new_();
		RUST2_FFI_CALLABLE void write(const ::moq::Timestamp &timestamp, ::std::size_t bytes);
		RUST2_FFI_CALLABLE void cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE void discontinuity();
		RUST2_FFI_CALLABLE void reorder(const ::moq::Timestamp &delay);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_Estimate estimate() const;
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
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Media media_get() const;
		RUST2_FFI_CALLABLE void media_set(const ::moq::hang_catalog_Media &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Text text_get() const;
		RUST2_FFI_CALLABLE void text_set(::moq::hang_catalog_Text value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Json json_get() const;
		RUST2_FFI_CALLABLE void json_set(::moq::hang_catalog_Json value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Binary binary_get() const;
		RUST2_FFI_CALLABLE void binary_set(::moq::hang_catalog_Binary value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_M2ts> m2ts_get() const;
		RUST2_FFI_CALLABLE void m2ts_set(::std::optional<::moq::hang_catalog_M2ts> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Stats> stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(::std::optional<::moq::hang_catalog_Stats> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Echo> echo_get() const;
		RUST2_FFI_CALLABLE void echo_set(::std::optional<::moq::hang_catalog_Echo> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_Contribution>> contributions_get() const;
		RUST2_FFI_CALLABLE void contributions_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_Contribution>> value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> check_renditions() const;
		RUST2_FFI_CALLABLE ::moq::hang_Catalog media() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json_pretty() const;
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
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Media media_get() const;
		RUST2_FFI_CALLABLE void media_set(const ::moq::hang_catalog_Media &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Text text_get() const;
		RUST2_FFI_CALLABLE void text_set(::moq::hang_catalog_Text value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Json json_get() const;
		RUST2_FFI_CALLABLE void json_set(::moq::hang_catalog_Json value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Binary binary_get() const;
		RUST2_FFI_CALLABLE void binary_set(::moq::hang_catalog_Binary value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_M2ts> m2ts_get() const;
		RUST2_FFI_CALLABLE void m2ts_set(::std::optional<::moq::hang_catalog_M2ts> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Stats> stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(::std::optional<::moq::hang_catalog_Stats> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Echo> echo_get() const;
		RUST2_FFI_CALLABLE void echo_set(::std::optional<::moq::hang_catalog_Echo> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_Contribution>> contributions_get() const;
		RUST2_FFI_CALLABLE void contributions_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_Contribution>> value);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_hang_Extra ext_get() const;
		RUST2_FFI_CALLABLE void ext_set(::moq::mux_catalog_hang_Extra value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> check_renditions() const;
		RUST2_FFI_CALLABLE ::moq::hang_Catalog media() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json_pretty() const;
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
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Media media_get() const;
		RUST2_FFI_CALLABLE void media_set(const ::moq::hang_catalog_Media &value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Text text_get() const;
		RUST2_FFI_CALLABLE void text_set(::moq::hang_catalog_Text value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Json json_get() const;
		RUST2_FFI_CALLABLE void json_set(::moq::hang_catalog_Json value);
		RUST2_FFI_CALLABLE ::moq::hang_catalog_Binary binary_get() const;
		RUST2_FFI_CALLABLE void binary_set(::moq::hang_catalog_Binary value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_M2ts> m2ts_get() const;
		RUST2_FFI_CALLABLE void m2ts_set(::std::optional<::moq::hang_catalog_M2ts> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Stats> stats_get() const;
		RUST2_FFI_CALLABLE void stats_set(::std::optional<::moq::hang_catalog_Stats> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_Echo> echo_get() const;
		RUST2_FFI_CALLABLE void echo_set(::std::optional<::moq::hang_catalog_Echo> value);
		RUST2_FFI_CALLABLE ::std::vector<::std::tuple<::moq::buf, ::moq::hang_catalog_Contribution>> contributions_get() const;
		RUST2_FFI_CALLABLE void contributions_set(::moq::taken<::std::tuple<::moq::view, ::moq::hang_catalog_Contribution>> value);
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Ext ext_get() const;
		RUST2_FFI_CALLABLE void ext_set(::moq::mux_container_ts_Ext value);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::hang_Error> check_renditions() const;
		RUST2_FFI_CALLABLE ::moq::hang_Catalog media() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::hang_Error> to_json_pretty() const;
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
		RUST2_FFI_CALLABLE ::std::optional<::moq::hang_catalog_VideoConfig> to_config() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Consumer_Container {
		RUST2_FFI_UNCONSUMED mux_container_Consumer_Container(mux_container_Consumer_Container &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Consumer_Container &operator=(mux_container_Consumer_Container &&other) noexcept = default;
		static ::moq::mux_container_Consumer_Container new_(::moq::track_Subscriber track, ::moq::mux_catalog_hang_Container format);
		RUST2_FFI_CONSUMES ::moq::mux_container_Consumer_Container with_warmup(::moq::duration warmup) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Consumer_Container with_delay(const ::moq::track_Delay &delay) &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_Consumer_Container_read_future read();
		RUST2_FFI_CALLABLE void set_delay(const ::moq::track_Delay &delay);
		RUST2_FFI_CALLABLE void set_max_buffer(::std::size_t bytes);
	};
	class RUST2_FFI_CONSUMABLE mux_container_Consumer_fmp4_Wire {
		RUST2_FFI_UNCONSUMED mux_container_Consumer_fmp4_Wire(mux_container_Consumer_fmp4_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Consumer_fmp4_Wire &operator=(mux_container_Consumer_fmp4_Wire &&other) noexcept = default;
		static ::moq::mux_container_Consumer_fmp4_Wire new_(::moq::track_Subscriber track, ::moq::mux_container_fmp4_Wire format);
		RUST2_FFI_CONSUMES ::moq::mux_container_Consumer_fmp4_Wire with_warmup(::moq::duration warmup) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Consumer_fmp4_Wire with_delay(const ::moq::track_Delay &delay) &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_Consumer_fmp4_Wire_read_future read();
		RUST2_FFI_CALLABLE void set_delay(const ::moq::track_Delay &delay);
		RUST2_FFI_CALLABLE void set_max_buffer(::std::size_t bytes);
	};
	class RUST2_FFI_CONSUMABLE mux_container_Consumer_legacy_Wire {
		RUST2_FFI_UNCONSUMED mux_container_Consumer_legacy_Wire(mux_container_Consumer_legacy_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Consumer_legacy_Wire &operator=(mux_container_Consumer_legacy_Wire &&other) noexcept = default;
		static ::moq::mux_container_Consumer_legacy_Wire new_(::moq::track_Subscriber track, const ::moq::mux_container_legacy_Wire &format);
		RUST2_FFI_CONSUMES ::moq::mux_container_Consumer_legacy_Wire with_warmup(::moq::duration warmup) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Consumer_legacy_Wire with_delay(const ::moq::track_Delay &delay) &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_Consumer_legacy_Wire_read_future read();
		RUST2_FFI_CALLABLE void set_delay(const ::moq::track_Delay &delay);
		RUST2_FFI_CALLABLE void set_max_buffer(::std::size_t bytes);
	};
	class RUST2_FFI_CONSUMABLE mux_container_Consumer_loc_Wire {
		RUST2_FFI_UNCONSUMED mux_container_Consumer_loc_Wire(mux_container_Consumer_loc_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Consumer_loc_Wire &operator=(mux_container_Consumer_loc_Wire &&other) noexcept = default;
		static ::moq::mux_container_Consumer_loc_Wire new_(::moq::track_Subscriber track, const ::moq::mux_container_loc_Wire &format);
		RUST2_FFI_CONSUMES ::moq::mux_container_Consumer_loc_Wire with_warmup(::moq::duration warmup) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_Consumer_loc_Wire with_delay(const ::moq::track_Delay &delay) &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_Consumer_loc_Wire_read_future read();
		RUST2_FFI_CALLABLE void set_delay(const ::moq::track_Delay &delay);
		RUST2_FFI_CALLABLE void set_max_buffer(::std::size_t bytes);
	};
	class RUST2_FFI_CONSUMABLE mux_container_GroupConsumer_Container {
		RUST2_FFI_UNCONSUMED mux_container_GroupConsumer_Container(mux_container_GroupConsumer_Container &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_GroupConsumer_Container &operator=(mux_container_GroupConsumer_Container &&other) noexcept = default;
		static ::moq::mux_container_GroupConsumer_Container new_(::moq::group_Consumer group, ::moq::mux_catalog_hang_Container format);
		RUST2_FFI_CALLABLE ::std::uint64_t sequence() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_GroupConsumer_Container_read_future read();
	};
	class RUST2_FFI_CONSUMABLE mux_container_GroupConsumer_fmp4_Wire {
		RUST2_FFI_UNCONSUMED mux_container_GroupConsumer_fmp4_Wire(mux_container_GroupConsumer_fmp4_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_GroupConsumer_fmp4_Wire &operator=(mux_container_GroupConsumer_fmp4_Wire &&other) noexcept = default;
		static ::moq::mux_container_GroupConsumer_fmp4_Wire new_(::moq::group_Consumer group, ::moq::mux_container_fmp4_Wire format);
		RUST2_FFI_CALLABLE ::std::uint64_t sequence() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_GroupConsumer_fmp4_Wire_read_future read();
	};
	class RUST2_FFI_CONSUMABLE mux_container_GroupConsumer_legacy_Wire {
		RUST2_FFI_UNCONSUMED mux_container_GroupConsumer_legacy_Wire(mux_container_GroupConsumer_legacy_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_GroupConsumer_legacy_Wire &operator=(mux_container_GroupConsumer_legacy_Wire &&other) noexcept = default;
		static ::moq::mux_container_GroupConsumer_legacy_Wire new_(::moq::group_Consumer group, const ::moq::mux_container_legacy_Wire &format);
		RUST2_FFI_CALLABLE ::std::uint64_t sequence() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_GroupConsumer_legacy_Wire_read_future read();
	};
	class RUST2_FFI_CONSUMABLE mux_container_GroupConsumer_loc_Wire {
		RUST2_FFI_UNCONSUMED mux_container_GroupConsumer_loc_Wire(mux_container_GroupConsumer_loc_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_GroupConsumer_loc_Wire &operator=(mux_container_GroupConsumer_loc_Wire &&other) noexcept = default;
		static ::moq::mux_container_GroupConsumer_loc_Wire new_(::moq::group_Consumer group, const ::moq::mux_container_loc_Wire &format);
		RUST2_FFI_CALLABLE ::std::uint64_t sequence() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_GroupConsumer_loc_Wire_read_future read();
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_Container_AudioConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_Container_AudioConfig(mux_container_Producer_Container_AudioConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_Container_AudioConfig &operator=(mux_container_Producer_Container_AudioConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_AudioConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_Container_AudioConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_Container_BinaryConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_Container_BinaryConfig(mux_container_Producer_Container_BinaryConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_Container_BinaryConfig &operator=(mux_container_Producer_Container_BinaryConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_BinaryConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_Container_BinaryConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_Container_JsonConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_Container_JsonConfig(mux_container_Producer_Container_JsonConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_Container_JsonConfig &operator=(mux_container_Producer_Container_JsonConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_JsonConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_Container_JsonConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_Container_TextConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_Container_TextConfig(mux_container_Producer_Container_TextConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_Container_TextConfig &operator=(mux_container_Producer_Container_TextConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_TextConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_Container_TextConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_Container_VideoConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_Container_VideoConfig(mux_container_Producer_Container_VideoConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_Container_VideoConfig &operator=(mux_container_Producer_Container_VideoConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_VideoConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_Container_VideoConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_Container_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_fmp4_Wire_AudioConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_fmp4_Wire_AudioConfig(mux_container_Producer_fmp4_Wire_AudioConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_fmp4_Wire_AudioConfig &operator=(mux_container_Producer_fmp4_Wire_AudioConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_AudioConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_fmp4_Wire_AudioConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_fmp4_Wire_BinaryConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_fmp4_Wire_BinaryConfig(mux_container_Producer_fmp4_Wire_BinaryConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_fmp4_Wire_BinaryConfig &operator=(mux_container_Producer_fmp4_Wire_BinaryConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_BinaryConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_fmp4_Wire_BinaryConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_fmp4_Wire_JsonConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_fmp4_Wire_JsonConfig(mux_container_Producer_fmp4_Wire_JsonConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_fmp4_Wire_JsonConfig &operator=(mux_container_Producer_fmp4_Wire_JsonConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_JsonConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_fmp4_Wire_JsonConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_fmp4_Wire_TextConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_fmp4_Wire_TextConfig(mux_container_Producer_fmp4_Wire_TextConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_fmp4_Wire_TextConfig &operator=(mux_container_Producer_fmp4_Wire_TextConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_TextConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_fmp4_Wire_TextConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_fmp4_Wire_VideoConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_fmp4_Wire_VideoConfig(mux_container_Producer_fmp4_Wire_VideoConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_fmp4_Wire_VideoConfig &operator=(mux_container_Producer_fmp4_Wire_VideoConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_VideoConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_fmp4_Wire_VideoConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_fmp4_Wire_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_legacy_Wire_AudioConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_legacy_Wire_AudioConfig(mux_container_Producer_legacy_Wire_AudioConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_legacy_Wire_AudioConfig &operator=(mux_container_Producer_legacy_Wire_AudioConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_AudioConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_legacy_Wire_AudioConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_legacy_Wire_BinaryConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_legacy_Wire_BinaryConfig(mux_container_Producer_legacy_Wire_BinaryConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_legacy_Wire_BinaryConfig &operator=(mux_container_Producer_legacy_Wire_BinaryConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_BinaryConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_legacy_Wire_BinaryConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_legacy_Wire_JsonConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_legacy_Wire_JsonConfig(mux_container_Producer_legacy_Wire_JsonConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_legacy_Wire_JsonConfig &operator=(mux_container_Producer_legacy_Wire_JsonConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_JsonConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_legacy_Wire_JsonConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_legacy_Wire_TextConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_legacy_Wire_TextConfig(mux_container_Producer_legacy_Wire_TextConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_legacy_Wire_TextConfig &operator=(mux_container_Producer_legacy_Wire_TextConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_TextConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_legacy_Wire_TextConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_legacy_Wire_VideoConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_legacy_Wire_VideoConfig(mux_container_Producer_legacy_Wire_VideoConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_legacy_Wire_VideoConfig &operator=(mux_container_Producer_legacy_Wire_VideoConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_VideoConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_legacy_Wire_VideoConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_legacy_Wire_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_loc_Wire_AudioConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_loc_Wire_AudioConfig(mux_container_Producer_loc_Wire_AudioConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_loc_Wire_AudioConfig &operator=(mux_container_Producer_loc_Wire_AudioConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_AudioConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_loc_Wire_AudioConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_loc_Wire_BinaryConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_loc_Wire_BinaryConfig(mux_container_Producer_loc_Wire_BinaryConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_loc_Wire_BinaryConfig &operator=(mux_container_Producer_loc_Wire_BinaryConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_BinaryConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_loc_Wire_BinaryConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_loc_Wire_JsonConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_loc_Wire_JsonConfig(mux_container_Producer_loc_Wire_JsonConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_loc_Wire_JsonConfig &operator=(mux_container_Producer_loc_Wire_JsonConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_JsonConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_loc_Wire_JsonConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_loc_Wire_TextConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_loc_Wire_TextConfig(mux_container_Producer_loc_Wire_TextConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_loc_Wire_TextConfig &operator=(mux_container_Producer_loc_Wire_TextConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_TextConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_loc_Wire_TextConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_Producer_loc_Wire_VideoConfig {
		RUST2_FFI_UNCONSUMED mux_container_Producer_loc_Wire_VideoConfig(mux_container_Producer_loc_Wire_VideoConfig &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_Producer_loc_Wire_VideoConfig &operator=(mux_container_Producer_loc_Wire_VideoConfig &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set(::moq::hang_catalog_VideoConfig config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_bitrate(::std::uint64_t bitrate);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_loc_Wire_VideoConfig_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> fail(::moq::Error err);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber consume() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_Producer_loc_Wire_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE mux_follow_Instances {
		RUST2_FFI_UNCONSUMED mux_follow_Instances(mux_follow_Instances &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_follow_Instances &operator=(mux_follow_Instances &&other) noexcept = default;
		static ::moq::result<::moq::mux_follow_Instances, ::moq::mux_Error> new_(const ::moq::origin_Consumer &origin, ::moq::input path, const ::moq::Instance *instance);
		RUST2_FFI_CALLABLE ::moq::mux_follow_Instances_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_follow_Event {
		RUST2_FFI_UNCONSUMED mux_follow_Event(mux_follow_Event &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_follow_Event &operator=(mux_follow_Event &&other) noexcept = default;
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
	class RUST2_FFI_CONSUMABLE mux_import_Container {
		RUST2_FFI_UNCONSUMED mux_import_Container(mux_import_Container &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_Container &operator=(mux_import_Container &&other) noexcept = default;
		static ::moq::result<::moq::mux_import_Container, ::moq::mux_Error> new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved reserved, const ::moq::mux_import_ContainerInit &init);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
	};
	class RUST2_FFI_CONSUMABLE mux_import_Container_Extra {
		RUST2_FFI_UNCONSUMED mux_import_Container_Extra(mux_import_Container_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_Container_Extra &operator=(mux_import_Container_Extra &&other) noexcept = default;
		static ::moq::result<::moq::mux_import_Container_Extra, ::moq::mux_Error> new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Extra reserved, const ::moq::mux_import_ContainerInit &init);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
	};
	class RUST2_FFI_CONSUMABLE mux_import_Container_Ext {
		RUST2_FFI_UNCONSUMED mux_import_Container_Ext(mux_import_Container_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_import_Container_Ext &operator=(mux_import_Container_Ext &&other) noexcept = default;
		static ::moq::result<::moq::mux_import_Container_Ext, ::moq::mux_Error> new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Ext reserved, const ::moq::mux_import_ContainerInit &init);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_enabled(bool enabled);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_enabled(bool enabled);
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
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
	};
	class RUST2_FFI_CONSUMABLE mux_json_Consumer_Json {
		RUST2_FFI_UNCONSUMED mux_json_Consumer_Json(mux_json_Consumer_Json &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_json_Consumer_Json &operator=(mux_json_Consumer_Json &&other) noexcept = default;
		static ::moq::result<::moq::mux_json_Consumer_Json, ::moq::mux_Error> from_track(::moq::track_Subscriber track, const ::moq::hang_catalog_JsonConfig &config);
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
	class RUST2_FFI_CONSUMABLE mux_rate_Follower {
		RUST2_FFI_UNCONSUMED mux_rate_Follower(mux_rate_Follower &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_rate_Follower &operator=(mux_rate_Follower &&other) noexcept = default;
		static ::moq::mux_rate_Follower new_(::moq::bandwidth_Consumer grant, ::moq::mux_rate_Policy policy);
		RUST2_FFI_CALLABLE ::moq::mux_rate_Policy policy() const;
		RUST2_FFI_CALLABLE void set_policy(::moq::mux_rate_Policy policy);
		RUST2_FFI_CALLABLE ::moq::mux_rate_Follower_next_future next();
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
		RUST2_FFI_CALLABLE void retain_Extra(::moq::hang_Catalog_Extra &catalog, const ::moq::mux_select_Selected &selected) const;
		RUST2_FFI_CALLABLE void retain_Ext(::moq::hang_Catalog_Ext &catalog, const ::moq::mux_select_Selected &selected) const;
		RUST2_FFI_CALLABLE void retain_unit(::moq::hang_Catalog &catalog, const ::moq::mux_select_Selected &selected) const;
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
	class RUST2_FFI_CONSUMABLE mux_timeline_Track {
		RUST2_FFI_UNCONSUMED mux_timeline_Track(mux_timeline_Track &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_timeline_Track &operator=(mux_timeline_Track &&other) noexcept = default;
		RUST2_FFI_CONSUMES void retire(const ::std::optional<::moq::clock_Instant> &drained) &&;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Timestamp, ::moq::mux_timeline_Error> map(const ::moq::Timestamp &source) const;
		RUST2_FFI_CALLABLE ::moq::mux_timeline_Transition declare() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_timeline_Error> end(const ::moq::Timestamp &source) const;
		RUST2_FFI_CALLABLE void plan(const ::moq::mux_timeline_Transition &transition, const ::moq::Timestamp &source, const ::std::optional<::moq::duration> &delay) const;
		RUST2_FFI_CALLABLE void gap() const;
		RUST2_FFI_CALLABLE void abandon(const ::moq::mux_timeline_Transition &transition) const;
	};
	class RUST2_FFI_CONSUMABLE mux_timeline_Producer {
		RUST2_FFI_UNCONSUMED mux_timeline_Producer(mux_timeline_Producer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_timeline_Producer &operator=(mux_timeline_Producer &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_timeline_Producer(const mux_timeline_Producer &other) = default;
		mux_timeline_Producer &operator=(const mux_timeline_Producer &other) = default;
		static ::moq::mux_timeline_Producer new_(::moq::duration delay);
		RUST2_FFI_CONSUMES ::moq::mux_timeline_Producer reset(::moq::duration delay) &&;
		RUST2_FFI_CALLABLE ::moq::mux_timeline_Consumer consume() const;
		RUST2_FFI_CALLABLE ::moq::mux_timeline_Track track(::moq::mux_timeline_Kind kind) const;
		RUST2_FFI_CALLABLE bool video(const ::moq::Timestamp &timestamp, const ::moq::clock_Instant &now) const;
		RUST2_FFI_CALLABLE void video_restarted() const;
		RUST2_FFI_CALLABLE bool audio(const ::moq::mux_timeline_Audio &audio, const ::moq::clock_Instant &now) const;
		RUST2_FFI_CALLABLE void audio_restarted() const;
		RUST2_FFI_CALLABLE void audio_stopped() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> due(const ::moq::Timestamp &timestamp) const;
	};
	class RUST2_FFI_CONSUMABLE mux_timeline_Consumer {
		RUST2_FFI_UNCONSUMED mux_timeline_Consumer(mux_timeline_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_timeline_Consumer &operator=(mux_timeline_Consumer &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_timeline_Consumer(const mux_timeline_Consumer &other) = default;
		mux_timeline_Consumer &operator=(const mux_timeline_Consumer &other) = default;
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> playhead(const ::moq::clock_Instant &now) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> target() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> due(const ::moq::Timestamp &timestamp) const;
		RUST2_FFI_CALLABLE ::std::optional<::std::size_t> present(const ::std::vector<::moq::Timestamp> &timestamps, const ::moq::clock_Instant &now) const;
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_hang_Consumer {
		RUST2_FFI_UNCONSUMED mux_catalog_hang_Consumer(mux_catalog_hang_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_hang_Consumer &operator=(mux_catalog_hang_Consumer &&other) noexcept = default;
		static ::moq::mux_catalog_hang_Consumer new_(::moq::track_Subscriber track);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_hang_Consumer_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_hang_Consumer_Extra {
		RUST2_FFI_UNCONSUMED mux_catalog_hang_Consumer_Extra(mux_catalog_hang_Consumer_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_hang_Consumer_Extra &operator=(mux_catalog_hang_Consumer_Extra &&other) noexcept = default;
		static ::moq::mux_catalog_hang_Consumer_Extra new_(::moq::track_Subscriber track);
		RUST2_FFI_CALLABLE ::moq::mux_catalog_hang_Consumer_Extra_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_catalog_hang_Consumer_Ext {
		RUST2_FFI_UNCONSUMED mux_catalog_hang_Consumer_Ext(mux_catalog_hang_Consumer_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_catalog_hang_Consumer_Ext &operator=(mux_catalog_hang_Consumer_Ext &&other) noexcept = default;
		static ::moq::mux_catalog_hang_Consumer_Ext new_(::moq::track_Subscriber track);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_enabled(bool enabled);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_enabled(bool enabled);
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_enabled(bool enabled);
		RUST2_FFI_CALLABLE void report_skipped(::std::uint64_t frames) const;
		RUST2_FFI_CALLABLE void report_bitrate(const ::moq::bandwidth_Rate &bitrate) const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_enabled(bool enabled);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut(const ::std::optional<::moq::Timestamp> &end);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> set_enabled(bool enabled);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
	};
	class RUST2_FFI_CONSUMABLE mux_container_flv_Import {
		RUST2_FFI_UNCONSUMED mux_container_flv_Import(mux_container_flv_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_flv_Import &operator=(mux_container_flv_Import &&other) noexcept = default;
		static ::moq::mux_container_flv_Import new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_flv_Import with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_flv_Import_Extra {
		RUST2_FFI_UNCONSUMED mux_container_flv_Import_Extra(mux_container_flv_Import_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_flv_Import_Extra &operator=(mux_container_flv_Import_Extra &&other) noexcept = default;
		static ::moq::mux_container_flv_Import_Extra new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Extra reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_flv_Import_Extra with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_flv_Import_Ext {
		RUST2_FFI_UNCONSUMED mux_container_flv_Import_Ext(mux_container_flv_Import_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_flv_Import_Ext &operator=(mux_container_flv_Import_Ext &&other) noexcept = default;
		static ::moq::mux_container_flv_Import_Ext new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Ext reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_flv_Import_Ext with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Wire {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Wire(mux_container_fmp4_Wire &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Wire &operator=(mux_container_fmp4_Wire &&other) noexcept = default;
		static ::moq::result<::moq::mux_container_fmp4_Wire, ::moq::mux_container_fmp4_Error> new_(::moq::mp4_atom_Trak trak);
		static ::moq::result<::moq::mux_container_fmp4_Wire, ::moq::mux_container_fmp4_Error> from_init(::moq::input init_data);
		RUST2_FFI_CALLABLE ::moq::mp4_atom_Trak trak() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Error {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Error(mux_container_fmp4_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Error &operator=(mux_container_fmp4_Error &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Error(const mux_container_fmp4_Error &other) = default;
		mux_container_fmp4_Error &operator=(const mux_container_fmp4_Error &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Fragmenter {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Fragmenter(mux_container_fmp4_Fragmenter &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Fragmenter &operator=(mux_container_fmp4_Fragmenter &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::result<::std::vector<::moq::mux_container_fmp4_Fragment>, ::moq::mux_Error> push(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CALLABLE ::moq::result<::std::vector<::moq::mux_container_fmp4_Fragment>, ::moq::mux_Error> push_group(const ::moq::mux_container_Frame &frame);
		RUST2_FFI_CONSUMES ::moq::result<::std::optional<::moq::mux_container_fmp4_Fragment>, ::moq::mux_Error> flush() &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Import {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Import(mux_container_fmp4_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Import &operator=(mux_container_fmp4_Import &&other) noexcept = default;
		static ::moq::mux_container_fmp4_Import new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved reserved);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut();
		RUST2_FFI_CONSUMES ::moq::mux_container_fmp4_Import with_select(::moq::mux_select_Broadcast select) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_fmp4_Import with_passthrough(bool passthrough) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Import_Extra {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Import_Extra(mux_container_fmp4_Import_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Import_Extra &operator=(mux_container_fmp4_Import_Extra &&other) noexcept = default;
		static ::moq::mux_container_fmp4_Import_Extra new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Extra reserved);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut();
		RUST2_FFI_CONSUMES ::moq::mux_container_fmp4_Import_Extra with_select(::moq::mux_select_Broadcast select) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_fmp4_Import_Extra with_passthrough(bool passthrough) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Import_Ext {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Import_Ext(mux_container_fmp4_Import_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Import_Ext &operator=(mux_container_fmp4_Import_Ext &&other) noexcept = default;
		static ::moq::mux_container_fmp4_Import_Ext new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Ext reserved);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> cut();
		RUST2_FFI_CONSUMES ::moq::mux_container_fmp4_Import_Ext with_select(::moq::mux_select_Broadcast select) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_fmp4_Import_Ext with_passthrough(bool passthrough) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> discontinuity();
	};
	class RUST2_FFI_CONSUMABLE mux_container_fmp4_Muxer {
		RUST2_FFI_UNCONSUMED mux_container_fmp4_Muxer(mux_container_fmp4_Muxer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_fmp4_Muxer &operator=(mux_container_fmp4_Muxer &&other) noexcept = default;
		static ::moq::result<::moq::mux_container_fmp4_Muxer, ::moq::mux_Error> video(const ::moq::hang_catalog_VideoConfig &config);
		static ::moq::result<::moq::mux_container_fmp4_Muxer, ::moq::mux_Error> audio(const ::moq::hang_catalog_AudioConfig &config);
		RUST2_FFI_CALLABLE ::moq::Timescale timescale() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_fmp4_Muxer, ::moq::mux_Error> with_timescale(const ::moq::Timescale &timescale) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_fmp4_Muxer with_framerate(const ::std::optional<double> &framerate) &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_fmp4_Muxer_read_future read(::moq::group_Consumer &group);
		RUST2_FFI_CALLABLE ::moq::result<::std::optional<::moq::buf>, ::moq::mux_Error> init() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::buf, ::moq::mux_Error> fragment(::std::uint32_t sequence, const ::std::vector<::moq::mux_container_Frame> &frames, const ::std::optional<::moq::Timestamp> &end) const;
		RUST2_FFI_CALLABLE ::moq::mux_container_fmp4_Fragmenter fragmenter(const ::moq::mux_container_fmp4_fragment_Config &config) const;
	};
	class RUST2_FFI_CONSUMABLE mux_container_mkv_Import {
		RUST2_FFI_UNCONSUMED mux_container_mkv_Import(mux_container_mkv_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_mkv_Import &operator=(mux_container_mkv_Import &&other) noexcept = default;
		static ::moq::mux_container_mkv_Import new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_mkv_Import with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_mkv_Import_Extra {
		RUST2_FFI_UNCONSUMED mux_container_mkv_Import_Extra(mux_container_mkv_Import_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_mkv_Import_Extra &operator=(mux_container_mkv_Import_Extra &&other) noexcept = default;
		static ::moq::mux_container_mkv_Import_Extra new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Extra reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_mkv_Import_Extra with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_mkv_Import_Ext {
		RUST2_FFI_UNCONSUMED mux_container_mkv_Import_Ext(mux_container_mkv_Import_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_mkv_Import_Ext &operator=(mux_container_mkv_Import_Ext &&other) noexcept = default;
		static ::moq::mux_container_mkv_Import_Ext new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Ext reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_mkv_Import_Ext with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
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
		RUST2_FFI_CALLABLE bool is_empty() const;
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
	class RUST2_FFI_CONSUMABLE mux_container_ts_Export_Consumer_Extra {
		RUST2_FFI_UNCONSUMED mux_container_ts_Export_Consumer_Extra(mux_container_ts_Export_Consumer_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Export_Consumer_Extra &operator=(mux_container_ts_Export_Consumer_Extra &&other) noexcept = default;
		static ::moq::mux_container_ts_Export_Consumer_Extra new_(::moq::mux_Source source, ::moq::mux_catalog_Consumer_Extra catalog);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Export_Consumer_Extra with_mux_rate(::std::uint64_t mux_rate) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Export_Consumer_Extra with_delay(::moq::duration delay) &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Export_Consumer_Extra_next_future next();
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Export stats() const;
		RUST2_FFI_CALLABLE ::std::uint64_t discontinuity() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_ts_Export_Consumer_Extra, ::moq::mux_Error> follow(::moq::broadcast_Consumer broadcast, ::moq::mux_catalog_Consumer_Extra catalog) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Export_Consumer_Ext {
		RUST2_FFI_UNCONSUMED mux_container_ts_Export_Consumer_Ext(mux_container_ts_Export_Consumer_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Export_Consumer_Ext &operator=(mux_container_ts_Export_Consumer_Ext &&other) noexcept = default;
		static ::moq::mux_container_ts_Export_Consumer_Ext new_(::moq::mux_Source source, ::moq::mux_catalog_Consumer_Ext catalog);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Export_Consumer_Ext with_mux_rate(::std::uint64_t mux_rate) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Export_Consumer_Ext with_delay(::moq::duration delay) &&;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Export_Consumer_Ext_next_future next();
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Export stats() const;
		RUST2_FFI_CALLABLE ::std::uint64_t discontinuity() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::mux_container_ts_Export_Consumer_Ext, ::moq::mux_Error> follow(::moq::broadcast_Consumer broadcast, ::moq::mux_catalog_Consumer_Ext catalog) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Export_Consumer {
		RUST2_FFI_UNCONSUMED mux_container_ts_Export_Consumer(mux_container_ts_Export_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Export_Consumer &operator=(mux_container_ts_Export_Consumer &&other) noexcept = default;
		static ::moq::mux_container_ts_Export_Consumer new_(::moq::mux_Source source, ::moq::mux_catalog_Consumer catalog);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Export_Consumer with_mux_rate(::std::uint64_t mux_rate) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Export_Consumer with_delay(::moq::duration delay) &&;
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
		static ::moq::result<::moq::mux_container_ts_Passthrough, ::moq::mux_Error> new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Passthrough with_pcr_pid(::std::uint16_t pid) &&;
		RUST2_FFI_CALLABLE ::moq::buf track() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Passthrough_Extra {
		RUST2_FFI_UNCONSUMED mux_container_ts_Passthrough_Extra(mux_container_ts_Passthrough_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Passthrough_Extra &operator=(mux_container_ts_Passthrough_Extra &&other) noexcept = default;
		static ::moq::result<::moq::mux_container_ts_Passthrough_Extra, ::moq::mux_Error> new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Extra reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Passthrough_Extra with_pcr_pid(::std::uint16_t pid) &&;
		RUST2_FFI_CALLABLE ::moq::buf track() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Passthrough_Ext {
		RUST2_FFI_UNCONSUMED mux_container_ts_Passthrough_Ext(mux_container_ts_Passthrough_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Passthrough_Ext &operator=(mux_container_ts_Passthrough_Ext &&other) noexcept = default;
		static ::moq::result<::moq::mux_container_ts_Passthrough_Ext, ::moq::mux_Error> new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Ext reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Passthrough_Ext with_pcr_pid(::std::uint16_t pid) &&;
		RUST2_FFI_CALLABLE ::moq::buf track() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::mux_Error> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Programs {
		RUST2_FFI_UNCONSUMED mux_container_ts_Programs(mux_container_ts_Programs &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Programs &operator=(mux_container_ts_Programs &&other) noexcept = default;
		static ::moq::mux_container_ts_Programs new_(::moq::origin_Producer origin, ::moq::Path name, ::moq::mux_catalog_Config config);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats() const;
		RUST2_FFI_CALLABLE ::moq::result<::std::vector<::moq::broadcast_Producer>, ::moq::buf> finish();
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Release {
		RUST2_FFI_UNCONSUMED mux_container_ts_Release(mux_container_ts_Release &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Release &operator=(mux_container_ts_Release &&other) noexcept = default;
		static ::moq::mux_container_ts_Release_new_future new_(::moq::mux_Source source, ::moq::mux_catalog_CatalogFormat catalog_format);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Release with_delay(::moq::duration delay) &&;
		RUST2_FFI_CALLABLE ::std::uint64_t dropped() const;
		RUST2_FFI_CALLABLE ::std::optional<double> drift() const;
		RUST2_FFI_CALLABLE ::std::uint64_t out_of_tolerance() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Release stats() const;
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_Release_next_future next();
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Import {
		RUST2_FFI_UNCONSUMED mux_container_ts_Import(mux_container_ts_Import &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Import &operator=(mux_container_ts_Import &&other) noexcept = default;
		static ::moq::mux_container_ts_Import new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Import with_program(::std::uint16_t program) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Import with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> finish();
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Import_Extra {
		RUST2_FFI_UNCONSUMED mux_container_ts_Import_Extra(mux_container_ts_Import_Extra &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Import_Extra &operator=(mux_container_ts_Import_Extra &&other) noexcept = default;
		static ::moq::mux_container_ts_Import_Extra new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Extra reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Import_Extra with_program(::std::uint16_t program) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Import_Extra with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> finish();
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats() const;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE mux_container_ts_Import_Ext {
		RUST2_FFI_UNCONSUMED mux_container_ts_Import_Ext(mux_container_ts_Import_Ext &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		mux_container_ts_Import_Ext &operator=(mux_container_ts_Import_Ext &&other) noexcept = default;
		static ::moq::mux_container_ts_Import_Ext new_(::moq::broadcast_Producer broadcast, ::moq::mux_catalog_Reserved_Ext reserved);
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Import_Ext with_program(::std::uint16_t program) &&;
		RUST2_FFI_CONSUMES ::moq::mux_container_ts_Import_Ext with_container(::moq::hang_catalog_Container container) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> decode(::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> seek(::std::uint64_t sequence);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> finish();
		RUST2_FFI_CALLABLE ::moq::mux_container_ts_stats_Snapshot stats() const;
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
	struct mux_select_Selected {
		::std::vector<::moq::buf> video;
		::std::vector<::moq::buf> audio;
		friend bool operator==(const mux_select_Selected &a, const mux_select_Selected &b);
		friend bool operator!=(const mux_select_Selected &a, const mux_select_Selected &b);
		::moq::buf debug() const;
		static mux_select_Selected default_();
		static ::moq::mux_select_Selected of_Extra(const ::moq::hang_Catalog_Extra &catalog);
		static ::moq::mux_select_Selected of_Ext(const ::moq::hang_Catalog_Ext &catalog);
		static ::moq::mux_select_Selected of_unit(const ::moq::hang_Catalog &catalog);
	};
	class mux_timeline_Transition {
		friend bool operator==(const mux_timeline_Transition &a, const mux_timeline_Transition &b);
		friend bool operator!=(const mux_timeline_Transition &a, const mux_timeline_Transition &b);
		::moq::buf debug() const;
	};
	struct mux_timeline_Audio {
		::moq::duration end;
		::moq::duration buffered;
		::moq::duration target;
		::moq::buf debug() const;
	};
	struct mux_codec_aac_Config {
		::std::uint8_t profile;
		::std::uint32_t sample_rate;
		::std::uint32_t channel_count;
		static ::moq::result<::moq::mux_codec_aac_Config, ::moq::mux_codec_aac_Error> parse(::moq::buf &buf_);
		::moq::result<::moq::buf, ::moq::mux_codec_aac_Error> encode() const;
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
	struct mux_container_mp4_Finish {
		::std::uint64_t moov_offset;
		::moq::buf moov;
		::std::uint64_t mdat_offset;
		::moq::buf mdat;
		::moq::buf debug() const;
	};
	struct mux_container_ts_Descriptor {
		::std::uint8_t tag;
		::moq::buf data;
		friend bool operator==(const mux_container_ts_Descriptor &a, const mux_container_ts_Descriptor &b);
		friend bool operator!=(const mux_container_ts_Descriptor &a, const mux_container_ts_Descriptor &b);
		::moq::buf debug() const;
	};
	class mux_container_fmp4_fragment_Config {
		::moq::buf debug() const;
		static mux_container_fmp4_fragment_Config default_();
		::moq::mux_container_fmp4_fragment_MissingDuration missing_duration_get() const;
		void missing_duration_set(::moq::mux_container_fmp4_fragment_MissingDuration value);
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
	inline bool operator==(const mux_container_TimestampRewind &a, const mux_container_TimestampRewind &b);
	inline bool operator==(const mux_select_Selected &a, const mux_select_Selected &b);
	inline bool operator==(const mux_timeline_Transition &a, const mux_timeline_Transition &b);
	inline bool operator==(const mux_codec_flac_Config &a, const mux_codec_flac_Config &b);
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
	inline bool operator==(::moq::mux_timeline_Kind a, ::moq::mux_timeline_Kind b);
	inline bool operator!=(::moq::mux_timeline_Kind a, ::moq::mux_timeline_Kind b);
	inline ::moq::buf mux_timeline_Kind_debug(::moq::mux_timeline_Kind self);
	inline bool operator==(::moq::mux_timeline_Error a, ::moq::mux_timeline_Error b);
	inline bool operator!=(::moq::mux_timeline_Error a, ::moq::mux_timeline_Error b);
	inline ::moq::buf mux_timeline_Error_to_string(::moq::mux_timeline_Error self);
	inline ::moq::buf mux_timeline_Error_debug(::moq::mux_timeline_Error self);
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
	inline bool operator==(::moq::mux_codec_aac_adts_Error a, ::moq::mux_codec_aac_adts_Error b);
	inline bool operator!=(::moq::mux_codec_aac_adts_Error a, ::moq::mux_codec_aac_adts_Error b);
	inline ::moq::buf mux_codec_aac_adts_Error_to_string(::moq::mux_codec_aac_adts_Error self);
	inline ::moq::buf mux_codec_aac_adts_Error_debug(::moq::mux_codec_aac_adts_Error self);
	inline ::moq::buf mux_container_fmp4_fragment_MissingDuration_debug(::moq::mux_container_fmp4_fragment_MissingDuration self);
	inline mux_container_fmp4_fragment_MissingDuration mux_container_fmp4_fragment_MissingDuration_default();
	inline bool operator==(::moq::mux_container_ts_stats_Class a, ::moq::mux_container_ts_stats_Class b);
	inline bool operator!=(::moq::mux_container_ts_stats_Class a, ::moq::mux_container_ts_stats_Class b);
	inline ::moq::buf mux_container_ts_stats_Class_debug(::moq::mux_container_ts_stats_Class self);
	inline mux_container_ts_stats_Class mux_container_ts_stats_Class_default();
	inline ::moq::result<::moq::hang_catalog_AudioConfig, ::moq::mux_Error> mux_codec_aac_config(::moq::input init);
	inline ::moq::buf mux_codec_annexb_START_CODE();
	inline ::moq::result<::moq::buf, ::moq::mux_codec_annexb_Error> mux_codec_annexb_from_length_prefixed(::moq::input payload, ::std::size_t length_size, ::std::optional<::moq::input> prefix);
	inline ::moq::buf mux_codec_annexb_build_prefix(const ::moq::items<::moq::view> &nals);
	inline ::moq::result<void, ::moq::buf> mux_codec_annexb_length_prefixed_to_annexb(::moq::input data, ::std::size_t length_size, ::moq::buf &out);
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
	enum class InvalidHop {
		range = 0,
		too_many = 1,
		duplicate = 2,
	};
	enum class auth_Answer {
		granted = 0,
		requests = 1,
	};
	enum class auth_RequestKind {
		subscribe = 0,
		publish_namespace = 1,
		discover = 2,
		publish = 3,
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
		lan = 4,
		expired = 5,
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
	enum class track_Order {
		ascending = 0,
		descending = 1,
	};
	enum class origin_Link {
		local = 0,
		peer = 1,
		upstream = 2,
	};
	inline constexpr ::std::size_t Path_MAX_PARTS = MOQ_PATH_MAX_PARTS;
	inline constexpr ::std::size_t Path_MAX_BYTES = MOQ_PATH_MAX_BYTES;
	inline constexpr ::std::uint64_t setup_Token_OUT_OF_BAND = MOQ_SETUP_TOKEN_OUT_OF_BAND;
	inline constexpr ::std::uint64_t transport_MIN_RECEIVE_WINDOW = MOQ_TRANSPORT_MIN_RECEIVE_WINDOW;
	inline constexpr ::std::uint64_t transport_SETUP_RECEIVE_WINDOW = MOQ_TRANSPORT_SETUP_RECEIVE_WINDOW;
	inline constexpr ::std::uint64_t group_MAX_CACHE_BYTES = MOQ_GROUP_MAX_CACHE_BYTES;
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
		RUST2_FFI_CALLABLE ::std::uint64_t hash() const;
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static Path default_();
		static ::moq::result<::moq::Path, ::moq::path_Error> new_(::moq::input s);
		static ::moq::Path from_static(::moq::input s);
		RUST2_FFI_CALLABLE bool has_prefix(const ::moq::Path &prefix) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Path> strip_prefix(const ::moq::Path &prefix) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Path> head(::std::size_t n) const;
		RUST2_FFI_CALLABLE ::std::vector<::moq::buf> parts() const;
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
		RUST2_FFI_CONSUMES ::moq::Server with_stats(::moq::stats_Session stats) &&;
		RUST2_FFI_CONSUMES ::moq::Server with_limits(const ::moq::session_Limits &limits) &&;
		RUST2_FFI_CONSUMES ::moq::Server with_versions(::moq::Versions versions) &&;
		RUST2_FFI_CONSUMES ::moq::Server with_extensions(const ::moq::setup_Extensions &extensions) &&;
		RUST2_FFI_CONSUMES ::moq::Server with_implementation(::moq::input implementation) &&;
		RUST2_FFI_CALLABLE ::moq::Server_accept_Session_future accept_Session(const ::moq::clock_Instant &now, ::moq::transport_memory_Session session) const;
		RUST2_FFI_CALLABLE ::moq::Server_accept_request_Session_future accept_request_Session(const ::moq::clock_Instant &now, ::moq::transport_memory_Session session) const;
	};
	class RUST2_FFI_CONSUMABLE Session {
		RUST2_FFI_UNCONSUMED Session(Session &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Session &operator=(Session &&other) noexcept = default;
		RUST2_FFI_CALLABLE Session clone() const;
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
		RUST2_FFI_CALLABLE ::moq::goaway_Producer drain() const;
		RUST2_FFI_CALLABLE ::moq::goaway_Consumer draining() const;
		RUST2_FFI_CALLABLE ::moq::Session_idle_future idle() const;
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
		RUST2_FFI_CONSUMES ::moq::Client with_origin(::moq::origin_Producer origin) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_limits(const ::moq::session_Limits &limits) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_allocator(::moq::bandwidth_Allocator allocator) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_viewers(::moq::bandwidth_Viewers viewers) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_versions(::moq::Versions versions) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_url(::moq::Url url) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_token(const ::moq::setup_Token &token) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_auth(::moq::auth_Handle auth) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_extensions(const ::moq::setup_Extensions &extensions) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_implementation(::moq::input implementation) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_cost(const ::moq::origin_Cost &cost) &&;
		RUST2_FFI_CONSUMES ::moq::Client with_peer_hop(const ::moq::Hop &hop) &&;
		RUST2_FFI_CALLABLE ::moq::Client_connect_Session_future connect_Session(const ::moq::clock_Instant &now, ::moq::transport_memory_Session session) const;
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
		RUST2_FFI_CALLABLE bool has_setup_token() const;
		RUST2_FFI_CALLABLE ::moq::buf as_str() const;
		static ::std::optional<::moq::Version> from_code(::std::uint64_t code);
		RUST2_FFI_CALLABLE ::std::uint64_t code() const;
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
	class RUST2_FFI_CONSUMABLE Hops {
		RUST2_FFI_UNCONSUMED Hops(Hops &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Hops &operator=(Hops &&other) noexcept = default;
		RUST2_FFI_CALLABLE Hops clone() const;
		friend bool operator==(const Hops &a, const Hops &b);
		friend bool operator!=(const Hops &a, const Hops &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static Hops default_();
		static ::moq::Hops new_();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::InvalidHop> push(const ::moq::Hop &hop);
		RUST2_FFI_CALLABLE bool contains(const ::moq::Hop &hop) const;
		RUST2_FFI_CALLABLE ::std::size_t len() const;
		RUST2_FFI_CALLABLE bool is_empty() const;
		RUST2_FFI_CALLABLE ::std::vector<::moq::Hop> iter() const;
		RUST2_FFI_CALLABLE ::std::vector<::moq::Hop> as_slice() const;
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
		RUST2_FFI_CALLABLE ::std::optional<::moq::auth_RequestKind> kind() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Path> path() const;
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
		RUST2_FFI_CALLABLE ::moq::auth_Handle_add_future add(::moq::input token) const;
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
	};
	class RUST2_FFI_CONSUMABLE auth_Dial {
		RUST2_FFI_UNCONSUMED auth_Dial(auth_Dial &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		auth_Dial &operator=(auth_Dial &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::Url url_get() const;
		RUST2_FFI_CALLABLE void url_set(::moq::Url value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::setup_Token> token_get() const;
		RUST2_FFI_CALLABLE void token_set(const ::std::optional<::moq::setup_Token> &value);
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
		RUST2_FFI_CALLABLE ::moq::auth_Dial dial(const ::moq::Url &url, const ::moq::Versions &versions) const;
		RUST2_FFI_CALLABLE void connected(const ::moq::Session &session) const;
		RUST2_FFI_CALLABLE bool changed(const ::moq::auth_Dial &dial) const;
	};
	class RUST2_FFI_CONSUMABLE goaway_Producer {
		RUST2_FFI_UNCONSUMED goaway_Producer(goaway_Producer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		goaway_Producer &operator=(goaway_Producer &&other) noexcept = default;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::Error> send(::moq::goaway_Goaway goaway) &&;
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
		RUST2_FFI_CONSUMES ::moq::goaway_Goaway with_timeout(::moq::duration timeout) &&;
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
	class RUST2_FFI_CONSUMABLE server_Handshake {
		RUST2_FFI_UNCONSUMED server_Handshake(server_Handshake &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		server_Handshake &operator=(server_Handshake &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf path() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::setup_Role> role() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Hop> peer_hop() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> peer_implementation() const;
		RUST2_FFI_CALLABLE ::moq::auth_Handle auth() const;
		RUST2_FFI_CALLABLE ::moq::auth_Requests requests();
		RUST2_FFI_CALLABLE ::std::optional<::moq::setup_Token> token() const;
		RUST2_FFI_CONSUMES ::moq::server_Handshake with_publisher_Producer(const ::moq::origin_Producer &publish) &&;
		RUST2_FFI_CONSUMES ::moq::server_Handshake with_publisher_Consumer(const ::moq::origin_Consumer &publish) &&;
		RUST2_FFI_CONSUMES ::moq::server_Handshake with_subscriber(::moq::origin_Producer subscribe) &&;
		RUST2_FFI_CONSUMES ::moq::server_Handshake with_peer_hop(const ::moq::Hop &hop) &&;
		RUST2_FFI_CONSUMES ::moq::server_Handshake with_stats(::moq::stats_Session stats) &&;
		RUST2_FFI_CONSUMES ::moq::server_Handshake with_limits(const ::moq::session_Limits &limits) &&;
		RUST2_FFI_CONSUMES ::moq::server_Handshake with_deadline(const ::moq::clock_Instant &at) &&;
		RUST2_FFI_CONSUMES ::moq::server_Handshake_ok_future ok() &&;
		RUST2_FFI_CONSUMES void close(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE server_Activity {
		RUST2_FFI_UNCONSUMED server_Activity(server_Activity &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		server_Activity &operator=(server_Activity &&other) noexcept = default;
		RUST2_FFI_CALLABLE server_Activity clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static server_Activity default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::server_Active> enter() const;
		RUST2_FFI_CALLABLE ::moq::server_Activity_close_future close() const;
	};
	class RUST2_FFI_CONSUMABLE server_Active {
		RUST2_FFI_UNCONSUMED server_Active(server_Active &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		server_Active &operator=(server_Active &&other) noexcept = default;
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
		RUST2_FFI_CALLABLE void report(::moq::stats_Report &report) const;
	};
	class RUST2_FFI_CONSUMABLE stats_Handle {
		RUST2_FFI_UNCONSUMED stats_Handle(stats_Handle &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		stats_Handle &operator=(stats_Handle &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED stats_Handle(const stats_Handle &other) = default;
		stats_Handle &operator=(const stats_Handle &other) = default;
		static stats_Handle default_();
		RUST2_FFI_CALLABLE void refuse(::moq::Path label, ::moq::stats_RefusalKind reason) const;
		RUST2_FFI_CALLABLE ::moq::stats_Session session(::moq::Path label) const;
		RUST2_FFI_CALLABLE ::moq::stats_Session request(::moq::Path label) const;
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
		RUST2_FFI_CALLABLE ::moq::duration cache_duration_get() const;
		RUST2_FFI_CALLABLE void cache_duration_set(::moq::duration value);
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
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> remove(::moq::input name) const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> finish(::moq::input name) const;
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
		RUST2_FFI_CALLABLE ::moq::result<::std::uint64_t, ::moq::Error> append(const ::std::optional<::moq::Timestamp> &timestamp, ::moq::input payload);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> insert(const ::moq::Datagram &datagram);
		RUST2_FFI_CALLABLE ::moq::datagram_Consumer consume() const;
		RUST2_FFI_CONSUMES void finish() &&;
		RUST2_FFI_CONSUMES void abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::std::optional<::std::int8_t> priority() const;
		RUST2_FFI_CALLABLE ::moq::datagram_Producer_poll_priority_future poll_priority(const ::std::optional<::std::int8_t> &last) const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::datagram_Producer &other) const;
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
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::datagram_Consumer &other) const;
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
		RUST2_FFI_CALLABLE ::moq::result<::std::optional<::moq::Timestamp>, ::moq::Error> rescale(const ::std::optional<::moq::Timestamp> &timestamp) const;
		RUST2_FFI_CALLABLE ::moq::datagram_Producer produce(::moq::input name) const;
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
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> timestamp_get() const;
	};
	class RUST2_FFI_CONSUMABLE frame_Frame {
		RUST2_FFI_UNCONSUMED frame_Frame(frame_Frame &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		frame_Frame &operator=(frame_Frame &&other) noexcept = default;
		RUST2_FFI_CALLABLE frame_Frame clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> timestamp_get() const;
		RUST2_FFI_CALLABLE void timestamp_set(const ::std::optional<::moq::Timestamp> &value);
		RUST2_FFI_CALLABLE ::moq::buf payload_get() const;
		RUST2_FFI_CALLABLE void payload_set(::moq::input value);
		static ::moq::frame_Frame new_(const ::std::optional<::moq::Timestamp> &timestamp, ::moq::input payload);
		RUST2_FFI_CALLABLE ::moq::clock_Instant arrived() const;
	};
	class RUST2_FFI_CONSUMABLE frame_Buffer {
		RUST2_FFI_UNCONSUMED frame_Buffer(frame_Buffer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		frame_Buffer &operator=(frame_Buffer &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static frame_Buffer default_();
		static ::moq::frame_Buffer new_();
		RUST2_FFI_CALLABLE ::std::size_t capacity() const;
		RUST2_FFI_CALLABLE ::std::size_t len() const;
		RUST2_FFI_CALLABLE bool is_empty() const;
		RUST2_FFI_CALLABLE ::std::vector<::moq::frame_Frame> filled() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::frame_Frame> push(::moq::frame_Frame frame);
		RUST2_FFI_CALLABLE void clear();
	};
	class RUST2_FFI_CONSUMABLE frame_Consumer {
		RUST2_FFI_UNCONSUMED frame_Consumer(frame_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		frame_Consumer &operator=(frame_Consumer &&other) noexcept = default;
		RUST2_FFI_CALLABLE frame_Consumer clone() const;
		RUST2_FFI_CALLABLE ::moq::frame_Consumer_read_chunk_future read_chunk();
		RUST2_FFI_CALLABLE ::moq::frame_Consumer_read_all_future read_all();
		RUST2_FFI_CALLABLE ::std::uint64_t size_get() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> timestamp_get() const;
	};
	class RUST2_FFI_CONSUMABLE group_Producer {
		RUST2_FFI_UNCONSUMED group_Producer(group_Producer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		group_Producer &operator=(group_Producer &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timescale> timescale() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> start_at(::std::uint64_t index);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> write_frame(const ::std::optional<::moq::Timestamp> &timestamp, ::moq::input data);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> write_frames_8(::moq::frame_Buffer &frames);
		RUST2_FFI_CONSUMES ::moq::result<::moq::frame_Producer, ::moq::Error> create_frame(const ::moq::frame_Info &frame) &&;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> finish();
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::Error> abort(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::group_Producer_closed_future closed() const;
		RUST2_FFI_CALLABLE ::moq::group_Producer_ready_future ready(const ::moq::frame_Info &frame) const;
		RUST2_FFI_CALLABLE ::std::size_t frame_count() const;
		RUST2_FFI_CALLABLE ::moq::group_Consumer consume() const;
		RUST2_FFI_CALLABLE ::moq::group_Demand demand() const;
		RUST2_FFI_CALLABLE ::std::uint64_t sequence_get() const;
	};
	class RUST2_FFI_CONSUMABLE group_Demand {
		RUST2_FFI_UNCONSUMED group_Demand(group_Demand &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		group_Demand &operator=(group_Demand &&other) noexcept = default;
		RUST2_FFI_CALLABLE group_Demand clone() const;
		RUST2_FFI_CALLABLE ::std::uint64_t sequence() const;
		RUST2_FFI_CALLABLE bool wanted() const;
		RUST2_FFI_CALLABLE ::moq::group_Demand_wanted_changed_future wanted_changed();
		RUST2_FFI_CALLABLE ::moq::group_Demand_closed_future closed() const;
	};
	class RUST2_FFI_CONSUMABLE group_Consumer {
		RUST2_FFI_UNCONSUMED group_Consumer(group_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		group_Consumer &operator=(group_Consumer &&other) noexcept = default;
		RUST2_FFI_CALLABLE group_Consumer clone() const;
		RUST2_FFI_CALLABLE void keep_alive() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timescale> timescale() const;
		RUST2_FFI_CALLABLE ::moq::clock_Instant arrived() const;
		RUST2_FFI_CALLABLE ::std::uint64_t index() const;
		RUST2_FFI_CONSUMES ::moq::group_Consumer with_range(const ::moq::bounds_u64 &frames) &&;
		RUST2_FFI_CALLABLE void skip_to(::std::uint64_t index);
		RUST2_FFI_CALLABLE ::std::size_t frame_count() const;
		RUST2_FFI_CALLABLE ::moq::group_Consumer_next_frame_future next_frame();
		RUST2_FFI_CALLABLE ::moq::group_Consumer_read_frame_future read_frame();
		RUST2_FFI_CALLABLE ::moq::group_Consumer_poll_read_frames_8_future poll_read_frames_8(::moq::frame_Buffer &out);
		RUST2_FFI_CALLABLE ::moq::group_Consumer_read_frames_8_future read_frames_8(::moq::frame_Buffer &out);
		RUST2_FFI_CALLABLE ::moq::group_Consumer_finished_future finished();
		RUST2_FFI_CALLABLE ::std::uint64_t sequence_get() const;
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
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> wanted() const;
		RUST2_FFI_CALLABLE ::moq::track_Demand_wanted_changed_future wanted_changed();
		RUST2_FFI_CALLABLE ::moq::track_Demand_closed_future closed() const;
	};
	class RUST2_FFI_CONSUMABLE track_Subscriber {
		RUST2_FFI_UNCONSUMED track_Subscriber(track_Subscriber &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Subscriber &operator=(track_Subscriber &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Control control() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber_recv_group_future recv_group();
		RUST2_FFI_CONSUMES ::moq::track_Ordered ordered() &&;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Subscriber &other) const;
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> floor() const;
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> next_in_range(::std::uint64_t from) const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber_finished_future finished();
		RUST2_FFI_CONSUMES ::moq::track_Subscriber with_range(const ::moq::bounds_u64 &groups) &&;
		RUST2_FFI_CALLABLE ::moq::track_Subscription subscription() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> update(::moq::track_Subscription subscription);
		RUST2_FFI_CALLABLE ::moq::track_Subscriber_largest_future largest();
	};
	class RUST2_FFI_CONSUMABLE track_Ordered {
		RUST2_FFI_UNCONSUMED track_Ordered(track_Ordered &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Ordered &operator=(track_Ordered &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Ordered_next_group_future next_group();
		RUST2_FFI_CONSUMES ::moq::track_Ordered with_range(const ::moq::bounds_u64 &groups) &&;
		RUST2_FFI_CALLABLE ::moq::track_Control control() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscription subscription() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> update(::moq::track_Subscription subscription);
		RUST2_FFI_CALLABLE ::moq::track_Ordered_finished_future finished();
		RUST2_FFI_CALLABLE ::moq::track_Ordered_largest_future largest();
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Ordered &other) const;
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
		RUST2_FFI_CALLABLE bool compress_get() const;
		RUST2_FFI_CALLABLE void compress_set(bool value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint32_t> refresh_get() const;
		RUST2_FFI_CALLABLE void refresh_set(const ::std::optional<::std::uint32_t> &value);
		RUST2_FFI_CONSUMES ::moq::track_Info with_timescale(const ::std::optional<::moq::Timescale> &timescale) &&;
		RUST2_FFI_CONSUMES ::moq::track_Info with_max_age(const ::std::optional<::moq::duration> &max_age) &&;
		RUST2_FFI_CONSUMES ::moq::track_Info with_priority(::std::int8_t priority) &&;
		RUST2_FFI_CONSUMES ::moq::track_Info with_compress(bool compress) &&;
		RUST2_FFI_CONSUMES ::moq::track_Info with_refresh(const ::std::optional<::std::uint32_t> &refresh) &&;
		RUST2_FFI_CALLABLE ::moq::result<::std::optional<::moq::Timestamp>, ::moq::Error> rescale(const ::std::optional<::moq::Timestamp> &timestamp) const;
	};
	class RUST2_FFI_CONSUMABLE track_Producer {
		RUST2_FFI_UNCONSUMED track_Producer(track_Producer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Producer &operator=(track_Producer &&other) noexcept = default;
		RUST2_FFI_CALLABLE track_Producer clone() const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Info info() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> create_group(const ::moq::group_Info &group) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> append_group() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> write_frame(const ::std::optional<::moq::Timestamp> &timestamp, ::moq::input frame);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> finish() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> finish_at(const ::moq::track_Position &end, ::std::optional<::moq::Error> error);
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> start_at(const ::std::optional<::std::uint64_t> &sequence);
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups, const ::moq::StreamError &err) const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> finished() const;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::Error> abort(::moq::Error err) &&;
		RUST2_FFI_CONSUMES ::moq::result<void, ::moq::track_Producer> abort_unwanted(::moq::Error err) &&;
		RUST2_FFI_CALLABLE ::moq::track_Producer_closed_future closed() const;
		RUST2_FFI_CALLABLE bool is_closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Position> largest() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::track_Producer &other) const;
		RUST2_FFI_CALLABLE ::moq::track_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::track_Consumer consume() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscriber subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::moq::track_Producer_subscription_changed_future subscription_changed();
		RUST2_FFI_CALLABLE ::std::optional<::moq::track_Subscription> subscription() const;
		RUST2_FFI_CALLABLE ::moq::track_Requests requests() const;
	};
	class RUST2_FFI_CONSUMABLE track_Consumer {
		RUST2_FFI_UNCONSUMED track_Consumer(track_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Consumer &operator=(track_Consumer &&other) noexcept = default;
		RUST2_FFI_CALLABLE track_Consumer clone() const;
		RUST2_FFI_CALLABLE ::moq::buf name() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info broadcast() const;
		RUST2_FFI_CALLABLE ::moq::track_Consumer_subscribe_future subscribe(::std::optional<::moq::track_Subscription> subscription) const;
		RUST2_FFI_CALLABLE ::moq::track_Consumer_fetch_group_future fetch_group(::std::uint64_t sequence, ::std::optional<::moq::group_Fetch> options) const;
		RUST2_FFI_CALLABLE ::moq::track_Consumer_info_future info() const;
		RUST2_FFI_CALLABLE ::moq::track_Consumer_largest_future largest();
	};
	class RUST2_FFI_CONSUMABLE track_Control {
		RUST2_FFI_UNCONSUMED track_Control(track_Control &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Control &operator=(track_Control &&other) noexcept = default;
		RUST2_FFI_CALLABLE track_Control clone() const;
		RUST2_FFI_CALLABLE ::moq::track_Subscription subscription() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> update(::moq::track_Subscription subscription) const;
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
		RUST2_FFI_CONSUMES ::moq::track_Subscription with_range(const ::moq::bounds_u64 &groups) &&;
		RUST2_FFI_CONSUMES ::moq::track_Subscription with_order(::moq::track_Order order) &&;
	};
	class RUST2_FFI_CONSUMABLE track_Fill {
		RUST2_FFI_UNCONSUMED track_Fill(track_Fill &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		track_Fill &operator=(track_Fill &&other) noexcept = default;
		RUST2_FFI_CALLABLE track_Fill clone() const;
		RUST2_FFI_CALLABLE ::moq::track_Position start() const;
		RUST2_FFI_CALLABLE ::std::uint64_t frame_start(::std::uint64_t sequence) const;
		RUST2_FFI_CALLABLE ::std::uint64_t end() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> time() const;
		RUST2_FFI_CALLABLE ::std::int8_t priority() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> max_delay() const;
		RUST2_FFI_CALLABLE ::moq::track_Order order() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::group_Producer, ::moq::Error> accept(::std::uint64_t sequence, ::std::optional<::moq::track_Info> info) const;
		RUST2_FFI_CALLABLE void drop_groups(const ::moq::range_u64 &groups) const;
		RUST2_FFI_CALLABLE void refuse_groups(const ::moq::range_u64 &groups, ::moq::Error err) const;
		RUST2_FFI_CALLABLE bool is_wanted(::std::uint64_t sequence) const;
		RUST2_FFI_CALLABLE ::moq::track_Fill_unwanted_future unwanted(::std::uint64_t sequence) const;
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> wanted() const;
		RUST2_FFI_CALLABLE ::moq::track_Fill_wanted_changed_future wanted_changed();
	};
	class RUST2_FFI_CONSUMABLE origin_Following {
		RUST2_FFI_UNCONSUMED origin_Following(origin_Following &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Following &operator=(origin_Following &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::origin_Following_available_future available();
	};
	class RUST2_FFI_CONSUMABLE origin_Config {
		RUST2_FFI_UNCONSUMED origin_Config(origin_Config &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Config &operator=(origin_Config &&other) noexcept = default;
		RUST2_FFI_CALLABLE origin_Config clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static origin_Config default_();
		RUST2_FFI_CALLABLE ::moq::Hop hop_get() const;
		RUST2_FFI_CALLABLE void hop_set(const ::moq::Hop &value);
		RUST2_FFI_CALLABLE ::moq::origin_HopLimit max_hops_get() const;
		RUST2_FFI_CALLABLE void max_hops_set(const ::moq::origin_HopLimit &value);
		RUST2_FFI_CALLABLE ::moq::cache_Pool pool_get() const;
		RUST2_FFI_CALLABLE void pool_set(::moq::cache_Pool value);
		RUST2_FFI_CALLABLE ::moq::duration cache_duration_get() const;
		RUST2_FFI_CALLABLE void cache_duration_set(::moq::duration value);
		RUST2_FFI_CALLABLE ::moq::duration update_hold_get() const;
		RUST2_FFI_CALLABLE void update_hold_set(::moq::duration value);
		static ::moq::origin_Config new_(const ::moq::Hop &hop);
	};
	class RUST2_FFI_CONSUMABLE origin_Consumer {
		RUST2_FFI_UNCONSUMED origin_Consumer(origin_Consumer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Consumer &operator=(origin_Consumer &&other) noexcept = default;
		RUST2_FFI_CALLABLE origin_Consumer clone() const;
		RUST2_FFI_CALLABLE ::moq::Hop hop() const;
		RUST2_FFI_CONSUMES ::moq::origin_Consumer local() &&;
		RUST2_FFI_CONSUMES ::moq::origin_Consumer with_hidden(bool hidden) &&;
		RUST2_FFI_CONSUMES ::moq::origin_Consumer covering() &&;
		RUST2_FFI_CALLABLE ::moq::result<::moq::origin_Consumer, ::moq::Error> exclude(::moq::input prefix) const;
		RUST2_FFI_CONSUMES ::moq::origin_Consumer with_stats(::moq::stats_Session session) &&;
		RUST2_FFI_CALLABLE ::moq::announce_Consumer announced() const;
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
	class RUST2_FFI_CONSUMABLE origin_Prepared {
		RUST2_FFI_UNCONSUMED origin_Prepared(origin_Prepared &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Prepared &operator=(origin_Prepared &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::broadcast_Producer broadcast() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::origin_Publication, ::moq::Error> publish() &&;
	};
	class RUST2_FFI_CONSUMABLE origin_Producer {
		RUST2_FFI_UNCONSUMED origin_Producer(origin_Producer &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Producer &operator=(origin_Producer &&other) noexcept = default;
		RUST2_FFI_CALLABLE origin_Producer clone() const;
		static ::std::tuple<::moq::origin_Producer, ::moq::origin_Driver> new_(::moq::origin_Config config);
		RUST2_FFI_CONSUMES ::moq::origin_Producer with_stats(::moq::stats_Session session) &&;
		RUST2_FFI_CONSUMES ::moq::origin_Producer with_link(::moq::origin_Link link) &&;
		RUST2_FFI_CONSUMES ::moq::origin_Producer with_preference(const ::moq::origin_Preference &preference) &&;
		RUST2_FFI_CALLABLE ::moq::origin_Config config() const;
		RUST2_FFI_CALLABLE ::moq::Hop hop() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::origin_Publication, ::moq::Error> create_broadcast(::moq::input path) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::origin_Publication, ::moq::Error> publish(::moq::input path, ::moq::origin_Route route) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::origin_Requests, ::moq::Error> requests(::moq::input prefix) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::origin_Producer, ::moq::Error> scope(::moq::input root, const ::moq::Patterns &patterns) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::origin_Producer, ::moq::Error> mount(::moq::input at, ::moq::input target) const;
		RUST2_FFI_CALLABLE ::moq::origin_Consumer consume() const;
		RUST2_FFI_CALLABLE ::moq::Path root() const;
		RUST2_FFI_CALLABLE ::moq::Patterns allowed() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::Path, ::moq::Error> absolute(::moq::input path) const;
	};
	class RUST2_FFI_CONSUMABLE origin_Publication {
		RUST2_FFI_UNCONSUMED origin_Publication(origin_Publication &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Publication &operator=(origin_Publication &&other) noexcept = default;
		RUST2_FFI_CALLABLE origin_Publication clone() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Producer broadcast() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::Error> announce(::moq::origin_Route route) const;
		RUST2_FFI_CALLABLE void unannounce() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::origin_Route> route() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Info info() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Demand demand() const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::track_Producer, ::moq::Error> create_track(::moq::input name, ::std::optional<::moq::track_Info> info) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::datagram_Producer, ::moq::Error> create_datagrams(::moq::input name, const ::moq::datagram_Info &info) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::track_Request, ::moq::Error> reserve_track(::moq::input name) const;
		RUST2_FFI_CALLABLE ::moq::result<::moq::track_Producer, ::moq::Error> unique_track(::moq::input suffix, ::std::optional<::moq::track_Info> info) const;
		RUST2_FFI_CALLABLE ::moq::buf unique_name(::moq::input suffix) const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Requests requests() const;
		RUST2_FFI_CALLABLE ::moq::broadcast_Consumer consume() const;
		RUST2_FFI_CALLABLE void close() const;
		RUST2_FFI_CALLABLE bool is_clone(const ::moq::broadcast_Producer &other) const;
	};
	class RUST2_FFI_CONSUMABLE origin_Request {
		RUST2_FFI_UNCONSUMED origin_Request(origin_Request &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Request &operator=(origin_Request &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::Path path() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::origin_Prepared, ::moq::Error> prepare() &&;
		RUST2_FFI_CONSUMES void serve_Producer(const ::moq::broadcast_Producer &broadcast) &&;
		RUST2_FFI_CONSUMES void serve_Consumer(const ::moq::broadcast_Consumer &broadcast) &&;
		RUST2_FFI_CONSUMES void reject(::moq::Error err) &&;
	};
	class RUST2_FFI_CONSUMABLE origin_Requests {
		RUST2_FFI_UNCONSUMED origin_Requests(origin_Requests &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Requests &operator=(origin_Requests &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::origin_Route route() const;
		RUST2_FFI_CALLABLE ::moq::origin_Requests_requested_broadcast_future requested_broadcast() const;
	};
	class RUST2_FFI_CONSUMABLE origin_Route {
		RUST2_FFI_UNCONSUMED origin_Route(origin_Route &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		origin_Route &operator=(origin_Route &&other) noexcept = default;
		RUST2_FFI_CALLABLE origin_Route clone() const;
		friend bool operator==(const origin_Route &a, const origin_Route &b);
		friend bool operator!=(const origin_Route &a, const origin_Route &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static origin_Route default_();
		RUST2_FFI_CALLABLE ::std::optional<::moq::Instance> instance_get() const;
		RUST2_FFI_CALLABLE void instance_set(::std::optional<::moq::Instance> value);
		RUST2_FFI_CALLABLE ::moq::Hops hops_get() const;
		RUST2_FFI_CALLABLE void hops_set(::moq::Hops value);
		RUST2_FFI_CALLABLE ::moq::origin_Cost cost_get() const;
		RUST2_FFI_CALLABLE void cost_set(const ::moq::origin_Cost &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::auth_Credential> auth_get() const;
		RUST2_FFI_CALLABLE void auth_set(::std::optional<::moq::auth_Credential> value);
		RUST2_FFI_CONSUMES ::moq::origin_Route with_instance(::moq::Instance instance, ::std::uint64_t epoch) &&;
		RUST2_FFI_CONSUMES ::moq::origin_Route with_hops(::moq::Hops hops) &&;
		RUST2_FFI_CONSUMES ::moq::origin_Route with_cost(const ::moq::origin_Cost &cost) &&;
		RUST2_FFI_CONSUMES ::moq::origin_Route with_auth(::moq::auth_Credential auth) &&;
		RUST2_FFI_CALLABLE bool is_anonymous() const;
		RUST2_FFI_CALLABLE ::moq::origin_Source source() const;
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
	};
	class RUST2_FFI_CONSUMABLE transport_memory_Session {
		RUST2_FFI_UNCONSUMED transport_memory_Session(transport_memory_Session &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		transport_memory_Session &operator=(transport_memory_Session &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED transport_memory_Session(const transport_memory_Session &other) = default;
		transport_memory_Session &operator=(const transport_memory_Session &other) = default;
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
		::moq::buf debug() const;
		static ::moq::Timestamp ZERO();
		static ::moq::result<::moq::Timestamp, ::moq::TimeOverflow> new_(::std::uint64_t value, const ::moq::Timescale &scale);
		static ::moq::result<::moq::Timestamp, ::moq::TimeOverflow> from_scale(::std::uint64_t value, ::std::uint64_t units_per_second);
		static ::moq::result<::moq::Timestamp, ::moq::TimeOverflow> from_secs(::std::uint64_t seconds);
		static ::moq::result<::moq::Timestamp, ::moq::TimeOverflow> from_millis(::std::uint64_t millis);
		static ::moq::result<::moq::Timestamp, ::moq::TimeOverflow> from_micros(::std::uint64_t micros);
		static ::moq::result<::moq::Timestamp, ::moq::TimeOverflow> from_nanos(::std::uint64_t nanos);
		::std::uint64_t value() const;
		::moq::Timescale scale() const;
		bool is_zero() const;
		::moq::result<::moq::Timestamp, ::moq::TimeOverflow> convert(const ::moq::Timescale &new_scale) const;
		::moq::u128 as_scale(const ::moq::Timescale &target) const;
		::std::uint64_t as_secs() const;
		::moq::u128 as_millis() const;
		::moq::u128 as_micros() const;
		::moq::u128 as_nanos() const;
		::moq::result<::moq::Timestamp, ::moq::TimeOverflow> checked_add(const ::moq::Timestamp &rhs) const;
		::moq::result<::moq::Timestamp, ::moq::TimeOverflow> checked_sub(const ::moq::Timestamp &rhs) const;
		static ::moq::Timestamp now();
	};
	struct Datagram {
		::std::uint64_t sequence;
		::std::optional<::moq::Timestamp> timestamp;
		::moq::buf payload;
		friend bool operator==(const Datagram &a, const Datagram &b);
		friend bool operator!=(const Datagram &a, const Datagram &b);
		::moq::buf debug() const;
	};
	class Hop {
		friend bool operator==(const Hop &a, const Hop &b);
		friend bool operator!=(const Hop &a, const Hop &b);
		::moq::buf to_string() const;
		::moq::buf debug() const;
		static ::moq::Hop UNKNOWN();
		static ::moq::result<::moq::Hop, ::moq::InvalidHop> new_(::std::uint64_t id);
		static ::moq::Hop random();
		::std::uint64_t id() const;
	};
	class setup_Extensions {
		friend bool operator==(const setup_Extensions &a, const setup_Extensions &b);
		friend bool operator!=(const setup_Extensions &a, const setup_Extensions &b);
		::moq::buf debug() const;
		static setup_Extensions default_();
		bool auth_get() const;
		void auth_set(bool value);
		bool solicit_get() const;
		void solicit_set(bool value);
		bool ietf_get() const;
		void ietf_set(bool value);
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
		void add(const ::moq::stats_Presence &other);
		::std::uint64_t active() const;
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
		::std::uint64_t lan_get() const;
		void lan_set(::std::uint64_t value);
		::std::uint64_t expired_get() const;
		void expired_set(::std::uint64_t value);
		::std::uint64_t count(::moq::stats_RefusalKind kind) const;
		void add(const ::moq::stats_Refusals &other);
		bool regressed(const ::moq::stats_Refusals &previous) const;
	};
	class transport_ReceiveWindow {
		friend bool operator==(const transport_ReceiveWindow &a, const transport_ReceiveWindow &b);
		friend bool operator!=(const transport_ReceiveWindow &a, const transport_ReceiveWindow &b);
		::moq::buf debug() const;
		::std::uint64_t connection_get() const;
		void connection_set(::std::uint64_t value);
		::std::uint64_t stream_get() const;
		void stream_set(::std::uint64_t value);
		static ::moq::transport_ReceiveWindow new_(::std::uint64_t connection, ::std::uint64_t stream);
	};
	class bandwidth_Rate {
		friend bool operator==(const bandwidth_Rate &a, const bandwidth_Rate &b);
		friend bool operator!=(const bandwidth_Rate &a, const bandwidth_Rate &b);
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
	struct frame_Info {
		::std::uint64_t size;
		::std::optional<::moq::Timestamp> timestamp;
		::moq::buf debug() const;
	};
	struct group_Info {
		::std::uint64_t sequence;
		friend bool operator==(const group_Info &a, const group_Info &b);
		friend bool operator!=(const group_Info &a, const group_Info &b);
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
	struct track_Position {
		::std::uint64_t group;
		::std::uint64_t frame;
		friend bool operator==(const track_Position &a, const track_Position &b);
		friend bool operator!=(const track_Position &a, const track_Position &b);
		::moq::buf debug() const;
		static track_Position default_();
		static ::moq::track_Position group_(::std::uint64_t group_);
		static ::std::optional<::moq::track_Position> after(::std::uint64_t group_, ::std::uint64_t frame_);
		static ::std::optional<::moq::track_Position> after_group(::std::uint64_t group_);
		::moq::bound_u64 group_end() const;
	};
	struct track_Anchor_Absolute {
		::std::uint64_t _0;
	};
	struct track_Anchor_Relative {
		::std::uint64_t _0;
	};
	struct track_Anchor_Time {
		::moq::Timestamp _0;
	};
	struct track_Range {
		::moq::track_Anchor start;
		::std::optional<::moq::track_Anchor> end;
		friend bool operator==(const track_Range &a, const track_Range &b);
		friend bool operator!=(const track_Range &a, const track_Range &b);
		::moq::buf debug() const;
		static ::moq::track_Range live();
		static ::moq::track_Range oldest();
		static ::moq::track_Range groups(const ::moq::bounds_u64 &groups);
	};
	class origin_Cost {
		friend bool operator==(const origin_Cost &a, const origin_Cost &b);
		friend bool operator!=(const origin_Cost &a, const origin_Cost &b);
		::moq::buf debug() const;
		static origin_Cost default_();
		static ::moq::origin_Cost new_(::std::uint64_t cost);
		::std::uint64_t value() const;
		static ::moq::origin_Cost MAX();
		static ::moq::origin_Cost DRAIN();
	};
	class origin_HopLimit {
		friend bool operator==(const origin_HopLimit &a, const origin_HopLimit &b);
		friend bool operator!=(const origin_HopLimit &a, const origin_HopLimit &b);
		::moq::buf debug() const;
		static origin_HopLimit default_();
		static ::std::optional<::moq::origin_HopLimit> new_(::std::size_t max);
		::std::size_t get() const;
	};
	class origin_Preference {
		friend bool operator==(const origin_Preference &a, const origin_Preference &b);
		friend bool operator!=(const origin_Preference &a, const origin_Preference &b);
		::moq::buf debug() const;
		static origin_Preference default_();
		static ::moq::origin_Preference new_(::std::int8_t preference);
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
	struct SessionError_KeyValueFormatting {
	};
	struct SessionError_TooManyRequests {
	};
	struct SessionError_GoawayTimeout {
	};
	struct SessionError_Timeout {
	};
	struct SessionError_AuthTokenCacheOverflow {
	};
	struct SessionError_Version {
	};
	struct SessionError_MalformedAuthToken {
	};
	struct SessionError_UnknownAuthTokenAlias {
	};
	struct SessionError_TooManyRequestUpdates {
	};
	struct SessionError_App {
		::std::uint16_t _0;
	};
	struct SessionError_Unknown {
		::std::uint32_t _0;
	};
	struct SessionError_Unsupported {
	};
	struct SessionError_InvalidRequestId {
	};
	struct SessionError_InvalidPath {
	};
	struct SessionError_InvalidAuthority {
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
	struct origin_Source_Local {
	};
	struct origin_Source_Peer {
		::moq::Hop _0;
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
	inline bool operator==(const Timestamp &a, const Timestamp &b);
	inline bool operator==(const Datagram &a, const Datagram &b);
	inline bool operator==(const Hop &a, const Hop &b);
	inline bool operator==(const setup_Extensions &a, const setup_Extensions &b);
	inline bool operator==(const setup_Token &a, const setup_Token &b);
	inline bool operator==(const session_Limits &a, const session_Limits &b);
	inline bool operator==(const stats_Content &a, const stats_Content &b);
	inline bool operator==(const stats_Traffic &a, const stats_Traffic &b);
	inline bool operator==(const stats_Presence &a, const stats_Presence &b);
	inline bool operator==(const stats_Totals &a, const stats_Totals &b);
	inline bool operator==(const stats_Failures &a, const stats_Failures &b);
	inline bool operator==(const stats_Refusals &a, const stats_Refusals &b);
	inline bool operator==(const transport_ReceiveWindow &a, const transport_ReceiveWindow &b);
	inline bool operator==(const bandwidth_Rate &a, const bandwidth_Rate &b);
	inline bool operator==(const group_Info &a, const group_Info &b);
	inline bool operator==(const track_Delay &a, const track_Delay &b);
	inline bool operator==(const track_Position &a, const track_Position &b);
	inline bool operator==(const ::moq::track_Anchor &a, const ::moq::track_Anchor &b);
	inline bool operator!=(const ::moq::track_Anchor &a, const ::moq::track_Anchor &b);
	inline ::moq::buf track_Anchor_debug(const ::moq::track_Anchor &self);
	inline bool operator==(const track_Range &a, const track_Range &b);
	inline bool operator==(const origin_Cost &a, const origin_Cost &b);
	inline bool operator==(const origin_HopLimit &a, const origin_HopLimit &b);
	inline bool operator==(const origin_Preference &a, const origin_Preference &b);
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
	inline bool operator==(::moq::InvalidHop a, ::moq::InvalidHop b);
	inline bool operator!=(::moq::InvalidHop a, ::moq::InvalidHop b);
	inline ::moq::buf InvalidHop_to_string(::moq::InvalidHop self);
	inline ::moq::buf InvalidHop_debug(::moq::InvalidHop self);
	inline bool operator==(::moq::auth_Answer a, ::moq::auth_Answer b);
	inline bool operator!=(::moq::auth_Answer a, ::moq::auth_Answer b);
	inline ::moq::buf auth_Answer_debug(::moq::auth_Answer self);
	inline bool operator==(::moq::auth_RequestKind a, ::moq::auth_RequestKind b);
	inline bool operator!=(::moq::auth_RequestKind a, ::moq::auth_RequestKind b);
	inline ::moq::buf auth_RequestKind_debug(::moq::auth_RequestKind self);
	inline bool operator==(::moq::path_Error a, ::moq::path_Error b);
	inline bool operator!=(::moq::path_Error a, ::moq::path_Error b);
	inline ::moq::buf path_Error_to_string(::moq::path_Error self);
	inline ::moq::buf path_Error_debug(::moq::path_Error self);
	inline bool operator==(::moq::setup_Role a, ::moq::setup_Role b);
	inline bool operator!=(::moq::setup_Role a, ::moq::setup_Role b);
	inline ::moq::buf setup_Role_to_string(::moq::setup_Role self);
	inline ::moq::buf setup_Role_debug(::moq::setup_Role self);
	inline ::moq::buf setup_Role_as_str(::moq::setup_Role self);
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
	inline bool operator==(::moq::transport_Address a, ::moq::transport_Address b);
	inline bool operator!=(::moq::transport_Address a, ::moq::transport_Address b);
	inline ::moq::buf transport_Address_debug(::moq::transport_Address self);
	inline bool operator==(::moq::transport_CloseKind a, ::moq::transport_CloseKind b);
	inline bool operator!=(::moq::transport_CloseKind a, ::moq::transport_CloseKind b);
	inline ::moq::buf transport_CloseKind_debug(::moq::transport_CloseKind self);
	inline transport_CloseKind transport_CloseKind_default();
	inline bool operator==(::moq::track_Order a, ::moq::track_Order b);
	inline bool operator!=(::moq::track_Order a, ::moq::track_Order b);
	inline ::moq::buf track_Order_debug(::moq::track_Order self);
	inline track_Order track_Order_default();
	inline bool operator==(::moq::origin_Link a, ::moq::origin_Link b);
	inline bool operator!=(::moq::origin_Link a, ::moq::origin_Link b);
	inline ::moq::buf origin_Link_debug(::moq::origin_Link self);
	inline origin_Link origin_Link_default();
	inline bool operator==(const ::moq::origin_Source &a, const ::moq::origin_Source &b);
	inline bool operator!=(const ::moq::origin_Source &a, const ::moq::origin_Source &b);
	inline ::moq::buf origin_Source_debug(const ::moq::origin_Source &self);
	inline origin_Source origin_Source_default();
	inline ::moq::buf transport_memory_Error_to_string(const ::moq::transport_memory_Error &self);
	inline ::moq::buf transport_memory_Error_debug(const ::moq::transport_memory_Error &self);
	inline ::moq::buf IMPLEMENTATION();
	inline ::std::vector<::moq::buf> ALPNS();
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

// quic.hpp
#ifndef MOQ_HPP_CRATE14
#define MOQ_HPP_CRATE14
#include "rt.hpp"
namespace moq {
	enum class moq_quic_Side {
		client = 0,
		server = 1,
	};
	enum class moq_quic_stream_Dir {
		bi = 0,
		uni = 1,
	};
	inline constexpr ::std::size_t moq_quic_VarInt_MAX_SIZE = MOQ_MOQ_QUIC_VARINT_MAX_SIZE;
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
	class RUST2_FFI_CONSUMABLE moq_quic_connection_Stats {
		RUST2_FFI_UNCONSUMED moq_quic_connection_Stats(moq_quic_connection_Stats &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_quic_connection_Stats &operator=(moq_quic_connection_Stats &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_quic_connection_Stats clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_quic_connection_Stats default_();
		RUST2_FFI_CALLABLE ::moq::moq_quic_UdpStats udp_tx_get() const;
		RUST2_FFI_CALLABLE void udp_tx_set(const ::moq::moq_quic_UdpStats &value);
		RUST2_FFI_CALLABLE ::moq::moq_quic_UdpStats udp_rx_get() const;
		RUST2_FFI_CALLABLE void udp_rx_set(const ::moq::moq_quic_UdpStats &value);
		RUST2_FFI_CALLABLE ::moq::moq_quic_FrameStats frame_tx_get() const;
		RUST2_FFI_CALLABLE void frame_tx_set(const ::moq::moq_quic_FrameStats &value);
		RUST2_FFI_CALLABLE ::moq::moq_quic_FrameStats frame_rx_get() const;
		RUST2_FFI_CALLABLE void frame_rx_set(const ::moq::moq_quic_FrameStats &value);
		RUST2_FFI_CALLABLE ::moq::moq_quic_PathStats path_get() const;
		RUST2_FFI_CALLABLE void path_set(const ::moq::moq_quic_PathStats &value);
		RUST2_FFI_CALLABLE ::moq::moq_quic_CreditStats credit_get() const;
		RUST2_FFI_CALLABLE void credit_set(const ::moq::moq_quic_CreditStats &value);
	};
	class RUST2_FFI_CONSUMABLE moq_quic_crypto_HandshakeData {
		RUST2_FFI_UNCONSUMED moq_quic_crypto_HandshakeData(moq_quic_crypto_HandshakeData &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_quic_crypto_HandshakeData &operator=(moq_quic_crypto_HandshakeData &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_quic_crypto_HandshakeData clone() const;
		friend bool operator==(const moq_quic_crypto_HandshakeData &a, const moq_quic_crypto_HandshakeData &b);
		friend bool operator!=(const moq_quic_crypto_HandshakeData &a, const moq_quic_crypto_HandshakeData &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> alpn_get() const;
		RUST2_FFI_CALLABLE void alpn_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> server_name_get() const;
		RUST2_FFI_CALLABLE void server_name_set(::std::optional<::moq::input> value);
	};
	class RUST2_FFI_CONSUMABLE moq_quic_FrameStats {
		RUST2_FFI_UNCONSUMED moq_quic_FrameStats(moq_quic_FrameStats &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_quic_FrameStats &operator=(moq_quic_FrameStats &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_quic_FrameStats clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_quic_FrameStats default_();
		RUST2_FFI_CALLABLE ::std::uint64_t acks_get() const;
		RUST2_FFI_CALLABLE void acks_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t ack_frequency_get() const;
		RUST2_FFI_CALLABLE void ack_frequency_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t crypto_get() const;
		RUST2_FFI_CALLABLE void crypto_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t connection_close_get() const;
		RUST2_FFI_CALLABLE void connection_close_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t data_blocked_get() const;
		RUST2_FFI_CALLABLE void data_blocked_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t datagram_get() const;
		RUST2_FFI_CALLABLE void datagram_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint8_t handshake_done_get() const;
		RUST2_FFI_CALLABLE void handshake_done_set(::std::uint8_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t immediate_ack_get() const;
		RUST2_FFI_CALLABLE void immediate_ack_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t max_data_get() const;
		RUST2_FFI_CALLABLE void max_data_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t max_stream_data_get() const;
		RUST2_FFI_CALLABLE void max_stream_data_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t max_streams_bidi_get() const;
		RUST2_FFI_CALLABLE void max_streams_bidi_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t max_streams_uni_get() const;
		RUST2_FFI_CALLABLE void max_streams_uni_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t new_connection_id_get() const;
		RUST2_FFI_CALLABLE void new_connection_id_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t new_token_get() const;
		RUST2_FFI_CALLABLE void new_token_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t path_challenge_get() const;
		RUST2_FFI_CALLABLE void path_challenge_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t path_response_get() const;
		RUST2_FFI_CALLABLE void path_response_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t ping_get() const;
		RUST2_FFI_CALLABLE void ping_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t reset_stream_get() const;
		RUST2_FFI_CALLABLE void reset_stream_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t retire_connection_id_get() const;
		RUST2_FFI_CALLABLE void retire_connection_id_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t stream_data_blocked_get() const;
		RUST2_FFI_CALLABLE void stream_data_blocked_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t streams_blocked_bidi_get() const;
		RUST2_FFI_CALLABLE void streams_blocked_bidi_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t streams_blocked_uni_get() const;
		RUST2_FFI_CALLABLE void streams_blocked_uni_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t stop_sending_get() const;
		RUST2_FFI_CALLABLE void stop_sending_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t stream_get() const;
		RUST2_FFI_CALLABLE void stream_set(::std::uint64_t value);
	};
	class RUST2_FFI_CONSUMABLE moq_quic_PathStats {
		RUST2_FFI_UNCONSUMED moq_quic_PathStats(moq_quic_PathStats &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_quic_PathStats &operator=(moq_quic_PathStats &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_quic_PathStats clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_quic_PathStats default_();
		RUST2_FFI_CALLABLE ::moq::duration rtt_get() const;
		RUST2_FFI_CALLABLE void rtt_set(::moq::duration value);
		RUST2_FFI_CALLABLE ::moq::duration min_rtt_get() const;
		RUST2_FFI_CALLABLE void min_rtt_set(::moq::duration value);
		RUST2_FFI_CALLABLE ::std::uint64_t cwnd_get() const;
		RUST2_FFI_CALLABLE void cwnd_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::optional<::std::uint64_t> bandwidth_estimate_get() const;
		RUST2_FFI_CALLABLE void bandwidth_estimate_set(const ::std::optional<::std::uint64_t> &value);
		RUST2_FFI_CALLABLE ::std::uint64_t congestion_events_get() const;
		RUST2_FFI_CALLABLE void congestion_events_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t spurious_congestion_events_get() const;
		RUST2_FFI_CALLABLE void spurious_congestion_events_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t lost_packets_get() const;
		RUST2_FFI_CALLABLE void lost_packets_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t lost_bytes_get() const;
		RUST2_FFI_CALLABLE void lost_bytes_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t sent_packets_get() const;
		RUST2_FFI_CALLABLE void sent_packets_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t sent_plpmtud_probes_get() const;
		RUST2_FFI_CALLABLE void sent_plpmtud_probes_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t lost_plpmtud_probes_get() const;
		RUST2_FFI_CALLABLE void lost_plpmtud_probes_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t sent_bandwidth_probes_get() const;
		RUST2_FFI_CALLABLE void sent_bandwidth_probes_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t sent_bandwidth_probe_bytes_get() const;
		RUST2_FFI_CALLABLE void sent_bandwidth_probe_bytes_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint64_t black_holes_detected_get() const;
		RUST2_FFI_CALLABLE void black_holes_detected_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::uint16_t current_mtu_get() const;
		RUST2_FFI_CALLABLE void current_mtu_set(::std::uint16_t value);
	};
	class RUST2_FFI_CONSUMABLE moq_quic_Error {
		RUST2_FFI_UNCONSUMED moq_quic_Error(moq_quic_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_quic_Error &operator=(moq_quic_Error &&other) noexcept = default;
	};
	class moq_quic_VarInt {
		friend bool operator==(const moq_quic_VarInt &a, const moq_quic_VarInt &b);
		friend bool operator!=(const moq_quic_VarInt &a, const moq_quic_VarInt &b);
		::moq::buf to_string() const;
		::moq::buf debug() const;
		static moq_quic_VarInt default_();
		static ::moq::moq_quic_VarInt MAX();
		static ::moq::moq_quic_VarInt from_u32(::std::uint32_t x);
		static ::moq::result<::moq::moq_quic_VarInt, ::moq::buf> from_u64(::std::uint64_t x);
		::std::uint64_t into_inner() const;
		void encode(::moq::buf &output) const;
	};
	class moq_quic_stream_Id {
		friend bool operator==(const moq_quic_stream_Id &a, const moq_quic_stream_Id &b);
		friend bool operator!=(const moq_quic_stream_Id &a, const moq_quic_stream_Id &b);
		::moq::buf to_string() const;
		::moq::buf debug() const;
		static ::moq::moq_quic_stream_Id new_(::moq::moq_quic_Side initiator, ::moq::moq_quic_stream_Dir dir, ::std::uint64_t index);
		::moq::moq_quic_Side initiator() const;
		::moq::moq_quic_stream_Dir dir() const;
		::std::uint64_t index() const;
	};
	class moq_quic_UdpStats {
		::moq::buf debug() const;
		static moq_quic_UdpStats default_();
		::std::uint64_t datagrams_get() const;
		void datagrams_set(::std::uint64_t value);
		::std::uint64_t bytes_get() const;
		void bytes_set(::std::uint64_t value);
		::std::uint64_t ios_get() const;
		void ios_set(::std::uint64_t value);
	};
	class moq_quic_CreditStats {
		friend bool operator==(const moq_quic_CreditStats &a, const moq_quic_CreditStats &b);
		friend bool operator!=(const moq_quic_CreditStats &a, const moq_quic_CreditStats &b);
		::moq::buf debug() const;
		static moq_quic_CreditStats default_();
		::std::uint64_t leased_get() const;
		void leased_set(::std::uint64_t value);
		::std::uint64_t available_get() const;
		void available_set(::std::uint64_t value);
		::std::uint64_t evicted_get() const;
		void evicted_set(::std::uint64_t value);
		::std::uint64_t peak_spent_get() const;
		void peak_spent_set(::std::uint64_t value);
	};
	struct moq_quic_Rate_Unlimited {
	};
	struct moq_quic_Rate_BytesPerSecond {
		::std::uint64_t _0;
	};
	inline bool operator==(const moq_quic_VarInt &a, const moq_quic_VarInt &b);
	inline bool operator==(const moq_quic_stream_Id &a, const moq_quic_stream_Id &b);
	inline bool operator==(const moq_quic_CreditStats &a, const moq_quic_CreditStats &b);
	inline bool operator==(const ::moq::moq_quic_Rate &a, const ::moq::moq_quic_Rate &b);
	inline bool operator!=(const ::moq::moq_quic_Rate &a, const ::moq::moq_quic_Rate &b);
	inline ::moq::buf moq_quic_Rate_debug(const ::moq::moq_quic_Rate &self);
	inline bool operator==(::moq::moq_quic_Side a, ::moq::moq_quic_Side b);
	inline bool operator!=(::moq::moq_quic_Side a, ::moq::moq_quic_Side b);
	inline ::moq::buf moq_quic_Side_debug(::moq::moq_quic_Side self);
	inline bool moq_quic_Side_is_client(::moq::moq_quic_Side self);
	inline bool moq_quic_Side_is_server(::moq::moq_quic_Side self);
	inline bool operator==(::moq::moq_quic_stream_Dir a, ::moq::moq_quic_stream_Dir b);
	inline bool operator!=(::moq::moq_quic_stream_Dir a, ::moq::moq_quic_stream_Dir b);
	inline ::moq::buf moq_quic_stream_Dir_to_string(::moq::moq_quic_stream_Dir self);
	inline ::moq::buf moq_quic_stream_Dir_debug(::moq::moq_quic_stream_Dir self);
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
namespace moq {
	#if defined(__clang__)
	#define RUST2_FFI_LIFETIMEBOUND [[clang::lifetimebound]]
	#define RUST2_FFI_CONSUMABLE [[clang::consumable(unconsumed)]]
	#define RUST2_FFI_CALLABLE [[clang::callable_when("unconsumed", "unknown")]]
	#define RUST2_FFI_CONSUMES [[clang::callable_when("unconsumed", "unknown"), clang::set_typestate(consumed)]]
	#define RUST2_FFI_UNCONSUMED [[clang::return_typestate(unconsumed)]]
	#define RUST2_FFI_MOVED_FROM [[clang::return_typestate(consumed)]]
	#else
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
	class view {
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
	class buf {
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
	template <class T> class items {
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
	template <class T, class E> class result {
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
	template <class E> class result<void, E> {
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
	class task {
		task() noexcept = default;
		task(task &&other) noexcept : node_(::std::exchange(other.node_, nullptr));
		task &operator=(task &&other) noexcept;
		~task();
		template <class Op, class Exec, class F> friend void detail::spawn(task &, typename Op::raw *, Exec, F);
	};
	template <class Op> class future {
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
	enum class InvalidHop;
	enum class IntersectionError;
	enum class clock_MissedTickBehavior;
	enum class tokio_ConnectError;
	enum class tokio_Restart;
	enum class tokio_Status;
	enum class tokio_Transport;
	enum class auth_Answer;
	enum class auth_RequestKind;
	enum class path_Error;
	enum class setup_Role;
	enum class stats_Role;
	enum class stats_Message;
	enum class stats_Failure;
	enum class stats_RefusalKind;
	enum class transport_Address;
	enum class transport_CloseKind;
	enum class track_Order;
	enum class origin_Link;
	enum class hang_health_Verdict;
	enum class hang_health_Role;
	enum class hang_health_Reset;
	enum class hang_health_Input;
	enum class hang_catalog_AudioCodecKind;
	enum class hang_catalog_VideoCodecKind;
	enum class hang_catalog_Range;
	enum class mux_catalog_CatalogFormat;
	enum class mux_container_Kind;
	enum class mux_import_AudioFormat;
	enum class mux_import_ContainerFormat;
	enum class mux_import_Kind;
	enum class mux_import_VideoFormat;
	enum class mux_timeline_Kind;
	enum class mux_timeline_Error;
	enum class mux_codec_flac_Error;
	enum class mux_codec_mp3_Error;
	enum class mux_codec_vp8_Error;
	enum class mux_codec_vp9_Error;
	enum class mux_container_ts_Framing;
	enum class mux_codec_aac_adts_Error;
	enum class mux_container_fmp4_fragment_MissingDuration;
	enum class mux_container_ts_stats_Class;
	enum class moq_tokio_quic_CongestionControl;
	enum class moq_tokio_connection_Phase;
	enum class moq_tokio_accept_Failure;
	enum class moq_tokio_Redirect;
	enum class mp4_atom_SencBoxVersion;
	enum class rustls_client_Tls12Resumption;
	enum class moq_sock_demux_Discard;
	enum class mp4_atom_ClockType;
	enum class moq_sock_udp_EcnCodepoint;
	enum class moq_quic_Side;
	enum class moq_quic_stream_Dir;
	enum class mp4_atom_ReferenceTime;
	enum class mp4_atom_VexuProjKind;
	enum class mp4_atom_AudioChannelPosition;
	enum class Level;
	class TimeOverflow;
	class Timescale;
	class Timestamp;
	struct Datagram;
	class Hop;
	class clock_Instant;
	class clock_Elapsed;
	class clock_Wall;
	class setup_Extensions;
	struct setup_Token;
	class session_Limits;
	class stats_Content;
	class stats_Traffic;
	class stats_Presence;
	class stats_Totals;
	class stats_Failures;
	class stats_Refusals;
	class transport_ReceiveWindow;
	class bandwidth_Rate;
	struct frame_Info;
	struct group_Info;
	struct track_Delay;
	struct track_Position;
	struct track_Range;
	class origin_Cost;
	class origin_HopLimit;
	class origin_Preference;
	struct hang_health_Limits_f64;
	struct hang_health_Limits_duration;
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
	struct mux_select_Selected;
	class mux_timeline_Transition;
	struct mux_timeline_Audio;
	struct mux_codec_aac_Config;
	struct mux_codec_flac_Config;
	struct mux_codec_h264_Sps;
	struct mux_codec_mp3_Config;
	class mux_codec_opus_Mapping;
	struct mux_container_fmp4_Fragment;
	struct mux_container_legacy_Wire;
	struct mux_container_loc_Wire;
	struct mux_container_mp4_Finish;
	struct mux_container_ts_Descriptor;
	class mux_container_fmp4_fragment_Config;
	class mp4_atom_FourCC;
	struct mp4_atom_Mp4a;
	struct mp4_atom_Mp3;
	struct mp4_atom_Opus;
	struct mp4_atom_Flac;
	struct mp4_atom_Ac3;
	struct mp4_atom_Eac3;
	struct mp4_atom_Ipcm;
	struct mp4_atom_Fpcm;
	struct mp4_atom_Sowt;
	struct mp4_atom_Twos;
	struct mp4_atom_Lpcm;
	struct mp4_atom_In24;
	struct mp4_atom_In32;
	struct mp4_atom_Fl32;
	struct mp4_atom_Fl64;
	struct mp4_atom_S16l;
	struct mp4_atom_Wvtt;
	struct mp4_atom_Samr;
	struct mp4_atom_Mett;
	struct mp4_atom_Metx;
	struct mp4_atom_Urim;
	struct mp4_atom_Camm;
	struct mp4_atom_Evte;
	struct mp4_atom_Mebx;
	struct mp4_atom_Tkhd;
	struct mp4_atom_Edts;
	struct mp4_atom_Senc;
	struct mp4_atom_Tref;
	struct mp4_atom_Audio;
	struct mp4_atom_MetaData;
	struct rustls_client_TicketRequest;
	class moq_quic_VarInt;
	struct mp4_atom_Avcc;
	struct mp4_atom_Btrt;
	struct mp4_atom_Pasp;
	struct mp4_atom_Taic;
	struct mp4_atom_Fiel;
	struct mp4_atom_Hvcc;
	struct mp4_atom_Lhvc;
	struct mp4_atom_Ccst;
	struct mp4_atom_VpcC;
	struct mp4_atom_Av1c;
	struct mp4_atom_Esds;
	struct mp4_atom_RgbaColor;
	struct mp4_atom_Ftab;
	struct mp4_atom_Disp;
	struct mp4_atom_Dops;
	struct mp4_atom_Cmpd;
	struct mp4_atom_Dfla;
	struct mp4_atom_Ac3SpecificBox;
	struct mp4_atom_Ec3SpecificBox;
	struct mp4_atom_PcmC;
	struct mp4_atom_Chnl;
	struct mp4_atom_PlainText;
	struct mp4_atom_VttC;
	struct mp4_atom_Vlab;
	struct mp4_atom_AmrSampleEntry;
	struct mp4_atom_Damr;
	struct mp4_atom_TxtC;
	struct mp4_atom_Uri;
	struct mp4_atom_UriI;
	struct mp4_atom_Silb;
	struct mp4_atom_MebxKey;
	class mp4_atom_FixedPoint_i16;
	class mp4_atom_FixedPoint_i8;
	class mp4_atom_FixedPoint_u16;
	class mp4_atom_FixedPoint_u8;
	struct mp4_atom_Matrix;
	struct mp4_atom_Elst;
	struct mp4_atom_Hdlr;
	struct mp4_atom_Mdhd;
	struct mp4_atom_TrackReferenceTypeBox;
	struct mp4_atom_Chpl;
	struct mp4_atom_Cprt;
	struct mp4_atom_Kind;
	struct mp4_atom_Rtng;
	struct mp4_atom_PcmFormat;
	class moq_quic_stream_Id;
	class moq_quic_UdpStats;
	class moq_quic_CreditStats;
	struct mp4_atom_AvccExt;
	struct mp4_atom_St3d;
	struct mp4_atom_Hfov;
	struct mp4_atom_HvcCArray;
	struct mp4_atom_esds_EsDescriptor;
	struct mp4_atom_FontEntry;
	struct mp4_atom_OpusChannelMapping;
	struct mp4_atom_Component;
	struct mp4_atom_UncompressedComponent;
	struct mp4_atom_Ec3IndependentSubstream;
	struct mp4_atom_SchemeIdEntry;
	struct mp4_atom_Keyd;
	struct mp4_atom_Dtyp;
	struct mp4_atom_Loca;
	struct mp4_atom_Setu;
	struct mp4_atom_ElstEntry;
	struct mp4_atom_Ftyp;
	struct mp4_atom_Styp;
	struct mp4_atom_Pitm;
	struct mp4_atom_Iloc;
	struct mp4_atom_Iinf;
	struct mp4_atom_Auxc;
	struct mp4_atom_Clap;
	struct mp4_atom_Imir;
	struct mp4_atom_Irot;
	struct mp4_atom_Iscl;
	struct mp4_atom_Ispe;
	struct mp4_atom_Pixi;
	struct mp4_atom_Rref;
	struct mp4_atom_Ipma;
	struct mp4_atom_Iref;
	struct mp4_atom_Idat;
	struct mp4_atom_Ilst;
	struct mp4_atom_Covr;
	struct mp4_atom_Desc;
	struct mp4_atom_Tool;
	struct mp4_atom_Name;
	struct mp4_atom_Year;
	struct mp4_atom_Keys;
	struct mp4_atom_Mvhd;
	struct mp4_atom_Ainf;
	struct mp4_atom_Skip;
	struct mp4_atom_Stts;
	struct mp4_atom_Stsc;
	struct mp4_atom_Stsz;
	struct mp4_atom_Stss;
	struct mp4_atom_Stco;
	struct mp4_atom_Co64;
	struct mp4_atom_Cslg;
	struct mp4_atom_Ctts;
	struct mp4_atom_Sbgp;
	struct mp4_atom_Sgpd;
	struct mp4_atom_Subs;
	struct mp4_atom_Saio;
	struct mp4_atom_Saiz;
	struct mp4_atom_Dinf;
	struct mp4_atom_Dref;
	struct mp4_atom_Hmhd;
	struct mp4_atom_Smhd;
	struct mp4_atom_Vmhd;
	struct mp4_atom_Mvex;
	struct mp4_atom_Mehd;
	struct mp4_atom_Trex;
	struct mp4_atom_Emsg;
	struct mp4_atom_Mfhd;
	struct mp4_atom_Tfhd;
	struct mp4_atom_Tfdt;
	struct mp4_atom_Trun;
	struct mp4_atom_Mdat;
	struct mp4_atom_Free;
	struct mp4_atom_Sidx;
	struct mp4_atom_Prft;
	struct mp4_atom_Mfra;
	struct mp4_atom_Tfra;
	struct mp4_atom_Mfro;
	struct mp4_atom_ChplEntry;
	struct mp4_atom_Svhd;
	struct mp4_atom_Must;
	struct mp4_atom_esds_DecoderConfig;
	class mp4_atom_esds_SLConfig;
	class mp4_atom_u24;
	struct mp4_atom_ItemLocation;
	struct mp4_atom_ItemInfoEntry;
	struct mp4_atom_PropertyAssociations;
	struct mp4_atom_Reference;
	struct mp4_atom_Copyright;
	struct mp4_atom_IlstData;
	struct mp4_atom_KeyEntry;
	struct mp4_atom_Zeroed;
	struct mp4_atom_SttsEntry;
	struct mp4_atom_StscEntry;
	struct mp4_atom_CttsEntry;
	struct mp4_atom_SbgpEntry;
	struct mp4_atom_SgpdEntry;
	struct mp4_atom_SubsEntry;
	struct mp4_atom_AuxInfo;
	struct mp4_atom_RgbColor;
	struct mp4_atom_SegmentReference;
	struct mp4_atom_FragmentInfo;
	struct mp4_atom_C2pa;
	class mp4_atom_ExtendedType;
	struct mp4_atom_Prhd;
	struct mp4_atom_Prji;
	struct mp4_atom_Stri;
	struct mp4_atom_Hero;
	struct mp4_atom_Pkin;
	struct mp4_atom_esds_DecoderSpecific;
	struct mp4_atom_ExplicitSpeakerPosition;
	struct mp4_atom_ItemLocationExtent;
	struct mp4_atom_PropertyAssociation;
	struct mp4_atom_SubsSubsample;
	struct mp4_atom_Url;
	struct mp4_atom_Urn;
	struct mp4_atom_TrunEntry;
	struct rustls_crypto_hpke_HpkePublicKey;
	struct mp4_atom_Equi;
	struct mp4_atom_Cbmp;
	struct mp4_atom_Mshp;
	struct mp4_atom_Blin;
	struct mp4_atom_Dadj;
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
	class Timed_Json;
	class Hops;
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
	class auth_Dial;
	class auth_Token;
	class auth_Tokens;
	class goaway_Producer;
	class goaway_Goaway;
	class goaway_Consumer;
	class path_Relative;
	class server_Handshake;
	class server_Activity;
	class server_Active;
	class session_Stats;
	class stats_Tier;
	class stats_Snapshot;
	class stats_Report;
	class stats_TrafficEntry;
	class stats_SessionEntry;
	class stats_Config;
	class stats_Registry;
	class stats_Handle;
	class stats_Wire;
	class stats_Session;
	class transport_Wire;
	class transport_Probe;
	class bandwidth_Demand;
	class bandwidth_Reservation;
	class bandwidth_Viewer;
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
	class frame_Buffer;
	class frame_Consumer;
	class group_Producer;
	class group_Demand;
	class group_Consumer;
	class group_Fetch;
	class track_Requests;
	class track_Demand;
	class track_Subscriber;
	class track_Ordered;
	class track_Request;
	class track_Info;
	class track_Producer;
	class track_Consumer;
	class track_Control;
	class track_Subscription;
	class track_Fill;
	class origin_Following;
	class origin_Config;
	class origin_Consumer;
	class origin_Driver;
	class origin_Prepared;
	class origin_Producer;
	class origin_Publication;
	class origin_Request;
	class origin_Requests;
	class origin_Route;
	class announce_Consumer;
	class announce_Event;
	class hang_stats_Snapshot;
	class hang_stats_Snapshot_string;
	class hang_stats_Rendition;
	class hang_stats_Transport;
	class hang_echo_Snapshot;
	class hang_echo_Snapshot_string;
	class hang_echo_Rendition;
	class hang_echo_Arrival;
	class hang_health_Observer;
	class hang_health_Report_echo_Snapshot_string;
	class hang_health_Report_stats_Snapshot_string;
	class hang_health_Thresholds;
	class hang_health_TransportThresholds;
	class hang_health_PublisherThresholds;
	class hang_health_ViewerThresholds;
	class hang_health_Evidence;
	class hang_health_Connection;
	class hang_health_Rendition;
	class hang_catalog_Consumer;
	class hang_catalog_Consumer_Extra;
	class hang_catalog_Consumer_Ext;
	class hang_catalog_Audio;
	class hang_catalog_AudioConfig;
	class hang_catalog_Binary;
	class hang_catalog_BinaryConfig;
	class hang_catalog_Container;
	class hang_catalog_Contribution;
	class hang_catalog_Echo;
	class hang_catalog_Json;
	class hang_catalog_JsonConfig;
	class hang_catalog_M2ts;
	class hang_catalog_Media;
	class hang_catalog_Join;
	class hang_catalog_Stats;
	class hang_catalog_Text;
	class hang_catalog_TextConfig;
	class hang_catalog_Color;
	class hang_catalog_Projection;
	class hang_catalog_Video;
	class hang_catalog_VideoProperties;
	class hang_catalog_VideoConfig;
	class mux_catalog_Consumer;
	class mux_catalog_Consumer_Extra;
	class mux_catalog_Consumer_Ext;
	class mux_catalog_Estimate;
	class mux_catalog_Estimator;
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
	class mux_follow_Event;
	class mux_import_AudioInit;
	class mux_import_ContainerInit;
	class mux_import_VideoInit;
	class mux_import_Container;
	class mux_import_Container_Extra;
	class mux_import_Container_Ext;
	class mux_import_Track;
	class mux_import_TrackStream;
	class mux_json_Snapshot_Json;
	class mux_json_Stream_Json;
	class mux_json_Consumer_Json;
	class mux_json_Config;
	class mux_json_Options;
	class mux_rate_Follower;
	class mux_rate_Policy;
	class mux_rate_Control;
	class mux_select_Broadcast;
	class mux_select_Video;
	class mux_select_Audio;
	class mux_timeline_Track;
	class mux_timeline_Producer;
	class mux_timeline_Consumer;
	class tokio_tls_Error;
	class tokio_tls_Identity;
	class tokio_tls_Peers;
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
	class mux_catalog_hang_Consumer;
	class mux_catalog_hang_Consumer_Extra;
	class mux_catalog_hang_Consumer_Ext;
	class mux_catalog_hang_Container;
	class mux_catalog_hang_Extra;
	class mux_catalog_msf_Consumer;
	class mux_catalog_msf_Consumer_Extra;
	class mux_catalog_msf_Consumer_Ext;
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
	class mux_container_fmp4_Wire;
	class mux_container_fmp4_Error;
	class mux_container_fmp4_Fragmenter;
	class mux_container_fmp4_Import;
	class mux_container_fmp4_Import_Extra;
	class mux_container_fmp4_Import_Ext;
	class mux_container_fmp4_Muxer;
	class mux_container_mkv_Import;
	class mux_container_mkv_Import_Extra;
	class mux_container_mkv_Import_Ext;
	class mux_container_ts_Ext;
	class mux_container_ts_Mpegts;
	class mux_container_ts_PesPacket;
	class mux_container_ts_Program;
	class mux_container_ts_SiEntry;
	class mux_container_ts_Track;
	class mux_container_ts_Verbatim;
	class mux_container_ts_Export_Consumer_Extra;
	class mux_container_ts_Export_Consumer_Ext;
	class mux_container_ts_Export_Consumer;
	class mux_container_ts_Follower;
	class mux_container_ts_Follower_Extra;
	class mux_container_ts_Follower_Ext;
	class mux_container_ts_Passthrough;
	class mux_container_ts_Passthrough_Extra;
	class mux_container_ts_Passthrough_Ext;
	class mux_container_ts_Programs;
	class mux_container_ts_Release;
	class mux_container_ts_Import;
	class mux_container_ts_Import_Extra;
	class mux_container_ts_Import_Ext;
	class mux_codec_aac_adts_Header;
	class mux_container_ts_stats_Report;
	class mux_container_ts_stats_Snapshot;
	class mux_container_ts_stats_Stream;
	class mux_container_ts_stats_Export;
	class mux_container_ts_stats_Release;
	class mux_container_ts_stats_Log;
	class moq_tokio_Addrs;
	class moq_tokio_cli_Deprecated;
	class mp4_atom_Codec;
	class moq_tokio_tcp_TlsAcceptor;
	class moq_tokio_accept_Health;
	class rustls_ClientConfig;
	class rustls_RootCertStore;
	class rustls_pki_types_CertificateDer;
	class moq_tokio_connect_Resolved;
	class moq_tokio_connection_Goaway;
	class moq_tokio_websocket_Config;
	class moq_tokio_connection_Observation;
	class moq_tokio_connection_Snapshot;
	class moq_tokio_listen_Resolved;
	class moq_tokio_tcp_Config;
	class moq_tokio_unix_Config;
	class moq_tokio_listen_LoadBalancer;
	class moq_tokio_listen_Reset;
	class moq_tokio_quic_Resolved;
	class moq_tokio_quic_Controller;
	class moq_tokio_websocket_Listener;
	class moq_tokio_quic_demux_Demux;
	class moq_tokio_quic_driver_connection_Connection;
	class moq_quic_Limits;
	class mp4_atom_Trak;
	class moq_tokio_connect_Addr;
	class mp4_atom_Avc1;
	class mp4_atom_Avc3;
	class mp4_atom_Jpeg;
	class mp4_atom_Hev1;
	class mp4_atom_Hvc1;
	class mp4_atom_Vp08;
	class mp4_atom_Vp09;
	class mp4_atom_Av01;
	class mp4_atom_Tx3g;
	class rustls_pki_types_TrustAnchor;
	class moq_tokio_quic_ServerId;
	class moq_tokio_listen_Forward;
	class mp4_atom_Meta;
	class mp4_atom_Mdia;
	class mp4_atom_Udta;
	class mp4_atom_Visual;
	class rustls_client_danger_DangerousClientConfig;
	class rustls_client_Resumption;
	class rustls_DistinguishedName;
	class moq_tokio_websocket_Resolved;
	class moq_tokio_unix_Allow;
	class moq_tokio_websocket_Error;
	class moq_sock_demux_Counters;
	class moq_tokio_quic_demux_WebRtc;
	class moq_tokio_quic_driver_send_Options;
	class moq_tokio_quic_driver_send_Stream;
	class moq_tokio_quic_driver_send_Group;
	class moq_tokio_quic_driver_recv_Stream;
	class moq_tokio_quic_driver_send_Error;
	class moq_quic_connection_Stats;
	class moq_quic_crypto_HandshakeData;
	class mp4_atom_Spatial;
	class mp4_atom_Any;
	class mp4_atom_Minf;
	class mp4_atom_Compressor;
	class rustls_pki_types_Der;
	class moq_tokio_quic_demux_Datagram;
	class moq_sock_demux_Pin;
	class moq_tokio_quic_driver_send_Offset;
	class moq_tokio_quic_driver_recv_Error;
	class moq_quic_FrameStats;
	class moq_quic_PathStats;
	class mp4_atom_Sv3d;
	class mp4_atom_Vexu;
	class mp4_atom_Iprp;
	class mp4_atom_Ipco;
	class mp4_atom_Moov;
	class mp4_atom_Stbl;
	class mp4_atom_Stsd;
	class mp4_atom_Nmhd;
	class mp4_atom_Sthd;
	class mp4_atom_Moof;
	class mp4_atom_Traf;
	class mp4_atom_Proj;
	class mp4_atom_VexuProj;
	class mp4_atom_Eyes;
	class mp4_atom_Pack;
	class mp4_atom_Cams;
	class mp4_atom_Cmfy;
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
	class moq_congestion_Factory;
	class moq_quic_Error;
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
	struct SessionError_KeyValueFormatting;
	struct SessionError_TooManyRequests;
	struct SessionError_GoawayTimeout;
	struct SessionError_Timeout;
	struct SessionError_AuthTokenCacheOverflow;
	struct SessionError_Version;
	struct SessionError_MalformedAuthToken;
	struct SessionError_UnknownAuthTokenAlias;
	struct SessionError_TooManyRequestUpdates;
	struct SessionError_App;
	struct SessionError_Unknown;
	struct SessionError_Unsupported;
	struct SessionError_InvalidRequestId;
	struct SessionError_InvalidPath;
	struct SessionError_InvalidAuthority;
	using SessionError = ::std::variant<SessionError_Cancel, SessionError_Internal, SessionError_Unauthorized, SessionError_Expired, SessionError_ProtocolViolation, SessionError_KeyValueFormatting, SessionError_TooManyRequests, SessionError_GoawayTimeout, SessionError_Timeout, SessionError_AuthTokenCacheOverflow, SessionError_Version, SessionError_MalformedAuthToken, SessionError_UnknownAuthTokenAlias, SessionError_TooManyRequestUpdates, SessionError_App, SessionError_Unknown, SessionError_Unsupported, SessionError_InvalidRequestId, SessionError_InvalidPath, SessionError_InvalidAuthority>;
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
	struct track_Anchor_Absolute;
	struct track_Anchor_Relative;
	struct track_Anchor_Time;
	using track_Anchor = ::std::variant<track_Anchor_Absolute, track_Anchor_Relative, track_Anchor_Time>;
	struct origin_Source_Local;
	struct origin_Source_Peer;
	using origin_Source = ::std::variant<origin_Source_Local, origin_Source_Peer>;
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
	using mux_container_ts_Step = ::std::variant<mux_container_ts_Step_Rewind, mux_container_ts_Step_Leap>;
	struct moq_quic_Rate_Unlimited;
	struct moq_quic_Rate_BytesPerSecond;
	using moq_quic_Rate = ::std::variant<moq_quic_Rate_Unlimited, moq_quic_Rate_BytesPerSecond>;
	struct mp4_atom_Colr_Nclx;
	struct mp4_atom_Colr_Nclc;
	struct mp4_atom_Colr_Ricc;
	struct mp4_atom_Colr_Prof;
	using mp4_atom_Colr = ::std::variant<mp4_atom_Colr_Nclx, mp4_atom_Colr_Nclc, mp4_atom_Colr_Ricc, mp4_atom_Colr_Prof>;
	struct mp4_atom_UncC_V1;
	struct mp4_atom_UncC_V0;
	using mp4_atom_UncC = ::std::variant<mp4_atom_UncC_V1, mp4_atom_UncC_V0>;
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
	struct mp4_atom_FlacMetadataBlock_StreamInfo;
	struct mp4_atom_FlacMetadataBlock_Padding;
	struct mp4_atom_FlacMetadataBlock_Application;
	struct mp4_atom_FlacMetadataBlock_SeekTable;
	struct mp4_atom_FlacMetadataBlock_VorbisComment;
	struct mp4_atom_FlacMetadataBlock_CueSheet;
	struct mp4_atom_FlacMetadataBlock_Picture;
	struct mp4_atom_FlacMetadataBlock_Reserved;
	struct mp4_atom_FlacMetadataBlock_Forbidden;
	using mp4_atom_FlacMetadataBlock = ::std::variant<mp4_atom_FlacMetadataBlock_StreamInfo, mp4_atom_FlacMetadataBlock_Padding, mp4_atom_FlacMetadataBlock_Application, mp4_atom_FlacMetadataBlock_SeekTable, mp4_atom_FlacMetadataBlock_VorbisComment, mp4_atom_FlacMetadataBlock_CueSheet, mp4_atom_FlacMetadataBlock_Picture, mp4_atom_FlacMetadataBlock_Reserved, mp4_atom_FlacMetadataBlock_Forbidden>;
	struct mp4_atom_ChannelStructure_ExplicitPositions;
	struct mp4_atom_ChannelStructure_DefinedLayout;
	using mp4_atom_ChannelStructure = ::std::variant<mp4_atom_ChannelStructure_ExplicitPositions, mp4_atom_ChannelStructure_DefinedLayout>;
	struct mp4_atom_Uuid_C2pa;
	struct mp4_atom_Uuid_Unknown;
	using mp4_atom_Uuid = ::std::variant<mp4_atom_Uuid_C2pa, mp4_atom_Uuid_Unknown>;
	struct mp4_atom_SpeakerPosition_Standard;
	struct mp4_atom_SpeakerPosition_Explicit;
	using mp4_atom_SpeakerPosition = ::std::variant<mp4_atom_SpeakerPosition_Standard, mp4_atom_SpeakerPosition_Explicit>;
	struct mp4_atom_DataType_WellKnown;
	struct mp4_atom_DataType_Custom;
	struct mp4_atom_DataType_Unknown;
	using mp4_atom_DataType = ::std::variant<mp4_atom_DataType_WellKnown, mp4_atom_DataType_Custom, mp4_atom_DataType_Unknown>;
	struct mp4_atom_StszSamples_Identical;
	struct mp4_atom_StszSamples_Different;
	using mp4_atom_StszSamples = ::std::variant<mp4_atom_StszSamples_Identical, mp4_atom_StszSamples_Different>;
	struct mp4_atom_DrefEntry_Url;
	struct mp4_atom_DrefEntry_Urn;
	struct mp4_atom_DrefEntry_Unknown;
	using mp4_atom_DrefEntry = ::std::variant<mp4_atom_DrefEntry_Url, mp4_atom_DrefEntry_Urn, mp4_atom_DrefEntry_Unknown>;
	struct mp4_atom_EmsgTimestamp_Relative;
	struct mp4_atom_EmsgTimestamp_Absolute;
	using mp4_atom_EmsgTimestamp = ::std::variant<mp4_atom_EmsgTimestamp_Relative, mp4_atom_EmsgTimestamp_Absolute>;
	struct mp4_atom_TrunSamples_Inherited;
	struct mp4_atom_TrunSamples_Explicit;
	using mp4_atom_TrunSamples = ::std::variant<mp4_atom_TrunSamples_Inherited, mp4_atom_TrunSamples_Explicit>;
	struct mp4_atom_ProjKind_Equi;
	struct mp4_atom_ProjKind_Cbmp;
	struct mp4_atom_ProjKind_Mshp;
	using mp4_atom_ProjKind = ::std::variant<mp4_atom_ProjKind_Equi, mp4_atom_ProjKind_Cbmp, mp4_atom_ProjKind_Mshp>;
	struct mp4_atom_IlstDataValue_Reserved;
	struct mp4_atom_IlstDataValue_Utf8;
	struct mp4_atom_IlstDataValue_Utf16;
	struct mp4_atom_IlstDataValue_Jpeg;
	struct mp4_atom_IlstDataValue_Png;
	struct mp4_atom_IlstDataValue_Bmp;
	struct mp4_atom_IlstDataValue_BeSignedInt;
	struct mp4_atom_IlstDataValue_BeUnsignedInt;
	struct mp4_atom_IlstDataValue_BeFloat32;
	struct mp4_atom_IlstDataValue_BeFloat64;
	struct mp4_atom_IlstDataValue_Unknown;
	using mp4_atom_IlstDataValue = ::std::variant<mp4_atom_IlstDataValue_Reserved, mp4_atom_IlstDataValue_Utf8, mp4_atom_IlstDataValue_Utf16, mp4_atom_IlstDataValue_Jpeg, mp4_atom_IlstDataValue_Png, mp4_atom_IlstDataValue_Bmp, mp4_atom_IlstDataValue_BeSignedInt, mp4_atom_IlstDataValue_BeUnsignedInt, mp4_atom_IlstDataValue_BeFloat32, mp4_atom_IlstDataValue_BeFloat64, mp4_atom_IlstDataValue_Unknown>;
	struct mp4_atom_AnySampleGroupEntry_DirectReferenceSampleList;
	struct mp4_atom_AnySampleGroupEntry_UnknownGroupingType;
	using mp4_atom_AnySampleGroupEntry = ::std::variant<mp4_atom_AnySampleGroupEntry_DirectReferenceSampleList, mp4_atom_AnySampleGroupEntry_UnknownGroupingType>;
	struct mp4_atom_SubsSubsampleSize_U16;
	struct mp4_atom_SubsSubsampleSize_U32;
	using mp4_atom_SubsSubsampleSize = ::std::variant<mp4_atom_SubsSubsampleSize_U16, mp4_atom_SubsSubsampleSize_U32>;
	using Waiter_poll_future_future = ::moq::future<detail::Waiter_poll_future_future_op>;
	using Server_accept_Session_future = ::moq::future<detail::Server_accept_Session_future_op>;
	using Server_accept_request_Session_future = ::moq::future<detail::Server_accept_request_Session_future_op>;
	using Session_peer_implementation_future = ::moq::future<detail::Session_peer_implementation_future_op>;
	using Session_close_future = ::moq::future<detail::Session_close_future_op>;
	using Session_closed_future = ::moq::future<detail::Session_closed_future_op>;
	using Session_idle_future = ::moq::future<detail::Session_idle_future_op>;
	using Client_connect_Session_future = ::moq::future<detail::Client_connect_Session_future_op>;
	using clock_Deadline_poll_future = ::moq::future<detail::clock_Deadline_poll_future_op>;
	using clock_Interval_tick_future = ::moq::future<detail::clock_Interval_tick_future_op>;
	using coop_Budget_poll_yield_future = ::moq::future<detail::coop_Budget_poll_yield_future_op>;
	using hang_Catalog_subscribe_future = ::moq::future<detail::hang_Catalog_subscribe_future_op>;
	using hang_Catalog_Extra_subscribe_future = ::moq::future<detail::hang_Catalog_Extra_subscribe_future_op>;
	using hang_Catalog_Ext_subscribe_future = ::moq::future<detail::hang_Catalog_Ext_subscribe_future_op>;
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
	using tokio_Server_serve_consume_future = ::moq::future<detail::tokio_Server_serve_consume_future_op>;
	using tokio_Server_serve_both_future = ::moq::future<detail::tokio_Server_serve_both_future_op>;
	using tokio_Server_bind_future = ::moq::future<detail::tokio_Server_bind_future_op>;
	using tokio_Server_listen_future = ::moq::future<detail::tokio_Server_listen_future_op>;
	using auth_Watch_changed_future = ::moq::future<detail::auth_Watch_changed_future_op>;
	using auth_Requests_next_future = ::moq::future<detail::auth_Requests_next_future_op>;
	using auth_Request_closed_future = ::moq::future<detail::auth_Request_closed_future_op>;
	using auth_Issued_closed_future = ::moq::future<detail::auth_Issued_closed_future_op>;
	using auth_Handle_add_future = ::moq::future<detail::auth_Handle_add_future_op>;
	using auth_Credential_closed_future = ::moq::future<detail::auth_Credential_closed_future_op>;
	using auth_Tokens_answered_future = ::moq::future<detail::auth_Tokens_answered_future_op>;
	using goaway_Consumer_poll_future = ::moq::future<detail::goaway_Consumer_poll_future_op>;
	using server_Handshake_ok_future = ::moq::future<detail::server_Handshake_ok_future_op>;
	using server_Activity_close_future = ::moq::future<detail::server_Activity_close_future_op>;
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
	using datagram_Producer_poll_priority_future = ::moq::future<detail::datagram_Producer_poll_priority_future_op>;
	using datagram_Consumer_info_future = ::moq::future<detail::datagram_Consumer_info_future_op>;
	using datagram_Consumer_recv_future = ::moq::future<detail::datagram_Consumer_recv_future_op>;
	using frame_Consumer_read_chunk_future = ::moq::future<detail::frame_Consumer_read_chunk_future_op>;
	using frame_Consumer_read_all_future = ::moq::future<detail::frame_Consumer_read_all_future_op>;
	using group_Producer_closed_future = ::moq::future<detail::group_Producer_closed_future_op>;
	using group_Producer_ready_future = ::moq::future<detail::group_Producer_ready_future_op>;
	using group_Demand_wanted_changed_future = ::moq::future<detail::group_Demand_wanted_changed_future_op>;
	using group_Demand_closed_future = ::moq::future<detail::group_Demand_closed_future_op>;
	using group_Consumer_next_frame_future = ::moq::future<detail::group_Consumer_next_frame_future_op>;
	using group_Consumer_read_frame_future = ::moq::future<detail::group_Consumer_read_frame_future_op>;
	using group_Consumer_poll_read_frames_8_future = ::moq::future<detail::group_Consumer_poll_read_frames_8_future_op>;
	using group_Consumer_read_frames_8_future = ::moq::future<detail::group_Consumer_read_frames_8_future_op>;
	using group_Consumer_finished_future = ::moq::future<detail::group_Consumer_finished_future_op>;
	using track_Requests_requested_fill_future = ::moq::future<detail::track_Requests_requested_fill_future_op>;
	using track_Demand_wanted_changed_future = ::moq::future<detail::track_Demand_wanted_changed_future_op>;
	using track_Demand_closed_future = ::moq::future<detail::track_Demand_closed_future_op>;
	using track_Subscriber_recv_group_future = ::moq::future<detail::track_Subscriber_recv_group_future_op>;
	using track_Subscriber_finished_future = ::moq::future<detail::track_Subscriber_finished_future_op>;
	using track_Subscriber_largest_future = ::moq::future<detail::track_Subscriber_largest_future_op>;
	using track_Ordered_next_group_future = ::moq::future<detail::track_Ordered_next_group_future_op>;
	using track_Ordered_finished_future = ::moq::future<detail::track_Ordered_finished_future_op>;
	using track_Ordered_largest_future = ::moq::future<detail::track_Ordered_largest_future_op>;
	using track_Request_subscription_changed_future = ::moq::future<detail::track_Request_subscription_changed_future_op>;
	using track_Producer_closed_future = ::moq::future<detail::track_Producer_closed_future_op>;
	using track_Producer_subscription_changed_future = ::moq::future<detail::track_Producer_subscription_changed_future_op>;
	using track_Consumer_subscribe_future = ::moq::future<detail::track_Consumer_subscribe_future_op>;
	using track_Consumer_fetch_group_future = ::moq::future<detail::track_Consumer_fetch_group_future_op>;
	using track_Consumer_info_future = ::moq::future<detail::track_Consumer_info_future_op>;
	using track_Consumer_largest_future = ::moq::future<detail::track_Consumer_largest_future_op>;
	using track_Fill_unwanted_future = ::moq::future<detail::track_Fill_unwanted_future_op>;
	using track_Fill_wanted_changed_future = ::moq::future<detail::track_Fill_wanted_changed_future_op>;
	using origin_Following_available_future = ::moq::future<detail::origin_Following_available_future_op>;
	using origin_Consumer_request_future = ::moq::future<detail::origin_Consumer_request_future_op>;
	using origin_Requests_requested_broadcast_future = ::moq::future<detail::origin_Requests_requested_broadcast_future_op>;
	using announce_Consumer_next_future = ::moq::future<detail::announce_Consumer_next_future_op>;
	using announce_Consumer_poll_synced_future = ::moq::future<detail::announce_Consumer_poll_synced_future_op>;
	using hang_catalog_Consumer_next_future = ::moq::future<detail::hang_catalog_Consumer_next_future_op>;
	using hang_catalog_Consumer_Extra_next_future = ::moq::future<detail::hang_catalog_Consumer_Extra_next_future_op>;
	using hang_catalog_Consumer_Ext_next_future = ::moq::future<detail::hang_catalog_Consumer_Ext_next_future_op>;
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
	using mux_json_Consumer_Json_next_future = ::moq::future<detail::mux_json_Consumer_Json_next_future_op>;
	using mux_json_Consumer_Json_latest_future = ::moq::future<detail::mux_json_Consumer_Json_latest_future_op>;
	using mux_rate_Follower_next_future = ::moq::future<detail::mux_rate_Follower_next_future_op>;
	using tokio_connection_Monitor_state_changed_future = ::moq::future<detail::tokio_connection_Monitor_state_changed_future_op>;
	using tokio_connection_Monitor_poll_presence_future = ::moq::future<detail::tokio_connection_Monitor_poll_presence_future_op>;
	using tokio_connection_Monitor_presence_changed_future = ::moq::future<detail::tokio_connection_Monitor_presence_changed_future_op>;
	using tokio_server_Listener_accept_future = ::moq::future<detail::tokio_server_Listener_accept_future_op>;
	using tokio_server_Listener_close_future = ::moq::future<detail::tokio_server_Listener_close_future_op>;
	using tokio_server_Request_reject_future = ::moq::future<detail::tokio_server_Request_reject_future_op>;
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
	using moq_tokio_quic_driver_connection_Connection_open_uni_future = ::moq::future<detail::moq_tokio_quic_driver_connection_Connection_open_uni_future_op>;
	using moq_tokio_quic_driver_connection_Connection_open_bi_future = ::moq::future<detail::moq_tokio_quic_driver_connection_Connection_open_bi_future_op>;
	using moq_tokio_quic_driver_connection_Connection_accept_uni_future = ::moq::future<detail::moq_tokio_quic_driver_connection_Connection_accept_uni_future_op>;
	using moq_tokio_quic_driver_connection_Connection_accept_bi_future = ::moq::future<detail::moq_tokio_quic_driver_connection_Connection_accept_bi_future_op>;
	using moq_tokio_quic_driver_connection_Connection_read_datagram_future = ::moq::future<detail::moq_tokio_quic_driver_connection_Connection_read_datagram_future_op>;
	using moq_tokio_quic_driver_connection_Connection_poll_recv_datagram_future = ::moq::future<detail::moq_tokio_quic_driver_connection_Connection_poll_recv_datagram_future_op>;
	using moq_tokio_quic_driver_connection_Connection_closed_future = ::moq::future<detail::moq_tokio_quic_driver_connection_Connection_closed_future_op>;
	using moq_tokio_quic_demux_WebRtc_recv_future = ::moq::future<detail::moq_tokio_quic_demux_WebRtc_recv_future_op>;
	using moq_tokio_quic_driver_send_Stream_write_all_future = ::moq::future<detail::moq_tokio_quic_driver_send_Stream_write_all_future_op>;
	using moq_tokio_quic_driver_send_Stream_finish_future = ::moq::future<detail::moq_tokio_quic_driver_send_Stream_finish_future_op>;
	using moq_tokio_quic_driver_send_Stream_stopped_future = ::moq::future<detail::moq_tokio_quic_driver_send_Stream_stopped_future_op>;
	using moq_tokio_quic_driver_send_Stream_poll_closed_future = ::moq::future<detail::moq_tokio_quic_driver_send_Stream_poll_closed_future_op>;
	using moq_tokio_quic_driver_recv_Stream_read_chunk_future = ::moq::future<detail::moq_tokio_quic_driver_recv_Stream_read_chunk_future_op>;
	using moq_tokio_quic_driver_recv_Stream_read_to_end_future = ::moq::future<detail::moq_tokio_quic_driver_recv_Stream_read_to_end_future_op>;
	using moq_tokio_quic_driver_recv_Stream_received_reset_future = ::moq::future<detail::moq_tokio_quic_driver_recv_Stream_received_reset_future_op>;
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
#ifndef MOQ_HPP_CRATE13
#define MOQ_HPP_CRATE13
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
#ifndef MOQ_HPP_CRATE12
#define MOQ_HPP_CRATE12
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
#include "net.hpp"
namespace moq {
	class RUST2_FFI_CONSUMABLE Timed_Json {
		RUST2_FFI_UNCONSUMED Timed_Json(Timed_Json &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		Timed_Json &operator=(Timed_Json &&other) noexcept = default;
		RUST2_FFI_CALLABLE Timed_Json clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::Json value_get() const;
		RUST2_FFI_CALLABLE void value_set(::moq::Json value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Timestamp> at_get() const;
		RUST2_FFI_CALLABLE void at_set(const ::std::optional<::moq::Timestamp> &value);
		RUST2_FFI_CONSUMES ::moq::Timed_Json at(const ::moq::Timestamp &at) &&;
	};
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
#include "congestion.hpp"
#include "net.hpp"
#include "quic.hpp"
#include "sock.hpp"
#include "rustls.hpp"
#include "rustlspkitypes.hpp"
#include "tracingcore.hpp"
#include "url.hpp"
namespace moq {
	enum class tokio_ConnectError {
		unauthorized = 0,
		forbidden = 1,
	};
	enum class tokio_Restart {
		redial = 0,
		wait = 1,
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
		unix = 3,
		web_transport = 4,
		web_rtc = 5,
	};
	enum class moq_tokio_quic_CongestionControl {
		loss = 0,
		delay = 1,
		none = 2,
	};
	enum class moq_tokio_connection_Phase {
		connecting = 0,
		connected = 1,
		disconnected = 2,
		migrating = 3,
		closed = 4,
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
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Duration> max_get() const;
		RUST2_FFI_CALLABLE void max_set(const ::moq::tokio_Duration *value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Duration> healthy_get() const;
		RUST2_FFI_CALLABLE void healthy_set(const ::moq::tokio_Duration *value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Duration> timeout_get() const;
		RUST2_FFI_CALLABLE void timeout_set(const ::moq::tokio_Duration *value);
		RUST2_FFI_CALLABLE ::moq::duration initial() const;
		RUST2_FFI_CALLABLE ::moq::duration max() const;
		RUST2_FFI_CALLABLE ::moq::duration timeout() const;
	};
	class RUST2_FFI_CONSUMABLE tokio_Client {
		RUST2_FFI_UNCONSUMED tokio_Client(tokio_Client &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_Client &operator=(tokio_Client &&other) noexcept = default;
		RUST2_FFI_CALLABLE tokio_Client clone() const;
		static ::moq::result<::moq::tokio_Client, ::moq::tokio_Error> new_(::moq::tokio_client_Config config);
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_publisher_Producer(const ::moq::origin_Producer &publish) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_publisher_Consumer(const ::moq::origin_Consumer &publish) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_origin(::moq::origin_Producer origin) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_subscriber(::moq::origin_Producer subscribe) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_limits(const ::moq::session_Limits &limits) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_stats(::moq::stats_Session stats) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_cost(::std::uint64_t cost) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Client with_peer_hop(const ::moq::Hop &hop) &&;
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
		RUST2_FFI_CONSUMES ::moq::tokio_Server_serve_consume_future serve_consume(::moq::origin_Producer origin) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_Server_serve_both_future serve_both(::moq::origin_Consumer publish, ::moq::origin_Producer subscribe) &&;
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
	class RUST2_FFI_CONSUMABLE tokio_tls_Peers {
		RUST2_FFI_UNCONSUMED tokio_tls_Peers(tokio_tls_Peers &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_tls_Peers &operator=(tokio_tls_Peers &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED tokio_tls_Peers(const tokio_tls_Peers &other) = default;
		tokio_tls_Peers &operator=(const tokio_tls_Peers &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static tokio_tls_Peers default_();
		static ::moq::tokio_tls_Peers new_();
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::tokio_tls_Error> insert(::moq::input fingerprint) const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::tokio_tls_Error> remove(::moq::input fingerprint) const;
		RUST2_FFI_CALLABLE bool contains(::moq::input fingerprint) const;
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
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_tls_Peers> peers_get() const;
		RUST2_FFI_CALLABLE void peers_set(::std::optional<::moq::tokio_tls_Peers> value);
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
	};
	class RUST2_FFI_CONSUMABLE tokio_tls_PeerIdentity {
		RUST2_FFI_UNCONSUMED tokio_tls_PeerIdentity(tokio_tls_PeerIdentity &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_tls_PeerIdentity &operator=(tokio_tls_PeerIdentity &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED tokio_tls_PeerIdentity(const tokio_tls_PeerIdentity &other) = default;
		tokio_tls_PeerIdentity &operator=(const tokio_tls_PeerIdentity &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static ::moq::tokio_tls_PeerIdentity from_chain(::std::vector<::moq::rustls_pki_types_CertificateDer> chain);
		RUST2_FFI_CALLABLE ::std::vector<::moq::rustls_pki_types_CertificateDer> chain() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> fingerprint() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> name() const;
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
		RUST2_FFI_CALLABLE ::moq::setup_Extensions extensions_get() const;
		RUST2_FFI_CALLABLE void extensions_set(const ::moq::setup_Extensions &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::buf> implementation_get() const;
		RUST2_FFI_CALLABLE void implementation_set(::std::optional<::moq::input> value);
		RUST2_FFI_CALLABLE ::moq::tokio_tls_Connect tls_get() const;
		RUST2_FFI_CALLABLE void tls_set(::moq::tokio_tls_Connect value);
		RUST2_FFI_CALLABLE ::moq::tokio_Backoff backoff_get() const;
		RUST2_FFI_CALLABLE void backoff_set(::moq::tokio_Backoff value);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_connection_Goaway goaway_get() const;
		RUST2_FFI_CALLABLE void goaway_set(::moq::moq_tokio_connection_Goaway value);
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
		RUST2_FFI_CALLABLE ::moq::setup_Extensions extensions_get() const;
		RUST2_FFI_CALLABLE void extensions_set(const ::moq::setup_Extensions &value);
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
		RUST2_FFI_CALLABLE bool has_explicit_bind() const;
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
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_tokio_quic_Controller> controller_get() const;
		RUST2_FFI_CALLABLE void controller_set(::std::optional<::moq::moq_tokio_quic_Controller> value);
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
		RUST2_FFI_CALLABLE ::moq::server_Activity activity_get() const;
		RUST2_FFI_CALLABLE void activity_set(::moq::server_Activity value);
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
		RUST2_FFI_CONSUMES ::moq::result<::moq::tokio_Server, ::moq::tokio_Error> init() &&;
		RUST2_FFI_CONSUMES ::moq::result<::moq::tokio_Server, ::moq::tokio_Error> init_streams() &&;
	};
	class RUST2_FFI_CONSUMABLE tokio_server_Listener {
		RUST2_FFI_UNCONSUMED tokio_server_Listener(tokio_server_Listener &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		tokio_server_Listener &operator=(tokio_server_Listener &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::tokio_server_Listener_accept_future accept();
		RUST2_FFI_CONSUMES ::moq::tokio_server_Listener_close_future close() &&;
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
		RUST2_FFI_CONSUMES ::moq::tokio_server_Request with_peer_hop(const ::moq::Hop &hop) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_server_Request with_stats(::moq::stats_Session stats) &&;
		RUST2_FFI_CONSUMES ::moq::tokio_server_Request with_limits(const ::moq::session_Limits &limits) &&;
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_tokio_quic_driver_connection_Connection> quic() const;
		RUST2_FFI_CONSUMES ::moq::result<::moq::tokio_server_Request, ::moq::buf> with_quic_limits(const ::moq::moq_quic_Limits &limits) &&;
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
		RUST2_FFI_CALLABLE ::std::optional<::moq::Hop> peer_hop() const;
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
		RUST2_FFI_CALLABLE ::moq::duration timeout_get() const;
		RUST2_FFI_CALLABLE void timeout_set(::moq::duration value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_connection_Goaway {
		RUST2_FFI_UNCONSUMED moq_tokio_connection_Goaway(moq_tokio_connection_Goaway &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_connection_Goaway &operator=(moq_tokio_connection_Goaway &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_connection_Goaway clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_tokio_connection_Goaway default_();
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
		RUST2_FFI_CALLABLE ::moq::moq_tokio_connection_Phase phase_get() const;
		RUST2_FFI_CALLABLE void phase_set(::moq::moq_tokio_connection_Phase value);
		RUST2_FFI_CALLABLE ::std::uint64_t connects_get() const;
		RUST2_FFI_CALLABLE void connects_set(::std::uint64_t value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::clock_Instant> last_connected_get() const;
		RUST2_FFI_CALLABLE void last_connected_set(const ::std::optional<::moq::clock_Instant> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Transport> transport_get() const;
		RUST2_FFI_CALLABLE void transport_set(const ::std::optional<::moq::tokio_Transport> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::tokio_Error> failure_get() const;
		RUST2_FFI_CALLABLE void failure_set(::std::optional<::moq::tokio_Error> value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::Version> version_get() const;
		RUST2_FFI_CALLABLE void version_set(const ::moq::Version *value);
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
		RUST2_FFI_CALLABLE ::moq::setup_Extensions extensions_get() const;
		RUST2_FFI_CALLABLE void extensions_set(const ::moq::setup_Extensions &value);
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
		RUST2_FFI_CALLABLE ::moq::duration idle_timeout_get() const;
		RUST2_FFI_CALLABLE void idle_timeout_set(::moq::duration value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::duration> keep_alive_get() const;
		RUST2_FFI_CALLABLE void keep_alive_set(const ::std::optional<::moq::duration> &value);
		RUST2_FFI_CALLABLE bool mtu_discovery_get() const;
		RUST2_FFI_CALLABLE void mtu_discovery_set(bool value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_tokio_quic_CongestionControl> congestion_control_get() const;
		RUST2_FFI_CALLABLE void congestion_control_set(const ::std::optional<::moq::moq_tokio_quic_CongestionControl> &value);
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_tokio_quic_Controller> controller_get() const;
		RUST2_FFI_CALLABLE void controller_set(::std::optional<::moq::moq_tokio_quic_Controller> value);
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
		RUST2_FFI_CALLABLE bool has_controller() const;
		RUST2_FFI_CALLABLE ::moq::duration keep_alive_interval() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_quic_Controller {
		RUST2_FFI_UNCONSUMED moq_tokio_quic_Controller(moq_tokio_quic_Controller &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_quic_Controller &operator=(moq_tokio_quic_Controller &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_quic_Controller clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::moq_congestion_Factory _0_get() const;
		RUST2_FFI_CALLABLE void _0_set(::moq::moq_congestion_Factory value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_websocket_Listener {
		RUST2_FFI_UNCONSUMED moq_tokio_websocket_Listener(moq_tokio_websocket_Listener &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_websocket_Listener &operator=(moq_tokio_websocket_Listener &&other) noexcept = default;
		static ::moq::moq_tokio_websocket_Listener_bind_future bind(::moq::socket_addr addr);
		RUST2_FFI_CONSUMES ::moq::result<::moq::moq_tokio_websocket_Listener, ::moq::moq_tokio_websocket_Error> with_protocols(const ::moq::items<::moq::view> &protocols) &&;
		RUST2_FFI_CONSUMES ::moq::moq_tokio_websocket_Listener with_quic(const ::moq::moq_tokio_quic_Resolved &quic) &&;
		RUST2_FFI_CALLABLE ::moq::result<::moq::socket_addr, ::moq::moq_tokio_websocket_Error> local_addr() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_accept_Health accept_health() const;
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
	class RUST2_FFI_CONSUMABLE moq_tokio_quic_driver_connection_Connection {
		RUST2_FFI_UNCONSUMED moq_tokio_quic_driver_connection_Connection(moq_tokio_quic_driver_connection_Connection &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_quic_driver_connection_Connection &operator=(moq_tokio_quic_driver_connection_Connection &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_quic_driver_connection_Connection clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_connection_Connection_open_uni_future open_uni(const ::moq::moq_tokio_quic_driver_send_Options &options) const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_send_Group send_group(::std::int64_t priority) const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_connection_Connection_open_bi_future open_bi(const ::moq::moq_tokio_quic_driver_send_Options &options) const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_connection_Connection_accept_uni_future accept_uni() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_connection_Connection_accept_bi_future accept_bi() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_connection_Connection_read_datagram_future read_datagram() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_connection_Connection_poll_recv_datagram_future poll_recv_datagram() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_connection_Connection_closed_future closed() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_quic_Error> close_reason() const;
		RUST2_FFI_CALLABLE void close(const ::moq::moq_quic_VarInt &error_code, ::moq::input reason) const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::moq_tokio_quic_driver_send_Error> send_datagram(::moq::input data, const ::moq::moq_tokio_quic_driver_send_Options &options) const;
		RUST2_FFI_CALLABLE ::std::optional<::std::size_t> max_datagram_size() const;
		RUST2_FFI_CALLABLE ::std::size_t datagram_send_buffer_space() const;
		RUST2_FFI_CALLABLE ::moq::socket_addr remote_address() const;
		RUST2_FFI_CALLABLE ::moq::duration rtt() const;
		RUST2_FFI_CALLABLE ::moq::moq_quic_connection_Stats stats() const;
		RUST2_FFI_CALLABLE ::std::optional<::moq::moq_quic_crypto_HandshakeData> handshake_data() const;
		RUST2_FFI_CALLABLE ::std::optional<::std::vector<::moq::rustls_pki_types_CertificateDer>> peer_identity() const;
		RUST2_FFI_CALLABLE ::std::size_t stable_id() const;
		RUST2_FFI_CALLABLE ::moq::result<void, ::moq::buf> set_limits(const ::moq::moq_quic_Limits &limits) const;
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
		RUST2_FFI_CALLABLE bool configured() const;
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
		RUST2_FFI_CALLABLE void uid_set(const ::std::vector<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::vector<::std::uint32_t> gid_get() const;
		RUST2_FFI_CALLABLE void gid_set(const ::std::vector<::std::uint32_t> &value);
		RUST2_FFI_CALLABLE ::std::vector<::std::int32_t> pid_get() const;
		RUST2_FFI_CALLABLE void pid_set(const ::std::vector<::std::int32_t> &value);
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
	class RUST2_FFI_CONSUMABLE moq_tokio_quic_driver_send_Options {
		RUST2_FFI_UNCONSUMED moq_tokio_quic_driver_send_Options(moq_tokio_quic_driver_send_Options &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_quic_driver_send_Options &operator=(moq_tokio_quic_driver_send_Options &&other) noexcept = default;
		RUST2_FFI_CALLABLE moq_tokio_quic_driver_send_Options clone() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		static moq_tokio_quic_driver_send_Options default_();
		RUST2_FFI_CALLABLE ::std::int64_t priority_get() const;
		RUST2_FFI_CALLABLE void priority_set(::std::int64_t value);
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_quic_driver_send_Stream {
		RUST2_FFI_UNCONSUMED moq_tokio_quic_driver_send_Stream(moq_tokio_quic_driver_send_Stream &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_quic_driver_send_Stream &operator=(moq_tokio_quic_driver_send_Stream &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_send_Stream_write_all_future write_all(::moq::input buf_);
		RUST2_FFI_CONSUMES ::moq::moq_tokio_quic_driver_send_Stream_finish_future finish() &&;
		RUST2_FFI_CONSUMES void reset(const ::moq::moq_quic_VarInt &error_code) &&;
		RUST2_FFI_CONSUMES void reset_at(const ::moq::moq_tokio_quic_driver_send_Offset &offset, const ::moq::moq_quic_VarInt &error_code) &&;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_send_Offset offset() const;
		RUST2_FFI_CALLABLE void set_priority(::std::int64_t priority);
		RUST2_FFI_CALLABLE ::std::int64_t priority() const;
		RUST2_FFI_CALLABLE void set_group(const ::moq::moq_tokio_quic_driver_send_Group *group);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_send_Stream_stopped_future stopped() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_send_Stream_poll_closed_future poll_closed() const;
		RUST2_FFI_CALLABLE ::moq::moq_quic_stream_Id id() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_quic_driver_send_Group {
		RUST2_FFI_UNCONSUMED moq_tokio_quic_driver_send_Group(moq_tokio_quic_driver_send_Group &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_quic_driver_send_Group &operator=(moq_tokio_quic_driver_send_Group &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED moq_tokio_quic_driver_send_Group(const moq_tokio_quic_driver_send_Group &other) = default;
		moq_tokio_quic_driver_send_Group &operator=(const moq_tokio_quic_driver_send_Group &other) = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE void set_priority(::std::int64_t priority) const;
		RUST2_FFI_CALLABLE ::std::int64_t priority() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_quic_driver_recv_Stream {
		RUST2_FFI_UNCONSUMED moq_tokio_quic_driver_recv_Stream(moq_tokio_quic_driver_recv_Stream &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_quic_driver_recv_Stream &operator=(moq_tokio_quic_driver_recv_Stream &&other) noexcept = default;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_recv_Stream_read_chunk_future read_chunk(::std::size_t max);
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_recv_Stream_read_to_end_future read_to_end(::std::size_t limit);
		RUST2_FFI_CALLABLE void set_priority(::std::int64_t priority);
		RUST2_FFI_CONSUMES void stop(const ::moq::moq_quic_VarInt &error_code) &&;
		RUST2_FFI_CALLABLE ::moq::moq_quic_stream_Id id() const;
		RUST2_FFI_CALLABLE ::moq::moq_tokio_quic_driver_recv_Stream_received_reset_future received_reset();
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_quic_driver_send_Error {
		RUST2_FFI_UNCONSUMED moq_tokio_quic_driver_send_Error(moq_tokio_quic_driver_send_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_quic_driver_send_Error &operator=(moq_tokio_quic_driver_send_Error &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED moq_tokio_quic_driver_send_Error(const moq_tokio_quic_driver_send_Error &other) = default;
		moq_tokio_quic_driver_send_Error &operator=(const moq_tokio_quic_driver_send_Error &other) = default;
		friend bool operator==(const moq_tokio_quic_driver_send_Error &a, const moq_tokio_quic_driver_send_Error &b);
		friend bool operator!=(const moq_tokio_quic_driver_send_Error &a, const moq_tokio_quic_driver_send_Error &b);
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
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
	class RUST2_FFI_CONSUMABLE moq_tokio_quic_driver_send_Offset {
		RUST2_FFI_UNCONSUMED moq_tokio_quic_driver_send_Offset(moq_tokio_quic_driver_send_Offset &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_quic_driver_send_Offset &operator=(moq_tokio_quic_driver_send_Offset &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED moq_tokio_quic_driver_send_Offset(const moq_tokio_quic_driver_send_Offset &other) = default;
		moq_tokio_quic_driver_send_Offset &operator=(const moq_tokio_quic_driver_send_Offset &other) = default;
		friend bool operator==(const moq_tokio_quic_driver_send_Offset &a, const moq_tokio_quic_driver_send_Offset &b);
		friend bool operator!=(const moq_tokio_quic_driver_send_Offset &a, const moq_tokio_quic_driver_send_Offset &b);
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
	};
	class RUST2_FFI_CONSUMABLE moq_tokio_quic_driver_recv_Error {
		RUST2_FFI_UNCONSUMED moq_tokio_quic_driver_recv_Error(moq_tokio_quic_driver_recv_Error &&other RUST2_FFI_MOVED_FROM) noexcept = default;
		moq_tokio_quic_driver_recv_Error &operator=(moq_tokio_quic_driver_recv_Error &&other) noexcept = default;
		RUST2_FFI_UNCONSUMED moq_tokio_quic_driver_recv_Error(const moq_tokio_quic_driver_recv_Error &other) = default;
		moq_tokio_quic_driver_recv_Error &operator=(const moq_tokio_quic_driver_recv_Error &other) = default;
		friend bool operator==(const moq_tokio_quic_driver_recv_Error &a, const moq_tokio_quic_driver_recv_Error &b);
		friend bool operator!=(const moq_tokio_quic_driver_recv_Error &a, const moq_tokio_quic_driver_recv_Error &b);
		RUST2_FFI_CALLABLE ::moq::buf to_string() const;
		RUST2_FFI_CALLABLE ::moq::buf debug() const;
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
	inline bool operator==(::moq::tokio_Restart a, ::moq::tokio_Restart b);
	inline bool operator!=(::moq::tokio_Restart a, ::moq::tokio_Restart b);
	inline ::moq::buf tokio_Restart_debug(::moq::tokio_Restart self);
	inline tokio_Restart tokio_Restart_default();
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
	inline bool operator==(::moq::moq_tokio_quic_CongestionControl a, ::moq::moq_tokio_quic_CongestionControl b);
	inline bool operator!=(::moq::moq_tokio_quic_CongestionControl a, ::moq::moq_tokio_quic_CongestionControl b);
	inline ::moq::buf moq_tokio_quic_CongestionControl_debug(::moq::moq_tokio_quic_CongestionControl self);
	inline bool operator==(::moq::moq_tokio_connection_Phase a, ::moq::moq_tokio_connection_Phase b);
	inline bool operator!=(::moq::moq_tokio_connection_Phase a, ::moq::moq_tokio_connection_Phase b);
	inline ::moq::buf moq_tokio_connection_Phase_debug(::moq::moq_tokio_connection_Phase self);
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
		trace = 0,
		debug = 1,
		info = 2,
		warn = 3,
		error = 4,
	};
	inline bool operator==(::moq::Level a, ::moq::Level b);
	inline bool operator!=(::moq::Level a, ::moq::Level b);
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
