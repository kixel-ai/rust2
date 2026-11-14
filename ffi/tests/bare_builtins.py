"""Fails when a generated module reads a Python builtin by its bare name.

Run as `python3 -I bare_builtins.py api.py`. A method or function named as a builtin, such as
`bytes`, shadows that builtin in the class or module that declares it, so the module
reads each builtin through an import alias such as `_bytes` instead.
"""

import ast
import builtins
import sys

allowed = {"__name__"}
tree = ast.parse(open(sys.argv[1]).read())
bare = sorted(
	f"{sys.argv[1]}:{n.lineno}: {n.id}"
	for n in ast.walk(tree)
	if isinstance(n, ast.Name) and isinstance(n.ctx, ast.Load) and hasattr(builtins, n.id) and n.id not in allowed
)
if bare:
	sys.exit("a builtin read by its bare name:\n" + "\n".join(bare))
