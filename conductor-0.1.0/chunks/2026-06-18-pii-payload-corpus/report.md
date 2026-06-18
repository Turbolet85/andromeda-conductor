# Report — 2026-06-18-pii-payload-corpus

**Chunk:** PII payload corpus — seven P-047 categories (email/JWT/bearer/API-key/credit-card/SSN/secret key=value) embedded across spans/logs/exceptions (conductor-emit, P-035/P-047/P-048)
**Date:** 2026-06-18T19:45:24Z
**Commits:** (none yet — this wrap creates the chunk commit)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-emit/src/pii.rs` (new) · `crates/conductor-emit/tests/pii_payload_corpus.rs` (new) · `crates/conductor-emit/src/message.rs` (mod) · `crates/conductor-emit/src/lib.rs` (mod)
- **Symbols / APIs:** new PUBLIC (re-exported from `conductor-emit`): `PiiCategory` (enum, 7 variants + `all()`/`field_key()`), `PiiCorpus` (`seeded(u64)`/`seed()`/`value()`), `pii_trace_request(&str, &PiiCorpus, &[PiiCategory]) -> ExportTraceServiceRequest`, `pii_logs_request(&str, &PiiCorpus, &[PiiCategory]) -> ExportLogsServiceRequest`. New `pub(crate)`: `message::span_with_attributes` (mirrors `span_with_events`). No IPC method · no endpoint · no event · no socket · no port · no env var.
- **Crates / modules:** no new crate; one new module `conductor-emit::pii` (internal, re-exported). `conductor-emit` only — no cross-seam edge added.
- **Dependencies:** **none added / none bumped** — `rand_chacha` + `rand_core` (runtime) and `tokio` + `tokio-stream` (dev) were already `conductor-emit` deps; no `Cargo.toml` change.
- **Schema / config:** none — no scenario-config/garde wiring (deferred to the scenario epoch per scope.md); no `runs.db`/violation-schema touch.
- **Coverage of new surfaces:**
  - `pii_trace_request` / `pii_logs_request` (OTLP **product-stream** builders, emitted AT Pulse :4317) → validation n/a (no external input — programmatic `u64`/`&str`/`&[PiiCategory]` args, no deserialize/path/MCP stdout) · instrumentation: existing egress spans `emit.batch` / `emit.logs_batch` cover the export (no new must-trace op) ✓ · PII: the emitted values ARE synthetic PII — this is the deliberate PRODUCT fault payload (feeds Pulse's P-035 scrubber), NOT self-observation; self-obs logs no payload values (egress spans are `skip_all`, count-only); no host path / struct name on the wire or in logs · tests unit+integ ✓ · a11y n/a · tokens n/a
  - `PiiCorpus::seeded(u64)` (seeded synthetic generator) → validation n/a (programmatic seed) · PII synthetic-by-construction (never host/env-derived: no `std::env`/hostname/home-dir/credential read in the generation path) · determinism seeded (ChaCha8, same seed ⇒ identical corpus) · tests unit ✓

## Deviations from intent
- **Test mechanic: plain `#[test]` + `for category in PiiCategory::all()` loops instead of the plan's suggested `rstest #[case]` rows.** Justification: `conductor-emit` has NO `rstest` dev-dep, and all six existing emit test modules + integration tests use plain `#[test]`/`#[tokio::test]` + variant loops. The mandated *runner* is cargo-nextest (matched); rstest is an optional fixture layer, not a per-test mandate. Matching the crate's established convention avoided introducing a dependency and honored the plan's "no new dependency" constraint. Per-category coverage is fully achieved via the loops.
- **Trace builder emits two spans** (root = PII span-attributes carrier, OK; child = PII `exception`-event carrier, ERROR, linked via `parent_span_id`) rather than one span carrying both. Justification: keeps each `message.rs` primitive single-purpose (`span_with_attributes` ↔ attributes, `span_with_events` ↔ events) and cleanly separates the "spans" and "exceptions" signal carriers — a valid minimal 2-span trace, no combined builder needed.
- **Plan open-questions resolved in code:** chose the `span_with_attributes` helper (plan's preferred option over the inline fallback); credit-card PAN is **Luhn-valid** via a computed check digit (unit-tested); log-record severity left unspecified (PII is orthogonal to severity — that's P-007).
- All within `research.md`'s touchpoint lists; no out-of-scope edit; the `message.rs` edit was research's conditional modify-target.

## Decisions & corrections
- No user corrections this chunk (implementation-only session).
- **Synthetic PII is product data, not a leak:** the seven categories are emitted onto the OTLP stream Conductor drives AT Pulse — the harness's whole purpose for P-035/P-047/P-048. This is distinct from self-observation (which stays tracing-JSON, count-only, no payloads) — the obs/redaction invariant is not engaged.
- **Field keys are labels, not detection hints:** Pulse's scrubber detects PII by VALUE shape, so attribute/field keys (`user.email`, `payment.pan`, …) are descriptive labels only.
- **Scrub verification is out of scope** (emission only); the `pii-scrub` scenario (Epoch 7, conductor-verify, MCP read-back of scrubbed Report excerpts) consumes this corpus.
- Follow-up still open (carried from prior chunks, not this chunk's drift): `opentelemetry-proto` `default-features=false` trim of the dormant transitive `opentelemetry_sdk`; scenario-config garde wiring for `PiiCorpus`/`ServiceTopology` (scenario epoch).

## Outcome
- **Acceptance criteria met:** exactly 7 categories ✓ · seeded reproducible (same seed identical / ≥2 seeds divergent) ✓ · each payload structurally valid incl. Luhn-valid PAN ✓ · corpus reaches the wire on spans + logs + exceptions (loopback Trace+Logs capture stubs) ✓ · synthetic-only ✓ · no leak into self-obs/artifacts ✓ · refused transport ⇒ `EmitError` ✓ · public API re-exported ✓.
- **Gates green (commands run):** `cargo nextest run -p conductor-emit` → 57/57 · `cargo nextest run --workspace --profile ci` → 138/138 · `cargo clippy --workspace --all-targets -- -D warnings` → clean · `cargo test -p conductor-emit --doc` → 0 · `cargo llvm-cov nextest -p conductor-emit` → pii.rs 99.43% / message.rs 100% / total 97.46% (≫ 60% floor).
- **Smoke:** no boot-path change (library emission primitive; no binary/entry-point touched). `bash scripts/agent-run.sh status` dispatches (usage, exit 0) — harness intact. The new code's wire path is proven by the loopback integration tests.
