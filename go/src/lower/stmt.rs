//! A function body as Go statements: the lines lowering writes, the control flow [`cfg`]
//! structures around them and tidies, and where each variable is declared.

use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use rust2_driver::cfg::{self, Form, Jump};

pub enum Stmt {
	/// Lines of Go, each ending in a newline.
	Lines(String),
	If {
		cond: Cond,
		then: Vec<Stmt>,
		els: Vec<Stmt>,
	},
	/// `label: for cond { body }`.
	Loop {
		label: Option<String>,
		cond: Cond,
		body: Vec<Stmt>,
	},
	/// `{ body }` and then its label, as Go's `break` leaves only a loop or a `switch`: a
	/// `break` to it is a `goto` the label.
	Block {
		label: Option<String>,
		body: Vec<Stmt>,
	},
	Break(Option<String>),
	Continue(Option<String>),
	Return(Option<String>),
	Switch {
		disc: String,
		cases: Vec<(String, Vec<Stmt>)>,
	},
}

/// A Go boolean expression.
pub struct Cond(pub String);

impl cfg::Cond for Cond {
	fn negate(&mut self) {
		let c = &self.0;
		self.0 = match c.strip_prefix('!') {
			Some(inner) if super::fold::operand(inner) => inner.to_string(),
			_ if super::fold::operand(c) => format!("!{c}"),
			_ => format!("!({c})"),
		};
	}

	fn is_true(&self) -> bool {
		self.0 == "true"
	}

	/// Never, as the reads it holds made their variables used, which Go requires.
	fn pure(&self) -> bool {
		false
	}
}

impl cfg::Syntax for Stmt {
	type Cond = Cond;

	fn block(label: String, body: Vec<Self>) -> Self {
		Stmt::Block {
			label: Some(label),
			body,
		}
	}

	fn forever(label: String, body: Vec<Self>) -> Self {
		Stmt::Loop {
			label: Some(label),
			cond: Cond("true".into()),
			body,
		}
	}

	fn branch(cond: Cond, then: Vec<Self>, els: Vec<Self>) -> Self {
		Stmt::If { cond, then, els }
	}

	fn jump(jump: Jump, label: String) -> Self {
		match jump {
			Jump::Break => Stmt::Break(Some(label)),
			Jump::Continue => Stmt::Continue(Some(label)),
		}
	}

	fn dispatch(state: &str, label: &str, cases: Vec<(usize, Vec<Self>)>, out: &mut Vec<Self>) {
		out.push(Stmt::Lines(format!("\t{state} := 0\n")));
		out.push(Stmt::Loop {
			label: Some(label.to_string()),
			cond: Cond("true".into()),
			body: vec![Stmt::Switch {
				disc: state.to_string(),
				cases: cases.into_iter().map(|(node, body)| (node.to_string(), body)).collect(),
			}],
		});
	}

	fn set(state: &str, node: usize) -> Self {
		Stmt::Lines(format!("\t{state} = {node}\n"))
	}

	fn view(&self) -> cfg::View<'_, Self> {
		match self {
			Stmt::Block { label, body } => Form::Block { label, body },
			Stmt::Loop { label, cond, body } => Form::Loop {
				label,
				cond: Some(cond),
				body,
			},
			Stmt::If { cond, then, els } => Form::If { cond, then, els },
			Stmt::Switch { cases, .. } => Form::Switch(cases.iter().map(|(_, body)| body).collect()),
			Stmt::Break(label) => Form::Break(label),
			Stmt::Continue(label) => Form::Continue(label),
			Stmt::Return(value) => Form::Return(value.is_some()),
			Stmt::Lines(_) => Form::Other,
		}
	}

	fn view_mut(&mut self) -> cfg::ViewMut<'_, Self> {
		match self {
			Stmt::Block { label, body } => Form::Block { label, body },
			Stmt::Loop { label, cond, body } => Form::Loop {
				label,
				cond: Some(cond),
				body,
			},
			Stmt::If { cond, then, els } => Form::If { cond, then, els },
			Stmt::Switch { cases, .. } => Form::Switch(cases.iter_mut().map(|(_, body)| body).collect()),
			Stmt::Break(label) => Form::Break(label),
			Stmt::Continue(label) => Form::Continue(label),
			Stmt::Return(value) => Form::Return(value.is_some()),
			Stmt::Lines(_) => Form::Other,
		}
	}
}

/// Write `stmts` as Go, `gofmt` to indent them. `blocks` holds the labels of the blocks
/// they are in, which a `break` reaches by `goto`.
pub fn print(stmts: &[Stmt], blocks: &mut HashSet<String>, out: &mut String) {
	let target = |label: &Option<String>| label.as_ref().map(|l| format!(" {l}")).unwrap_or_default();
	for stmt in stmts {
		match stmt {
			Stmt::Lines(text) => out.push_str(text),
			Stmt::If { cond, then, els } => {
				writeln!(out, "if {} {{", cond.0).unwrap();
				print(then, blocks, out);
				match els.as_slice() {
					[] => {}
					[nested @ Stmt::If { .. }] => {
						out.push_str("} else ");
						print(std::slice::from_ref(nested), blocks, out);
						continue;
					}
					els => {
						out.push_str("} else {\n");
						print(els, blocks, out);
					}
				}
				out.push_str("}\n");
			}
			Stmt::Loop { label, cond, body } => {
				if let Some(label) = label {
					writeln!(out, "{label}:").unwrap();
				}
				match cond.0.as_str() {
					"true" => out.push_str("for {\n"),
					cond => writeln!(out, "for {cond} {{").unwrap(),
				}
				print(body, blocks, out);
				out.push_str("}\n");
			}
			Stmt::Block { label, body } => {
				if let Some(label) = label {
					blocks.insert(label.clone());
				}
				out.push_str("{\n");
				print(body, blocks, out);
				out.push_str("}\n");
				if let Some(label) = label {
					writeln!(out, "{label}:").unwrap();
				}
			}
			Stmt::Break(Some(label)) if blocks.contains(label) => writeln!(out, "goto {label}").unwrap(),
			Stmt::Break(label) => writeln!(out, "break{}", target(label)).unwrap(),
			Stmt::Continue(label) => writeln!(out, "continue{}", target(label)).unwrap(),
			Stmt::Return(value) => match value {
				Some(value) => writeln!(out, "return {value}").unwrap(),
				None => out.push_str("return\n"),
			},
			Stmt::Switch { disc, cases } => {
				writeln!(out, "switch {disc} {{").unwrap();
				for (case, body) in cases {
					writeln!(out, "case {case}:").unwrap();
					print(body, blocks, out);
				}
				out.push_str("}\n");
			}
		}
	}
}

/// Where each variable of `decls`, a name and its declaration, is declared: at the start
/// of the innermost statement list that holds every statement naming it and that no loop
/// holds, so Go zeroes it only where control reaches it, still once per call. A variable
/// nothing names goes first in `body`.
pub fn declare(body: &mut Vec<Stmt>, decls: Vec<(String, String)>) {
	let mut at: HashMap<&str, Vec<Step>> = HashMap::new();
	let names: HashSet<&str> = decls.iter().map(|(name, _)| name.as_str()).collect();
	uses(body, &mut Vec::new(), &names, &mut at);
	let mut placed: HashMap<Vec<(usize, usize)>, String> = HashMap::new();
	for (name, decl) in &decls {
		let path = at.get(name.as_str()).map_or(&[][..], |path| path.as_slice());
		let outer = path.iter().position(|step| step.looped).unwrap_or(path.len());
		let key = path[..outer].iter().map(|step| (step.stmt, step.list)).collect();
		placed.entry(key).or_default().push_str(decl);
	}
	insert(body, &mut Vec::new(), &mut placed);
}

/// One step into a statement's list: the statement's index, which of its lists, and
/// whether the statement is a loop.
#[derive(Clone, Copy, PartialEq)]
struct Step {
	stmt: usize,
	list: usize,
	looped: bool,
}

/// The statement lists a statement holds, in order.
fn lists(stmt: &mut Stmt) -> Vec<&mut Vec<Stmt>> {
	match stmt {
		Stmt::If { then, els, .. } => vec![then, els],
		Stmt::Loop { body, .. } | Stmt::Block { body, .. } => vec![body],
		Stmt::Switch { cases, .. } => cases.iter_mut().map(|(_, body)| body).collect(),
		Stmt::Lines(_) | Stmt::Break(_) | Stmt::Continue(_) | Stmt::Return(_) => Vec::new(),
	}
}

/// Narrow each name's path in `at` to the list at `path` holding a statement of `stmts`
/// naming it, or to what it shares with the paths already there.
fn uses<'a>(stmts: &mut [Stmt], path: &mut Vec<Step>, names: &HashSet<&'a str>, at: &mut HashMap<&'a str, Vec<Step>>) {
	for (i, stmt) in stmts.iter_mut().enumerate() {
		let text = match &*stmt {
			Stmt::Lines(text) => Some(text.as_str()),
			Stmt::If { cond, .. } | Stmt::Loop { cond, .. } => Some(cond.0.as_str()),
			Stmt::Switch { disc, .. } => Some(disc.as_str()),
			Stmt::Return(value) => value.as_deref(),
			Stmt::Block { .. } | Stmt::Break(_) | Stmt::Continue(_) => None,
		};
		let idents = text
			.into_iter()
			.flat_map(|t| t.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')));
		for ident in idents {
			if let Some(&name) = names.get(ident) {
				match at.get_mut(name) {
					Some(known) => {
						let shared = known.iter().zip(path.iter()).take_while(|(a, b)| a == b).count();
						known.truncate(shared);
					}
					None => {
						at.insert(name, path.clone());
					}
				}
			}
		}
		let looped = matches!(stmt, Stmt::Loop { .. });
		for (list, stmts) in lists(stmt).into_iter().enumerate() {
			path.push(Step { stmt: i, list, looped });
			uses(stmts, path, names, at);
			path.pop();
		}
	}
}

/// Put the declarations `placed`, by the path to their list, at the start of each.
fn insert(stmts: &mut Vec<Stmt>, path: &mut Vec<(usize, usize)>, placed: &mut HashMap<Vec<(usize, usize)>, String>) {
	for (i, stmt) in stmts.iter_mut().enumerate() {
		for (list, stmts) in lists(stmt).into_iter().enumerate() {
			path.push((i, list));
			insert(stmts, path, placed);
			path.pop();
		}
	}
	if let Some(decls) = placed.remove(path.as_slice()) {
		stmts.insert(0, Stmt::Lines(decls));
	}
}
