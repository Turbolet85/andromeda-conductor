# 1A Decomposition Tree — conductor-0.2.0

_Phase 1 debug artifact. Sub-block tier visible here; dropped at 1C linearization._
_Each leaf: candidate title + `(source: …)`. Capability ids map to `conductor-0.2.0/requirements.md`._

## Epoch 1 — Foundation: re-aim at the SUT (depth 2 → 5 chunks)

### Sub-block 1.A — SUT capability source & drift
- SUT capability manifest (source: intent §4 F1 EXPECT — versioned `refs/` artifact; precedent `.andromeda/refs/pulse-v0_2_0-capability-audit-2026-06-12.md`; replaces `conductor-core/src/scenario.rs` `(1..=60)`) → **v2-01**
- SUT-drift check (source: intent §4 F1 EXPECT second clause — "fails loudly when the SUT advances past what Conductor knows") → **v2-02**

### Sub-block 1.B — Classification & in-lane coverage
- Current-SUT coverage classification incl. recorded boundaries (source: intent §4 F2 + §5 explicit boundary; arch §Scope law; input.md §Coverage classification; carries F3's posture record) → **v2-03**, **v2-05**
- In-lane SUT scenarios (source: intent §4 F2 "the ones that fall squarely in Conductor's lane"; arch §Scope law "no scenario without a P-ID") → **v2-04**

### Sub-block 1.C — Load-envelope guard
- SUT load envelope + environment-suspect flagging (source: intent §4 F4 — DuckDB append stall after ~10 min sustained 10k/s under deterministic L4; test-plan §1 "NOT a load-tester" bound) → **v2-06**, **v2-07**

_(v2-21 Conductor's verification ledger — by-construction, discharged by route's Phase 6 matrix emission; no chunk.)_

## Epoch 2 — Live-path enablement (depth 1 → 5 chunks)

- Workspace-key divergence probe (source: intent §4 F10 — Pulse-side defect, two-launch check is "the first action of this version"; arch §Read-Back Dependency Posture Blocked-precondition rule) → **v2-17**
- Pulse run contract (source: intent §4 F11 — `ANDROMEDA_PULSE_L4_DETERMINISTIC` on the launched app not the sidecar; doubles as Pulse P-073 verification; arch §Standard Contracts readiness gate) → **v2-18**
- Faithful emission dispatcher (source: intent §4 F5 — replaces `conductor-run::coarse_emit`; arch `conductor-emit` primitives) → **v2-08**
- Per-check read-back extraction (source: intent §4 F6 — replaces the `"incidents-listed"` placeholder; `evaluate_check` → `classify` unchanged, `execute_scenario` signature preserved) → **v2-09**
- First live green preflight (source: intent §4 F7 — headline acceptance of the 0.1.0 e2e chunk, never achieved; test-plan §3 `boot` readiness signal) → **v2-10**

## Epoch 3 — Live proof: the five families (depth 1 → 5 chunks)

- fingerprint-storm live proof (source: intent §4 F8 + F2 P-074; test-plan §6 Scenario "Fingerprint-storm"; obs-plan §4 fingerprint-storm must-trace path) → **v2-11**
- error-baseline-spike live proof (source: intent §4 F8; test-plan §6 Scenario "Headless deterministic scenario run"; input.md scenario catalog) → **v2-12**
- restart-suppression live proof (source: intent §4 F8 incl. one bypass case; test-plan §6 Scenario "Restart-suppression"; obs-plan §4 bypass path) → **v2-13**
- pii-scrub live proof (source: intent §4 F8; input.md scenario catalog pii-scrub; test-plan §6 auto-scenario instance of Path 1) → **v2-14**
- connection-lifecycle live proof (source: intent §4 F8 incl. `PortOccupier`; input.md scenario catalog; security-plan Vector 6 port-occupier) → **v2-15**

## Epoch 4 — Lifecycle & delegated timing (depth 1 → 4 chunks)

- severity-lifecycle live proof (source: intent §4 F9; test-plan §6 Scenario "Severity-lifecycle full pass"; obs-plan §4 severity-lifecycle must-trace path) → **v2-16**
- Per-check latency measurement (source: intent §4 F12 first clause — `SloTier` closed set vs sub-5s budgets; obs-plan §5 `latency_ms` / `slo_tier` model) → **v2-19**
- Delegated timing budgets proven (source: intent §4 F12 second clause — closes Pulse P-075; input.md refs `capability-verification-matrix.json` four delegated entries) → **v2-20**
- Operator-pause + checklist live firing (source: intent §4 F20; arch §Occupied Resources operator-pause command; a11y-plan §1 operator-pause + checklist paths) → **v2-29**

## Epoch 5 — Verification surfaces (depth 2 → 6 chunks)

### Sub-block 5.A — A11y execution & gate
- Desktop a11y sweep (source: intent §4 F14 — harness already shipped, RUN don't re-author; a11y-plan §3 harness contract + §6 contrast pairs; test-plan §9 Linux+xvfb E2E job) → **v2-22**
- Screen-reader manual spec (source: intent §4 F14; a11y-plan §3 Screen reader test pattern — manual, supplemental, never sole) → **v2-23**
- A11y CI gate (source: intent §4 F15 — reuse the `ci.yml` obs-gate scaffold; a11y-plan §3 CI integration + violation JSON; obs-plan §6 envelope) → **v2-24**

### Sub-block 5.B — Cross-surface parity
- Cross-surface envelope parity + stale `rmcp` doc reconcile (source: intent §4 F16 + F19; test-plan §6 Scenario "Both-surface parity" / Critical Path 7; obs-plan §4 both-surface parity path) → **v2-25**, **v2-28**

### Sub-block 5.C — Carried obs/GUI surfaces
- Live per-P-ID verdict lamps (source: intent §4 F21; a11y-plan §4 coverage-matrix pattern; design lamp never-color-alone rule) → **v2-30**
- `scenario.run` root span tree (source: intent §4 F21; obs-plan §4 must-trace span tree + span naming convention) → **v2-31**

## Epoch 6 — Polish & ship (depth 1 → 3 chunks)

- Coverage completeness gate (source: intent §4 F17 — zero-gap over the CURRENT SUT set per F1/F2, no longer a hardcoded 60; test-plan §6 Scenario "Coverage-matrix completeness gate") → **v2-26**
- Dependency polish (source: intent §4 F22 — explicitly "folded into a polish chunk, not a chunk of their own") → **v2-32**
- Release build and bundle (source: intent §4 F18; arch §Deployment local release + Tauri 2 bundler; security-plan §Dependency Security audit-green precondition) → **v2-27**

---

**Leaf total: 28 chunks** across 6 epochs (5 / 5 / 5 / 4 / 6 / 3) — Phase 0 estimated 24 (+16.7%, within the ±20% band).
