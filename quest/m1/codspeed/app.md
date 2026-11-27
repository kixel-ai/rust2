# [XS] Uninstall the CodSpeed GitHub App

## Goal

The CodSpeed GitHub App is uninstalled from kixel-ai (all repositories, installation 181428216) and moq-dev (selected repositories, installation 176003935), and the CodSpeed-side organizations and projects are deleted, so CodSpeed keeps no access to either organization or their data.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2): ✅ an agent drives Chrome with `agent-browser` for both organizations' settings and CodSpeed's dashboard (rejected: the maintainer does it; uninstall the app but keep the CodSpeed account). The API can't uninstall an app with our token, so it needs an org owner's browser session.

Done 2026-11-27: the app is uninstalled from both organizations (`gh api` lists no `codspeed` in either), and the CodSpeed projects `kixel-ai/rind`, `kixel-ai/rust2`, and `moq-dev/moq` are deleted (their runs and benchmarks are gone; the dashboard lists none of them). Only the empty organization shells remain.

Check it's done with `gh api --paginate orgs/<org>/installations --jq '.installations[].app_slug'` listing no `codspeed` in either organization, and with CodSpeed's dashboard, signed in, showing neither organization or its projects.

## Waiting

- Maintainer (action): email contact@codspeed.io to delete the CodSpeed organizations `kixel-ai` and `moq-dev`, naming both and confirming the deletion is permanent. CodSpeed has no dashboard control for it (its [data deletion](https://codspeed.io/docs/data-deletion) page routes organization deletion through support), and an agent has no mailbox of its own to send from. Check that neither organization appears in the signed-in dashboard's account switcher, then delete this quest.
