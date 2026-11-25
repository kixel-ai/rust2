# [XS] Uninstall the CodSpeed GitHub App

## Goal

The CodSpeed GitHub App is uninstalled from kixel-ai (all repositories, installation 181428216) and moq-dev (selected repositories, installation 176003935), and the CodSpeed-side organizations and projects are deleted, so CodSpeed keeps no access to either organization or their data.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2): ✅ an agent drives Chrome with `agent-browser` for both organizations' settings and CodSpeed's dashboard (rejected: the maintainer does it; uninstall the app but keep the CodSpeed account). The API can't uninstall an app with our token, so it needs an org owner's browser session.

Check it's done with `gh api --paginate orgs/<org>/installations --jq '.installations[].app_slug'` listing no `codspeed` in either organization, and with CodSpeed's dashboard, signed in, showing neither organization or its projects.

## Waiting

- kixel-ai/rind: [kixel-ai/rind#1013](https://github.com/kixel-ai/rind/pull/1013) merged, since rind's weekly bench uploads to CodSpeed until then.
