
// --- Appended by rust2's driver/lean/compare/moq.sh, which renamed each call above of the
// route algebra to its `rec_` method here, or dropped its import for the function here. ---

/// Each algebra call the vectors made, as `lean_diff` encodes a case for `MoqDiff.lean`.
mod recorded {
	use std::cell::RefCell;

	use super::super::lean_diff::{Enc, W};

	thread_local! {
		pub static CALLS: RefCell<Vec<W>> = const { RefCell::new(Vec::new()) };
	}

	/// Records a call of `kind` with its inputs, and what it returned, then returns that.
	pub fn record<T: Enc>(kind: i128, inputs: impl FnOnce(&mut W), out: T) -> T {
		let mut w = W::default();
		w.int(kind);
		inputs(&mut w);
		w.int(1).put(&out);
		CALLS.with_borrow_mut(|calls| calls.push(w));
		out
	}
}

use recorded::record;

impl super::lean_diff::Enc for Run {
	fn enc(&self, w: &mut super::lean_diff::W) {
		w.put(self.above).put(self.epoch);
	}
}

impl Offer {
	fn rec_own(me: NodeId, origin: NodeId, upstream: bool) -> Option<Self> {
		let out = Offer::own(me, origin, upstream);
		record(1, |w| _ = w.put(me).put(origin).put(upstream), out)
	}

	fn rec_onward(&self, me: NodeId, session: u64, to: NodeId, upstream: bool) -> Option<Self> {
		let out = self.onward(me, session, to, upstream);
		record(2, |w| _ = w.put(self).put(me).put(session).put(to).put(upstream), out)
	}
}

impl Here<'_> {
	fn rec_arrive(&self, port: &Port, draining: bool, offer: &Offer) -> Option<Offer> {
		let out = self.arrive(port, draining, offer);
		let inputs = |w: &mut super::lean_diff::W| {
			w.put(self.closed).put(self.dead).put(self.draining);
			w.put(port).put(draining).put(offer);
		};
		record(3, inputs, out)
	}
}

fn order(best: &Best<Node>, current: Option<Node>) -> relay::Order<Node> {
	record(4, |w| _ = w.put(best).put(current), relay::order(best, current))
}

impl Claim<Run> {
	fn rec_over(&self, route: &Offer) -> Self {
		record(5, |w| _ = w.put(self).put(route), self.over(route))
	}

	fn rec_exported(&self, me: Id, forwards: bool, upstream: bool) -> Option<Self> {
		let out = self.exported(me, forwards, upstream);
		record(6, |w| _ = w.put(self).put(me).put(forwards).put(upstream), out)
	}

	fn rec_adopted(&self, me: Id, forwards: bool, port: &Port) -> Option<Self> {
		let out = self.adopted(me, forwards, port);
		record(7, |w| _ = w.put(self).put(me).put(forwards).put(port), out)
	}

	fn rec_lowered(&self, by: u64) -> Self {
		record(8, |w| _ = w.put(self).put(by), self.lowered(by))
	}
}

fn claim_order(claim: &Claim<Run>, route: Option<&Offer>, origin: NodeId) -> relay::ClaimOrder {
	let out = relay::claim_order(claim, route, origin);
	record(9, |w| _ = w.put(claim).put(route).put(origin), out)
}

fn precedence(item: &Run) -> relay::Precedence {
	record(10, |w| _ = w.put(item), relay::precedence(item))
}

fn floor_of(hop: Hop, far: NodeId) -> ((Id, Id), u64) {
	record(11, |w| _ = w.put(hop).put(far), relay::floor_of(hop, far))
}

/// Writes every algebra call behind the checked-in `Vectors.agda` and `ClaimVectors.agda`,
/// once the generators here are shown to write those files exactly.
#[test]
#[ignore = "run by rust2's driver/lean/compare/moq.sh"]
fn write_vector_calls() {
	use std::io::Write;

	let path = std::env::var("LEAN_VECTORS_OUT").expect("LEAN_VECTORS_OUT");
	let mut vectors = 0;
	for (file, now) in [
		("Vectors.agda", route_vectors()),
		("ClaimVectors.agda", claim_vectors()),
	] {
		let checked_in = std::fs::read_to_string(vectors_path(file)).unwrap();
		assert!(checked_in == now, "{file} is not what its generator writes");
		vectors += now.matches("\n_ = refl\n").count();
	}
	let mut out = std::io::BufWriter::new(std::fs::File::create(path).expect("create"));
	let calls = recorded::CALLS.take();
	for w in &calls {
		let line: Vec<String> = w.0.iter().map(i128::to_string).collect();
		writeln!(out, "{}", line.join(" ")).expect("write");
	}
	eprintln!("{vectors} vectors, {} algebra calls", calls.len());
}
