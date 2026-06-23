# Session Handoff

**Last Updated:** 2026-06-23T19:34:48Z
**Branch:** build/conductor-0.1.0
**Status:** clean
**Last Commit:** 2026-06-23-5-command-agent-run-harness — feat: 5-command agent-run harness — conductor preflight verb + .sh/.ps1 hardening (conductor-cli)

## Position
- Done: **2026-06-23-5-command-agent-run-harness** — **Epoch 8 (CLI surface) ch2/5.** Shipped the `conductor preflight [--json]` verb (the `agent-run boot` entrypoint ch1 never built) as a serialize-and-exit over the readiness gate — a new `pipeline::readiness()` reusing the hardened `ReadbackClient::connect` + `run_preflight` **UNCHANGED** (no new spawn boundary). Hardened the `.sh`/`.ps1` harness at parity: `run` stage flags `--unit/--integration/--e2e`, `run_id` charset-validation, status/logs latest-default. P4 decisions: preflight is a **go/no-go GATE** (exit non-zero on `ready:false`, test-plan §3); stage flags thin-now. Zero core/seam model change.
- Next: **Epoch 8 ch3 — Line-oriented output rendering** (owo-colors/indicatif/comfy-table status lines + coverage table) → `/andromeda-phase` to promote + plan.

## Work done
1 NEW `conductor-cli` file (`commands/preflight.rs`); 7 MOD (`cli`/`main`/`pipeline` [+`readiness()`/`unreachable_state()`]/`commands/mod`.rs, `cli_smoke.rs` [+preflight E2E, 4-verb help], `agent-run.{sh,ps1}`). Gates: conductor-cli **8/8** · workspace **374/374** (373→374) · clippy `-D` clean · doctest ok · `cargo audit` exit 0 · smoke ✓ (preflight JSON ready:false+exit1 · boot wrapper · run --unit · both scripts parse). Goldens UNCHANGED. Code-graph 1091n/4761e (1086→1091).

## Drift resolved
**drift = 0.** 2 amendments applied: arch §Occupied Resources += `CONDUCTOR_PREFLIGHT_TIMEOUT` env var; layout-templates §cli Primary screens += `conductor preflight` verb + agent-run stage flags (first layout-templates amendment → sidecar created). **0 escalations** — the escalate-severity boundary detectors (D-security-input/subprocess · D-obs-redaction) returned CLEAN on their own (consumed-hardened-infra: reused `ReadbackClient::connect` + `init_observability` unchanged; the ch1 playbook rule held without needing to fire). tests/design/a11y clean. Cascade no-op (CLAUDE.md/stack.md don't enumerate env vars — the SERVICE_NAME/ENV precedent; layout-templates has no summary + the cli-verb doesn't touch the webview-scoped frontend.md).

## Notes
- **Key decisions:** (1) **P4** preflight = go/no-go gate — exit non-zero on `ready:false` (test-plan §3), distinct from `conductor run`'s Blocked→exit 0 (curated → verification-harness.md). (2) **P4** stage flags thin: `--unit`=nextest · `--integration`=`-E 'kind(test)'` · `--e2e`=`-p conductor-cli`. (3) Reused the hardened `ReadbackClient::connect` + `run_preflight` UNCHANGED (no new spawn boundary); the unreachable `ReadyState` synthesis duplicates verify's private `UNREACHABLE_PRECONDITION` string (verify const + `spawn::*` are `pub(crate)`). (4) `pipeline::preflight` (run/suite) left UNTOUCHED — it keeps the client; `readiness()` discards it, so they can't merge without double-spawning. (5) Hermetic preflight E2E needs the `contracts/mcp-contract.toml` fixture (readiness loads the manifest before connect — unlike the run unreachable leg).
- **Curation:** Tier 2 ×1 (verification-harness.md — the preflight-gate exit-code distinction) · Tier 3 ×1 (session-learnings.md — verb shape + manifest fixture + .ps1 gotchas). Filtered 2 (timeout host-gotcha · playbook-held). 0 conflicts, 0 deferred.
- **Follow-up (carried):**
  - **(NEW) DRY: expose `conductor_verify::readiness(data_dir, manifest, canary) -> ReadyState`** — the real-sidecar sibling of `preflight_boot` (whose doc already names the `conductor preflight` verb as a driver) — to retire the `UNREACHABLE_PRECONDITION` duplication in `pipeline.rs` (needs a `conductor-verify` touch; Epoch-9/10).
  - `scenario.run` root obs span (CLI driver) — still deferred (ch5/Epoch-10); the preflight verb reuses the `verify.readback.preflight` span + `run_id` (no new root span this chunk).
  - Two faithful content bridges → Epoch-10 (per-scenario emission fidelity + per-check read-back extraction + the faithful preflight canary; live runs stay Blocked until then).
  - `coverage-matrix.md` not yet at repo root (Epoch-8/later).
  - test-plan §3 ↔ obs-plan §3 dual-record-shape reconcile (pre-existing carried follow-up; dedicated pass — test-plan §3 is OWNER).
  - `opentelemetry-proto default-features=false` trim (dormant transitive OTel SDK).
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-06-23T19:34:48Z
