# [S] The merge queue's required check waits on no GitHub-hosted runner

## Goal

A merge group's required context reports as soon as its self-hosted build finishes, however
busy GitHub-hosted runners are across moq-dev.

## Plan

`main`'s ruleset requires `Queue`, which `.github/workflows/queue.yml` reports from a job on
`ubuntu-latest` after `queue-build.yml` builds the group on the `rust2-queue` runner. Hosted
capacity is shared by every moq-dev repository, so when moq's and rind's runs fill it, the
group sits with a passed build until the queue's response window drops it. On 2026-11-12,
[#709](https://github.com/moq-dev/rust2/pull/709)'s group (run 43171064245) built in 14
minutes and then waited for that runner for over 1h45m, while moq-dev/rind had 78 runs and
moq-dev/moq 63 queued; run 43095195569 from 2026-11-11 was still queued too.

Make the required context come from the self-hosted build or from nothing that queues on a
hosted runner, keeping what queue.yml's comments guard: only `queue-build.yml` at `main`
reaches the `rust2-queue` runner group, a pull request still satisfies the requirement
without building, and a failed or cancelled build fails the group. Recommendation: require
the build's own context (`Queue build / Build`) instead of `Queue`, if a pull request's
skipped run of it satisfies the ruleset as the skipped `Queue` job does; changing `main`'s
ruleset is a maintainer step, so the PR gives its exact change. Check moq's and rind's queues
for the same job and report them as follow-ups for their trees.

Public API: none.
