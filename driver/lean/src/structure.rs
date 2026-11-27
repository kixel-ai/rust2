//! Nodes of straight-line statements to a Lean `do` block.
//!
//! Lean's `do` has `if`, `match`, early `return`, and `let mut`, but no loop it can
//! reason about, so each natural loop becomes its own `partial_fixpoint` function: its
//! parameters are the variables live at its header, a back edge calls it again, and each
//! exit returns the variables the loop wrote that are live past it, tagged by exit with
//! `Sum`. Every other region nests under its branch, which rejoins at the branch's
//! immediate post-dominator; a path into a panic is copied where it is taken.

use std::collections::{BTreeSet, HashMap, HashSet};

/// A Lean expression.
#[derive(Clone, Debug)]
pub struct Ex {
	pub text: String,
	/// Whether it needs no parentheses as an argument.
	pub atom: bool,
	/// Whether it runs an action: `(← ..)` inside, or is one where bound.
	pub monadic: bool,
	/// The variables it reads.
	pub reads: BTreeSet<usize>,
	/// Its form as a `Prop`, for a condition, if it decides one.
	pub prop: Option<String>,
	/// A literal's digits without its type, for beside an operand that gives it.
	pub lit: Option<String>,
}

impl Ex {
	pub fn paren(&self) -> String {
		if self.atom {
			self.text.clone()
		} else {
			format!("({})", self.text)
		}
	}

	fn cond(&self) -> String {
		self.prop.clone().unwrap_or_else(|| self.text.clone())
	}
}

/// A Lean variable.
pub struct Var {
	pub name: String,
	/// Its Lean type, empty where no declaration needs one.
	pub ty: String,
	/// Whether the source named it.
	pub user: bool,
}

/// A straight-line statement.
#[derive(Clone, Debug)]
pub enum Op {
	/// `var := value`, or `var ← value` if bound.
	Set {
		var: usize,
		value: Ex,
		bind: bool,
	},
	/// `unless cond do throw msg`, where `msg` is a Lean `String` term.
	Check {
		cond: Ex,
		msg: String,
	},
	Throw(String),
}

/// How a node leaves.
pub enum Term {
	Goto(usize),
	/// The first arm whose condition holds, else the default.
	Cases {
		arms: Vec<(Ex, usize)>,
		default: usize,
	},
	/// A match on an enum: each arm's pattern, the variables it binds, and its node.
	Match {
		scrut: Ex,
		arms: Vec<(String, Vec<usize>, usize)>,
		default: Option<usize>,
	},
	Return,
	Throw(String),
	Unreachable,
}

/// Straight-line statements and how they leave.
pub struct Node {
	pub ops: Vec<Op>,
	pub term: Term,
}

/// A function to emit.
pub struct Function {
	pub name: String,
	pub binders: String,
	pub params: Vec<(String, String)>,
	pub ret: String,
	pub ret_value: Ex,
	pub vars: Vec<Var>,
	pub nodes: Vec<Node>,
}

/// A structured statement.
#[derive(Clone, Debug)]
enum Stmt {
	Op(Op),
	If {
		cond: Ex,
		then: Vec<Stmt>,
		els: Vec<Stmt>,
	},
	Match {
		scrut: Ex,
		arms: Vec<(String, Vec<usize>, Vec<Stmt>)>,
	},
	Return(Ex),
	/// `let [mut] var := value`, or with `default` if none.
	Let {
		var: usize,
		mutable: bool,
		value: Option<Ex>,
		bind: bool,
	},
}

const KEYWORDS: &[&str] = &[
	"at",
	"by",
	"do",
	"else",
	"end",
	"fun",
	"from",
	"have",
	"if",
	"in",
	"let",
	"match",
	"mut",
	"open",
	"return",
	"show",
	"then",
	"with",
	"where",
	"for",
	"unless",
	"break",
	"continue",
	"try",
	"catch",
	"finally",
	"def",
	"theorem",
	"structure",
	"inductive",
	"instance",
	"class",
	"namespace",
	"section",
	"import",
	"variable",
	"deriving",
	"Type",
	"Prop",
	"Sort",
	"fun",
	"λ",
	"nomatch",
	"nofun",
	"calc",
	"suffices",
	"obtain",
	"example",
	"abbrev",
	"axiom",
	"partial",
	"private",
	"protected",
	"public",
	"noncomputable",
	"universe",
	"macro",
	"syntax",
	"local",
	"this",
];

/// A Rust name as a Lean identifier.
pub fn ident(name: &str) -> String {
	let name = name.strip_prefix("r#").unwrap_or(name);
	if KEYWORDS.contains(&name) {
		format!("«{name}»")
	} else {
		name.to_string()
	}
}

/// The Lean definitions of `f`, and whether it runs in `Rust.M`.
pub fn emit(mut f: Function) -> Result<(String, bool), String> {
	clean(&mut f);
	let f = &f;
	let mut s = Structurer::new(f)?;
	let main = s.region(0, None, None)?;
	let monadic = !s.loops.is_empty() || main.iter().any(effectful);
	// Loop functions, inner first.
	let mut order: Vec<usize> = (0..s.loops.len()).collect();
	order.sort_by_key(|&l| s.loops[l].body.len());
	let mut loop_bodies = Vec::new();
	for &l in &order {
		loop_bodies.push(s.region(s.loops[l].header, None, Some(l))?);
	}
	let names: Vec<String> = f.vars.iter().chain(&s.extra).map(|v| v.name.clone()).collect();
	let tys: Vec<String> = f.vars.iter().chain(&s.extra).map(|v| v.ty.clone()).collect();
	let p = Printer {
		names: &names,
		tys: &tys,
		in_loop: std::cell::Cell::new(true),
	};
	let mut out = String::new();
	for (&l, mut body) in order.iter().zip(loop_bodies) {
		fold_returns(&mut body, f);
		let params = s.loops[l].params.clone();
		p.declare(&mut body, &params)?;
		let sig = params
			.iter()
			.map(|&v| format!("({} : {})", names[v], tys[v]))
			.collect::<Vec<_>>()
			.join(" ");
		let ret = s.loop_ret(l);
		if std::env::var_os("RUST2_LEAN_DEBUG").is_some() {
			eprintln!("== {}\n{}", s.loop_name(l), p.block(&body, 1));
		}
		out += &format!(
			"def {}{} {sig} : Rust.M {ret} := do\n{}partial_fixpoint\n\n",
			s.loop_name(l),
			f.binders,
			p.block(&body, 1)
		);
	}
	p.in_loop.set(false);
	let mut body = main;
	fold_returns(&mut body, f);
	if std::env::var_os("RUST2_LEAN_DEBUG").is_some() {
		eprintln!("== {}\n{}", f.name, p.block(&body, 1));
	}
	let params: Vec<usize> = f
		.params
		.iter()
		.map(|(name, _)| {
			f.vars
				.iter()
				.position(|v| &v.name == name)
				.expect("parameters have variables")
		})
		.collect();
	p.declare(&mut body, &params)?;
	let sig = f
		.params
		.iter()
		.map(|(n, t)| format!("({n} : {t})"))
		.collect::<Vec<_>>()
		.join(" ");
	let sig = if sig.is_empty() {
		String::new()
	} else {
		format!(" {sig}")
	};
	if monadic {
		out += &format!(
			"def {}{}{sig} : Rust.M {} := do\n{}",
			f.name,
			f.binders,
			f.ret,
			p.block(&body, 1)
		);
	} else if let [Stmt::Return(e)] = body.as_slice() {
		out += &format!("def {}{}{sig} : {} :=\n  {}\n", f.name, f.binders, f.ret, e.text);
	} else {
		out += &format!(
			"def {}{}{sig} : {} := Id.run do\n{}",
			f.name,
			f.binders,
			f.ret,
			p.block(&body, 1)
		);
	}
	Ok((out, monadic))
}

struct Loop {
	header: usize,
	body: HashSet<usize>,
	parent: Option<usize>,
	/// The nodes outside it its body jumps to, but for panics.
	exits: Vec<usize>,
	params: Vec<usize>,
	/// For each exit, the variables it returns.
	outs: Vec<Vec<usize>>,
	/// Whether its body returns from the function, as one more exit after the others.
	returns: bool,
}

struct Structurer<'f> {
	f: &'f Function,
	succs: Vec<Vec<usize>>,
	terminal: Vec<bool>,
	/// Nodes outside every loop whose every path returns or panics, which a loop's exit
	/// to one copies into the loop as a return from the function.
	tail: Vec<bool>,
	/// Nodes that only return, copied into each branch that reaches one.
	ret_only: Vec<bool>,
	loops: Vec<Loop>,
	/// The innermost loop of each node.
	innermost: Vec<Option<usize>>,
	live_in: Vec<BTreeSet<usize>>,
	/// Each region's immediate post-dominators, by loop (`None` for the function).
	ipdom: HashMap<Option<usize>, HashMap<usize, Option<usize>>>,
	/// Variables the next fresh one is numbered after.
	fresh: usize,
	extra: Vec<Var>,
}

impl<'f> Structurer<'f> {
	fn new(f: &'f Function) -> Result<Self, String> {
		let n = f.nodes.len();
		let succs: Vec<Vec<usize>> = f.nodes.iter().map(|node| term_succs(&node.term)).collect();
		// Nodes that only panic, which least-fixpoint excludes loops from.
		let mut terminal = vec![false; n];
		loop {
			let mut changed = false;
			for i in 0..n {
				if terminal[i] {
					continue;
				}
				let t = match &f.nodes[i].term {
					Term::Throw(_) | Term::Unreachable => true,
					_ => !succs[i].is_empty() && succs[i].iter().all(|&s| terminal[s]),
				};
				if t {
					terminal[i] = true;
					changed = true;
				}
			}
			if !changed {
				break;
			}
		}
		let idom = dominators(n, 0, &succs);
		let dominates = |a: usize, mut b: usize| loop {
			if a == b {
				return true;
			}
			if b == 0 || idom[b] == usize::MAX {
				return false;
			}
			b = idom[b];
		};
		// Natural loops, by header; a retreating edge to a node that does not dominate its
		// source makes the graph irreducible.
		let order = postorder(n, 0, &succs);
		let mut rpo = vec![usize::MAX; n];
		for (i, &node) in order.iter().rev().enumerate() {
			rpo[node] = i;
		}
		let mut headers: HashMap<usize, HashSet<usize>> = HashMap::new();
		for u in 0..n {
			if rpo[u] == usize::MAX {
				continue;
			}
			for &h in &succs[u] {
				if rpo[h] <= rpo[u] {
					if !dominates(h, u) {
						return Err("irreducible control flow is not supported".into());
					}
					let body = headers.entry(h).or_insert_with(|| HashSet::from([h]));
					let mut work = vec![u];
					while let Some(x) = work.pop() {
						if body.insert(x) {
							for p in (0..n).filter(|&p| succs[p].contains(&x)) {
								work.push(p);
							}
						}
					}
				}
			}
		}
		let mut loops: Vec<Loop> = headers
			.into_iter()
			.map(|(header, body)| Loop {
				header,
				body,
				parent: None,
				exits: Vec::new(),
				params: Vec::new(),
				outs: Vec::new(),
				returns: false,
			})
			.collect();
		loops.sort_by_key(|l| (l.body.len(), l.header));
		for i in 0..loops.len() {
			loops[i].parent = (i + 1..loops.len()).find(|&j| loops[j].body.contains(&loops[i].header));
		}
		let innermost: Vec<_> = (0..n)
			.map(|node| (0..loops.len()).find(|&l| loops[l].body.contains(&node)))
			.collect();
		let live_in = liveness(f, &succs);
		let mut tail = vec![false; n];
		loop {
			let mut changed = false;
			for i in 0..n {
				if tail[i] || terminal[i] || innermost[i].is_some() {
					continue;
				}
				let t = match &f.nodes[i].term {
					Term::Return => true,
					_ => !succs[i].is_empty() && succs[i].iter().all(|&x| tail[x] || terminal[x]),
				};
				if t {
					tail[i] = true;
					changed = true;
				}
			}
			if !changed {
				break;
			}
		}
		let mut ret_only: Vec<bool> = f
			.nodes
			.iter()
			.map(|node| node.ops.is_empty() && matches!(node.term, Term::Return))
			.collect();
		loop {
			let mut changed = false;
			for i in 0..n {
				if !ret_only[i] && f.nodes[i].ops.is_empty() && matches!(f.nodes[i].term, Term::Goto(t) if ret_only[t])
				{
					ret_only[i] = true;
					changed = true;
				}
			}
			if !changed {
				break;
			}
		}
		let mut s = Structurer {
			f,
			succs,
			terminal,
			tail,
			ret_only,
			loops,
			innermost,
			live_in,
			ipdom: HashMap::new(),
			fresh: f.vars.len(),
			extra: Vec::new(),
		};
		for l in 0..s.loops.len() {
			let mut exits: Vec<usize> = s.loops[l]
				.body
				.iter()
				.flat_map(|&n| s.succs[n].iter().copied())
				.filter(|t| !s.loops[l].body.contains(t) && !s.terminal[*t] && !s.tail[*t])
				.collect();
			let returns = s.loops[l]
				.body
				.iter()
				.flat_map(|&n| s.succs[n].iter().copied())
				.any(|t| !s.loops[l].body.contains(&t) && s.tail[t]);
			s.loops[l].returns = returns;
			exits.sort();
			exits.dedup();
			let written: BTreeSet<usize> = s.loops[l].body.iter().flat_map(|&n| writes(&f.nodes[n])).collect();
			let outs = exits
				.iter()
				.map(|&t| s.live_in[t].intersection(&written).copied().collect())
				.collect();
			let params: Vec<usize> = s.live_in[s.loops[l].header].iter().copied().collect();
			for &v in &params {
				if f.vars[v].ty.is_empty() {
					return Err(format!("loop variable `{}` has no type", f.vars[v].name));
				}
			}
			s.loops[l].exits = exits;
			s.loops[l].outs = outs;
			s.loops[l].params = params;
		}
		for ctx in std::iter::once(None).chain((0..s.loops.len()).map(Some)) {
			let ip = s.postdominators(ctx)?;
			s.ipdom.insert(ctx, ip);
		}
		Ok(s)
	}

	fn in_body(&self, ctx: Option<usize>, node: usize) -> bool {
		ctx.is_none_or(|l| self.loops[l].body.contains(&node))
	}

	/// The child loop `node` heads within `ctx`, if any.
	fn child(&self, ctx: Option<usize>, node: usize) -> Option<usize> {
		let l = self.innermost[node]?;
		(self.loops[l].header == node && Some(l) != ctx).then_some(l)
	}

	/// The region successors of `node` in `ctx`: `None` leaves the region.
	fn rsuccs(&self, ctx: Option<usize>, node: usize) -> Vec<Option<usize>> {
		let targets: Vec<usize> = match self.child(ctx, node) {
			Some(k) => self.loops[k].exits.clone(),
			None => self.succs[node]
				.iter()
				.copied()
				.filter(|&t| !self.terminal[t])
				.collect(),
		};
		let mut out: Vec<Option<usize>> = targets
			.into_iter()
			.map(|t| match ctx {
				Some(l) if t == self.loops[l].header || !self.loops[l].body.contains(&t) => None,
				_ => Some(t),
			})
			.collect();
		if out.is_empty() {
			out.push(None);
		}
		out
	}

	/// The immediate post-dominator of each node in `ctx`'s region, `None` where only
	/// leaving the region rejoins its paths.
	fn postdominators(&self, ctx: Option<usize>) -> Result<HashMap<usize, Option<usize>>, String> {
		let n = self.f.nodes.len();
		let members: Vec<usize> = (0..n)
			.filter(|&x| !self.terminal[x] && self.in_body(ctx, x))
			.filter(|&x| self.innermost[x] == ctx || self.child(ctx, x).is_some_and(|k| self.loops[k].parent == ctx))
			.collect();
		// Reverse graph over members plus an exit at index `n`.
		let exit = n;
		let mut rev: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
		for &m in &members {
			for t in self.rsuccs(ctx, m) {
				match t {
					Some(t) => {
						if !members.contains(&t) {
							return Err("a jump into the middle of a loop is not supported".into());
						}
						rev[t].push(m);
					}
					None => rev[exit].push(m),
				}
			}
		}
		let idom = dominators(n + 1, exit, &rev);
		Ok(members
			.iter()
			.map(|&m| {
				let d = idom[m];
				(m, (d != exit && d != usize::MAX).then_some(d))
			})
			.collect())
	}

	fn new_var(&mut self, name: &str) -> usize {
		let mut unique = name.to_string();
		let mut k = 1;
		while self.f.vars.iter().chain(&self.extra).any(|v| v.name == unique) {
			unique = format!("{name}{k}");
			k += 1;
		}
		self.extra.push(Var {
			name: unique,
			ty: String::new(),
			user: false,
		});
		self.fresh += 1;
		self.fresh - 1
	}

	fn var_name(&self, v: usize) -> &str {
		if v < self.f.vars.len() {
			&self.f.vars[v].name
		} else {
			&self.extra[v - self.f.vars.len()].name
		}
	}

	fn loop_name(&self, l: usize) -> String {
		let mut order: Vec<usize> = (0..self.loops.len()).collect();
		order.sort_by_key(|&k| self.loops[k].header);
		let i = order.iter().position(|&k| k == l).expect("listed");
		if self.loops.len() == 1 {
			format!("{}_loop", self.f.name)
		} else {
			format!("{}_loop{i}", self.f.name)
		}
	}

	fn loop_ret(&self, l: usize) -> String {
		let mut parts: Vec<String> = self.loops[l]
			.outs
			.iter()
			.map(|outs| match outs.as_slice() {
				[] => "Unit".into(),
				[v] => self.f.vars[*v].ty.clone(),
				vs => format!(
					"({})",
					vs.iter()
						.map(|&v| self.f.vars[v].ty.clone())
						.collect::<Vec<_>>()
						.join(" × ")
				),
			})
			.collect();
		if self.loops[l].returns {
			parts.push(self.f.ret.clone());
		}
		match parts.as_slice() {
			[] => "Empty".into(),
			[one] => one.clone(),
			_ => {
				let mut out = parts.last().expect("several").clone();
				for p in parts[..parts.len() - 1].iter().rev() {
					out = format!("({p} ⊕ {out})");
				}
				out
			}
		}
	}

	fn tuple(&self, vars: &[usize]) -> String {
		match vars {
			[] => "()".into(),
			[v] => self.var_name(*v).to_string(),
			vs => format!(
				"({})",
				vs.iter().map(|&v| self.var_name(v)).collect::<Vec<_>>().join(", ")
			),
		}
	}

	/// The tag of exit `i` of `k`, around `payload`.
	fn tag(i: usize, k: usize, payload: &str) -> String {
		if k == 1 {
			return payload.to_string();
		}
		let mut out = if i == k - 1 {
			payload.to_string()
		} else {
			format!(".inl {payload}")
		};
		for _ in 0..i {
			out = format!(".inr ({out})");
		}
		out
	}

	fn call_loop(&self, l: usize) -> Ex {
		let args: Vec<&str> = self.loops[l].params.iter().map(|&v| self.var_name(v)).collect();
		let text = if args.is_empty() {
			self.loop_name(l)
		} else {
			format!("{} {}", self.loop_name(l), args.join(" "))
		};
		Ex {
			text,
			atom: false,
			monadic: true,
			reads: self.loops[l].params.iter().copied().collect(),
			prop: None,
			lit: None,
		}
	}

	/// How many tagged exits loop `l` has, its return included.
	fn tags(&self, l: usize) -> usize {
		self.loops[l].exits.len() + usize::from(self.loops[l].returns)
	}

	/// Return `value` from the function, through loop `ctx`'s return exit if in one.
	fn ret(&self, ctx: Option<usize>, value: Ex) -> Stmt {
		match ctx {
			None => Stmt::Return(value),
			Some(l) => {
				let k = self.tags(l);
				Stmt::Return(Ex {
					text: Self::tag(k - 1, k, &value.paren()),
					atom: false,
					..value
				})
			}
		}
	}

	/// Statements for leaving along an edge to `t` that the walk does not continue.
	fn jump(&mut self, t: usize, ctx: Option<usize>) -> Result<Option<Vec<Stmt>>, String> {
		if self.terminal[t] {
			return Ok(Some(self.region(t, None, None)?));
		}
		if self.ret_only[t] {
			return Ok(Some(self.region(t, None, ctx)?));
		}
		if let Some(l) = ctx
			&& !self.loops[l].body.contains(&t)
			&& self.tail[t]
		{
			return Ok(Some(self.region(t, None, ctx)?));
		}
		if let Some(l) = ctx {
			if t == self.loops[l].header {
				let call = self.call_loop(l);
				return Ok(Some(vec![Stmt::Return(Ex {
					text: format!("(← {})", call.text),
					atom: true,
					..call
				})]));
			}
			if !self.loops[l].body.contains(&t) {
				let i = self.loops[l]
					.exits
					.iter()
					.position(|&e| e == t)
					.expect("exits are listed");
				let outs = self.loops[l].outs[i].clone();
				let text = Self::tag(i, self.tags(l), &self.tuple(&outs));
				return Ok(Some(vec![Stmt::Return(Ex {
					text,
					atom: false,
					monadic: false,
					reads: outs.into_iter().collect(),
					prop: None,
					lit: None,
				})]));
			}
		}
		Ok(None)
	}

	fn edge(&mut self, t: usize, follow: Option<usize>, ctx: Option<usize>) -> Result<Vec<Stmt>, String> {
		if Some(t) == follow {
			return Ok(Vec::new());
		}
		if let Some(stmts) = self.jump(t, ctx)? {
			return Ok(stmts);
		}
		self.region(t, follow, ctx)
	}

	/// The statements from `start` until `follow`, in loop `ctx`'s region.
	fn region(&mut self, start: usize, follow: Option<usize>, ctx: Option<usize>) -> Result<Vec<Stmt>, String> {
		let mut out = Vec::new();
		let mut cur = start;
		loop {
			if Some(cur) == follow {
				return Ok(out);
			}
			let rctx = if self.terminal[cur] { None } else { ctx };
			if let Some(k) = self.child(rctx, cur).filter(|_| !self.terminal[cur]) {
				let r = self.new_var("r");
				out.push(Stmt::Op(Op::Set {
					var: r,
					value: self.call_loop(k),
					bind: true,
				}));
				let exits = self.loops[k].exits.clone();
				let outs = self.loops[k].outs.clone();
				let returns = self.loops[k].returns;
				let ipd = self.ipdom[&rctx].get(&cur).copied().flatten();
				if exits.len() == 1 && !returns {
					self.unpack(&mut out, r, &outs[0]);
					let t = exits[0];
					if Some(t) == follow {
						return Ok(out);
					}
					if let Some(stmts) = self.jump(t, rctx)? {
						out.extend(stmts);
						return Ok(out);
					}
					cur = t;
					continue;
				}
				let total = self.tags(k);
				let mut scrut = var(r, self.var_name(r));
				scrut.reads.insert(r);
				if exits.is_empty() {
					// A loop that neither exits nor returns yields `Empty`, which is any type.
					if !returns {
						scrut.text = format!("(Empty.elim {})", scrut.text);
					}
					out.push(self.ret(rctx, scrut));
					return Ok(out);
				}
				let mut arms = Vec::new();
				for (i, &t) in exits.iter().enumerate() {
					let p = self.new_var("p");
					let pat = Self::tag(i, total, self.var_name(p));
					let mut stmts = Vec::new();
					self.unpack(&mut stmts, p, &outs[i]);
					stmts.extend(self.edge(t, ipd, rctx)?);
					arms.push((pat, vec![p], stmts));
				}
				if returns {
					let p = self.new_var("p");
					let pat = Self::tag(total - 1, total, self.var_name(p));
					let mut value = var(p, self.var_name(p));
					value.reads.insert(p);
					arms.push((pat, vec![p], vec![self.ret(rctx, value)]));
				}
				out.push(Stmt::Match { scrut, arms });
				match ipd {
					Some(next) => cur = next,
					None => return Ok(out),
				}
				continue;
			}
			let node = &self.f.nodes[cur];
			out.extend(node.ops.iter().cloned().map(Stmt::Op));
			match &node.term {
				Term::Return => {
					let ret = &self.f.ret_value;
					let ret_var = ret.reads.iter().next().copied();
					// `ret := e; return ret` is `return e`.
					let mut reads = ret.reads.clone();
					let text = match (out.last(), ret_var) {
						(Some(Stmt::Op(Op::Set { var, value, bind })), Some(rv))
							if *var == rv && self.f.vars[rv].name == "ret" && !value.reads.contains(&rv) =>
						{
							let value = if *bind {
								format!("(← {})", value.text)
							} else {
								value.text.clone()
							};
							let text = if ret.text == self.f.vars[rv].name {
								value
							} else {
								ret.text
									.replacen(&format!("({}, ", self.f.vars[rv].name), &format!("({value}, "), 1)
							};
							reads.remove(&rv);
							reads.extend(value_reads(out.last()));
							out.pop();
							text
						}
						_ => ret.text.clone(),
					};
					let monadic = text.contains('←');
					let value = Ex {
						text,
						atom: false,
						monadic,
						reads,
						prop: None,
						lit: None,
					};
					out.push(self.ret(rctx, value));
					return Ok(out);
				}
				Term::Throw(msg) => {
					out.push(Stmt::Op(Op::Throw(msg.clone())));
					return Ok(out);
				}
				Term::Unreachable => {
					out.push(Stmt::Op(Op::Throw("\"entered unreachable code\"".into())));
					return Ok(out);
				}
				Term::Goto(t) => {
					let t = *t;
					if Some(t) == follow {
						return Ok(out);
					}
					if let Some(stmts) = self.jump(t, rctx)? {
						out.extend(stmts);
						return Ok(out);
					}
					cur = t;
				}
				Term::Cases { arms, default } => {
					let arms = arms.clone();
					let default = *default;
					let ipd = self.branch_follow(rctx, cur);
					let mut els = self.edge(default, ipd, rctx)?;
					for (cond, t) in arms.into_iter().rev() {
						let then = self.edge(t, ipd, rctx)?;
						els = vec![Stmt::If { cond, then, els }];
					}
					out.extend(els);
					match ipd {
						Some(next) => cur = next,
						None => return Ok(out),
					}
				}
				Term::Match { scrut, arms, default } => {
					let scrut = scrut.clone();
					let arms = arms.clone();
					let default = *default;
					let ipd = self.branch_follow(rctx, cur);
					let mut lowered = Vec::new();
					for (pat, vars, t) in arms {
						lowered.push((pat, vars, self.edge(t, ipd, rctx)?));
					}
					if let Some(d) = default {
						lowered.push(("_".into(), Vec::new(), self.edge(d, ipd, rctx)?));
					}
					out.push(Stmt::Match { scrut, arms: lowered });
					match ipd {
						Some(next) => cur = next,
						None => return Ok(out),
					}
				}
			}
		}
	}

	fn branch_follow(&self, ctx: Option<usize>, node: usize) -> Option<usize> {
		if self.terminal[node] {
			return None;
		}
		self.ipdom
			.get(&ctx)
			.and_then(|m| m.get(&node).copied().flatten())
			.filter(|&f| !self.ret_only[f])
	}

	/// `v := from.i` for each variable a loop returned in `from`.
	fn unpack(&self, out: &mut Vec<Stmt>, from: usize, vars: &[usize]) {
		let name = self.var_name(from).to_string();
		for (i, &v) in vars.iter().enumerate() {
			let text = if vars.len() == 1 {
				name.clone()
			} else {
				let mut parts = vec!["2"; i];
				if i + 1 < vars.len() {
					parts.push("1");
				}
				format!("{name}.{}", parts.join("."))
			};
			let mut value = var(from, &text);
			value.reads.insert(from);
			out.push(Stmt::Op(Op::Set {
				var: v,
				value,
				bind: false,
			}));
		}
	}
}

fn var(_v: usize, name: &str) -> Ex {
	Ex {
		text: name.to_string(),
		atom: true,
		monadic: false,
		reads: BTreeSet::new(),
		prop: None,
		lit: None,
	}
}

/// Drop writes nothing reads, and bind a match arm's field straight to the variable the
/// arm copies it into.
fn clean(f: &mut Function) {
	loop {
		let mut reads = vec![0usize; f.vars.len()];
		let mut count = |e: &Ex| {
			for &r in &e.reads {
				reads[r] += 1;
			}
		};
		for node in &f.nodes {
			for op in &node.ops {
				match op {
					Op::Set { value, .. } => count(value),
					Op::Check { cond, .. } => count(cond),
					Op::Throw(_) => {}
				}
			}
			match &node.term {
				Term::Cases { arms, .. } => arms.iter().for_each(|(c, _)| count(c)),
				Term::Match { scrut, .. } => count(scrut),
				_ => {}
			}
		}
		count(&f.ret_value);
		let before: usize = f.nodes.iter().map(|n| n.ops.len()).sum();
		for node in &mut f.nodes {
			node.ops
				.retain(|op| !matches!(op, Op::Set { var, value, bind: false } if reads[*var] == 0 && !value.monadic));
		}
		if f.nodes.iter().map(|n| n.ops.len()).sum::<usize>() == before {
			break;
		}
	}
	let mut preds = vec![0usize; f.nodes.len()];
	for node in &f.nodes {
		for t in term_succs(&node.term) {
			preds[t] += 1;
		}
	}
	let mut sets = vec![0usize; f.vars.len()];
	let mut read_in: Vec<HashSet<usize>> = vec![HashSet::new(); f.vars.len()];
	for (i, node) in f.nodes.iter().enumerate() {
		for op in &node.ops {
			match op {
				Op::Set { var, value, .. } => {
					sets[*var] += 1;
					value.reads.iter().for_each(|&r| _ = read_in[r].insert(i));
				}
				Op::Check { cond, .. } => cond.reads.iter().for_each(|&r| _ = read_in[r].insert(i)),
				Op::Throw(_) => {}
			}
		}
		let term_reads: Vec<usize> = match &node.term {
			Term::Cases { arms, .. } => arms.iter().flat_map(|(c, _)| c.reads.iter().copied()).collect(),
			Term::Match { scrut, .. } => scrut.reads.iter().copied().collect(),
			Term::Return => f.ret_value.reads.iter().copied().collect(),
			_ => Vec::new(),
		};
		term_reads.into_iter().for_each(|r| _ = read_in[r].insert(i));
	}
	for m in 0..f.nodes.len() {
		let Term::Match { arms, .. } = &f.nodes[m].term else {
			continue;
		};
		let mut renames = Vec::new();
		for (a, (_, vars, t)) in arms.iter().enumerate() {
			let t = *t;
			if preds[t] != 1 {
				continue;
			}
			let Some(Op::Set {
				var: x,
				value,
				bind: false,
			}) = f.nodes[t].ops.first()
			else {
				continue;
			};
			let Some(i) = vars
				.iter()
				.position(|&p| value.reads.len() == 1 && value.reads.contains(&p) && value.text == f.vars[p].name)
			else {
				continue;
			};
			let p = vars[i];
			let x = *x;
			let only_here = read_in[x].iter().all(|&n| n == t) && read_in[p].iter().all(|&n| n == t);
			let p_once = f.nodes[t].ops[1..].iter().all(|op| match op {
				Op::Set { value, .. } => !value.reads.contains(&p),
				Op::Check { cond, .. } => !cond.reads.contains(&p),
				Op::Throw(_) => true,
			});
			if f.vars[x].user && sets[x] == 1 && only_here && p_once {
				renames.push((a, i, p, x, t));
			}
		}
		for (a, i, p, x, t) in renames {
			let (old, new) = (f.vars[p].name.clone(), f.vars[x].name.clone());
			if let Term::Match { arms, .. } = &mut f.nodes[m].term {
				let (pat, vars, _) = &mut arms[a];
				vars[i] = x;
				*pat = pat
					.split(' ')
					.map(|tok| if tok == old { new.as_str() } else { tok })
					.collect::<Vec<_>>()
					.join(" ");
			}
			f.nodes[t].ops.remove(0);
		}
	}
}

/// `ret := e` then `return .. ret ..` is `return .. e ..`.
fn fold_returns(stmts: &mut Vec<Stmt>, f: &Function) {
	for s in stmts.iter_mut() {
		match s {
			Stmt::If { then, els, .. } => {
				fold_returns(then, f);
				fold_returns(els, f);
			}
			Stmt::Match { arms, .. } => arms.iter_mut().for_each(|a| fold_returns(&mut a.2, f)),
			_ => {}
		}
	}
	let Some(ret) = f.vars.iter().position(|v| v.name == "ret") else {
		return;
	};
	let n = stmts.len();
	if n < 2 {
		return;
	}
	let (Stmt::Op(Op::Set { var, value, bind }), Stmt::Return(e)) = (&stmts[n - 2], &stmts[n - 1]) else {
		return;
	};
	if *var != ret || value.reads.contains(&ret) {
		return;
	}
	let words: Vec<&str> = e
		.text
		.split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '\''))
		.collect();
	if words.iter().filter(|w| **w == "ret").count() != 1 {
		return;
	}
	let shown = if *bind {
		format!("(← {})", value.text)
	} else {
		value.paren()
	};
	let mut text = String::new();
	let mut word = String::new();
	for c in e.text.chars().chain(std::iter::once('\0')) {
		if c.is_alphanumeric() || c == '_' || c == '\'' {
			word.push(c);
			continue;
		}
		text += if word == "ret" { &shown } else { &word };
		word.clear();
		if c != '\0' {
			text.push(c);
		}
	}
	// `return (e)` reads better as `return e`.
	if text.starts_with('(') && text.ends_with(')') && text[1..text.len() - 1] == shown {
		text = shown.clone();
	}
	if text == value.paren() && !value.atom {
		text = value.text.clone();
	}
	let mut reads = e.reads.clone();
	reads.remove(&ret);
	reads.extend(value.reads.iter().copied());
	let monadic = e.monadic || value.monadic || *bind;
	stmts.truncate(n - 2);
	stmts.push(Stmt::Return(Ex {
		text,
		atom: false,
		monadic,
		reads,
		prop: None,
		lit: None,
	}));
}

fn value_reads(stmt: Option<&Stmt>) -> BTreeSet<usize> {
	match stmt {
		Some(Stmt::Op(Op::Set { value, .. })) => value.reads.clone(),
		_ => BTreeSet::new(),
	}
}

fn term_succs(term: &Term) -> Vec<usize> {
	match term {
		Term::Goto(t) => vec![*t],
		Term::Cases { arms, default } => arms.iter().map(|a| a.1).chain([*default]).collect(),
		Term::Match { arms, default, .. } => arms.iter().map(|a| a.2).chain(*default).collect(),
		Term::Return | Term::Throw(_) | Term::Unreachable => vec![],
	}
}

fn writes(node: &Node) -> Vec<usize> {
	node.ops
		.iter()
		.filter_map(|op| match op {
			Op::Set { var, .. } => Some(*var),
			_ => None,
		})
		.collect()
}

/// The variables live into each node; the variables a match binds are bound at its arm.
fn liveness(f: &Function, succs: &[Vec<usize>]) -> Vec<BTreeSet<usize>> {
	let n = f.nodes.len();
	let bound: HashSet<usize> = f
		.nodes
		.iter()
		.flat_map(|node| match &node.term {
			Term::Match { arms, .. } => arms.iter().flat_map(|a| a.1.iter().copied()).collect(),
			_ => Vec::new(),
		})
		.collect();
	let mut uses = vec![BTreeSet::new(); n];
	let mut defs = vec![BTreeSet::new(); n];
	for (i, node) in f.nodes.iter().enumerate() {
		let read = |reads: &BTreeSet<usize>, defs: &BTreeSet<usize>, uses: &mut BTreeSet<usize>| {
			for r in reads {
				if !defs.contains(r) && !bound.contains(r) {
					uses.insert(*r);
				}
			}
		};
		for op in &node.ops {
			match op {
				Op::Set { var, value, .. } => {
					read(&value.reads, &defs[i], &mut uses[i]);
					defs[i].insert(*var);
				}
				Op::Check { cond, .. } => read(&cond.reads, &defs[i], &mut uses[i]),
				Op::Throw(_) => {}
			}
		}
		match &node.term {
			Term::Cases { arms, .. } => {
				for (c, _) in arms {
					read(&c.reads, &defs[i], &mut uses[i]);
				}
			}
			Term::Match { scrut, .. } => read(&scrut.reads, &defs[i], &mut uses[i]),
			Term::Return => read(&f.ret_value.reads, &defs[i], &mut uses[i]),
			_ => {}
		}
	}
	let mut live = vec![BTreeSet::new(); n];
	loop {
		let mut changed = false;
		for i in (0..n).rev() {
			let mut out: BTreeSet<usize> = BTreeSet::new();
			for &s in &succs[i] {
				out.extend(live[s].iter().copied());
			}
			let mut new: BTreeSet<usize> = out.difference(&defs[i]).copied().collect();
			new.extend(uses[i].iter().copied());
			if new != live[i] {
				live[i] = new;
				changed = true;
			}
		}
		if !changed {
			return live;
		}
	}
}

fn postorder(n: usize, entry: usize, succs: &[Vec<usize>]) -> Vec<usize> {
	let mut seen = vec![false; n];
	let mut order = Vec::new();
	let mut stack = vec![(entry, 0usize)];
	seen[entry] = true;
	while let Some((node, i)) = stack.pop() {
		if i < succs[node].len() {
			stack.push((node, i + 1));
			let s = succs[node][i];
			if !seen[s] {
				seen[s] = true;
				stack.push((s, 0));
			}
		} else {
			order.push(node);
		}
	}
	order
}

/// Immediate dominators from `entry` (Cooper, Harvey, and Kennedy); `usize::MAX` where
/// unreachable.
fn dominators(n: usize, entry: usize, succs: &[Vec<usize>]) -> Vec<usize> {
	let order = postorder(n, entry, succs);
	let mut index = vec![usize::MAX; n];
	for (i, &node) in order.iter().enumerate() {
		index[node] = i;
	}
	let mut preds = vec![Vec::new(); n];
	for (u, ss) in succs.iter().enumerate() {
		if index[u] == usize::MAX {
			continue;
		}
		for &s in ss {
			preds[s].push(u);
		}
	}
	let mut idom = vec![usize::MAX; n];
	idom[entry] = entry;
	loop {
		let mut changed = false;
		for &b in order.iter().rev() {
			if b == entry {
				continue;
			}
			let mut new = usize::MAX;
			for &p in &preds[b] {
				if idom[p] == usize::MAX {
					continue;
				}
				new = if new == usize::MAX {
					p
				} else {
					let (mut x, mut y) = (p, new);
					while x != y {
						while index[x] < index[y] {
							x = idom[x];
						}
						while index[y] < index[x] {
							y = idom[y];
						}
					}
					x
				};
			}
			if new != idom[b] {
				idom[b] = new;
				changed = true;
			}
		}
		if !changed {
			return idom;
		}
	}
}

fn effectful(stmt: &Stmt) -> bool {
	match stmt {
		Stmt::Op(Op::Set { value, bind, .. }) => *bind || value.monadic,
		Stmt::Op(Op::Check { .. } | Op::Throw(_)) => true,
		Stmt::If { cond, then, els } => cond.monadic || then.iter().any(effectful) || els.iter().any(effectful),
		Stmt::Match { scrut, arms } => scrut.monadic || arms.iter().any(|a| a.2.iter().any(effectful)),
		Stmt::Return(e) => e.monadic,
		Stmt::Let { value, bind, .. } => *bind || value.as_ref().is_some_and(|v| v.monadic),
	}
}

/// Whether `stmt` reads or writes `v`, and how many times it writes it.
fn refs(stmt: &Stmt, v: usize) -> bool {
	match stmt {
		Stmt::Op(Op::Set { var, value, .. }) => *var == v || value.reads.contains(&v),
		Stmt::Op(Op::Check { cond, .. }) => cond.reads.contains(&v),
		Stmt::Op(Op::Throw(_)) => false,
		Stmt::If { cond, then, els } => {
			cond.reads.contains(&v) || then.iter().any(|s| refs(s, v)) || els.iter().any(|s| refs(s, v))
		}
		Stmt::Match { scrut, arms } => scrut.reads.contains(&v) || arms.iter().any(|a| a.2.iter().any(|s| refs(s, v))),
		Stmt::Return(e) => e.reads.contains(&v),
		Stmt::Let { var, value, .. } => *var == v || value.as_ref().is_some_and(|x| x.reads.contains(&v)),
	}
}

/// Whether `v`'s first use in `stmts` writes it without reading it, in every branch.
fn defines(stmts: &[Stmt], v: usize) -> bool {
	match stmts.iter().find(|s| refs(s, v)) {
		Some(Stmt::Op(Op::Set { var, value, .. })) => *var == v && !value.reads.contains(&v),
		Some(Stmt::If { cond, then, els }) => {
			!cond.reads.contains(&v)
				&& [then, els]
					.iter()
					.all(|b| !b.iter().any(|s| refs(s, v)) || defines(b, v))
		}
		Some(Stmt::Match { scrut, arms }) => {
			!scrut.reads.contains(&v) && arms.iter().all(|a| !a.2.iter().any(|s| refs(s, v)) || defines(&a.2, v))
		}
		_ => false,
	}
}

fn sets(stmts: &[Stmt], v: usize) -> usize {
	stmts
		.iter()
		.map(|s| match s {
			Stmt::Op(Op::Set { var, .. }) => usize::from(*var == v),
			Stmt::If { then, els, .. } => sets(then, v) + sets(els, v),
			Stmt::Match { arms, .. } => arms.iter().map(|a| sets(&a.2, v)).sum(),
			_ => 0,
		})
		.sum()
}

fn collect_vars(stmts: &[Stmt], out: &mut BTreeSet<usize>, bound: &mut HashSet<usize>) {
	for s in stmts {
		match s {
			Stmt::Op(Op::Set { var, value, .. }) => {
				out.insert(*var);
				out.extend(value.reads.iter().copied());
			}
			Stmt::Op(Op::Check { cond, .. }) => out.extend(cond.reads.iter().copied()),
			Stmt::Op(Op::Throw(_)) => {}
			Stmt::If { cond, then, els } => {
				out.extend(cond.reads.iter().copied());
				collect_vars(then, out, bound);
				collect_vars(els, out, bound);
			}
			Stmt::Match { scrut, arms } => {
				out.extend(scrut.reads.iter().copied());
				for (_, vars, body) in arms {
					bound.extend(vars.iter().copied());
					collect_vars(body, out, bound);
				}
			}
			Stmt::Return(e) => out.extend(e.reads.iter().copied()),
			Stmt::Let { .. } => {}
		}
	}
}

/// Variable names and types, for declaring and printing.
struct Printer<'a> {
	names: &'a [String],
	tys: &'a [String],
	/// Whether it prints a `partial_fixpoint` body, whose monotonicity proof cannot
	/// split a match on a local variable; `id x` it can.
	in_loop: std::cell::Cell<bool>,
}

impl Printer<'_> {
	/// Declare each variable where its uses begin: `let` at its first write when that
	/// write is in the narrowest block holding all of them, `let mut` if written again.
	fn declare(&self, body: &mut Vec<Stmt>, params: &[usize]) -> Result<(), String> {
		let mut all = BTreeSet::new();
		let mut bound = HashSet::new();
		collect_vars(body, &mut all, &mut bound);
		for v in all {
			if bound.contains(&v) || params.contains(&v) {
				continue;
			}
			let total = sets(body, v);
			self.place(body, v, total)?;
		}
		let mut prefix = Vec::new();
		for &p in params {
			if sets(body, p) > 0 {
				prefix.push(Stmt::Let {
					var: p,
					mutable: true,
					value: Some(var(p, &self.names[p])),
					bind: false,
				});
			}
		}
		prefix.append(body);
		*body = prefix;
		Ok(())
	}

	fn place(&self, list: &mut Vec<Stmt>, v: usize, total: usize) -> Result<(), String> {
		let idxs: Vec<usize> = (0..list.len()).filter(|&i| refs(&list[i], v)).collect();
		let Some(&i) = idxs.first() else { return Ok(()) };
		if idxs.len() == 1 {
			// Each branch that uses it on its own, when no other does or each defines it first.
			let mut branches: Vec<&mut Vec<Stmt>> = match &mut list[i] {
				Stmt::If { cond, then, els } if !cond.reads.contains(&v) => vec![then, els],
				Stmt::Match { scrut, arms } if !scrut.reads.contains(&v) => arms.iter_mut().map(|a| &mut a.2).collect(),
				_ => Vec::new(),
			};
			branches.retain(|b| b.iter().any(|s| refs(s, v)));
			if branches.len() == 1 || (!branches.is_empty() && branches.iter().all(|b| defines(b, v))) {
				for b in branches {
					self.place(b, v, total)?;
				}
				return Ok(());
			}
		}
		if let Stmt::Op(Op::Set { var, value, bind }) = &list[i]
			&& *var == v
			&& !value.reads.contains(&v)
		{
			list[i] = Stmt::Let {
				var: v,
				mutable: total > 1,
				value: Some(value.clone()),
				bind: *bind,
			};
			return Ok(());
		}
		if self.tys[v].is_empty() {
			return Err(format!("variable `{}` has no type to declare it with", self.names[v]));
		}
		list.insert(
			i,
			Stmt::Let {
				var: v,
				mutable: true,
				value: None,
				bind: false,
			},
		);
		Ok(())
	}

	fn block(&self, stmts: &[Stmt], depth: usize) -> String {
		let mut out = String::new();
		self.print(stmts, depth, &mut out);
		if stmts.is_empty() {
			out += &format!("{}pure ()\n", "  ".repeat(depth));
		}
		out
	}

	fn print(&self, stmts: &[Stmt], depth: usize, out: &mut String) {
		let pad = "  ".repeat(depth);
		for s in stmts {
			match s {
				Stmt::Op(Op::Set { var, value, bind }) => {
					let arrow = if *bind { "←" } else { ":=" };
					*out += &format!("{pad}{} {arrow} {}\n", self.names[*var], value.text);
				}
				Stmt::Op(Op::Check { cond, msg }) => *out += &format!("{pad}unless {} do throw {msg}\n", cond.cond()),
				Stmt::Op(Op::Throw(msg)) => *out += &format!("{pad}throw {msg}\n"),
				Stmt::Return(e) => *out += &format!("{pad}return {}\n", e.text),
				Stmt::Let {
					var,
					mutable,
					value,
					bind,
				} => {
					let m = if *mutable { "mut " } else { "" };
					let name = &self.names[*var];
					match value {
						Some(v) if *bind => *out += &format!("{pad}let {m}{name} ← {}\n", v.text),
						Some(v) if v.lit.is_some() => {
							*out += &format!(
								"{pad}let {m}{name} : {} := {}\n",
								self.tys[*var],
								v.lit.clone().unwrap_or_default()
							)
						}
						Some(v) => *out += &format!("{pad}let {m}{name} := {}\n", v.text),
						None => *out += &format!("{pad}let {m}{name} : {} := default\n", self.tys[*var]),
					}
				}
				Stmt::If { cond, then, els } => {
					if then.is_empty() && els.is_empty() {
						continue;
					}
					if then.is_empty() {
						*out += &format!("{pad}if ¬({}) then\n", cond.cond());
						self.print(els, depth + 1, out);
						continue;
					}
					*out += &format!("{pad}if {} then\n", cond.cond());
					self.print(then, depth + 1, out);
					if !els.is_empty() {
						if let [Stmt::If { .. }] = els.as_slice() {
							let mut nested = String::new();
							self.print(els, depth, &mut nested);
							*out += &format!("{pad}else {}", nested.trim_start());
						} else {
							*out += &format!("{pad}else\n");
							self.print(els, depth + 1, out);
						}
					}
				}
				Stmt::Match { scrut, arms } => {
					let local = scrut.atom && scrut.text.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '\'');
					if self.in_loop.get() && local {
						*out += &format!("{pad}match id {} with\n", scrut.text);
					} else {
						*out += &format!("{pad}match {} with\n", scrut.text);
					}
					for (pat, _, body) in arms {
						*out += &format!("{pad}| {pat} =>\n");
						if body.is_empty() {
							*out += &format!("{pad}  pure ()\n");
						} else {
							self.print(body, depth + 1, out);
						}
					}
				}
			}
		}
	}
}
