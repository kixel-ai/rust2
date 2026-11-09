//! web-rs's web-codecs `VideoEncoder`, its calls kept as they are at `42d4e4de` but for the
//! config fields the fixture leaves unset: it hands JS a closure for each output and error,
//! passes frames and errors on through tokio's channels, and awaits the promises of
//! `isConfigSupported` and `flush`.

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use bytes::{Bytes, BytesMut};
use tokio::sync::{mpsc, watch};
use wasm_bindgen::{JsCast, prelude::*};

pub type Timestamp = Duration;

#[derive(Debug, Clone)]
pub enum Error {
	Dropped,
	InvalidDimensions,
	Unknown(JsValue),
}

impl From<JsValue> for Error {
	fn from(e: JsValue) -> Self {
		Self::Unknown(e)
	}
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Dimensions {
	pub width: u32,
	pub height: u32,
}

impl Dimensions {
	pub fn new(width: u32, height: u32) -> Self {
		Self { width, height }
	}
}

pub struct EncodedFrame {
	pub payload: Bytes,
	pub timestamp: Timestamp,
	pub keyframe: bool,
}

impl fmt::Debug for EncodedFrame {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("EncodedFrame")
			.field("payload", &self.payload.len())
			.field("timestamp", &self.timestamp)
			.field("keyframe", &self.keyframe)
			.finish()
	}
}

impl From<web_sys::EncodedVideoChunk> for EncodedFrame {
	fn from(chunk: web_sys::EncodedVideoChunk) -> Self {
		let size = chunk.byte_length() as usize;

		let mut payload = BytesMut::with_capacity(size);
		payload.resize(size, 0);
		chunk.copy_to_with_u8_slice(&mut payload).unwrap();

		Self {
			payload: payload.freeze(),
			timestamp: Timestamp::from_micros(chunk.timestamp() as _),
			keyframe: chunk.type_() == web_sys::EncodedVideoChunkType::Key,
		}
	}
}

#[derive(Debug, Default, Clone)]
pub struct VideoDecoderConfig {
	pub codec: String,
	pub resolution: Option<Dimensions>,
}

impl From<web_sys::VideoDecoderConfig> for VideoDecoderConfig {
	fn from(this: web_sys::VideoDecoderConfig) -> Self {
		let resolution = match (this.get_coded_width(), this.get_coded_height()) {
			(Some(width), Some(height)) if width != 0 && height != 0 => Some(Dimensions { width, height }),
			_ => None,
		};

		Self {
			codec: this.get_codec(),
			resolution,
		}
	}
}

#[derive(Debug, Default, Clone)]
pub struct VideoEncoderConfig {
	pub codec: String,
	pub resolution: Dimensions,
	pub latency_optimized: Option<bool>,
	pub bitrate: Option<u32>,
	pub framerate: Option<f64>,
	pub max_gop_duration: Option<Duration>,
}

impl VideoEncoderConfig {
	pub fn new<T: Into<String>>(codec: T, resolution: Dimensions) -> Self {
		Self {
			codec: codec.into(),
			resolution,
			latency_optimized: None,
			bitrate: None,
			framerate: None,
			max_gop_duration: None,
		}
	}

	pub async fn is_supported(&self) -> Result<bool, Error> {
		let res =
			wasm_bindgen_futures::JsFuture::from(web_sys::VideoEncoder::is_config_supported(&self.into())).await?;
		let support: web_sys::VideoEncoderSupport = res.unchecked_into();
		Ok(support.get_supported().unwrap_or(false))
	}

	pub fn is_valid(&self) -> Result<(), Error> {
		if self.resolution.width == 0 || self.resolution.height == 0 {
			return Err(Error::InvalidDimensions);
		}

		Ok(())
	}

	pub fn init(self) -> Result<(VideoEncoder, VideoEncoded), Error> {
		let (frames_tx, frames_rx) = mpsc::unbounded_channel();
		let (closed_tx, closed_rx) = watch::channel(Ok(()));
		let config = Rc::new(RefCell::new(None));

		let decoder = VideoEncoder::new(self, config.clone(), frames_tx, closed_tx)?;
		let decoded = VideoEncoded::new(config, frames_rx, closed_rx);

		Ok((decoder, decoded))
	}
}

impl From<&VideoEncoderConfig> for web_sys::VideoEncoderConfig {
	fn from(this: &VideoEncoderConfig) -> Self {
		let config = web_sys::VideoEncoderConfig::new(&this.codec, this.resolution.height, this.resolution.width);

		if let Some(value) = this.latency_optimized {
			config.set_latency_mode(match value {
				true => web_sys::LatencyMode::Realtime,
				false => web_sys::LatencyMode::Quality,
			});
		}

		if let Some(value) = this.bitrate {
			config.set_bitrate(value);
		}

		if let Some(value) = this.framerate {
			config.set_framerate(value);
		}

		config
	}
}

#[derive(Debug, Default)]
pub struct VideoEncodeOptions {
	// Force or deny a key frame.
	pub key_frame: Option<bool>,
}

pub struct VideoEncoder {
	inner: web_sys::VideoEncoder,
	config: VideoEncoderConfig,

	last_keyframe: Rc<RefCell<Option<Timestamp>>>,

	// These are held to avoid dropping them.
	#[allow(dead_code)]
	on_error: Closure<dyn FnMut(JsValue)>,
	#[allow(dead_code)]
	on_frame: Closure<dyn FnMut(JsValue, JsValue)>,
}

impl VideoEncoder {
	fn new(
		config: VideoEncoderConfig,
		on_config: Rc<RefCell<Option<VideoDecoderConfig>>>,
		on_frame: mpsc::UnboundedSender<EncodedFrame>,
		on_error: watch::Sender<Result<(), Error>>,
	) -> Result<Self, Error> {
		let last_keyframe = Rc::new(RefCell::new(None));
		let last_keyframe2 = last_keyframe.clone();

		let on_error2 = on_error.clone();
		let on_error = Closure::wrap(Box::new(move |e: JsValue| {
			on_error.send_replace(Err(Error::from(e))).ok();
		}) as Box<dyn FnMut(_)>);

		let on_frame = Closure::wrap(Box::new(move |frame: JsValue, meta: JsValue| {
			// First parameter is the frame, second optional parameter is metadata.
			let frame: web_sys::EncodedVideoChunk = frame.unchecked_into();
			let frame = EncodedFrame::from(frame);

			if let Ok(metadata) = meta.dyn_into::<js_sys::Object>()
				&& let Ok(config) = js_sys::Reflect::get(&metadata, &"decoderConfig".into())
				&& !config.is_falsy()
			{
				let config: web_sys::VideoDecoderConfig = config.unchecked_into();
				let config = VideoDecoderConfig::from(config);
				on_config.borrow_mut().replace(config);
			}

			if frame.keyframe {
				let mut last_keyframe = last_keyframe2.borrow_mut();
				if frame.timestamp > last_keyframe.unwrap_or_default() {
					*last_keyframe = Some(frame.timestamp);
				}
			}

			if on_frame.send(frame).is_err() {
				on_error2.send_replace(Err(Error::Dropped)).ok();
			}
		}) as Box<dyn FnMut(_, _)>);

		let init = web_sys::VideoEncoderInit::new(on_error.as_ref().unchecked_ref(), on_frame.as_ref().unchecked_ref());
		let inner: web_sys::VideoEncoder = web_sys::VideoEncoder::new(&init).unwrap();
		inner.configure(&(&config).into())?;

		Ok(Self {
			config,
			inner,
			last_keyframe,
			on_error,
			on_frame,
		})
	}

	pub fn encode(&mut self, frame: &web_sys::VideoFrame, options: VideoEncodeOptions) -> Result<(), Error> {
		let o = web_sys::VideoEncoderEncodeOptions::new();

		if let Some(key_frame) = options.key_frame {
			o.set_key_frame(key_frame);
		} else if let Some(max_gop_duration) = self.config.max_gop_duration {
			let timestamp = Timestamp::from_micros(frame.timestamp() as _);
			let mut last_keyframe = self.last_keyframe.borrow_mut();

			let duration = timestamp - last_keyframe.unwrap_or_default();
			if duration >= max_gop_duration {
				o.set_key_frame(true);
			}

			*last_keyframe = Some(timestamp);
		}

		self.inner.encode_with_options(frame, &o)?;

		Ok(())
	}

	pub fn queue_size(&self) -> u32 {
		self.inner.encode_queue_size()
	}

	pub fn config(&self) -> &VideoEncoderConfig {
		&self.config
	}

	pub async fn flush(&mut self) -> Result<(), Error> {
		wasm_bindgen_futures::JsFuture::from(self.inner.flush()).await?;
		Ok(())
	}
}

impl Drop for VideoEncoder {
	fn drop(&mut self) {
		let _ = self.inner.close();
	}
}

pub struct VideoEncoded {
	config: Rc<RefCell<Option<VideoDecoderConfig>>>,
	frames: mpsc::UnboundedReceiver<EncodedFrame>,
	closed: watch::Receiver<Result<(), Error>>,
}

impl VideoEncoded {
	fn new(
		config: Rc<RefCell<Option<VideoDecoderConfig>>>,
		frames: mpsc::UnboundedReceiver<EncodedFrame>,
		closed: watch::Receiver<Result<(), Error>>,
	) -> Self {
		Self { config, frames, closed }
	}

	pub async fn frame(&mut self) -> Result<Option<EncodedFrame>, Error> {
		tokio::select! {
			biased;
			frame = self.frames.recv() => Ok(frame),
			Ok(()) = self.closed.changed() => Err(self.closed.borrow().clone().err().unwrap()),
		}
	}

	/// Returns the decoder config, after the first frame has been encoded.
	pub fn config(&self) -> Option<VideoDecoderConfig> {
		self.config.borrow().clone()
	}
}
