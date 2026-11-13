//! The API lock: the export's API checked against a baseline an earlier export wrote, so a
//! break of it fails the export once, in the API every binding is generated from, rather
//! than once per language.
//!
//! An item is known by its kind and Rust path, as the config names it. One the baseline has
//! that the export lacks, or has on fewer of the baseline's targets, is removed; one whose
//! signature, fields, or variants differ is changed; and one whose C name differs is renamed,
//! as an item added beside it makes it when their names clash, since a C caller of the old
//! name then calls the new item. A tuple's struct is known by its Rust spelling, and is
//! renamed the same way. An addition passes: a new item, method, trait impl, or a
//! variant of an enum another crate cannot match exhaustively.

use std::collections::{BTreeMap, BTreeSet};

use rustc_span::{DUMMY_SP, Span};

use crate::abi::{Abi, Ty};
use crate::ir::{Access, Api, Constant, Function, Literal, On, Traits, Type};
use crate::merge::{self, Maps};

/// Each language's public names, keyed by the item's kind and Rust path.
pub type Names = BTreeMap<String, BTreeMap<String, String>>;

/// A generator's names for exported types, translated from C keys to stable item keys.
pub fn types(abi: &Abi<'_>, classes: &std::collections::HashMap<String, String>) -> BTreeMap<String, String> {
	let api = abi.api;
	let records = api.records.iter().enumerate().map(|(i, r)| (abi.record(i), &r.shown));
	let enums = api
		.enums
		.iter()
		.enumerate()
		.map(|(i, e)| (abi.enumeration(i), &e.shown));
	let objects = api.objects.iter().enumerate().map(|(i, o)| (abi.object(i), &o.shown));
	let callbacks = api
		.callbacks
		.iter()
		.enumerate()
		.map(|(i, c)| (abi.callback(i), &c.path));
	records
		.chain(enums)
		.chain(objects)
		.chain(callbacks)
		.filter_map(|(c, path)| classes.get(&c).map(|name| (type_key(path), name.clone())))
		.collect()
}

/// Record names made for enum variants by a generator's collision rules.
///
/// `variant` gives a variant's name by its enum's C name and its Rust name, if it has one.
pub fn variants(abi: &Abi<'_>, names: &mut BTreeMap<String, String>, variant: impl Fn(&str, &str) -> Option<String>) {
	for (i, e) in abi.api.enums.iter().enumerate() {
		for v in &e.variants {
			if let Some(name) = variant(&abi.enumeration(i), &v.name) {
				names.insert(format!("{} / variant {}", type_key(&e.shown), v.name), name);
			}
		}
	}
}

/// Record the final public name of a function or field accessor.
pub fn function(names: &mut BTreeMap<String, String>, function: &Function, name: &str) {
	names.insert(function_key(function), name.to_string());
}

/// The shared exception identity after references and borrowed strings lose ownership.
pub fn error_key(abi: &Abi<'_>, ty: &Type) -> String {
	fn owned(ty: &Type) -> Type {
		match ty {
			Type::Ref(_, inner) => owned(inner),
			Type::Str => Type::String,
			Type::Option(inner) => Type::Option(Box::new(owned(inner))),
			Type::Box(inner) => Type::Box(Box::new(owned(inner))),
			Type::List(c, inner) => Type::List(*c, Box::new(owned(inner))),
			Type::Tuple(elements) => Type::Tuple(elements.iter().map(owned).collect()),
			ty => ty.clone(),
		}
	}
	format!("error {}", abi.spell(&owned(ty)))
}

/// Compare the names the generators actually emitted, retaining the current item's span.
///
/// An item whose C name `base_api`, the baseline's API, gave it changed is already a structural
/// break, so its binding names, which follow from the C name, are not reported again.
pub fn name_breaks(base: &Names, now: &Names, base_api: &Api, abi: &Abi<'_>) -> Vec<Break> {
	let api = abi.api;
	let index = Index::with(abi);
	let renamed: BTreeSet<String> = Index::new(base_api, &abi.prefix)
		.entries
		.into_iter()
		.filter(|(key, entry)| index.entries.get(key).is_some_and(|now| now.c != entry.c))
		.map(|(key, _)| key)
		.collect();
	let mut locations: BTreeMap<String, (Span, String)> = index
		.entries
		.iter()
		.map(|(key, entry)| (key.clone(), (entry.span, entry.what.clone())))
		.collect();
	for (spelled, (_, span)) in &index.tuples {
		locations.insert(format!("tuple {spelled}"), (*span, format!("tuple `{spelled}`")));
	}
	for callback in &api.callbacks {
		for method in &callback.methods {
			locations.insert(
				function_key(method),
				(method.span, format!("fn `{}`", method.shown.join("::"))),
			);
		}
	}
	let functions = api
		.functions
		.iter()
		.chain(api.records.iter().flat_map(|r| &r.methods))
		.chain(api.enums.iter().flat_map(|e| &e.methods))
		.chain(api.objects.iter().flat_map(|o| &o.methods));
	for function in functions {
		let mut record = |err: &Type| {
			let key = error_key(abi, err);
			locations.entry(key.clone()).or_insert((
				function.span,
				format!("exception class for `{}`", key.strip_prefix("error ").unwrap()),
			));
		};
		if let Some(err) = &function.err {
			record(err);
		}
		for param in &function.params {
			crate::ir::each(&param.ty, &mut |ty| {
				if let Type::Closure(closure) = ty
					&& let Some(err) = &closure.err
				{
					record(err);
				}
			});
		}
	}
	let homes = api
		.records
		.iter()
		.map(|r| (&r.home, r.span))
		.chain(api.enums.iter().map(|e| (&e.home, e.span)))
		.chain(api.objects.iter().map(|o| (&o.home, o.span)))
		.chain(api.callbacks.iter().map(|c| (&c.home, c.span)))
		.chain(api.functions.iter().map(|f| (&f.home, f.span)))
		.chain(api.constants.iter().map(|c| (&c.home, c.span)));
	for krate in &api.crates {
		locations.insert(format!("crate {krate}"), (DUMMY_SP, format!("crate `{krate}`")));
	}
	for (home, span) in homes {
		locations.insert(
			format!("crate {}", home.krate),
			(span, format!("crate `{}`", home.krate)),
		);
	}
	let mut breaks = Vec::new();
	for (language, names) in base {
		for (key, old) in names {
			let owner = key.split_once(" / ").map_or(key.as_str(), |(owner, _)| owner);
			let Some((span, what)) = locations.get(owner) else {
				continue;
			};
			if renamed.contains(owner) {
				continue;
			}
			let new = now.get(language).and_then(|names| names.get(key));
			if new != Some(old) {
				let change = match new {
					Some(new) => format!("is now `{new}`"),
					None => "is no longer exported".to_string(),
				};
				breaks.push(Break {
					span: *span,
					what: format!(
						"{}{}: its {language} name `{old}` {change}",
						what,
						key.strip_prefix(owner).unwrap_or("")
					),
				});
			}
		}
	}
	breaks
}

/// An item the export's API breaks the baseline's at: where, and how.
pub struct Break {
	/// The item's span in the export, or the baseline's if the export lacks it, or none.
	pub span: Span,
	/// The item, then each way it breaks.
	pub what: String,
}

/// Every break of `base`, the baseline, by `now`, the export's API, both prefixed `prefix`.
pub fn breaks(base: &Api, now: &Api, prefix: &str) -> Vec<Break> {
	// Every path starts with the facade's root, which prefixes every C name.
	let root = (base.records.iter().map(|r| &r.shown))
		.chain(base.enums.iter().map(|e| &e.shown))
		.chain(base.objects.iter().map(|o| &o.shown))
		.chain(base.functions.iter().map(|f| &f.shown))
		.chain(base.constants.iter().map(|c| &c.path))
		.chain(base.callbacks.iter().map(|c| &c.path))
		.find_map(|path| path.first());
	if let Some(root) = root.filter(|root| *root != prefix) {
		return vec![Break {
			span: DUMMY_SP,
			what: format!("library `{root}`: it is now `{prefix}`, which every path and C name starts with"),
		}];
	}
	let base = Index::new(base, prefix);
	let now = Index::new(now, prefix);
	let mut breaks = Vec::new();
	let dropped: Vec<&String> = base
		.api
		.targets
		.iter()
		.filter(|t| !now.api.targets.contains(t))
		.collect();
	for target in &dropped {
		breaks.push(Break {
			span: DUMMY_SP,
			what: format!("target `{target}`: the export no longer serves it; give it as a --target"),
		});
	}
	let maps = base.maps(&now);
	for (key, ours) in &base.entries {
		let Some(theirs) = now.entries.get(key) else {
			let mut what = format!("{}: it is removed", ours.what);
			if let Some(other) = now.named(&ours.c) {
				what += &format!(", and its C name `{}` names {}", ours.c, other.what);
			}
			// Where it was, else the type it was a member of.
			let span = match ours.span.is_dummy() {
				false => ours.span,
				true => (ours.owner.as_ref().and_then(|owner| now.entries.get(owner))).map_or(DUMMY_SP, |o| o.span),
			};
			breaks.push(Break { span, what });
			continue;
		};
		let mut changes = Vec::new();
		let lost: Vec<&str> = (base.api.named(ours.on).into_iter())
			.filter(|t| now.api.targets.iter().any(|n| n == t))
			.filter(|t| !now.api.named(theirs.on).contains(t))
			.collect();
		if !lost.is_empty() {
			changes.push(format!("it is no longer on {}", lost.join(", ")));
		}
		changes.extend(changed(&base, &now, &maps, &ours.item, &theirs.item));
		if ours.c != theirs.c {
			let mut renamed = format!("its C name `{}` is now `{}`", ours.c, theirs.c);
			if let Some(other) = now.named(&ours.c) {
				renamed += &format!(", and `{}` names {}", ours.c, other.what);
			}
			changes.push(renamed);
		}
		if !changes.is_empty() {
			breaks.push(Break {
				span: theirs.span,
				what: format!("{}: {}", ours.what, changes.join("; ")),
			});
		}
	}
	// A tuple the export no longer uses is no break: what used it reports its own change.
	for (spelled, (c, _)) in &base.tuples {
		let Some((theirs, span)) = now.tuples.get(spelled) else {
			continue;
		};
		if c != theirs {
			let mut what = format!("tuple `{spelled}`: its C name `{c}` is now `{theirs}`");
			if let Some((other, _)) = now.tuples.iter().find(|(_, (n, _))| n == c) {
				what += &format!(", and `{c}` names tuple `{other}`");
			}
			breaks.push(Break { span: *span, what });
		}
	}
	breaks
}

/// An item of an API.
#[derive(Clone, Copy)]
enum Item<'a> {
	Record(usize),
	Enum(usize),
	Object(usize),
	Callback(usize),
	Function(&'a Function),
	Constant(&'a Constant),
}

struct Entry<'a> {
	item: Item<'a>,
	/// How an error names it, such as ``fn `moq::track::Producer::append` ``.
	what: String,
	/// Its C name, the one every other is made from.
	c: String,
	span: Span,
	on: On,
	/// A method's type's key.
	owner: Option<String>,
}

/// An API's items, each by its key: its kind and Rust path with its instance's arguments.
struct Index<'a> {
	api: &'a Api,
	entries: BTreeMap<String, Entry<'a>>,
	/// Each tuple's struct by its Rust spelling: its C name, and a function that takes or
	/// returns it.
	tuples: BTreeMap<String, (String, Span)>,
}

impl<'a> Index<'a> {
	fn new(api: &'a Api, prefix: &str) -> Self {
		Self::with(&Abi::new(api, prefix))
	}

	/// The index of `abi`'s API, reusing its C names.
	fn with(abi: &Abi<'a>) -> Self {
		let api = abi.api;
		let mut entries = BTreeMap::new();
		let shown = |shown: &[String]| shown.join("::");
		let mut types = Vec::new();
		for (i, r) in api.records.iter().enumerate() {
			types.push((
				Item::Record(i),
				"struct",
				&r.shown,
				abi.record(i),
				r.span,
				r.on,
				&r.methods,
			));
		}
		for (i, e) in api.enums.iter().enumerate() {
			types.push((
				Item::Enum(i),
				"enum",
				&e.shown,
				abi.enumeration(i),
				e.span,
				e.on,
				&e.methods,
			));
		}
		for (i, o) in api.objects.iter().enumerate() {
			types.push((
				Item::Object(i),
				"type",
				&o.shown,
				abi.object(i),
				o.span,
				o.on,
				&o.methods,
			));
		}
		for (item, kind, path, c, span, on, methods) in types {
			let key = type_key(path);
			// A method's C name goes through its type's, as the C header declares it.
			let owner = match item {
				Item::Record(_) => Ty::Struct(c.clone()),
				Item::Enum(i) if api.enums[i].plain() => Ty::Enum(c.clone()),
				Item::Enum(_) => Ty::Struct(c.clone()),
				_ => Ty::Handle {
					name: c.clone(),
					constant: false,
				},
			};
			for method in methods {
				let c = abi.export(method, Some(&owner)).call.name;
				entries.insert(function_key(method), Entry::function(method, c, Some(key.clone())));
			}
			let what = format!("{kind} `{}`", shown(path));
			entries.insert(
				key,
				Entry {
					item,
					what,
					c,
					span,
					on,
					owner: None,
				},
			);
		}
		for (i, callback) in api.callbacks.iter().enumerate() {
			let entry = Entry {
				item: Item::Callback(i),
				what: format!("trait `{}`", shown(&callback.path)),
				c: abi.callback(i),
				span: callback.span,
				on: callback.on,
				owner: None,
			};

			entries.insert(type_key(&callback.path), entry);
		}
		for function in &api.functions {
			let c = abi.export(function, None).call.name;
			entries.insert(function_key(function), Entry::function(function, c, None));
		}
		for constant in &api.constants {
			let entry = Entry {
				item: Item::Constant(constant),
				what: format!("const `{}`", shown(&constant.path)),
				c: abi.constant(&constant.path),
				span: constant.span,
				on: constant.on,
				owner: None,
			};
			entries.insert(format!("const {}", shown(&constant.path)), entry);
		}
		let mut tuples: BTreeMap<String, (String, Span)> = (abi.tuples.iter())
			.map(|(c, elements)| (abi.spell(&Type::Tuple(elements.clone())), (c.clone(), DUMMY_SP)))
			.collect();
		// Each item that names a type, with its span: a function by its signature, a
		// record or enum by its fields.
		let functions = (api.functions.iter())
			.chain(api.records.iter().flat_map(|r| &r.methods))
			.chain(api.enums.iter().flat_map(|e| &e.methods))
			.chain(api.objects.iter().flat_map(|o| &o.methods))
			.chain(api.callbacks.iter().flat_map(|c| &c.methods))
			.map(|f| {
				let types: Vec<&Type> = f.params.iter().map(|p| &p.ty).chain([&f.ok]).chain(&f.err).collect();
				(f.span, types)
			});
		let records = (api.records.iter()).map(|r| (r.span, r.fields.iter().map(|f| &f.ty).collect()));
		let enums = (api.enums.iter()).map(|e| {
			(
				e.span,
				e.variants.iter().flat_map(|v| &v.fields).map(|f| &f.ty).collect(),
			)
		});
		for (at, types) in functions.chain(records).chain(enums) {
			for ty in types {
				crate::ir::each(ty, &mut |ty| {
					if let Type::Tuple(elements) = ty {
						let name = abi.tuple(elements);
						if let Some((_, span)) = tuples.values_mut().find(|(c, span)| *c == name && span.is_dummy()) {
							*span = at;
						}
					}
				});
			}
		}
		Self { api, entries, tuples }
	}

	/// The item whose C name is `c`.
	fn named(&self, c: &str) -> Option<&Entry<'a>> {
		self.entries.values().find(|entry| entry.c == c)
	}

	/// Where each of this API's types lands among `now`'s, by key, or past every index
	/// `now` has, each apart, for one it lacks or has as another kind.
	fn maps(&self, now: &Index<'_>) -> Maps {
		let mut missing = usize::MAX;
		let mut at = |key: String, kind: fn(&Item<'_>) -> Option<usize>| {
			(now.entries.get(&key).and_then(|entry| kind(&entry.item))).unwrap_or_else(|| {
				missing -= 1;
				missing
			})
		};
		let api = self.api;
		let record = |item: &Item<'_>| match item {
			Item::Record(i) => Some(*i),
			_ => None,
		};
		let enumeration = |item: &Item<'_>| match item {
			Item::Enum(i) => Some(*i),
			_ => None,
		};
		let object = |item: &Item<'_>| match item {
			Item::Object(i) => Some(*i),
			_ => None,
		};
		let callback = |item: &Item<'_>| match item {
			Item::Callback(i) => Some(*i),
			_ => None,
		};
		Maps {
			records: api.records.iter().map(|r| at(type_key(&r.shown), record)).collect(),
			enums: api.enums.iter().map(|e| at(type_key(&e.shown), enumeration)).collect(),
			objects: api.objects.iter().map(|o| at(type_key(&o.shown), object)).collect(),
			callbacks: api.callbacks.iter().map(|c| at(type_key(&c.path), callback)).collect(),
		}
	}
}

impl<'a> Entry<'a> {
	fn function(function: &'a Function, c: String, owner: Option<String>) -> Self {
		let what = match function.access {
			Access::Field { set: false, .. } => "field",
			Access::Field { set: true, .. } => "setter of field",
			_ => "fn",
		};
		Self {
			item: Item::Function(function),
			what: format!("{what} `{}`", function.shown.join("::")),
			c,
			span: function.span,
			on: function.on,
			owner,
		}
	}
}

/// A type's key: one for every kind, so a type that becomes another kind is changed.
pub fn type_key(path: &[String]) -> String {
	format!("type {}", path.join("::"))
}

/// A function or accessor identity, independent of its generated name.
pub fn function_key(function: &Function) -> String {
	let kind = match function.access {
		Access::Field { set: false, .. } => "get",
		Access::Field { set: true, .. } => "set",
		_ => "fn",
	};
	format!("{kind} {}", function.shown.join("::"))
}

/// How `ours`, of the baseline, and `theirs`, of the export, both of one key, differ, with
/// the baseline's types mapped by `maps` to the export's.
fn changed(base: &Index<'_>, now: &Index<'_>, maps: &Maps, ours: &Item<'_>, theirs: &Item<'_>) -> Vec<String> {
	let (b, n) = (base.api, now.api);
	let mut changes = Vec::new();
	let mut traits = |ours: Traits, theirs: Traits| {
		let lost: Vec<&str> = [
			("Clone", ours.clone && !theirs.clone),
			("PartialEq", ours.eq && !theirs.eq),
			("Display", ours.display && !theirs.display),
			("Debug", ours.debug && !theirs.debug),
			("Default", ours.default && !theirs.default),
		]
		.into_iter()
		.filter_map(|(name, lost)| lost.then_some(name))
		.collect();
		if !lost.is_empty() {
			changes.push(format!("it no longer implements {}", lost.join(", ")));
		}
	};
	match (*ours, *theirs) {
		(Item::Record(i), Item::Record(j)) => {
			traits(b.records[i].traits, n.records[j].traits);
			let mut ours = merge::bare_record(&b.records[i]);
			maps.fields(&mut ours.fields);
			maps.types(&mut ours.args);
			let mut theirs = merge::bare_record(&n.records[j]);
			(ours.traits, theirs.traits) = (Traits::default(), Traits::default());
			if ours.fields != theirs.fields {
				changes.extend(fields("field", &ours.fields, &theirs.fields));
			}
			(ours.fields, theirs.fields) = (Vec::new(), Vec::new());
			if ours != theirs {
				changes.push("how it exports".to_string());
			}
		}
		(Item::Enum(i), Item::Enum(j)) => {
			let (e, f) = (&b.enums[i], &n.enums[j]);
			traits(e.traits, f.traits);
			if e.exhaustive && !f.exhaustive {
				changes.push("it is now `#[non_exhaustive]`".to_string());
			}
			// A plain enum is a C enum, and one with fields a struct of a tag and a payload.
			match (e.plain(), f.plain()) {
				(true, false) => changes.push("a variant with fields makes it a struct in C".to_string()),
				(false, true) => changes.push("no variant has fields, which makes it an enum in C".to_string()),
				_ => {}
			}
			for (at, variant) in e.variants.iter().enumerate() {
				let Some((to, other)) = f.variants.iter().enumerate().find(|(_, v)| v.name == variant.name) else {
					changes.push(format!("variant `{}` is removed", variant.name));
					continue;
				};
				// The tag of an enum with fields is the variant's place.
				if !e.plain() && !f.plain() && at != to {
					changes.push(format!("variant `{}`'s tag {at} is now {to}", variant.name));
				}
				let mut ours = merge::bare_variant(variant);
				maps.fields(&mut ours.fields);
				let theirs = merge::bare_variant(other);
				if ours.fields != theirs.fields {
					let what = format!("variant `{}`'s field", variant.name);
					changes.extend(fields(&what, &ours.fields, &theirs.fields));
				} else if ours != theirs {
					changes.push(format!("variant `{}` changed", variant.name));
				}
				if e.plain() && f.plain() && variant.discriminant != other.discriminant {
					changes.push(format!(
						"variant `{}`'s value {} is now {}",
						variant.name, variant.discriminant, other.discriminant
					));
				}
				let lost: Vec<&str> = (b.named(variant.on).into_iter())
					.filter(|t| n.targets.iter().any(|n| n == t) && !n.named(other.on).contains(t))
					.collect();
				if !lost.is_empty() {
					changes.push(format!(
						"variant `{}` is no longer on {}",
						variant.name,
						lost.join(", ")
					));
				}
			}
			// Another crate matching an exhaustive enum has no arm for a new variant.
			if e.exhaustive {
				for variant in f
					.variants
					.iter()
					.filter(|v| !e.variants.iter().any(|o| o.name == v.name))
				{
					changes.push(format!("variant `{}` is added to an exhaustive enum", variant.name));
				}
			}
			let mut ours = merge::bare_enum(e);
			maps.types(&mut ours.args);
			let mut theirs = merge::bare_enum(f);
			for e in [&mut ours, &mut theirs] {
				e.traits = Traits::default();
				e.exhaustive = false;
			}
			if ours != theirs {
				changes.push("how it exports".to_string());
			}
		}
		(Item::Object(i), Item::Object(j)) => {
			traits(b.objects[i].traits, n.objects[j].traits);
			let mut ours = merge::bare_object(&b.objects[i]);
			maps.types(&mut ours.args);
			let mut theirs = merge::bare_object(&n.objects[j]);
			// An added `Clone` may let copies of a handle share one value: what it adds.
			if !ours.traits.clone && theirs.traits.clone && !ours.shared {
				theirs.shared = false;
			}
			(ours.traits, theirs.traits) = (Traits::default(), Traits::default());
			if ours != theirs {
				changes.push("how it exports".to_string());
			}
		}
		(Item::Callback(i), Item::Callback(j)) => {
			let mut ours = merge::bare_callback(&b.callbacks[i]);
			maps.functions(&mut ours.methods);
			let theirs = merge::bare_callback(&n.callbacks[j]);
			let names = |methods: &[Function]| methods.iter().map(|m| m.name().to_string()).collect::<Vec<_>>();
			for method in &ours.methods {
				match theirs.methods.iter().find(|m| m.name() == method.name()) {
					None => changes.push(format!("method `{}` is removed", method.name())),
					Some(other) if other != method => {
						changes.extend(
							signature(method, other)
								.into_iter()
								.map(|c| format!("method `{}`'s {c}", method.name())),
						);
					}
					Some(_) => {}
				}
			}
			// A foreign implementation lacks a method the trait now requires.
			for name in names(&theirs.methods)
				.iter()
				.filter(|m| !names(&ours.methods).contains(m))
			{
				changes.push(format!(
					"method `{name}` is added, which every implementation must have"
				));
			}
			if changes.is_empty() && ours != theirs {
				changes.push("how it exports".to_string());
			}
		}
		(Item::Function(f), Item::Function(g)) => {
			let mut ours = merge::bare_function(f);
			maps.function(&mut ours);
			let theirs = merge::bare_function(g);
			changes.extend(signature(&ours, &theirs).into_iter().map(|c| format!("its {c}")));
		}
		(Item::Constant(c), Item::Constant(d)) => {
			// Its value may change: what a constant is called and its type are its API.
			let bare = |c: &Constant| {
				let mut c = merge::bare_constant(c);
				c.value = match c.value {
					Literal::Bool(_) => Literal::Bool(false),
					Literal::Int(int, _) => Literal::Int(int, 0),
				};
				c
			};
			let ty = |c: &Constant| match c.value {
				Literal::Bool(_) => "bool",
				Literal::Int(int, _) => int.rust(),
			};
			if ty(c) != ty(d) {
				changes.push(format!("its type: it was `{}`, and is now `{}`", ty(c), ty(d)));
			} else if bare(c) != bare(d) {
				changes.push("how it exports".to_string());
			}
		}
		_ => changes.push("it is another kind of item".to_string()),
	}
	changes
}

/// How the fields `ours` became `theirs`, each named as `what`.
fn fields(what: &str, ours: &[crate::ir::Field], theirs: &[crate::ir::Field]) -> Vec<String> {
	let mut changes = Vec::new();
	for (i, field) in ours.iter().enumerate() {
		match theirs.iter().position(|f| f.name == field.name) {
			None => changes.push(format!("{what} `{}` is removed", field.name)),
			Some(j) if theirs[j].ty != field.ty => changes.push(format!("{what} `{}`'s type changed", field.name)),
			// A value's fields are laid out in order, in C as in every binding's struct.
			Some(j) if j != i => changes.push(format!("{what} `{}` moved", field.name)),
			Some(_) => {}
		}
	}
	for field in theirs.iter().filter(|f| !ours.iter().any(|o| o.name == f.name)) {
		changes.push(format!(
			"{what} `{}` is added, which every value must now give",
			field.name
		));
	}
	changes
}

/// What of the function `ours` changed in `theirs`, both bare and of one key.
fn signature(ours: &Function, theirs: &Function) -> Vec<String> {
	let (mut ours, mut theirs) = (ours.clone(), theirs.clone());
	// Which name the config gives a field: no binding names it otherwise.
	for f in [&mut ours, &mut theirs] {
		if let Access::Field { shared, .. } = &mut f.access {
			*shared = false;
		}
	}
	let mut changes = Vec::new();
	if ours.receiver != theirs.receiver {
		changes.push("receiver".to_string());
	}
	if ours.params.len() != theirs.params.len() {
		changes.push(format!(
			"parameters: it took {}, and now takes {}",
			ours.params.len(),
			theirs.params.len()
		));
	} else {
		for (p, q) in ours.params.iter().zip(&theirs.params) {
			if p.name != q.name {
				changes.push(format!("parameter `{}`, now `{}`", p.name, q.name));
			} else if p != q {
				changes.push(format!("parameter `{}`'s type", p.name));
			}
		}
	}
	if ours.ok != theirs.ok {
		changes.push("return type".to_string());
	}
	if ours.err != theirs.err {
		changes.push("error type".to_string());
	}
	if ours.future != theirs.future {
		changes.push("whether it is async".to_string());
	}
	if changes.is_empty() && ours != theirs {
		changes.push("how it exports".to_string());
	}
	changes
}
