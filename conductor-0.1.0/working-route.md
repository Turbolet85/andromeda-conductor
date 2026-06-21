# Working Route — conductor-0.1.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve then skips frozen lines); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation
[2026-06-14-cargo-workspace-scaffold] Cargo workspace scaffold — 8 crate-per-seam members, workspace manifest, rust-toolchain pin ≥1.94.1
   ↓
[2026-06-15-conductor-core-shared-types] conductor-core shared types — Verdict/ReportState enums, scenario model, verdict/error wall
   ↓
[2026-06-15-config-validation-surface] Config-validation surface — serde + garde range/cross-field rules, CONDUCTOR_* path-handle canonicalize
   ↓
[2026-06-15-dependency-audit-gate] Dependency-audit gate — cargo-audit + cargo-deny over the OTLP/gRPC/SQLite tree, committed Cargo.lock
   ↓
[2026-06-15-structured-logging-stack] Structured logging stack — tracing + tracing-subscriber JSON (no OTel SDK), service-identity fields, std::panic::set_hook capture
   ↓
[2026-06-15-log-error-boundary-redaction] Log + error-boundary redaction — tracing-subscriber field-allowlist + anyhow-edge sanitization (no host-paths/struct-names/stack-traces)
   ↓
[2026-06-15-design-token-typography-bundle] Design-token + typography bundle — Tailwind v4.1 @theme tokens, JetBrains Mono + IBM Plex Sans
   ↓
[2026-06-16-test-framework-fixtures-coverage-tooling] Test framework + fixtures + coverage tooling — cargo-nextest, rstest, proptest, insta, assert_cmd/fs, cargo-llvm-cov
   ↓
[2026-06-16-base-ci-agent-run-harness-skeleton] Base CI + agent-run harness skeleton — GitHub Actions build/nextest/clippy, agent-run.{sh,ps1} stub

### Epoch 2 — Timeline engine
[2026-06-16-seeded-phase-scheduler] Seeded phase scheduler — current_thread tokio::time deterministic phase sequencing
   ↓
[2026-06-16-scenario-config-model] Scenario-config model — declarative per-phase emission spec, serde + garde validated
   ↓
[2026-06-16-emission-journal-writer] Emission-journal writer — per-run JSONL, std::time wall-clock stamps, tests/obs-owned schema
   ↓
[2026-06-17-determinism-replay-harness] Determinism-replay harness — same scenario+seed yields identical stream shape via insta golden + proptest, tokio start_paused

### Epoch 3 — Emission primitives
[2026-06-17-raw-otlp-message-scaffold] Raw OTLP message scaffold — opentelemetry-proto structs over tonic/prost gRPC egress to :4317
   ↓
[2026-06-17-error-spans] Error spans — Status.Code=ERROR with root-vs-child placement (P-005, P-008)
   ↓
[2026-06-18-exception-events-fingerprint-control] Exception events + fingerprint control — identical/path/line variants, line-insensitive fingerprint (P-006, P-017, P-018)
   ↓
[2026-06-18-severity-logs] Severity logs — SeverityNumber across the 17-boundary (P-007)
   ↓
[2026-06-18-latency-shaping] Latency shaping — target p50/p95/p99 per operation (P-011, P-012)
   ↓
[2026-06-18-multi-service-topology] Multi-service topology — service.name virtual topology + W3C trace propagation (P-008, P-027)
   ↓
[2026-06-18-pii-payload-corpus] PII payload corpus — seven P-047 categories across spans/logs/exceptions (P-035, P-048)
   ↓
[2026-06-18-traffic-rate-ramps] Traffic-rate ramps — halo-breathing emission ramps (P-026)

### Epoch 4 — Fault helpers
[2026-06-19-port-occupier-fault] Port-occupier fault — sacrificial :4317 listener before Pulse starts (P-003 ReceiverFailed)
   ↓
[2026-06-19-emission-gap-resume] Emission gap/resume — exact gap lengths >20s with resume (P-015 restart detection)
   ↓
[2026-06-19-abrupt-silence-fault] Abrupt-silence fault — permanent emission stop (P-014)
   ↓
[2026-06-20-bursty-train-pattern] Bursty-train pattern — active 5min / quiet 10min repeating (P-013 activity-floor)

### Epoch 5 — Verification & read-back
[2026-06-21-mcp-read-back-client] MCP read-back client — rmcp over TokioChildProcess stdio, hardened fixed-path sidecar spawn (.env data-dir)
   ↓
[2026-06-21-preflight-readiness-gate] Preflight readiness gate — pinned 2024-11-05 + tool presence + data-dir canary, Blocked on mismatch
   ↓
[2026-06-21-otlp-egress-liveness-check] OTLP egress liveness check — loopback :4317 connectable, refused ⇒ harness Err
   ↓
[2026-06-21-verdict-assertion-policy-split] Verdict + assertion-policy split — hard Pass/Fail vs CalibrationRegion classification
   ↓
[2026-06-21-expected-outcome-slo-timing-model] Expected-outcome + SLO timing model — per-scenario expected blocks, tier-scaled tolerance <5s/<20s/<90s   PREREQ: feeds matched/observed/expected into the ClaimClass/classify→Assessment mechanism from verdict-assertion-policy-split (concrete comparison kinds deferred here)
   ↓
[2026-06-21-operator-pause-orchestration] Operator-pause orchestration — go/no-go holds + resume-on-confirm for non-Conductor actions

### Epoch 6 — Run report & persistence
[2026-06-21-run-report-envelope-serializer] Run-report envelope serializer — canonical shape shared by Markdown + runs.db + JSONL
   ↓
[2026-06-21-runs-db-index] runs.db index — rusqlite schema, bound-parameter writes, JSON1 fingerprint arrays
   ↓
[2026-06-21-markdown-run-report] Markdown run report — per-scenario Pass/Fail/ManualCheck/KnownResidual/Blocked render
   ↓
[2026-06-21-coverage-matrix-generator] Coverage-matrix generator — all 60 P-IDs classified auto/drive+observe/static-only

### Epoch 7 — Scenario catalog
Connection-lifecycle scenarios — Listening/Receiving/Idle/Stalled states with orthogonal port-occupier (P-001..P-004)
   ↓
Hard-signals scenarios — ERROR/exception/severity-boundary/root-vs-deep checks (P-005..P-008)
   ↓
Error-baseline-spike + latency-regression scenarios — baseline convergence, ramp, candidate persistence (P-009..P-012)
   ↓
Activity-floor + restart-suppression scenarios — train/lunch/silence, restart gap, suppression/bypass triple (P-013..P-016, P-057)
   ↓
Fingerprint-storm scenarios — identity/path/line-variant fingerprints, storm cue thresholds (P-017, P-018)
   ↓
Severity-lifecycle scenarios — tiered inputs, auto-resolve, ack-retrigger, per-tier SLO (P-019..P-023, P-059, P-060)
   ↓
Constellation + context-grounding scenarios — service dot/hue/stability, git commits/recurrence, P-032 known-residual (P-025..P-027, P-036)
   ↓
Scrub/pipeline/degraded/report-surface scenarios — PII scrub, cadence/hot-reload, model-off, render-timing (P-035, P-037, P-045, P-047..P-056)

### Epoch 8 — CLI surface
conductor run/suite/report verbs — clap CLI over current_thread bootstrap
   ↓
5-command agent-run harness — boot=preflight, run=nextest+scenarios, status=runs.db/JSONL read, cleanup=idempotent, logs=journal (.sh + .ps1)
   ↓
Line-oriented output rendering — owo-colors/indicatif/comfy-table status lines + coverage table
   ↓
isatty-gated operator-pause — inquire confirm + paused-count spinner mirror, headless never blocks
   ↓
Sanitized stderr + agent-mode logging — error:/hint: format, JSON-to-file journal

### Epoch 9 — Desktop control panel
Frameless window shell — Tauri 2 decorations:false drag-region titlebar, deny-by-default capabilities ≥2.10.3
   ↓
Paused-count hold-point signature — frozen heartbeat freeze/tint/resume in titlebar
   ↓
Scenario/suite picker + start/stop — shadcn Command/Select with run controls
   ↓
Live-counter Channel stream — Tauri Channel backend-to-frontend emission counters + target status
   ↓
Component primitives library — six status-lamp variants + dialog scaffold + operator-checklist primitive
   ↓
Coverage-matrix view — dense single-row-per-P-ID list with verdict/report-state lamps
   ↓
Run-report + operator-checklist views — verdict lines + ManualCheck induced-state checklist
   ↓
Operator-pause go/no-go dialog — AlertDialog gating each committed timeline step
   ↓
Desktop a11y harness setup — axe/Lighthouse/colorjs.io over tauri-driver, shadcn/Radix ARIA binding, token-pair contrast
   ↓
Desktop a11y verification — axe violations + contrast + keyboard trap/focus-order on four accessible paths, NVDA/VoiceOver manual spec

### Epoch 10 — Polish & ship
Live-Pulse E2E proof — error-baseline-spike/fingerprint-storm/restart-suppression/pii-scrub/connection-lifecycle MCP-verified
   ↓
Severity-lifecycle full pass — auto-resolve + resolution-summary observed via read-back
   ↓
Cross-surface parity proof — CLI vs Tauri identical envelope for the same seed
   ↓
CI quality-gate config — coverage threshold + flakiness budget + nextest JUnit / llvm-cov artifact upload
   ↓
Obs CI conformance gate — agent-latest.jsonl upload + log-schema conformance + zero-unlogged-panics check
   ↓
A11y CI gate + violation JSON — axe/contrast/keyboard PASS/FAIL into obs envelope, service-tagged
   ↓
Coverage-matrix completeness gate — 60 P-IDs zero-gap, all CI gates green (definition of done)
   ↓
Release build + Tauri bundle — cargo build --release + Tauri 2 bundle, final SLO verification
