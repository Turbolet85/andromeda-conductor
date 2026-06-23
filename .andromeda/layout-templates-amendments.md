# Layout Templates — Amendments

_Append-only changelog of amendments to `layout-templates.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-23-5-command-agent-run-harness — conductor preflight verb + agent-run stage flags registered
**Section:** §Surface: cli — Primary screens (commands)
**Change:** added the `conductor preflight [--json]` verb (the readiness gate / `agent-run boot` entrypoint; exits 0 iff ready:true, else non-zero) to the cli verb list, and named the `agent-run.sh` 5-command set + the `run` `--unit`/`--integration`/`--e2e` stage flags.
**Why:** the chunk landed the `conductor preflight` verb (cli verb surface 3→4) + the `run` stage flags (report §Changes); §cli Primary screens listed only run/suite/report + a generic `agent-run.sh` pointer. The preflight verb is a genuinely-new this-chunk surface (the `report`-verb precedent registered at ch1); D-layout-surface routine. No cascade — layout-templates has no specialist-summary doc and the cli-verb addition does not touch the frontend.md rule (webview-scoped). First amendment to layout-templates.
