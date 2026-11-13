//! A function body being lowered: its names, and the calls the library mappings make
//! through it, to the crate's own functions, constructors, and std's.

use rustc_data_structures::fx::FxHashSet;
use rustc_middle::ty::{self, GenericArgs, Instance, Ty, TypeVisitableExt};
use rustc_span::Span;

use super::library;
use super::{AdtRepr, Translator, escape};
use crate::ts::{Expr, Key, Param, Stmt, StmtKind};

pub struct Body<'b, 'a, 'tcx> {
	pub t: &'b mut Translator<'a, 'tcx>,
	/// Every name in the function, so none shadows another.
	names: FxHashSet<String>,
	/// Which of the function's parameters are byte slices it takes as their parts, by
	/// [`Translator::parted`], until its own body takes them.
	pub parted: Vec<bool>,
	/// Where the function hands back its `Ok`'s parts, by [`Translator::slotted`], until
	/// its own body takes it.
	pub slotted: Option<super::mir::Slotted<'tcx>>,
	/// The function the call being lowered calls, if it takes its caller's location, and
	/// the location the call passes, until a call of it takes it.
	pub site: Option<(Instance<'tcx>, Expr)>,
	/// The std function the call being lowered maps, in a program with a panic hook, and
	/// where its panics are, if rustc says, until its mapping takes it.
	pub std_site: Option<(Instance<'tcx>, Option<Expr>)>,
	/// Where the std function whose mapping is being lowered panics, in a program with a
	/// panic hook, if known.
	pub panic_at: Option<Expr>,
}

impl<'b, 'a, 'tcx> Body<'b, 'a, 'tcx> {
	pub fn new(t: &'b mut Translator<'a, 'tcx>) -> Self {
		Body {
			t,
			names: FxHashSet::default(),
			parted: Vec::new(),
			slotted: None,
			site: None,
			std_site: None,
			panic_at: None,
		}
	}

	pub fn err(&mut self, span: Span, msg: impl Into<String>) -> Expr {
		self.t.err(span, msg);
		Expr::undefined()
	}

	/// A name no other binding in the function has.
	pub fn fresh(&mut self, base: &str) -> String {
		let base = escape(base);
		let mut name = base.clone();
		let mut n = 2;
		while !self.names.insert(name.clone()) {
			name = format!("{base}{n}");
			n += 1;
		}
		name
	}

	pub fn spill(&mut self, value: Expr, base: &str, out: &mut Vec<Stmt>) -> Expr {
		let name = self.fresh(base);
		out.push(StmtKind::decl(false, name.clone(), None, Some(value)).at(None));
		Expr::ident(name)
	}

	/// `From::from(value)`, converting a `from` into a `to`.
	pub fn convert(&mut self, value: Expr, from: Ty<'tcx>, to: Ty<'tcx>, span: Span, out: &mut Vec<Stmt>) -> Expr {
		if from == to {
			return value;
		}
		let tcx = self.t.tcx;
		let Some(from_trait) = tcx.get_diagnostic_item(rustc_span::sym::From) else {
			return self.err(span, "no `From` trait");
		};
		let Some(from_fn) = tcx
			.associated_items(from_trait)
			.in_definition_order()
			.find(|i| i.is_fn())
		else {
			return self.err(span, "no `From::from`");
		};
		let args = tcx.mk_args(&[to.into(), from.into()]);
		self.call_def(from_fn.def_id, args, vec![value], span, out)
	}

	/// A call of `def_id` with `gargs`, from wherever it resolves.
	pub fn call_def(
		&mut self,
		def_id: rustc_hir::def_id::DefId,
		gargs: GenericArgs<'tcx>,
		args: Vec<Expr>,
		span: Span,
		out: &mut Vec<Stmt>,
	) -> Expr {
		let tcx = self.t.tcx;

		// A tuple struct or variant's constructor, called or passed as a value.
		if let rustc_hir::def::DefKind::Ctor(..) = tcx.def_kind(def_id) {
			return self.ctor(def_id, gargs, args, span);
		}

		let instance = match Instance::try_resolve(tcx, self.t.typing_env_pub(), def_id, gargs) {
			Ok(Some(instance)) => instance,
			// A trait method of a type parameter: each instance resolves it.
			Ok(None) if self.t.polymorphic() && gargs.has_param() => return Expr::undefined(),
			_ => return self.err(span, format!("`{}` does not resolve", tcx.def_path_str(def_id))),
		};
		let std_site = match self.std_site.take() {
			Some((at, located)) if at == instance => Some(located),
			other => {
				self.std_site = other;
				None
			}
		};
		if let Some(sym) = self.t.bound_fn(instance, span) {
			return Expr::Sym(sym).call(args.into_iter().map(Expr::typed).collect());
		}
		if let Some(glue) = super::bindgen::glue(self.t, instance) {
			return super::bindgen::call(self, instance, glue, args, span, out);
		}
		if let ty::InstanceKind::Virtual(method, _) = instance.def
			&& self.t.dispatches(instance)
		{
			let mut args = args;
			if super::caller::tracks(self.t, instance) {
				args.push(self.caller_arg(instance, span));
			}
			return self.virtual_call(method, gargs.type_at(0), args, span);
		}
		// An `async` body's `Future::poll`, which its future has itself.
		if let ty::InstanceKind::Item(did) = instance.def
			&& tcx.is_coroutine(did)
		{
			let mut values = args;
			let future = values.remove(0);
			return future.member("poll").call(values);
		}
		if let ty::InstanceKind::Item(did) = instance.def
			&& self.t.lowers(self.t.typing_env_pub(), instance)
		{
			let values: Vec<Expr> = args.into_iter().map(Expr::typed).collect();
			// An `async fn` that only makes its future: the future, made here. Its body is
			// the future's, so its caller's location is never read.
			if let Some(class) = super::mir::coroutine::of_async_fn(self.t, instance, span) {
				if self.site.as_ref().is_some_and(|(at, _)| *at == instance) {
					self.site = None;
				}
				return Expr::New(Box::new(Expr::Sym(class)), values);
			}
			let call = self.direct_call(instance, values, span, out);
			// The crate's own `next` returns an item or `rs.DONE`, and its caller an `Option`.
			if super::iter::is_next(tcx, did) {
				let option = self.t.sig(did, instance.args).output();
				self.t.ty(option, span);
				let ty::Adt(_, option_args) = option.kind() else {
					unreachable!("`next` returns an `Option`")
				};
				let none = self.t.none_arg(option_args.type_at(0));
				return Expr::rs("iterToOption", [call].into_iter().chain(none).collect());
			}
			// One that hands back its `Ok`'s parts, made whole here, before anything else can
			// write its slot.
			if let Some(slotted) = self.t.slotted(instance) {
				let ret = self.fresh(if slotted.returned { "part" } else { "err" });
				out.push(StmtKind::decl(false, ret.clone(), None, Some(call)).at(None));
				let value = slotted.rebuild(self.t, &ret, span);
				let result = self.fresh("result");
				let ty = self.t.ty(slotted.ret, span);
				out.push(StmtKind::decl(false, result.clone(), Some(ty), Some(value)).at(None));
				return Expr::ident(result);
			}
			return call;
		}
		let start = out.len();
		let outer = self.panic_at.clone();
		if let Some(at) = &std_site {
			self.panic_at = at.clone();
		}
		let mut call = self.library_call(def_id, gargs, instance, args, span, out);
		self.panic_at = outer;
		if let Some(at) = std_site {
			Self::locate_std(at, &mut call, &mut out[start..]);
		}
		call
	}

	/// A call of std's `instance`, by `def_id` with `gargs`, which its mapping makes.
	fn library_call(
		&mut self,
		def_id: rustc_hir::def_id::DefId,
		gargs: GenericArgs<'tcx>,
		instance: Instance<'tcx>,
		args: Vec<Expr>,
		span: Span,
		out: &mut Vec<Stmt>,
	) -> Expr {
		let tcx = self.t.tcx;
		if let Some(referent) = library::referent(tcx, def_id, gargs, instance) {
			// An operand that was a reference to an `rs.Place`, as `&&mut u32` is, passes
			// what the Place points at, and a `&mut` to a `&mut` or a box what the place it
			// is holds, as JS holds a box as what it holds, but where that is a place too. A
			// mapping that takes what a `&mut` points at, as `pointees` says, was given the
			// inner `&mut`, and takes what that points at.
			let inputs = |gargs| self.t.sig(def_id, gargs).inputs().to_vec();
			let pointees = |gargs| super::pointer::pointees(self.t, def_id, gargs);
			let (given, taken) = (pointees(gargs), pointees(referent));
			let args = std::iter::zip(args, std::iter::zip(inputs(gargs), inputs(referent)))
				.enumerate()
				.map(|(i, (arg, (was, now)))| match was.kind() {
					_ if was == now => arg,
					ty::Ref(_, inner, mutability) if inner.is_box() => match mutability {
						ty::Mutability::Mut if self.t.place_pointee(now).is_none() => self.t.pointee_value(was, arg),
						_ => arg,
					},
					ty::Ref(_, _, ty::Mutability::Mut) => {
						let inner = match given[i] {
							true => arg,
							false => self.t.pointee_value(was, arg),
						};
						match taken[i] {
							true => self.t.pointee_value(now, inner),
							false => inner,
						}
					}
					ty::Ref(_, inner, _) => self.t.pointee_value(*inner, arg),
					_ => arg,
				})
				.collect();
			return self.call_def(def_id, referent, args, span, out);
		}
		if let Some((inherited, via)) = library::Inherited::of(tcx, instance) {
			let value = self.call_def(via, gargs, args, span, out);
			return inherited.test(self, value, out);
		}
		let call = library::Call {
			def_id,
			args: gargs,
			span,
		};
		match library::call(self, &call, args, out) {
			Some(e) => e,
			None => self.err(
				span,
				format!("unsupported call to `{}`", tcx.def_path_str_with_args(def_id, gargs)),
			),
		}
	}

	/// Pass `at`, where std's `instance` panics, to each runtime call its mapping made,
	/// `call` and `stmts`, that panics there. Without one, each passes `undefined`, so a
	/// panic there finds no location for its hook, which the runtime throws on.
	fn locate_std(at: Option<Expr>, call: &mut Expr, stmts: &mut [Stmt]) {
		let at = at.unwrap_or_else(Expr::undefined);
		super::caller::locate_expr(call, &at);
		super::caller::locate(stmts, &at);
	}

	fn ctor(
		&mut self,
		def_id: rustc_hir::def_id::DefId,
		gargs: GenericArgs<'tcx>,
		args: Vec<Expr>,
		span: Span,
	) -> Expr {
		let tcx = self.t.tcx;
		let values = args;
		let parent = tcx.parent(def_id);
		let (adt_id, variant) = match tcx.def_kind(parent) {
			rustc_hir::def::DefKind::Variant => (
				tcx.parent(parent),
				tcx.adt_def(tcx.parent(parent)).variant_index_with_id(parent),
			),
			_ => (parent, rustc_abi::FIRST_VARIANT),
		};
		let adt = tcx.adt_def(adt_id);
		if let Some(std) = library::Std::of(tcx, adt_id) {
			let mut values = values;
			return match (std, variant.index()) {
				(library::Std::Option, 1) | (library::Std::Poll, 0) => values.pop().unwrap(),
				(library::Std::Cow, _)
					if let Some(borrowed) = library::cow_borrowed(tcx, Ty::new_adt(tcx, adt, gargs)) =>
				{
					library::cow_made(borrowed, variant, values.pop().unwrap())
				}
				(library::Std::Result, i) => Expr::result(i == 0, values.pop().unwrap()),
				(
					library::Std::Reverse
					| library::Std::AssertUnwindSafe
					| library::Std::Wrapping
					| library::Std::Saturating,
					_,
				) => values.pop().unwrap(),
				(library::Std::Bound | library::Std::TryLockError, _) => {
					let ts_ty = self.t.ty(Ty::new_adt(tcx, adt, gargs), span);
					library::tagged(adt.variant(variant).name.as_str(), values.pop(), ts_ty)
				}
				_ => self.err(span, format!("constructing `{}`", tcx.def_path_str(adt_id))),
			};
		}
		if !self.t.own(adt_id) {
			return self.err(
				span,
				format!("constructing `{}` {}", tcx.def_path_str(adt_id), self.t.foreign(adt_id)),
			);
		}
		let info = self.t.adt(adt, span);
		let ty = tcx.type_of(adt_id).instantiate(tcx, gargs).skip_norm_wip();
		match info.repr {
			AdtRepr::Struct { ctor, fields } => {
				let args = (adt.variant(variant).fields.iter().zip(values))
					.map(|(field, value)| {
						let value = self.t.field_value(value, adt_id, field.did, gargs, span);
						field_arg(tcx, field, value)
					})
					.collect();
				self.t.construct(info.sym, &ctor, &fields, args)
			}
			AdtRepr::Tags => {
				let ts_ty = self.t.ty(ty, span);
				Expr::As(Box::new(Expr::Str(adt.variant(variant).name.to_string())), ts_ty)
			}
			AdtRepr::Slots(_) => self.err(span, format!("constructing union `{}`", tcx.def_path_str(adt_id))),
			AdtRepr::Union => {
				let mut props = vec![(
					Key::Name("tag".into()),
					Expr::Str(adt.variant(variant).name.to_string()),
				)];
				for (i, value) in values.into_iter().enumerate() {
					props.push((Key::Name(format!("_{i}")), value));
				}
				let ts_ty = self.t.ty(ty, span);
				Expr::As(Box::new(Expr::Object(props)), ts_ty)
			}
		}
	}

	/// A call of `instance`, whose body translates, with `values`, each byte slice it takes
	/// as its parts split into them.
	pub fn direct_call(
		&mut self,
		instance: Instance<'tcx>,
		values: Vec<Expr>,
		span: Span,
		out: &mut Vec<Stmt>,
	) -> Expr {
		let parted = self.t.parted(instance);
		let mut values = match parted.is_empty() {
			true => values,
			false => self.slice_args(values, &parted, out),
		};
		if super::caller::tracks(self.t, instance) {
			values.push(self.caller_arg(instance, span));
		}
		Expr::Sym(self.t.callee(instance, span)).call(values)
	}

	/// `values`, with each byte slice `parted` marks as its `data`, `start`, and `length`,
	/// evaluated in the order `values` are: a slice made in place gives its parts, and a
	/// path or a tuple's part is read three times, but anything else is held first, after
	/// what comes before it.
	fn slice_args(&mut self, values: Vec<Expr>, parted: &[bool], out: &mut Vec<Stmt>) -> Vec<Expr> {
		let mut args: Vec<Expr> = Vec::with_capacity(values.len() + 2);
		for (value, &parted) in values.into_iter().zip(parted) {
			if !parted {
				args.push(value);
				continue;
			}
			let mut made = &value;
			while let Expr::Located(_, e) | Expr::As(e, _) = made {
				made = e;
			}
			if let Some(parts) = super::mir::made_slice(made) {
				args.extend(parts.iter().cloned());
				continue;
			}
			let slice = match &value {
				Expr::Index(base, i) if base.is_path() && i.is_constant() => value,
				_ if value.is_path() => value,
				_ => {
					for arg in &mut args {
						if !arg.is_constant() {
							let name = self.fresh("arg");
							let value = std::mem::replace(arg, Expr::ident(&name));
							out.push(StmtKind::decl(false, name, None, Some(value)).at(None));
						}
					}
					let name = self.fresh("slice");
					out.push(StmtKind::decl(false, name.clone(), None, Some(value)).at(None));
					Expr::ident(name)
				}
			};
			args.extend(["data", "start", "length"].map(|key| slice.clone().member(key)));
		}
		args
	}

	/// A value of one of std's types made of zero-sized `values`, which MIR reads as a
	/// constant rather than from what made it, or `values` back for another type.
	pub fn zero_sized_std(&mut self, ty: Ty<'tcx>, values: Vec<Expr>, span: Span) -> Result<Expr, Vec<Expr>> {
		let tcx = self.t.tcx;
		let ty::Adt(adt, _) = ty.kind() else {
			return Err(values);
		};
		if library::nothing(tcx, ty) {
			return Ok(Expr::undefined());
		}
		// Typed, as what reads it may infer nothing of it.
		if library::empty_iter(tcx, adt.did()) {
			let ts_ty = self.t.ty(ty, span);
			return Ok(Expr::As(Box::new(Expr::rs("iterEmpty", Vec::new())), ts_ty));
		}
		let Some((make, gargs)) = super::iter::maker(self.t, ty) else {
			return Err(values);
		};
		let mut pre = Vec::new();
		let made = self.call_def(make, gargs, values, span, &mut pre);
		Ok(match pre.is_empty() {
			true => made,
			false => self.err(span, format!("a constant of type `{ty}` whose making takes statements")),
		})
	}

	/// A function item as a value, typed as one: a declared function's `()` result is `void`,
	/// which a value's type says is `undefined`.
	pub fn fn_value(&mut self, def_id: rustc_hir::def_id::DefId, gargs: GenericArgs<'tcx>, span: Span) -> Expr {
		let tcx = self.t.tcx;
		let sig = self.t.sig(def_id, gargs);
		// A constructor makes its value, as a call of it does.
		if matches!(tcx.def_kind(def_id), rustc_hir::def::DefKind::Ctor(..)) {
			let names: Vec<String> = (0..sig.inputs().len()).map(|i| self.fresh(&format!("a{i}"))).collect();
			let mut body = Vec::new();
			let made = self.call_def(def_id, gargs, names.iter().map(Expr::ident).collect(), span, &mut body);
			body.push(StmtKind::Return(Some(made.typed())).at(None));
			let params = std::iter::zip(names, sig.inputs())
				.map(|(name, ty)| Param {
					name,
					ty: Some(self.t.ty(*ty, span)),
				})
				.collect();
			let ret = self.t.ty(sig.output(), span);
			return Expr::Arrow(params, ret, body);
		}
		// A binding's export, or a function whose body translates, is named directly.
		if let Ok(Some(instance)) = Instance::try_resolve(tcx, self.t.typing_env_pub(), def_id, gargs) {
			// Std's `Fn` impls call it from their own source, whose location it would take, as
			// would `Location::caller` itself.
			if super::caller::tracks(self.t, instance) || super::caller::is_caller(tcx, def_id) {
				let path = tcx.def_path_str(def_id);
				return self.err(
					span,
					format!(
						"`{path}`, a `#[track_caller]` function, as a value std's code calls, whose caller is in std's source"
					),
				);
			}
			let sym = match self.t.bound_fn(instance, span) {
				Some(sym) => Some(sym),
				// One that takes byte slices as their parts, hands back its `Ok`'s parts, or is a
				// `next`, which returns an item or `rs.DONE`, is wrapped to take and return what
				// a call does.
				None => (self.t.lowers(self.t.typing_env_pub(), instance)
					&& self.t.parted(instance).is_empty()
					&& self.t.slotted(instance).is_none()
					&& !super::iter::is_next(tcx, instance.def_id()))
				.then(|| self.t.callee(instance, span)),
			};
			if let Some(sym) = sym {
				return match sig.output().is_unit() {
					true => Expr::As(Box::new(Expr::Sym(sym)), self.t.fn_ty(sig.inputs(), sig.output(), span)),
					false => Expr::Sym(sym),
				};
			}
		}
		// Any other, std's included, is its reify shim, which calls it as written calls do.
		super::mir::fn_value(self, def_id, gargs, span)
	}
}

/// A field's value as a class's constructor takes it: without an outer `as` where the
/// field's type gives the value its type, but not where TypeScript infers a class
/// parameter from it, which would widen a literal.
fn field_arg<'tcx>(tcx: ty::TyCtxt<'tcx>, field: &ty::FieldDef, value: Expr) -> Expr {
	match tcx
		.type_of(field.did)
		.instantiate_identity()
		.skip_norm_wip()
		.has_param()
	{
		true => value,
		false => value.typed(),
	}
}
