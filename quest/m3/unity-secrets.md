# [XS] Unity credentials only in the unity environment

## Goal

`UNITY_EMAIL` and `UNITY_PASSWORD`, the maintainer's Unity account that the nightly's Unity job signs in with, live only as secrets of the repository's `unity` environment, which only `main` may deploy to, and no longer as repository secrets.
A repository secret reaches every workflow on every branch of the repository, including a pull request's from a branch here, so any branch could read the password; an environment secret reaches only a job on `main` that names the environment.

## Plan

The nightly's Unity job already runs in the `unity` environment, and an environment secret takes precedence over a repository secret of the same name, so no workflow change is needed.
Once the maintainer has set both, delete the two repository secrets (`gh secret delete UNITY_EMAIL -R kixel-ai/rust2`, and the same for `UNITY_PASSWORD`), dispatch the nightly on `main`, and check that its Unity job passes.
Check also that `gh api repos/kixel-ai/rust2/environments/unity/deployment-branch-policies -q '.branch_policies[].name'` lists only `main`.

## Waiting

- Maintainer (action): set the Unity account's email and password as secrets of the `unity` environment (`gh secret set UNITY_EMAIL -R kixel-ai/rust2 --env unity`, and the same for `UNITY_PASSWORD`). An agent cannot: only the maintainer has the values, GitHub never returns a secret's value, and Bitwarden Secrets Manager does not hold them. Check: `gh secret list -R kixel-ai/rust2 --env unity` lists both.
