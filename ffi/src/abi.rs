//! The C ABI the IR becomes: every C name, and every function's C signature, decided
//! once so the header and the Rust side agree.
//!
//! Names follow one rule: the prefix, then the item's path from the facade's root, each
//! segment in Rust's case, joined by `_`; a method's path goes through its type's, and a
//! field's getter and setter add `_get` and `_set` to it. Every join keeps one `_` of each
//! run, since C++ reserves every name containing `__`, so Rust's `_deprecated` is
//! `moq_deprecated`. Case keeps a type apart from a function or module of its name, and
//! Rust's verb-first names keep a method apart from a field's accessors. Joining still loses
//! where segments split (`audio_encoder::finish` and `audio::encoder_finish` both join as
//! `moq_audio_encoder_finish`), so every name is registered, and of the items whose names
//! clash, every one but the first by rank steps along its name's `x_`, `x_2`, `x_3`, ...,
//! as do the names made from it: `moq_audio_encoder_finish` and
//! `moq_audio_encoder_finish_`. A name made from one ending in `_` is its stem's, so where
//! that clashes too, the item steps on: an enum `Status` beside the runtime's
//! `MOQ_STATUS_MOVED` is `moq_Status_2`, with `MOQ_STATUS_2_MOVED`. Only the names that
//! clash change.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use rustc_span::Span;

use crate::ir::{
	Access, Api, Closure, Dyn, Family, Function, Home, Int, Jni, Literal, Mutability, On, Order, Param, Receiver, Type,
};

/// A C name one item claims twice, which no rename can tell apart: the name, then what
/// claimed it first and second.
pub(crate) type Clash = (String, (String, Span), (String, Span));

/// What a C name names, which a clash renames: a type, a function by its address in the
/// API, a constant by its path, or a tuple by its elements.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Item {
	Record(usize),
	Enum(usize),
	Object(usize),
	Callback(usize),
	Function(usize),
	Constant(Vec<String>),
	Tuple(Vec<Type>),
}

/// The item a claimed name belongs to: whether the claimed name is its own C name rather
/// than one made from it, and whether it is that one as Rust spells it, unrenamed and with
/// no `_` [`join`] dropped.
struct Owner {
	item: Item,
	own: bool,
	spelled: bool,
}

/// A C name claimed, by what, and the item a clash renames, or none for the runtime's.
struct Claim {
	name: String,
	what: String,
	span: Span,
	owner: Option<Owner>,
}

/// A C type, as a parameter, field, or return value takes it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) enum Ty {
	Void,
	Bool,
	Int(Int),
	F32,
	F64,
	/// A `char`, as its code point.
	Char,
	/// `<prefix>_buf`: a view of bytes passed in, an owned buffer returned.
	Buf,
	/// `<prefix>_u128`
	U128,
	/// `<prefix>_duration`
	Duration,
	/// `<prefix>_socket_addr`, or `_socket_addr_v4` or `_socket_addr_v6` for one family.
	SocketAddr(Family),
	/// `<prefix>_ip_addr`, or `_ipv4_addr` or `_ipv6_addr` for one family.
	IpAddr(Family),
	/// A struct: a record, a data enum, an option, or a callback's table and data.
	Struct(String),
	/// A plain enum, passed as its C enum, which is an `int`.
	Enum(String),
	/// A handle: `const` when only borrowed.
	Handle {
		name: String,
		constant: bool,
	},
	/// A future's handle.
	Future(String),
	/// `<prefix>_waker`
	Waker,
	/// `<prefix>_foreign_future`: a future C implements, which Rust polls and frees.
	ForeignFuture,
	/// `<prefix>_foreign_reader`: a reader C implements, which Rust reads and seeks.
	ForeignReader,
	/// `<prefix>_poll`
	Poll,
	/// `<prefix>_status`, an `int32_t`: why a call failed a check of a handle, which a
	/// function that can fail one writes through its last parameter.
	Status,
	/// A pointer the callee writes a result through.
	Out(Box<Ty>),
	/// A pointer the callee only reads through: a value's receiver.
	In(Box<Ty>),
	/// A pointer the callee reads and writes back through: a value's `&mut self`.
	InOut(Box<Ty>),
	/// A function lent for the call, taking its data first: what it takes and returns.
	Fn(Vec<Ty>, Box<Ty>),
	/// The data a lent function takes, which Rust only passes back to it.
	Data,
	/// A JNI value's pointer, a `JNIEnv *` or a `jobject`, which only Kotlin's glue passes.
	Jni(Jni),
}

/// A C function: its name, parameters, and return type.
#[derive(Clone)]
pub(crate) struct Sig {
	pub(crate) name: String,
	pub(crate) params: Vec<(String, Ty)>,
	pub(crate) ret: Ty,
}

impl Sig {
	/// Its twin that runs it on a home: `<name>_homed`, of the same signature.
	pub(crate) fn homed(&self) -> Sig {
		Sig {
			name: join(&self.name, "homed"),
			params: self.params.clone(),
			ret: self.ret.clone(),
		}
	}

	/// The `status` parameter it takes last, if a call can fail a check of a handle.
	pub(crate) fn status(&self) -> Option<&str> {
		self.params
			.last()
			.filter(|(_, ty)| is_status(ty))
			.map(|(name, _)| name.as_str())
	}
}

/// The runtime's C struct of a socket address, if `socket`, or else an IP address, of
/// `family`, after the prefix: a struct per family, so each holds exactly its Rust type.
pub(crate) fn addr(socket: bool, family: Family) -> &'static str {
	match (socket, family) {
		(true, Family::Any) => "socket_addr",
		(true, Family::V4) => "socket_addr_v4",
		(true, Family::V6) => "socket_addr_v6",
		(false, Family::Any) => "ip_addr",
		(false, Family::V4) => "ipv4_addr",
		(false, Family::V6) => "ipv6_addr",
	}
}

/// Whether `ty` is a `status` parameter's.
pub(crate) fn is_status(ty: &Ty) -> bool {
	matches!(ty, Ty::Out(inner) if **inner == Ty::Status)
}

/// The C side of an exported function.
pub(crate) struct Export {
	/// The function itself: for a future, the call that makes it.
	pub(crate) call: Sig,
	/// The C name its path gives it, which another language names it by, as a clash in C
	/// renames only C's.
	pub(crate) named: String,
	/// For a future: its handle type and the functions over it.
	pub(crate) future: Option<Future>,
	/// Whether it makes a value that is not `Send` from none: its call, or for a future its
	/// start, has a twin of the same signature, `<name>_homed`, that runs it on a home, a
	/// thread Rust owns, which the value then stays on. A binding with no thread that stays
	/// put calls that one.
	pub(crate) homed: bool,
}

impl Export {
	/// The twin of its call that runs it on a home, if it makes a value that is not `Send`.
	pub(crate) fn homed_call(&self) -> Option<Sig> {
		self.homed.then(|| self.call.homed())
	}

	/// It as a binding with no thread that stays put calls it: by its twins that run on a
	/// home, if it has them.
	pub(crate) fn on_home(mut self) -> Self {
		if self.homed {
			self.call = self.call.homed();
			if let Some(future) = &mut self.future {
				future.start = future.start.homed();
			}
		}
		self
	}

	/// The twin of its future's start that runs it on a home, as [`Self::homed_call`].
	pub(crate) fn homed_start(&self) -> Option<Sig> {
		(self.future.as_ref()).and_then(|future| self.homed.then(|| future.start.homed()))
	}
}

/// How the glue calls a value's `&mut self` method for a language whose values are
/// immutable, which [`Abi::handed_back`] makes.
#[derive(Clone)]
pub(crate) struct Handed {
	/// The C name of the method's function, which a clash may have renamed.
	pub(crate) name: String,
	/// How many parameters both take after `self`, before the outs.
	lead: usize,
	/// Whether the method returns nothing.
	unit: bool,
	/// Whether it returns a tuple, which the C function writes through its outs.
	tuple: bool,
	/// Whether both take a status last, as a call that can fail a check does.
	status: bool,
}

impl Handed {
	/// The glue's expression, of the handed-back export's C return, calling the C function
	/// in place on a copy of the value that `args[0]` points to and handing the copy back.
	/// `args` are of that export's C signature: `self`, what both take after it, then the outs the value and
	/// each element Rust returns are written through, which the C function takes from the
	/// one after the value's, and last the status, if both take one.
	pub(crate) fn call(&self, args: &[String]) -> String {
		let lead = self.lead;
		let (args, status) = match self.status {
			true => args.split_at(args.len() - 1),
			false => (args, &[][..]),
		};
		let mut real = vec!["&mut handed_".to_string()];
		real.extend(args[1..=lead].iter().cloned());
		if self.tuple {
			real.extend(args[lead + 2..].iter().cloned());
		}
		real.extend(status.iter().cloned());
		let call = format!("crate::{}({})", self.name, real.join(", "));
		// Apart from the names the glue gives its arguments, which are numbered.
		let body = match (self.unit, self.tuple) {
			(true, _) => format!("{call};\n\thanded_"),
			(_, true) => format!("{call};\n\tstd::ptr::write({}, handed_)", args[lead + 1]),
			(_, false) => format!(
				"let handed_r_ = {call};\n\tstd::ptr::write({}, handed_);\n\tstd::ptr::write({}, handed_r_)",
				args[lead + 1],
				args[lead + 2]
			),
		};
		format!("unsafe {{\n\tlet mut handed_ = *({});\n\t{body}\n}}", args[0])
	}
}

/// The C functions of a future besides the call that makes it.
pub(crate) struct Future {
	/// The handle's type.
	pub(crate) handle: String,
	/// The call and its first poll in one, which writes a ready result as the poll does and
	/// hands back the handle only if the future pends.
	pub(crate) start: Sig,
	pub(crate) poll: Sig,
	pub(crate) free: Sig,
}

/// Each `<prefix>_status` a call can fail with, numbered from 1, and what it means, as
/// the runtime's `Status` names them.
pub(crate) const STATUSES: [(&str, &str); 7] = [
	("moved", "the handle's value was moved out"),
	("exclusive", "another call or future borrows the value exclusively"),
	("borrowed", "another call or future borrows the value"),
	("thread", "a value that is not Sync is borrowed on another thread"),
	("lent", "a handle a call lent is borrowed exclusively or moved out"),
	("outlived", "a handle lent from another outlived it"),
	(
		"not_send",
		"a value that is not Send is used on another thread than the one that made it",
	),
];

/// Whether a call can fail a check of each object's handle, by Rust's rules alone: its
/// value is not `Sync`, so a borrow on another thread fails; an export borrows it
/// exclusively or moves it out, so a borrow beside another call's fails; or a handle to
/// it is lent, by a call returning a borrow or to a closure or trait for a call, so a use
/// past the lend fails, as may a trait's method returning it. Every other object's handles
/// are only ever borrowed shared, which never fails, so a call borrows them uncounted.
fn checked(api: &Api) -> Vec<bool> {
	/// Mark each object a parameter of type `ty` borrows exclusively, moves, or lends,
	/// `owned` when `ty` is passed by value rather than behind a shared reference.
	fn param(api: &Api, ty: &Type, owned: bool, checked: &mut [bool]) {
		match ty {
			Type::Object(i) if owned && !api.objects[*i].copy => checked[*i] = true,
			Type::Ref(Mutability::Mut, inner) => {
				if let Type::Object(i) = inner.base() {
					checked[*i] = true;
				}
				param(api, inner, true, checked);
			}
			Type::Ref(Mutability::Not, inner) => param(api, inner, false, checked),
			Type::Option(inner) | Type::Box(inner) | Type::List(_, inner) => param(api, inner, owned, checked),
			Type::Tuple(elements) => elements.iter().for_each(|ty| param(api, ty, owned, checked)),
			Type::Closure(closure) => closure.params.iter().for_each(|ty| lent(api, ty, checked)),
			// A record or data enum passed by value moves each object it holds.
			Type::Record(_) | Type::Enum(_) if owned => lent(api, ty, checked),
			_ => {}
		}
	}
	/// Mark each object a value of `ty` Rust lends the foreign side holds.
	fn lent(api: &Api, ty: &Type, checked: &mut [bool]) {
		let mut handles = Vec::new();
		api.handles(ty, &mut handles, &mut Vec::new());
		handles.into_iter().for_each(|i| checked[i] = true);
	}
	let mut checked: Vec<bool> = api.objects.iter().map(|o| !o.sync).collect();
	let functions = (api.records.iter().flat_map(|r| &r.methods))
		.chain(api.enums.iter().flat_map(|e| &e.methods))
		.chain(&api.functions)
		.map(|f| (None, f))
		.chain((api.objects.iter().enumerate()).flat_map(|(i, o)| o.methods.iter().map(move |f| (Some(i), f))));
	for (owner, f) in functions {
		match (owner, f.receiver) {
			(Some(i), Some(Receiver::Mut)) => checked[i] = true,
			(Some(i), Some(Receiver::Owned)) if !api.objects[i].copy => checked[i] = true,
			_ => {}
		}
		f.params.iter().for_each(|p| param(api, &p.ty, true, &mut checked));
		if f.lends {
			lent(api, &f.ok, &mut checked);
		}
		// A handle holding an exclusive lend refuses what would carry the borrow past it.
		if f.lends_mut
			&& let Type::Object(j) = f.ok
		{
			checked[j] = true;
		}
	}
	// What an implementation is lent, and what it hands Rust, which moves it out.
	for method in api.callbacks.iter().flat_map(|c| &c.methods) {
		method.params.iter().for_each(|p| lent(api, &p.ty, &mut checked));
		lent(api, &method.ok, &mut checked);
		method.err.iter().for_each(|err| lent(api, err, &mut checked));
	}
	checked
}

/// Whether an exclusive lend returns each object.
fn held(api: &Api) -> Vec<bool> {
	let mut held = vec![false; api.objects.len()];
	let functions = (api.records.iter().flat_map(|r| &r.methods))
		.chain(api.enums.iter().flat_map(|e| &e.methods))
		.chain(api.objects.iter().flat_map(|o| &o.methods))
		.chain(&api.functions);
	for f in functions.filter(|f| f.lends_mut) {
		if let Type::Object(j) = f.ok {
			held[j] = true;
		}
	}
	held
}

/// Whether a handle of each object may be lent from one that is not `Send`, directly or
/// through a chain of lent handles: a shared borrow of it borrows that owner on its thread,
/// which first drains what other threads queued to drop there, so it runs user code.
fn affine(api: &Api) -> Vec<bool> {
	let mut affine: Vec<bool> = api.objects.iter().map(|o| !o.send).collect();
	loop {
		let mut grew = false;
		for (i, object) in api.objects.iter().enumerate() {
			if !affine[i] {
				continue;
			}
			for f in object.methods.iter().filter(|f| f.lends) {
				let mut handles = Vec::new();
				api.handles(&f.ok, &mut handles, &mut Vec::new());
				for j in handles.into_iter().filter(|&j| j != i) {
					grew |= !std::mem::replace(&mut affine[j], true);
				}
			}
		}
		if !grew {
			return affine;
		}
	}
}

/// Whether a handle of each object may be lent through a chain of lenders as long as the
/// caller makes it, as by calling a `view(&self) -> &Self` on each handle it returned: a
/// shared borrow of it walks that chain, so its cost has no bound. True of each object a
/// cycle of lending methods reaches; any other's chain is at most one handle per object.
fn chained(api: &Api) -> Vec<bool> {
	let lent: Vec<Vec<usize>> = (api.objects.iter())
		.map(|object| {
			let mut lent = Vec::new();
			for f in object.methods.iter().filter(|f| f.lends) {
				api.handles(&f.ok, &mut lent, &mut Vec::new());
			}
			lent
		})
		.collect();
	past_cycles(&lent)
}

/// Whether a cycle of `edges`, each node's successors, reaches each node.
fn past_cycles(edges: &[Vec<usize>]) -> Vec<bool> {
	let n = edges.len();
	// The nodes reachable in one edge or more from `from`.
	let reach = |from: usize| {
		let mut seen = vec![false; n];
		let mut stack = edges[from].clone();
		while let Some(j) = stack.pop() {
			if !std::mem::replace(&mut seen[j], true) {
				stack.extend(&edges[j]);
			}
		}
		seen
	};
	let mut past = vec![false; n];
	for i in 0..n {
		let seen = reach(i);
		if seen[i] {
			for (j, seen) in seen.into_iter().enumerate() {
				past[j] |= seen;
			}
		}
	}
	past
}

/// `ty` as a clone of each borrow it holds.
fn owned(ty: &Type) -> Type {
	match ty {
		Type::Ref(_, inner) => owned(inner),
		Type::Option(inner) => Type::Option(Box::new(owned(inner))),
		Type::Box(inner) => Type::Box(Box::new(owned(inner))),
		Type::List(c, inner) => Type::List(*c, Box::new(owned(inner))),
		Type::Tuple(elements) => Type::Tuple(elements.iter().map(owned).collect()),
		ty => ty.clone(),
	}
}

/// The suffix naming each instance of a generic item, by its arguments.
type Suffixes = HashMap<Vec<Type>, String>;

/// The `cfg` the crate's build script sets to the target it builds for.
pub(crate) const CFG: &str = "rust2_ffi_target";

pub(crate) struct Abi<'a> {
	pub(crate) api: &'a Api,
	/// The library's name as [`prefix`] makes it a C name's first segment.
	pub(crate) prefix: String,
	/// Each generic item's instances, by whether it is a function and its C name, with
	/// the suffix that names each by its arguments.
	suffixes: std::cell::RefCell<HashMap<(bool, String), Suffixes>>,
	/// Each `Option<T>` struct by C name, with `T`.
	pub(crate) options: BTreeMap<String, Type>,
	/// Each range, bound, and pair of bounds struct by C name, with its type.
	pub(crate) ranges: BTreeMap<String, Type>,
	/// Each `Vec<T>` struct by C name, with `T`.
	pub(crate) lists: BTreeMap<String, Type>,
	/// Each tuple struct by C name, with its elements.
	pub(crate) tuples: BTreeMap<String, Vec<Type>>,
	/// Each `[u8; N]` struct by C name, with `N`.
	pub(crate) arrays: BTreeMap<String, usize>,
	/// Whether a call can fail a check of each object's handle: see [`checked`].
	pub(crate) checked: Vec<bool>,
	/// Whether an exclusive lend returns each object, whose handle may then hold its
	/// receiver's borrow.
	pub(crate) held: Vec<bool>,
	/// Whether each object is not `Send` or may be lent from one: see [`affine`].
	affine: Vec<bool>,
	/// Whether each object's chain of lenders has no bound: see [`chained`].
	chained: Vec<bool>,
	/// Every C name claimed.
	claims: Vec<Claim>,
	/// How many steps each item whose own C name clashed took past it.
	renames: HashMap<Item, usize>,
	/// Each name one item claims twice.
	pub(crate) clashes: Vec<Clash>,
}

impl<'a> Abi<'a> {
	/// The C ABI of `api`, exported by the library `lib`, which [`prefix`] must name a prefix.
	pub(crate) fn new(api: &'a Api, lib: &str) -> Self {
		let mut abi = Self {
			api,
			prefix: prefix(lib).expect("the driver refuses a library that names no prefix"),
			suffixes: Default::default(),
			options: BTreeMap::new(),
			ranges: BTreeMap::new(),
			lists: BTreeMap::new(),
			tuples: BTreeMap::new(),
			arrays: BTreeMap::new(),
			checked: checked(api),
			held: held(api),
			affine: affine(api),
			chained: chained(api),
			claims: Vec::new(),
			renames: HashMap::new(),
			clashes: Vec::new(),
		};
		// Each round renames what clashed, until a round finds nothing more to rename.
		loop {
			abi.claims.clear();
			abi.claim_all();
			if !abi.rename() {
				break;
			}
		}
		abi.clashes = abi.residue();
		abi.clashes.extend(abi.scoped());
		abi
	}

	/// Every name the C header declares for the API, the runtime's aside, each once.
	pub(crate) fn declared(&self) -> BTreeSet<&str> {
		(self.claims.iter())
			.filter(|claim| claim.what != "the runtime")
			.map(|claim| claim.name.as_str())
			.collect()
	}

	/// This ABI over `api`, the API it was made of without its functions taking a JNI value,
	/// as every language but Kotlin exports it: each name stays the one the whole API gives
	/// it, so the C its glue calls keeps the names the crate defines.
	pub(crate) fn without_jni<'b>(&self, api: &'b Api) -> Abi<'b> {
		// Each function's address in `self.api`, by its address in `api`, which lists the
		// same functions in the same order but those taking a JNI value.
		let lists = |api: &'a Api| -> Vec<&'a [Function]> {
			(api.records.iter().map(|r| &r.methods[..]))
				.chain(api.enums.iter().map(|e| &e.methods[..]))
				.chain(api.objects.iter().map(|o| &o.methods[..]))
				.chain([&api.functions[..]])
				.collect()
		};
		let ours = lists(self.api);
		let theirs = (api.records.iter().map(|r| &r.methods[..]))
			.chain(api.enums.iter().map(|e| &e.methods[..]))
			.chain(api.objects.iter().map(|o| &o.methods[..]))
			.chain([&api.functions[..]]);
		let mut moved = HashMap::new();
		for (ours, theirs) in ours.into_iter().zip(theirs) {
			let mut theirs = theirs.iter();
			for function in ours.iter().filter(|f| !f.jni()) {
				let to = theirs.next().expect("the API without JNI keeps every other function");
				moved.insert(function as *const Function as usize, to as *const Function as usize);
			}
		}
		let renames = (self.renames.iter())
			.filter_map(|(item, &steps)| match item {
				Item::Function(at) => Some((Item::Function(*moved.get(at)?), steps)),
				item => Some((item.clone(), steps)),
			})
			.collect();
		Abi {
			api,
			prefix: self.prefix.clone(),
			suffixes: self.suffixes.clone(),
			options: self.options.clone(),
			ranges: self.ranges.clone(),
			lists: self.lists.clone(),
			tuples: self.tuples.clone(),
			arrays: self.arrays.clone(),
			// Its names, for [`Self::declared`], but not their owners, which are `self.api`'s.
			claims: (self.claims.iter())
				.map(|claim| Claim {
					name: claim.name.clone(),
					what: claim.what.clone(),
					span: claim.span,
					owner: None,
				})
				.collect(),
			renames,
			clashes: self.clashes.clone(),
			checked: self.checked.clone(),
			held: self.held.clone(),
			affine: self.affine.clone(),
			chained: self.chained.clone(),
		}
	}

	/// The `cfg` predicate a crate's Rust compiles what is on `on` under, or none if it is
	/// on every target: its build script sets [`CFG`] to the target it builds for.
	pub(crate) fn cfg(&self, on: On) -> Option<String> {
		if self.api.everywhere(on) {
			return None;
		}
		let targets: Vec<String> = self.api.named(on).iter().map(|t| format!("{CFG} = {t:?}")).collect();
		Some(format!("any({})", targets.join(", ")))
	}

	/// The targets every type `ty` names is on, which a value of it is on.
	pub(crate) fn on(&self, ty: &Type) -> On {
		let api = self.api;
		let mut on = On::ALL;
		crate::ir::each(ty, &mut |ty| {
			let of = match ty {
				Type::Record(i) => api.records[*i].on,
				Type::Enum(i) => api.enums[*i].on,
				Type::Object(i) => api.objects[*i].on,
				Type::Callback(i, _) => api.callbacks[*i].on,
				_ => On::ALL,
			};
			on.0 &= of.0;
		});
		on
	}

	/// The C name of `item` whose path joins as `name`: it keeping one `_` of each run, then
	/// [`stepped`] as often as it clashed.
	fn renamed(&self, item: Item, name: impl FnOnce() -> String) -> String {
		let name = single(&name());
		match self.renames.get(&item) {
			Some(&n) => stepped(&name, n),
			None => name,
		}
	}

	/// The claims of each name claimed more than once.
	fn groups(&self) -> BTreeMap<&str, Vec<&Claim>> {
		let mut groups: BTreeMap<&str, Vec<&Claim>> = BTreeMap::new();
		for claim in &self.claims {
			groups.entry(&claim.name).or_default().push(claim);
		}
		groups.retain(|_, claims| claims.len() > 1);
		groups
	}

	/// Rename the items whose names clash, returning whether any was. The runtime's names
	/// come first, then names made from an item, then items' own as Rust spells them, then
	/// those a [`join`] or a step respelled, each by what claims it: the first keeps the name and
	/// every later one steps once more.
	fn rename(&mut self) -> bool {
		let mut renames: HashMap<Item, usize> = HashMap::new();
		for group in self.groups().into_values() {
			let mut group = group;
			let rank = |claim: &Claim| match &claim.owner {
				None => 0,
				Some(Owner { own: false, .. }) => 1,
				Some(Owner { spelled: true, .. }) => 2,
				Some(_) => 3,
			};
			group.sort_by(|a, b| (rank(a), &a.what).cmp(&(rank(b), &b.what)));
			let first = group[0].owner.as_ref().map(|owner| &owner.item);
			for claim in &group[1..] {
				// A name an item claims twice, no rename of it tells apart.
				if let Some(owner) = claim.owner.as_ref().filter(|owner| Some(&owner.item) != first) {
					let steps = self.renames.get(&owner.item).copied().unwrap_or(0);
					renames.entry(owner.item.clone()).or_insert(steps + 1);
				}
			}
		}
		let renamed = !renames.is_empty();
		self.renames.extend(renames);
		renamed
	}

	/// Each name still claimed twice once renaming is done: one item's, twice.
	fn residue(&self) -> Vec<Clash> {
		let mut clashes = Vec::new();
		for (name, group) in self.groups() {
			for claim in &group[1..] {
				clashes.push((
					name.to_string(),
					(group[0].what.clone(), group[0].span),
					(claim.what.clone(), claim.span),
				));
			}
		}
		clashes
	}

	/// Each two names of one C scope that [`escape`] makes alike, as it may one C or C++
	/// reserves or one ending in `_` and a number: a struct's fields, a data enum's members and each
	/// variant's fields, a trait's table and each method's parameters, and a function's.
	fn scoped(&self) -> Vec<Clash> {
		let api = self.api;
		let mut clashes = Vec::new();
		let mut scope = |names: Vec<(String, String)>, span: Span| {
			for (k, (name, what)) in names.iter().enumerate() {
				if let Some((_, first)) = names[..k].iter().find(|(other, _)| other == name) {
					clashes.push((name.clone(), (first.clone(), span), (what.clone(), span)));
				}
			}
		};
		let fields = |of: &str, fields: &[crate::ir::Field]| -> Vec<(String, String)> {
			(fields.iter())
				.map(|f| (escape(&f.name), format!("field `{}` of {of}", f.name)))
				.collect()
		};
		let mut functions: Vec<(&Function, Option<Ty>)> = Vec::new();
		for (i, record) in api.records.iter().enumerate() {
			let rust = format!("struct `{}`", api.rust(&record.path, &record.args));
			scope(fields(&rust, &record.fields), record.span);
			functions.extend(record.methods.iter().map(|m| (m, Some(Ty::Struct(self.record(i))))));
		}
		for (i, e) in api.enums.iter().enumerate() {
			let rust = format!("enum `{}`", api.rust(&e.path, &e.args));
			let members = (e.variants.iter())
				.filter(|v| !v.fields.is_empty())
				.map(|v| (escape(&snake(&v.name)), format!("variant `{}` of {rust}", v.name)));
			scope(
				std::iter::once(("tag".to_string(), format!("the tag of {rust}")))
					.chain(members)
					.collect(),
				e.span,
			);
			for v in &e.variants {
				scope(fields(&format!("variant `{}` of {rust}", v.name), &v.fields), e.span);
			}
			functions.extend(e.methods.iter().map(|m| (m, Some(self.ty(&Type::Enum(i))))));
		}
		for (i, object) in api.objects.iter().enumerate() {
			let owner = Ty::Handle {
				name: self.object(i),
				constant: false,
			};
			functions.extend(object.methods.iter().map(|m| (m, Some(owner.clone()))));
		}
		for callback in &api.callbacks {
			let rust = format!("trait `{}`", callback.path.join("::"));
			let table = (callback.methods.iter()).map(|m| (member(m), format!("method `{}` of {rust}", m.name())));
			scope(
				std::iter::once(("free".to_string(), format!("the free of {rust}")))
					.chain(table)
					.collect(),
				callback.span,
			);
			for method in &callback.methods {
				let params =
					(method.params.iter()).map(|p| (escape(&p.name), format!("parameter `{}` of {rust}", p.name)));
				scope(
					std::iter::once(("data".to_string(), "its data".to_string()))
						.chain(params)
						.collect(),
					method.span,
				);
			}
		}
		functions.extend(api.functions.iter().map(|f| (f, None)));
		for (function, owner) in functions {
			let export = self.export(function, owner.as_ref());
			let sigs = std::iter::once(&export.call)
				.chain(export.future.iter().flat_map(|future| [&future.start, &future.poll]));
			for sig in sigs {
				// Each C parameter's Rust one, the first not yet named that escapes to it.
				let mut named = vec![false; function.params.len()];
				let fun = function.path.join("::");
				let names = (sig.params.iter())
					.map(|(name, _)| {
						let rust = (0..named.len()).find(|&k| !named[k] && escape(&function.params[k].name) == *name);
						let what = match rust {
							Some(k) => {
								named[k] = true;
								format!("parameter `{}` of fn `{fun}`", function.params[k].name)
							}
							None => format!("`{name}`, which C adds to fn `{fun}`"),
						};
						(name.clone(), what)
					})
					.collect();
				scope(names, function.span);
			}
		}
		clashes
	}

	/// The name a Rust path joins as, which starts with the facade's root: its item's C
	/// name, but for a clash or a `_` [`join`] drops, which other languages' names keep.
	pub(crate) fn name(&self, path: &[String]) -> String {
		std::iter::once(self.prefix.clone())
			.chain(self.segments(path))
			.collect::<Vec<_>>()
			.join("_")
	}

	/// The segments of the C name of the item at `shown` after the prefix: each of its
	/// path's after the root, without an instance's arguments.
	fn segments(&self, shown: &[String]) -> Vec<String> {
		(shown[1..].iter())
			.map(|segment| segment.split('<').next().unwrap_or(segment).to_string())
			.collect()
	}

	/// A constant's name for a path: the name in upper case.
	pub(crate) fn constant(&self, path: &[String]) -> String {
		self.renamed(Item::Constant(path.to_vec()), || self.name(path).to_uppercase())
	}

	pub(crate) fn record(&self, i: usize) -> String {
		self.renamed(Item::Record(i), || self.unrenamed(&Item::Record(i)))
	}

	pub(crate) fn enumeration(&self, i: usize) -> String {
		self.renamed(Item::Enum(i), || self.unrenamed(&Item::Enum(i)))
	}

	pub(crate) fn object(&self, i: usize) -> String {
		self.renamed(Item::Object(i), || self.unrenamed(&Item::Object(i)))
	}

	/// The C name of the type `item` as no clash renames it.
	fn unrenamed(&self, item: &Item) -> String {
		let api = self.api;
		match *item {
			Item::Record(i) => self.type_instance(&api.records[i].path, &api.records[i].shown, &api.records[i].args),
			Item::Enum(i) => self.type_instance(&api.enums[i].path, &api.enums[i].shown, &api.enums[i].args),
			Item::Object(i) => self.type_instance(&api.objects[i].path, &api.objects[i].shown, &api.objects[i].args),
			Item::Callback(i) => self.name(&api.callbacks[i].path),
			ref other => unreachable!("{other:?} is no type"),
		}
	}

	/// The segments the unrenamed C name of the type `item` joins after the prefix: its
	/// path's, then an instance's suffix.
	fn type_segments(&self, item: &Item) -> Vec<String> {
		let api = self.api;
		let shown = match *item {
			Item::Record(i) => &api.records[i].shown,
			Item::Enum(i) => &api.enums[i].shown,
			Item::Object(i) => &api.objects[i].shown,
			Item::Callback(i) => &api.callbacks[i].path,
			ref other => unreachable!("{other:?} is no type"),
		};
		let mut segments = self.segments(shown);
		let name = self.unrenamed(item);
		segments.extend(name[self.name(shown).len()..].strip_prefix('_').map(str::to_string));
		segments
	}

	/// The C name of the instance of the type at `path` at `args`, shown as `shown`, among
	/// the type's every instance, whichever kind each is.
	fn type_instance(&self, path: &[String], shown: &[String], args: &[Type]) -> String {
		let api = self.api;
		self.instance(false, self.name(shown), args, || {
			let records = api.records.iter().map(|r| (&r.path, &r.args));
			let enums = api.enums.iter().map(|e| (&e.path, &e.args));
			let objects = api.objects.iter().map(|o| (&o.path, &o.args));
			(records.chain(enums).chain(objects))
				.filter(|(other, _)| other[..] == *path)
				.map(|(_, args)| args.clone())
				.collect()
		})
	}

	/// An instance's C name: its generic item's, `name`, then the suffix naming `args`
	/// apart from the arguments of the item's other instances, which `group` lists.
	fn instance(&self, function: bool, name: String, args: &[Type], group: impl FnOnce() -> Vec<Vec<Type>>) -> String {
		if args.is_empty() {
			return name;
		}
		let key = (function, name);
		if let Some(suffix) = self.suffixes.borrow().get(&key).and_then(|suffixes| suffixes.get(args)) {
			return format!("{}_{suffix}", key.1);
		}
		let group = group();
		let forms: Vec<Vec<Vec<String>>> = (group.iter())
			.map(|args| args.iter().map(|arg| self.arg_forms(arg)).collect())
			.collect();
		let suffixes: Suffixes = group.into_iter().zip(shortest(&forms)).collect();
		let suffix = suffixes[args].clone();
		self.suffixes.borrow_mut().insert(key.clone(), suffixes);
		format!("{}_{suffix}", key.1)
	}

	/// The names an instance's argument `ty` may take, shortest first: a type's path's
	/// last segment, then its own arguments' names, lengthened in step until whole, then
	/// its path's segments, one more at a time. They read only `ty` itself, never another
	/// instance's name, so two items whose instances name each other, as `A<B<u8>>` and
	/// `B<A<u8>>`, name each without the other.
	fn arg_forms(&self, ty: &Type) -> Vec<String> {
		let (shown, args) = match ty {
			Type::Record(i) => (&self.api.records[*i].shown, &self.api.records[*i].args),
			Type::Enum(i) => (&self.api.enums[*i].shown, &self.api.enums[*i].args),
			Type::Object(i) => (&self.api.objects[*i].shown, &self.api.objects[*i].args),
			Type::Ref(_, inner) | Type::Box(inner) => return self.arg_forms(inner),
			ty => return vec![self.key(ty)],
		};
		let segments = self.segments(shown);
		let nested: Vec<Vec<String>> = args.iter().map(|arg| self.arg_forms(arg)).collect();
		let steps = nested.iter().map(Vec::len).max().unwrap_or(1);
		let form = |len: usize, step: usize| {
			std::iter::once(segments[segments.len() - len..].join("_"))
				.chain(nested.iter().map(|forms| forms[step.min(forms.len() - 1)].clone()))
				.collect::<Vec<_>>()
				.join("_")
		};
		let mut forms: Vec<String> = (0..steps).map(|step| form(1, step)).collect();
		forms.extend((2..=segments.len()).map(|len| form(len, steps - 1)));
		forms.dedup();
		forms
	}

	/// A function's C name: a method's through its type's, then each argument of the
	/// instance it is. A field's getter adds `_get`, and its setter `_set`.
	fn function_name(&self, function: &Function, owner: Option<&Ty>) -> String {
		// An item of the API it is, which outlives this, so its address names it.
		let item = Item::Function(function as *const Function as usize);
		self.renamed(item, || self.unrenamed_function(function, owner))
	}

	fn unrenamed_function(&self, function: &Function, owner: Option<&Ty>) -> String {
		let owner = match owner {
			Some(Ty::Struct(owner) | Ty::Enum(owner) | Ty::Handle { name: owner, .. }) => Some(owner.as_str()),
			_ => None,
		};
		let name = match owner {
			Some(owner) => format!("{owner}_{}", function.name()),
			None => self.name(&function.shown),
		};
		match function.access {
			Access::Field { set: false, .. } => return format!("{name}_get"),
			Access::Field { set: true, .. } => return format!("{name}_set"),
			_ => {}
		}
		let Some(instance) = function.instance.as_ref() else {
			return name;
		};
		self.instance(true, name, &instance.named, || {
			let api = self.api;
			let records = (0..api.records.len()).map(|i| (Some(self.record(i)), &api.records[i].methods));
			let enums = (0..api.enums.len()).map(|i| (Some(self.enumeration(i)), &api.enums[i].methods));
			let objects = (0..api.objects.len()).map(|i| (Some(self.object(i)), &api.objects[i].methods));
			let free = std::iter::once((None, &api.functions));
			(records.chain(enums).chain(objects).chain(free))
				.filter(|(other, _)| other.as_deref() == owner)
				.flat_map(|(_, functions)| functions)
				.filter(|other| other.path == function.path)
				.filter_map(|other| other.instance.as_ref().map(|instance| instance.named.clone()))
				.collect()
		})
	}

	pub(crate) fn callback(&self, i: usize) -> String {
		self.renamed(Item::Callback(i), || self.unrenamed(&Item::Callback(i)))
	}

	/// Each exported type's name in Pascal case, by its C name, kept [`apart`] from those
	/// beside it and [`unique`] among them, the ranges' and arrays' classes, and the language's `taken`
	/// names: `scope` gives
	/// the scope a type's [`Home`] puts it in, and how many leading segments of its path its
	/// name there leaves out.
	pub(crate) fn classes(
		&self,
		scope: impl Fn(&Home) -> (String, usize),
		taken: impl Fn(&str) -> bool,
	) -> HashMap<String, String> {
		let api = self.api;
		let records = (0..api.records.len()).map(|i| (Item::Record(i), &api.records[i].home));
		let enums = (0..api.enums.len()).map(|i| (Item::Enum(i), &api.enums[i].home));
		let objects = (0..api.objects.len()).map(|i| (Item::Object(i), &api.objects[i].home));
		let callbacks = (0..api.callbacks.len()).map(|i| (Item::Callback(i), &api.callbacks[i].home));
		let ranges: std::collections::BTreeSet<String> = self
			.ranges
			.keys()
			.map(|c| pascal(&c[self.prefix.len() + 1..]))
			.chain(self.arrays.values().map(|n| Self::array_class(*n)))
			.collect();
		let taken = move |name: &str| taken(name) || ranges.contains(name);
		let mut scopes: BTreeMap<String, Vec<(String, Vec<String>, usize)>> = BTreeMap::new();
		for (item, home) in records.chain(enums).chain(objects).chain(callbacks) {
			let (scope, outside) = scope(home);
			// Named as Rust's path is, whatever C's clashes rename.
			let segments = self.type_segments(&item);
			let c = self.renamed(item.clone(), || self.unrenamed(&item));
			scopes.entry(scope).or_default().push((c, segments, outside));
		}
		(scopes.into_values())
			.flat_map(move |types| {
				// Names still shared once `apart` has split what it can keep the segment before
				// them, a module of their own, so each of `connect::Config` and `listen::Config`
				// keeps its module. Only the names a clash would refuse change.
				let mut outside: Vec<usize> = types.iter().map(|(_, _, outside)| *outside).collect();
				let names = loop {
					let names = apart(
						types
							.iter()
							.zip(&outside)
							.map(|((_, segments, _), &outside)| &segments[outside..]),
					);
					let mut count: HashMap<&str, usize> = HashMap::new();
					for name in &names {
						*count.entry(name).or_default() += 1;
					}
					let mut grew = false;
					for (i, name) in names.iter().enumerate() {
						if count[name.as_str()] > 1 && outside[i] > 0 {
							outside[i] -= 1;
							grew = true;
						}
					}
					if !grew {
						break names;
					}
				};
				// Those still alike, as segments apart only by case are, the one Rust names in
				// Pascal case first, then by C name.
				let mut named: Vec<(bool, String, String)> = (types.into_iter().zip(names))
					.map(|((c, segments, _), name)| {
						let last = segments.last().map_or("", String::as_str);
						(pascal(last) != last, c, name)
					})
					.collect();
				named.sort();
				let (cs, names): (Vec<String>, Vec<String>) = named.into_iter().map(|(_, c, name)| (c, name)).unzip();
				cs.into_iter().zip(unique(names, &taken))
			})
			.collect()
	}

	/// The namespace of `language`'s glue: `<prefix>_<language>`, with a trailing `_` until no C name is it
	/// or starts with it and a `_`.
	pub(crate) fn space(&self, language: &str) -> String {
		let abi = self;
		let api = abi.api;
		let mut names: Vec<String> = Vec::new();
		let mut functions: Vec<(&Function, Option<Ty>)> = Vec::new();
		for (i, record) in api.records.iter().enumerate() {
			names.push(abi.record(i));
			functions.extend(record.methods.iter().map(|m| (m, Some(Ty::Struct(abi.record(i))))));
		}
		for (i, e) in api.enums.iter().enumerate() {
			names.push(abi.enumeration(i));
			functions.extend(e.methods.iter().map(|m| (m, Some(abi.ty(&Type::Enum(i))))));
		}
		for (i, object) in api.objects.iter().enumerate() {
			names.push(abi.object(i));
			let owner = Ty::Handle {
				name: abi.object(i),
				constant: false,
			};
			functions.extend(object.methods.iter().map(|m| (m, Some(owner.clone()))));
		}
		names.extend((0..api.callbacks.len()).map(|i| abi.callback(i)));
		functions.extend(api.functions.iter().map(|f| (f, None)));
		names.extend(
			functions
				.into_iter()
				.map(|(function, owner)| abi.export(function, owner.as_ref()).call.name),
		);
		let mut space = abi.rt(language);
		while names
			.iter()
			.any(|name| *name == space || name.starts_with(&format!("{space}_")))
		{
			space = stepped(&space, 1);
		}
		space
	}

	/// The prefixed name of a runtime item: `buf` is `moq_buf`.
	pub(crate) fn rt(&self, name: &str) -> String {
		format!("{}_{name}", self.prefix)
	}

	/// The C name of `Option<inner>`'s struct.
	pub(crate) fn option(&self, inner: &Type) -> String {
		join(&format!("{}_option", self.prefix), &self.key(inner))
	}

	/// The C name of `Vec<elem>`'s struct: `<prefix>_vec_string`.
	pub(crate) fn list(&self, elem: &Type) -> String {
		join(&format!("{}_vec", self.prefix), &self.key(elem))
	}

	/// The C name of a tuple's struct: `<prefix>_tuple_string_u64`.
	pub(crate) fn tuple(&self, elements: &[Type]) -> String {
		self.renamed(Item::Tuple(elements.iter().map(owned).collect()), || {
			let keys: Vec<String> = elements.iter().map(|e| self.key(e)).collect();
			format!("{}_tuple_{}", self.prefix, keys.join("_"))
		})
	}

	/// The C name of `[u8; n]`'s struct: `<prefix>_array_u8_32`.
	pub(crate) fn array(&self, n: usize) -> String {
		format!("{}_array_u8_{n}", self.prefix)
	}

	/// The class a language with no fixed-size array of its own generates for `[u8; n]`:
	/// `ArrayU8x32`, which no exported type may take.
	pub(crate) fn array_class(n: usize) -> String {
		format!("ArrayU8x{n}")
	}

	/// The C name of a range, bound, or pair of bounds struct: `<prefix>_range_u64`.
	pub(crate) fn range(&self, ty: &Type) -> String {
		join(&self.prefix, &self.key(ty))
	}

	/// A type's name within the prefix, for naming the structs and instances built from it.
	pub(crate) fn key(&self, ty: &Type) -> String {
		let strip = |name: String| name[self.prefix.len() + 1..].to_string();
		match ty {
			Type::Unit => "unit".to_string(),
			Type::Bool => "bool".to_string(),
			Type::Str | Type::String => "string".to_string(),
			Type::Slice | Type::Vec | Type::Bytes => "bytes".to_string(),
			Type::Path | Type::PathBuf => "path".to_string(),
			Type::Int(int) => int.rust().to_string(),
			// Apart from its integer's, as a binding checks it is not zero.
			Type::NonZero(int) => format!("non_zero_{}", int.rust()),
			Type::F32 => "f32".to_string(),
			Type::F64 => "f64".to_string(),
			Type::Char => "char".to_string(),
			Type::U128 => "u128".to_string(),
			Type::Duration => "duration".to_string(),
			Type::Array(n) => format!("array_u8_{n}"),
			Type::SocketAddr(family) => addr(true, *family).to_string(),
			Type::IpAddr(family) => addr(false, *family).to_string(),
			Type::Option(inner) => format!("option_{}", self.key(inner)),
			Type::List(_, inner) => format!("vec_{}", self.key(inner)),
			Type::Tuple(elements) => strip(self.tuple(elements)),
			Type::Record(i) => strip(self.record(*i)),
			Type::Enum(i) => strip(self.enumeration(*i)),
			Type::Object(i) => strip(self.object(*i)),
			Type::Range(int) => format!("range_{}", int.rust()),
			Type::Bound(int) => format!("bound_{}", int.rust()),
			Type::Bounds(int) => format!("bounds_{}", int.rust()),
			Type::Ref(_, inner) | Type::Box(inner) => self.key(inner),
			// Only an instance's name holds one, where a C identifier has no `-`.
			Type::Const(Literal::Int(_, v)) if *v < 0 => format!("neg{}", v.unsigned_abs()),
			Type::Const(value) => value.to_string(),
			other => unreachable!("no option struct holds {other:?}"),
		}
	}

	/// A type by its key, with each tuple spelled as Rust spells it, which tells apart
	/// tuples that share a C name.
	pub(crate) fn spell(&self, ty: &Type) -> String {
		match ty {
			Type::Tuple(elements) => {
				let elements: Vec<String> = elements.iter().map(|e| self.spell(e)).collect();
				match elements.len() {
					1 => format!("({},)", elements[0]),
					_ => format!("({})", elements.join(", ")),
				}
			}
			ty => self.key(ty),
		}
	}

	/// Whether a value of `ty` holds memory Rust allocated.
	pub(crate) fn owns(&self, ty: &Type) -> bool {
		self.holds(ty, false)
	}

	/// Whether C's copy of a returned `ty` holds memory Rust allocated: a reference
	/// returns as an owned copy of what it borrows.
	pub(crate) fn returns_owned(&self, ty: &Type) -> bool {
		self.holds(ty, true)
	}

	fn holds(&self, ty: &Type, refs: bool) -> bool {
		match ty {
			_ if Abi::is_buf(ty) => true,
			Type::Object(_) | Type::List(..) => true,
			Type::Ref(_, inner) if refs => self.holds(inner, refs),
			Type::Option(inner) | Type::Box(inner) => self.holds(inner, refs),
			Type::Tuple(elements) => elements.iter().any(|ty| self.holds(ty, refs)),
			Type::Record(i) => self.api.records[*i].fields.iter().any(|f| self.holds(&f.ty, refs)),
			Type::Enum(i) => self.api.enums[*i]
				.variants
				.iter()
				.any(|v| v.fields.iter().any(|f| self.holds(&f.ty, refs))),
			_ => false,
		}
	}

	/// Whether `ty` crosses as a `<prefix>_buf`.
	pub(crate) fn is_buf(ty: &Type) -> bool {
		matches!(
			ty.base(),
			Type::Str | Type::String | Type::Slice | Type::Vec | Type::Bytes | Type::Path | Type::PathBuf
		)
	}

	/// Whether `Option<ty>` crosses as `ty`, with a null pointer for `None`: a handle, a
	/// buffer, or a `Vec`'s array.
	pub(crate) fn nullable(ty: &Type) -> bool {
		matches!(ty.base(), Type::Object(_) | Type::List(..)) || Self::is_buf(ty)
	}

	/// The C type of `ty` as a value: a parameter, field, or what a function returns.
	pub(crate) fn ty(&self, ty: &Type) -> Ty {
		match ty {
			Type::Const(_) => unreachable!("a const argument is no value"),
			Type::Unit => Ty::Void,
			Type::Bool => Ty::Bool,
			Type::Int(int) | Type::NonZero(int) => Ty::Int(*int),
			Type::F32 => Ty::F32,
			Type::F64 => Ty::F64,
			Type::Char => Ty::Char,
			Type::Str | Type::String | Type::Slice | Type::Vec | Type::Bytes | Type::Path | Type::PathBuf => Ty::Buf,
			Type::U128 => Ty::U128,
			Type::Duration => Ty::Duration,
			Type::Array(n) => Ty::Struct(self.array(*n)),
			Type::SocketAddr(family) => Ty::SocketAddr(*family),
			Type::IpAddr(family) => Ty::IpAddr(*family),
			Type::Waker => Ty::Waker,
			Type::Future => Ty::ForeignFuture,
			Type::Reader => Ty::ForeignReader,
			Type::Jni(jni) => Ty::Jni(*jni),
			Type::List(_, inner) => Ty::Struct(self.list(inner)),
			Type::Option(inner) => match Self::nullable(inner) {
				true => self.ty(inner),
				false => Ty::Struct(self.option(inner)),
			},
			Type::Box(inner) => self.ty(inner),
			Type::Ref(m, inner) => match &**inner {
				Type::Object(i) => Ty::Handle {
					name: self.object(*i),
					constant: *m == crate::ir::Mutability::Not,
				},
				inner => self.ty(inner),
			},
			Type::Record(i) => Ty::Struct(self.record(*i)),
			Type::Enum(i) => match self.api.enums[*i].plain() {
				true => Ty::Enum(self.enumeration(*i)),
				false => Ty::Struct(self.enumeration(*i)),
			},
			Type::Object(i) => Ty::Handle {
				name: self.object(*i),
				constant: false,
			},
			Type::Callback(i, _) => Ty::Struct(self.callback(*i)),
			Type::Closure(closure) => self.closure(closure),
			Type::Range(_) | Type::Bound(_) | Type::Bounds(_) => Ty::Struct(self.range(ty)),
			Type::Tuple(elements) => Ty::Struct(self.tuple(elements)),
		}
	}

	/// The C function a closure is: after its data, each value it takes, lent for its
	/// call; then for a `Result`, an out pointer for its value and one for its error,
	/// returning whether it succeeded, or else its value. A buffer it returns is Rust's,
	/// one `<prefix>_buf_copy` made.
	pub(crate) fn closure(&self, closure: &Closure) -> Ty {
		let mut params: Vec<Ty> = closure.params.iter().map(|ty| self.ty(ty)).collect();
		let ok = self.ty(&closure.ok);
		match &closure.err {
			None => Ty::Fn(params, Box::new(ok)),
			Some(err) => {
				if ok != Ty::Void {
					params.push(Ty::Out(Box::new(ok)));
				}
				params.push(Ty::Out(Box::new(self.ty(err))));
				Ty::Fn(params, Box::new(Ty::Bool))
			}
		}
	}

	/// The C type of `ty` returned: a borrow is returned as a clone, so a handle in it is
	/// the caller's own.
	fn returned(&self, ty: &Type) -> Ty {
		self.ty(&owned(ty))
	}

	/// The C signature of `function`, a method of the type named `owner` if it has one.
	pub(crate) fn export(&self, function: &Function, owner: Option<&Ty>) -> Export {
		let name = self.function_name(function, owner);
		let named = self.unrenamed_function(function, owner);
		let mut params = Vec::new();
		if let (Some(receiver), Some(owner)) = (function.receiver, owner) {
			let ty = match owner {
				Ty::Handle { name, .. } => Ty::Handle {
					name: name.clone(),
					constant: receiver == Receiver::Ref
						|| (receiver == Receiver::Owned
							&& (0..self.api.objects.len())
								.any(|i| self.api.objects[i].copy && &self.object(i) == name)),
				},
				// A value's receiver is read through a pointer, and written back in place for
				// `&mut self`.
				value if receiver == Receiver::Mut => Ty::InOut(Box::new(value.clone())),
				value => Ty::In(Box::new(value.clone())),
			};
			params.push(("self".to_string(), ty));
		}
		// What Rust writes back for each value C passes for `&mut`, after every parameter.
		let mut written = Vec::new();
		for param in &function.params {
			let copied = match &param.ty {
				Type::Object(i) => Some(*i),
				Type::Option(inner) => match **inner {
					Type::Object(i) => Some(i),
					_ => None,
				},
				_ => None,
			};
			let ty = match copied {
				// Taken by value, a `Copy` object is copied out, leaving its handle as it was.
				Some(i) if self.api.objects[i].copy => Ty::Handle {
					name: self.object(i),
					constant: true,
				},
				_ => self.ty(&param.ty),
			};
			let name = escape(&param.name);
			if let Type::Closure(closure) = &param.ty {
				let taken: Vec<String> = params.iter().map(|(name, _)| name.clone()).collect();
				let data = beside(function, &taken, join(&name, "data"));
				let free = beside(function, &taken, join(&name, "free"));
				params.push((name, ty));
				params.push((data, Ty::Data));
				// A kept closure's free, which Rust calls with its data once it drops it.
				if closure.kept {
					params.push((free, Ty::Fn(Vec::new(), Box::new(Ty::Void))));
				}
				continue;
			}
			if param.ty.written() {
				written.push((Self::written(function, &name), Ty::Out(Box::new(ty.clone()))));
			}
			params.push((name, ty));
		}
		// A value returned is C's return, after which only the written outs follow.
		let writes = written.len();
		let mut outs = written;
		match &function.ok {
			Type::Unit => {}
			Type::Tuple(elements) => {
				for (n, element) in elements.iter().enumerate() {
					outs.push((format!("out{n}"), Ty::Out(Box::new(self.returned(element)))));
				}
			}
			ok => outs.push(("out".to_string(), Ty::Out(Box::new(self.returned(ok))))),
		}
		if let Some(err) = &function.err {
			outs.push(("err".to_string(), Ty::Out(Box::new(self.returned(err)))));
		}
		let status = (self.status(function, owner)).then(|| {
			let status = beside(function, &[], "status".to_string());
			(status, Ty::Out(Box::new(Ty::Status)))
		});
		if function.future {
			let handle = join(&name, "future");
			let mut poll_params = vec![
				("future".to_string(), Ty::Future(handle.clone())),
				("waker".to_string(), Ty::Waker),
			];
			poll_params.extend(outs.iter().cloned());
			// The status trails each, as every function's does.
			let mut start_params = params.clone();
			start_params.extend(outs);
			start_params.push(("future".to_string(), Ty::Out(Box::new(Ty::Future(handle.clone())))));
			start_params.extend(status.clone());
			params.extend(status);
			let start = Sig {
				name: join(&name, "start"),
				params: start_params,
				ret: Ty::Poll,
			};
			// A poll of a future that is not `Send` fails on another thread than its own.
			if !function.send {
				poll_params.push(("status".to_string(), Ty::Out(Box::new(Ty::Status))));
			}
			let poll = Sig {
				name: join(&name, "poll"),
				params: poll_params,
				ret: Ty::Poll,
			};
			let free = Sig {
				name: join(&name, "future_free"),
				params: vec![("future".to_string(), Ty::Future(handle.clone()))],
				ret: Ty::Void,
			};
			return Export {
				call: Sig {
					name,
					params,
					ret: Ty::Future(handle.clone()),
				},
				future: Some(Future {
					handle,
					start,
					poll,
					free,
				}),
				named,
				homed: self.homed(function, owner),
			};
		}
		let ret = match (&function.err, &function.ok) {
			(Some(_), _) => {
				params.extend(outs);
				Ty::Bool
			}
			(None, Type::Tuple(_)) => {
				params.extend(outs);
				Ty::Void
			}
			(None, ok) => {
				params.extend(outs.into_iter().take(writes));
				self.returned(ok)
			}
		};
		params.extend(status);
		Export {
			call: Sig { name, params, ret },
			future: None,
			named,
			homed: self.homed(function, owner),
		}
	}

	/// Whether a call of `function`, a method of `owner` if it has one, makes a value that
	/// is not `Send`, an object or a future, without taking one: see [`Export::homed`].
	pub(crate) fn homed(&self, function: &Function, owner: Option<&Ty>) -> bool {
		let unsent = |ty: &Type| {
			let mut handles = Vec::new();
			self.api.handles(ty, &mut handles, &mut Vec::new());
			handles.iter().any(|&i| !self.api.objects[i].send)
		};
		let makes =
			(function.future && !function.send) || unsent(&function.ok) || function.err.as_ref().is_some_and(unsent);
		let receiver = match (function.receiver, owner) {
			(Some(_), Some(Ty::Handle { name, .. })) => {
				(0..self.api.objects.len()).any(|i| &self.object(i) == name && !self.api.objects[i].send)
			}
			(Some(_), Some(owner)) => {
				let records = (0..self.api.records.len()).map(Type::Record);
				let enums = (0..self.api.enums.len()).map(Type::Enum);
				records
					.chain(enums)
					.find(|ty| &self.ty(ty) == owner)
					.is_some_and(|ty| unsent(&ty))
			}
			_ => false,
		};
		// A handle that may be absent leaves the call to a home when it is: the twin runs it
		// where the first present one stays, else on a home.
		let optional = |ty: &Type| match ty {
			Type::Option(inner) => match &**inner {
				Type::Object(i) => !self.api.objects[*i].send,
				Type::Ref(_, inner) => matches!(&**inner, Type::Object(i) if !self.api.objects[*i].send),
				_ => false,
			},
			_ => false,
		};
		// A JNI env is its thread's, so a call taking one stays there.
		makes && !receiver && !function.jni() && function.params.iter().all(|p| optional(&p.ty) || !unsent(&p.ty))
	}

	/// Whether a call of `function`, a method of `owner` if it has one, can fail a check of
	/// a handle it borrows or moves, so it takes a `status` to write why.
	pub(crate) fn status(&self, function: &Function, owner: Option<&Ty>) -> bool {
		let mut handles = Vec::new();
		let mut seen = Vec::new();
		match (function.receiver, owner) {
			(Some(_), Some(Ty::Handle { name, .. })) => {
				handles.extend((0..self.api.objects.len()).find(|&i| &self.object(i) == name))
			}
			// A record's or enum's receiver lifts each handle it holds.
			(Some(_), Some(owner)) => {
				let records = (0..self.api.records.len()).map(Type::Record);
				let enums = (0..self.api.enums.len()).map(Type::Enum);
				if let Some(ty) = records.chain(enums).find(|ty| &self.ty(ty) == owner) {
					self.api.handles(&ty, &mut handles, &mut seen);
				}
			}
			_ => {}
		}
		for param in &function.params {
			self.api.handles(&param.ty, &mut handles, &mut seen);
		}
		handles.iter().any(|&i| self.checked[i])
	}

	/// Whether `<T>_clone`, `_eq`, `_to_string`, `_debug`, and `_copy` of object `i` take a
	/// `status`, as each borrows its handles shared.
	pub(crate) fn traits_status(&self, i: usize) -> bool {
		self.checked[i]
	}

	/// Whether a record's or enum's `<T>_eq`, `_to_string`, and `_debug` take a `status`,
	/// as each borrows the handles it holds shared.
	pub(crate) fn value_status(&self, ty: &Type) -> bool {
		let mut handles = Vec::new();
		self.api.handles(ty, &mut handles, &mut Vec::new());
		handles.iter().any(|&i| self.checked[i])
	}

	/// Whether the std trait functions of `ty`, an object, record, or enum, take a `status`.
	pub(crate) fn traits_status_of(&self, ty: &Type) -> bool {
		match ty {
			Type::Object(i) => self.traits_status(*i),
			ty => self.value_status(ty),
		}
	}

	/// Whether a call of `function`, a method of `owner` if it has one, is a leaf: proven to
	/// run no code but the glue's, its runtime's, and std's, to wait on no lock, and to call
	/// into no binding. Python then keeps the GIL across it, Dart makes it a leaf call,
	/// which runs on the isolate's own thread with no thread hop, and Kotlin marks it
	/// `@FastNative`, which ART's collector waits on, so its bound keeps a collection from
	/// stalling behind it. It is a `&self` or
	/// `&mut self` method, or the read or write of a `pub` field, of a `Send` object, which no
	/// object that is not `Send` may lend, taking only scalars and returning one or nothing,
	/// whose Rust [`Function::leaf`] proves: each body it runs, the method's and each
	/// `Deref`'s on the way, calls only bodies proven the same way and core's intrinsics that
	/// only compute, drops nothing whose drop runs other code, loops nowhere, recurses
	/// nowhere, asserts nothing, which would run the panic hook on failure, and uses no
	/// static. Its MIR proves it, which is the code that runs, not how its source looks or
	/// what an attribute claims: the MIR the export's check compiles under the profile it
	/// checks, release unless its cargo args pick another, with every overflow asserted.
	/// Each binding makes it a leaf call only where the glue's build script finds that
	/// profile's `PROFILE`, directory, `OPT_LEVEL`, and debug assertions, and the crate's
	/// panic strategy and debug assertions are the check's too, as `PROVEN` says; else a
	/// build may compile code the proof never saw,
	/// under a `cfg` such as `debug_assertions` or one a build script derives, and the call
	/// is made as any other.
	///
	/// Why holding the GIL across one cannot deadlock: a deadlock needs the call to wait on
	/// another thread that waits on the GIL. A leaf's only shared state is the handle's
	/// borrow, which fails rather than waits on a conflicting borrow, and spins only through
	/// another thread's few instructions inside the borrow check itself, which take no GIL.
	/// Its value is `Send`, so its borrow runs no `Thread::own`, whose drain would drop what
	/// other threads queued of the thread's values that are not `Send`, by any `Drop`; a
	/// handle lent from another borrows that one too, so no owner up its chain of lenders
	/// may be other than `Send` either, and that chain is no longer than the export's objects
	/// are many, as no cycle of lending methods reaches the object. A body with no loop,
	/// no recursion, and no call but to such bodies ends in a bounded number of steps whoever
	/// holds the GIL, and since it runs no Python, nothing in it needs the GIL either.
	pub(crate) fn leaf(&self, function: &Function, owner: Option<&Type>) -> bool {
		let Some(Type::Object(i)) = owner else {
			return false;
		};
		let shape = match function.access {
			Access::Field { set: false, .. } => {
				matches!(&function.ok, Type::Ref(_, ty) if ty.scalar()) && function.params.is_empty()
			}
			Access::Field { set: true, .. } => matches!(&function.params[..], [param] if param.ty.scalar()),
			Access::Call => {
				matches!(function.receiver, Some(Receiver::Ref | Receiver::Mut))
					&& (function.ok.scalar() || matches!(function.ok, Type::Unit))
					&& function.params.iter().all(|p| p.ty.scalar())
			}
			_ => false,
		};
		shape && function.leaf && !self.affine[*i] && !self.chained[*i] && function.err.is_none() && !function.future
	}

	/// Whether `ty`'s `<T>_eq`, or with `hash` its `<T>_hash`, is a leaf, as [`Self::leaf`]
	/// says: its impl is derived or std's down to primitives, and it borrows only the handle
	/// of a `Send` object no other that is not `Send` may lend, through a chain of lenders
	/// with a bound, or as a value, none.
	pub(crate) fn leaf_trait(&self, ty: &Type, hash: bool) -> bool {
		let (traits, handles) = match ty {
			Type::Object(i) => (self.api.objects[*i].traits, self.affine[*i] || self.chained[*i]),
			Type::Record(i) => (self.api.records[*i].traits, self.holds_handles(ty)),
			Type::Enum(i) => (self.api.enums[*i].traits, self.holds_handles(ty)),
			_ => return false,
		};
		!handles
			&& match hash {
				true => traits.leaf_hash,
				false => traits.leaf_eq,
			}
	}

	/// Whether lifting a value of `ty` borrows any handle.
	fn holds_handles(&self, ty: &Type) -> bool {
		let mut handles = Vec::new();
		self.api.handles(ty, &mut handles, &mut Vec::new());
		!handles.is_empty()
	}

	/// The out pointer Rust writes back the value C passed for `&mut` as parameter `name`
	/// through, named apart from every parameter of `function` and every earlier one's out.
	pub(crate) fn written(function: &Function, name: &str) -> String {
		let mut taken = Vec::new();
		for param in function.params.iter().filter(|param| param.ty.written()) {
			let escaped = escape(&param.name);
			let out = beside(function, &taken, join(&escaped, "out"));
			if escaped == name {
				return out;
			}
			taken.push(out);
		}
		unreachable!("`{name}` is a parameter of `{}` Rust writes back", function.name())
	}

	/// A value's `&mut self` method as a language whose values are immutable calls it:
	/// through `&self`, returning the value as the call left it, before Rust's own return
	/// or each element of its tuple: that function, its export, and how the glue calls it.
	/// The export keeps the C name of the method's, which a clash may have renamed. `None`
	/// for any other function.
	pub(crate) fn handed_back(
		&self,
		function: &Function,
		owner: &Ty,
		owner_ty: &Type,
	) -> Option<(Function, Export, Handed)> {
		if function.receiver != Some(Receiver::Mut) || matches!(owner, Ty::Handle { .. }) {
			return None;
		}
		// `written_back` in collect.rs refuses a future's and a `Result`'s.
		assert!(
			!function.future && function.err.is_none(),
			"a value's `&mut self` returns a plain value"
		);
		let real = self.export(function, Some(owner));
		let returned = match &function.ok {
			Type::Unit => Vec::new(),
			Type::Tuple(elements) => elements.clone(),
			ok => vec![ok.clone()],
		};
		let back = Function {
			receiver: Some(Receiver::Ref),
			ok: match returned.is_empty() {
				true => owner_ty.clone(),
				false => Type::Tuple([vec![owner_ty.clone()], returned].concat()),
			},
			..function.clone()
		};
		// Named as the method is: a rename is keyed by the API's own function.
		let mut export = self.export(&back, Some(owner));
		export.call.name = real.call.name.clone();
		export.named = real.named;
		let elements = match &function.ok {
			Type::Tuple(elements) => elements.len(),
			_ => 0,
		};
		let status = usize::from(real.call.status().is_some());
		let handed = Handed {
			lead: real.call.params.len() - 1 - elements - status,
			status: status == 1,
			name: real.call.name,
			unit: function.ok == Type::Unit,
			tuple: elements > 0,
		};
		Some((back, export, handed))
	}

	fn claim(&mut self, name: String, what: String, span: Span, owner: Option<Owner>) {
		self.claims.push(Claim {
			name,
			what,
			span,
			owner,
		});
	}

	/// The owner of a name made from `item`.
	fn made(item: &Item) -> Option<Owner> {
		Some(Owner {
			item: item.clone(),
			own: false,
			spelled: false,
		})
	}

	/// The owner of `item`'s own C name, `base`, which joins the prefix and `segments`
	/// unless a join or a rename respelled it.
	fn own(&self, item: &Item, base: &str, segments: Vec<String>) -> Option<Owner> {
		let prefix = match base.starts_with(&self.prefix) {
			true => self.prefix.clone(),
			false => self.prefix.to_uppercase(),
		};
		let spelled: Vec<String> = std::iter::once(prefix).chain(segments).collect();
		Some(Owner {
			item: item.clone(),
			own: true,
			spelled: spelled.join("_") == base,
		})
	}

	/// The item a name made from `ty` belongs to: a type, or a tuple.
	fn item_of(&self, ty: &Type) -> Option<Item> {
		match ty {
			Type::Record(i) => Some(Item::Record(*i)),
			Type::Enum(i) => Some(Item::Enum(*i)),
			Type::Object(i) => Some(Item::Object(*i)),
			Type::Tuple(elements) => Some(Item::Tuple(elements.iter().map(owned).collect())),
			Type::Ref(_, inner) | Type::Box(inner) => self.item_of(inner),
			_ => None,
		}
	}

	/// Claim every name the export defines, and note each option struct.
	fn claim_all(&mut self) {
		let api = self.api;
		let crate_span = api
			.records
			.first()
			.map(|r| r.span)
			.or_else(|| api.objects.first().map(|o| o.span))
			.unwrap_or(rustc_span::DUMMY_SP);
		// C's runtime, then what C++'s adds in the namespace its types share, Kotlin's in its
		// package, and Swift's in its module and the atomics header its runtime adds.
		for runtime in [
			"contract",
			"target",
			"status",
			"status_message",
			"buf",
			"buf_free",
			"u128",
			"duration",
			"socket_addr",
			"socket_addr_v4",
			"socket_addr_v6",
			"ip_addr",
			"ipv4_addr",
			"ipv6_addr",
			"waker",
			"foreign_future",
			"foreign_reader",
			"buf_copy",
			"buf_clone",
			"thread_wake",
			"thread_drain",
			"close_drain",
			// Python's extension module function, beside each native function by this name.
			"_register",
			"poll",
			"view",
			"items",
			"taken",
			"borrow",
			"result",
			"task",
			"future",
			"poll_t",
			"detail",
			"native",
			"bytes",
			"input",
			"mut",
			"atomic_load",
			"atomic_store",
			"atomic_add",
			"atomic_exchange",
			"atomic_cas",
		] {
			self.claim(self.rt(runtime), "the runtime".to_string(), crate_span, None);
		}
		for constant in ["CONTRACT", "POLL_PENDING", "POLL_READY", "POLL_ERROR", "ATOMICS_H"] {
			let name = format!("{}_{constant}", self.prefix.to_uppercase());
			self.claim(name, "the runtime".to_string(), crate_span, None);
		}
		for (status, _) in STATUSES {
			let name = format!("{}_STATUS_{}", self.prefix.to_uppercase(), status.to_uppercase());
			self.claim(name, "the runtime".to_string(), crate_span, None);
		}
		let mut types: Vec<&Type> = Vec::new();
		// Each function, with its type's C name and segments if it is a method.
		let mut functions: Vec<(&Function, Option<Ty>, Vec<String>)> = Vec::new();
		for (i, record) in api.records.iter().enumerate() {
			let item = Item::Record(i);
			let name = self.record(i);
			let segments = self.type_segments(&item);
			let what = format!("struct `{}`", api.rust(&record.path, &record.args));
			self.claim_traits(&item, &name, &what, record.span, record.traits, true);
			self.claim(
				name.clone(),
				what,
				record.span,
				self.own(&item, &name, segments.clone()),
			);
			types.extend(record.fields.iter().map(|f| &f.ty));
			functions.extend(
				record
					.methods
					.iter()
					.map(|m| (m, Some(Ty::Struct(name.clone())), segments.clone())),
			);
		}
		for (i, e) in api.enums.iter().enumerate() {
			let item = Item::Enum(i);
			let name = self.enumeration(i);
			let segments = self.type_segments(&item);
			let what = format!("enum `{}`", api.rust(&e.path, &e.args));
			let upper = name.to_uppercase();
			for variant in &e.variants {
				self.claim(
					join(&upper, &snake(&variant.name).to_uppercase()),
					what.clone(),
					e.span,
					Self::made(&item),
				);
				types.extend(variant.fields.iter().map(|f| &f.ty));
			}
			let plain = e.plain();
			if !plain {
				self.claim(join(&name, "tag"), what.clone(), e.span, Self::made(&item));
				// Each variant's payload, which C++ names even when it has no fields.
				for variant in &e.variants {
					self.claim(payload(&name, &variant.name), what.clone(), e.span, Self::made(&item));
				}
			}
			self.claim_traits(&item, &name, &what, e.span, e.traits, !plain);
			self.claim(name.clone(), what, e.span, self.own(&item, &name, segments.clone()));
			let owner = match plain {
				true => Ty::Enum(name.clone()),
				false => Ty::Struct(name.clone()),
			};
			functions.extend(e.methods.iter().map(|m| (m, Some(owner.clone()), segments.clone())));
		}
		for (i, object) in api.objects.iter().enumerate() {
			let item = Item::Object(i);
			let name = self.object(i);
			let segments = self.type_segments(&item);
			let what = format!("type `{}`", api.rust(&object.path, &object.args));
			self.claim(join(&name, "free"), what.clone(), object.span, Self::made(&item));
			if self.checked[i] {
				for made in ["drop", "drop_signal"] {
					self.claim(join(&name, made), what.clone(), object.span, Self::made(&item));
				}
			}
			if object.shared {
				self.claim(join(&name, "borrowed"), what.clone(), object.span, Self::made(&item));
			}
			self.claim_traits(&item, &name, &what, object.span, object.traits, false);
			// `Default` of a value that is not `Send` makes one from none.
			if object.traits.default && !object.send {
				self.claim(
					join(&name, "default_homed"),
					what.clone(),
					object.span,
					Self::made(&item),
				);
			}
			self.claim(
				name.clone(),
				what,
				object.span,
				self.own(&item, &name, segments.clone()),
			);
			let owner = Ty::Handle { name, constant: false };
			functions.extend(
				object
					.methods
					.iter()
					.map(|m| (m, Some(owner.clone()), segments.clone())),
			);
		}
		for (i, callback) in api.callbacks.iter().enumerate() {
			let item = Item::Callback(i);
			let name = self.callback(i);
			let segments = self.type_segments(&item);
			let what = format!("trait `{}`", callback.path.join("::"));
			self.claim(join(&name, "vtable"), what.clone(), callback.span, Self::made(&item));
			self.claim(name.clone(), what, callback.span, self.own(&item, &name, segments));
			for method in &callback.methods {
				types.extend(method.params.iter().map(|p| &p.ty));
				types.push(&method.ok);
				types.extend(&method.err);
			}
		}
		functions.extend(api.functions.iter().map(|f| (f, None, Vec::new())));
		for (function, owner, mut segments) in functions {
			let item = Item::Function(function as *const Function as usize);
			let export = self.export(function, owner.as_ref());
			let name = export.call.name.clone();
			let what = match function.access {
				Access::Field { set: false, .. } => format!("field `{}`", function.path.join("::")),
				Access::Field { set: true, .. } => format!("setter of field `{}`", function.path.join("::")),
				_ => format!("fn `{}`", function.path.join("::")),
			};
			// Its own segments: its type's then its name's, or its path's, then a field's
			// accessor and an instance's suffix.
			match owner {
				Some(_) => segments.push(function.name().to_string()),
				None => segments = self.segments(&function.shown),
			}
			match function.access {
				Access::Field { set: false, .. } => segments.push("get".to_string()),
				Access::Field { set: true, .. } => segments.push("set".to_string()),
				_ => {}
			}
			let unrenamed = self.unrenamed_function(function, owner.as_ref());
			let joined = format!("{}_{}", self.prefix, segments.join("_"));
			segments.extend(
				unrenamed
					.strip_prefix(&joined)
					.and_then(|s| s.strip_prefix('_'))
					.map(str::to_string),
			);
			let own = self.own(&item, &name, segments);
			self.claim(name.clone(), what.clone(), function.span, own);
			if let Some(Future {
				handle,
				start,
				poll,
				free,
			}) = &export.future
			{
				for made in [handle, &start.name, &poll.name, &free.name] {
					self.claim(made.clone(), what.clone(), function.span, Self::made(&item));
				}
			}
			for homed in export.homed_call().into_iter().chain(export.homed_start()) {
				self.claim(homed.name, what.clone(), function.span, Self::made(&item));
			}
			types.extend(function.params.iter().map(|p| &p.ty));
			// A tuple returned whole is written element by element, with no struct of its own.
			match &function.ok {
				Type::Tuple(elements) => types.extend(elements),
				ok => types.push(ok),
			}
			types.extend(&function.err);
		}
		for constant in &api.constants {
			let item = Item::Constant(constant.path.clone());
			let name = self.constant(&constant.path);
			let segments: Vec<String> = self.segments(&constant.path).iter().map(|s| s.to_uppercase()).collect();
			let own = self.own(&item, &name, segments);
			let what = format!("const `{}`", constant.path.join("::"));
			self.claim(name, what, constant.span, own);
		}
		let mut options = BTreeMap::new();
		let mut ranges = BTreeMap::new();
		let mut lists = BTreeMap::new();
		let mut tuples: BTreeMap<String, Vec<Type>> = BTreeMap::new();
		let mut arrays = BTreeMap::new();
		let mut mixed = Vec::new();
		let mut stack = types;
		while let Some(ty) = stack.pop() {
			match ty {
				Type::Option(inner) => {
					if !Self::nullable(inner) {
						options.insert(self.option(inner), (**inner).clone());
					}
					stack.push(inner);
				}
				Type::List(_, inner) => {
					// A returned `Vec<&T>` holds clones, which its free frees.
					lists.insert(self.list(inner), owned(inner));
					stack.push(inner);
				}
				Type::Ref(_, inner) | Type::Box(inner) => stack.push(inner),
				Type::Tuple(elements) => {
					// A returned tuple of borrows holds clones, as a `Vec` does.
					let owned: Vec<Type> = elements.iter().map(owned).collect();
					let name = self.tuple(elements);
					let layout = |elements: &[Type]| elements.iter().map(|e| self.ty(e)).collect::<Vec<_>>();
					match tuples.get(&name) {
						// Its name runs its elements' together, which two nestings can share.
						Some(other) if layout(other) != layout(&owned) => mixed.push((name, owned)),
						Some(_) => {}
						None => {
							tuples.insert(name, owned);
						}
					}
					stack.extend(elements);
				}
				Type::Closure(closure) => {
					stack.extend(&closure.params);
					stack.push(&closure.ok);
					stack.extend(&closure.err);
				}
				Type::Range(_) | Type::Bound(_) => {
					ranges.insert(self.range(ty), ty.clone());
				}
				Type::Array(n) => {
					arrays.insert(self.array(*n), *n);
				}
				Type::Bounds(int) => {
					ranges.insert(self.range(ty), ty.clone());
					ranges.insert(self.range(&Type::Bound(*int)), Type::Bound(*int));
				}
				_ => {}
			}
		}
		for (name, inner) in &options {
			let owner = self.item_of(inner).and_then(|item| Self::made(&item));
			self.claim(name.clone(), "an option".to_string(), crate_span, owner);
		}
		for name in ranges.keys() {
			self.claim(name.clone(), "a range".to_string(), crate_span, None);
		}
		for name in arrays.keys() {
			self.claim(name.clone(), "an array".to_string(), crate_span, None);
		}
		for (name, elements) in tuples
			.iter()
			.chain(mixed.iter().map(|(name, elements)| (name, elements)))
		{
			let ty = Type::Tuple(elements.clone());
			let what = format!("tuple `{}`", self.spell(&ty));
			let owner = self.item_of(&ty).and_then(|item| Self::made(&item));
			self.claim(name.clone(), what, crate_span, owner);
		}
		for (name, inner) in &lists {
			let item = self.item_of(inner);
			let owner = || item.as_ref().and_then(Self::made);
			self.claim(name.clone(), "a Vec".to_string(), crate_span, owner());
			self.claim(join(name, "free"), "a Vec".to_string(), crate_span, owner());
		}
		if ranges.values().any(|ty| matches!(ty, Type::Bound(_))) {
			let bound = self.rt("bound");
			for kind in ["", "_UNBOUNDED", "_INCLUDED", "_EXCLUDED"] {
				let name = match kind {
					"" => bound.clone(),
					kind => format!("{}{kind}", bound.to_uppercase()),
				};
				self.claim(name, "a bound".to_string(), crate_span, None);
			}
		}
		self.options = options;
		self.ranges = ranges;
		self.lists = lists;
		self.tuples = tuples;
		self.arrays = arrays;
	}

	/// Claim the functions a type's std traits export.
	fn claim_traits(
		&mut self,
		item: &Item,
		name: &str,
		what: &str,
		span: Span,
		traits: crate::ir::Traits,
		owned: bool,
	) {
		let mut claim = |suffix: &str| self.claim(join(name, suffix), what.to_string(), span, Self::made(item));
		if traits.clone && !owned {
			claim("clone");
		}
		if traits.eq {
			claim("eq");
		}
		if traits.hash {
			claim("hash");
		}
		if traits.ord != Order::None {
			claim("partial_cmp");
		}
		if traits.ord == Order::Total {
			claim("cmp");
		}
		if traits.display {
			claim("to_string");
		}
		if traits.debug {
			claim("debug");
		}
		if traits.default {
			claim("default");
		}
		if owned {
			claim("free");
		}
	}
}

/// The suffix naming each of a generic item's instances, given the names each of its
/// arguments may take, shortest first: each argument's shortest, lengthened by one at a
/// time only where two instances' suffixes are alike while their arguments differ there.
/// Each suffix depends on the whole group, not its order, so an instance added later can
/// lengthen another's name. Instances alike at every length keep their clash, which
/// claiming their C names reports.
fn shortest(group: &[Vec<Vec<String>>]) -> Vec<String> {
	let mut lens: Vec<Vec<usize>> = group.iter().map(|args| vec![1; args.len()]).collect();
	loop {
		let names: Vec<String> = (group.iter().zip(&lens))
			.map(|(args, lens)| {
				(args.iter().zip(lens))
					.map(|(forms, &len)| forms[len - 1].as_str())
					.collect::<Vec<_>>()
					.join("_")
			})
			.collect();
		let mut grow = std::collections::BTreeSet::new();
		for (a, b) in (0..group.len()).flat_map(|a| (0..group.len()).map(move |b| (a, b))) {
			if a == b || names[a] != names[b] {
				continue;
			}
			for (j, forms) in group[a].iter().enumerate() {
				if group[b].get(j) != Some(forms) && lens[a][j] < forms.len() {
					grow.insert((a, j));
				}
			}
		}
		if grow.is_empty() {
			return names;
		}
		for (a, j) in grow {
			lens[a][j] += 1;
		}
	}
}

/// Each name in Pascal case of items by their paths' segments: the segments joined, but
/// where two would join alike, each of those keeps a `_` at every split after the leading
/// segments they all share, so `tokio::ConnectError` and `tokio::connect::Error` are
/// `TokioConnectError` and `TokioConnect_Error`. Two that still match, as segments apart
/// only by case do, stay alike for the language's claim to refuse at both.
pub(crate) fn apart<'s>(items: impl IntoIterator<Item = &'s [String]>) -> Vec<String> {
	let items: Vec<&[String]> = items.into_iter().collect();
	let joined: Vec<String> = items.iter().map(|segments| pascal(&segments.join("_"))).collect();
	let mut groups: HashMap<&str, Vec<usize>> = HashMap::new();
	for (i, name) in joined.iter().enumerate() {
		groups.entry(name).or_default().push(i);
	}
	let mut names = joined.clone();
	for group in groups.values().filter(|group| group.len() > 1) {
		let first = items[group[0]];
		let shared = (0..first.len())
			.take_while(|&k| group.iter().all(|&i| items[i].get(k) == Some(&first[k])))
			.count();
		for &i in group {
			let rest: Vec<String> = items[i][shared..].iter().map(|segment| pascal(segment)).collect();
			names[i] = pascal(&items[i][..shared].join("_")) + &rest.join("_");
		}
	}
	names
}

/// A name in Pascal case, word by word as [`snake`] splits it: `track_Producer` and
/// `TrackProducer` are `TrackProducer`, and `BOXED` is `Boxed`.
pub(crate) fn pascal(name: &str) -> String {
	snake(name)
		.split('_')
		.filter(|s| !s.is_empty())
		.map(|s| {
			let mut chars = s.chars();
			match chars.next() {
				Some(first) => first.to_uppercase().chain(chars).collect(),
				None => String::new(),
			}
		})
		.collect()
}

/// The C name of the struct of an enum's variant's fields: its own in Rust's case, after
/// the enum's.
pub(crate) fn payload(name: &str, variant: &str) -> String {
	join(name, variant)
}

/// A Rust identifier in snake case: `TrackProducer` is `track_producer`, and a name
/// already in snake case stays as it is.
pub(crate) fn snake(name: &str) -> String {
	let chars: Vec<char> = name.chars().collect();
	let mut out = String::new();
	for (i, &c) in chars.iter().enumerate() {
		if c.is_uppercase() {
			let prev = i.checked_sub(1).map(|j| chars[j]);
			let next = chars.get(i + 1);
			let boundary = match prev {
				Some(p) if p.is_lowercase() || p.is_ascii_digit() => true,
				Some(p) if p.is_uppercase() => next.is_some_and(|n| n.is_lowercase()),
				_ => false,
			};
			if boundary && !out.ends_with('_') {
				out.push('_');
			}
			out.extend(c.to_lowercase());
		} else {
			out.push(c);
		}
	}
	out
}

/// The name `what` takes in a scope whose names so far are `names`: `name`, with a trailing
/// `_` until neither `taken` nor an earlier item names it.
pub(crate) fn claim(
	names: &mut BTreeMap<String, String>,
	taken: impl Fn(&str) -> bool,
	name: &str,
	what: String,
) -> String {
	let mut free = name.to_string();
	while taken(&free) || names.contains_key(&free) {
		free.push('_');
	}
	names.insert(free.clone(), what);
	free
}

/// Names made distinct from each other and from what `taken` names: one taken, or an
/// earlier one's, takes a trailing `_` until it is neither, never becoming another's own.
pub(crate) fn unique(names: Vec<String>, taken: impl Fn(&str) -> bool) -> Vec<String> {
	let mut used = std::collections::BTreeSet::new();
	(names.iter())
		.map(|name| {
			let mut unique = name.clone();
			while taken(&unique) || used.contains(&unique) || (unique != *name && names.contains(&unique)) {
				unique.push('_');
			}
			used.insert(unique.clone());
			unique
		})
		.collect()
}

/// A language's names of `rust`, the names of one scope, kept apart from each other and from
/// what `taken` names for each, by its index, whatever their order. `spelled` is each one's
/// spelling in the language, and `own` whether a spelling reads back as its Rust name. One
/// that does keeps it while neither `taken` nor another that does and is not taken holds it,
/// and every other, in Rust's order of names, takes the first of its `step`s that is free, as
/// [`underscored`] counts them. So beside `new`, which keeps `new`, a `new_` that camel case
/// spells `new` takes `new_`, and no order of the source moves either.
pub(crate) fn named(
	rust: &[&str],
	spelled: Vec<String>,
	own: impl Fn(&str, &str) -> bool,
	taken: impl Fn(usize, &str) -> bool,
	step: impl Fn(&str, usize) -> String,
) -> Vec<String> {
	named_apart(rust, spelled, own, taken, step, |_, a, _, b| a == b)
}

/// [`named`]'s names, where two items clash only as `clash` reads them, each by its index and
/// name, for a language whose items of one scope may share a name, such as Dart's named
/// constructor and instance method, or Swift's overloads of different labels.
pub(crate) fn named_apart(
	rust: &[&str],
	spelled: Vec<String>,
	own: impl Fn(&str, &str) -> bool,
	taken: impl Fn(usize, &str) -> bool,
	step: impl Fn(&str, usize) -> String,
	clash: impl Fn(usize, &str, usize, &str) -> bool,
) -> Vec<String> {
	let mut names = spelled.clone();
	// Those that could keep their spelling: one taken moves anyway, so it holds back no other.
	let owns: Vec<bool> = (0..rust.len())
		.map(|k| own(rust[k], &spelled[k]) && !taken(k, &spelled[k]))
		.collect();
	let kept: Vec<bool> = (0..rust.len())
		.map(|k| owns[k] && (0..rust.len()).all(|j| j == k || !owns[j] || !clash(k, &spelled[k], j, &spelled[j])))
		.collect();
	let mut held: Vec<usize> = (0..rust.len()).filter(|&k| kept[k]).collect();
	let mut moved: Vec<usize> = (0..rust.len()).filter(|&k| !kept[k]).collect();
	moved.sort_by_key(|&k| rust[k]);
	for k in moved {
		let mut n = 0;
		while taken(k, &names[k]) || held.iter().any(|&j| clash(k, &names[k], j, &names[j])) {
			n += 1;
			names[k] = step(&spelled[k], n);
		}
		held.push(k);
	}
	names
}

/// Asserts that no two of the `members` of the type `rust` in `language` are one, as `same`
/// reads them. Each generator names a scope's members apart through [`named`], and from
/// every other scope of the type that they share, so two alike are its bug, not the crate's.
pub(crate) fn assert_apart(rust: &str, language: &str, members: &[String], same: impl Fn(&str, &str) -> bool) {
	for (k, member) in members.iter().enumerate() {
		if members[..k].iter().any(|other| same(other, member)) {
			panic!("rust2-ffi named two members of `{rust}` `{member}` in {language}");
		}
	}
}

/// The `n`th name to try in place of `name` where it is taken: `n` more trailing `_`s, after
/// its name and before any `(` of argument labels.
pub(crate) fn underscored(name: &str, n: usize) -> String {
	let at = name.find('(').unwrap_or(name.len());
	format!("{}{}{}", &name[..at], "_".repeat(n), &name[at..])
}

/// A language's names of `params`, each as `spell` spells its Rust name, kept apart from each
/// other and from what `taken` names as [`named`] keeps a scope's, by [`numbered`] steps, since
/// a generated call's own locals end in `_`. So of `new_` and `new`, both `new` in camel case,
/// Rust's `new` keeps `new` and `new_` takes `new2`, in either order.
pub(crate) fn distinct(params: &[Param], spell: impl Fn(&str) -> String, taken: impl Fn(&str) -> bool) -> Vec<String> {
	let rust: Vec<&str> = params.iter().map(|param| param.name.as_str()).collect();
	let spelled = rust.iter().map(|name| spell(name)).collect();
	let own = |rust: &str, name: &str| name == rust || snake(name) == rust;
	named(&rust, spelled, own, |_, name| taken(name), numbered)
}

/// The `n`th name to try in place of `name` where it is taken: it with the number `n + 1`,
/// `foo2`, for a name that never ends in `_`.
pub(crate) fn numbered(name: &str, n: usize) -> String {
	format!("{name}{}", n + 1)
}

/// A name that C or C++, which includes the header, would read as a keyword or macro or
/// reserve for itself, as [`escaped`] moves it: a keyword or a generated parameter's name
/// such as `out` takes `_`, and one already ending in `_` moves on to `_2`, so `new` and
/// `new_` are `new_` and `new_2`.
pub(crate) fn escape(name: &str) -> String {
	escaped(name, keyword)
}

/// Whether C or C++ reads `name` as a keyword, or C names a generated parameter so.
fn keyword(name: &str) -> bool {
	const KEYWORDS: &[&str] = &[
		"auto",
		"bool",
		"break",
		"case",
		"char",
		"const",
		"continue",
		"default",
		"do",
		"double",
		"else",
		"enum",
		"extern",
		"false",
		"float",
		"for",
		"goto",
		"if",
		"inline",
		"int",
		"long",
		"register",
		"restrict",
		"return",
		"short",
		"signed",
		"sizeof",
		"static",
		"struct",
		"switch",
		"true",
		"typedef",
		"union",
		"unsigned",
		"void",
		"volatile",
		"while",
		"out",
		"err",
		"future",
		"waker",
		"data",
		"alignas",
		"alignof",
		"and",
		"and_eq",
		"asm",
		"bitand",
		"bitor",
		"catch",
		"char8_t",
		"char16_t",
		"char32_t",
		"class",
		"compl",
		"concept",
		"consteval",
		"constexpr",
		"constinit",
		"const_cast",
		"co_await",
		"co_return",
		"co_yield",
		"decltype",
		"delete",
		"dynamic_cast",
		"explicit",
		"export",
		"friend",
		"mutable",
		"namespace",
		"new",
		"noexcept",
		"not",
		"not_eq",
		"nullptr",
		"operator",
		"or",
		"or_eq",
		"private",
		"protected",
		"public",
		"reinterpret_cast",
		"requires",
		"static_assert",
		"static_cast",
		"template",
		"this",
		"thread_local",
		"throw",
		"try",
		"typeid",
		"typename",
		"using",
		"virtual",
		"wchar_t",
		"xor",
		"xor_eq",
		// A macro 32-bit x86's GNU dialects predefine, which `macros.txt`, of 64-bit hosts,
		// lacks.
		"i386",
	];
	// `out0`, `out1`, ... are a returned tuple's.
	let out = name
		.strip_prefix("out")
		.is_some_and(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()));
	out || KEYWORDS.contains(&name)
}

/// `name` as C and C++ leave it to programs, past what `keyword` names and every macro a
/// header defines: one C++ reserves, holding `__` or starting with `_` and a capital, first
/// [`unreserved`], and one starting with a digit with `_` prepended. Then, a name ends in one
/// `_` at most, and the names of its stem that no macro holds, nor the stem itself where
/// `keyword` names it, keep their order, as [`stepped`] counts them: `unix` and `unix_` are
/// `unix_` and `unix_2`, and a keyword `new` and `new_` are `new_` and `new_2`. Names apart
/// in Rust stay apart unless one is reserved or ends in `_` and a number, such as `x__`
/// beside `x_2`, which a scope's check refuses.
pub(crate) fn escaped(name: &str, keyword: impl Fn(&str) -> bool) -> String {
	let name = unreserved(name);
	if name.starts_with(|c: char| c.is_ascii_digit()) {
		return format!("_{name}");
	}
	let stem = name.strip_suffix('_').unwrap_or(&name);
	let place = name.len() - stem.len();
	let keyword = keyword(stem);
	(0..)
		.map(|n| stepped(stem, n))
		.filter(|name| !(defined(name) || (keyword && name == stem)))
		.nth(place)
		.expect("a free name")
}

/// `name` as C and C++ leave it to programs: one C++ reserves for the implementation,
/// holding `__` or starting with `_` and a capital ([lex.name]), drops its leading `_`s and
/// keeps one `_` of each run, so `__all__` is `all_`: appending keeps a reserved name
/// reserved. Rust's `__` alone, which a parameter may take, is `_`.
pub(crate) fn unreserved(name: &str) -> String {
	if !(name.contains("__") || reserved(name)) {
		return name.to_string();
	}
	match single(name.trim_start_matches('_')) {
		name if name.is_empty() => "_".to_string(),
		name => name,
	}
}

/// The first segment of every C name the library `lib` exports: its name without leading or
/// trailing `_`s, keeping one `_` of each run, so a join keeps it whole and nothing it starts
/// is reserved, as C reserves every global name starting with `_`. `None` for a name that
/// then starts with no letter, such as `__` or `_1`.
pub(crate) fn prefix(lib: &str) -> Option<String> {
	let prefix = single(lib.trim_matches('_'));
	prefix.starts_with(|c: char| c.is_ascii_alphabetic()).then_some(prefix)
}

/// `name` keeping one `_` of each run.
fn single(name: &str) -> String {
	let mut out = String::with_capacity(name.len());
	for c in name.chars() {
		if !(c == '_' && out.ends_with('_')) {
			out.push(c);
		}
	}
	out
}

/// The C name made of `name` and `part`: them joined by `_`, keeping one `_` of each run,
/// since C++ reserves every name holding `__`. So a name ending in `_`, as Rust's `erase_`
/// or a clash's step does, makes the names its stem would: `api_Status_`'s `eq` is
/// `api_Status_eq`, which a clash then steps past.
pub(crate) fn join(name: &str, part: &str) -> String {
	single(&format!("{name}_{part}"))
}

/// A name C makes for a parameter of `function`, `name`, [`stepped`] until neither a
/// parameter of it in Rust escapes to it nor `taken` holds it.
fn beside(function: &Function, taken: &[String], name: String) -> String {
	let mut beside = name.clone();
	let mut n = 0;
	while function.params.iter().any(|p| escape(&p.name) == beside) || taken.contains(&beside) {
		n += 1;
		beside = stepped(&name, n);
	}
	beside
}

/// What the C names made of `name` start with, before a `_` and the part: `name` without a
/// trailing `_`, so `format!("{}_eq", stem(name))` is [`join`]`(name, "eq")`.
pub(crate) fn stem(name: &str) -> &str {
	name.strip_suffix('_').unwrap_or(name)
}

/// The `n`th name to try in place of `name` where it is taken. The names of a stem `x` count
/// along `x`, `x_`, `x_2`, `x_3`, ..., from `name`'s place, so `delete` steps to `delete_`
/// and `delete_` to `delete_2`: never `__`, which would make a name C++ reserves.
pub(crate) fn stepped(name: &str, n: usize) -> String {
	match (name.strip_suffix('_'), n) {
		(_, 0) => name.to_string(),
		(Some(stem), n) => format!("{stem}_{}", n + 1),
		(None, 1) => format!("{name}_"),
		(None, n) => format!("{name}_{n}"),
	}
}

/// Whether a C or C++ header may define `name` as a macro, one `macros.sh` found, which would
/// expand where the header names it.
pub(crate) fn defined(name: &str) -> bool {
	static MACROS: std::sync::LazyLock<Vec<&str>> = std::sync::LazyLock::new(|| {
		(include_str!("macros.txt").lines())
			.filter(|line| !line.starts_with('#'))
			.collect()
	});
	MACROS.binary_search(&name).is_ok()
}

/// Whether C and C++ reserve a name for the implementation, as `_Bool` and `__cplusplus`:
/// one starting with `__` or `_` and an uppercase letter.
pub(crate) fn reserved(name: &str) -> bool {
	let mut chars = name.chars();
	chars.next() == Some('_') && chars.next().is_some_and(|c| c == '_' || c.is_ascii_uppercase())
}

/// The keywords a Rust name takes raw, `r#type`.
pub(crate) const RUST_KEYWORDS: &[&str] = &[
	"abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "do", "dyn", "else", "enum",
	"extern", "false", "final", "fn", "for", "gen", "if", "impl", "in", "let", "loop", "macro", "match", "mod", "move",
	"mut", "override", "priv", "pub", "ref", "return", "static", "struct", "trait", "true", "try", "type", "typeof",
	"unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

/// A callback method's member of its `<callback>_vtable`, which C and each binding's Rust
/// glue name alike: its [`escape`]d name, a Rust keyword and `free`, the table's own,
/// escaped as C's keywords are.
pub(crate) fn member(method: &Function) -> String {
	member_named(method.name())
}

/// The member of a callback's table for a method named `name`: see [`member`].
fn member_named(name: &str) -> String {
	escaped(name, |stem| {
		stem == "free" || RUST_KEYWORDS.contains(&stem) || keyword(stem)
	})
}

/// What a callback parameter needs, by the form it takes.
pub(crate) fn dyn_wrap(form: Dyn, expr: &str) -> String {
	match form {
		Dyn::Impl => expr.to_string(),
		Dyn::Box => format!("::std::boxed::Box::new({expr})"),
		Dyn::Arc => format!("::std::sync::Arc::new({expr})"),
	}
}

#[cfg(test)]
mod tests {
	use super::{apart, defined, distinct, escape, named, named_apart, past_cycles, shortest, underscored, unique};
	use crate::ir::{Function, Int, Type};

	/// Whether C or C++ reserves `name` in any scope: one holding `__`, or starting with `_`
	/// and a capital.
	fn reserved(name: &str) -> bool {
		name.contains("__") || super::reserved(name)
	}

	/// Each macro's stem, and it with one `_`, escape apart, to neither a macro nor a name C
	/// or C++ reserves. More `_`s make a reserved name, which unreserves onto one of these two.
	#[test]
	fn escapes_keep_apart_from_every_macro() {
		let macros = include_str!("macros.txt").lines().filter(|line| !line.starts_with('#'));
		for stem in macros.map(|m| m.trim_end_matches('_')) {
			let escaped = [escape(stem), escape(&format!("{stem}_"))];
			assert_ne!(escaped[0], escaped[1], "{stem}'s names escape alike");
			for e in &escaped {
				assert!(!defined(e) && !reserved(e), "{stem} escapes to {e}");
			}
		}
	}

	#[test]
	fn escapes_step_past_keywords_and_macros() {
		// A keyword, or a name the ABI's own parameters take, takes `_`, past which the name
		// with `_` steps.
		assert_eq!(escape("new"), "new_");
		assert_eq!(escape("new_"), "new_2");
		assert_eq!(escape("out"), "out_");
		assert_eq!(escape("out0"), "out0_");
		assert_eq!(escape("out0_"), "out0_2");
		// C defines `bool` as a macro too, which takes one `_` all the same.
		assert_eq!(escape("bool"), "bool_");
		assert_eq!(escape("bool_"), "bool_2");
		// Any other name keeps its own, a macro's moving to the next of its stem's names.
		assert_eq!(escape("value_"), "value_");
		assert_eq!(escape("unix"), "unix_");
		assert_eq!(escape("unix_"), "unix_2");
		assert_eq!(escape("errno"), "errno_");
		// Past `ABDAY_2`, ..., `ABDAY_7`, which `<langinfo.h>` defines.
		assert_eq!(escape("ABDAY_1"), "ABDAY_1_");
		assert_eq!(escape("ABDAY_1_"), "ABDAY_1_2");
		// A table's member takes Rust's keywords and `free` as keywords too.
		let method = super::member_named;
		assert_eq!(method("free"), "free_");
		assert_eq!(method("free_"), "free_2");
		assert_eq!(method("type"), "type_");
		assert_eq!(method("type_"), "type_2");
		assert_eq!(method("delete"), "delete_");
	}

	#[test]
	fn reserved_names_escape_unreserved() {
		// serde's derived `Deserialize::deserialize` parameter.
		assert_eq!(escape("__deserializer"), "deserializer");
		assert_eq!(escape("_Bool"), "Bool");
		assert_eq!(escape("__cplusplus"), "cplusplus");
		assert_eq!(escape("__cplusplus_"), "cplusplus_");
		assert_eq!(escape("a__b"), "a_b");
		assert_eq!(escape("__"), "_");
		assert_eq!(escape("__1"), "_1");
		// Unreserved, it escapes as any other name, keywords and macros included.
		assert_eq!(escape("__new"), "new_");
		assert_eq!(escape("EOF__"), "EOF_2");
		assert_eq!(escape("_STRING_H"), "STRING_H");
		// C++ reserves a lowercase `_x` only in the global namespace, which holds no field or
		// parameter, and C only at file scope.
		assert_eq!(escape("_bool"), "_bool");
		assert_eq!(escape("_1"), "_1");
	}

	/// Every identifier of up to five of `_`, `a`, `A`, `1`, and the keywords' and macros' with
	/// trailing `_`s, escapes to a name neither C nor C++ reserves or defines; and two escape
	/// alike only where one is reserved or ends in `_` and a number, which a scope refuses.
	#[test]
	fn no_escape_is_reserved() {
		let mut rust: Vec<String> = vec![String::new()];
		let mut all = Vec::new();
		for _ in 0..5 {
			rust = (rust.iter())
				.flat_map(|name| "_aA1".chars().map(move |c| format!("{name}{c}")))
				.collect();
			all.extend(rust.iter().filter(|n| !n.starts_with('1') && *n != "_").cloned());
		}
		for stem in ["new", "out", "out0", "data", "bool", "unix", "errno", "EOF", "ABDAY_1"] {
			all.extend((0..3).map(|n| format!("{stem}{}", "_".repeat(n))));
		}
		let mut by: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
		for name in &all {
			let e = escape(name);
			assert!(!reserved(&e) && !defined(&e), "{name} escapes to {e}");
			let numbered = |name: &str| name.rsplit_once('_').is_some_and(|(_, n)| n.parse::<u32>().is_ok());
			let plain = |name: &str| !reserved(name) && !numbered(name);
			if let Some(other) = by.insert(e.clone(), name.clone()) {
				assert!(!plain(name) || !plain(&other), "{other} and {name} both escape to {e}");
			}
		}
	}

	/// The C names of `functions`, each free and named as its string, with `params` given
	/// to the first.
	fn c_names(functions: &[&str], params: &[&str]) -> (Vec<String>, Vec<super::Clash>) {
		let mut functions: Vec<Function> = functions.iter().map(|name| Function::bare(name)).collect();
		functions[0].params = (params.iter())
			.map(|name| crate::ir::Param {
				name: name.to_string(),
				ty: Type::Int(Int::U8),
				interned: false,
			})
			.collect();
		let api = crate::ir::Api::of(functions);
		let abi = super::Abi::new(&api, "ns");
		let names = (api.functions.iter()).map(|f| abi.export(f, None).call.name).collect();
		(names, abi.clashes)
	}

	/// A name a join respells steps past those Rust spells so, and past each name a step
	/// already took, never making `__`.
	#[test]
	fn a_respelled_name_steps_past_rusts_own() {
		let (names, clashes) = c_names(&["__x", "x", "x_", "_x_", "a__b"], &[]);
		assert_eq!(names, ["ns_x_2", "ns_x", "ns_x_", "ns_x_3", "ns_a_b"]);
		assert!(clashes.is_empty(), "{clashes:?}");
	}

	/// Two names of one scope that escape alike are refused, which C would otherwise read as
	/// one parameter declared twice.
	#[test]
	fn a_scope_refuses_names_that_escape_alike() {
		let (_, clashes) = c_names(&["pick"], &["_A", "A"]);
		let refused: Vec<_> = clashes
			.iter()
			.map(|(name, (first, _), (second, _))| (name.as_str(), first.as_str(), second.as_str()))
			.collect();
		assert_eq!(
			refused,
			[("A", "parameter `_A` of fn `ns::pick`", "parameter `A` of fn `ns::pick`")]
		);
		// A keyword and it with `_` escape apart.
		let (_, clashes) = c_names(&["pick"], &["new", "new_", "out"]);
		assert!(clashes.is_empty(), "{clashes:?}");
	}

	#[test]
	fn joins_keep_one_underscore_of_each_run() {
		assert_eq!(super::join("api_Status_", "eq"), "api_Status_eq");
		assert_eq!(super::join("api_Status", "eq"), "api_Status_eq");
		assert_eq!(super::join("api", "__all__"), "api_all_");
		assert_eq!(super::join("by_", "data"), "by_data");
		assert_eq!(super::stem("api_Status_"), "api_Status");
		assert_eq!(
			format!("{}_eq", super::stem("api_Status_")),
			super::join("api_Status_", "eq")
		);
	}

	#[test]
	fn a_prefix_keeps_whole_and_unreserved() {
		for (lib, prefix) in [
			("moq", "moq"),
			("a__b", "a_b"),
			("api__", "api"),
			("_Api", "Api"),
			("_x_1", "x_1"),
		] {
			assert_eq!(super::prefix(lib).as_deref(), Some(prefix), "{lib}");
		}
		for lib in ["_", "__", "_1a"] {
			assert_eq!(super::prefix(lib), None, "{lib}");
		}
		// Every name made of a library `api__`'s items starts with its prefix and `_`.
		let api = crate::ir::Api::of(vec![Function::bare("f")]);
		let abi = super::Abi::new(&api, "api__");
		assert_eq!(abi.prefix, "api");
		assert_eq!(abi.export(&api.functions[0], None).call.name, "api_f");
	}

	#[test]
	fn past_cycles_marks_what_a_cycle_reaches() {
		// 0 lends itself and 1; 2 lends 3 and 3 lends 2, which lends 4; 5 lends 6.
		let edges = [vec![0, 1], vec![], vec![3, 4], vec![2], vec![], vec![6], vec![]];
		assert_eq!(past_cycles(&edges), [true, true, true, true, true, false, false]);
	}

	/// Each item's [`apart`] name, by its path's segments.
	fn names(items: &[&[&str]]) -> Vec<String> {
		let items: Vec<Vec<String>> = (items.iter())
			.map(|segments| segments.iter().map(|s| s.to_string()).collect())
			.collect();
		apart(items.iter().map(Vec::as_slice))
	}

	#[test]
	fn splits_only_names_that_join_alike() {
		// The split comes after the segments the clashing names share, so one whose rest is
		// a single segment keeps its name.
		assert_eq!(
			names(&[
				&["tokio", "ConnectError"],
				&["tokio", "connect", "Error"],
				&["tokio", "Client"]
			]),
			["TokioConnectError", "TokioConnect_Error", "TokioClient"]
		);
		// Both split where neither's rest is a single segment.
		assert_eq!(names(&[&["a", "b_c", "Dx"], &["a", "b", "CDx"]]), ["ABC_Dx", "AB_CDx"]);
		// Segments apart only by case still match, for the claim to refuse.
		assert_eq!(
			names(&[&["connect", "X"], &["Connect", "X"]]),
			["Connect_X", "Connect_X"]
		);
	}

	#[test]
	fn unique_keeps_names_their_own() {
		let names = |names: &[&str], taken: &[&str]| {
			unique(names.iter().map(|n| n.to_string()).collect(), |n| taken.contains(&n))
		};
		// A later alike one, or one taken, takes a `_`, past any that is another's own.
		assert_eq!(names(&["A", "A", "A_"], &[]), ["A", "A__", "A_"]);
		assert_eq!(names(&["String", "B"], &["String"]), ["String_", "B"]);
	}

	#[test]
	fn named_keeps_names_apart_in_either_order() {
		// Camel case, which drops a trailing `_`: a name reads back as its Rust if it is that
		// Rust's own snake case.
		let names = |rust: &[&str], taken: &[&str]| {
			let spelled = rust.iter().map(|name| name.trim_end_matches('_').to_string()).collect();
			let own = |rust: &str, name: &str| name == rust;
			named(rust, spelled, own, |_, name| taken.contains(&name), underscored)
		};
		// The name spelled as Rust's keeps it, and the other recovers Rust's `_`, in either order.
		assert_eq!(names(&["new_", "new"], &[]), ["new_", "new"]);
		assert_eq!(names(&["new", "new_"], &[]), ["new", "new_"]);
		// Those that move go in Rust's name order, whatever the scope's.
		assert_eq!(names(&["a__", "a", "a_"], &[]), ["a__", "a", "a_"]);
		// Where none is spelled as Rust's, the first in Rust's order takes the spelling.
		assert_eq!(names(&["a__", "a_"], &[]), ["a_", "a"]);
		// A name taken moves even where it is Rust's own.
		assert_eq!(names(&["a_", "a"], &["a"]), ["a__", "a_"]);
		// Two that read back alike as Rust's own both move, in the scope's order where Rust's
		// names are alike too.
		let spelled = vec!["x".to_string(), "x".to_string()];
		assert_eq!(
			named(&["x", "x"], spelled, |_, _| true, |_, _| false, underscored),
			["x", "x_"]
		);
	}

	#[test]
	fn named_apart_keeps_a_name_where_nothing_clashes() {
		// Each item's space, as Dart's constructor and instance method are: 0 and 1 apart, 2 in both.
		let names = |rust: &[&str], spaces: &[u8]| {
			let spelled = rust.iter().map(|name| name.trim_end_matches('_').to_string()).collect();
			let clash = |k: usize, a: &str, j: usize, b: &str| {
				a == b && (spaces[k] == spaces[j] || spaces[k] == 2 || spaces[j] == 2)
			};
			named_apart(
				rust,
				spelled,
				|rust, name| name == rust,
				|_, _| false,
				underscored,
				clash,
			)
		};
		// Apart, both keep `open`, in either order.
		assert_eq!(names(&["open_", "open"], &[1, 0]), ["open", "open"]);
		assert_eq!(names(&["open", "open_"], &[0, 1]), ["open", "open"]);
		// One in both spaces clashes with each, and Rust's own keeps the name.
		assert_eq!(names(&["open_", "open"], &[2, 0]), ["open_", "open"]);
		assert_eq!(names(&["open", "open_"], &[1, 2]), ["open", "open_"]);
		// One whose spelling only it finds taken, as a Swift function's is where an enum case
		// holds one of its overloads, moves, and leaves the spelling to the other that reads
		// back as its Rust, not to `FOO_BAR`, which comes first in Rust's order.
		let spelled = vec!["fooBar".to_string(); 3];
		let clash = |_: usize, a: &str, _: usize, b: &str| a == b;
		let taken = |k: usize, name: &str| k == 0 && name == "fooBar";
		let own = |rust: &str, _: &str| rust != "FOO_BAR";
		let rust = ["foo_bar", "fooBar", "FOO_BAR"];
		let names = named_apart(&rust, spelled, own, taken, underscored, clash);
		assert_eq!(names, ["fooBar__", "fooBar", "fooBar_"]);
	}

	#[test]
	fn distinct_params_number_the_one_not_spelled_as_rust() {
		let params = |names: &[&str]| -> Vec<crate::ir::Param> {
			(names.iter())
				.map(|name| crate::ir::Param {
					name: name.to_string(),
					ty: crate::ir::Type::Bool,
					interned: false,
				})
				.collect()
		};
		let camel = |name: &str| name.trim_end_matches('_').to_string();
		let names = |names: &[&str]| distinct(&params(names), camel, |_| false);
		// Rust's `new` keeps `new` in either order, and a number never ends a name in `_`.
		assert_eq!(names(&["new_", "new"]), ["new2", "new"]);
		assert_eq!(names(&["new", "new_"]), ["new", "new2"]);
		// A step skips a name another keeps, and one taken.
		assert_eq!(names(&["a_", "a", "a2"]), ["a3", "a", "a2"]);
		let receiving = distinct(&params(&["self_"]), camel, |name| name == "self");
		assert_eq!(receiving, ["self2"]);
	}

	#[test]
	fn underscored_steps_before_labels() {
		assert_eq!(underscored("delete", 1), "delete_");
		assert_eq!(underscored("delete(_:_:)", 2), "delete__(_:_:)");
	}

	/// Each instance's arguments, each by its path's segments, whose forms are their
	/// suffixes, shortest first.
	fn group(instances: &[&[&[&str]]]) -> Vec<Vec<Vec<String>>> {
		(instances.iter())
			.map(|args| {
				(args.iter())
					.map(|segments| {
						(1..=segments.len())
							.map(|len| segments[segments.len() - len..].join("_"))
							.collect()
					})
					.collect()
			})
			.collect()
	}

	#[test]
	fn lengthens_only_where_instances_clash() {
		// Only the clashing arguments lengthen, a segment a round, until they differ.
		let names = shortest(&group(&[
			&[&["a", "x", "foo"], &["u8"]],
			&[&["b", "x", "foo"], &["u8"]],
			&[&["foo"], &["u16"]],
			&[&["c", "bar"], &["u8"]],
		]));
		assert_eq!(names, ["a_x_foo_u8", "b_x_foo_u8", "foo_u16", "bar_u8"]);
		// One with no more segments keeps its name while the other lengthens.
		assert_eq!(
			shortest(&group(&[&[&["foo"]], &[&["polar", "foo"]]])),
			["foo", "polar_foo"]
		);
	}

	#[test]
	fn names_apart_from_order() {
		let instances: [&[&[&str]]; 3] = [&[&["x", "foo"]], &[&["y", "foo"]], &[&["bar"]]];
		let forward = shortest(&group(&instances));
		let mut reversed = shortest(&group(&[instances[2], instances[1], instances[0]]));
		reversed.reverse();
		assert_eq!(forward, ["x_foo", "y_foo", "bar"]);
		assert_eq!(forward, reversed);
	}
}
