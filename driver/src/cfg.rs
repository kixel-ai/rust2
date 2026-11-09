//! Structured control flow from a control-flow graph, for a language with labeled
//! `break` and `continue` but no `goto`, or one whose `goto` costs what it saves.
//!
//! A reducible graph, which every function without `.await` has, becomes nested blocks,
//! loops, and `if`s, left by labeled `break` and `continue`: Ramsey's "Beyond Relooper"
//! (ICFP 2022), which places each node under its immediate dominator. An irreducible
//! graph, such as a coroutine's resume function, which enters loops in the middle,
//! becomes a loop over a `switch` on the next node, with each chain of nodes that have
//! one predecessor in one case.
//!
//! Each translator structures its own statements, which [`Syntax`] builds and shows.

/// How a node leaves.
pub enum Exit<C> {
	/// To one successor.
	Goto(usize),
	/// To the first target whose condition holds, else to the last.
	Branch(Vec<(C, usize)>, usize),
	/// Nowhere: the node returned, or can't finish.
	End,
}

/// What emits each node.
pub trait Lower<S: Syntax> {
	/// Emit `node`'s statements into `out`, and say how it leaves. A branch is decided
	/// from what `out` already holds.
	fn node(&mut self, node: usize, out: &mut Vec<S>) -> Exit<S::Cond>;
	/// Control is about to leave the statements emitted so far for code emitted
	/// elsewhere: emit whatever the node lowering held back.
	fn flush(&mut self, out: &mut Vec<S>);
}

/// A control-flow graph, entered at node 0.
pub struct Graph {
	pub succs: Vec<Vec<usize>>,
}

/// A translator's statement: the control flow structuring builds, and the shape of each
/// statement its passes rewrite.
pub trait Syntax: Sized {
	/// What an `if` tests.
	type Cond: Cond;
	/// `label: { body }`, which `break label` leaves.
	fn block(label: String, body: Vec<Self>) -> Self;
	/// `label: while (true) { body }`.
	fn forever(label: String, body: Vec<Self>) -> Self;
	/// `if (cond) { then } else { els }`.
	fn branch(cond: Self::Cond, then: Vec<Self>, els: Vec<Self>) -> Self;
	/// `break label` or `continue label`.
	fn jump(jump: Jump, label: String) -> Self;
	/// `let state = 0; label: while (true) switch (state) { case n: body }`.
	fn dispatch(state: &str, label: &str, cases: Vec<(usize, Vec<Self>)>, out: &mut Vec<Self>);
	/// `state = node`, which the next turn of [`Syntax::dispatch`]'s loop goes to.
	fn set(state: &str, node: usize) -> Self;
	/// The statement's shape, to read.
	fn view(&self) -> View<'_, Self>;
	/// The statement's shape, to rewrite in place.
	fn view_mut(&mut self) -> ViewMut<'_, Self>;
}

/// A condition of a translator's statements.
pub trait Cond {
	/// The condition that holds where this one doesn't.
	fn negate(&mut self);
	/// Whether it is the literal `true`.
	fn is_true(&self) -> bool;
	/// Whether evaluating it does nothing but read.
	fn pure(&self) -> bool;
}

/// Which way a jump goes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Jump {
	/// Leave the labeled block or loop.
	Break,
	/// Start the labeled loop's next turn.
	Continue,
}

/// A statement as structuring sees it: its bodies `B`, condition `C`, and label `L`.
pub enum Form<B, C, L> {
	/// A block, which only a labeled `break` leaves.
	Block { label: L, body: B },
	/// A loop, with its condition if it is a `while`.
	Loop { label: L, cond: Option<C>, body: B },
	/// An `if`, its `else` empty if it has none.
	If { cond: C, then: B, els: B },
	/// A `switch`'s cases' bodies and its default's, which an unlabeled `break` leaves.
	Switch(Vec<B>),
	/// A `break`, and the label it names.
	Break(L),
	/// A `continue`, and the label it names.
	Continue(L),
	/// A `return`, and whether it gives a value.
	Return(bool),
	/// Anything that falls through.
	Other,
}

/// A statement's shape, borrowed to read.
pub type View<'a, S> = Form<&'a Vec<S>, &'a <S as Syntax>::Cond, &'a Option<String>>;
/// A statement's shape, borrowed to rewrite.
pub type ViewMut<'a, S> = Form<&'a mut Vec<S>, &'a mut <S as Syntax>::Cond, &'a mut Option<String>>;

/// The graph's nodes in order, its dominators, and its shape.
struct Shape {
	/// Each node's place in reverse postorder, or `usize::MAX` if unreachable.
	rpo: Vec<usize>,
	/// The reachable nodes in reverse postorder.
	order: Vec<usize>,
	idom: Vec<usize>,
	/// Each node's dominator-tree children, in reverse postorder.
	children: Vec<Vec<usize>>,
	/// Whether each node is placed after the loop its dominator heads, which it leaves.
	exit: Vec<bool>,
	/// How many reachable predecessors reach each node by a forward edge.
	forward: Vec<usize>,
	/// Whether a back edge targets each node.
	header: Vec<bool>,
	/// Whether every back edge targets a node that dominates its source.
	reducible: bool,
}

impl Shape {
	fn new(g: &Graph) -> Self {
		let n = g.succs.len();
		// Postorder, iteratively.
		let mut post = Vec::with_capacity(n);
		let mut seen = vec![false; n];
		let mut stack = vec![(0usize, 0usize)];
		seen[0] = true;
		while let Some((node, i)) = stack.pop() {
			match g.succs[node].get(i) {
				Some(&next) => {
					stack.push((node, i + 1));
					if !seen[next] {
						seen[next] = true;
						stack.push((next, 0));
					}
				}
				None => post.push(node),
			}
		}
		let order: Vec<usize> = post.into_iter().rev().collect();
		let mut rpo = vec![usize::MAX; n];
		for (i, &node) in order.iter().enumerate() {
			rpo[node] = i;
		}
		let mut preds = vec![Vec::new(); n];
		for &node in &order {
			for &next in &g.succs[node] {
				preds[next].push(node);
			}
		}
		// Cooper, Harvey, and Kennedy's iterative dominators.
		let mut idom = vec![usize::MAX; n];
		idom[0] = 0;
		let intersect = |idom: &[usize], mut a: usize, mut b: usize| {
			while a != b {
				while rpo[a] > rpo[b] {
					a = idom[a];
				}
				while rpo[b] > rpo[a] {
					b = idom[b];
				}
			}
			a
		};
		let mut changed = true;
		while changed {
			changed = false;
			for &node in order.iter().skip(1) {
				let mut new = usize::MAX;
				for &p in &preds[node] {
					if idom[p] == usize::MAX {
						continue;
					}
					new = match new {
						usize::MAX => p,
						cur => intersect(&idom, p, cur),
					};
				}
				if new != idom[node] {
					idom[node] = new;
					changed = true;
				}
			}
		}
		let dominates = |a: usize, mut b: usize| loop {
			if a == b {
				return true;
			}
			if b == 0 {
				return false;
			}
			b = idom[b];
		};
		let mut forward = vec![0; n];
		let mut header = vec![false; n];
		let mut reducible = true;
		for &node in &order {
			for &next in &g.succs[node] {
				if rpo[next] > rpo[node] {
					forward[next] += 1;
				} else {
					header[next] = true;
					reducible &= dominates(next, node);
				}
			}
		}
		// Each loop's nodes: what reaches a back edge's source without passing its header.
		// Only a reducible graph is placed as a tree.
		let mut body: Vec<Option<Vec<bool>>> = vec![None; n];
		for &node in order.iter().filter(|_| reducible) {
			for &next in &g.succs[node] {
				if rpo[next] > rpo[node] {
					continue;
				}
				let nodes = body[next].get_or_insert_with(|| {
					let mut nodes = vec![false; n];
					nodes[next] = true;
					nodes
				});
				let mut stack = vec![node];
				while let Some(m) = stack.pop() {
					if !std::mem::replace(&mut nodes[m], true) {
						stack.extend(&preds[m]);
					}
				}
			}
		}
		// The loop whose test each node is part of: its header, and the nodes after it
		// that one way in and one way on lead to, up to its first branch.
		let mut test = vec![usize::MAX; n];
		for &h in &order {
			let Some(b) = &body[h] else {
				continue;
			};
			let mut node = h;
			loop {
				test[node] = h;
				match g.succs[node].as_slice() {
					&[next] if b[next] && !header[next] && preds[next].len() == 1 => node = next,
					_ => break,
				}
			}
		}
		// Where a loop's test leaves it, what follows is placed after the loop, which a
		// `break` leaves, rather than in the branch. Other ways out, often a `return` from
		// within, stay where they leave.
		let mut children = vec![Vec::new(); n];
		let mut exit = vec![false; n];
		for &node in order.iter().skip(1) {
			let d = idom[node];
			match test[d] {
				h if h != usize::MAX && !body[h].as_ref().is_some_and(|b| b[node]) => {
					children[h].push(node);
					exit[node] = true;
				}
				_ => children[d].push(node),
			}
		}
		Self {
			rpo,
			order,
			idom,
			children,
			exit,
			forward,
			header,
			reducible,
		}
	}

	fn merge(&self, node: usize) -> bool {
		self.forward[node] > 1
	}
}

/// The structured statements of `g`, each node emitted by `lower`.
pub fn structure<S: Syntax>(g: &Graph, lower: &mut impl Lower<S>, labels: &mut u32) -> Vec<S> {
	let shape = Shape::new(g);
	let mut out = Vec::new();
	match shape.reducible {
		true => {
			let mut s = Ramsey {
				shape: &shape,
				lower,
				labels: vec![None; g.succs.len()],
				next: labels,
				stmt: std::marker::PhantomData,
			};
			s.tree(0, &mut out);
		}
		false => dispatch(g, &shape, lower, labels, &mut out),
	}
	// Dropping a jump can leave a branch empty, which the next pass inverts.
	simplify(&mut out);
	simplify(&mut out);
	out
}

struct Ramsey<'s, S, L> {
	shape: &'s Shape,
	lower: &'s mut L,
	/// The label of each node's loop or of the block it follows.
	labels: Vec<Option<String>>,
	next: &'s mut u32,
	stmt: std::marker::PhantomData<S>,
}

impl<S: Syntax, L: Lower<S>> Ramsey<'_, S, L> {
	fn label(&mut self, node: usize, prefix: &str) -> String {
		if let Some(label) = &self.labels[node] {
			return label.clone();
		}
		let label = format!("{prefix}{}", *self.next);
		*self.next += 1;
		self.labels[node] = Some(label.clone());
		label
	}

	/// `x` and what it dominates.
	fn tree(&mut self, x: usize, out: &mut Vec<S>) {
		// The nodes placed after its loop, and the merge nodes it dominates within it,
		// the last in reverse postorder outermost.
		let (mut exits, mut merges): (Vec<usize>, Vec<usize>) = self.shape.children[x]
			.iter()
			.copied()
			.filter(|&c| self.shape.exit[c] || self.shape.merge(c))
			.partition(|&c| self.shape.exit[c]);
		exits.sort_by_key(|&c| std::cmp::Reverse(self.shape.rpo[c]));
		merges.sort_by_key(|&c| std::cmp::Reverse(self.shape.rpo[c]));
		match self.shape.header[x] {
			true => self.after(x, &exits, &merges, out),
			false => self.within(x, &merges, out),
		}
	}

	/// `x`'s loop, in a block for each node placed after it.
	fn after(&mut self, x: usize, exits: &[usize], merges: &[usize], out: &mut Vec<S>) {
		if let [y, rest @ ..] = exits {
			let label = self.label(*y, "b");
			let mut body = Vec::new();
			self.after(x, rest, merges, &mut body);
			out.push(S::block(label, body));
			self.tree(*y, out);
			return;
		}
		let label = self.label(x, "l");
		let mut body = Vec::new();
		self.within(x, merges, &mut body);
		out.push(S::forever(label, body));
	}

	fn within(&mut self, x: usize, merges: &[usize], out: &mut Vec<S>) {
		if let [y, rest @ ..] = merges {
			let label = self.label(*y, "b");
			let mut body = Vec::new();
			self.within(x, rest, &mut body);
			out.push(S::block(label, body));
			self.tree(*y, out);
			return;
		}
		match self.lower.node(x, out) {
			Exit::Goto(target) => self.branch(x, target, out),
			Exit::Branch(arms, default) => {
				let mut els = Vec::new();
				self.branch(x, default, &mut els);
				for (cond, target) in arms.into_iter().rev() {
					let mut then = Vec::new();
					self.branch(x, target, &mut then);
					els = vec![S::branch(cond, then, els)];
				}
				out.extend(els);
			}
			Exit::End => self.lower.flush(out),
		}
	}

	fn branch(&mut self, from: usize, to: usize, out: &mut Vec<S>) {
		if self.shape.rpo[to] <= self.shape.rpo[from] {
			self.lower.flush(out);
			let label = self.label(to, "l");
			out.push(S::jump(Jump::Continue, label));
		} else if self.shape.merge(to) || self.shape.exit[to] {
			self.lower.flush(out);
			let label = self.label(to, "b");
			out.push(S::jump(Jump::Break, label));
		} else {
			debug_assert_eq!(self.shape.idom[to], from);
			self.tree(to, out);
		}
	}
}

/// `let n = 0; for (;;) switch (n) { ... }`: each node with other than one predecessor is a
/// case, which the chain of nodes after it with one predecessor follows in place.
fn dispatch<S: Syntax>(g: &Graph, shape: &Shape, lower: &mut impl Lower<S>, labels: &mut u32, out: &mut Vec<S>) {
	let label = format!("l{labels}");
	let var = format!("bb{labels}");
	*labels += 1;
	let n = shape.rpo.len();
	let mut preds = vec![0usize; n];
	let mut case = vec![false; n];
	case[0] = true;
	for &node in &shape.order {
		for &next in &g.succs[node] {
			preds[next] += 1;
		}
	}
	for &node in &shape.order {
		if preds[node] != 1 {
			case[node] = true;
		}
	}
	let mut d = Dispatch {
		lower,
		case: &case,
		label: &label,
		var: &var,
		stmt: std::marker::PhantomData,
	};
	let mut cases = Vec::new();
	for &node in &shape.order {
		if !case[node] {
			continue;
		}
		let mut body = Vec::new();
		d.chain(node, &mut body);
		cases.push((node, body));
	}
	S::dispatch(&var, &label, cases, out);
}

struct Dispatch<'d, S, L> {
	lower: &'d mut L,
	case: &'d [bool],
	label: &'d str,
	var: &'d str,
	stmt: std::marker::PhantomData<S>,
}

impl<S: Syntax, L: Lower<S>> Dispatch<'_, S, L> {
	fn chain(&mut self, node: usize, out: &mut Vec<S>) {
		match self.lower.node(node, out) {
			Exit::Goto(target) => self.jump(target, out),
			Exit::Branch(arms, default) => {
				let mut els = Vec::new();
				self.jump(default, &mut els);
				for (cond, target) in arms.into_iter().rev() {
					let mut then = Vec::new();
					self.jump(target, &mut then);
					els = vec![S::branch(cond, then, els)];
				}
				out.extend(els);
			}
			Exit::End => self.lower.flush(out),
		}
	}

	fn jump(&mut self, to: usize, out: &mut Vec<S>) {
		if !self.case[to] {
			self.chain(to, out);
			return;
		}
		self.lower.flush(out);
		out.push(S::set(self.var, to));
		out.push(S::jump(Jump::Continue, self.label.to_string()));
	}
}

/// Drop the jumps that only fall through, then the labels nothing targets.
pub fn simplify<S: Syntax>(stmts: &mut Vec<S>) {
	for stmt in stmts.iter_mut() {
		match stmt.view_mut() {
			Form::Block { label, body } => {
				simplify(body);
				if let Some(l) = label {
					strip_tail(body, Tail::Break(Some(l)));
				}
			}
			Form::Loop { label, body, .. } => {
				simplify(body);
				if let Some(l) = label {
					strip_tail(body, Tail::Continue(Some(l)));
				}
				strip_tail(body, Tail::Continue(None));
			}
			Form::If { then, els, .. } => {
				simplify(then);
				simplify(els);
			}
			Form::Switch(bodies) => bodies.into_iter().for_each(simplify),
			_ => {}
		}
	}
	// Unused labels, and blocks left without one.
	let mut i = 0;
	while i < stmts.len() {
		if let Form::Block { label, body } | Form::Loop { label, body, .. } = stmts[i].view_mut()
			&& label.as_ref().is_some_and(|l| !targets(body, l))
		{
			*label = None;
		}
		let body = match stmts[i].view_mut() {
			Form::Block { label: None, body } => Some(std::mem::take(body)),
			_ => None,
		};
		if let Some(body) = body {
			let len = body.len();
			stmts.splice(i..=i, body);
			i += len;
			continue;
		}
		// Nothing runs after a jump.
		if diverges_one(&stmts[i]) {
			stmts.truncate(i + 1);
			break;
		}
		i += 1;
	}
	// `if (c) {} else { x }` is `if (!c) { x }`.
	for stmt in stmts.iter_mut() {
		if let Form::If { cond, then, els } = stmt.view_mut()
			&& then.is_empty()
			&& !els.is_empty()
		{
			cond.negate();
			std::mem::swap(then, els);
		}
	}
	stmts.retain(
		|s| !matches!(s.view(), Form::If { cond, then, els } if then.is_empty() && els.is_empty() && cond.pure()),
	);
}

/// After a `then` that never falls through, the `else` is what follows the `if`. Every
/// name a body declares is its own, so nothing moved out can clash. Last, as it leaves
/// the jumps `simplify` strips no longer at the end of their block.
pub fn hoist_else<S: Syntax>(stmts: &mut Vec<S>) {
	let mut i = 0;
	while i < stmts.len() {
		let hoisted = match stmts[i].view_mut() {
			Form::If { then, els, .. } => {
				hoist_else(then);
				hoist_else(els);
				(!els.is_empty() && diverges(then)).then(|| std::mem::take(els))
			}
			Form::Block { body, .. } | Form::Loop { body, .. } => {
				hoist_else(body);
				None
			}
			Form::Switch(bodies) => {
				bodies.into_iter().for_each(hoist_else);
				None
			}
			_ => None,
		};
		if let Some(els) = hoisted {
			stmts.splice(i + 1..i + 1, els);
		}
		i += 1;
	}
}

/// Last, as the passes before match labeled jumps: a loop takes the label of a block that
/// holds only it, a loop that starts by leaving when `c` holds runs `while (!c)`, and a
/// jump to the innermost loop it is in names no label.
pub fn tidy_loops<S: Syntax>(stmts: &mut Vec<S>) {
	adopt_labels(stmts);
	simplify(stmts);
	rotate(stmts);
	unlabel(stmts, None, false);
	simplify(stmts);
}

/// `b: { l: while (..) { .. break b .. } }` is `l: while (..) { .. break l .. }`.
fn adopt_labels<S: Syntax>(stmts: &mut [S]) {
	for stmt in stmts.iter_mut() {
		let adopted = match stmt.view_mut() {
			Form::Block { label: Some(b), body } if body.len() == 1 => match body[0].view_mut() {
				Form::Loop { label, body, .. } => {
					let l = label.get_or_insert_with(|| b.clone()).clone();
					retarget(body, b, &l);
					true
				}
				_ => false,
			},
			_ => false,
		};
		if adopted && let Form::Block { body, .. } = stmt.view_mut() {
			*stmt = body.pop().expect("a block holding only a loop");
		}
		match stmt.view_mut() {
			Form::If { then, els, .. } => {
				adopt_labels(then);
				adopt_labels(els);
			}
			Form::Block { body, .. } | Form::Loop { body, .. } => adopt_labels(body),
			Form::Switch(bodies) => bodies.into_iter().for_each(|body| adopt_labels(body)),
			_ => {}
		}
	}
}

/// Point each `break from` in `stmts` at `to`.
fn retarget<S: Syntax>(stmts: &mut [S], from: &str, to: &str) {
	for stmt in stmts.iter_mut() {
		match stmt.view_mut() {
			Form::Break(Some(l)) if l == from => *l = to.to_string(),
			Form::If { then, els, .. } => {
				retarget(then, from, to);
				retarget(els, from, to);
			}
			Form::Block { body, .. } | Form::Loop { body, .. } => retarget(body, from, to),
			Form::Switch(bodies) => bodies.into_iter().for_each(|body| retarget(body, from, to)),
			_ => {}
		}
	}
}

/// `while (true) { if (c) break; .. }` is `while (!c) { .. }`, and
/// `while (true) { if (c) { a } else break; .. }` is `while (c) { a .. }`.
fn rotate<S: Syntax>(stmts: &mut [S]) {
	for stmt in stmts.iter_mut() {
		if let Form::Loop {
			label,
			cond: Some(cond),
			body,
		} = stmt.view_mut()
			&& cond.is_true()
			&& let Some(first) = body.first_mut()
			&& let Form::If { cond: test, then, els } = first.view_mut()
		{
			let leaves = |stmts: &[S]| matches!(stmts, [s] if matches!(s.view(), Form::Break(target) if target.is_none() || target == label));
			let rotated = if els.is_empty() && leaves(then) {
				std::mem::swap(cond, test);
				cond.negate();
				Some(Vec::new())
			} else if leaves(els) {
				std::mem::swap(cond, test);
				Some(std::mem::take(then))
			} else {
				None
			};
			if let Some(then) = rotated {
				body.splice(0..1, then);
			}
		}
		match stmt.view_mut() {
			Form::If { then, els, .. } => {
				rotate(then);
				rotate(els);
			}
			Form::Block { body, .. } | Form::Loop { body, .. } => rotate(body),
			Form::Switch(bodies) => bodies.into_iter().for_each(|body| rotate(body)),
			_ => {}
		}
	}
}

/// Drop the label of each jump to `innermost`, the loop `stmts` are in, which an
/// unlabeled `break` in a `switch` would leave instead.
fn unlabel<S: Syntax>(stmts: &mut [S], innermost: Option<&str>, in_switch: bool) {
	for stmt in stmts.iter_mut() {
		match stmt.view_mut() {
			Form::Continue(l) if l.is_some() && l.as_deref() == innermost => *l = None,
			Form::Break(l) if !in_switch && l.is_some() && l.as_deref() == innermost => *l = None,
			Form::If { then, els, .. } => {
				unlabel(then, innermost, in_switch);
				unlabel(els, innermost, in_switch);
			}
			Form::Block { body, .. } => unlabel(body, innermost, in_switch),
			// An unlabeled loop matches no jump's label.
			Form::Loop { label, body, .. } => unlabel(body, Some(label.as_deref().unwrap_or("")), false),
			Form::Switch(bodies) => bodies.into_iter().for_each(|body| unlabel(body, innermost, true)),
			_ => {}
		}
	}
}

/// A jump that may end a body, falling through.
#[derive(Clone, Copy)]
enum Tail<'a> {
	Break(Option<&'a str>),
	Continue(Option<&'a str>),
	Return,
}

/// Remove `jump` wherever it is the last thing `stmts` runs before falling off its end.
fn strip_tail<S: Syntax>(stmts: &mut Vec<S>, jump: Tail<'_>) {
	let Some(last) = stmts.last_mut() else {
		return;
	};
	let same = match (last.view_mut(), jump) {
		(Form::Break(x), Tail::Break(y)) | (Form::Continue(x), Tail::Continue(y)) => x.as_deref() == y,
		(Form::Return(false), Tail::Return) => true,
		(Form::If { then, els, .. }, _) => {
			strip_tail(then, jump);
			strip_tail(els, jump);
			false
		}
		(Form::Block { body, .. }, _) => {
			strip_tail(body, jump);
			false
		}
		_ => false,
	};
	if same {
		stmts.pop();
	}
}

/// Drop the `return;`s a function returning nothing ends with.
pub fn strip_return<S: Syntax>(stmts: &mut Vec<S>) {
	strip_tail(stmts, Tail::Return);
	simplify(stmts);
	simplify(stmts);
}

/// Whether a `break` or `continue` in `stmts` names `label`.
fn targets<S: Syntax>(stmts: &[S], label: &str) -> bool {
	stmts.iter().any(|stmt| match stmt.view() {
		Form::Break(Some(l)) | Form::Continue(Some(l)) => l == label,
		Form::If { then, els, .. } => targets(then, label) || targets(els, label),
		Form::Block { body, .. } | Form::Loop { body, .. } => targets(body, label),
		Form::Switch(bodies) => bodies.into_iter().any(|body| targets(body, label)),
		_ => false,
	})
}

/// Whether control never falls off the end of `stmts`.
pub fn diverges<S: Syntax>(stmts: &[S]) -> bool {
	stmts.last().is_some_and(diverges_one)
}

/// Whether control never reaches the statement after `stmt`.
fn diverges_one<S: Syntax>(stmt: &S) -> bool {
	match stmt.view() {
		Form::Return(_) | Form::Break(_) | Form::Continue(_) => true,
		Form::If { then, els, .. } => diverges(then) && diverges(els),
		Form::Block { label: None, body } => diverges(body),
		Form::Loop {
			label,
			cond: Some(cond),
			body,
		} => cond.is_true() && !breaks_out(body, label.as_deref(), false),
		_ => false,
	}
}

/// Whether a `break` in `stmts` targets the loop that holds them, labeled `label`.
/// Inside a `nested` loop, only a labeled `break` can. A `break` to any other label
/// skips the loop's next statement too.
fn breaks_out<S: Syntax>(stmts: &[S], label: Option<&str>, nested: bool) -> bool {
	stmts.iter().any(|stmt| match stmt.view() {
		Form::Break(None) => !nested,
		Form::Break(Some(target)) => Some(target.as_str()) == label,
		Form::If { then, els, .. } => breaks_out(then, label, nested) || breaks_out(els, label, nested),
		Form::Block { body, .. } => breaks_out(body, label, nested),
		// An unlabeled `break` in a case leaves the switch.
		Form::Switch(bodies) => bodies.into_iter().any(|body| breaks_out(body, label, true)),
		Form::Loop { body, .. } => breaks_out(body, label, true),
		_ => false,
	})
}
