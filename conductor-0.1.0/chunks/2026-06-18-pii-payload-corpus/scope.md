# Scope — PII payload corpus

**Marker:** `2026-06-18-pii-payload-corpus`
**Version:** conductor-0.1.0 · **Epoch 3 — Emission primitives (7/8)**
**Crate:** `conductor-emit`
**P-IDs:** P-047 (the seven PII categories) · P-035 (PII scrub claim this feeds) · P-048 (scrub structure-preservation)

## What it builds
A deterministic, seeded **PII payload corpus** — synthetic-but-structurally-valid sample values across the
seven P-047 categories — plus the emission primitives that embed that corpus into all three OTLP signal types
so a downstream `pii-scrub` scenario can verify Pulse scrubs them while preserving surrounding structure.

The seven P-047 categories (per `input.md` coverage matrix line 64–65 / pii-scrub row 112):
1. **emails**
2. **JWT** (three-segment `header.payload.signature`)
3. **bearer** tokens (`Authorization: Bearer …`)
4. **API keys**
5. **credit cards** (PAN, structurally valid)
6. **SSN**
7. **secret-like `key=value`** (e.g. `password=…`, `api_secret=…`)

Emission surfaces (all three signal types — "through spans/logs/exceptions"):
- **spans** — PII embedded in span attribute `KeyValue`s (over the `message.rs`/`span_tree.rs` builders);
- **logs** — PII embedded in OTLP log-record body / attributes (over `logs.rs`);
- **exceptions** — PII embedded in `exception` span-event fields — `exception.message` / `exception.stacktrace`
  (over `exception.rs`).

Expected deliverable shape (WHAT, not HOW — final form is the plan's job): a new `conductor-emit` module
(e.g. `pii.rs`) exposing a `PiiCategory` enum over the seven categories, a **seeded** corpus generator
(same seed ⇒ same corpus, per the determinism invariant), and request-builder(s) that inject the corpus
across spans/logs/exceptions; new `lib.rs` re-exports for the public corpus API; unit tests + a loopback
integration test asserting each category reaches the wire in each of the three signal types.

## Boundaries (what it is NOT)
- **Emission-side only.** This produces the corpus + embeds it on the wire. Asserting Pulse's Report excerpts
  come back **scrubbed with structure preserved** (the `pii-scrub` scenario, P-035/P-047/P-048) is **Epoch 7**
  (Scrub/pipeline/degraded scenarios) over MCP read-back — NOT this chunk. No verdict logic, no `conductor-verify`.
- **No scenario-config / garde wiring.** Declaring a PII spec in a `scenarios/*.toml` is deferred to the
  scenario epoch (same posture as the `ServiceTopology` garde follow-up).
- **Synthetic data only.** The corpus is generated fake fixtures by construction — never real or host-derived
  PII, no secrets, no network. (The values must merely *match the shape* each category's detector keys on.)
- **`conductor-emit` only.** No new crate, no cross-seam dependency; composes existing in-crate primitives.

## Surfaces / contracts it touches
- **Crate:** `conductor-emit` (Epoch 3). Composes the existing `pub(crate)` resource/span primitives
  (`message.rs`), the exception-event builder (`exception.rs`), and the log-record builder (`logs.rs`);
  reuses `TraceEmitter`/`LogsEmitter` + `EmitError` for egress.
- **OTLP wire:** PII rides as opentelemetry-proto raw-type values — span/log `KeyValue` attributes, log body,
  exception-event fields — no OTel SDK.
- **Determinism invariant:** seeded generation (`seed_from_u64`, ChaCha8) so the same scenario+seed yields the
  same corpus shape; "structure preserved" means each payload is structurally valid for its category so a
  scrubber recognizes and redacts it.
- **Verdict/error wall:** builder is an emit-side self-validating type; transport faults surface as `EmitError`
  (`Result::Err`), never a panic — consistent with the sibling emit modules.

## Acceptance intent (anchor for validation-1)
- A `PiiCategory` (or equivalent) covers exactly the seven P-047 categories — no more, no fewer.
- The corpus is seeded + reproducible (same seed ⇒ identical corpus).
- The corpus is embeddable into, and observably reaches the wire on, all three signal types (spans, logs,
  exceptions) — proven by a loopback integration test per signal type.
- Each emitted payload is structurally valid for its category (so Pulse's scrubber can key on it).
- Public API re-exported from `conductor-emit/src/lib.rs`; gates green (emit + workspace nextest, clippy `-D`,
  llvm-cov, doctest).
