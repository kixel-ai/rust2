//! tokio's sync channels, which translation maps itself, as it maps std: `mpsc`, bounded
//! and unbounded, `watch`, and `oneshot`, whose handles are `rs/tokio.ts`'s classes and
//! whose `async fn`s are its futures, and the support `select!` calls. Their errors, and
//! the rest of tokio a crate reaches, translate from tokio's MIR.

use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::Span;

use super::body::Body;
use super::library::{Call, targ};
use crate::ts::{Expr, Stmt};

/// A handle of one of tokio's channels, or what one lends, which JS keeps as a class of
/// `rs/tokio.ts`'s.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Handle {
	/// `mpsc::Sender<T>` and `UnboundedSender<T>`.
	MpscSender,
	/// `mpsc::Receiver<T>` and `UnboundedReceiver<T>`.
	MpscReceiver,
	/// `mpsc::WeakSender<T>` and `WeakUnboundedSender<T>`.
	MpscWeakSender,
	MpscPermit,
	/// `mpsc::PermitIterator<T>`.
	MpscPermits,
	MpscOwnedPermit,
	WatchSender,
	WatchReceiver,
	WatchRef,
	OneshotSender,
	OneshotReceiver,
}

impl Handle {
	/// The handle `did` is, if it is one of tokio's.
	pub fn of(tcx: TyCtxt<'_>, did: DefId) -> Option<Self> {
		if tcx.crate_name(did.krate).as_str() != "tokio" || !matches!(tcx.def_kind(did), DefKind::Struct) {
			return None;
		}
		let module = tcx.opt_item_name(tcx.parent(did))?;
		Some(match (module.as_str(), tcx.opt_item_name(did)?.as_str()) {
			("bounded", "Sender") | ("unbounded", "UnboundedSender") => Self::MpscSender,
			("bounded", "Receiver") | ("unbounded", "UnboundedReceiver") => Self::MpscReceiver,
			("bounded", "WeakSender") | ("unbounded", "WeakUnboundedSender") => Self::MpscWeakSender,
			("bounded", "Permit") => Self::MpscPermit,
			("bounded", "PermitIterator") => Self::MpscPermits,
			("bounded", "OwnedPermit") => Self::MpscOwnedPermit,
			("watch", "Sender") => Self::WatchSender,
			("watch", "Receiver") => Self::WatchReceiver,
			("watch", "Ref") => Self::WatchRef,
			("oneshot", "Sender") => Self::OneshotSender,
			("oneshot", "Receiver") => Self::OneshotReceiver,
			_ => return None,
		})
	}

	/// The handle `ty` is, if it is one.
	pub fn of_ty(tcx: TyCtxt<'_>, ty: Ty<'_>) -> Option<Self> {
		match ty.kind() {
			ty::Adt(adt, _) => Self::of(tcx, adt.did()),
			_ => None,
		}
	}

	/// Its class in `rs/tokio.ts`.
	pub fn class(self) -> &'static str {
		match self {
			Self::MpscSender => "MpscSender",
			Self::MpscReceiver => "MpscReceiver",
			Self::MpscWeakSender => "MpscWeakSender",
			Self::MpscPermit => "MpscPermit",
			Self::MpscPermits => "MpscPermits",
			Self::MpscOwnedPermit => "MpscOwnedPermit",
			Self::WatchSender => "WatchSender",
			Self::WatchReceiver => "WatchReceiver",
			Self::WatchRef => "WatchRef",
			Self::OneshotSender => "OneshotSender",
			Self::OneshotReceiver => "OneshotReceiver",
		}
	}
}

/// Whether `did`, an item of tokio, has a mapping in place of its MIR: a handle, a function
/// or impl of one, the future of one of their `async fn`s, a channel's constructor, or what
/// `select!` calls. The mapping stands in for it or refuses it.
pub fn mapped(tcx: TyCtxt<'_>, did: DefId) -> bool {
	if tcx.crate_name(did.krate).as_str() != "tokio" {
		return false;
	}
	// A closure or `async` body's function, as `typeck_root_def_id` finds it, which panics
	// at another crate's constant.
	let mut did = did;
	while matches!(tcx.def_kind(did), DefKind::Closure | DefKind::SyntheticCoroutineBody) {
		did = tcx.parent(did);
	}
	if let Some(imp) = tcx
		.opt_parent(did)
		.filter(|&imp| matches!(tcx.def_kind(imp), DefKind::Impl { .. }))
	{
		return Handle::of_ty(tcx, tcx.type_of(imp).instantiate_identity().skip_norm_wip()).is_some();
	}
	Handle::of(tcx, did).is_some() || free(tcx, did).is_some()
}

/// A free function of tokio's the mapping names: its module and name.
fn free(tcx: TyCtxt<'_>, did: DefId) -> Option<(&'static str, &'static str)> {
	if !matches!(tcx.def_kind(did), DefKind::Fn) {
		return None;
	}
	let module = tcx.opt_item_name(tcx.parent(did))?;
	Some(match (module.as_str(), tcx.item_name(did).as_str()) {
		("bounded", "channel") => ("mpsc", "channel"),
		("unbounded", "unbounded_channel") => ("mpsc", "unbounded_channel"),
		("watch", "channel") => ("watch", "channel"),
		("oneshot", "channel") => ("oneshot", "channel"),
		("support", "thread_rng_n") => ("select", "thread_rng_n"),
		("support", "poll_budget_available") => ("select", "poll_budget_available"),
		_ => return None,
	})
}

/// A call of one of tokio's functions: its mapping, or a refusal.
pub fn call<'tcx>(b: &mut Body<'_, '_, 'tcx>, call: &Call<'tcx>, args: Vec<Expr>, out: &mut Vec<Stmt>) -> Option<Expr> {
	let tcx = b.t.tcx;
	let name = tcx.item_name(call.def_id);
	let name = name.as_str();
	let imp = tcx
		.opt_parent(call.def_id)
		.filter(|&imp| matches!(tcx.def_kind(imp), DefKind::Impl { .. }));
	let Some(imp) = imp else {
		return free_call(b, call, args, out);
	};
	let self_ty = tcx.type_of(imp).instantiate_identity().skip_norm_wip();
	let handle = Handle::of_ty(tcx, self_ty)?;
	let ty::Adt(adt, _) = self_ty.kind() else {
		return None;
	};
	let unbounded = tcx.item_name(adt.did()).as_str().starts_with("Unbounded");
	if name.starts_with("blocking_") {
		return Some(b.err(
			call.span,
			format!("`{name}`, which blocks the thread until it is done, and no JS thread can block"),
		));
	}
	// The value type every handle is over.
	let value = targ(call.args, 0);
	let span = call.span;
	let mut args = args.into_iter();
	let mut arg = || args.next().expect("an argument");
	let output = b.t.sig(call.def_id, call.args).output();
	let rs = Expr::rs;
	Some(match (handle, name) {
		(Handle::MpscSender, "send") if unbounded => {
			let (tx, v) = (arg(), arg());
			let error = maker(b, err_of(output), None, span);
			rs("mpscUnboundedSend", vec![tx, v, error])
		}
		(Handle::MpscSender, "send") => {
			let (tx, v) = (arg(), arg());
			let output = b.t.future_output(output, span);
			let error = maker(b, err_of(output), None, span);
			rs(
				"mpscSend",
				[vec![tx, v, error], b.t.drop_fn(value, span).into_iter().collect()].concat(),
			)
		}
		(Handle::MpscSender, "try_send") => {
			let (tx, v) = (arg(), arg());
			let full = maker(b, err_of(output), Some("Full"), span);
			let closed = maker(b, err_of(output), Some("Closed"), span);
			rs("mpscTrySend", vec![tx, v, full, closed])
		}
		(Handle::MpscSender, "send_timeout") => {
			return Some(b.err(
				span,
				"`send_timeout`, which waits on tokio's timer, which is not supported",
			));
		}
		(Handle::MpscSender, "closed") => rs("mpscClosed", vec![arg()]),
		(Handle::MpscSender, "is_closed") => rs("mpscIsClosed", vec![arg()]),
		(Handle::MpscSender, "reserve" | "reserve_owned") => {
			let tx = arg();
			let output = b.t.future_output(output, span);
			let error = made(b, err_of(output), None, vec![Expr::undefined()], span, out);
			let f = if name == "reserve" {
				"mpscReserve"
			} else {
				"mpscReserveOwned"
			};
			rs(f, vec![tx, error])
		}
		(Handle::MpscSender, "reserve_many") => {
			let (tx, n) = (arg(), arg());
			let output = b.t.future_output(output, span);
			let error = made(b, err_of(output), None, vec![Expr::undefined()], span, out);
			rs("mpscReserveMany", vec![tx, n, error])
		}
		(Handle::MpscSender, "try_reserve" | "try_reserve_many") => {
			let mut values = vec![arg()];
			if name == "try_reserve_many" {
				values.push(arg());
			}
			let full = made(b, err_of(output), Some("Full"), vec![Expr::undefined()], span, out);
			let closed = made(b, err_of(output), Some("Closed"), vec![Expr::undefined()], span, out);
			let f = if name == "try_reserve" {
				"mpscTryReserve"
			} else {
				"mpscTryReserveMany"
			};
			rs(f, [values, vec![full, closed]].concat())
		}
		(Handle::MpscSender, "try_reserve_owned") => {
			let full = maker(b, err_of(output), Some("Full"), span);
			let closed = maker(b, err_of(output), Some("Closed"), span);
			rs("mpscTryReserveOwned", vec![arg(), full, closed])
		}
		(Handle::MpscSender | Handle::MpscReceiver, "same_channel") => rs("mpscSameChannel", vec![arg(), arg()]),
		(Handle::MpscSender | Handle::MpscReceiver, "capacity") => rs("mpscCapacity", vec![arg()]),
		(Handle::MpscSender | Handle::MpscReceiver, "max_capacity") => rs("mpscMaxCapacity", vec![arg()]),
		(Handle::MpscSender, "downgrade") => rs("mpscDowngrade", vec![arg()]),
		(Handle::MpscSender | Handle::MpscWeakSender, "strong_count")
		| (Handle::MpscReceiver, "sender_strong_count") => rs("mpscStrongCount", vec![arg()]),
		(Handle::MpscSender | Handle::MpscWeakSender, "weak_count") | (Handle::MpscReceiver, "sender_weak_count") => {
			rs("mpscWeakCount", vec![arg()])
		}
		(Handle::MpscWeakSender, "upgrade") => rs("mpscUpgrade", vec![arg()]),
		(Handle::MpscReceiver, "recv") => {
			let rx = arg();
			let none = b.t.none_arg(value);
			rs("mpscRecv", [vec![rx], none.into_iter().collect()].concat())
		}
		(Handle::MpscReceiver, "poll_recv") => {
			let (rx, cx) = (arg(), arg());
			let none = b.t.none_arg(value);
			rs("mpscPollRecv", [vec![rx, cx], none.into_iter().collect()].concat())
		}
		(Handle::MpscReceiver, "recv_many") => {
			let (rx, buffer, limit) = (arg(), arg(), arg());
			rs("mpscRecvMany", vec![rx, buffer, limit, push(value)])
		}
		(Handle::MpscReceiver, "poll_recv_many") => {
			let (rx, cx, buffer, limit) = (arg(), arg(), arg(), arg());
			rs("mpscPollRecvMany", vec![rx, cx, buffer, limit, push(value)])
		}
		(Handle::MpscReceiver, "try_recv") => {
			let empty = made(b, err_of(output), Some("Empty"), Vec::new(), span, out);
			let disconnected = made(b, err_of(output), Some("Disconnected"), Vec::new(), span, out);
			rs("mpscTryRecv", vec![arg(), empty, disconnected])
		}
		(Handle::MpscReceiver, "close") => rs("mpscClose", vec![arg()]),
		(Handle::MpscReceiver, "is_closed") => rs("mpscRxIsClosed", vec![arg()]),
		(Handle::MpscReceiver, "is_empty") => rs("mpscIsEmpty", vec![arg()]),
		(Handle::MpscReceiver, "len") => rs("mpscLen", vec![arg()]),
		(Handle::MpscPermit, "send") => rs("mpscPermitSend", vec![arg(), arg()]),
		(Handle::MpscOwnedPermit, "send") => rs("mpscOwnedPermitSend", vec![arg(), arg()]),
		(Handle::MpscOwnedPermit, "release") => rs("mpscOwnedPermitRelease", vec![arg()]),
		(Handle::MpscOwnedPermit, "same_channel") => rs("mpscOwnedPermitSameChannel", vec![arg(), arg()]),
		(Handle::MpscOwnedPermit, "same_channel_as_sender") => {
			rs("mpscOwnedPermitSameChannelAsSender", vec![arg(), arg()])
		}
		(Handle::WatchReceiver, "borrow") => rs("watchBorrow", vec![arg()]),
		(Handle::WatchReceiver, "borrow_and_update") => rs("watchBorrow", vec![arg(), Expr::lit("true")]),
		(Handle::WatchReceiver, "has_changed") => {
			let error = made(b, err_of(output), None, vec![Expr::undefined()], span, out);
			rs("watchHasChanged", vec![arg(), error])
		}
		(Handle::WatchReceiver, "is_closed") => rs("watchIsClosed", vec![arg()]),
		(Handle::WatchReceiver, "mark_changed") => rs("watchMarkChanged", vec![arg()]),
		(Handle::WatchReceiver, "mark_unchanged") => rs("watchMarkUnchanged", vec![arg()]),
		(Handle::WatchReceiver, "changed") => {
			let output = b.t.future_output(output, span);
			let error = made(b, err_of(output), None, vec![Expr::undefined()], span, out);
			rs("watchChanged", vec![arg(), error])
		}
		(Handle::WatchReceiver, "wait_for") => {
			let (rx, f) = (arg(), arg());
			let output = b.t.future_output(output, span);
			let error = made(b, err_of(output), None, vec![Expr::undefined()], span, out);
			let dropper = b.t.drop_fn(targ(call.args, 1), span);
			rs(
				"watchWaitFor",
				[vec![rx, f, error], dropper.into_iter().collect()].concat(),
			)
		}
		(Handle::WatchSender | Handle::WatchReceiver, "same_channel") => rs("watchSameChannel", vec![arg(), arg()]),
		(Handle::WatchSender, "new") => {
			let dropper = b.t.drop_fn(value, span);
			of(
				b,
				"watchSenderNew",
				value,
				[vec![arg()], dropper.into_iter().collect()].concat(),
				span,
			)
		}
		(Handle::WatchSender, "send") => {
			let (tx, v) = (arg(), arg());
			let error = maker(b, err_of(output), None, span);
			let dropper = b.t.drop_fn(value, span);
			rs(
				"watchSend",
				[vec![tx, v, error], dropper.into_iter().collect()].concat(),
			)
		}
		(Handle::WatchSender, "send_modify" | "send_if_modified") => {
			let (tx, f) = (arg(), arg());
			let closure = targ(call.args, 1);
			let copied = Expr::lit(b.t.needs_place(value).to_string());
			// What drops a closure that `Fn` or `FnMut` call after a call through `FnOnce`,
			// as rustc's shim does: one `FnOnce` calls drops what it owns itself.
			let dropper = match super::closure_kind(closure) {
				Some(ty::ClosureKind::FnOnce) => None,
				_ => b.t.drop_fn(closure, span),
			};
			let f_name = if name == "send_modify" {
				"watchSendModify"
			} else {
				"watchSendIfModified"
			};
			rs(f_name, [vec![tx, f, copied], dropper.into_iter().collect()].concat())
		}
		(Handle::WatchSender, "send_replace") => rs("watchSendReplace", vec![arg(), arg()]),
		(Handle::WatchSender, "borrow") => rs("watchSenderBorrow", vec![arg()]),
		(Handle::WatchSender, "is_closed") => rs("watchSenderIsClosed", vec![arg()]),
		(Handle::WatchSender, "closed") => rs("watchSenderClosed", vec![arg()]),
		(Handle::WatchSender, "subscribe") => rs("watchSenderSubscribe", vec![arg()]),
		(Handle::WatchSender, "receiver_count") => rs("watchReceiverCount", vec![arg()]),
		(Handle::WatchSender, "sender_count") => rs("watchSenderCount", vec![arg()]),
		(Handle::WatchRef, "has_changed") => arg().member("hasChanged"),
		(Handle::OneshotSender, "send") => rs("oneshotSend", vec![arg(), arg()]),
		(Handle::OneshotSender, "closed") => rs("oneshotClosed", vec![arg()]),
		(Handle::OneshotSender, "is_closed") => rs("oneshotIsClosed", vec![arg()]),
		(Handle::OneshotSender, "poll_closed") => rs("oneshotPollClosed", vec![arg(), arg()]),
		(Handle::OneshotReceiver, "close") => rs("oneshotClose", vec![arg()]),
		(Handle::OneshotReceiver, "is_terminated") => rs("oneshotIsTerminated", vec![arg()]),
		(Handle::OneshotReceiver, "is_empty") => rs("oneshotIsEmpty", vec![arg()]),
		(Handle::OneshotReceiver, "try_recv") => {
			let empty = made(b, err_of(output), Some("Empty"), Vec::new(), span, out);
			let closed = made(b, err_of(output), Some("Closed"), Vec::new(), span, out);
			rs("oneshotTryRecv", vec![arg(), empty, closed])
		}
		_ => return None,
	})
}

/// A call of one of tokio's free functions the mapping names.
fn free_call<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let _ = out;
	let span = call.span;
	let made = |b: &mut Body<'_, '_, 'tcx>, name: &str, args: Vec<Expr>| {
		let value = targ(call.args, 0);
		let dropper = b.t.drop_fn(value, span);
		of(b, name, value, [args, dropper.into_iter().collect()].concat(), span)
	};
	Some(match free(b.t.tcx, call.def_id)? {
		("mpsc", "channel") => made(b, "mpscChannel", args),
		("mpsc", "unbounded_channel") => made(b, "mpscUnboundedChannel", args),
		("watch", "channel") => made(b, "watchChannel", args),
		("oneshot", "channel") => made(b, "oneshotChannel", args),
		("select", "thread_rng_n") => Expr::As(
			Box::new(Expr::rs("selectStart", args)),
			super::num::Int::of(b.t.tcx.types.u32)?.ty(),
		),
		// No tokio runtime runs a translation, so no task's budget is ever spent.
		("select", "poll_budget_available") => Expr::undefined(),
		_ => return None,
	})
}

/// A call of a std trait's method on one of tokio's handles: its mapping, or its arguments
/// back for std's own mapping, as `Debug`'s and `IntoFuture`'s.
pub fn trait_call<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Result<Option<Expr>, Vec<Expr>> {
	let tcx = b.t.tcx;
	// Only a trait's method has the handle as its first type argument's `Self`.
	if tcx.trait_of_assoc(call.def_id).is_none() {
		return Err(args);
	}
	let self_ty = targ(call.args, 0);
	let Some(handle) = Handle::of_ty(tcx, self_ty.peel_refs()) else {
		return Err(args);
	};
	// A method of a reference, as `Clone` of one copies it and `Future` of a `&mut`
	// polls what it borrows, std's.
	if self_ty.is_ref() {
		return Err(args);
	}
	let ty::Adt(_, handle_args) = self_ty.peel_refs().kind() else {
		return Err(args);
	};
	let value = held(*handle_args);
	let span = call.span;
	let mut args = args.into_iter();
	let mut arg = || args.next().expect("an argument");
	Ok(Some(match (handle, owner, name) {
		(Handle::MpscSender, "Clone", "clone") => Expr::rs("mpscSenderClone", vec![arg()]),
		(Handle::MpscWeakSender, "Clone", "clone") => Expr::rs("mpscWeakClone", vec![arg()]),
		(Handle::WatchSender, "Clone", "clone") => Expr::rs("watchSenderClone", vec![arg()]),
		(Handle::WatchReceiver, "Clone", "clone") => Expr::rs("watchReceiverClone", vec![arg()]),
		(Handle::WatchRef, "Deref", "deref") => arg().member("value"),
		(Handle::WatchSender, "Default", "default") => {
			let tcx = b.t.tcx;
			let default = super::collections::trait_fn(tcx, "Default", "default");
			let init = b.call_def(default, tcx.mk_args(&[value.into()]), Vec::new(), span, out);
			let dropper = b.t.drop_fn(value, span);
			of(
				b,
				"watchSenderNew",
				value,
				[vec![init], dropper.into_iter().collect()].concat(),
				span,
			)
		}
		(Handle::OneshotReceiver, "Future", "poll") => {
			let (rx, cx) = (arg(), arg());
			let output = b.t.future_output(self_ty.peel_refs(), span);
			let error = made(b, err_of(output), None, vec![Expr::undefined()], span, out);
			let mut args = vec![rx, cx, error];
			// Where tokio's `poll` panics once it completed, which a hook reads.
			if b.t.hooked {
				let own = ty::Instance::try_resolve(tcx, b.t.typing_env_pub(), call.def_id, call.args)
					.ok()
					.flatten()
					.and_then(|poll| super::caller::own_panic(tcx, poll.def_id()));
				args.push(match own {
					Some(at) => at,
					None => b.err(
						span,
						"tokio's poll of a oneshot receiver, in a program with a panic hook, whose compiled code shows no one panic",
					),
				});
			}
			Expr::rs("oneshotPoll", args)
		}
		// The rest as std maps them: `Debug` by `fmt`, and `IntoFuture` as itself.
		_ => return Err(args.collect()),
	}))
}

/// `Debug` of one of tokio's handles, as tokio writes it, or a refusal of an mpsc one but
/// a weak sender, which writes addresses.
pub fn debug<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, value: Expr, f: Expr, span: Span) -> Option<Expr> {
	let tcx = b.t.tcx;
	let ty::Adt(adt, args) = ty.kind() else {
		return None;
	};
	let handle = Handle::of(tcx, adt.did())?;
	let shown = |b: &mut Body<'_, '_, 'tcx>| b.fmt_fn(super::fmt::Trait::Debug, held(*args), span);
	Some(match handle {
		Handle::WatchSender => Expr::rs("watchSenderDebug", vec![value, f, shown(b)]),
		Handle::WatchReceiver => Expr::rs("watchReceiverDebug", vec![value, f, shown(b)]),
		Handle::WatchRef => Expr::rs("watchRefDebug", vec![value, f, shown(b)]),
		Handle::OneshotSender => Expr::rs("oneshotDebug", vec![value, f, Expr::Str("Sender".into())]),
		Handle::OneshotReceiver => Expr::rs("oneshotDebug", vec![value, f, Expr::Str("Receiver".into())]),
		// `debug_struct(name).finish()`.
		Handle::MpscWeakSender => f
			.member("out")
			.member("writeStr")
			.call(vec![Expr::Str(tcx.item_name(adt.did()).to_string())]),
		_ => b.err(
			span,
			format!("`Debug` of `{ty}`, which writes addresses, which JS has no counterpart of"),
		),
	})
}

/// `rs.<name><T>(args)`, a function that makes a channel of `value`s, given `T` since
/// TypeScript infers none from a call without a value.
fn of<'tcx>(b: &mut Body<'_, '_, 'tcx>, name: &str, value: Ty<'tcx>, args: Vec<Expr>, span: Span) -> Expr {
	Expr::Instantiate(Box::new(Expr::Rs(name.into())), vec![b.t.ty(value, span)]).call(args)
}

/// Whether `did` is a watch sender's `send_modify` or `send_if_modified`, whose mapping
/// hands its closure a value JS copies as its place, and writes that back.
pub fn lends(tcx: TyCtxt<'_>, did: DefId) -> bool {
	let Some(imp) = tcx.inherent_impl_of_assoc(did) else {
		return false;
	};
	let ty::Adt(adt, _) = tcx.type_of(imp).instantiate_identity().skip_norm_wip().kind() else {
		return false;
	};
	Handle::of(tcx, adt.did()) == Some(Handle::WatchSender)
		&& matches!(tcx.item_name(did).as_str(), "send_modify" | "send_if_modified")
}

/// The type of the values a handle's channel holds, its one type parameter, after the
/// lifetime of a `watch::Ref`.
pub fn held<'tcx>(args: ty::GenericArgs<'tcx>) -> Ty<'tcx> {
	args.types().next().expect("a channel's value type")
}

/// `E` of `ty`, a `Result<_, E>`.
fn err_of(ty: Ty<'_>) -> Ty<'_> {
	match ty.kind() {
		ty::Adt(_, args) => args.type_at(1),
		_ => unreachable!("a `Result`"),
	}
}

/// The constructor of `ty`'s variant named `variant`, or of its struct.
fn ctor<'tcx>(ty: Ty<'tcx>, variant: Option<&str>) -> (DefId, ty::GenericArgs<'tcx>) {
	let ty::Adt(adt, args) = ty.kind() else {
		unreachable!("tokio's errors are structs and enums")
	};
	let variant = match variant {
		Some(name) => adt
			.variants()
			.iter()
			.find(|v| v.name.as_str() == name)
			.expect("tokio's error variant"),
		None => adt.non_enum_variant(),
	};
	(variant.ctor_def_id().expect("a tuple or unit constructor"), *args)
}

/// A value of `ty`'s variant `variant`, or of its struct, of `fields`.
fn made<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	ty: Ty<'tcx>,
	variant: Option<&str>,
	fields: Vec<Expr>,
	span: Span,
	out: &mut Vec<Stmt>,
) -> Expr {
	let (ctor, args) = ctor(ty, variant);
	b.call_def(ctor, args, fields, span, out)
}

/// The function making `ty`'s variant `variant`, or its struct, of its one field.
fn maker<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, variant: Option<&str>, span: Span) -> Expr {
	let (ctor, args) = ctor(ty, variant);
	b.fn_value(ctor, args, span)
}

/// What `recv_many` pushes each value onto its `Vec<T>` by, as `Vec::push` does.
fn push(value: Ty<'_>) -> Expr {
	match value.kind() {
		ty::Uint(ty::UintTy::U8) => Expr::Rs("byteVecPush".into()),
		_ => Expr::Rs("mpscVecPush".into()),
	}
}
