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
The queue's build lives in `.github/workflows/queue-build.yml`, which the merge group calls from `main`, so a PR's edit to it runs only after the PR lands.
A PR the queue drops for a failed build or a conflict gets a Codex fixer, which requeues it or hands it back with a diagnosis.
The admin bypass of `main`'s ruleset is for emergencies.
`just ci` selects what the diff touches; `just gate` runs the full suite nightly.

# Imported code

Each imported component keeps its own license and records its upstream source and revision.
