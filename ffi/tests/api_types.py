# A typed caller of the api fixture's Python, which tests/fixtures.rs's `api_py_types`
# type-checks alongside it but never runs. Each list it passes in holds one alternative of
# a widened element type, or is no `list` at all, as a caller's own may be.

import api

v4s: list[tuple[str, int]] = [("127.0.0.1", 8080)]
ports: list[int] = api.ports(v4s)
ports = api.ports(addr for addr in v4s)

rows: list[list[int]] = [[1, 2], [3]]
sums: list[int] = api.sums(rows)
sums = api.sums(((1, 2), (3,)))

names: tuple[str, ...] = ("a", "b")
peer = api.Peer(v4s[0], names)
first: str = peer.names[0]
