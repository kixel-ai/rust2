//! One walk of what crosses the C ABI, in either direction, shared by every binding: how a
//! value lowers into its C form and lifts back, which C parameters each of a call's Rust
//! parameters fills, and what a function Rust calls back takes and returns. The walk
//! decides each step and the C slots it fills; a binding spells each step in its
//! language, as wit-bindgen's `Bindgen` does for the component model. Each step is a
//! variant, so a rule adding one is a compile error in every binding until it spells it.

use crate::abi::{self, Abi, Export, Ty};
use crate::ir::{Closure, Family, Function, Int, Param, Receiver, Type};

/// How a value of a type lowers into its C form and lifts back: references and boxes
/// seen through, and each form spelled alike in C merged.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Value<'a> {
	Bool,
	Int(Int),
	/// An integer whose zero Rust's type does not hold.
	NonZero(Int),
	F32,
	F64,
	/// A code point, which may be no Unicode scalar.
	Char,
	U128,
	Duration,
	/// `[u8; N]`
	Array(usize),
	SocketAddr(Family),
	IpAddr(Family),
	/// `str` or `String`, a buffer of UTF-8.
	Text,
	/// `Path` or `PathBuf`, a buffer of the path's OS bytes.
	Path,
	/// `[u8]`, `Vec<u8>`, or `Bytes`, a buffer.
	Bytes,
	Waker,
	/// A range, bound, or pair of bounds of an integer: the C struct its type names.
	Range(&'a Type),
	/// A collection, of the element.
	List(&'a Type),
	/// An `Option`, as C holds it.
	Option(Optional<'a>),
	Record(usize),
	Enum(usize),
	Object(usize),
	Callback(usize),
	/// A tuple of one, a struct of its element alone.
	Single(&'a Type),
	/// A tuple of two or more.
	Tuple(&'a [Type]),
}

/// How C holds an `Option`: as a null of its value's own form, or in a struct of a flag
/// and the value.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Optional<'a> {
	/// A null handle.
	Object(usize),
	/// A null buffer.
	Text,
	Path,
	Bytes,
	/// A null array: the list's element.
	List(&'a Type),
	/// `<prefix>_option_<T>`, of the value.
	Some(&'a Type),
}

/// `ty`'s form as it crosses as a value.
pub(crate) fn value(ty: &Type) -> Value<'_> {
	match ty {
		Type::Bool => Value::Bool,
		Type::Int(int) => Value::Int(*int),
		Type::NonZero(int) => Value::NonZero(*int),
		Type::F32 => Value::F32,
		Type::F64 => Value::F64,
		Type::Char => Value::Char,
		Type::U128 => Value::U128,
		Type::Duration => Value::Duration,
		Type::Array(n) => Value::Array(*n),
		Type::SocketAddr(family) => Value::SocketAddr(*family),
		Type::IpAddr(family) => Value::IpAddr(*family),
		Type::Str | Type::String => Value::Text,
		Type::Path | Type::PathBuf => Value::Path,
		Type::Slice | Type::Vec | Type::Bytes => Value::Bytes,
		Type::Waker => Value::Waker,
		Type::Range(_) | Type::Bound(_) | Type::Bounds(_) => Value::Range(ty),
		Type::List(_, elem) => Value::List(elem),
		Type::Option(inner) => Value::Option(match inner.base() {
			Type::Object(i) => Optional::Object(*i),
			Type::Str | Type::String => Optional::Text,
			Type::Path | Type::PathBuf => Optional::Path,
			Type::Slice | Type::Vec | Type::Bytes => Optional::Bytes,
			Type::List(_, elem) => Optional::List(elem),
			_ => Optional::Some(inner),
		}),
		Type::Box(inner) | Type::Ref(_, inner) => value(inner),
		Type::Record(i) => Value::Record(*i),
		Type::Enum(i) => Value::Enum(*i),
		Type::Object(i) => Value::Object(*i),
		Type::Callback(i, _) => Value::Callback(*i),
		Type::Tuple(elements) => match &elements[..] {
			[element] => Value::Single(element),
			elements => Value::Tuple(elements),
		},
		Type::Unit => unreachable!("no value is unit"),
		Type::Const(_) => unreachable!("a const argument is no value"),
		Type::Jni(_) => unreachable!("only Kotlin exports what takes a JNI value"),
		Type::Reader | Type::Future | Type::Closure(_) => unreachable!("{ty:?} crosses only as a parameter"),
	}
}

/// What a buffer C lends holds, which a binding may check before the call.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Buffer {
	Text,
	Path,
	Bytes,
}

/// How one parameter of a call crosses into Rust.
#[derive(Clone, Copy)]
pub(crate) enum Arg<'a> {
	/// A value C passes in for `&mut`, of the type behind the reference, which Rust writes
	/// back through its out, [`Abi::written`].
	Written(&'a Type),
	/// A closure: its function, its data, and for one Rust keeps, the free Rust calls once
	/// it drops it.
	Closure(&'a Closure),
	/// A reader the binding implements, lent for the call.
	Reader,
	/// A trait the binding implements, given to Rust.
	Callback(usize),
	Waker,
	/// A future of `()` the binding implements, given to Rust.
	Future,
	/// A code point, which may be no Unicode scalar.
	Char,
	/// An integer whose zero Rust's type does not hold.
	NonZero(Int),
	/// A buffer, or an `Option` of one, lent as a view.
	Buffer { holds: Buffer, optional: bool },
	/// An object's handle, or for an `Option`, a null one: the C handle type named.
	Handle { c: &'a str, optional: bool },
	/// Any other value, as it lowers.
	Value(&'a Type),
}

/// A parameter of a call, as it crosses.
pub(crate) struct Lowered<'a> {
	/// Its place among the parameters after `self`.
	pub(crate) k: usize,
	pub(crate) param: &'a Param,
	pub(crate) arg: Arg<'a>,
	/// Whether the call moves an object's value out of a handle it holds, which then
	/// closes: by value, boxed, or in an `Option`, a list, or a tuple, unless `Copy`.
	pub(crate) moves: bool,
}

/// How a method's receiver crosses.
pub(crate) enum Receiving<'a> {
	/// An object's handle, of the C type named: `moves` its value out, as an owned `self`
	/// of an object that is not `Copy` does.
	Handle { c: &'a str, moves: bool },
	/// A value read through a pointer, and for `&mut self`, `written` back in place.
	Value { written: bool },
}

/// A value a call returns, owned, and the C out it is written through where C returns
/// something else: a status, a future, or another element of its tuple.
pub(crate) struct Returned {
	pub(crate) ty: Type,
	pub(crate) out: String,
}

/// A call into Rust of a function, as every binding makes it.
pub(crate) struct Call<'a> {
	pub(crate) receiver: Option<Receiving<'a>>,
	pub(crate) params: Vec<Lowered<'a>>,
	/// What it returns: none for `()`, each element of a tuple, or the value.
	pub(crate) oks: Vec<Returned>,
	/// The error of a `Result`, owned, through the out `err`.
	pub(crate) err: Option<Type>,
	/// The outs C writes after the parameters, but the status.
	pub(crate) outs: Vec<&'a (String, Ty)>,
	/// Whether a check of a handle can fail the call, which then writes its status.
	pub(crate) status: bool,
}

/// The call of `function` through `export`, its C signature, as a method of the type
/// whose C type is `owner` if it has one.
pub(crate) fn call<'a>(abi: &Abi<'_>, function: &'a Function, owner: Option<&Ty>, export: &'a Export) -> Call<'a> {
	let mut c_params = export.call.params.iter();
	let receiver = function.receiver.filter(|_| owner.is_some()).map(|receiver| {
		let (_, c) = c_params.next().expect("a method has its receiver");
		match c {
			Ty::Handle { name, constant } => Receiving::Handle {
				c: name,
				// A `Copy` object's handle is `const` when taken by value, as it is copied out.
				moves: receiver == Receiver::Owned && !constant,
			},
			c => Receiving::Value {
				written: matches!(c, Ty::InOut(_)),
			},
		}
	});
	let mut params = Vec::new();
	for (k, param) in function.params.iter().enumerate() {
		let (_, c) = c_params.next().expect("each parameter has its C one");
		let moves = moved(abi, &param.ty);
		let arg = match (&param.ty, c) {
			(ty, _) if ty.written() => Arg::Written(ty.base()),
			(Type::Closure(closure), _) => {
				c_params.next().expect("a closure has its data");
				if closure.kept {
					c_params.next().expect("a kept closure has its free");
				}
				Arg::Closure(closure)
			}
			(_, Ty::ForeignReader) => Arg::Reader,
			(Type::Callback(i, _), _) => Arg::Callback(*i),
			(Type::Waker, _) => Arg::Waker,
			(Type::Future, _) => Arg::Future,
			(Type::Char, _) => Arg::Char,
			(Type::NonZero(int), _) => Arg::NonZero(*int),
			(ty, Ty::Buf) => {
				let (holds, optional) = match ty.buffer() {
					Some(Type::Option(inner)) => (buffer(&inner), true),
					Some(held) => (buffer(&held), false),
					None => unreachable!("a buffer holds {ty:?}"),
				};
				Arg::Buffer { holds, optional }
			}
			(ty, Ty::Handle { name, .. }) => Arg::Handle {
				c: name,
				optional: matches!(ty.base(), Type::Option(_)),
			},
			(ty, _) => Arg::Value(ty),
		};
		params.push(Lowered { k, param, arg, moves });
	}
	let outs = c_params.filter(|(_, ty)| !abi::is_status(ty)).collect();
	// A tuple returned whole is written element by element, even a tuple of one.
	let oks = match &function.ok {
		Type::Unit => Vec::new(),
		Type::Tuple(elements) => (elements.iter().enumerate())
			.map(|(k, ty)| Returned {
				ty: owned(ty),
				out: format!("out{k}"),
			})
			.collect(),
		ok => vec![Returned {
			ty: owned(ok),
			out: "out".to_string(),
		}],
	};
	Call {
		receiver,
		params,
		oks,
		err: function.err.as_ref().map(owned),
		outs,
		status: export.call.status().is_some(),
	}
}

fn buffer(held: &Type) -> Buffer {
	match held {
		Type::String => Buffer::Text,
		Type::PathBuf => Buffer::Path,
		Type::Bytes => Buffer::Bytes,
		other => unreachable!("a buffer holds {other:?}"),
	}
}

/// Whether passing `ty` moves an object's value out: by value, boxed, or in an `Option`,
/// a list, or a tuple, unless `Copy`, and never through a reference.
pub(crate) fn moved(abi: &Abi<'_>, ty: &Type) -> bool {
	match ty {
		Type::Object(i) => !abi.api.objects[*i].copy,
		Type::Option(inner) | Type::Box(inner) | Type::List(_, inner) => moved(abi, inner),
		Type::Tuple(elements) => elements.iter().any(|ty| moved(abi, ty)),
		_ => false,
	}
}

/// `ty` returned: a borrow is returned as a clone, so owned.
pub(crate) fn owned(ty: &Type) -> Type {
	match ty {
		Type::Ref(_, inner) => owned(inner),
		Type::Option(inner) => Type::Option(Box::new(owned(inner))),
		Type::Box(inner) => Type::Box(Box::new(owned(inner))),
		Type::List(c, inner) => Type::List(*c, Box::new(owned(inner))),
		Type::Tuple(elements) => Type::Tuple(elements.iter().map(owned).collect()),
		ty => ty.clone(),
	}
}

/// How one value Rust passes a function the binding implements lifts.
#[derive(Clone, Copy)]
pub(crate) enum Lent<'a> {
	/// An object Rust lends for the call, which the binding must not keep past it.
	Object(usize),
	/// A value Rust lends for the call: bytes and text the binding copies.
	Value(&'a Type),
}

/// What a function the binding implements returns to Rust, which Rust then owns.
#[derive(Clone, Copy)]
pub(crate) enum Return<'a> {
	Unit,
	Value(&'a Type),
	/// A `Result`, whose value, if not `()`, and error are written through the outs `ok`
	/// and `err` C passes last, returning whether it succeeded.
	Result { ok: Option<&'a Type>, err: &'a Type },
}

/// A function the binding implements, which Rust calls through C: a closure's trampoline,
/// or a trait's method.
pub(crate) struct Foreign<'a> {
	/// Each value it takes, after its data, by its C name and type.
	pub(crate) params: Vec<(String, Ty, Lent<'a>)>,
	pub(crate) ret: Return<'a>,
	/// Its C parameters after its data, the outs of a `Result` last, and its C return.
	pub(crate) c_params: Vec<(String, Ty)>,
	pub(crate) c_ret: Ty,
}

/// A closure the binding implements, as Rust calls it.
pub(crate) fn closure<'a>(abi: &Abi<'_>, closure: &'a Closure) -> Foreign<'a> {
	foreign(abi, closure.params.iter(), &closure.ok, closure.err.as_ref())
}

/// A trait's method the binding implements, as Rust calls it.
pub(crate) fn method<'a>(abi: &Abi<'_>, method: &'a Function) -> Foreign<'a> {
	foreign(abi, method.params.iter().map(|p| &p.ty), &method.ok, method.err.as_ref())
}

fn foreign<'a>(
	abi: &Abi<'_>,
	params: impl Iterator<Item = &'a Type>,
	ok: &'a Type,
	err: Option<&'a Type>,
) -> Foreign<'a> {
	let params: Vec<(String, Ty, Lent)> = (params.enumerate())
		.map(|(k, ty)| {
			let lent = match ty {
				Type::Ref(_, object) => match **object {
					Type::Object(i) => Lent::Object(i),
					_ => Lent::Value(ty),
				},
				ty => Lent::Value(ty),
			};
			(format!("p{k}"), abi.ty(ty), lent)
		})
		.collect();
	let mut c_params: Vec<(String, Ty)> = params.iter().map(|(n, ty, _)| (n.clone(), ty.clone())).collect();
	let (ret, c_ret) = match err {
		None => match ok {
			Type::Unit => (Return::Unit, Ty::Void),
			ok => (Return::Value(ok), abi.ty(ok)),
		},
		Some(err) => {
			let ok = (*ok != Type::Unit).then_some(ok);
			if let Some(ok) = ok {
				c_params.push(("o_".to_string(), Ty::Out(Box::new(abi.ty(ok)))));
			}
			c_params.push(("e_".to_string(), Ty::Out(Box::new(abi.ty(err)))));
			(Return::Result { ok, err }, Ty::Bool)
		}
	};
	Foreign {
		params,
		ret,
		c_params,
		c_ret,
	}
}
