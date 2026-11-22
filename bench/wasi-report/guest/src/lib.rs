//! wb's shapes as a component, each calling the same Rust the core module's glue does.

#[cfg(not(feature = "async"))]
wit_bindgen::generate!({ path: "wit", world: "sync" });
#[cfg(feature = "async")]
wit_bindgen::generate!({ path: "wit", world: "full" });

use exports::rust2::wb::api;

struct Guest;

impl api::Guest for Guest {
	type Info = Info;
	type Track = Track;

	fn noop() {
		wb::noop()
	}
	fn visit_with(f: &rust2::wb::callback::Visit) -> u64 {
		wb::visit(|| f.call())
	}
	fn boom() {
		wb::boom()
	}
}

struct Info(wb::Info);

impl api::GuestInfo for Info {
	fn priority(&self) -> i8 {
		self.0.priority()
	}
}

struct Track(wb::Track);

impl api::GuestTrack for Track {
	fn new() -> Self {
		Self(wb::Track::new())
	}
	fn is_closed(&self) -> bool {
		self.0.is_closed()
	}
	fn start(&self) -> u64 {
		self.0.start()
	}
	fn largest(&self) -> Option<api::Span> {
		self.0.largest().map(|s| api::Span { start: s.start, end: s.end })
	}
	fn info(&self) -> api::Info {
		api::Info::new(Info(self.0.info()))
	}
	fn write(&self, frame: Vec<u8>) {
		self.0.write(&frame)
	}
	fn payload(&self) -> Vec<u8> {
		self.0.payload().to_vec()
	}
	fn big(&self, len: u32) -> Vec<u8> {
		self.0.big(len).to_vec()
	}
}

#[cfg(feature = "async")]
impl exports::rust2::wb::reads::Guest for Guest {
	async fn read(track: api::TrackBorrow<'_>) -> u64 {
		track.get::<Track>().0.read().await
	}
}

#[cfg(not(feature = "async"))]
export!(Guest);
#[cfg(feature = "async")]
export!(Guest);
