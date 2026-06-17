# Master Route — Conductor

<!--
Cross-version immutable index. APPEND-ONLY via promotion in /andromeda-phase — route never adds records.
One record per promoted chunk, grouped under its version:
  {marker} · {status: pending|complete} · {super-laconic description} · → {link to chunk folder}
marker = {date}-{slug} (e.g. 2026-06-04-otlp-http-ingest), minted at promotion.
-->

## conductor-0.1.0
2026-06-14-cargo-workspace-scaffold · complete · Cargo workspace scaffold — 8 crate-per-seam members + toolchain pin ≥1.94.1 · → conductor-0.1.0/chunks/2026-06-14-cargo-workspace-scaffold/
2026-06-15-conductor-core-shared-types · complete · conductor-core shared types — Verdict/ReportState enums, scenario model, verdict/error wall · → conductor-0.1.0/chunks/2026-06-15-conductor-core-shared-types/
2026-06-15-config-validation-surface · complete · Config-validation surface — garde Validate on scenario model, garde Report→CoreError bridge, CONDUCTOR_* path canonicalize · → conductor-0.1.0/chunks/2026-06-15-config-validation-surface/
2026-06-15-dependency-audit-gate · complete · Dependency-audit gate — cargo-audit + cargo-deny over the OTLP/gRPC/SQLite tree + committed Cargo.lock · → conductor-0.1.0/chunks/2026-06-15-dependency-audit-gate/
2026-06-15-structured-logging-stack · complete · Structured logging stack — tracing + tracing-subscriber JSON (no OTel SDK), service-identity + run_id fields, std::panic::set_hook capture · → conductor-0.1.0/chunks/2026-06-15-structured-logging-stack/
2026-06-15-log-error-boundary-redaction · complete · Log + error-boundary redaction — tracing field-allowlist + anyhow-edge sanitization (no host-paths/struct-names/stack-traces) · → conductor-0.1.0/chunks/2026-06-15-log-error-boundary-redaction/
2026-06-15-design-token-typography-bundle · complete · Design-token + typography bundle — Tailwind v4.1 @theme token layer + self-hosted JetBrains Mono / IBM Plex Sans (desktop-webview foundation) · → conductor-0.1.0/chunks/2026-06-15-design-token-typography-bundle/
2026-06-16-test-framework-fixtures-coverage-tooling · complete · Test framework + fixtures + coverage tooling — cargo-nextest profiles (.config/nextest.toml, zero-retry ci) + rstest/proptest/insta fixtures + assert_cmd/assert_fs + cargo-llvm-cov · → conductor-0.1.0/chunks/2026-06-16-test-framework-fixtures-coverage-tooling/
2026-06-16-base-ci-agent-run-harness-skeleton · complete · Base CI + agent-run harness skeleton — GitHub Actions build/nextest(ci)/clippy/doctest + cargo-audit/deny + llvm-cov + frontend npm-audit/vite-build gate; agent-run.{sh,ps1} 5-command skeleton · → conductor-0.1.0/chunks/2026-06-16-base-ci-agent-run-harness-skeleton/
2026-06-16-seeded-phase-scheduler · complete · Seeded phase scheduler — deterministic current_thread tokio::time phase sequencing on a seeded RNG (conductor-timeline) · → conductor-0.1.0/chunks/2026-06-16-seeded-phase-scheduler/
2026-06-16-scenario-config-model · complete · Scenario-config model — declarative per-phase emission spec (serde + garde), wires Scenario→PhaseTimeline into the seeded scheduler (conductor-core + conductor-timeline) · → conductor-0.1.0/chunks/2026-06-16-scenario-config-model/
2026-06-16-emission-journal-writer · complete · Emission-journal writer — per-run JSONL emission journal (tests/obs-owned line schema, std::time journal_emitted_at stamps, run_id-stemmed never-overwritten, CONDUCTOR_RUNS_DIR-overridable) wired into the timeline · → conductor-0.1.0/chunks/2026-06-16-emission-journal-writer/
2026-06-17-determinism-replay-harness · complete · Determinism-replay harness — insta golden freeze of the absolute PhaseTransition stream shape (full Scenario-fixture→PhaseTimeline→run_timeline pipeline) + proptest replay/bounds/monotonic invariants across the seed space, all under tokio start_paused (conductor-timeline) · → conductor-0.1.0/chunks/2026-06-17-determinism-replay-harness/
