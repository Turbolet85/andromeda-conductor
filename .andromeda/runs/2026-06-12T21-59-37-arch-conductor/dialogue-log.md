# Phase 0 — Input Validation Dialogue

**User input:**

# Conductor — creator brief

> **Canonical entry point for the greenfield pipeline** (`/andromeda-arch` and every specialist read this).
> The name is reserved by the Pulse capability spec itself (§Purpose-3: "Conductor — Pulse's test harness
> companion"). **Normative companions in `.andromeda/refs/` — read them alongside this brief:**
> - `refs/pulse-capability-spec.md` — THE requirement source: every P-001..P-060 carries a
>   "Conductor verification" clause with concrete scenario parameters (post chunk-#100 amendments —
>   current as of 2026-06-12).
> - `refs/capability-verification-matrix.json` — the per-P static-verification map; four entries carry
>   explicit **Conductor dynamic-verification notes** (P-025 / P-027 / P-037 / P-045) — those are formally
>   delegated to Conductor and are first-class requirements here.
> - `refs/pulse-v0_2_0-capability-audit-2026-06-12.md` — provenance: the audit that pinned the facts
>   below; its per-P verdict tables seed Conductor's `coverage-matrix.md`.

## Problem

Pulse v0.2.0 claims 60 formal capabilities (P-001..P-060). The static layer is covered (nextest / ui /
a11y tests mapped by the verification matrix). What does NOT exist is the **dynamic layer**: nothing can
drive a real, running Pulse through its claimed behaviors — establish baselines, ramp error rates at 3.5×,
storm a fingerprint 12 times in 30s, go silent for exactly 25s — and check that the right incident
surfaces within the right SLO. The capability spec reserves a companion app for this (Conductor) and
writes a verification clause per capability; the roadmap (Phase 4) requires it before Pulse v0.3.0
performance work, and the matrix already delegates four timing/visual claims to it by name.

## What this is

**Conductor** — a scenario-driven OTLP fault-injection + verification harness for Pulse. It emits
precisely shaped telemetry (traces / metrics / logs) on a deterministic timeline, induces the fault
patterns the spec enumerates, and verifies Pulse's reaction — programmatically where a read-back channel
exists, via an operator checklist where the reaction is visual.

## Scope law (from the spec, verbatim intent)

Conductor implements **exactly enough** fault injection and scenario surface to verify each P-XXX claim —
no more (feature creep), no less (untested claims). Every scenario cites the P-IDs it verifies; a scenario
with no P-ID does not exist. New scenarios enter only when the capability spec gains new P-XXX entries.

## Target user

The Pulse developer (solo, local). Runs next to a real Pulse instance on the dev host (roadmap Phase 4:
dynamic capability validation; Phase 5: the workload source for performance measurement).

## Capabilities (0.1.0)

1. **Timeline engine** — scenarios are declarative phase sequences (per phase: duration · per-service
   emission spec: span rate, error fraction, latency distribution targets, exception emission, log
   severity mix, silence). Deterministic under a seed: same scenario + seed ⇒ same stream shape.
   Wall-clock-stamped **emission journal** per run = ground truth of what was sent when (the left side of
   every SLO check).
2. **Emission primitives** (each exists because named P-clauses need it):
   - spans with controlled `Status.Code=ERROR`, **root vs child** error placement (P-005, P-008);
   - exception span events with synthesized type/message/stacktrace and **fingerprint control** —
     identical stack · path-variant · line-variant (P-006, P-017, P-018); per the amended P-017
     clause (c), the line-variant case expects an **IDENTICAL** fingerprint (line-number insensitivity);
   - log records with controlled `SeverityNumber` across the 17-boundary (P-007);
   - latency shaping to target p50/p95/p99 per operation (P-011 — note Pulse's latency exclusion floor is
     50 samples, distinct from the error-rate floor of 10; P-012);
   - multi-service attribution via `service.name` over a configurable virtual topology with W3C trace
     propagation across emulated call edges (P-008, P-027, baselines per service);
   - PII payload corpus covering the seven P-047 categories (emails, JWT, bearer, API keys, credit cards,
     SSN, secret-like key=value) (P-047, P-035, P-048);
   - traffic-rate ramps for halo breathing observation (P-026).
3. **Fault helpers**:
   - port-occupier: hold Pulse's ingest port **:4317** with a sacrificial listener before Pulse starts
     (P-003 ReceiverFailed);
   - emission gap/resume with exact gap lengths (P-015 restart detection: gap >20s + resume);
   - abrupt permanent silence (P-014); bursty train pattern — active 5min / quiet 10min repeating (P-013
     activity-floor training, "developer-on-lunch" test).
4. **Verification layer**:
   - per-scenario **expected-outcome blocks** (what Pulse must do + the SLO window, from the spec);
   - **MCP read-back client** over Pulse's tested tool surface — `query_incident_list` ·
     `retrieve_report` (markdown + degraded_mode flag) · `retrieve_telemetry_slice` ·
     `mark_incident_resolved`. Operator prerequisite (documented, never silently assumed): Pulse built
     with the non-default `mcp-server` cargo feature AND runtime env `ANDROMEDA_PULSE_MCP_ENABLED`;
   - **assertion policy split** (the spec's own boundary): deterministic claims (hard signals, baseline
     math, suppression/bypass logic, lifecycle timing) = hard pass/fail; model-interpretive claims
     (severity choice, hypothesis quality, P-008 root-vs-deep weighting — model-side per the amended
     spec) = calibration-region checks + report-for-human, never hard-failed on exact values;
   - **run report**: per scenario — pass / fail / manual-check / **known-residual** (a documented
     spec-vs-runtime gap expected to fail until a named fix lands — P-032 ships as the first instance),
     with the emission journal cross-referenced; manual-check items render as an operator checklist
     ("halo shifted toward burgundy?", "no OS toast appeared?") with induced state + expected observation;
   - **operator pauses**: scenarios needing actions Conductor must not perform itself (restart Pulse
     mid-baseline for P-009/P-013 persistence; config edits for P-052/P-055/P-056; acknowledge an
     incident for P-023; model on/off for P-020/P-053) pause with an explicit instruction and resume on
     confirmation.
5. **Control panel (minimal UI)** — scenario/suite picker, start/stop, live emission counters + target
   status, the run report view, operator-pause prompts. A control surface, not a dashboard.

## Scenario catalog (0.1.0 — grouped; parameters are the spec's, cited by P-ID)

| Scenario family | P-IDs | Key shape (from the spec) |
|---|---|---|
| connection-lifecycle | P-001..P-004 | no-emit (Listening) → emit (Receiving ≤1s) → stop (Idle 10–60s → Stalled >60s); port-occupier on :4317 → ReceiverFailed ≤2s; error-burst + clean stop (orthogonality) |
| hard-signals | P-005..P-008 | ERROR spans (≤500ms p99 to candidate); exception events; SeverityNumber 17-boundary; child-only vs root error traces (root-vs-deep = calibration-region, model-side weighting) |
| error-baseline-spike | P-009, P-010 | sustained known rate 90s (±10% baseline convergence, ≥10 spans/min floor) → ramp 3.5× for 60s → candidate ≤2s after 30s persistence |
| latency-regression | P-011, P-012 | known p50/p95/p99 for 90s (±15%, ≥50-sample floor) → 3× p99 for 90s → candidate after 60s persistence |
| activity-floor | P-013, P-014 | bursty train (5min/10min) training + 30min lunch test (no false silent); fresh service: 5min active → abrupt stop → cue at 30s |
| restart-suppression | P-015, P-016, P-057 | gap 25s + resume → RestartEvent ≤2s; in 60s window: 15s burst suppressed, 60s sustained surfaces; bypass triple: 8×/3% suppressed · 12×/4% relative-bypass · 6×/7% absolute-bypass |
| fingerprint-storm | P-017, P-018 | fingerprint triple: identical / path-variant / line-variant — ALL THREE identical fp (amended clause (c)); type-variant + frame-variant → different; 6 hits per 30s → Suggested cue, 12 → Autonomous hint |
| severity-lifecycle | P-019..P-023, P-059, P-060 | tier-calibrated inputs; cease signal → auto-resolve at 120s + resolution summary; ack → retrigger 1min (no new) vs 6min (new); sustained 10min incident → multiple interpretation updates; per-tier SLO timing (<5s / <20s / <90s, hardware-profile-aware) |
| constellation | P-025, P-026, P-027 | new `service.name` → dot ≤5s (matrix-delegated to Conductor); hue shift ≤2s pipeline latency (delegated); stable positions across Pulse restart; >20 services cap; rate ramps for breathing |
| context-grounding | P-032, P-036 | run inside a real git workspace with known commits → trigger incident → Report context shows last-5 commits (**known-residual**: production recent_commits producer is a stub until v0.3.0 — Conductor is its designated detector); recur a fingerprint across two sessions → "Previously seen" populated |
| pii-scrub | P-035, P-047, P-048 | all seven PII categories through spans/logs/exceptions → corpus + Report excerpts scrubbed, structure preserved |
| pipeline-ops | P-052, P-055, P-056 | known cadence config (10s/5s/60s) → L4 invocation rate matches; threshold hot-reload mid-run → prospective-only (no retroactive cues), opt-in backfill produces them |
| degraded-mode | P-020, P-053 | same workload with model disabled / fallback tier (operator-set) → hard signals still surface as Suggested; Report structure matches reduced contract |
| report-surfaces | P-037, P-045 | report render <2s + counter refresh ≤1s — matrix-delegated timing claims; measured via MCP poll timing + operator observation |

## Coverage classification (what "verify all functionality" means here)

Conductor's repo carries `coverage-matrix.md` classifying every P-001..P-060 as:
- **auto** — Conductor drives AND asserts (journal + MCP read-back + timing);
- **drive+observe** — Conductor induces the state; the operator confirms the visual/UX claim via the
  generated checklist (halo hue/breathing, widget invariants, dropdown order, no-notification);
- **static-only** — no dynamic telemetry dimension (e.g. P-038 clipboard, P-040 MCP-independence, P-046
  export UI, P-049 encryption-at-rest, P-051 storage UI, P-054 four-host matrix); these stay with Pulse's
  own test matrix — Conductor explicitly does NOT duplicate them, but provides the workload when their
  verification needs one.
Seed the classification from the audit's verdict tables (`refs/pulse-v0_2_0-capability-audit-*.md`).
A P-XXX missing from the matrix is a defect. "0.1.0 done" requires the matrix complete with zero
unclassified entries.

## Non-goals (0.1.0 scope guard)

- NOT a load-tester: bounded "typical / high" load profiles only (P-060 SLO checks); 50k+ spans/sec
  saturation regimes are explicitly out (the spec excludes them from Conductor scenarios; Pulse's own
  `perf_load_profiles.rs` suite owns saturation).
- NO Pulse process management — Conductor never starts/stops/restarts Pulse or edits its config; those
  are operator-pause steps. (Keeps Conductor honest: it validates Pulse from the outside, as a real
  workload.)
- NO UI automation of Pulse (Playwright/axe live in Pulse's own suites) — visual claims are operator
  checklist items.
- NO scenario DSL — declarative config files + the built-in catalog; new behavior = new P-XXX first.
- NO multi-target / distributed generation; no cloud; local dev host only.

## Constraints / stack intent

- Rust 2024 + tokio; Tauri 2 for the control panel — Pulse-consistent (conventions reuse; the future
  Andromeda memory-parser meets the same stack it will meet in Pulse).
- **OTLP transport: gRPC to 127.0.0.1:4317 (tonic/prost)** — pinned by the audit: Pulse's ingest exposes
  gRPC only, loopback-bound; there is no HTTP receiver. Port-occupier targets :4317.
- Reference prior art in the Pulse repo (read, don't vendor): `crates/ingest/examples/inject_demo.rs`
  (minimal injector) and `crates/ingest/examples/load_profiles.rs` (the chunk-#99 live injector that
  drove 10k spans/s against :4317) — proven emission patterns to mine.
- Environment: P-032/P-036 scenarios run inside a real git workspace with known recent commits
  (documented per-scenario environment requirements).
- Development Style: **agent-driven** (headless-drivable core + `scripts/agent-run.sh`; the UI is a thin
  shell over the same commands).

## What "0.1.0 done" means

- `coverage-matrix.md` complete: all 60 P-IDs classified auto / drive+observe / static-only, zero gaps.
- Every catalog scenario runs from the control panel AND headless; deterministic under a fixed seed;
  emission journal written per run.
- Run report distinguishes pass / fail / manual-check / known-residual; manual checklist renders with
  induced-state context; P-032 reports as known-residual (not as a surprise failure).
- End-to-end proof against a real Pulse: at minimum `error-baseline-spike`, `fingerprint-storm`,
  `restart-suppression` (incl. one bypass case), `pii-scrub`, and `connection-lifecycle` produce verified
  expected outcomes (MCP read-back where applicable), and one full `severity-lifecycle` pass observes
  auto-resolve + resolution summary.

**Clarifications (if any):**
- None — input was a complete pre-seeded creator brief (.andromeda/input.md, self-declared canonical entry point); both extractability checks (product type, research signal) passed on first read.

**Confirmed summary:**
- Type: Desktop app (Tauri 2 control panel) over a headless-drivable core — developer test harness
- Core idea: Conductor — a scenario-driven OTLP fault-injection + verification harness that drives a live Pulse instance through its 60 claimed capabilities (P-001..P-060) and verifies the reactions within their SLOs.
- Key aspects: deterministic seeded timeline engine + emission journal; P-ID-bound scenario catalog + coverage matrix (auto / drive+observe / static-only); MCP read-back verification with assertion policy split and known-residual class; pinned stack Rust 2024 + tokio + Tauri 2 + tonic/prost; agent-driven dev style, local-only, no Pulse process management

**Slug:** conductor

# Phase 3 — Quiz I Dialogue

**Pre-filled confirmations (batch, all confirmed unchanged):**
- Scale Intent: personal — confirmed
- Platform: Desktop app (Tauri 2 control panel) over a headless-drivable core — confirmed
- Primary Language: Rust 2024 + tokio — confirmed
- Target Users: The Pulse developer (solo, local) — confirmed
- Development Style: agent-driven — confirmed
- Core Functionality: scenario-driven OTLP fault-injection + verification harness for Pulse P-001..P-060 — confirmed

**Core field answers (no research):**
- Q: Growth Model — options: 1) Modular monolith, 2) Monolith, 3) Plugin/extension system; soft default ① (solo architect + multi-surface scope → Cargo-workspace modular monolith)
  A: 1 (Modular monolith)

**Research-derived field answers:**
- Q: Timing-Tolerance Model
  Stakes: how every wall-clock SLO assertion absorbs jitter — wrong choice = flaky failures or masked regressions.
  Research findings:
    - Options: 1) Fixed absolute slack band per tier, 2) Measured-baseline per-run calibration, 3) Statistical percentile over repeated trials, 4) Error-budget burn-rate, 5) Hybrid journal-relative deadline + tier-scaled slack split by assertion class
    - Recommended: 5
    - Reasoning: anchors to emission journal already chosen as ground truth; folds tolerance into the spec's assertion-policy split; single-seed acceptance gate can't pay for N-trial percentiles or fleet burn-rates; option 2 is the additive upgrade path.
  User response: accepted
  Final answer: Hybrid journal-relative deadline + tier-scaled bounded slack, split by assertion class
- Q: Read-Back Channel Dependency Posture
  Stakes: behavior when Pulse MCP surface is absent/unresponsive/version-mismatched; gates auto vs drive+observe split.
  Research findings:
    - Options: 1) MCP-required fail-fast, 2) MCP-preferred silent operator-fallback, 3) Version-pinned MCP contract + preflight gate + per-scenario explicit degrade with "blocked" state, 4) Capability-handshake presence-only gate
    - Recommended: 3
    - Reasoning: agent-driven runs are often unattended — silent downgrade or version-blind handshake risks false-green certification (Figma/Claude Desktop April 2026 version-mismatch case); blocked state keeps preconditions separate from pass/fail/manual-check/known-residual.
  User response: accepted
  Final answer: Version-pinned MCP contract + preflight gate, per-scenario explicit degrade, "blocked" report state
- Q: Probabilistic-Assertion Policy
  Stakes: run-report taxonomy for model-interpretive claims; wrong policy = flaky certification or buried regressions.
  Research findings:
    - Options: 1) Two-state split (hard / calibration-region), 2) Three-state (hard / soft-warn / report-only), 3) Two-state with banded region metadata, 4) Single-state advisory
    - Recommended: 1
    - Reasoning: the brief's stated policy verbatim; consistent with accepted Timing-Tolerance Model routing to calibration-region; scope law says "exactly enough — no more"; field practice standardizes on deterministic-hard / interpretive-calibration.
  User response: accepted
  Final answer: Two-state split (hard-fail / calibration-region)
- Q: Run-History & Artifact Persistence
  Stakes: file-only per run vs embedded queryable index; affects seed-to-seed comparison, P-036 recurrence, agent-readability.
  Research findings:
    - Options: 1) File-only (JSONL + Markdown), 2) Files + rusqlite 0.31 SQLite index, 3) Files + redb 2.x KV, 4) Files + sqlx async SQLite
    - Recommended: 2
    - Reasoning: P-036 cross-session recurrence is genuine cross-run state; rusqlite is sync/append-mostly fitting tokio harness without async DB layer; JSON1 queries journal slices; files stay agent-parseable ground truth.
  User response: accepted
  Final answer: File artifacts (JSONL journal + Markdown report) + embedded rusqlite index (runs.db)

**Final summary confirmed:** yes

# Phase 6 — Quiz II Dialogue

**Pre-filled confirmations (batch, all confirmed unchanged):**
- Language: Rust 2024 + tokio 1.48.x (flavor left open) — confirmed
- Outbound: OTLP/gRPC → 127.0.0.1:4317 via tonic 0.14.6 / tonic-prost / prost 0.14 — confirmed
- Inbound: MCP read-back, version-pinned manifest + initialize preflight + blocked state — confirmed
- Core↔UI: Tauri 2 (2.10.x) commands + Channel — confirmed
- Storage model: JSONL journal + Markdown report + thin SQLite runs.db (access layer left open) — confirmed
- Messaging: none (in-process mpsc) — confirmed
- Deployment: local cargo build --release + scripts/agent-run.sh; GH Actions build+test only — confirmed
- Mobile / AI-ML / Push: N/A / N/A / in-app Channel only — confirmed
- Deferred to specialists: frontend (design), auth (security), tests, logging (obs), a11y — confirmed

**Fork decisions:**
- Q: Tokio Runtime Flavor
  Stakes: emission-ordering reproducibility under seed vs throughput headroom.
  Research basis: research-targeted.md § Backend Framework Comparison + § Key Trade-offs #2 (tokio 1.48.x) + input.md (Tauri's own runtime; ~10k spans/s prior art)
  Research findings: 1) current_thread (core-owned), 2) multi_thread fixed pool(2), 3) multi_thread default; Recommended: 1; Downstream forks: Q2, Q3, Q7 — KEYSTONE
  User response: accepted
  Final answer: current_thread, core-owned runtime per entry point
- Q: Workspace / Core-Structure Shape
  Stakes: compiler-enforced headless/CLI/GUI split; home of runtime ownership.
  Research basis: research-targeted.md § Backend Framework + § Module Boundary / Trade-off #4 + v2.tauri.app/concept/architecture + Pulse crates/… layout
  Research findings: 1) workspace core lib + CLI bin + Tauri bin, 2) single crate + feature split, 3) two-crate coarse split; Recommended: 1; Downstream forks: Q9, Q10, Q4 — KEYSTONE
  Note: first sub-agent returned a meta-summary instead of the output block — retried once with identical context, block returned.
  User response: accepted
  Final answer: Cargo workspace — core lib + agent-run CLI bin + Tauri bin
- Q: OTLP Emission Strategy (rubber-stamp — presented directly from Phase 4 research, single viable option)
  Stakes: whether fault injection is possible at all (fingerprint/severity/placement control).
  Research basis: research-targeted.md § Key Trade-offs #1 + § API Style (opentelemetry-proto 0.32.0 released 2026-05-08)
  Findings: 1) opentelemetry-proto raw types, 2) opentelemetry-otlp SDK exporter (rejected: near-deal-breaker); Recommended: 1
  User response: accepted
  Final answer: opentelemetry-proto 0.32.0 raw types over tonic 0.14.6
- Q: SQLite Access Layer (rubber-stamp — Quiz I pinned semantics; version ratification)
  Stakes: runs.db access seam + stale version pin.
  Research basis: research-targeted.md § Database Comparison + Quiz I § Run-History
  Findings: 1) rusqlite 0.38.0 bundled (bump 0.31→0.38.0), 2) sqlx 0.8.6 (rejected), 3) SeaORM 2.0 (rejected); Recommended: 1
  User response: accepted (version bump ratified)
  Final answer: rusqlite 0.38.0 + bundled SQLite 3.51.1 (JSON1)
- Q: MCP Read-Back Client Idiom
  Stakes: who owns JSON-RPC plumbing + version negotiation; error mapping into blocked/fail/harness-error.
  Research basis: research-targeted.md § API Style + WebSearch/WebFetch 2026-06-12 — rmcp 1.7.0 verified on docs.rs (serve_client, peer_info, TokioChildProcess, ProtocolVersion::V_2025_11_25)
  Findings: 1) rmcp 1.7.0, 2) hand-rolled JSON-RPC (jsonrpsee 0.26 / serde_json), 3) rust-mcp-sdk 0.x; Recommended: 1; Downstream forks: Q8, Q10 — KEYSTONE
  User response: accepted
  Final answer: rmcp 1.7.0 (official Rust MCP SDK)
- Q: Config Validation Library
  Stakes: scenario-config invariants + contract manifest; cross-field rules.
  Research basis: research-targeted.md § Validation Library + docs.rs/github jprochazk/garde (range + context-passing custom verified)
  Findings: 1) serde + garde 0.23.0, 2) serde + validator 0.20.x, 3) serde + serde_valid (niche obsoleted by rmcp typed payloads); Recommended: 1; Downstream forks: Q9, Q10 — KEYSTONE
  User response: accepted
  Final answer: serde 1.0.x + garde 0.23.0
- Q: Module Boundary Enforcement (rubber-stamp — second face of workspace fork)
  Stakes: compiler-enforced vs advisory seams.
  Research basis: research-targeted.md § Module Boundary Enforcement + Quiz II Fork 2 + Quiz I § Growth Model
  Findings: 1) full crate-per-seam (conductor-timeline · -emit · -faults · -verify · -report + conductor-cli + Tauri bin), 2) coarse core-lib-only split; Recommended: 1
  User response: accepted
  Final answer: full crate-per-seam workspace
- Q: Error Handling Pattern
  Stakes: harness failures vs verification outcomes; report-state mapping.
  Research basis: research-targeted.md § API Style (error-handling fork) + § Key Trade-offs #5
  Findings: 1) thiserror 2.0.18 + anyhow 1.0.102, 2) snafu 0.8.x, 3) miette 7.x over thiserror, 4) option 1 + type-level Verdict/ReportState wall (verdicts as values, Err = harness failures only); Recommended: 4; Downstream forks: none (final)
  User response: accepted
  Final answer: thiserror 2.0.18 per-seam + anyhow 1.0.102 edges + Verdict/ReportState value types

**Final summary confirmed:** yes
