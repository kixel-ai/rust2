//! The one entry point both sides of the differential test run: native Rust built for
//! wasm32 and run through wasm-bindgen's CLI, and its TypeScript translation, in one
//! browser. It reports each step as a line of text, so the two sides agree only if each
//! step does: closures called from JS, promises awaited and made, tasks spawned, and three
//! frames through web-codecs' `VideoEncoder`.

use std::cell::RefCell;
use std::fmt::Write;
use std::rc::Rc;

use js_sys::{Array, Function, Promise};
use wasm_bindgen::closure::ScopedClosure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{JsFuture, future_to_promise, spawn_local};

use crate::encoder::{Dimensions, Error, VideoEncodeOptions, VideoEncoderConfig};

/// Run every step, to a promise of the report.
pub fn run() -> Promise {
	future_to_promise(async {
		let mut out = String::new();
		closures(&mut out);
		futures(&mut out).await;
		encode(&mut out).await;
		failure(&mut out).await;
		Ok(JsValue::from(out))
	})
}

/// What JS threw or gave, as text the same on both sides: an error's name and message,
/// never its stack, which names where it was thrown.
fn text(v: &JsValue) -> String {
	match v.dyn_ref::<js_sys::Error>() {
		Some(e) => format!("{}: {}", String::from(e.name()), String::from(e.message())),
		None => format!("{v:?}"),
	}
}

/// What a call of a JS function gave, or threw.
fn called(r: Result<JsValue, JsValue>) -> String {
	match r {
		Ok(v) => format!("ok {}", text(&v)),
		Err(e) => format!("threw {}", text(&e)),
	}
}

/// The function a closure is, to call from Rust as JS does.
fn function<T: ?Sized>(c: &ScopedClosure<'_, T>) -> Function {
	c.as_ref().clone().unchecked_into()
}

/// Log `line`, made before the log is borrowed: what made it may log too.
fn note(log: &RefCell<Vec<String>>, line: String) {
	log.borrow_mut().push(line);
}

/// A value whose drop is logged, so the log shows when a closure holding it drops.
struct Noisy(Rc<RefCell<Vec<String>>>, &'static str);

impl Drop for Noisy {
	fn drop(&mut self) {
		self.0.borrow_mut().push(format!("drop {}", self.1));
	}
}

fn closures(out: &mut String) {
	let log = Rc::new(RefCell::new(Vec::new()));

	// An `FnMut` keeps its state between calls, and converts its argument and result.
	let mut count = 0;
	let counter = Closure::wrap(Box::new(move |x: u32| -> u32 {
		count += 1;
		x * 2 + count
	}) as Box<dyn FnMut(u32) -> u32>);
	let f = function(&counter);
	writeln!(out, "counter {}", called(f.call1(&JsValue::NULL, &3.into()))).unwrap();
	writeln!(out, "counter {}", called(f.call1(&JsValue::NULL, &4.into()))).unwrap();
	writeln!(out, "counter {}", called(f.call1(&JsValue::NULL, &"x".into()))).unwrap();
	drop(counter);
	writeln!(out, "dropped {}", called(f.call1(&JsValue::NULL, &3.into()))).unwrap();

	// An `FnMut` called again while it runs throws, and a `Fn` runs again.
	let again = Rc::new(RefCell::new(None::<Function>));
	let inner = again.clone();
	let reentrant = Closure::<dyn FnMut(u32) -> JsValue>::new(move |x: u32| -> JsValue {
		match x {
			0 => "bottom".into(),
			_ => called(inner.borrow().as_ref().unwrap().call1(&JsValue::NULL, &(x - 1).into())).into(),
		}
	});
	*again.borrow_mut() = Some(function(&reentrant));
	writeln!(
		out,
		"fn mut {}",
		called(function(&reentrant).call1(&JsValue::NULL, &1.into()))
	)
	.unwrap();
	let inner = again.clone();
	let shared = Closure::<dyn Fn(u32) -> JsValue>::new(move |x: u32| -> JsValue {
		match x {
			0 => "bottom".into(),
			_ => called(inner.borrow().as_ref().unwrap().call1(&JsValue::NULL, &(x - 1).into())).into(),
		}
	});
	*again.borrow_mut() = Some(function(&shared));
	writeln!(out, "fn {}", called(function(&shared).call1(&JsValue::NULL, &2.into()))).unwrap();
	*again.borrow_mut() = None;
	drop((reentrant, shared));

	// A closure that drops its own `Closure` while it runs drops what it owns once it returns.
	let held = Rc::new(RefCell::new(None::<Closure<dyn FnMut() -> String>>));
	let (inner, noisy, logged) = (held.clone(), Noisy(log.clone(), "self"), log.clone());
	*held.borrow_mut() = Some(Closure::new(move || {
		let _ = &noisy;
		drop(inner.borrow_mut().take());
		logged.borrow_mut().push("taken".into());
		"returned".to_string()
	}));
	let f = function(held.borrow().as_ref().unwrap());
	note(&log, called(f.call0(&JsValue::NULL)));
	note(&log, called(f.call0(&JsValue::NULL)));

	// `forget` and `into_js_value` keep a closure alive, and what it owns undropped.
	let noisy = Noisy(log.clone(), "forgotten");
	let forgotten = Closure::<dyn Fn() -> u32>::new(move || noisy.1.len() as u32);
	let f = function(&forgotten);
	forgotten.forget();
	note(&log, format!("forgotten {}", called(f.call0(&JsValue::NULL))));
	let noisy = Noisy(log.clone(), "leaked");
	let f: Function = Closure::<dyn Fn() -> u32>::new(move || noisy.1.len() as u32)
		.into_js_value()
		.unchecked_into();
	note(&log, format!("leaked {}", called(f.call0(&JsValue::NULL))));

	// A once closure runs once.
	let noisy = Noisy(log.clone(), "once");
	let once = Closure::once(move |s: String| -> String {
		let _ = &noisy;
		format!("{s}!")
	});
	let f = function(&once);
	note(&log, format!("once {}", called(f.call1(&JsValue::NULL, &"a".into()))));
	note(&log, format!("once {}", called(f.call1(&JsValue::NULL, &"b".into()))));
	drop(once);
	let noisy = Noisy(log.clone(), "once_into_js");
	let f: Function = Closure::once_into_js(move |x: f64| -> f64 {
		let _ = &noisy;
		x / 2.0
	})
	.unchecked_into();
	note(
		&log,
		format!("once_into_js {}", called(f.call1(&JsValue::NULL, &5.into()))),
	);
	note(
		&log,
		format!("once_into_js {}", called(f.call1(&JsValue::NULL, &5.into()))),
	);

	// A closure's `Err` is thrown, and its `Ok` returned.
	let fallible = Closure::<dyn Fn(i32) -> Result<i32, JsValue>>::new(|x: i32| match x {
		0 => Err(js_sys::Error::new("zero").into()),
		_ => Ok(100 / x),
	});
	let f = function(&fallible);
	writeln!(out, "fallible {}", called(f.call1(&JsValue::NULL, &4.into()))).unwrap();
	writeln!(out, "fallible {}", called(f.call1(&JsValue::NULL, &0.into()))).unwrap();
	drop(fallible);

	// A borrowed closure, valid until its `ScopedClosure` drops.
	let mut sum = 0;
	let f = {
		let mut add = |x: u32| sum += x;
		let scoped = ScopedClosure::borrow_mut(&mut add);
		let f = function(&scoped);
		writeln!(out, "borrowed {}", called(f.call1(&JsValue::NULL, &5.into()))).unwrap();
		f
	};
	writeln!(out, "borrowed {}", called(f.call1(&JsValue::NULL, &6.into()))).unwrap();
	writeln!(out, "sum {sum}").unwrap();

	// A closure an import borrows only for the call, whose arguments convert from JS.
	let items = Array::of3(&1.into(), &"two".into(), &3.5.into());
	let mut seen = Vec::new();
	items.for_each(&mut |v, i, all| seen.push(format!("{i}:{}/{}", text(&v), all.length())));
	writeln!(out, "for_each {}", seen.join(" ")).unwrap();

	for line in log.borrow().iter() {
		writeln!(out, "log {line}").unwrap();
	}
}

async fn futures(out: &mut String) {
	// A promise's value and its rejection, awaited.
	let resolved = JsFuture::from(Promise::resolve(&JsValue::from(5))).await;
	writeln!(out, "resolved {}", called(resolved)).unwrap();
	let rejected = JsFuture::from(Promise::reject(&js_sys::Error::new("no").into())).await;
	writeln!(out, "rejected {}", called(rejected)).unwrap();
	let awaited = Promise::resolve(&JsValue::from("into")).await;
	writeln!(out, "awaited {}", called(awaited)).unwrap();

	// A future's promise settles with its result.
	let ok = JsFuture::from(future_to_promise(async { Ok(JsValue::from(7)) })).await;
	writeln!(out, "future ok {}", called(ok)).unwrap();
	let err = JsFuture::from(future_to_promise(async { Err(JsValue::from("bad")) })).await;
	writeln!(out, "future err {}", called(err)).unwrap();

	// Spawned tasks, which run in order with this one, on the microtasks they queue.
	let log = Rc::new(RefCell::new(Vec::new()));
	for name in ["a", "b"] {
		let log = log.clone();
		spawn_local(async move {
			note(&log, format!("{name}1"));
			let _ = JsFuture::from(Promise::resolve(&JsValue::NULL)).await;
			note(&log, format!("{name}2"));
			let _ = JsFuture::from(Promise::resolve(&JsValue::NULL)).await;
			note(&log, format!("{name}3"));
		});
	}
	log.borrow_mut().push("m1".into());
	for _ in 0..3 {
		let _ = JsFuture::from(Promise::resolve(&JsValue::NULL)).await;
		log.borrow_mut().push("m".into());
	}
	writeln!(out, "tasks {}", log.borrow().join(" ")).unwrap();
}

/// What each step of an encode gave, or why it failed.
fn shown<T: std::fmt::Debug>(r: &Result<T, Error>) -> String {
	match r {
		Err(Error::Unknown(e)) => format!("Err({})", text(e)),
		r => format!("{r:?}"),
	}
}

/// A 64x64 RGBA frame of a gradient that moves with `i`.
fn frame(i: u32) -> web_sys::VideoFrame {
	let mut data = Vec::with_capacity(64 * 64 * 4);
	for y in 0..64u32 {
		for x in 0..64u32 {
			data.extend([(x * 4 + i * 8) as u8, (y * 4) as u8, ((x + y) * 2) as u8, 255]);
		}
	}
	let init = web_sys::VideoFrameBufferInit::new(64, 64, web_sys::VideoPixelFormat::Rgba, (i * 33_333) as i32);
	web_sys::VideoFrame::new_with_u8_slice_and_video_frame_buffer_init(&mut data, &init).unwrap()
}

async fn encode(out: &mut String) {
	let mut config = VideoEncoderConfig::new("vp8", Dimensions::new(64, 64));
	config.bitrate = Some(200_000);
	config.framerate = Some(30.0);
	writeln!(out, "valid {:?}", config.is_valid()).unwrap();
	writeln!(out, "supported {}", shown(&config.is_supported().await)).unwrap();
	let unsupported = VideoEncoderConfig::new("unsupported", Dimensions::new(64, 64));
	writeln!(out, "unsupported {}", shown(&unsupported.is_supported().await)).unwrap();

	let (mut encoder, mut encoded) = config.init().unwrap();
	writeln!(out, "codec {}", encoder.config().codec).unwrap();
	for i in 0..3 {
		let frame = frame(i);
		let options = VideoEncodeOptions {
			key_frame: Some(i == 0),
		};
		writeln!(out, "encode {} {}", i, shown(&encoder.encode(&frame, options))).unwrap();
		frame.close();
	}
	writeln!(out, "queued {}", encoder.queue_size() <= 3).unwrap();
	writeln!(out, "flush {}", shown(&encoder.flush().await)).unwrap();
	writeln!(out, "config {:?}", encoded.config()).unwrap();
	// Dropped, the encoder closes and drops its closures, whose sender then closes the channel.
	drop(encoder);
	loop {
		match encoded.frame().await {
			Ok(Some(frame)) => {
				let hex: String = frame.payload.iter().map(|b| format!("{b:02x}")).collect();
				writeln!(out, "frame {frame:?} {hex}").unwrap();
			}
			r => {
				writeln!(out, "end {}", shown(&r)).unwrap();
				break;
			}
		}
	}
}

/// An encoder the browser rejects the config of, through its error callback.
async fn failure(out: &mut String) {
	let config = VideoEncoderConfig::new("bogus", Dimensions::new(64, 64));
	let (mut encoder, mut encoded) = config.init().unwrap();
	writeln!(out, "failed {}", shown(&encoded.frame().await)).unwrap();
	let frame = frame(0);
	writeln!(
		out,
		"encode after {}",
		shown(&encoder.encode(&frame, VideoEncodeOptions::default()))
	)
	.unwrap();
	frame.close();
}
