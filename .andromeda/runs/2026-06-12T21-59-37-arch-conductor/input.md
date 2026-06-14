## User Input

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

## Extracted

- Product type: Desktop app (Tauri 2 control panel) over a headless-drivable core — a developer test harness (verification companion app), not an end-user product
- Core idea: Conductor — a scenario-driven OTLP fault-injection + verification harness that drives a live Pulse instance through its 60 claimed capabilities (P-001..P-060) and verifies the reactions within their SLOs.
- Key aspects:
  - Deterministic seeded timeline engine emitting shaped traces/metrics/logs over gRPC to 127.0.0.1:4317, with a per-run wall-clock emission journal as ground truth
  - Scenario catalog strictly bound to P-IDs (scope law: no scenario without a P-ID) + coverage-matrix.md classifying all 60 P-IDs as auto / drive+observe / static-only
  - Verification layer: MCP read-back client over Pulse's tool surface, assertion policy split (deterministic = hard pass/fail, model-interpretive = calibration-region), run report with pass/fail/manual-check/known-residual, operator pauses for actions Conductor must not perform
  - Stack pinned by the brief: Rust 2024 + tokio, Tauri 2 control panel, tonic/prost gRPC; agent-driven development style; local dev host only; never manages the Pulse process
  - Normative companions in .andromeda/refs/: pulse-capability-spec.md (requirement source), capability-verification-matrix.json (four P-IDs formally delegated to Conductor: P-025/P-027/P-037/P-045), pulse-v0_2_0-capability-audit-2026-06-12.md (provenance; seeds coverage-matrix.md)
