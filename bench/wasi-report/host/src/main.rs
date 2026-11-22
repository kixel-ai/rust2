//! What a call costs a wasmtime host: wb's shapes through the exporter's C ABI in a core
//! module, through a component's canonical ABI, and in Rust itself.

use std::hint::black_box;
use std::time::Instant;

use wasmtime::component::{self as cm, ComponentType, Lift, ResourceAny, ResourceTable};
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::p1::WasiP1Ctx;
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};

const N: u32 = 1_000_000;

/// The core module's `wb_Track_start`, which the host's closure calls back.
static START: std::sync::OnceLock<wasmtime::TypedFunc<i32, i64>> = std::sync::OnceLock::new();

/// Nanoseconds a call, the second of two runs so both warmed up.
fn time(name: &str, f: impl FnMut()) {
	time_n(name, N, f)
}

/// [`time`] over `n` calls.
fn time_n(name: &str, n: u32, mut f: impl FnMut()) {
	let mut ns = 0.0;
	for _ in 0..2 {
		let start = Instant::now();
		for _ in 0..n {
			f();
		}
		ns = start.elapsed().as_nanos() as f64 / f64::from(n);
	}
	println!("{name}\t{ns:.1}");
}

fn native() {
	println!("# rust");
	let track = wb::Track::new();
	let frame = [1u8; 100];
	time("noop", || wb::noop());
	time("handle call", || {
		black_box(black_box(&track).is_closed());
	});
	time("u64 getter", || {
		black_box(black_box(&track).start());
	});
	time("record return", || {
		black_box(black_box(&track).largest());
	});
	time("object return", || {
		black_box(black_box(&track).info());
	});
	time("write 100 B", || black_box(&track).write(black_box(&frame)));
	time("payload 100 B", || {
		black_box(black_box(&track).payload().to_vec());
	});
}

fn core(path: &str) -> wasmtime::Result<()> {
	println!("# core module, wasm32-wasip1, the exporter's C ABI");
	let engine = Engine::default();
	let module = Module::from_file(&engine, path)?;
	let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
	wasmtime_wasi::p1::add_to_linker_sync(&mut linker, |cx| cx)?;
	// What the host implements for wakers, traits, closures, and readers: unused here.
	// A closure the host lends: its token is the track whose start it reads, calling back
	// into the module, unless it is 0.
	linker.func_wrap("rust2", "wb_web_i_fn_0", |mut caller: wasmtime::Caller<'_, WasiP1Ctx>, track: i32| -> i64 {
		if track == 0 {
			return 1;
		}
		let start = START.get().unwrap();
		start.call(&mut caller, track).unwrap()
	})?;
	linker.func_wrap("rust2", "rust2_wake", |_: i32| {})?;
	linker.func_wrap("rust2", "rust2_waker_drop", |_: i32| {})?;
	linker.func_wrap("rust2", "rust2_free", |_: i32| {})?;
	linker.func_wrap("rust2", "rust2_future_poll", |_: i32, _: i32| 0i32)?;
	linker.func_wrap("rust2", "rust2_reader_read", |_: i32, _: i32, _: i32, _: i32, _: i32| 0i32)?;
	linker.func_wrap("rust2", "rust2_reader_seek", |_: i32, _: i32, _: i64, _: i32, _: i32| 0i32)?;
	let wasi = wasmtime_wasi::WasiCtxBuilder::new().inherit_stdio().build_p1();
	let mut store = Store::new(&engine, wasi);
	let instance = linker.instantiate(&mut store, &module)?;
	instance.get_typed_func::<(), ()>(&mut store, "_initialize")?.call(&mut store, ())?;
	let memory = instance.get_memory(&mut store, "memory").unwrap();
	let f = |store: &mut Store<WasiP1Ctx>, name: &str| instance.get_func(&mut *store, name).unwrap();
	let alloc = f(&mut store, "wb_web_alloc").typed::<(i32, i32), i32>(&store)?;
	let noop = f(&mut store, "wb_noop").typed::<(), ()>(&store)?;
	let new = f(&mut store, "wb_Track_new").typed::<(), i32>(&store)?;
	let is_closed = f(&mut store, "wb_Track_is_closed").typed::<i32, i32>(&store)?;
	let start = f(&mut store, "wb_Track_start").typed::<i32, i64>(&store)?;
	START.set(start.clone()).ok();
	let largest = f(&mut store, "wb_Track_largest").typed::<(i32, i32), ()>(&store)?;
	let info = f(&mut store, "wb_Track_info").typed::<i32, i32>(&store)?;
	let info_free = f(&mut store, "wb_Info_free").typed::<i32, ()>(&store)?;
	let write = f(&mut store, "wb_Track_write").typed::<(i32, i32), ()>(&store)?;
	let payload = f(&mut store, "wb_Track_payload").typed::<(i32, i32), ()>(&store)?;
	let buf_free = f(&mut store, "wb_buf_free").typed::<i32, ()>(&store)?;
	let read_start = f(&mut store, "wb_Track_read_start").typed::<(i32, i32, i32), i32>(&store)?;
	let big = f(&mut store, "wb_Track_big").typed::<(i32, i32, i32), ()>(&store)?;
	let visit = f(&mut store, "wb_visit").typed::<(i32, i32), i64>(&store)?;
	let fn_0 = f(&mut store, "wb_web_fn_0").typed::<(), i32>(&store)?;
	let boom = f(&mut store, "wb_boom").typed::<(), ()>(&store)?;

	// The binding's stack in linear memory, as the web binding keeps one.
	let stack = alloc.call(&mut store, (4096, 8))?;
	let track = new.call(&mut store, ())?;
	let frame = [1u8; 100];
	assert_eq!(start.call(&mut store, track)?, 7);

	time("noop", || noop.call(&mut store, ()).unwrap());
	time("handle call", || {
		black_box(is_closed.call(&mut store, track).unwrap() != 0);
	});
	time("u64 getter", || {
		black_box(start.call(&mut store, track).unwrap());
	});
	time("record return", || {
		largest.call(&mut store, (stack, track)).unwrap();
		let mut out = [0u8; 24];
		memory.read(&store, stack as usize, &mut out).unwrap();
		black_box(out);
	});
	time("object return", || {
		let h = info.call(&mut store, track).unwrap();
		info_free.call(&mut store, h).unwrap();
	});
	time("write 100 B", || {
		// The frame copied in, then a view of it: { ptr, len, owner = NULL }.
		let data = memory.data_mut(&mut store);
		let at = stack as usize + 16;
		data[at..at + 100].copy_from_slice(black_box(&frame));
		let mut view = [0u8; 12];
		view[..4].copy_from_slice(&(at as u32).to_le_bytes());
		view[4..8].copy_from_slice(&100u32.to_le_bytes());
		data[stack as usize..stack as usize + 12].copy_from_slice(&view);
		write.call(&mut store, (track, stack)).unwrap();
	});
	time("payload 100 B", || {
		payload.call(&mut store, (stack, track)).unwrap();
		let data = memory.data(&store);
		let at = stack as usize;
		let ptr = u32::from_le_bytes(data[at..at + 4].try_into().unwrap()) as usize;
		let len = u32::from_le_bytes(data[at + 4..at + 8].try_into().unwrap()) as usize;
		black_box(data[ptr..ptr + len].to_vec());
		buf_free.call(&mut store, stack).unwrap();
	});
	time("payload 100 B lent", || {
		// Read in place, as the web binding's `rt.Bytes` lends past 64 bytes.
		payload.call(&mut store, (stack, track)).unwrap();
		let data = memory.data(&store);
		let at = stack as usize;
		let ptr = u32::from_le_bytes(data[at..at + 4].try_into().unwrap()) as usize;
		black_box(data[ptr]);
		buf_free.call(&mut store, stack).unwrap();
	});
	time("ready read", || {
		let poll = read_start.call(&mut store, (track, stack, stack + 8)).unwrap();
		assert_eq!(poll, 1);
		let mut out = [0u8; 8];
		memory.read(&store, stack as usize, &mut out).unwrap();
		black_box(out);
	});
	let shim = fn_0.call(&mut store, ())?;
	time("lent closure", || {
		assert_eq!(visit.call(&mut store, (shim, 0)).unwrap(), 2);
	});
	time("lent closure calling back", || {
		assert_eq!(visit.call(&mut store, (shim, track)).unwrap(), 8);
	});
	for len in [1u32 << 10, 1 << 20] {
		time_n(&format!("payload {len} B"), N >> (len.ilog2() / 2), || {
			big.call(&mut store, (stack, track, len as i32)).unwrap();
			let data = memory.data(&store);
			let at = stack as usize;
			let ptr = u32::from_le_bytes(data[at..at + 4].try_into().unwrap()) as usize;
			let n = u32::from_le_bytes(data[at + 4..at + 8].try_into().unwrap()) as usize;
			black_box(data[ptr..ptr + n].to_vec());
			buf_free.call(&mut store, stack).unwrap();
		});
		time_n(&format!("payload {len} B lent"), N >> (len.ilog2() / 2), || {
			big.call(&mut store, (stack, track, len as i32)).unwrap();
			let data = memory.data(&store);
			let at = stack as usize;
			let ptr = u32::from_le_bytes(data[at..at + 4].try_into().unwrap()) as usize;
			black_box(data[ptr]);
			buf_free.call(&mut store, stack).unwrap();
		});
	}
	let trapped = boom.call(&mut store, ());
	println!("panic: {}", trapped.is_err());
	println!("call after panic: {:?}", noop.call(&mut store, ()).map_err(|e| e.to_string()));
	Ok(())
}

struct Ctx {
	wasi: WasiCtx,
	table: ResourceTable,
	/// What the host's closure calls back into, and on which track, if it does.
	back: Option<(cm::TypedFunc<(ResourceAny,), (u64,)>, ResourceAny)>,
}

/// The host's closure, a resource the component imports.
struct Visit;

impl WasiView for Ctx {
	fn ctx(&mut self) -> WasiCtxView<'_> {
		WasiCtxView { ctx: &mut self.wasi, table: &mut self.table }
	}
}

fn ctx() -> Ctx {
	Ctx { wasi: WasiCtx::builder().inherit_stdio().build(), table: ResourceTable::new(), back: None }
}

/// The host's side of interface `callback`: `visit.call` returns 1, or calls back.
fn callback(linker: &mut cm::Linker<Ctx>) -> wasmtime::Result<()> {
	let mut callback = linker.instance("rust2:wb/callback")?;
	callback.resource("visit", cm::ResourceType::host::<Visit>(), |_, _| Ok(()))?;
	callback.func_wrap(
		"[method]visit.call",
		|mut store: wasmtime::StoreContextMut<'_, Ctx>, (_,): (cm::Resource<Visit>,)| -> wasmtime::Result<(u64,)> {
			match store.data().back {
				None => Ok((1,)),
				Some((start, track)) => start.call(&mut store, (track,)),
			}
		},
	)?;
	Ok(())
}

/// `span`, read only through `black_box`.
#[allow(dead_code)]
#[derive(ComponentType, Lift, Clone, Copy, Debug)]
#[component(record)]
struct Span {
	start: u64,
	end: u64,
}

/// The component's functions of interface `api`.
struct Api {
	noop: cm::TypedFunc<(), ()>,
	new: cm::TypedFunc<(), (ResourceAny,)>,
	is_closed: cm::TypedFunc<(ResourceAny,), (bool,)>,
	start: cm::TypedFunc<(ResourceAny,), (u64,)>,
	largest: cm::TypedFunc<(ResourceAny,), (Option<Span>,)>,
	info: cm::TypedFunc<(ResourceAny,), (ResourceAny,)>,
	write: cm::Func,
	payload: cm::TypedFunc<(ResourceAny,), (Vec<u8>,)>,
	big: cm::TypedFunc<(ResourceAny, u32), (Vec<u8>,)>,
	visit: cm::TypedFunc<(cm::Resource<Visit>,), (u64,)>,
	boom: cm::TypedFunc<(), ()>,
}

fn api(store: &mut Store<Ctx>, instance: &cm::Instance) -> wasmtime::Result<Api> {
	let iface = instance.get_export_index(&mut *store, None, "rust2:wb/api").unwrap();
	let mut func = |name: &str| {
		let idx = instance.get_export_index(&mut *store, Some(&iface), name).unwrap();
		instance.get_func(&mut *store, idx).unwrap()
	};
	let noop = func("noop");
	let new = func("[constructor]track");
	let is_closed = func("[method]track.is-closed");
	let start = func("[method]track.start");
	let largest = func("[method]track.largest");
	let info = func("[method]track.info");
	let write = func("[method]track.write");
	let payload = func("[method]track.payload");
	let big = func("[method]track.big");
	let visit = func("visit-with");
	let boom = func("boom");
	Ok(Api {
		noop: noop.typed(&*store)?,
		new: new.typed(&*store)?,
		is_closed: is_closed.typed(&*store)?,
		start: start.typed(&*store)?,
		largest: largest.typed(&*store)?,
		info: info.typed(&*store)?,
		write,
		payload: payload.typed(&*store)?,
		big: big.typed(&*store)?,
		visit: visit.typed(&*store)?,
		boom: boom.typed(&*store)?,
	})
}

fn component_sync(path: &str, async_abi: bool) -> wasmtime::Result<()> {
	println!(
		"# component, wasm32-wasip2, sync calls, component-model-async {}",
		if async_abi { "on" } else { "off" }
	);
	let mut config = Config::new();
	config.wasm_component_model_async(async_abi);
	let engine = Engine::new(&config)?;
	let component = cm::Component::from_file(&engine, path)?;
	let mut linker = cm::Linker::<Ctx>::new(&engine);
	wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
	callback(&mut linker)?;
	let mut store = Store::new(&engine, ctx());
	let instance = linker.instantiate(&mut store, &component)?;
	let api = api(&mut store, &instance)?;
	let write = api.write.typed::<(ResourceAny, &[u8]), ()>(&store)?;
	let (track,) = api.new.call(&mut store, ())?;
	let frame = [1u8; 100];
	assert_eq!(api.start.call(&mut store, (track,))?.0, 7);

	time("noop", || api.noop.call(&mut store, ()).unwrap());
	time("handle call", || {
		black_box(api.is_closed.call(&mut store, (track,)).unwrap());
	});
	time("u64 getter", || {
		black_box(api.start.call(&mut store, (track,)).unwrap());
	});
	time("record return", || {
		black_box(api.largest.call(&mut store, (track,)).unwrap());
	});
	time("object return", || {
		let (info,) = api.info.call(&mut store, (track,)).unwrap();
		info.resource_drop(&mut store).unwrap();
	});
	time("write 100 B", || write.call(&mut store, (track, black_box(&frame[..]))).unwrap());
	time("payload 100 B", || {
		black_box(api.payload.call(&mut store, (track,)).unwrap());
	});
	time("lent closure", || {
		assert_eq!(api.visit.call(&mut store, (cm::Resource::new_borrow(0),)).unwrap().0, 2);
	});
	for len in [1u32 << 10, 1 << 20] {
		time_n(&format!("payload {len} B"), N >> (len.ilog2() / 2), || {
			black_box(api.big.call(&mut store, (track, len)).unwrap());
		});
	}
	store.data_mut().back = Some((api.start, track));
	println!(
		"lent closure calling back: {:?}",
		api.visit.call(&mut store, (cm::Resource::new_borrow(0),)).map_err(|e| format!("{e:#}"))
	);
	store.data_mut().back = None;
	println!("panic: {}", api.boom.call(&mut store, ()).is_err());
	println!("call after panic: {:?}", api.noop.call(&mut store, ()).map_err(|e| format!("{e:#}")));
	Ok(())
}

async fn component_p3(path: &str) -> wasmtime::Result<()> {
	println!("# component, wasm32-wasip3, component-model-async on");
	let mut config = Config::new();
	config.wasm_component_model_async(true);
	let engine = Engine::new(&config)?;
	let component = cm::Component::from_file(&engine, path)?;
	let mut linker = cm::Linker::<Ctx>::new(&engine);
	wasmtime_wasi::p3::add_to_linker(&mut linker)?;
	callback(&mut linker)?;
	let mut store = Store::new(&engine, ctx());
	let instance = linker.instantiate_async(&mut store, &component).await?;
	let api = api(&mut store, &instance)?;
	let reads = instance.get_export_index(&mut store, None, "rust2:wb/reads").unwrap();
	let read = instance.get_export_index(&mut store, Some(&reads), "read").unwrap();
	let read = instance.get_func(&mut store, read).unwrap().typed::<(ResourceAny,), (u64,)>(&store)?;
	let (track,) = api.new.call_async(&mut store, ()).await?;

	// A sync function on a store whose host is async: each call through `call_async`.
	for round in 0..2 {
		let start = Instant::now();
		for _ in 0..N {
			black_box(api.start.call_async(&mut store, (track,)).await?);
		}
		if round == 1 {
			println!("u64 getter (call_async)\t{:.1}", start.elapsed().as_nanos() as f64 / f64::from(N));
		}
	}

	// An async function ready at its first poll, through `call_concurrent`.
	store
		.run_concurrent(async |accessor| -> wasmtime::Result<()> {
			for round in 0..2 {
				let start = Instant::now();
				for _ in 0..N {
					let (v,) = read.call_concurrent(accessor, (track,)).await?;
					black_box(v);
				}
				let ns = start.elapsed().as_nanos() as f64 / f64::from(N);
				if round == 1 {
					println!("ready read (call_concurrent)\t{ns:.1}");
				}
			}
			Ok(())
		})
		.await??;
	Ok(())
}

fn main() -> wasmtime::Result<()> {
	let args: Vec<String> = std::env::args().collect();
	native();
	core(&args[1])?;
	component_sync(&args[2], false)?;
	component_sync(&args[2], true)?;
	tokio::runtime::Builder::new_current_thread().enable_all().build()?.block_on(component_p3(&args[3]))?;
	Ok(())
}
