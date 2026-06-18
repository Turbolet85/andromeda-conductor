# obs extract

## Relevance
Partial — latency shaping is a standalone Epoch 3 primitive that produces OTLP `Span` timing fields; self-obs occurs at the boundary (span emission), not in the primitive's internals.

## Constraints
- Per §1 Instrumentation scope: `conductor-emit` is instrumentable; timing fields are shaped via `start_time_unix_nano` / `end_time_unix_nano` (raw OTLP, no virtual-clock dependency) per §1 entity spec
- Per §3 OTel SDK init (behavioral): no SDK init allowed; seeded-RNG determinism is the invariant (§1 justification "zero work-stealing preserves emission ordering as function of seed") — ChaCha8Rng discipline inherited from conductor-timeline per scope
- Per §4 Span coverage (Fault-injection spans): fault spans are children of `timeline.execute`; latency-shaping durations are realized on the concrete `Span` objects passed to `TraceEmitter`, not a span itself
- Per §1 Telemetry Strategy (perf-budget-instruments trigger): `latency_ms` field in JSONL journal is computed `read_back_observed_at − journal_emitted_at` (wall-clock, per scope "Determinism contract"); shaped durations on OTLP spans are informational / test-harness calibration (not the SLO-math source)
- Per §2 Agent-readable invariants: every signal must be agent-parseable; shaped durations in OTLP are introspectable via CLI JSON emission sink (paste-to-AI)

## Patterns to follow
- Raw-OTLP `Span` timing pattern (§1 conductor-emit entity / §4 span kinds): manual instrumentation wraps duration shaping; no auto-instrumentation (CLI is not HTTP)
- Determinism seed contract (§1 stack justification / scope "Deterministic under a seed"): use `ChaCha8Rng` seeded RNG for percentile sampling; document reproducibility guarantee
- Module boundary spans (§4 span naming `{module}.{operation}`): if latency shaping surfaces a top-level operation (e.g., `latency.sample_profile`), wrap it in a low-cardinality span per the bounded-set invariant (§11 Anti-Patterns)
- OTLP semantic conventions boundary (§1 conductor-emit instrumentation scope / scope "OTLP semantic conventions"): only mutate `start_time_unix_nano` / `end_time_unix_nano` on the `Span` struct; base timestamp always from `std::time::SystemTime` (never tokio virtual clock)

## Anti-patterns to avoid
- No OTel SDK init for seeded-RNG state or batch export (§3 OTel SDK init behavioral ban — would break determinism)
- No high-cardinality span names parameterized by operation ID or individual sample index (§11 Anti-Patterns Spans / bounded span-name set — latency shaping must aggregate to `{module}.{operation}` per-operation, not per-sample)
- No absolute file paths or struct-name debug-dumps in span attributes or OTLP metadata (§11 Anti-Patterns Logs / PII Scrubbing — latency shaping stays within OTLP primitives, but any boundary logging must respect redaction)

## Contract bindings
- **Obs ↔ Verification (conductor-verify):** scope notes "no verification" but P-012 (latency-regression detection) depends on the shaped durations; the verify chunk (Epoch 5) consumes these `Span.start_time_unix_nano / end_time_unix_nano` fields and compares realized percentiles to targets. Latency shaping produces; verify consumes.
- **Obs ↔ Tests harness:** §3 Harness Contract specifies JSONL log schema; SLO tiers (`<5s | <20s | <90s`) appear in the `slo_tier` field. Latency shaping does NOT emit the JSONL record itself (that is the report seam, Epoch 6); it supplies the shaped durations that the verify→report chain uses to compute `latency_ms`. Tests consume the final JSONL and assert on `latency_ms` per SLO tier.

## Acceptance criteria contributions
- (obs) Shaped OTLP span durations (start/end time on `Span` objects) follow the target profile (p50/p95/p99 in ms) within sampling-convergence tolerance over ≥50 samples per operation.
- (obs) Determinism: identical seed + profile reproduces the exact duration sequence; unit tests verify reproducibility across runs.
- (obs) Boundary instrumentation: if a top-level `latency.sample_profile(profile) → Vec<Duration>` function is added, wrap it in a `#[tracing::instrument]` span named per `{module}.{operation}` (e.g., `latency.sample`, bounded cardinality), with span attributes `operation` (string), `sample_count` (integer ≥50), `slo_tier` (if applicable); output is machine-parseable (OTLP Span objects with deterministic timing fields).
- (obs) No OTel SDK initialization in the module; seeded RNG (ChaCha8) is the determinism source.

## Relevant amendment history
- **2026-06-15-structured-logging-stack:** clarified self-obs log line schema (base set: timestamp_ms, level, target, service-identity, run_id) vs Run-report envelope (11 fields incl. latency_ms, slo_tier). Latency shaping produces durations that flow into the envelope's `latency_ms` field; this chunk is pre-envelope.
- **2026-06-15-log-error-boundary-redaction:** redaction model anchored to absolute host-file paths (not token-level ::); struct-name guard via allowlist + Display. Latency shaping stays within OTLP primitives (no sensitive data); no new redaction scope.
- **2026-06-16-emission-journal-writer:** Run-report envelope schema finalized to 11 fields with `read_back_observed_at` (ISO-8601, null until read-back). Latency shaping does NOT write the envelope; the report seam does. The envelope's `latency_ms = read_back_observed_at − journal_emitted_at` (wall-clock) is the SLO measurement; latency shaping's OTLP-timing shapes are informational calibration inputs to the verify→report chain.
- **2026-06-17-raw-otlp-message-scaffold:** no-SDK invariant clarified as behavioral; dormant transitive opentelemetry crates noted (not a violation). Latency shaping inherits this: no SDK init, seeded RNG for determinism, OTLP primitive authoring.