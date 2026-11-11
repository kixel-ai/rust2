//! One API for several targets: each target's, collected by its own check, merged into the
//! union of their items, each marked with the targets it compiles on.
//!
//! An item two targets both compile must be the same on each, since one binding declares
//! it once; one that differs, such as a function taking `u32` on one and `u64` on another,
//! a record with a field only one has, or a constant of another value, is refused at its
//! span, naming both targets. An enum's variants merge as its items do, each on its own
//! targets, so a value of a variant its target lacks is refused when it crosses.

use std::path::{Path, PathBuf};

use rustc_span::source_map::SourceMap;
use rustc_span::{DUMMY_SP, Span};

use crate::ir::{Api, Callback, Constant, Enum, Field, Function, Object, On, Record, TaskKind, Type, Variant};

/// A target's API, as a check writes it for the export of several targets.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Target {
	/// The facade's package, which every target's must share.
	pub package: String,
	/// The features the facade compiled with, which every target's must share, since the
	/// crate enables one set.
	pub features: Vec<String>,
	/// The config's `exclude`, which every target's must share.
	pub exclude: Vec<String>,
	pub api: Api,
}

impl Target {
	/// Write it to `path`, with each span as its file and offsets in `source_map`, the
	/// file relative to the first of `roots` it is in.
	pub fn write(&self, path: &Path, source_map: &SourceMap, roots: &[PathBuf]) -> Result<(), String> {
		let json = span::with(source_map, roots, || serde_json::to_string(self))
			.map_err(|err| format!("writing {}: {err}", path.display()))?;
		crate::driver::write(path, &json)
	}

	/// Read what [`Self::write`] wrote, with each span loaded into `source_map` from
	/// `roots`, or none where its file is not on this host as it was.
	pub fn read(path: &Path, source_map: &SourceMap, roots: &[PathBuf]) -> Result<Self, String> {
		let json = std::fs::read_to_string(path).map_err(|err| format!("reading {}: {err}", path.display()))?;
		span::with(source_map, roots, || serde_json::from_str(&json))
			.map_err(|err| format!("reading {}: {err}", path.display()))
	}
}

/// An item two targets compile differently, which one binding cannot serve: what it is,
/// then each target with its span there.
pub struct Differs {
	pub what: String,
	pub first: (String, Span),
	pub second: (String, Span),
}

/// The APIs of several targets, one each, merged into one serving them all, with every
/// item that differs between two of them.
pub fn merge(mut apis: Vec<Api>) -> (Api, Vec<Differs>) {
	apis.sort_by(|a, b| a.targets.cmp(&b.targets));
	let targets: Vec<String> = apis.iter().flat_map(|api| api.targets.clone()).collect();
	let mut apis = apis.into_iter().enumerate().map(|(i, mut api)| {
		mark(&mut api, On(1 << i));
		api
	});
	let mut merged = Merged {
		api: apis.next().expect("an export has a target"),
		differs: Vec::new(),
		renumbered: Vec::new(),
	};
	merged.api.targets = targets;
	for api in apis {
		merged.add(api);
	}
	merged.finish()
}

/// Mark every item of `api` as on `on`.
fn mark(api: &mut Api, on: On) {
	let functions = |methods: &mut [Function]| methods.iter_mut().for_each(|m| m.on = on);
	for record in &mut api.records {
		record.on = on;
		functions(&mut record.methods);
	}
	for e in &mut api.enums {
		e.on = on;
		e.variants.iter_mut().for_each(|v| v.on = on);
		functions(&mut e.methods);
	}
	for object in &mut api.objects {
		object.on = on;
		functions(&mut object.methods);
	}
	for callback in &mut api.callbacks {
		callback.on = on;
		functions(&mut callback.methods);
	}
	functions(&mut api.functions);
	api.constants.iter_mut().for_each(|c| c.on = on);
}

struct Merged {
	api: Api,
	differs: Vec<Differs>,
	/// Each variant whose discriminant differs between two targets, with its enum: an
	/// error only if the merged enum is plain, whose C values are its discriminants.
	renumbered: Vec<(usize, Differs)>,
}

/// Where each type of the API being added lands among the merged API's.
#[derive(Default)]
struct Maps {
	records: Vec<usize>,
	enums: Vec<usize>,
	objects: Vec<usize>,
	callbacks: Vec<usize>,
}

impl Maps {
	/// Each index `find` finds an equal item at, and past `len` for each it does not, in order.
	fn of<T>(items: &[T], len: usize, find: impl Fn(&T) -> Option<usize>) -> Vec<usize> {
		let mut next = len;
		(items.iter())
			.map(|item| {
				find(item).unwrap_or_else(|| {
					next += 1;
					next - 1
				})
			})
			.collect()
	}

	fn ty(&self, ty: &mut Type) {
		match ty {
			Type::Record(i) => *i = self.records[*i],
			Type::Enum(i) => *i = self.enums[*i],
			Type::Object(i) => *i = self.objects[*i],
			Type::Callback(i, _) => *i = self.callbacks[*i],
			Type::List(_, inner) | Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) => self.ty(inner),
			Type::Closure(params, ret) => {
				params.iter_mut().for_each(|p| self.ty(p));
				self.ty(ret);
			}
			Type::Tuple(elements) => elements.iter_mut().for_each(|e| self.ty(e)),
			_ => {}
		}
	}

	fn types(&self, types: &mut [Type]) {
		types.iter_mut().for_each(|ty| self.ty(ty));
	}

	fn fields(&self, fields: &mut [Field]) {
		fields.iter_mut().for_each(|f| self.ty(&mut f.ty));
	}

	fn function(&self, f: &mut Function) {
		f.params.iter_mut().for_each(|p| self.ty(&mut p.ty));
		self.ty(&mut f.ok);
		if let Some(err) = &mut f.err {
			self.ty(err);
		}
		if let Some(TaskKind::Made { object, .. }) = f.task.as_mut().map(|task| &mut task.kind) {
			*object = self.objects[*object];
		}
		if let Some(instance) = &mut f.instance {
			instance.args.iter_mut().flatten().for_each(|ty| self.ty(ty));
			self.types(&mut instance.named);
		}
	}

	fn functions(&self, functions: &mut [Function]) {
		functions.iter_mut().for_each(|f| self.function(f));
	}
}

/// `item` with what may differ between targets alike cleared: its span, docs, targets,
/// and `clear`'s, which clears its members, compared on their own.
fn bare<T: Clone>(item: &T, clear: impl Fn(&mut T)) -> T {
	let mut item = item.clone();
	clear(&mut item);
	item
}

fn bare_function(f: &Function) -> Function {
	bare(f, |f| {
		f.span = DUMMY_SP;
		f.docs.clear();
		f.on = On::ALL;
	})
}

fn bare_record(r: &Record) -> Record {
	bare(r, |r| {
		r.span = DUMMY_SP;
		r.docs.clear();
		r.on = On::ALL;
		r.methods.clear();
		r.fields.iter_mut().for_each(|f| f.docs.clear());
	})
}

fn bare_enum(e: &Enum) -> Enum {
	bare(e, |e| {
		e.span = DUMMY_SP;
		e.docs.clear();
		e.on = On::ALL;
		e.methods.clear();
		e.variants.clear();
	})
}

fn bare_variant(v: &Variant) -> Variant {
	bare(v, |v| {
		v.on = On::ALL;
		// Compared apart, as only a plain enum's matter.
		v.discriminant = 0;
		v.fields.iter_mut().for_each(|f| f.docs.clear());
	})
}

fn bare_object(o: &Object) -> Object {
	bare(o, |o| {
		o.span = DUMMY_SP;
		o.docs.clear();
		o.on = On::ALL;
		o.methods.clear();
	})
}

fn bare_callback(c: &Callback) -> Callback {
	bare(c, |c| {
		c.span = DUMMY_SP;
		c.docs.clear();
		c.on = On::ALL;
		c.methods = c.methods.iter().map(bare_function).collect();
	})
}

fn bare_constant(c: &Constant) -> Constant {
	bare(c, |c| {
		c.span = DUMMY_SP;
		c.docs.clear();
		c.on = On::ALL;
	})
}

impl Merged {
	/// The first target `on` names.
	fn first(&self, on: On) -> String {
		self.api.named(on).first().expect("an item is on a target").to_string()
	}

	/// Refuse `what`, which the merged API has on `ours` at `span` and `other` on its target
	/// at `theirs`.
	fn differ(&mut self, what: String, ours: (On, Span), theirs: (On, Span)) {
		let differs = self.differs(what, ours, theirs);
		self.differs.push(differs);
	}

	fn differs(&self, what: String, (ours, span): (On, Span), (other, theirs): (On, Span)) -> Differs {
		let first = (self.first(ours), span);
		let second = (self.first(other), theirs);
		Differs { what, first, second }
	}

	/// The merged API, and what differs between its targets, once every target is added.
	fn finish(mut self) -> (Api, Vec<Differs>) {
		let renumbered = std::mem::take(&mut self.renumbered);
		for (at, differs) in renumbered {
			if self.api.enums[at].plain() {
				self.differs.push(differs);
			}
		}
		// A C enum's values name its variants on every target alike.
		for e in &self.api.enums {
			if !e.plain() {
				continue;
			}
			let what = format!("enum `{}`", e.shown.join("::"));
			for (i, a) in e.variants.iter().enumerate() {
				if let Some(b) = e.variants[i + 1..].iter().find(|b| b.discriminant == a.discriminant) {
					let differs = self.differs(what.clone(), (a.on, e.span), (b.on, e.span));
					self.differs.push(differs);
					break;
				}
			}
		}
		self.api.crates = order_crates(&self.api);
		(self.api, self.differs)
	}

	/// Add `api`'s items, of one target, to the merged API's.
	fn add(&mut self, mut api: Api) {
		let maps = Maps {
			records: Maps::of(&api.records, self.api.records.len(), |r| {
				(self.api.records.iter()).position(|m| m.shown == r.shown)
			}),
			enums: Maps::of(&api.enums, self.api.enums.len(), |e| {
				(self.api.enums.iter()).position(|m| m.shown == e.shown)
			}),
			objects: Maps::of(&api.objects, self.api.objects.len(), |o| {
				(self.api.objects.iter()).position(|m| m.shown == o.shown && m.foreign == o.foreign)
			}),
			callbacks: Maps::of(&api.callbacks, self.api.callbacks.len(), |c| {
				(self.api.callbacks.iter()).position(|m| m.path == c.path)
			}),
		};
		for record in &mut api.records {
			maps.fields(&mut record.fields);
			maps.types(&mut record.args);
			maps.functions(&mut record.methods);
		}
		for e in &mut api.enums {
			e.variants.iter_mut().for_each(|v| maps.fields(&mut v.fields));
			maps.types(&mut e.args);
			maps.functions(&mut e.methods);
		}
		for object in &mut api.objects {
			maps.types(&mut object.args);
			maps.functions(&mut object.methods);
		}
		for callback in &mut api.callbacks {
			maps.functions(&mut callback.methods);
		}
		maps.functions(&mut api.functions);
		// In dependency order, as each list is: one only this target has goes before the
		// first crate after it here that the merged list has already.
		for (i, krate) in api.crates.iter().enumerate() {
			if self.api.crates.contains(krate) {
				continue;
			}
			let at = (api.crates[i + 1..].iter())
				.find_map(|next| self.api.crates.iter().position(|c| c == next))
				.unwrap_or(self.api.crates.len());
			self.api.crates.insert(at, krate.clone());
		}
		for (record, at) in api.records.into_iter().zip(maps.records) {
			let Some(ours) = self.api.records.get(at) else {
				self.api.records.push(record);
				continue;
			};
			if bare_record(ours) != bare_record(&record) {
				let what = format!("struct `{}`", record.shown.join("::"));
				self.differ(what, (ours.on, ours.span), (record.on, record.span));
			}
			self.api.records[at].on.0 |= record.on.0;
			let methods = std::mem::take(&mut self.api.records[at].methods);
			self.api.records[at].methods = self.functions(methods, record.methods);
		}
		for (e, at) in api.enums.into_iter().zip(maps.enums) {
			let Some(ours) = self.api.enums.get(at) else {
				self.api.enums.push(e);
				continue;
			};
			let what = format!("enum `{}`", e.shown.join("::"));
			if bare_enum(ours) != bare_enum(&e) {
				self.differ(what.clone(), (ours.on, ours.span), (e.on, e.span));
			}
			self.variants(at, &e, what);
			self.api.enums[at].on.0 |= e.on.0;
			let methods = std::mem::take(&mut self.api.enums[at].methods);
			self.api.enums[at].methods = self.functions(methods, e.methods);
		}
		for (object, at) in api.objects.into_iter().zip(maps.objects) {
			let Some(ours) = self.api.objects.get(at) else {
				self.api.objects.push(object);
				continue;
			};
			if bare_object(ours) != bare_object(&object) {
				let what = format!("type `{}`", object.shown.join("::"));
				self.differ(what, (ours.on, ours.span), (object.on, object.span));
			}
			self.api.objects[at].on.0 |= object.on.0;
			let methods = std::mem::take(&mut self.api.objects[at].methods);
			self.api.objects[at].methods = self.functions(methods, object.methods);
		}
		for (callback, at) in api.callbacks.into_iter().zip(maps.callbacks) {
			let Some(ours) = self.api.callbacks.get(at) else {
				self.api.callbacks.push(callback);
				continue;
			};
			if bare_callback(ours) != bare_callback(&callback) {
				let what = format!("trait `{}`", callback.path.join("::"));
				self.differ(what, (ours.on, ours.span), (callback.on, callback.span));
			}
			let ours = &mut self.api.callbacks[at];
			ours.on.0 |= callback.on.0;
			ours.methods.iter_mut().for_each(|m| m.on.0 |= callback.on.0);
		}
		let functions = std::mem::take(&mut self.api.functions);
		self.api.functions = self.functions(functions, api.functions);
		for constant in api.constants {
			let Some(at) = self.api.constants.iter().position(|c| c.path == constant.path) else {
				self.api.constants.push(constant);
				continue;
			};
			let ours = &self.api.constants[at];
			if bare_constant(ours) != bare_constant(&constant) {
				let what = format!("const `{}`", constant.path.join("::"));
				self.differ(what, (ours.on, ours.span), (constant.on, constant.span));
			}
			self.api.constants[at].on.0 |= constant.on.0;
		}
	}

	/// `theirs` merged into `ours`, each function matched by its path with its instance's
	/// arguments, and what it reads.
	fn functions(&mut self, mut ours: Vec<Function>, theirs: Vec<Function>) -> Vec<Function> {
		for function in theirs {
			// A field's getter and setter share its path.
			let Some(at) = ours
				.iter()
				.position(|f| f.shown == function.shown && f.access == function.access)
			else {
				ours.push(function);
				continue;
			};
			if bare_function(&ours[at]) != bare_function(&function) {
				let what = format!("fn `{}`", function.shown.join("::"));
				self.differ(what, (ours[at].on, ours[at].span), (function.on, function.span));
			}
			ours[at].on.0 |= function.on.0;
		}
		ours
	}

	/// The variants of `theirs` merged into the merged API's enum `at`, each after the one
	/// it follows in `theirs`.
	fn variants(&mut self, at: usize, theirs: &Enum, what: String) {
		let span = self.api.enums[at].span;
		let mut after = 0;
		for variant in &theirs.variants {
			let ours = &mut self.api.enums[at].variants;
			match ours.iter().position(|v| v.name == variant.name) {
				Some(found) => {
					let on = ours[found].on;
					if bare_variant(&ours[found]) != bare_variant(variant) {
						self.differ(what.clone(), (on, span), (variant.on, theirs.span));
					} else if ours[found].discriminant != variant.discriminant {
						let differs = self.differs(what.clone(), (on, span), (variant.on, theirs.span));
						self.renumbered.push((at, differs));
					}
					self.api.enums[at].variants[found].on.0 |= variant.on.0;
					after = found + 1;
				}
				None => {
					ours.insert(after, variant.clone());
					after += 1;
				}
			}
		}
	}
}

/// `api`'s crates in an order where each follows every crate whose types its items name,
/// on any target, as each target's order has it; else in the order they were merged in.
/// Two targets may order crates differently where neither names the other's types.
fn order_crates(api: &Api) -> Vec<String> {
	let crates = &api.crates;
	let index = |krate: &str| crates.iter().position(|c| c == krate);
	// Each crate's home, then a type its items name.
	let mut uses: Vec<(&str, &Type)> = Vec::new();
	fn functions<'a>(uses: &mut Vec<(&'a str, &'a Type)>, functions: &'a [Function]) {
		for f in functions {
			let types = f.params.iter().map(|p| &p.ty).chain([&f.ok]).chain(&f.err);
			uses.extend(types.map(|ty| (f.home.krate.as_str(), ty)));
		}
	}
	for r in &api.records {
		let types = r.fields.iter().map(|f| &f.ty).chain(&r.args);
		uses.extend(types.map(|ty| (r.home.krate.as_str(), ty)));
		functions(&mut uses, &r.methods);
	}
	for e in &api.enums {
		let fields = e.variants.iter().flat_map(|v| &v.fields);
		uses.extend(
			fields
				.map(|f| &f.ty)
				.chain(&e.args)
				.map(|ty| (e.home.krate.as_str(), ty)),
		);
		functions(&mut uses, &e.methods);
	}
	for o in &api.objects {
		uses.extend(o.args.iter().map(|ty| (o.home.krate.as_str(), ty)));
		functions(&mut uses, &o.methods);
	}
	for c in &api.callbacks {
		functions(&mut uses, &c.methods);
	}
	functions(&mut uses, &api.functions);
	let mut before = vec![std::collections::BTreeSet::new(); crates.len()];
	for (home, ty) in uses {
		let Some(home) = index(home) else { continue };
		crate::ir::each(ty, &mut |named| {
			let krate = match named {
				Type::Record(i) => &api.records[*i].home.krate,
				Type::Enum(i) => &api.enums[*i].home.krate,
				Type::Object(i) => &api.objects[*i].home.krate,
				Type::Callback(i, _) => &api.callbacks[*i].home.krate,
				_ => return,
			};
			match index(krate) {
				Some(named) if named != home => _ = before[home].insert(named),
				_ => {}
			}
		});
	}
	let mut placed = vec![false; crates.len()];
	let mut order = Vec::with_capacity(crates.len());
	while order.len() < crates.len() {
		let next = (0..crates.len())
			.find(|&i| !placed[i] && before[i].iter().all(|&j| placed[j]))
			.expect("crates whose items name each other's types, which Rust's dependencies cannot");
		placed[next] = true;
		order.push(crates[next].clone());
	}
	order
}

/// Spans as their file and byte offsets, which a later session loads that file for.
pub mod span {
	use std::cell::Cell;
	use std::path::{Path, PathBuf};

	use rustc_span::source_map::SourceMap;
	use rustc_span::{BytePos, DUMMY_SP, FileName, Span};
	use serde::{Deserialize, Deserializer, Serialize, Serializer};

	thread_local! {
		static CONTEXT: Cell<Option<(*const SourceMap, *const [PathBuf])>> = const { Cell::new(None) };
	}

	/// Run `f` with spans read and written through `source_map`, their files relative to
	/// `roots`.
	pub fn with<R>(source_map: &SourceMap, roots: &[PathBuf], f: impl FnOnce() -> R) -> R {
		let previous = CONTEXT.replace(Some((source_map, roots)));
		let result = f();
		CONTEXT.set(previous);
		result
	}

	fn context<R>(f: impl FnOnce(&SourceMap, &[PathBuf]) -> R) -> R {
		let (map, roots) = CONTEXT
			.get()
			.expect("a span is read or written only within `span::with`");
		// SAFETY: `with` sets them to references that outlive `f`'s run, then restores them.
		f(unsafe { &*map }, unsafe { &*roots })
	}

	/// A span's file, relative to the root at its index if it is in one, with `/` between
	/// its components whatever the host, else absolute; its offsets; and the file's length,
	/// which tells a file changed since, or checked out with other line endings.
	#[derive(Serialize, Deserialize)]
	struct Located {
		root: Option<usize>,
		file: PathBuf,
		lo: u32,
		hi: u32,
		len: u32,
	}

	/// `path` relative to the first of `roots` it is in.
	fn relative(path: &Path, roots: &[PathBuf]) -> Option<(usize, PathBuf)> {
		roots.iter().enumerate().find_map(|(i, root)| {
			let rest = path.strip_prefix(root).ok()?;
			let parts: Option<Vec<&str>> = rest.components().map(|c| c.as_os_str().to_str()).collect();
			Some((i, PathBuf::from(parts?.join("/"))))
		})
	}

	pub fn serialize<S: Serializer>(span: &Span, s: S) -> Result<S::Ok, S::Error> {
		let located = context(|map, roots| {
			let span = span.source_callsite();
			let file = map.lookup_source_file(span.lo());
			let FileName::Real(real) = &file.name else {
				return None;
			};
			let path = real.local_path()?;
			let (root, path) = match relative(path, roots) {
				Some((root, path)) => (Some(root), path),
				None => (None, path.to_path_buf()),
			};
			Some(Located {
				root,
				file: path,
				lo: (span.lo() - file.start_pos).0,
				hi: (span.hi() - file.start_pos).0,
				len: (file.end_position() - file.start_pos).0,
			})
		});
		located.serialize(s)
	}

	pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Span, D::Error> {
		let Some(located) = Option::<Located>::deserialize(d)? else {
			return Ok(DUMMY_SP);
		};
		Ok(context(|map, roots| {
			let path = match located.root {
				Some(root) => match roots.get(root) {
					Some(root) => root.join(&located.file),
					None => return DUMMY_SP,
				},
				None => located.file,
			};
			match map.load_file(&path) {
				Ok(file) if (file.end_position() - file.start_pos).0 == located.len && located.hi <= located.len => {
					Span::with_root_ctxt(
						file.start_pos + BytePos(located.lo),
						file.start_pos + BytePos(located.hi),
					)
				}
				// Its file is not on this host, or not as it was.
				_ => DUMMY_SP,
			}
		}))
	}
}

/// The path a target's API is staged at in `dir`.
pub fn staged(dir: &Path, target: &str) -> PathBuf {
	dir.join(format!("{target}.json"))
}
