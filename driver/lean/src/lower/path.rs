//! moq-net's `Path`, modeled as the string it holds, since its inline representation reads
//! bytes back as UTF-8 through `unsafe`. Its `PartialEq`, `Ord`, and `Borrow<str>` read
//! that string, so a `Path` compares, orders, and is looked up as a Lean `String`, whose
//! order is Rust's byte order on UTF-8. The methods that read the representation are written
//! here as Lean; every other method translates from its MIR through them. `lean-moq` checks
//! the model against moq-net's own `Path`.
//!
//! Replacing `Path`'s inline representation with a safe one, such as an `Arc<str>`, would let
//! it translate like any other type and retire this model.

use super::*;

/// The methods of `Path` that read its representation, which the model writes.
const MODELED: [&str; 3] = ["as_str", "empty", "head"];

impl<'tcx> Cx<'tcx> {
	/// Whether `did` is moq-net's `path::Path`, matched by where moq-net defines it.
	pub(super) fn moq_path(&self, did: DefId) -> bool {
		let tcx = self.tcx;
		if tcx.crate_name(did.krate).as_str() != "moq_net" || !matches!(tcx.def_kind(did), DefKind::Struct) {
			return false;
		}
		let names: Vec<String> = tcx
			.def_path(did)
			.data
			.iter()
			.filter_map(|d| d.data.get_opt_name())
			.map(|n| n.to_string())
			.collect();
		names == ["path", "Path"]
	}

	/// Whether `ty` is moq-net's `Path`.
	pub(super) fn moq_path_ty(&self, ty: Ty<'tcx>) -> bool {
		matches!(ty.kind(), ty::Adt(def, _) if self.moq_path(def.did()))
	}

	/// Which of `MODELED` the function `did` is, if it is one.
	pub(super) fn path_model(&self, did: DefId) -> Option<&'static str> {
		let tcx = self.tcx;
		if tcx.trait_impl_of_assoc(did).is_some() {
			return None;
		}
		let imp = tcx.impl_of_assoc(did)?;
		if !self.moq_path_ty(tcx.type_of(imp).instantiate_identity().skip_norm_wip()) {
			return None;
		}
		let name = tcx.item_name(did);
		MODELED.into_iter().find(|m| *m == name.as_str())
	}

	/// The inherent method of `Path` named `name`, beside its method `sibling`.
	fn path_method(&self, sibling: DefId, name: &str) -> Option<DefId> {
		let tcx = self.tcx;
		let ty::Adt(def, _) = tcx
			.type_of(tcx.impl_of_assoc(sibling)?)
			.instantiate_identity()
			.skip_norm_wip()
			.kind()
		else {
			return None;
		};
		tcx.inherent_impls(def.did()).iter().find_map(|imp| {
			tcx.associated_items(*imp)
				.filter_by_name_unhygienic(rustc_span::Symbol::intern(name))
				.next()
				.map(|i| i.def_id)
		})
	}

	/// The translated functions the model of `did` calls: `head` reads `head_len`.
	pub(super) fn path_model_callees(&self, did: DefId, model: &str) -> Vec<DefId> {
		match model {
			"head" => self.path_method(did, "head_len").into_iter().collect(),
			_ => Vec::new(),
		}
	}

	/// The model's Lean definition of `did`, which mirrors its Rust body with the
	/// representation read as the string it holds.
	pub(super) fn path_model_text(&mut self, did: DefId, model: &str) -> R<String> {
		let tcx = self.tcx;
		let span = tcx.def_span(did);
		self.krate = did.krate;
		let name = self.fn_name(did);
		let (text, monadic) = match model {
			"as_str" => (format!("def {name} (self : String) : String :=\n  self\n"), false),
			"empty" => (format!("def {name} : String :=\n  \"\"\n"), false),
			// `head_len`'s bytes, sliced off the front. Whether the representation was a static
			// string or a copy, the bytes are the same; `copy_bounded`'s length assert holds of
			// every path, which fits `MAX_BYTES`.
			_ => {
				let Some(len) = self.path_method(did, "head_len") else {
					return refuse(
						span,
						"`Path::head`'s model reads `Path::head_len`, which this `Path` lacks",
					);
				};
				let usize = self.ty(tcx.types.usize, span)?;
				let call = format!("{} self n", self.fn_name(len));
				let call = match self.shapes.get(&len).is_none_or(|s| s.monadic || s.failed) {
					true => format!("(← {call})"),
					false => call,
				};
				(
					format!(
						"def {name} (self : String) (n : {usize}) : Rust.M (Option String) := do\n  match {call} with\n  | none => pure none\n  | some e => pure (some (← Rust.strSliceTo self e))\n"
					),
					true,
				)
			}
		};
		self.shapes
			.get_mut(&did)
			.expect("ordered functions have shapes")
			.monadic = monadic;
		Ok(format!(
			"/-- `{}`, modeled on the string a `Path` holds -/\n{text}",
			self.rust_path(did)
		))
	}
}
