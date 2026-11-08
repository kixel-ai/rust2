//! What typing a generic type's definition named: the one account its instances, its drop
//! glue, and the check of its instances' `Option`s and `Poll`s read.
//!
//! A projection from a generic type's parameters that typing its fields names, such as
//! smallvec's `<A as Array>::Item`, becomes a TypeScript type parameter of its own, which
//! each instance gives what the projection is there. An instance without type parameters
//! that a projection argument holds is named by an alias, typed once. A type typed with itself, directly or
//! through others, names instances of a definition that is still being typed, whose
//! projections are not all known yet: those instances' projection arguments are filled in
//! once every definition typed with them is.

use rustc_hir::def_id::DefId;
use rustc_middle::ty::{self, GenericArgsRef, Ty, TypeVisitableExt, TypingEnv};
use rustc_span::Span;

use super::{Adt, Translator};
use crate::program::{SymKind, Symbol};
use crate::ts::{Item, SymId, Type};

/// The account of one generic type's definition.
#[derive(Default)]
pub struct Defined<'tcx> {
	/// Each projection from its type parameters its fields' types name, by the name of the
	/// TypeScript type parameter it is, after its own.
	pub projections: Vec<(Ty<'tcx>, String)>,
	/// Each part its fields' types type that its instances decide, and those of each
	/// instance in [`Self::held`], at that instance's arguments.
	pub checks: Vec<Check<'tcx>>,
	/// What each of those checks: its type, what it decides, and its part and what that
	/// holds, at its arguments.
	checked: rustc_data_structures::fx::FxHashSet<(DefId, Decides, Ty<'tcx>, Ty<'tcx>, Ty<'tcx>)>,
	/// Each instance of the crate's generic types over its type parameters its fields' types
	/// type.
	held: Vec<Ty<'tcx>>,
	/// While it is typed, the shallowest definition still being typed that it was typed
	/// with, by its depth in [`Defining::typing`]; `None` once it is complete.
	open: Option<usize>,
}

/// A part of a generic type's definition, over a type parameter or a projection from one,
/// whose representation its instances decide.
#[derive(Clone)]
pub struct Decided<'tcx> {
	/// The field whose type holds it.
	pub field: DefId,
	/// It, over the definition's own parameters.
	pub part: Ty<'tcx>,
	/// What an `Option` or a `Poll` holds, an `Option`-like `ControlFlow` continues with, any
	/// other `ControlFlow` breaks with, or a future outputs.
	pub inner: Ty<'tcx>,
	/// TypeScript's type of `inner` in the definition.
	pub defined: Type,
	/// What its instances decide of it.
	pub decides: Decides,
}

/// What an instance decides of a [`Decided`] part.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Decides {
	/// An `Option`'s `None`, or that of a `ControlFlow` typed as one.
	Option,
	/// A `Poll`'s `Pending`.
	Poll,
	/// Whether a `ControlFlow` typed as an `rs.Result` of its break stays one, which an
	/// `Option<!>` or `Result<!, _>` break makes otherwise.
	Flow,
	/// Whether a future's output can be `rs.Pending`, as the future's own `Pending` is.
	Future,
}

/// A [`Decided`] part of the definition of the crate's generic `adt`, at `args` over the
/// parameters of the definition that checks it: its own at its own parameters, or an
/// instance's it holds.
#[derive(Clone)]
pub struct Check<'tcx> {
	pub adt: DefId,
	pub decided: Decided<'tcx>,
	pub args: GenericArgsRef<'tcx>,
}

/// `args`, over a definition's parameters, at that definition's instance `at`.
pub fn instantiate_args<'tcx>(
	tcx: ty::TyCtxt<'tcx>,
	env: TypingEnv<'tcx>,
	args: GenericArgsRef<'tcx>,
	at: GenericArgsRef<'tcx>,
) -> GenericArgsRef<'tcx> {
	let bound = ty::EarlyBinder::bind(tcx, args);
	tcx.try_instantiate_and_normalize_erasing_regions(at, env, bound)
		.unwrap_or_else(|_| bound.instantiate(tcx, at).skip_norm_wip())
}

/// The state of typing definitions.
#[derive(Default)]
pub struct Defining<'tcx> {
	records: rustc_data_structures::fx::FxHashMap<DefId, Defined<'tcx>>,
	/// The definitions being typed, innermost last.
	typing: Vec<Typing>,
	/// Definitions typed, waiting on one shallower in [`Self::typing`] they were typed with.
	members: Vec<Member>,
	/// Instances of definitions still being typed, by the [`Type::Later`] each typed.
	pending: Vec<Pending<'tcx>>,
	/// How many projections and decided parts every record holds, which grows while
	/// instances are filled.
	recorded: usize,
	/// The field of the definition being typed whose type is.
	pub field: Option<DefId>,
	/// The instances being typed in the type expression being typed, which a definition
	/// starts afresh.
	pub(super) instancing: Vec<Ty<'tcx>>,
	/// The TypeScript type of each instance typed in that type expression, which one span
	/// refuses alike wherever in it, so a deep one is typed once.
	pub(super) instanced: rustc_data_structures::fx::FxHashMap<Ty<'tcx>, Type>,
	/// Whether that type expression nested past the crate's recursion limit, which is
	/// refused, so nothing else in it is typed.
	pub(super) too_deep: bool,
	/// The instances whose drop glue is being classified, by what it drops through.
	pub(super) glue_open: Vec<Ty<'tcx>>,
	/// The shallowest of [`Self::glue_open`] the classification under way rested on.
	pub(super) glue_rests: usize,
	/// Each classification of an instance's glue, once it rests on none still under way.
	pub(super) glue_known: rustc_data_structures::fx::FxHashMap<(Ty<'tcx>, super::drop::Asked), bool>,
	/// Each that holds if the one under way it rested on, by its depth there, does.
	pub(super) glue_resting: rustc_data_structures::fx::FxHashMap<(Ty<'tcx>, super::drop::Asked), usize>,
	/// Those, in the order they were classified.
	pub(super) glue_order: Vec<(Ty<'tcx>, super::drop::Asked)>,
	/// How deep in projection arguments the type being typed is.
	pub(super) projecting: usize,
	/// The alias naming each instance without type parameters a projection argument held.
	aliases: rustc_data_structures::fx::FxHashMap<Ty<'tcx>, SymId>,
	/// The aliases of enum instances, by the enum, whose union each spells out at its
	/// instance, as TypeScript refuses an alias that names itself through a generic alias.
	enum_aliases: Vec<(SymId, SymId)>,
}

struct Typing {
	adt: DefId,
	members: usize,
	pending: usize,
}

/// A definition typed, and the type of each of its fields there, which may hold
/// [`Type::Later`]s until its cycle completes.
struct Member {
	adt: DefId,
	item: usize,
	fields: Vec<(DefId, Type)>,
	span: Span,
}

struct Pending<'tcx> {
	adt: DefId,
	args: GenericArgsRef<'tcx>,
	/// The definition typing it, and the field whose type did.
	at: Option<(DefId, TypingEnv<'tcx>)>,
	field: Option<DefId>,
	span: Span,
	/// Its projection arguments as last filled in.
	filled: Option<Vec<Type>>,
}

impl<'tcx> Translator<'_, 'tcx> {
	/// The record of the crate's generic `adt`, defining it if it isn't yet.
	pub fn defined(&mut self, adt: DefId) -> &Defined<'tcx> {
		if !self.defining_state.records.contains_key(&adt) {
			let tcx = self.tcx;
			self.adt(tcx.adt_def(adt), tcx.def_span(adt));
		}
		&self.defining_state.records[&adt]
	}

	/// Run `f` outside the definition being typed and the type expression in it, as for
	/// another definition, or a trait object's, whose types are over its own parameters.
	pub(super) fn outside_definition<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
		let defining = self.defining.take();
		let state = &mut self.defining_state;
		let field = state.field.take();
		let instancing = std::mem::take(&mut state.instancing);
		let instanced = std::mem::take(&mut state.instanced);
		let too_deep = std::mem::take(&mut state.too_deep);
		let projecting = std::mem::take(&mut state.projecting);
		let r = f(self);
		self.defining = defining;
		let state = &mut self.defining_state;
		(state.field, state.instancing, state.instanced) = (field, instancing, instanced);
		(state.too_deep, state.projecting) = (too_deep, projecting);
		r
	}

	/// Start typing the definition of `adt`.
	pub(super) fn start_definition(&mut self, adt: DefId) {
		let state = &mut self.defining_state;
		state.records.insert(
			adt,
			Defined {
				open: Some(state.typing.len()),
				..Default::default()
			},
		);
		state.typing.push(Typing {
			adt,
			members: state.members.len(),
			pending: state.pending.len(),
		});
	}

	/// Finish typing the definition of `adt`, whose item is `item` and whose fields typed as
	/// `fields`: complete it, with every definition typed with it, unless it was typed with
	/// one shallower that is still being typed.
	pub(super) fn finish_definition(&mut self, adt: DefId, item: usize, fields: Vec<(DefId, Type)>, span: Span) {
		let state = &mut self.defining_state;
		let depth = state.typing.len() - 1;
		state.members.push(Member {
			adt,
			item,
			fields,
			span,
		});
		let low = state.records[&adt].open.expect("an open definition");
		if low == depth && self.complete(depth) {
			self.defining_state.typing.pop();
			return;
		}
		let state = &mut self.defining_state;
		let low = state.records[&adt].open.expect("an open definition");
		state.typing.pop();
		let parent = state.typing.len() - 1;
		state.lower_typing(parent, low);
	}

	/// The TypeScript parameter a projection `ty` from type parameters is in the definition
	/// being typed, recording it while that definition is open.
	pub(super) fn projection_param(&mut self, ty: Ty<'tcx>) -> Option<Type> {
		let (adt, _) = self.defining?;
		let ty::Alias(_, alias) = ty.kind() else {
			return None;
		};
		let item = alias.kind.try_to_projection()?;
		if !ty.has_param() {
			return None;
		}
		let tcx = self.tcx;
		let record = self.defining_state.records.get_mut(&adt)?;
		if let Some((_, name)) = record.projections.iter().find(|(p, _)| *p == ty) {
			return Some(Type::Param(name.clone()));
		}
		record.open?;
		// Named for its associated type, numbered past any name it would shadow.
		let taken: Vec<String> = own_params(tcx, adt)
			.chain(record.projections.iter().map(|(_, name)| name.clone()))
			.collect();
		let base = tcx.item_name(item).to_string();
		let name = (1..)
			.map(|n| match n {
				1 => base.clone(),
				n => format!("{base}{n}"),
			})
			.find(|name| !taken.contains(name))
			.unwrap();
		record.projections.push((ty, name.clone()));
		self.defining_state.recorded += 1;
		Some(Type::Param(name))
	}

	/// Note that the definition being typed typed `part`, of `inner`, typed `defined`, for
	/// the check of its instances, if they decide it.
	pub(super) fn note_decided(&mut self, part: Ty<'tcx>, inner: Ty<'tcx>, defined: &Type, decides: Decides) {
		let (Some((adt, _)), Some(field)) = (self.defining, self.defining_state.field) else {
			return;
		};
		let depends = match decides {
			Decides::Flow => inner.has_param(),
			_ => self.erases_to(inner).param,
		};
		if !depends {
			return;
		}
		let identity = ty::GenericArgs::identity_for_item(self.tcx, adt);
		self.add_check(
			adt,
			Check {
				adt,
				decided: Decided {
					field,
					part,
					inner,
					defined: defined.clone(),
					decides,
				},
				args: identity,
			},
		);
	}

	/// Add `check` to the record of the open definition of `adt` unless one checks the same.
	fn add_check(&mut self, adt: DefId, check: Check<'tcx>) {
		let tcx = self.tcx;
		let env = TypingEnv::post_analysis(tcx, adt);
		let d = &check.decided;
		let part = super::copy::instantiate(tcx, env, d.part, check.args);
		let inner = super::copy::instantiate(tcx, env, d.inner, check.args);
		let key = (check.adt, d.decides, d.part, part, inner);
		let Some(record) = self.defining_state.records.get_mut(&adt) else {
			return;
		};
		if record.open.is_some() && record.checked.insert(key) {
			record.checks.push(check);
			self.defining_state.recorded += 1;
		}
	}

	/// Note that the definition being typed typed `instance`, an instance of the crate's
	/// generic type, so it checks the parts that instance's definition decided too. One in a
	/// projection argument is not: an instance of the definition types it at its own
	/// arguments, which checks it there.
	pub(super) fn note_held(&mut self, instance: Ty<'tcx>) {
		let (Some((adt, _)), Some(_)) = (self.defining, self.defining_state.field) else {
			return;
		};
		if !instance.has_param() || self.defining_state.projecting > 0 {
			return;
		}
		let Some(record) = self.defining_state.records.get_mut(&adt) else {
			return;
		};
		if record.open.is_some() && !record.held.contains(&instance) {
			record.held.push(instance);
		}
	}

	/// The projection arguments of an instance of the crate's generic `adt` with `args`, a
	/// definition still being typed, as a [`Type::Later`] filled in once it is complete.
	pub(super) fn later_projections(&mut self, adt: DefId, args: GenericArgsRef<'tcx>, span: Span) -> Option<Type> {
		let state = &mut self.defining_state;
		let low = state.records.get(&adt)?.open?;
		// Whatever types it is typed with it.
		if let Some(top) = state.typing.len().checked_sub(1) {
			state.lower_typing(top, low);
		}
		// Typed by the same definition before, so filling either in can't name it again
		// without end.
		let at = self.defining.map(|(did, _)| did);
		if at.is_some()
			&& let Some(id) =
				(state.pending.iter()).position(|p| p.adt == adt && p.args == args && p.at.map(|(did, _)| did) == at)
		{
			return Some(Type::Later(id));
		}
		state.pending.push(Pending {
			adt,
			args,
			at: self.defining,
			field: state.field,
			span,
			filled: None,
		});
		Some(Type::Later(state.pending.len() - 1))
	}

	/// Whether an instance of the crate's generic `adt` may name projections: it names some,
	/// or its definition is still being typed.
	pub(super) fn may_project(&self, adt: DefId) -> bool {
		(self.defining_state.records.get(&adt)).is_some_and(|r| r.open.is_some() || !r.projections.is_empty())
	}

	/// The alias naming `instance`, an instance of the crate's generic type `info` without
	/// type parameters, which a projection argument holds: `Held$Looped` for `Held<Looped>`,
	/// whose module exports `type Held$Looped = Held<Looped, Held$Looped>`. Defined the first
	/// time, and named while it is, so an instance its own projections hold again names it.
	pub(super) fn instance_alias(&mut self, instance: Ty<'tcx>, info: &Adt, span: Span) -> Type {
		if let Some(&alias) = self.defining_state.aliases.get(&instance) {
			return Type::Sym(alias, Vec::new());
		}
		let tcx = self.tcx;
		let ty::Adt(adt, args) = *instance.kind() else {
			unreachable!("an instance of a generic type")
		};
		let did = adt.did();
		// Defaults included, so no alias is named as its type is.
		let alias = self.program.sym(Symbol {
			module: self.type_module(did),
			base: tcx.item_name(did).to_string(),
			suffix: super::names::args(tcx, args.as_slice()).collect::<Vec<_>>().join("_"),
			owner: None,
			kind: SymKind::Union,
			order: self.order(tcx.def_span(did)),
			span: tcx.def_span(did),
			instance: String::new(),
			name: String::new(),
			alias: None,
		});
		self.defining_state.aliases.insert(instance, alias);
		// Typed as anywhere else, outside the projection argument.
		let projecting = std::mem::take(&mut self.defining_state.projecting);
		let ty = self.instance_ty(instance, info, span);
		self.defining_state.projecting = projecting;
		// Refused past the recursion limit, as each use of it is again.
		if self.defining_state.too_deep {
			self.defining_state.aliases.remove(&instance);
			return Type::Builtin("unknown");
		}
		if adt.is_enum() {
			self.defining_state.enum_aliases.push((alias, info.sym));
		}
		self.program.items.push(Item::Union {
			sym: alias,
			params: Vec::new(),
			ty,
			doc: None,
		});
		Type::Sym(alias, Vec::new())
	}

	/// Spell out each enum instance's alias as the enum's union at the instance, once every
	/// definition is complete.
	pub(super) fn expand_enum_aliases(&mut self) {
		let items: rustc_data_structures::fx::FxHashMap<SymId, usize> = (self.program.items.iter().enumerate())
			.filter_map(|(i, item)| match item {
				Item::Union { sym, .. } => Some((*sym, i)),
				_ => None,
			})
			.collect();
		for (alias, adt) in std::mem::take(&mut self.defining_state.enum_aliases) {
			let Item::Union { params, ty: union, .. } = &self.program.items[items[&adt]] else {
				unreachable!("an enum's union")
			};
			let Item::Union {
				ty: Type::Sym(_, args), ..
			} = &self.program.items[items[&alias]]
			else {
				unreachable!("an alias of an enum instance")
			};
			let expanded = union.subst(&|name| params.iter().position(|p| p == name).map(|i| args[i].clone()));
			let Item::Union { ty, .. } = &mut self.program.items[items[&alias]] else {
				unreachable!("an alias")
			};
			*ty = expanded;
		}
	}

	/// Start typing `instance`, an instance of the crate's generic type, unless it is
	/// typed already further out in the same type expression over type parameters, which no
	/// alias names: a projection of its parameters is then a type whose arguments hold it
	/// again, so its TypeScript type arguments would never end, which is refused.
	pub(super) fn enter_instance(&mut self, instance: Ty<'tcx>, span: Span) -> bool {
		let instancing = &self.defining_state.instancing;
		if self.defining_state.too_deep {
			return false;
		}
		let msg = match () {
			_ if instance.has_param() && instancing.contains(&instance) => format!(
				"`{instance}` is not supported yet: a projection of its parameters is a type whose arguments hold `{instance}` again, so its TypeScript type arguments would never end"
			),
			// Named by the outermost, which the source wrote.
			_ if instancing.len() > self.tcx.recursion_limit().0 => {
				self.defining_state.too_deep = true;
				format!(
					"`{}` is not supported: its projections' values nest instances past the crate's recursion limit",
					instancing[0]
				)
			}
			_ => {
				self.defining_state.instancing.push(instance);
				return true;
			}
		};
		self.err(span, msg);
		false
	}

	/// Finish typing the instance [`Self::enter_instance`] started, as `ty` if typed whole.
	pub(super) fn leave_instance(&mut self, instance: Ty<'tcx>, ty: Option<&Type>) {
		let state = &mut self.defining_state;
		state.instancing.pop();
		match (state.instancing.is_empty(), ty) {
			(true, _) => {
				state.instanced.clear();
				state.too_deep = false;
			}
			(false, Some(ty)) => {
				state.instanced.insert(instance, ty.clone());
			}
			(false, None) => {}
		}
	}

	/// The TypeScript type of `instance` if the type expression being typed typed it already.
	pub(super) fn instanced(&self, instance: Ty<'tcx>) -> Option<Type> {
		self.defining_state.instanced.get(&instance).cloned()
	}

	/// Complete the definition at `depth` in [`Defining::typing`] and every one typed with
	/// it: fill in the projection arguments of the instances they typed, and take in what
	/// each instance they hold decided, until no record grows, then close their records and
	/// settle what reads them. Filling may name an instance of a definition further out
	/// that is still being typed, which then completes them with its own, so this returns
	/// whether they completed.
	///
	/// Each round fills in what the round before named, so a finite layout settles in as
	/// many rounds as its chain of instances and projections is long. A type that holds
	/// itself at ever other arguments never settles, so, as rustc bounds its own recursion,
	/// more rounds than the crate's recursion limit are refused at the definition.
	fn complete(&mut self, depth: usize) -> bool {
		let tcx = self.tcx;
		let (members, pending) = {
			let typing = &self.defining_state.typing[depth];
			(typing.members, typing.pending)
		};
		let mut rounds = 0;
		let settled = loop {
			let recorded = self.defining_state.recorded;
			let count = self.defining_state.pending.len();
			for i in pending..count {
				self.fill_pending(i);
			}
			self.take_held(members);
			if self.defining_state.recorded == recorded && self.defining_state.pending.len() == count {
				break true;
			}
			rounds += 1;
			if rounds > tcx.recursion_limit().0 {
				break false;
			}
		};
		if !settled {
			let state = &mut self.defining_state;
			for p in &mut state.pending[pending..] {
				p.filled = Some(vec![Type::Builtin("unknown"); state.records[&p.adt].projections.len()]);
			}
			// Refused, so nothing of its instances is checked: the checks it took in, of ever
			// deeper instances, would each type one deeper again.
			for member in &state.members[members..] {
				state.records.get_mut(&member.adt).expect("a record").checks.clear();
			}
			// The definition completing them all, which finished typing last.
			let root = state.members.last().expect("the definition completing");
			let instance = Ty::new_adt(
				tcx,
				tcx.adt_def(root.adt),
				ty::GenericArgs::identity_for_item(tcx, root.adt),
			);
			let span = tcx.def_span(root.adt);
			self.err(
				span,
				format!(
					"`{instance}` is not supported: it holds itself at ever other arguments, whose projections name new instances or projections past the crate's recursion limit"
				),
			);
		}
		let root = self.defining_state.typing[depth].adt;
		if self.defining_state.records[&root].open.is_some_and(|low| low < depth) {
			return false;
		}
		let done: Vec<Member> = self.defining_state.members.drain(members..).collect();
		for member in &done {
			let record = self.defining_state.records.get_mut(&member.adt).expect("a record");
			record.open = None;
		}
		// Each instance typed while open now has its projection arguments.
		let filled: Vec<Vec<Type>> = (self.defining_state.pending[pending..].iter())
			.map(|p| p.filled.clone().unwrap_or_default())
			.collect();
		// One of a cycle further out, still being typed, is left for it.
		let later = |id: usize| id.checked_sub(pending).map(|i| filled[i].clone());
		let mut endless = false;
		// Wherever a type typed while they were open is kept: an item, such as an alias or a
		// trait object's type, made then or before, and the methods objects hold.
		if !filled.is_empty() {
			for item in &mut self.program.items {
				let tys: Vec<&mut Type> = match item {
					Item::Struct { fields, .. } => fields.iter_mut().map(|f| &mut f.ty).collect(),
					Item::Union { ty, .. } | Item::Value { ty, .. } => vec![ty],
					Item::Function(_) => continue,
				};
				for ty in tys {
					endless |= ty.fill(&later).is_none();
				}
			}
			endless |= self.fill_objects(&later);
		}
		let mut members_done = Vec::with_capacity(done.len());
		for mut member in done {
			for (_, ty) in &mut member.fields {
				endless |= ty.fill(&later).is_none();
			}
			// Its own parameters, then one per projection.
			let names = (self.defining_state.records[&member.adt].projections.iter()).map(|(_, name)| name.clone());
			let item = &mut self.program.items[member.item];
			if let Item::Struct { params, .. } | Item::Union { params, .. } = item {
				params.extend(names);
			}
			super::unnamed_params(item);
			members_done.push(member);
		}
		// The definition completing them all, which finished typing last.
		if endless && let Some(root) = members_done.last() {
			self.err(
				root.span,
				"a type whose projection is, at its own parameters, a type that holds it again, so its TypeScript type arguments would never end",
			);
		}
		for member in &members_done {
			for (field, ty) in &member.fields {
				self.note_field(*field, ty);
			}
		}
		// The checks of instances typed while their definitions were open.
		let checks: Vec<Pending<'tcx>> = self.defining_state.pending.drain(pending..).collect();
		for check in checks {
			let outer = std::mem::replace(&mut self.defining, check.at);
			let adt = tcx.adt_def(check.adt);
			if let Some(msg) = self.generic_erasure(adt, check.args, check.span) {
				self.err(check.span, msg);
			}
			self.defining = outer;
		}
		for member in members_done {
			let adt = tcx.adt_def(member.adt);
			let info = self.adts[&member.adt].clone();
			self.drop_glue(adt, &info, member.span);
		}
		true
	}

	/// Take into each definition from `members` on in [`Defining::members`] the checks of
	/// each instance it holds, at that instance's arguments, so an instance of the
	/// definition checks them as the instance it holds there would be: `Outer<S>` holding
	/// `Inner<S>` checks `Inner`'s `Option<S::Item>` at `Outer`'s instances.
	fn take_held(&mut self, members: usize) {
		let tcx = self.tcx;
		let adts: Vec<DefId> = self.defining_state.members[members..].iter().map(|m| m.adt).collect();
		for adt in adts {
			let env = TypingEnv::post_analysis(tcx, adt);
			let held = self.defining_state.records[&adt].held.clone();
			for instance in held {
				let ty::Adt(def, args) = *instance.kind() else {
					unreachable!("an instance of a generic type")
				};
				let Some(record) = self.defining_state.records.get(&def.did()) else {
					continue;
				};
				for check in record.checks.clone() {
					let args = instantiate_args(tcx, env, check.args, args);
					self.add_check(adt, Check { args, ..check });
				}
			}
		}
	}

	/// Fill in the projection arguments of the instance `pending[i]` typed, from its
	/// definition's projections so far. Filling them in may name more projections or
	/// instances, which the next round of [`Self::complete`] fills in turn.
	fn fill_pending(&mut self, i: usize) {
		let p = &self.defining_state.pending[i];
		let projections: Vec<Ty<'tcx>> = (self.defining_state.records[&p.adt].projections.iter())
			.map(|(p, _)| *p)
			.collect();
		if p.filled.as_ref().is_some_and(|f| f.len() == projections.len()) {
			return;
		}
		let (args, at, field, span) = (p.args, p.at, p.field, p.span);
		let env = at.map_or(self.typing_env(), |(_, env)| env);
		let outer = std::mem::replace(&mut self.defining, at);
		let outer_field = std::mem::replace(&mut self.defining_state.field, field);
		self.defining_state.projecting += 1;
		let tys: Vec<Type> = projections
			.iter()
			.map(|&p| {
				let at = super::copy::instantiate(self.tcx, env, p, args);
				self.ty(at, span)
			})
			.collect();
		self.defining_state.projecting -= 1;
		self.defining = outer;
		self.defining_state.field = outer_field;
		self.defining_state.pending[i].filled = Some(tys);
	}
}

impl Defining<'_> {
	/// Note that the definition at `depth` in [`Self::typing`] was typed with the one at `low`.
	fn lower_typing(&mut self, depth: usize, low: usize) {
		let adt = self.typing[depth].adt;
		if let Some(open) = &mut self.records.get_mut(&adt).expect("a record").open {
			*open = (*open).min(low);
		}
	}
}

/// The names of `adt`'s own type parameters.
fn own_params(tcx: ty::TyCtxt<'_>, adt: DefId) -> impl Iterator<Item = String> + '_ {
	(tcx.generics_of(adt).own_params.iter())
		.filter(|param| matches!(param.kind, ty::GenericParamDefKind::Type { .. }))
		.map(|param| param.name.to_string())
}
