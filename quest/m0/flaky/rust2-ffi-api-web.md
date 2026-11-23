# [S] Fix the flaky test rust2-ffi-api-web

## Goal

Fix `rust2-ffi api_web` so it passes reliably, or delete it, and remove its skip.
quest land watch disabled it after the land fixer judged it flaky in
more than one `merge_group` build.

Failing jobs:

- [Job 1](https://github.com/kixel-ai/rust2/actions/runs/45125842456/job/137529859975)
- [Job 2](https://github.com/kixel-ai/rust2/actions/runs/45128318651/job/137537711822)
