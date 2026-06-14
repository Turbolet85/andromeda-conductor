# 1A Decomposition Tree — Conductor 0.1.0

_Phase 1 debug artifact. Sub-block tier visible here (dropped in 1C linearization).
Each leaf = a chunk candidate + (source). Decomposed to each epoch's Phase-0 depth._

### Epoch 1 — Foundation (depth 2)
- **Workspace & core**
  - Cargo workspace scaffold (source: arch §Directory structure; security §Dependency Security toolchain pin)
  - conductor-core shared types — Verdict/ReportState + scenario model + error wall (source: arch §Error handling / Core structure)
- **Validation & supply-chain**
  - Config-validation surface — serde+garde + path-handle hardening (source: arch §Validation; security input-validation)
  - Dependency-audit gate — cargo-audit/cargo-deny (source: security dep-audit-tooling-install)
- **Observability & design bootstrap**
  - Structured logging stack — tracing+subscriber JSON + service identity (source: obs logger-stack-install / service-identity-wire)
  - Design-token + typography bundle (source: design §Palette/Typography; design-tokens-bundle-init)
- **CI & harness**
  - Base CI + agent-run harness skeleton (source: arch §CI/CD; tests §3 / a11y/obs CI-gate roots)

### Epoch 2 — Timeline engine (depth 1)
- Seeded phase scheduler (source: arch conductor-timeline; determinism discipline)
- Scenario-config model (source: input §Cap-1; arch §Config conventions)
- Emission-journal writer (source: input §Cap-1; obs §6 / tests §3 owned schema)
- Determinism-replay harness (source: arch determinism; tests property-test trigger)

### Epoch 3 — Emission primitives (depth 2)
- **OTLP scaffold**
  - Raw OTLP message scaffold (source: arch §OTLP Emission Strategy)
- **Hard signals**
  - Error spans + root/child placement (source: input §Cap-2; P-005/P-008)
  - Exception events + fingerprint control (source: input §Cap-2; P-006/P-017/P-018)
  - Severity logs (source: input §Cap-2; P-007)
- **Distributions & topology**
  - Latency shaping (source: input §Cap-2; P-011/P-012)
  - Multi-service topology + W3C propagation (source: input §Cap-2; P-008/P-027)
- **Payloads & rates**
  - PII payload corpus (source: input §Cap-2; P-047/P-035/P-048)
  - Traffic-rate ramps (source: input §Cap-2; P-026)

### Epoch 4 — Fault helpers (depth 1)
- Port-occupier fault (source: input §Cap-3; security Vector 6; P-003)
- Emission gap/resume (source: input §Cap-3; P-015)
- Abrupt-silence fault (source: input §Cap-3; P-014)
- Bursty-train pattern (source: input §Cap-3; P-013)

### Epoch 5 — Verification & read-back (depth 2)
- **MCP client & gate**
  - MCP read-back client + hardened spawn (source: arch §MCP Read-Back; security Vector 4)
  - Preflight readiness gate (source: arch §Standard Contracts readiness gate)
  - OTLP egress liveness check (source: arch §Standard Contracts liveness)
- **Verdict logic**
  - Verdict + assertion-policy split (source: arch §Probabilistic-Assertion Policy)
  - Expected-outcome + SLO timing model (source: arch §Timing-Tolerance Model)
- **Operator interaction**
  - Operator-pause orchestration (source: input §Cap-4)

### Epoch 6 — Run report & persistence (depth 1)
- Run-report envelope serializer (source: arch §Standard Contracts envelope; tests §3)
- runs.db index (source: arch §Data model conventions; security §Input Validation)
- Markdown run report (source: input §Cap-4; arch §Read-Back posture states)
- Coverage-matrix generator (source: input §Coverage classification)
- Artifact redaction layer (source: obs pii-scrubbing-wire)

### Epoch 7 — Scenario catalog (depth 2)
- **Lifecycle & signals**
  - Connection-lifecycle scenarios (source: input §Scenario catalog; P-001..P-004)
  - Hard-signals scenarios (source: input §Scenario catalog; P-005..P-008)
- **Baselines & timing**
  - Error-baseline-spike + latency-regression scenarios (source: input §Scenario catalog; P-009..P-012)
  - Activity-floor + restart-suppression scenarios (source: input §Scenario catalog; P-013..P-016/P-057)
- **Fingerprint & lifecycle**
  - Fingerprint-storm scenarios (source: input §Scenario catalog; P-017/P-018)
  - Severity-lifecycle scenarios (source: input §Scenario catalog; P-019..P-023/P-059/P-060)
- **Constellation & residual / scrub**
  - Constellation + context-grounding scenarios (source: input §Scenario catalog; P-025..P-027/P-032/P-036)
  - Scrub/pipeline/degraded/report-surface scenarios (source: input §Scenario catalog; P-035/P-037/P-045/P-047..P-056)

### Epoch 8 — CLI surface (depth 1)
- conductor run/suite/report verbs (source: arch conductor-cli; design §Surface: cli)
- 5-command agent-run harness (source: tests §3 5-command-discipline-wire)
- Line-oriented output rendering (source: design §Surface: cli)
- isatty-gated operator-pause (source: design signature CLI mirror; tests harness)
- Sanitized stderr + agent-mode logging (source: obs §3; security error-sanitization)

### Epoch 9 — Desktop control panel (depth 2)
- **Shell & signature**
  - Frameless window shell + capabilities (source: design §Surface desktop-webview; security §Code Patterns)
  - Paused-count hold-point signature (source: design §Signature element)
- **Controls & streaming**
  - Scenario/suite picker + start/stop (source: design §Component Patterns)
  - Live-counter Channel stream (source: arch §Real-time Strategy)
- **Result views**
  - Coverage-matrix view (source: design/layout §Coverage matrix)
  - Run-report + operator-checklist views (source: design/layout §Run-report / Operator-checklist)
  - Operator-pause go/no-go dialog (source: design/layout §Operator-pause dialog)
- **A11y**
  - Desktop a11y assertion harness (source: a11y §3 bootstrap)

### Epoch 10 — Polish & ship (depth 1)
- Live-Pulse E2E proof (source: input §What 0.1.0 done means)
- Severity-lifecycle full pass (source: input §What 0.1.0 done means)
- Cross-surface parity proof (source: tests Critical Path 7)
- Coverage-matrix completeness gate (source: input/tests/security CI gates)
- Release build + Tauri bundle (source: arch §Deployment)
