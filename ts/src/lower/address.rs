//! Addresses read as integers: `p as usize`, `addr`, and `expose_provenance`.
//!
//! Rust promises of an allocation's address only that it is aligned and that no other
//! live allocation overlaps it. JS frees an object only when it collects it, which no
//! program can see or time, so the one allocation JS frees as Rust does is an `Arc`'s or
//! `Rc`'s, when its strong and weak counts reach 0. `rs.addr` numbers each value an `Arc`
//! or `Rc` holds as an allocator places that allocation, and frees the number with it. A
//! pointer's address is then exact wherever its identity is, as
//! [`Translator::unidentified`] decides for comparing pointers, of a type the program keeps
//! only as an `Arc`'s or `Rc`'s value, which it never moves out of one. Nothing makes a
//! pointer of an address, which JS has no memory at.

use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_hir::def_id::DefId;
use rustc_middle::mir::{self, Operand, Rvalue, StatementKind, TerminatorKind};
use rustc_middle::ty::{self, Ty, TyCtxt, TypeVisitableExt};
use rustc_span::Span;

use super::Translator;
use super::body::Body;
use super::library::Std;
use super::pointer::Pointee;
use crate::ts::Expr;

/// The address of `p`, a pointer to `ty`, as a `usize`.
pub fn address<'tcx>(b: &mut Body<'_, '_, 'tcx>, p: Expr, ty: Ty<'tcx>, span: Span) -> Expr {
	match read(b, ty, span) {
		Ok(layout) => Expr::rs("addr", [p].into_iter().chain(layout).collect()),
		Err(refused) => refused,
	}
}

/// The size and alignment `rs.addr` reads the address of a pointer to `ty` by, or the
/// refusal of a type whose address JS has no counterpart of.
pub fn read<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Result<[Expr; 2], Expr> {
	// Only an instance decides what a parameter is, and is refused on its own.
	if !(b.t.polymorphic() && ty.has_param()) {
		let why = match b.t.pointee(ty) {
			_ if b.t.zero_sized(ty) => Some("which has no bytes and so no address of its own"),
			Pointee::Array => Some("a view JS makes of its own of an array that may be in a larger one"),
			_ if b.t.erased(ty) != ty => Some("which JS keeps as what it holds"),
			_ => b.t.unidentified(ty),
		};
		let what = format!("the address of a pointer to `{ty}`");
		if let Some(why) = why {
			return Err(b.err(span, format!("{what}, {why}")));
		}
		let read = b.t.sited(span, what);
		b.t.addresses.push((ty, read.0, read.1));
	}
	let (tcx, env) = (b.t.tcx, b.t.typing_env_pub());
	let layout = tcx.layout_of(env.as_query_input(ty)).ok();
	let size = layout.map_or(0, |l| l.size.bytes());
	let align = layout.map_or(1, |l| l.align.abi.bytes());
	Ok([size, align].map(|n| Expr::lit(n.to_string())))
}

impl<'a, 'tcx> Translator<'a, 'tcx> {
	/// Note `Arc::new_cyclic` of a `ty`, whose value JS has none of until its closure
	/// returns, so a `Weak::as_ptr` of one has nothing to point at.
	pub fn cyclic(&mut self, ty: Ty<'tcx>) {
		self.cyclic.insert(self.tcx.erase_and_anonymize_regions(ty));
	}

	/// Note `Weak::as_ptr` of a `Weak<ty>` at `span`, refused once translation settles if
	/// the crate makes a `ty` by `Arc::new_cyclic`.
	pub fn weak_pointer(&mut self, ty: Ty<'tcx>, span: Span) {
		let msg = format!(
			"`Weak::as_ptr` of a `Weak<{ty}>`, which `Arc::new_cyclic` makes and JS has no value of until its closure returns"
		);
		let (span, msg) = self.sited(span, msg);
		self.weak_pointers
			.push((self.tcx.erase_and_anonymize_regions(ty), span, msg));
	}

	/// Note `name`, such as `into_inner`, moving a `ty` out of its `Arc` or `Rc` at `span`,
	/// refused once translation settles if the program reads the address of one.
	pub fn arc_moved(&mut self, ty: Ty<'tcx>, name: &str, span: Span) {
		let msg = format!(
			"`{name}` of an `Arc` or `Rc` of `{ty}`, which moves it out of the allocation whose address the program reads"
		);
		let (span, msg) = self.sited(span, msg);
		self.arc_moves.push((super::copy::storage(self.tcx, ty), span, msg));
	}

	/// Refuse each address read of a type the program keeps anywhere but as an `Arc`'s or
	/// `Rc`'s value, or writes whole in place, each move of one out of its `Arc`, and each
	/// `Weak::as_ptr` of a value `Arc::new_cyclic` makes.
	pub(super) fn refuse_addresses(&mut self) {
		for (ty, span, msg) in std::mem::take(&mut self.weak_pointers) {
			if self.cyclic.contains(&ty) {
				self.refuse(span, msg);
			}
		}
		let reads = std::mem::take(&mut self.addresses);
		let moves = std::mem::take(&mut self.arc_moves);
		if reads.is_empty() {
			return;
		}
		let tcx = self.tcx;
		let addressed: FxHashSet<Ty<'tcx>> = reads.iter().map(|&(ty, ..)| super::copy::storage(tcx, ty)).collect();
		let mut walk = Kept::new(tcx, &addressed);
		walk.roots
			.extend(self.writes.constants(tcx).map(|ty| (ty, Some(Holder::Static))));
		walk.roots
			.extend(self.writes.kept(tcx).map(|ty| (ty, Some(Holder::Local))));
		// What the outside calls takes and returns whole, which JS keeps where it likes.
		for instance in &self.roots {
			let sig = self.sig(instance.def_id(), instance.args);
			let fn_name = tcx.def_path_str(instance.def_id());
			walk.roots.extend(
				sig.inputs_and_output
					.iter()
					.map(|ty| (ty, Some(Holder::Js(fn_name.clone())))),
			);
		}
		// Last, so first: what holds one by value names it before the local it came from.
		walk.roots.extend(self.writes.locals(tcx).map(|ty| (ty, None)));
		let kept = walk.run();
		for (ty, span, msg) in reads {
			let ty = super::copy::storage(tcx, ty);
			let why = match kept.get(&ty) {
				Some(Holder::Type(holder)) => format!("which `{holder}` holds"),
				Some(Holder::Local) => "which a local keeps that the program borrows, drops, or gives std".into(),
				Some(Holder::Static) => "which a static or constant holds".into(),
				Some(Holder::Js(name)) => format!("which JS keeps, as `{name}` takes or returns one"),
				None if self.written_whole(ty) => {
					"which the crate writes whole in place, moving another into its `Arc`".into()
				}
				None => continue,
			};
			self.refuse(
				span,
				format!("{msg}, {why}, where only an `Arc`'s or `Rc`'s value has an address JS frees as Rust does"),
			);
		}
		for (ty, span, msg) in moves {
			if addressed.contains(&ty) {
				self.refuse(span, msg);
			}
		}
	}
}

/// What `body` keeps where the program may read its address, other than as an `Arc`'s or
/// `Rc`'s value: the type of each local it borrows whole, drops, or gives a function that
/// `own` says doesn't translate, but `Arc::new`, `Rc::new`, or their `pin`, and apart,
/// the type each constant reference points at, as a static's is.
pub(super) fn kept<'tcx>(
	tcx: TyCtxt<'tcx>,
	body: &mir::Body<'tcx>,
	own: &dyn Fn(DefId) -> bool,
) -> (Vec<Ty<'tcx>>, Vec<Ty<'tcx>>) {
	let mut locals = FxHashSet::default();
	let mut constants = Vec::new();
	let constant = |operand: &Operand<'tcx>, constants: &mut Vec<Ty<'tcx>>| {
		if let Operand::Constant(c) = operand
			&& let ty::Ref(_, pointee, _) | ty::RawPtr(pointee, _) = *c.ty().kind()
		{
			constants.push(pointee);
		}
	};
	for data in body.basic_blocks.iter() {
		for stmt in &data.statements {
			let StatementKind::Assign(assign) = &stmt.kind else {
				continue;
			};
			match &assign.1 {
				Rvalue::Ref(_, _, place) | Rvalue::RawPtr(_, place) if let Some(local) = place.as_local() => {
					locals.insert(local);
				}
				Rvalue::Use(op, _) | Rvalue::Cast(_, op, _) | Rvalue::UnaryOp(_, op) => constant(op, &mut constants),
				Rvalue::Aggregate(_, ops) => ops.iter().for_each(|op| constant(op, &mut constants)),
				_ => {}
			}
		}
		match &data.terminator().kind {
			TerminatorKind::Drop { place, .. } if let Some(local) = place.as_local() => {
				locals.insert(local);
			}
			TerminatorKind::Call { func, args, .. } => {
				args.iter().for_each(|arg| constant(&arg.node, &mut constants));
				let env = ty::TypingEnv::post_analysis(tcx, body.source.def_id());
				let given = match func.const_fn_def() {
					Some((def_id, args))
						if let Ok(Some(instance)) = ty::Instance::try_resolve(tcx, env, def_id, args)
							&& let ty::InstanceKind::Item(did) = instance.def
							&& own(did) =>
					{
						false
					}
					Some((def_id, _)) => !matches!(
						super::library::key(tcx, def_id),
						Some((owner, name)) if matches!(owner.as_str(), "Arc" | "Rc") && matches!(name.as_str(), "new" | "pin")
					),
					None => true,
				};
				if given {
					locals.extend(args.iter().filter_map(|arg| arg.node.place()?.as_local()));
				}
			}
			_ => {}
		}
	}
	let locals = locals.into_iter().map(|local| body.local_decls[local].ty).collect();
	(locals, constants)
}

/// Where the program keeps a value of an addressed type other than as an `Arc`'s or
/// `Rc`'s value.
#[derive(Clone)]
enum Holder<'tcx> {
	/// By value in a value of this type: a field, an element, a payload, or a capture, or
	/// in a `Box` or a collection.
	Type(Ty<'tcx>),
	/// In a local a body borrows, drops, or gives std.
	Local,
	/// In a static or constant.
	Static,
	/// Where JS keeps what the function named takes or returns.
	Js(String),
}

/// The addressed types each value a program keeps holds where they have no address JS
/// frees as Rust does, walking what each holds by value and points at.
struct Kept<'s, 'tcx> {
	tcx: TyCtxt<'tcx>,
	addressed: &'s FxHashSet<Ty<'tcx>>,
	/// The values still to look in, each with what holds it, or `None` where it is a
	/// local, an `Arc`'s or `Rc`'s value, or what a reference points at, which hold it
	/// where an address may be its own.
	roots: Vec<(Ty<'tcx>, Option<Holder<'tcx>>)>,
	seen: FxHashSet<Ty<'tcx>>,
	found: FxHashMap<Ty<'tcx>, Holder<'tcx>>,
}

impl<'s, 'tcx> Kept<'s, 'tcx> {
	fn new(tcx: TyCtxt<'tcx>, addressed: &'s FxHashSet<Ty<'tcx>>) -> Self {
		Kept {
			tcx,
			addressed,
			roots: Vec::new(),
			seen: FxHashSet::default(),
			found: FxHashMap::default(),
		}
	}

	fn run(mut self) -> FxHashMap<Ty<'tcx>, Holder<'tcx>> {
		let tcx = self.tcx;
		let env = ty::TypingEnv::fully_monomorphized();
		while let Some((ty, holder)) = self.roots.pop() {
			let ty = tcx
				.try_normalize_erasing_regions(env, ty::Unnormalized::new_wip(ty))
				.unwrap_or(ty);
			let ty = tcx.erase_and_anonymize_regions(ty);
			if let Some(holder) = &holder
				&& self.addressed.contains(&super::copy::storage(tcx, ty))
			{
				self.found
					.entry(super::copy::storage(tcx, ty))
					.or_insert_with(|| holder.clone());
			}
			// What a type holds is held alike wherever the type is.
			if !self.seen.insert(ty) {
				continue;
			}
			let held = Some(Holder::Type(ty));
			match *ty.kind() {
				ty::Array(elem, _) | ty::Slice(elem) | ty::Pat(elem, _) => self.roots.push((elem, held)),
				ty::Tuple(tys) => self.roots.extend(tys.iter().map(|t| (t, held.clone()))),
				ty::Ref(_, pointee, _) | ty::RawPtr(pointee, _) => self.roots.push((pointee, None)),
				ty::Adt(adt, args) if super::library::is_std(tcx, adt.did()) => match Std::of(tcx, adt.did()) {
					// The value of its own allocation, which JS frees as Rust does.
					Some(Std::Arc | Std::Weak | Std::NonNull) => self.roots.push((args.type_at(0), None)),
					_ if adt.is_phantom_data() => {}
					// Each of its type arguments, by value or in a `Box` or a collection.
					_ => self.roots.extend(args.types().map(|t| (t, held.clone()))),
				},
				ty::Adt(adt, args) => self.roots.extend(
					adt.all_fields()
						.map(|f| (f.ty(tcx, args).skip_norm_wip(), held.clone())),
				),
				ty::Closure(_, args) => self
					.roots
					.extend(args.as_closure().upvar_tys().iter().map(|t| (t, held.clone()))),
				ty::CoroutineClosure(_, args) => self.roots.extend(
					args.as_coroutine_closure()
						.upvar_tys()
						.iter()
						.map(|t| (t, held.clone())),
				),
				// Its captures, and the locals it keeps across an `.await`.
				ty::Coroutine(def_id, args) => {
					self.roots
						.extend(args.as_coroutine().upvar_tys().iter().map(|t| (t, held.clone())));
					if let Ok(layout) = tcx.coroutine_layout(def_id, args) {
						for f in layout.field_tys.iter() {
							let saved = ty::EarlyBinder::bind(tcx, f.ty);
							let saved = tcx.instantiate_and_normalize_erasing_regions(args, env, saved);
							self.roots.push((saved, held.clone()));
						}
					}
				}
				_ => {}
			}
		}
		self.found
	}
}
