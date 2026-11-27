# Commits

PRs into `main` land as squash commits through GitHub's merge queue, so the PR title becomes the commit subject.

- Use conventional-commit subjects (`feat(ts): ...`, `fix: ...`, `chore: ...`, `docs: ...`).
- Never commit binaries or build artifacts.

# PRs

Keep the body short and structured, not narrated.
Have at least these sections:

- **Problem**: a summary of the problem and why this PR is needed.
- **Approach**: a summary of the approach taken to solve the problem.
- **Impact**: a bullet point for every public API change.
- **Alternatives**: any alternative approaches considered.
- **Follow-ups**: any issues encountered or quests created.

# Landing

Queue the reviewed head with `quest merge <pr> --land <sha>`.
The queue's build is `.github/workflows/build.yml` on GitHub-hosted runners; it is the only CI a PR gets. A pull request builds nothing: `queue.yml` only posts the required `Queue build / Build` context as skipped there, so the PR can enter the queue. Run `just ci` before every push, and test another platform over SSH on a host that has it.
A PR the queue drops for a failed build or a conflict gets a Codex fixer, which requeues it or hands it back with a diagnosis.
The admin bypass of `main`'s ruleset is for emergencies.
`just ci` selects what the diff touches by `sh/dispatch.sh`'s map; `just gate` runs the full suite nightly, beside `sh/coverage.py`, which fails where a module reads a path whose diff the map does not select it for.

# Imported code

Each imported component keeps its own license and records its upstream source and revision.
