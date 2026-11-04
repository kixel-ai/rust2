//! A function body as Go statements: the lines lowering writes, and the control flow
//! [`cfg`] structures around them and tidies.

use std::collections::HashSet;
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
