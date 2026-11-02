# Commits

PRs into `main` land as squash commits through the land gate, so the PR title becomes the commit subject.

- Use conventional-commit subjects (`feat(ts): ...`, `fix: ...`, `chore: ...`, `docs: ...`).
- AI commit attribution goes in a `Co-Authored-By:` trailer, not the commit body.
- Never commit binaries or build artifacts.

# PRs

Keep the body short and structured, not narrated.
Have at least these sections:

- **Problem**: a summary of the problem and why this PR is needed.
- **Approach**: a summary of the approach taken to solve the problem.
- **Impact**: a bullet point for every public API change. A breaking one also gets a `MIGRATE.md` entry.
- **Alternatives**: any alternative approaches considered.
- **Follow-ups**: any issues encountered or quests created.

Never rebase or force-push a pushed branch; merge the base branch in instead.
Open a draft PR while working, run `just ci` before every push, and mark the PR ready yourself when the work is done.

# Landing

PRs into `main` get no cloud CI; `just ci` runs the same gate locally.
PRs land only through `quest land`: queue the reviewed head with `quest merge <pr> <token> --land <sha>` (`/quest-merge` does this).
The land gate, `quest land serve` on the maintainer's host, takes the longest-queued PR plus up to log₂ n more that merge cleanly, runs `just ci` once on the batch, and lands each PR through GitHub's squash merge.
A failing batch is ejected together, and a Codex fixer makes mechanical fixes and requeues healthy PRs or hands them back with a comment: read why in the PR, fix the cause, and queue again.
`just ci` selects only what the diff touches, against the gate's base or `origin/main` locally, so a break can reach `main`; `just gate` runs the full suite, and it and the other optional workflows run nightly on `main`.
The admin bypass of `main`'s ruleset is for emergencies.

# Reviews

Review each PR locally with `/quest-review`, as the quest guide describes; merging never waits on an online reviewer, and never request one.
The PR body's `## Review` record counts as the required review when it names the head being merged, and so does a recorded skip of a change too trivial to review.
Address online reviewers' findings already posted on that head before merging: fix them, or reply where you disagree.

# Follow-ups

Record out-of-scope findings as follow-up quests with `/quest-plan`, not GitHub issues.

# Forks

A hard fork never merges upstream: fixes are cherry-picked by hand, and each carried change is listed in the fork's README with its upstream PR or the reason it has none.
Each imported component keeps its own license and records its upstream source and revision.
