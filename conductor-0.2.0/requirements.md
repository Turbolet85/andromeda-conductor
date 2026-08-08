# Conductor 0.2.0 — Requirements

_The capability set 0.2.0 must deliver, derived from `conductor-0.2.0/intent.md` §4 (findings F1–F22).
One numbered capability per line: `{id} · {title} — {EXPECT} (per intent {§finding})`. Anchors Phase 0
calibration and is reused verbatim as the `verification-matrix.json` ids; not a spec dump._

**Id scheme — `v2-NN` (version-scoped).** 0.1.0's `requirements.md` carries no numbered ids, so there is
no project scheme to continue. The `P-NNN` numbering in `.andromeda/refs/pulse-capability-spec.md` is the
**SUT's** id space (Pulse's, live ledger already at P-082) and is deliberately NOT continued here. Pulse
P-IDs appear below only inside acceptance criteria, and in the repo only in `coverage.rs` and scenario
configs — never as Conductor's own ids, never as matrix entries (per intent §6).

## Theme 1 — Synchronization with the SUT (keystone)

- **v2-01** · SUT capability set sourced from a versioned artifact — the accepted capability set tracks the SUT via a versioned artifact under `refs/`, so a Pulse release is a data update rather than a code change (per intent §4 F1).
- **v2-02** · SUT-drift check — a check fails loudly when the SUT advances past what Conductor knows, rather than silently accepting a stale universe (per intent §4 F1).
- **v2-03** · Full current-SUT capability classification — every current Pulse capability is classified auto / drive+observe / static-only / not-Conductor's, with the not-Conductor's boundary recorded as a deliberate decision (per intent §4 F2, §5 boundary).
- **v2-04** · In-lane SUT-capability scenarios — the capabilities in Conductor's lane are covered by scenarios, proving Pulse P-067, P-072 and P-079 (per intent §4 F2).
- **v2-05** · Interpretation-correctness posture — a recorded decision on whether 0.2.0 carries a real-model correctness leg or explicitly defers it with a named owner, so determinism never silently redefines what the harness proves (per intent §4 F3).
- **v2-06** · SUT load-envelope constraint — scenario durations and storm profiles stay inside the SUT's proven-good envelope, with the constraint recorded where scenario authors will see it (per intent §4 F4).
- **v2-07** · Over-envelope run flagged environment-suspect — a run exceeding the SUT's proven-good envelope is reported as environment-suspect rather than as a Pulse failure (per intent §4 F4).

## Theme 2 — The live proof

- **v2-08** · Faithful per-phase emission dispatcher — a per-phase dispatcher over the `conductor-emit` primitives emits each scenario's declared shape, extensible to the deferred families (per intent §4 F5).
- **v2-09** · Real per-check read-back extraction — per-check observed values are extracted from `query_incident_list` / `retrieve_telemetry_slice` and fed to the existing `evaluate_check` → `classify` path unchanged, with `execute_scenario`'s signature preserved (per intent §4 F6).
- **v2-10** · Live `ready: true` — a live preflight against a real Pulse in deterministic-L4 mode reaches `ready: true`, recorded with its journal + `runs.db` evidence (per intent §4 F7).
- **v2-11** · `fingerprint-storm` family live-proven — the family emits faithfully, reads back through MCP and yields a verified expected outcome, proving Pulse P-017/P-018 plus P-074's exactly-one-incident claim (per intent §4 F8, F2).
- **v2-12** · `error-baseline-spike` family live-proven — the family emits faithfully, reads back through MCP and yields a verified expected outcome, proving Pulse P-009..P-012 (per intent §4 F8).
- **v2-13** · `restart-suppression` family live-proven — the family emits faithfully, reads back through MCP and yields a verified expected outcome incl. one bypass case, proving Pulse P-015/P-016/P-057 (per intent §4 F8).
- **v2-14** · `pii-scrub` family live-proven — the family emits faithfully, reads back through MCP and yields a verified expected outcome, proving Pulse P-035/P-047/P-048 (per intent §4 F8).
- **v2-15** · `connection-lifecycle` family live-proven — the family emits faithfully, reads back through MCP and yields a verified expected outcome incl. the `PortOccupier` fault, proving Pulse P-001..P-004 (per intent §4 F8).
- **v2-16** · `severity-lifecycle` full pass — one full pass observes auto-resolve and the resolution summary via read-back (per intent §4 F9).

## Theme 3 — The cross-project blocker (external prereq)

- **v2-17** · Workspace-key divergence detected — Conductor detects an app/sidecar workspace-key mismatch and fails with a named precondition rather than an opaque `Blocked`, its verdict established by the two-launch check (repo root vs marker-less temp dir) (per intent §4 F10).
- **v2-18** · Machine-checked Pulse run contract — a recorded, reproducible run contract that preflight asserts and whose unmet terms it names, covering `ANDROMEDA_PULSE_L4_DETERMINISTIC` on the launched `pulse-app` and a shared data dir; doubles as the verification of Pulse P-073 (per intent §4 F11, F2).

## Theme 4 — The four delegated timing capabilities

- **v2-19** · Sub-5s budget representation + per-check latency — budgets below the closed 5s/20s/90s tier set are representable and latency is measured per check rather than as the coarse whole-loop delta (per intent §4 F12).
- **v2-20** · Four delegated budgets return real Pass/Fail — P-025 ≤2s, P-027 ≤5s, P-037 ≤2s and P-045 ≤1s each yield a real Pass/Fail at its actual value rather than a ManualCheck, closing Pulse's P-075 (per intent §4 F12).

## Theme 5 — Conductor's own verification ledger

- **v2-21** · Conductor's verification ledger — 0.2.0 carries a `verification-matrix.json` tracking Conductor's own delivery, with the `v2-NN` and `P-NNN` id spaces kept visibly separate (per intent §4 F13, §6).

## Theme 6 — Ship and close (0.1.0's migrated definition of done)

- **v2-22** · Desktop a11y sweep asserted — the shipped, display-gated real-webview specs run on Linux+xvfb against a live Pulse and assert zero axe violations, token-pair contrast, and keyboard-trap / focus-order across the four accessible paths, without re-authoring the harness (per intent §4 F14).
- **v2-23** · Screen-reader manual spec — an NVDA/VoiceOver manual spec accompanies the automated sweep (per intent §4 F14).
- **v2-24** · A11y CI gate + violation JSON — axe / contrast / keyboard PASS/FAIL is emitted into the obs envelope, service-tagged and gated in CI, with the redaction boundary applied to Conductor's own violation artifact (per intent §4 F15).
- **v2-25** · Cross-surface envelope parity — CLI and Tauri produce an identical envelope for the same seed (per intent §4 F16).
- **v2-26** · Coverage completeness gate — a gate fails on any gap in the classification of the current SUT capability set, with all CI gates green (per intent §4 F17).
- **v2-27** · Release build + bundle — `cargo build --release` plus a Tauri 2 bundle ship with a final SLO verification pass (per intent §4 F18).

## Theme 7 — Housekeeping carried forward

- **v2-28** · Stale `rmcp` wording reconciled — `test-plan.md`, `tests-summary.md` and `verification-harness.md` describe the hand-rolled JSON-RPC client that replaced rmcp (per intent §4 F19).
- **v2-29** · Operator pause + checklist exercised live — the operator-pause hold and the operator-checklist live items both fire during the live legs (per intent §4 F20).
- **v2-30** · Live per-P-ID verdict lamps — the coverage view carries live per-P-ID verdict lamps (per intent §4 F21).
- **v2-31** · `scenario.run` root obs span — a `scenario.run` root span anchors the self-observation trace (per intent §4 F21).
- **v2-32** · Dormant maintenance — `indicatif` 0.17→0.18 and the `opentelemetry-proto default-features=false` trim land inside a polish chunk rather than as chunks of their own (per intent §4 F22).

## Carried residuals (NOT 0.2.0 capabilities — deliberately unnumbered, no matrix entry)

- **`secret-scanning-ci-gate` — unrealized security-plan bootstrap item.** `security-plan.md` §Bootstrap phases + §Secret Management name a secret-scanning CI gate (no secret-shaped string in the `conductor-*` workspace; `*.p12`/`*.pem`/`*.cer` ignores; build red on any hit). It is absent from `.github/workflows/ci.yml` and from `.gitignore`, and has now survived a full version un-built. Surfaced by the security validator during the 0.2.0 route run and **deliberately excluded** from this version: it is outside the authored intent and Conductor owns no secrets by design. Recorded here because **no drift detector will surface it** — detectors compare the chunk report against the plan-of-record, not the plan against reality — so this line is the only thing keeping it visible. Revisit at 0.3.0 scoping.

