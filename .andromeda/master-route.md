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
2026-06-17-raw-otlp-message-scaffold · complete · Raw OTLP message scaffold — hand-built opentelemetry-proto trace structs (ExportTraceServiceRequest/ResourceSpans/Span) over tonic/tonic-prost gRPC egress to 127.0.0.1:4317 (conductor-emit) · → conductor-0.1.0/chunks/2026-06-17-raw-otlp-message-scaffold/
2026-06-17-error-spans · complete · Error spans — `Status.Code=ERROR` construction + intra-trace root-vs-child span placement (parent_span_id linkage) on the raw OTLP scaffold (conductor-emit, P-005/P-008) · → conductor-0.1.0/chunks/2026-06-17-error-spans/
2026-06-18-exception-events-fingerprint-control · complete · Exception events + fingerprint control — OTel `exception` span events (type/message/stacktrace) + seeded line-insensitive expected-fingerprint primitive over identical/path/line variants (conductor-emit, P-006/P-017/P-018) · → conductor-0.1.0/chunks/2026-06-18-exception-events-fingerprint-control/
2026-06-18-severity-logs · complete · Severity logs — OTLP log records (LogsEmitter + raw ExportLogsServiceRequest) with controlled SeverityNumber + matching SeverityText across the 17 WARN→ERROR boundary (conductor-emit, P-007) · → conductor-0.1.0/chunks/2026-06-18-severity-logs/
2026-06-18-latency-shaping · complete · Latency shaping — seeded per-operation span-duration sampling realizing target p50/p95/p99 over the ≥50-sample latency floor (conductor-emit, P-011/P-012) · → conductor-0.1.0/chunks/2026-06-18-latency-shaping/
2026-06-18-multi-service-topology · complete · Multi-service topology — multiple service.name ResourceSpans + shared trace_id W3C cross-service propagation/parent-linkage (conductor-emit, P-008/P-027) · → conductor-0.1.0/chunks/2026-06-18-multi-service-topology/
2026-06-18-pii-payload-corpus · complete · PII payload corpus — seven P-047 categories (email/JWT/bearer/API-key/credit-card/SSN/secret key=value) embedded across spans/logs/exceptions (conductor-emit, P-035/P-047/P-048) · → conductor-0.1.0/chunks/2026-06-18-pii-payload-corpus/
