"""Fails when a generated module reads a Python builtin by its bare name.

Run as `python3 -I bare_builtins.py api.py`. A method or function named as a builtin, such as
`bytes`, shadows that builtin in the class or module that declares it, so the module
reads each builtin through an import alias such as `_bytes` instead. A parameter named as
one, such as a record's `__init__` taking its field `len`, is that parameter in its body.
"""

import ast
import builtins
import sys

allowed = {"__name__"}
tree = ast.parse(open(sys.argv[1]).read())


def loads(node, params):
	"""Each name `node` reads, but the parameters of a function it is in."""
	if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.Lambda)):
		a = node.args
		params = params | {p.arg for p in a.posonlyargs + a.args + a.kwonlyargs + [a.vararg, a.kwarg] if p}
	if isinstance(node, ast.Name) and isinstance(node.ctx, ast.Load) and node.id not in params:
		yield node
	for child in ast.iter_child_nodes(node):
		yield from loads(child, params)


bare = sorted(
	f"{sys.argv[1]}:{n.lineno}: {n.id}"
	for n in loads(tree, frozenset())
	if hasattr(builtins, n.id) and n.id not in allowed
)
if bare:
	sys.exit("a builtin read by its bare name:\n" + "\n".join(bare))
