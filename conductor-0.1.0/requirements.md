# Conductor 0.1.0 — Requirements

_The capability set 0.1.0 must deliver. Flat list (WHAT, one line each, source-cited), drawn from
`architecture.md` established decisions + `input.md` capability set + the 6 specialist plans'
bootstrap surfaces. Anchors Phase 0 calibration; not a spec dump._

## Foundation — workspace & cross-cutting bootstrap

- **Crate-per-seam Cargo workspace** — `conductor-core` + `-timeline`/`-emit`/`-faults`/`-verify`/`-report` + `-cli`/`-tauri` bins; forbidden cross-seam deps won't compile — per arch §Module Boundaries / Directory structure.
- **Toolchain pin** — Rust 2024, `rust-toolchain.toml` ≥ 1.94.1 (tar-rs CVE bump over MSRV 1.88.0) — per security §Dependency Security.
- **Config validation surface** — serde + garde `range` + `#[garde(custom)]` cross-field rules on every scenario-config boundary struct — per arch §Validation, security input-validation-library-install.
- **Path-handle hardening** — `std::fs::canonicalize` + bounds-check on `CONDUCTOR_*` env path handles at the CLI edge (outside garde) — per security §Input Validation.
- **Dependency audit tooling** — `cargo-audit` (+ `cargo-deny` / `deny.toml`), committed `Cargo.lock` — per security dep-audit-tooling-install.
- **Error/verdict wall** — thiserror typed per-seam enums + anyhow at binary edges; verdicts/states are values, `Err` is harness-only; sanitized edges/artifacts — per arch §Error handling, security error-sanitization-wire.
- **Design token bundle** — Tailwind v4.1 `@theme` CSS custom properties (status tier + surfaces + auto dark/light) — per design `design-tokens-bundle-init`.
- **Typography stack** — self-hosted JetBrains Mono (status tier) + IBM Plex Sans (prose) via Fontsource — per design §Typography.
- **Structured logging stack** — `tracing` + `tracing-subscriber` JSON formatter; service identity (`service.name`/version/env); no OTel SDK for self-observation — per obs logger-stack-install / service-identity-wire.

## Core product capabilities

- **Timeline engine** — declarative phase sequences (duration · per-service emission spec), deterministic under a seed on `current_thread` tokio runtime — per input §Cap-1, arch `conductor-timeline`.
- **Emission journal** — wall-clock-stamped per-run JSONL of what was sent when (ground truth, left side of every SLO check) — per input §Cap-1, arch §Run-History; log format owned by tests/obs.
- **OTLP emission primitives** — raw opentelemetry-proto spans with controlled `Status.Code=ERROR`, root-vs-child placement, exception events with fingerprint control, `SeverityNumber` 17-boundary logs, latency p50/p95/p99 shaping, multi-service `service.name` + W3C propagation, PII corpus (7 categories), traffic ramps — per input §Cap-2, arch `conductor-emit`.
- **Fault helpers** — port-occupier on `:4317`, emission gap/resume, abrupt silence, bursty train (5min/10min) — per input §Cap-3, arch `conductor-faults`.
- **MCP read-back client** — rmcp 1.7.0 over `TokioChildProcess` stdio; tools `query_incident_list` / `retrieve_report` / `retrieve_telemetry_slice` / `mark_incident_resolved` — per input §Cap-4, arch §Inbound verification.
- **MCP preflight readiness gate** — pinned protocol `2024-11-05` + required-tool presence vs manifest + data-dir canary round-trip; mismatch/empty ⇒ Blocked, never silent downgrade — per arch §Standard Contracts, security Vector 4.
- **Subprocess-spawn hardening** — fixed sidecar program path, `ANDROMEDA_PULSE_DATA_DIR` via `.env(...)` only, reject injection metacharacters — per security §Input Validation / Code Patterns.
- **Verdict/assertion-policy split** — deterministic claims hard Pass/Fail; model-interpretive claims CalibrationRegion + report-for-human — per input §Cap-4, arch §Probabilistic-Assertion Policy.
- **Run report + states** — per-scenario Pass / Fail / ManualCheck / KnownResidual / Blocked, journal cross-referenced; P-032 ships as first KnownResidual — per input §Cap-4, arch §Read-Back Dependency Posture.
- **Run-history persistence** — per-run Markdown report + `runs.db` rusqlite index (run_id · seed · scenario · P-IDs · verdict · fingerprints · timestamps), bound-parameter SQL — per arch `conductor-report`, security §Input Validation.
- **Operator pauses** — go/no-go holds for actions Conductor must not perform (restart Pulse, config edits, ack, model on/off); resume on confirmation — per input §Cap-4.
- **Coverage matrix** — all 60 P-IDs classified auto / drive+observe / static-only, zero gaps (definition of done) — per input §Coverage classification, arch §Scope law.
- **Scenario catalog** — the spec's families keyed by P-ID (connection-lifecycle, hard-signals, error-baseline-spike, latency-regression, activity-floor, restart-suppression, fingerprint-storm, severity-lifecycle, constellation, context-grounding, pii-scrub, pipeline-ops, degraded-mode, report-surfaces) — per input §Scenario catalog.

## Surfaces

- **CLI / headless surface** — `conductor-cli` (`agent-run`) source of truth: `run` / `suite` / `report`; clap + owo-colors + indicatif + comfy-table + inquire; TTY-gated, never blocks headless on a prompt — per arch `conductor-cli`, design §Surface: cli.
- **Desktop control panel** — Tauri 2 frameless React 19 + Tailwind + shadcn/ui: scenario/suite picker, start/stop, live counters (`Channel`), coverage matrix, run-report view, operator-pause dialog, operator-checklist — per input §Cap-5, design §Surface: desktop-webview, layout-templates.
- **Paused-count hold-point signature** — the frozen heartbeat at the operator-pause (count freezes, tints green→amber, resumes/dims), mirrored desktop ↔ cli — per design §Signature element.
- **Tauri security guardrails** — deny-by-default capabilities (only the actual commands + one Channel), no `shell-open` with derived strings, no remote-origin iframes, Tauri ≥ 2.10.3 — per security §Code Patterns, design.

## Quality gates & harness

- **5-command test harness** — `scripts/agent-run.{sh,ps1}` boot(=preflight)/run/status/cleanup/logs; cargo-nextest + rstest fixtures + assert_cmd/assert_fs; envelope serializer; coverage via cargo-llvm-cov — per test-plan §3 bootstrap phases.
- **Determinism + contract + security tests** — deterministic-replay golden/property tests, MCP contract test (negotiate down to `2024-11-05`), security-vector negative tests, cross-surface parity — per test-plan §1 coverage triggers.
- **Observability self-instrumentation** — must-trace spans on the 7 critical paths, run_id correlation (IPC envelope, no W3C trace), field-allowlist redaction (no host paths / struct names), zero-unlogged-panics gate — per obs §3–4 / §10.
- **A11y assertions (desktop-webview)** — axe-core via `@axe-core/webdriverio` over the tauri-driver session + colorjs.io token-pair contrast + ARIA patterns (alertdialog/combobox/checkbox/status) + reduced-motion; operator/local-gated — per a11y §3 bootstrap phases.
- **CI gates** — GitHub Actions `cargo build` / nextest / clippy + cargo-audit/deny + coverage threshold; dynamic end-to-end proof is a local operator gate (needs live Pulse) — per arch §CI/CD, security/tests CI gates.
