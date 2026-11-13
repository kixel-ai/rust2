//! Trait objects. A `dyn Trait` is a JS object with a method for each of the trait's
//! dispatchable functions, its supertraits' included, or for std's or a mapped crate's
//! trait, each that a call dispatches to, named as an impl's function is,
//! for its trait (`area$Shape`), and taking the arguments after `self`. That is the
//! trait's type, which a TS caller can implement, generic over the trait's type parameters
//! and the associated types its objects bind (`Source<Item>`), and typed as the trait
//! declares them, which a call casts to what its object's arguments return. A coercion to one wraps the value in a
//! class of its type's own, shared by every trait object of the type, whose methods call
//! that type's impls, a std type's through their mappings, and whose `[Symbol.dispose]`, which a `Box<dyn Trait>` drops it by,
//! drops the value. An `Arc<dyn Trait>` is an `rs.ArcDyn` around the one a box would be. A
//! struct whose last field unsizes to one, such as a `RefCell<dyn Trait>`, is an
//! `rs.tailDyn` view of the struct it was made from, whose last field is that field's
//! object. A raw pointer unsizes as a `&mut` to what it points at does. A
//! `dyn Any`'s one method is `type_id`, a constant of its class's type's `TypeId`, and a
//! downcast, of it or of a `dyn Error`, checks for the class of the type it names. A `dyn Fn`,
//! `dyn FnMut`, or `dyn FnOnce` is the function it was made of, which a call calls as it
//! would the closure itself.

use std::collections::hash_map::Entry;

use rustc_hir::def_id::DefId;
use rustc_middle::ty::{
	self, GenericArgs, Instance, Interner as _, Ty, TyCtxt, TypeFoldable, TypeFolder, TypeSuperFoldable,
	TypeVisitableExt, TypingEnv, Upcast as _,
};
use rustc_span::Span;

use super::body::Body;
use super::library::Std;
use super::{AdtRepr, StructField, Translator};
use crate::program::{SymKind, Symbol};
use crate::ts::{self, Expr, FieldId, FieldInit, Item, Key, Param, Stmt, StmtKind, SymId, Type};

/// A trait that objects of it translate for: its type and the type's parameters, its
/// methods in vtable order, and the values its objects wrap.
pub struct Object<'tcx> {
	sym: SymId,
	/// Its type's parameters, each by its name: the trait's own type parameters, then the
	/// associated types its objects bind that no supertrait binds already.
	params: Vec<(String, Slot)>,
	methods: Vec<Method<'tcx>>,
	/// Where its type is in the program's items.
	item: usize,
	/// The types and holds of the classes its objects wrap values in, with the trait object
	/// each was made for.
	classes: Vec<(Ty<'tcx>, Held, Ty<'tcx>)>,
	/// The trait objects of it whose methods take and return what its type gives them at
	/// their arguments, or don't, which is refused.
	checked: Vec<Ty<'tcx>>,
}

/// A parameter of a trait object's type.
#[derive(Clone, Copy, PartialEq)]
enum Slot {
	/// The trait's type parameter at this index of its generics, past `Self`.
	Arg(usize),
	/// An associated type its objects bind.
	Binding(DefId),
}

#[derive(Clone)]
struct Method<'tcx> {
	def_id: DefId,
	/// The arguments of the trait it is of, over the object's trait's own parameters.
	tr: GenericArgs<'tcx>,
	name: String,
	/// Its type over the object type's parameters, once the object holds it: from the start for the trait of a crate that
	/// translates whole, which a TS caller may call or implement, and else, for std's or a
	/// mapped crate's trait, at the first call that dispatches to it, so a `dyn Buf` holds a
	/// few of `Buf`'s many methods, and a std type's class only the impls it calls.
	ty: Option<Type>,
}

/// The class a coercion wraps a value of one type in, and its field holding the value.
#[derive(Clone, Copy)]
pub struct Class {
	pub sym: SymId,
	pub v: FieldId,
}

/// A `dyn Fn`, `dyn FnMut`, or `dyn FnOnce`: its trait, and the signature it calls by.
pub struct DynFn<'tcx> {
	pub kind: ty::ClosureKind,
	pub inputs: Vec<Ty<'tcx>>,
	pub output: Ty<'tcx>,
}

/// How a wrapper class holds the value a pointer to it was unsized from.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Held {
	/// Its value, from `&T` or `Box<T>`: the class is an `rs.Place` of it, as a `v`.
	Value,
	/// The `rs.Place` a `&mut T` to a value JS copies is.
	Place,
}

impl<'tcx> Translator<'_, 'tcx> {
	/// The function trait and signature of `ty`, if it is a `dyn Fn`, `dyn FnMut`, or
	/// `dyn FnOnce`, whose JS value is the function it was made of.
	pub fn dyn_fn(&self, ty: Ty<'tcx>) -> Option<DynFn<'tcx>> {
		let tcx = self.tcx;
		let ty::Dynamic(preds, ..) = ty.kind() else {
			return None;
		};
		let principal = preds.principal()?;
		let kind = tcx.fn_trait_kind_from_def_id(principal.def_id())?;
		let args = tcx.instantiate_bound_regions_with_erased(principal).args.type_at(0);
		let output = preds
			.projection_bounds()
			.find(|p| Some(p.item_def_id()) == tcx.lang_items().fn_once_output())
			.and_then(|p| tcx.instantiate_bound_regions_with_erased(p).term.as_type())
			.unwrap_or(tcx.types.unit);
		Some(DynFn {
			kind,
			inputs: args.tuple_fields().to_vec(),
			output,
		})
	}

	/// `ty`'s `TypeId`: rustc's hash of it, as 32 hex digits, or `undefined` in a census over
	/// type parameters, where only an instance decides what `ty` is.
	pub fn type_id(&self, ty: Ty<'tcx>) -> Expr {
		match self.polymorphic() && ty.has_param() {
			true => Expr::undefined(),
			false => Expr::Str(format!("{:032x}", self.tcx.type_id_hash(ty).as_u128())),
		}
	}

	/// The function trait that calls the closure, async closure, or `dyn Fn*` that `ty` is,
	/// or boxes.
	pub fn owned_fn(&self, ty: Ty<'tcx>) -> Option<ty::ClosureKind> {
		let ty = self.unboxed(ty);
		match ty.kind() {
			ty::Closure(..) | ty::CoroutineClosure(..) => super::closure_kind(ty),
			ty::Dynamic(..) => self.dyn_fn(ty).map(|f| f.kind),
			_ => None,
		}
	}

	/// What `ty` owns through any boxes and `AssertUnwindSafe`s.
	pub fn unboxed(&self, mut ty: Ty<'tcx>) -> Ty<'tcx> {
		while let ty::Adt(adt, args) = ty.kind()
			&& matches!(Std::of(self.tcx, adt.did()), Some(Std::Box | Std::AssertUnwindSafe))
		{
			ty = args.type_at(0);
		}
		ty
	}

	/// The type of the trait object `ty`, a `dyn Trait`, its trait's made the first time.
	pub fn object(&mut self, ty: Ty<'tcx>, span: Span) -> Option<Type> {
		let ty::Dynamic(preds, ..) = ty.kind() else {
			return None;
		};
		let Some(principal) = preds.principal_def_id() else {
			self.unrepresented(
				span,
				format!("`{ty}`, a trait object of no trait with methods, which is not supported yet"),
			);
			return None;
		};
		if !self.objects.contains_key(&principal) {
			// Over the trait's own parameters, whatever definition is typing it.
			self.outside_definition(|t| t.define_object(principal, ty, span))?;
		}
		let args = self.object_args(principal, ty, span);
		let sym = self.objects[&principal].sym;
		Some(Type::Sym(sym, args.into_iter().map(|(_, t)| t).collect()))
	}

	/// Make the object of `principal`, the trait of the trait object `ty`, unless it is
	/// refused.
	fn define_object(&mut self, principal: DefId, ty: Ty<'tcx>, span: Span) -> Option<()> {
		let tcx = self.tcx;
		let identity = ty::Binder::dummy(ty::TraitRef::identity(tcx, principal));
		let mut params = Vec::new();
		for param in &tcx.generics_of(principal).own_params {
			match param.kind {
				ty::GenericParamDefKind::Type { .. } if param.index > 0 => {
					params.push((param.name.to_string(), Slot::Arg(param.index as usize)));
				}
				ty::GenericParamDefKind::Const { .. } => {
					let msg =
						format!("`{ty}`, a trait object of a trait with a const parameter, which is not supported yet");
					self.unrepresented(span, msg);
					return None;
				}
				_ => {}
			}
		}
		// The associated types every object binds alike, as a supertrait binds them, are no
		// parameters: `Counted` for `trait Counted: Source<Item = u32>`.
		let clause: ty::Clause<'tcx> = identity.upcast(tcx);
		let implied: Vec<DefId> = ty::elaborate::elaborate(tcx, [clause])
			.filter_map(|c| c.as_projection_clause())
			.map(|p| p.skip_binder().def_id())
			.collect();
		let ty::Dynamic(preds, ..) = ty.kind() else {
			unreachable!("a trait object");
		};
		for binding in preds.projection_bounds().map(|p| p.item_def_id()) {
			if !implied.contains(&binding) {
				params.push((tcx.item_name(binding).to_string(), Slot::Binding(binding)));
			}
		}
		if let Some((name, _)) = params
			.iter()
			.enumerate()
			.find_map(|(i, p)| params[..i].iter().find(|q| q.0 == p.0))
		{
			let msg = format!(
				"`{ty}`, whose trait's type parameters and associated types have two named `{name}`, which is not supported yet"
			);
			self.unrepresented(span, msg);
			return None;
		}
		// Each supertrait with the arguments the principal's give it, over the principal's
		// own parameters.
		let refs: Vec<ty::TraitRef<'tcx>> = ty::elaborate::supertraits(tcx, identity)
			.map(|tr| tr.skip_binder())
			.collect();
		let name = tcx.item_name(principal).to_string();
		let sym = self.program.sym(Symbol {
			module: self.type_module(principal),
			base: name,
			suffix: String::new(),
			owner: None,
			kind: SymKind::Union,
			order: self.order(tcx.def_span(principal)),
			span: tcx.def_span(principal),
			instance: String::new(),
			name: String::new(),
			alias: None,
		});
		let mut methods = Vec::new();
		let traits = tcx.supertrait_def_ids(principal);
		for trait_ref in traits.flat_map(|tr| refs.iter().filter(move |r| r.def_id == tr)) {
			let tr = trait_ref.def_id;
			for &def_id in tcx.own_existential_vtable_entries(tr) {
				// Of `Error`, only `source`: `type_id`'s `TypeId` has no JS value, `provide` is
				// unstable, and `description` and `cause` are deprecated.
				if is_error(tcx, tr) && tcx.item_name(def_id).as_str() != "source" {
					continue;
				}
				let name = format!("{}${}", super::camel(tcx.item_name(def_id).as_str()), tcx.item_name(tr));
				if methods.iter().any(|m: &Method<'_>| m.name == name) {
					self.err(span, format!("`{ty}`, whose traits have two methods named `{name}`"));
					return None;
				}
				methods.push(Method {
					def_id,
					tr: trait_ref.args,
					name,
					ty: None,
				});
			}
		}
		let count = methods.len();
		let names = params.iter().map(|(name, _)| name.clone()).collect();
		self.objects.insert(
			principal,
			Object {
				sym,
				params,
				methods,
				item: self.program.items.len(),
				classes: Vec::new(),
				checked: Vec::new(),
			},
		);
		self.program.items.push(Item::Union {
			sym,
			params: names,
			// Any object, as a downcast checks for the class of the value's own type, until it
			// holds `type_id`.
			ty: match is_any(tcx, principal) {
				true => Type::Builtin("object"),
				false => Type::Object(Vec::new()),
			},
			doc: self.doc(principal),
		});
		if self.own(principal) && !super::stock::is_stock(tcx, principal) {
			for index in 0..count {
				self.hold(principal, index, span);
			}
		}
		Some(())
	}

	/// Refuse the trait object `ty` of `principal` if a method its object holds takes or
	/// returns, at its arguments, what its type gives in another form than a cast reaches,
	/// as an `rs.OptionOf<T>` whose `T` is a `MaybeUninit<u32>`, which can be `undefined`
	/// though its type can't. Checked where a
	/// value becomes one or a call dispatches through one, by which time the object holds each method of a crate's own
	/// trait, and not where a type names one, as a method may return an object of other
	/// arguments without end, `Nest<Vec<T>>` of `Nest<T>`.
	fn check_object(&mut self, principal: DefId, ty: Ty<'tcx>, span: Span) {
		let object = &self.objects[&principal];
		if object.params.is_empty() || ty.has_param() || object.checked.contains(&ty) {
			return;
		}
		let held: Vec<(usize, Type)> = (object.methods.iter().enumerate())
			.filter_map(|(i, m)| Some((i, m.ty.clone()?)))
			.collect();
		self.objects
			.get_mut(&principal)
			.expect("a made object")
			.checked
			.push(ty);
		let args = self.object_args(principal, ty, span);
		for (index, defined) in held {
			let called = self.called_ty(principal, index, ty, span);
			if !fits(&defined.subst(&|p| param(&args, p)), &called) {
				let def_id = self.objects[&principal].methods[index].def_id;
				let msg = format!(
					"`{ty}`, whose `{}` takes or returns a value of another form than its trait's object type gives it, which is not supported yet",
					self.tcx.def_path_str(def_id)
				);
				self.err(span, msg);
			}
		}
	}

	/// The arguments the trait object `ty` gives its object's type parameters, by name.
	fn object_args(&mut self, principal: DefId, ty: Ty<'tcx>, span: Span) -> Vec<(String, Type)> {
		let params = self.objects[&principal].params.clone();
		params
			.into_iter()
			.map(|(name, p)| (name, self.ty(param_ty(self.tcx, ty, p), span)))
			.collect()
	}

	/// The type of the method `index` of the object of `principal` as the trait object `ty`
	/// calls it, at its own arguments.
	fn called_ty(&mut self, principal: DefId, index: usize, ty: Ty<'tcx>, span: Span) -> Type {
		let method = &self.objects[&principal].methods[index];
		let def_id = method.def_id;
		let args = instance_args(self.tcx, def_id, method.tr, ty, ty);
		let sig = self.sig(def_id, args);
		self.method_ty(def_id, sig, span)
	}

	/// The type of a trait object's method `def_id` of the signature `sig`.
	fn method_ty(&mut self, def_id: DefId, sig: ty::FnSig<'tcx>, span: Span) -> Type {
		// The methods it holds are declared, so their `()` is `void`.
		let mut params: Vec<Type> = sig.inputs()[1..].iter().map(|t| self.ty(*t, span)).collect();
		if super::caller::tracks_virtual(self, def_id) {
			params.push(super::caller::param_ty());
		}
		Type::Func(params, Box::new(self.ret_ty(sig.output(), span)))
	}

	/// The object of `principal` holding its method `index` from now on: in its type, and in
	/// each class its objects wrap a value in.
	fn hold(&mut self, principal: DefId, index: usize, span: Span) {
		self.outside_definition(|t| t.hold_method(principal, index, span));
	}

	/// [`Self::hold`], outside any definition being typed.
	fn hold_method(&mut self, principal: DefId, index: usize, span: Span) {
		let tcx = self.tcx;
		let object = &self.objects[&principal];
		let method = &object.methods[index];
		if method.ty.is_some() {
			return;
		}
		let def_id = method.def_id;
		// Its signature over the trait's own parameters, each associated type its objects
		// bind the type's parameter of its name.
		let sig = tcx
			.fn_sig(def_id)
			.instantiate(tcx, method_args(tcx, def_id, method.tr))
			.skip_norm_wip();
		let sig = tcx.instantiate_bound_regions_with_erased(sig);
		let env = TypingEnv::post_analysis(tcx, principal);
		let sig = tcx
			.try_normalize_erasing_regions(env, ty::Unnormalized::new_wip(sig))
			.unwrap_or(sig);
		let bound: Vec<(DefId, Ty<'tcx>)> = (object.params.iter().enumerate())
			.filter_map(|(i, (name, p))| match *p {
				Slot::Binding(def_id) => {
					let index = (tcx.generics_of(principal).count() + i) as u32;
					Some((def_id, Ty::new_param(tcx, index, rustc_span::Symbol::intern(name))))
				}
				Slot::Arg(_) => None,
			})
			.collect();
		let sig = sig.fold_with(&mut Bindings { tcx, bound: &bound });
		let func = self.method_ty(def_id, sig, span);
		let object = self.objects.get_mut(&principal).expect("a made object");
		object.methods[index].ty = Some(func);
		let members = object
			.methods
			.iter()
			.filter_map(|m| Some((m.name.clone(), m.ty.clone()?)))
			.collect();
		let classes = object.classes.clone();
		// `dyn Any`'s, which is any object until it holds `type_id`.
		if let Item::Union {
			ty: ty @ (Type::Object(_) | Type::Builtin("object")),
			..
		} = &mut self.program.items[object.item]
		{
			*ty = Type::Object(members);
		}
		for (from, held, object) in classes {
			self.class_method(principal, index, from, held, object, span);
		}
	}

	/// Fill in the [`Type::Later`]s of the methods objects hold, as [`Type::fill`] does,
	/// returning whether one reaches itself.
	pub(super) fn fill_objects(&mut self, later: &impl Fn(usize) -> Option<Vec<Type>>) -> bool {
		let mut endless = false;
		for object in self.objects.values_mut() {
			for ty in object.methods.iter_mut().filter_map(|m| m.ty.as_mut()) {
				endless |= ty.fill(later).is_none();
			}
		}
		endless
	}

	/// Whether an upcast from the trait object `from` to `to`, a made object, is cast to
	/// `to`'s type: a generic trait's, whose methods have a form only its arguments decide
	/// where `from`'s have another.
	pub fn upcast_retypes(&self, from: Ty<'tcx>, to: Ty<'tcx>) -> bool {
		let to = principal(to).expect("an object's trait");
		principal(from) != Some(to) && !self.objects[&to].params.is_empty()
	}

	/// Whether `instance` is a call through a trait object's method, whose wrapper takes
	/// its arguments as the crate's functions do. A `&dyn Debug` or `&dyn Display` is a
	/// function writing its value instead, a `dyn Future` an `rs.Future`, and a
	/// `dyn Iterator` an `rs.Iter`.
	pub fn dispatches(&self, instance: Instance<'tcx>) -> bool {
		let ty::InstanceKind::Virtual(..) = instance.def else {
			return false;
		};
		let object = instance.args.type_at(0);
		let ty::Dynamic(preds, ..) = object.kind() else {
			return false;
		};
		self.dyn_future(object).is_none()
			&& self.dyn_iter(object).is_none()
			&& self.dyn_fn(object).is_none()
			&& preds
				.principal_def_id()
				.is_none_or(|principal| super::fmt::Trait::of(self.tcx, principal).is_none())
	}

	/// The class wrapping a `from` held as `held`, made the first time, with the methods the
	/// object of the trait object `to` holds, and those it holds later. One class serves
	/// every trait object of its type, so a downcast finds it whichever trait's object the
	/// value became.
	pub fn object_class(&mut self, from: Ty<'tcx>, to: Ty<'tcx>, held: Held, span: Span) -> Option<Class> {
		let tcx = self.tcx;
		let from = tcx.erase_and_anonymize_regions(from);
		let to = tcx.erase_and_anonymize_regions(to);
		self.object(to, span)?;
		let principal = principal(to).expect("an object's trait");
		self.check_object(principal, to, span);
		let class = match self.object_classes.get(&(from, held)) {
			Some(&class) => class,
			None => self.new_object_class(from, held, span),
		};
		let object = self.objects.get_mut(&principal).expect("a made object");
		// A generic trait's class calls the impls of each of its objects' arguments, which
		// its methods refuse if they differ.
		let generic = !object.params.is_empty();
		if (object.classes.iter()).any(|c| (c.0, c.1) == (from, held) && (!generic || c.2 == to)) {
			return Some(class);
		}
		object.classes.push((from, held, to));
		for index in 0..object.methods.len() {
			if self.objects[&principal].methods[index].ty.is_some() {
				self.class_method(principal, index, from, held, to, span);
			}
		}
		Some(class)
	}

	/// The method `index` of the object of `principal` in the class wrapping a `from` held as
	/// `held`, which calls the type's impl at the arguments of the trait object `object`,
	/// made the first time.
	fn class_method(
		&mut self,
		principal: DefId,
		index: usize,
		from: Ty<'tcx>,
		held: Held,
		object: Ty<'tcx>,
		span: Span,
	) {
		let tcx = self.tcx;
		let Class { sym, v } = self.object_classes[&(from, held)];
		let method = self.objects[&principal].methods[index].clone();
		let args = instance_args(tcx, method.def_id, method.tr, object, from);
		match self.class_methods.entry((sym, method.name.clone())) {
			Entry::Occupied(called) if *called.get() == (method.def_id, args) => return,
			Entry::Occupied(called) => {
				let msg = match called.get().0 == method.def_id {
					true => format!(
						"`{from}` as `{object}`, whose class has `{}` of other arguments of the trait, which is not supported yet",
						method.name
					),
					false => format!(
						"`{from}` as `dyn {}`, whose class has another trait's method named `{}`, which is not supported yet",
						tcx.def_path_str(principal),
						method.name
					),
				};
				self.err(span, msg);
				return;
			}
			Entry::Vacant(entry) => {
				entry.insert((method.def_id, args));
			}
		}
		let sig = self.sig(method.def_id, args);
		// It takes what the object's type gives its method at the object's arguments, which
		// the impl takes as its own.
		let defined = match self.objects[&principal].params.is_empty() {
			true => None,
			false => {
				let object_args = self.object_args(principal, object, span);
				let Some(Type::Func(params, ret)) = &method.ty else {
					unreachable!("a held method's type");
				};
				let params: Vec<Type> = params.iter().map(|t| t.subst(&|p| param(&object_args, p))).collect();
				Some((params, ret.subst(&|p| param(&object_args, p))))
			}
		};
		let Some(receiver) = self.receiver(sig.inputs()[0], from, held, Expr::This.field(v)) else {
			let msg = format!(
				"`dyn {}` with the receiver `{}`, which is not supported yet",
				tcx.def_path_str(principal),
				sig.inputs()[0]
			);
			self.err(span, msg);
			return;
		};
		let mut b = Body::new(self);
		// What a `&mut` receiver points at, where std's mapping takes that, as a call's
		// operands pass it.
		let receiver = match b.t.mapped(method.def_id, args)
			&& super::pointer::pointees(b.t, method.def_id, args).first() == Some(&true)
		{
			true => b.t.pointee_value(sig.inputs()[0], receiver),
			false => receiver,
		};
		let mut params = Vec::new();
		let mut values = Vec::new();
		for (i, input) in sig.inputs()[1..].iter().enumerate() {
			let name = b.fresh("a");
			let ty = b.t.ty(*input, span);
			let value = Expr::ident(&name);
			let (ty, value) = match defined.as_ref().map(|(d, _)| d[i].clone()) {
				Some(defined) if defined != ty => (defined, Expr::As(Box::new(value), ty)),
				_ => (ty, value),
			};
			params.push(Param { name, ty: Some(ty) });
			values.push(value);
		}
		// The impl takes the location the object's caller passed, or, if only it tracks its
		// caller, its own, as rustc's shim passes.
		if let Ok(Some(imp)) = Instance::try_resolve(tcx, b.t.typing_env_pub(), method.def_id, args)
			&& super::caller::tracks(b.t, imp)
		{
			let location = match super::caller::tracks_virtual(b.t, method.def_id) {
				true => {
					let caller = b.fresh("caller");
					params.push(Param {
						name: caller.clone(),
						ty: Some(super::caller::param_ty()),
					});
					Expr::ident(caller)
				}
				false => super::caller::location(tcx, tcx.def_span(imp.def_id())),
			};
			b.site = Some((imp, location));
		}
		let mut body = Vec::new();
		let call = b.call_def(
			method.def_id,
			args,
			[receiver].into_iter().chain(values).collect(),
			span,
			&mut body,
		);
		let ret = self.ret_ty(sig.output(), span);
		// What the object's type returns, such as the `undefined` of a `T` that is `()`,
		// where the impl's `()` is `void`.
		let (ret, call) = match defined {
			Some((_, defined)) if defined != ret => (defined.clone(), Expr::As(Box::new(call.typed()), defined)),
			_ => (ret, call.typed()),
		};
		body.push(StmtKind::Return(Some(call)).at(None));
		let method = self.method(sym, method.name);
		self.program.methods.entry(sym).or_default().push(ts::Function {
			sym: method,
			params,
			ret,
			body,
			loc: None,
			doc: None,
		});
	}

	/// The class wrapping a `from` held as `held`, with no trait's methods yet: in `lib.ts`,
	/// as a copy is, named for the type after a `$`, `of$copy_Pair`.
	fn new_object_class(&mut self, from: Ty<'tcx>, held: Held, span: Span) -> Class {
		let tcx = self.tcx;
		let sym = self.program.sym(Symbol {
			module: Vec::new(),
			base: match held {
				Held::Value => "of".into(),
				Held::Place => "ofPlace".into(),
			},
			suffix: super::names::ty(tcx, from),
			owner: None,
			kind: SymKind::Struct,
			order: 0,
			span,
			instance: ty::print::with_no_trimmed_paths!(from.to_string()),
			name: String::new(),
			alias: None,
		});
		let v = self.program.field(sym, "v".into());
		let class = Class { sym, v };
		self.object_classes.insert((from, held), class);
		let ty = self.ty(from, span);
		let field_ty = match held {
			Held::Value => ty,
			Held::Place => Type::Rs("Place", vec![ty]),
		};
		self.program.items.push(Item::Struct {
			sym,
			params: Vec::new(),
			fields: vec![ts::Field {
				id: v,
				ty: field_ty,
				doc: None,
				init: FieldInit::Ctor,
			}],
			ctor: ts::Ctor::Positional,
			doc: None,
		});
		// A struct's glue drops a trait object in its last field by the object a view of
		// the struct made of where the field is.
		let value = match held {
			Held::Value => Expr::This.field(v),
			Held::Place => Expr::This.field(v).member("v"),
		};
		if let Some(dropped) = self.drop_value(value, from, span) {
			let dispose = self.method(sym, "[Symbol.dispose]".into());
			self.program.methods.entry(sym).or_default().push(ts::Function {
				sym: dispose,
				params: Vec::new(),
				ret: Type::Builtin("void"),
				body: vec![StmtKind::Expr(dropped).at(None)],
				loc: None,
				doc: None,
			});
		}
		class
	}

	/// A method `base` of the class `owner`.
	fn method(&mut self, owner: SymId, base: String) -> SymId {
		let module = self.program.syms[owner].module.clone();
		self.program.sym(Symbol {
			module,
			base,
			suffix: String::new(),
			owner: Some(owner),
			kind: SymKind::Method,
			order: 0,
			span: rustc_span::DUMMY_SP,
			instance: String::new(),
			name: String::new(),
			alias: None,
		})
	}

	/// What a wrapper of a `from`, holding `value` as `held`, passes as `self` of type
	/// `receiver`, if it can. A `&mut` wrapper is never a `Box<Self>`'s, which it reads anyway.
	fn receiver(&self, receiver: Ty<'tcx>, from: Ty<'tcx>, held: Held, value: Expr) -> Option<Expr> {
		let read = match held {
			Held::Place => value.clone().member("v"),
			Held::Value => value.clone(),
		};
		match receiver.kind() {
			ty::Ref(_, _, ty::Mutability::Not) => Some(read),
			ty::Ref(_, _, ty::Mutability::Mut) => Some(match held {
				// The class is a place of its value.
				Held::Value if self.needs_place(from) => Expr::This,
				_ => value,
			}),
			_ if receiver.is_box() => Some(read),
			_ => None,
		}
	}
}

/// Whether `def_id` is std's `Any`.
fn is_any(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
	tcx.is_diagnostic_item(rustc_span::Symbol::intern("Any"), def_id)
}

/// Whether `def_id` is std's `Error`.
pub fn is_error(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
	tcx.is_diagnostic_item(rustc_span::Symbol::intern("Error"), def_id)
}

/// Whether `ty` is a `dyn Error`.
pub fn is_error_object(tcx: TyCtxt<'_>, ty: Ty<'_>) -> bool {
	principal(ty).is_some_and(|p| is_error(tcx, p))
}

/// Whether `ty` is a `dyn Any`.
/// `dyn Any`, whose object's class a value of any type has.
pub fn any_object(tcx: TyCtxt<'_>) -> Option<Ty<'_>> {
	let any = tcx.get_diagnostic_item(rustc_span::Symbol::intern("Any"))?;
	let preds = tcx.mk_poly_existential_predicates(&[ty::Binder::dummy(ty::ExistentialPredicate::Trait(
		ty::ExistentialTraitRef::new(tcx, any, std::iter::empty::<ty::GenericArg<'_>>()),
	))]);
	Some(Ty::new_dynamic(tcx, preds, tcx.lifetimes.re_erased))
}

pub fn is_any_object(tcx: TyCtxt<'_>, ty: Ty<'_>) -> bool {
	principal(ty).is_some_and(|p| is_any(tcx, p))
}

/// Whether `ty` is a trait object, or a struct whose last field, through any structs it is
/// in, is one.
pub fn unsized_object<'tcx>(tcx: TyCtxt<'tcx>, mut ty: Ty<'tcx>) -> bool {
	while let ty::Adt(adt, args) = ty.kind()
		&& adt.is_struct()
		&& let Some(last) = adt.non_enum_variant().tail_opt()
	{
		ty = last.ty(tcx, *args).skip_norm_wip();
	}
	ty.is_trait()
}

/// What `ty` holds, if it is a `ManuallyDrop`.
fn manually_dropped<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	match ty.kind() {
		ty::Adt(adt, args) if Std::of(tcx, adt.did()) == Some(Std::ManuallyDrop) => Some(args.type_at(0)),
		_ => None,
	}
}

/// `ptr`, a reference or box, pointing at `pointee` instead.
fn repoint<'tcx>(tcx: TyCtxt<'tcx>, ptr: Ty<'tcx>, pointee: Ty<'tcx>) -> Ty<'tcx> {
	match *ptr.kind() {
		ty::Ref(region, _, mutability) => Ty::new_ref(tcx, region, pointee, mutability),
		_ => Ty::new_box(tcx, pointee),
	}
}

/// The trait of the trait object `ty`.
fn principal(ty: Ty<'_>) -> Option<DefId> {
	match ty.kind() {
		ty::Dynamic(preds, ..) => preds.principal_def_id(),
		_ => None,
	}
}

/// The arguments of a trait object's method `def_id` of a trait with the arguments `tr`:
/// those, then its own early-bound lifetimes, erased, which are all a dispatchable method
/// takes.
fn method_args<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId, tr: GenericArgs<'tcx>) -> GenericArgs<'tcx> {
	ty::GenericArgs::for_item(tcx, def_id, |param, _| match tr.get(param.index as usize) {
		Some(arg) => arg,
		None => tcx.lifetimes.re_erased.into(),
	})
}

/// The arguments of a trait object's method `def_id`, of a trait with the arguments `tr`
/// over the object's trait's own parameters, at those of the trait object `object`, for
/// `self_ty`.
fn instance_args<'tcx>(
	tcx: TyCtxt<'tcx>,
	def_id: DefId,
	tr: GenericArgs<'tcx>,
	object: Ty<'tcx>,
	self_ty: Ty<'tcx>,
) -> GenericArgs<'tcx> {
	let ty::Dynamic(preds, ..) = object.kind() else {
		unreachable!("a trait object");
	};
	let principal = tcx.instantiate_bound_regions_with_erased(preds.principal().expect("an object's trait"));
	let principal = principal.with_self_ty(tcx, self_ty).args;
	let tr = ty::EarlyBinder::bind(tcx, tr)
		.instantiate(tcx, principal)
		.skip_norm_wip();
	method_args(tcx, def_id, tr)
}

/// What the trait object `object` gives its object's type parameter `slot`.
fn param_ty<'tcx>(tcx: TyCtxt<'tcx>, object: Ty<'tcx>, slot: Slot) -> Ty<'tcx> {
	let ty::Dynamic(preds, ..) = object.kind() else {
		unreachable!("a trait object");
	};
	match slot {
		Slot::Arg(index) => {
			let principal = tcx.instantiate_bound_regions_with_erased(preds.principal().expect("an object's trait"));
			principal.args.type_at(index - 1)
		}
		Slot::Binding(def_id) => {
			let bound = preds
				.projection_bounds()
				.find(|p| p.item_def_id() == def_id)
				.expect("an object's binding");
			tcx.instantiate_bound_regions_with_erased(bound).term.expect_type()
		}
	}
}

/// The type `args` gives the parameter named `name`.
fn param(args: &[(String, Type)], name: &str) -> Option<Type> {
	args.iter().find(|(n, _)| n == name).map(|(_, t)| t.clone())
}

/// Whether a value of the type `called` is one of `defined`, the same type but where
/// `defined` has a form only an instance decides, which a cast reaches.
fn fits(defined: &Type, called: &Type) -> bool {
	let all = |a: &[Type], b: &[Type]| a.len() == b.len() && a.iter().zip(b).all(|(a, b)| fits(a, b));
	match (defined, called) {
		_ if defined == called || super::instanced(defined) => true,
		// The `()` a declared method returns as `void`.
		(Type::Builtin("undefined"), Type::Builtin("void")) => true,
		// What an `rs.OptionOf` or `rs.PollOf` resolves to at the called type's argument.
		(Type::Rs("OptionOf", x), Type::Union(y)) => match (&x[..], &y[..]) {
			([x], [some, none]) => {
				fits(x, some)
					&& *none
						== match super::holds_undefined(some) {
							true => Type::Rs("None", vec![]),
							false => Type::Builtin("undefined"),
						}
			}
			_ => false,
		},
		(Type::Rs("PollOf", x), called) => match (&x[..], called) {
			([x], Type::Rs("Poll", y)) => matches!(&y[..], [ready] if fits(x, ready) && !super::holds_pending(ready)),
			([x], Type::Union(y)) => matches!(&y[..], [ready, pending]
				if fits(x, ready) && super::holds_pending(ready) && *pending == Type::Rs("OuterPending", vec![])),
			_ => false,
		},
		(Type::Rs(a, x), Type::Rs(b, y)) => a == b && all(x, y),
		(Type::Sym(a, x), Type::Sym(b, y)) => a == b && all(x, y),
		(Type::Union(x), Type::Union(y)) | (Type::Tuple(x), Type::Tuple(y)) => all(x, y),
		(Type::Array(x), Type::Array(y)) => fits(x, y),
		(Type::Func(x, r), Type::Func(y, s)) => all(x, y) && fits(r, s),
		(Type::Object(x), Type::Object(y)) => {
			x.len() == y.len() && x.iter().zip(y).all(|((m, a), (n, b))| m == n && fits(a, b))
		}
		_ => false,
	}
}

/// Replaces each projection of `Self`'s associated type in `bound` with its parameter.
struct Bindings<'a, 'tcx> {
	tcx: TyCtxt<'tcx>,
	bound: &'a [(DefId, Ty<'tcx>)],
}

impl<'tcx> TypeFolder<TyCtxt<'tcx>> for Bindings<'_, 'tcx> {
	fn cx(&self) -> TyCtxt<'tcx> {
		self.tcx
	}

	fn fold_ty(&mut self, t: Ty<'tcx>) -> Ty<'tcx> {
		if let ty::Alias(_, alias) = t.kind()
			&& let Some(def_id) = alias.kind.try_to_projection()
			&& alias.self_ty().is_param(0)
			&& let Some(&(_, param)) = self.bound.iter().find(|(d, _)| *d == def_id)
		{
			return param;
		}
		t.super_fold_with(self)
	}
}

impl<'tcx> Body<'_, '_, 'tcx> {
	/// A call of the trait method `def_id` on a trait object, `args[0]`, through the method
	/// its object has.
	pub fn virtual_call(&mut self, def_id: DefId, object: Ty<'tcx>, mut args: Vec<Expr>, span: Span) -> Expr {
		let object = self.t.tcx.erase_and_anonymize_regions(object);
		if self.t.object(object, span).is_none() {
			return Expr::undefined();
		}
		let principal = principal(object).expect("an object's trait");
		let methods = &self.t.objects[&principal].methods;
		let Some(index) = methods.iter().position(|m| m.def_id == def_id) else {
			return self.err(
				span,
				format!("`{}` is not a method of `{object}`", self.t.tcx.def_path_str(def_id)),
			);
		};
		let name = methods[index].name.clone();
		let generic = !self.t.objects[&principal].params.is_empty();
		self.t.hold(principal, index, span);
		// Called, an object a TS caller made too.
		self.t.check_object(principal, object, span);
		let receiver = args.remove(0);
		if !generic {
			return receiver.member(name).call(args);
		}
		// What it takes and returns at the object's arguments, where its type has a form
		// only they decide, which a function it takes takes too.
		let object_args = self.t.object_args(principal, object, span);
		let Some(Type::Func(params, defined)) = self.t.objects[&principal].methods[index].ty.clone() else {
			unreachable!("a held method's type");
		};
		let Type::Func(taken, called) = self.t.called_ty(principal, index, object, span) else {
			unreachable!("a method's type");
		};
		let args = (args.into_iter().zip(params.iter().zip(&taken)))
			.map(
				|(arg, (defined, taken))| match defined.subst(&|p| param(&object_args, p)) {
					defined if defined != *taken => Expr::As(Box::new(arg), defined),
					_ => arg,
				},
			)
			.collect();
		let call = receiver.member(name).call(args);
		let defined = defined.subst(&|p| param(&object_args, p));
		// A `()` the object's type returns as `undefined` is the `void` the call is.
		match defined == *called || (defined, &*called) == (Type::Builtin("undefined"), &Type::Builtin("void")) {
			true => call,
			false => Expr::As(Box::new(call), *called),
		}
	}

	/// `value`, a `&T`, `&mut T`, or `Box<T>`, unsized to `to`, the same pointer to a trait
	/// object.
	pub fn object_of(&mut self, value: Expr, from: Ty<'tcx>, to: Ty<'tcx>, span: Span) -> Expr {
		let tcx = self.t.tcx;
		let unsupported = |b: &mut Self| b.err(span, format!("a coercion to `{to}`, which is not supported yet"));
		let (Some(pointee), Some(object)) = (from.builtin_deref(true), to.builtin_deref(true)) else {
			return unsupported(self);
		};
		// A `ManuallyDrop` is what it holds, which a box of it never drops.
		if let (Some(held), Some(object)) = (manually_dropped(tcx, pointee), manually_dropped(tcx, object)) {
			let (from, to) = (repoint(tcx, from, held), repoint(tcx, to, object));
			return self.object_of(value, from, to, span);
		}
		if !object.is_trait() {
			let view = self.tail_object(value, pointee, object, from.is_box(), span);
			return view.unwrap_or_else(|| unsupported(self));
		}
		// A box holds an array as a JS array and a reference as an `rs.Slice` of it, which one
		// class's methods can't both take.
		if pointee.is_array() {
			return self.err(
				span,
				format!("a trait object of `{pointee}`, which is not supported yet"),
			);
		}
		if let Some(f) = self.t.dyn_fn(object) {
			return self.fn_object(value, from, f.kind);
		}
		// A box of a value to write, as a `&dyn Debug` of it is, which owns and drops it.
		if let Some(tr) = principal(object).and_then(|p| super::fmt::Trait::of(self.t.tcx, p)) {
			return match pointee.kind() {
				_ if principal(pointee) == principal(object) => value,
				_ if pointee.is_trait() || !from.is_box() => unsupported(self),
				_ if self.t.polymorphic() && pointee.has_param() => Expr::undefined(),
				_ => {
					let write = self.fmt_fn(tr, pointee, span);
					let args = [value, write]
						.into_iter()
						.chain(self.t.drop_fn(pointee, span))
						.collect();
					Expr::rs("fmtOwned", args)
				}
			};
		}
		if self.t.object(object, span).is_none() {
			return Expr::undefined();
		}
		// An upcast, from a trait object, which has its supertraits' methods already, and is
		// of the class of its value's own type that a downcast checks for.
		if pointee.is_trait() {
			return match self.t.upcast_retypes(pointee, object) {
				true => Expr::As(Box::new(value), self.t.ty(to, span)),
				false => value,
			};
		}
		// A census over type parameters can't make the class: only an instance decides
		// what the pointee is.
		if self.t.polymorphic() && pointee.has_param() {
			return Expr::undefined();
		}
		let held = match from.kind() {
			ty::Ref(_, _, ty::Mutability::Mut) if self.t.needs_place(pointee) => Held::Place,
			ty::Ref(..) => Held::Value,
			_ if from.is_box() => Held::Value,
			_ => return unsupported(self),
		};
		match self.t.object_class(pointee, object, held, span) {
			Some(class) => Expr::New(Box::new(Expr::Sym(class.sym)), vec![value]),
			None => Expr::undefined(),
		}
	}

	/// `value`, a `from` reference or box, unsized to `to`: the same pointer to a trait object,
	/// or to a struct whose last field is one.
	fn unsize(&mut self, value: Expr, from: Ty<'tcx>, to: Ty<'tcx>, span: Span, out: &mut Vec<Stmt>) -> Expr {
		if let Some(tr) = self.fmt_object(to) {
			return self.fmt_value(tr, from, value, span, out);
		}
		match to.builtin_deref(true) {
			Some(object) if self.t.dyn_iter(object).is_some() => self.dyn_iter(from, to, value, span),
			Some(object) if self.t.dyn_future(object).is_some() => self.dyn_future(from, to, value, span),
			_ => self.object_of(value, from, to, span),
		}
	}

	/// `p`, a `from` raw pointer, unsized to `to`, unless it is null or dangling, which stays
	/// itself: what a box of the object it points at unsizes to, which drops it as the box
	/// `Box::from_raw` makes of it does, or else what a `&mut` to its place unsizes to. One
	/// that is the `rs.Place` of a library value, such as a cell, unsizes to the place of
	/// what a box of the value there unsizes to as each read finds it, since it may be
	/// uninitialized or replaced until then.
	pub fn raw_object(&mut self, p: Expr, from: Ty<'tcx>, to: Ty<'tcx>, span: Span) -> Expr {
		let tcx = self.t.tcx;
		let (Some(pointee), Some(object)) = (from.builtin_deref(true), to.builtin_deref(true)) else {
			return self.err(span, format!("a coercion to `{to}`, which is not supported yet"));
		};
		let erased = tcx.lifetimes.re_erased;
		let (ptr_from, ptr_to) = match self.t.needs_place(pointee) {
			true => (
				Ty::new_mut_ref(tcx, erased, pointee),
				Ty::new_mut_ref(tcx, erased, object),
			),
			false => (Ty::new_box(tcx, pointee), Ty::new_box(tcx, object)),
		};
		let read = !self.t.needs_place(pointee) && self.t.raw_place(from).is_some();
		if read && (self.t.raw_place(to).is_none() || self.t.needs_place(object)) {
			return self.err(
				span,
				format!(
					"a coercion of `{from}` to `{to}`, whose object would hold the value there as it is made, not as it is read, which is not supported yet"
				),
			);
		}
		let name = self.fresh("p");
		let mut body = Vec::new();
		let made = self.unsize(Expr::ident(&name), ptr_from, ptr_to, span, &mut body);
		let param = Param { name, ty: None };
		let ty = self.t.ty(ptr_to, span);
		let make = match body.is_empty() {
			true => Expr::Lambda(vec![param], Box::new(made)),
			false => {
				body.push(StmtKind::Return(Some(made)).at(None));
				Expr::Arrow(vec![param], ty, body)
			}
		};
		match read {
			true => Expr::rs("rawUnsizePlace", vec![p, make]),
			false => Expr::rs("rawUnsize", vec![p, make]),
		}
	}

	/// `value`, a `from`, a struct whose last field unsizes, as the struct `to` of the trait
	/// object: an `rs.tailDyn` view of it, whose last field is the object a `&mut` to that
	/// field would unsize to, made wherever it is read, so it reads and changes the field in
	/// place, and whose other fields, a cell's borrows, and a mutex's lock are `value`'s own.
	/// A box of it drops as one of `value` does. `None` if the struct is none it can view.
	fn tail_object(&mut self, value: Expr, from: Ty<'tcx>, to: Ty<'tcx>, owned: bool, span: Span) -> Option<Expr> {
		let tcx = self.t.tcx;
		if self.t.polymorphic() && from.has_param() {
			return Some(Expr::undefined());
		}
		let (ty::Adt(adt, from_args), ty::Adt(_, to_args)) = (from.kind(), to.kind()) else {
			return None;
		};
		let name = self.fresh("value");
		let v = Expr::ident(&name);
		let (key, read, lent, held) = match Std::of(tcx, adt.did()) {
			// `rs.RefCell` holds a `RefCell` or an `RwLock`.
			Some(Std::RefCell | Std::Mutex) => {
				let held = (from_args.type_at(0), to_args.type_at(0));
				let place = Expr::rs("valuePlace", vec![v.clone()]);
				(Key::Name("value".into()), v.clone().member("value"), place, held)
			}
			// `rs.Cell`, a `Cell`'s or an `UnsafeCell`'s, is the place of what it holds.
			Some(Std::Cell) => {
				let held = (from_args.type_at(0), to_args.type_at(0));
				(Key::Name("v".into()), v.clone().member("v"), v.clone(), held)
			}
			None if adt.is_struct() && self.t.own(adt.did()) => {
				let last = adt
					.non_enum_variant()
					.tail_opt()
					.expect("an unsized struct's last field");
				let AdtRepr::Struct { fields, .. } = self.t.adt(*adt, span).repr else {
					return None;
				};
				let Some(&StructField::Whole(id)) = fields.last() else {
					return None;
				};
				let held = (
					last.ty(tcx, *from_args).skip_norm_wip(),
					last.ty(tcx, *to_args).skip_norm_wip(),
				);
				let at = v.clone().field(id);
				let read = self.t.held_field(at.clone(), last.did, *from_args, span);
				let value = self.fresh("value");
				let set = Expr::Lambda(
					vec![Param {
						name: value.clone(),
						ty: None,
					}],
					Box::new(at.assign(Expr::ident(value))),
				);
				let place = Expr::rs("place", vec![Expr::Lambda(Vec::new(), Box::new(read.clone())), set]);
				(Key::Field(id), read, place, held)
			}
			_ => return None,
		};
		let (field_from, field_to) = held;
		let lent = match self.t.needs_place(field_from) {
			true => lent,
			false => read,
		};
		let erased = tcx.lifetimes.re_erased;
		let (ref_from, ref_to) = (
			Ty::new_mut_ref(tcx, erased, field_from),
			Ty::new_mut_ref(tcx, erased, field_to),
		);
		let mut body = Vec::new();
		// A future made over the field drops it, as a struct's glue drops its last field,
		// and polls it in place.
		let object = match self.t.dyn_future(field_to) {
			Some(_) => self.future_object(field_from, lent, false, true, span),
			None => self.unsize(lent, ref_from, ref_to, span, &mut body),
		};
		let param = Param {
			name,
			ty: Some(self.t.ty(from, span)),
		};
		let make = match body.is_empty() {
			true => Expr::Lambda(vec![param], Box::new(object)),
			false => {
				body.push(StmtKind::Return(Some(object)).at(None));
				Expr::Arrow(vec![param], self.t.ty(field_to, span), body)
			}
		};
		let args = [value, Expr::Object(vec![(key, make)])]
			.into_iter()
			.chain(owned.then(|| self.t.drop_fn(from, span)).flatten())
			.collect();
		Some(Expr::rs("tailDyn", args))
	}

	/// `value`, a `from` pointing at a function, as a `dyn Fn*` of `kind`: the function
	/// itself, through the references and boxes it is behind. A box owning one that an
	/// `FnOnce` call consumes but that `Fn` or `FnMut` calls is `rs.once` of it, which drops
	/// it after the call, as rustc's shim does, where an `FnOnce` closure's body drops what
	/// it owns; a box of a reference to one that drops is `rs.lent` of it, which never does.
	fn fn_object(&mut self, mut value: Expr, from: Ty<'tcx>, kind: ty::ClosureKind) -> Expr {
		let tcx = self.t.tcx;
		let (mut ty, mut owned) = (from, from.is_box());
		loop {
			ty = match ty.kind() {
				ty::Ref(_, inner, _) => {
					// A `&mut` to a function JS copies is its place.
					if self.t.place_pointee(ty).is_some() {
						value = value.member("v");
					}
					owned = false;
					*inner
				}
				ty::Adt(adt, args) if Std::of(tcx, adt.did()) == Some(Std::Box) => args.type_at(0),
				_ => break,
			};
		}
		if !from.is_box() || !self.t.drops(ty) {
			return value;
		}
		match owned {
			false => Expr::rs("lent", vec![value]),
			true if kind == ty::ClosureKind::FnOnce
				&& self.t.owned_fn(from).is_some_and(|c| c != ty::ClosureKind::FnOnce) =>
			{
				Expr::rs("once", vec![value])
			}
			true => value,
		}
	}
}

/// A call of `dyn Any`'s or `dyn Error`'s `is`, `downcast_ref`, or `downcast_mut`, or of a
/// `Box`'s, `Arc`'s, or `Rc`'s `downcast`, each of which checks for the class a coercion
/// wrapped a value of its type in, or its arguments back if it is none of these.
pub fn call<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &super::library::Call<'tcx>,
	owner: &str,
	name: &str,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Result<Option<Expr>, Vec<Expr>> {
	let tcx = b.t.tcx;
	// `TypeId::of`, and `Any::type_id` of a type's own impl, which a trait object's class calls.
	if let ("TypeId", "of") | ("Any", "type_id") = (owner, name) {
		return Ok(Some(b.t.type_id(call.args.type_at(0))));
	}
	if !matches!(
		(owner, name),
		("dyn Any" | "dyn Error", "is" | "downcast_ref" | "downcast_mut")
			| ("Box" | "Arc" | "Rc" | "dyn Error", "downcast")
	) {
		return Err(args);
	}
	let sig = b.t.sig(call.def_id, call.args);
	let receiver = sig.inputs()[0];
	let object = match receiver.kind() {
		ty::Adt(_, args) => args.type_at(0),
		_ => receiver.builtin_deref(true).unwrap_or(receiver),
	};
	if !is_any_object(tcx, object) && !is_error_object(tcx, object) {
		return Err(args);
	}
	let span = call.span;
	let to = call.args.types().last().expect("a downcast's type");
	let Some(value) = b.t.object_class(to, object, Held::Value, span) else {
		return Ok(None);
	};
	// A `&mut` to a value JS copies is the place it is in, which a class of its own holds.
	let place = match b.t.needs_place(to) && owner.starts_with("dyn ") && name != "downcast" {
		true => b.t.object_class(to, object, Held::Place, span),
		false => None,
	};
	let o = first(args);
	let o = if o.is_path() { o } else { b.spill(o, "object", out) };
	let is = |class: Class| o.clone().binary("instanceof", Expr::Sym(class.sym));
	// What the `Option` of a reference holds, which a `&mut` place never leaves `undefined`.
	let none = match sig.output().kind() {
		ty::Adt(_, args) => b.t.none(args.type_at(0)),
		_ => Expr::undefined(),
	};
	let e = match name {
		"is" => place.into_iter().fold(is(value), |e, place| e.binary("||", is(place))),
		"downcast_ref" | "downcast_mut" => {
			// A `&mut` to a value JS copies is the class itself, whose `v` is the value.
			let get = |class: Class, held: Held| match (name, held) {
				("downcast_mut", Held::Value) if place.is_some() => o.clone(),
				("downcast_mut", Held::Place) => o.clone().field(class.v),
				(_, Held::Value) => o.clone().field(class.v),
				(_, Held::Place) => o.clone().field(class.v).member("v"),
			};
			let rest = match place {
				Some(place) => Expr::Cond(Box::new(is(place)), Box::new(get(place, Held::Place)), Box::new(none)),
				None => none,
			};
			Expr::Cond(Box::new(is(value)), Box::new(get(value, Held::Value)), Box::new(rest))
		}
		_ if owner != "Arc" && owner != "Rc" => Expr::Cond(
			Box::new(is(value)),
			Box::new(Expr::result(true, o.clone().field(value.v))),
			Box::new(Expr::result(false, o.clone())),
		),
		// The `Arc` the object was made from, whose count the object shares.
		_ => {
			let held = crate::ts::Param {
				name: "object".into(),
				ty: None,
			};
			let held = Expr::Lambda(vec![held], Box::new(Expr::ident("object").field(value.v)));
			Expr::rs("arcDowncast", vec![o, Expr::Sym(value.sym), held])
		}
	};
	Ok(Some(e))
}

fn first(args: Vec<Expr>) -> Expr {
	args.into_iter().next().expect("a receiver")
}
