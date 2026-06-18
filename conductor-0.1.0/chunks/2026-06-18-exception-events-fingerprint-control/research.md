# Codebase Research — 2026-06-18-exception-events-fingerprint-control

## Scope
- **Depth:** moderate · **Reads:** 9 files · **Globs/Greps:** 3 globs + 3 greps · **code-graph:** schema probed (tables `defs`/`wsdef`/`occ`/`calls_m`); builder is net-new/additive so deep caller-impact query unwarranted.

## Authoritative spec correction (input.md §Capabilities + §Scenario catalog)
The fingerprint relationships are NOT what P1 scope.md first stated. Per `input.md:56-58` and `input.md:108` (**amended P-017 clause (c)**):
- **identical stack · path-variant · line-variant ⇒ ALL THREE the SAME fingerprint** (line-number AND path insensitivity).
- **type-variant + frame-variant ⇒ DIFFERENT fingerprint** (exception type + frame-function content ARE identity).
- Storm-cue thresholds (6/30s Suggested · 12/30s Autonomous hint) are the Epoch-7 fingerprint-storm SCENARIO — **out of scope** here.
→ scope.md's "path variant → different fingerprint" line is corrected (amended in scope.md); the working-route entry ("identical/path/line variants, line-insensitive fingerprint") was already correct.

## Files inspected
- `crates/conductor-emit/src/span_tree.rs` (full) — `ErrorPlacement` + `error_trace_request(service_name, seed, placement, message)` + `gen_id::<N>`. Seeded `ChaCha8Rng::seed_from_u64(seed)` owns all identity bytes; root→leaf chain; shape-projection determinism tests (ids+linkage+status, excludes timestamps). **The direct precedent + extension surface.**
- `crates/conductor-emit/src/message.rs` (full) — shared `pub(crate)` primitives: `span(name, trace_id, span_id, parent_span_id, status)`, `service_resource`, `ok_status`/`error_status`. **`span()` builds the `Span` with `..Default::default()` → its `events` vec is empty: this is the attach-point for exception span events.** Timestamps are wall-clock `std::time` (`unix_nanos()`), seed governs identity only.
- `crates/conductor-emit/src/lib.rs` (full) — `mod {client,error,message,span_tree}`; re-exports `error_trace_request`, `ErrorPlacement`, `trace_request`, `TraceEmitter`, `EmitError`. New module + re-export slot here.
- `crates/conductor-emit/Cargo.toml` — deps: `opentelemetry-proto {gen-tonic, trace}`, `tonic`, `thiserror`, `tracing`, `rand_chacha`, `rand_core`. No hashing crate present. dev-deps: `tokio {macros,rt,net}` + `tokio-stream {net}` (the loopback-stub harness).
- `crates/conductor-core/src/run_record.rs` (full) — the 11-field `RunRecord` envelope; **`fingerprints: Option<Vec<String>>` already declared** (`Some([])` = measured-with-none, `None` = blocked), golden-locked serialization order (`run_record.rs:99`). This chunk PRODUCES the strings; the journal field already exists.
- `crates/conductor-report/src/journal.rs` (full) — `JournalWriter::append(&RunRecord)`; `fingerprints` flows through verbatim, no computation here.
- `crates/conductor-core/src/phase_spec.rs` (full) — `EmissionSpec` is `#[non_exhaustive]` and its doc names "**fingerprint identity** … lands in the Epoch-3 emission seam, which extends `EmissionSpec`." Scenario-config wiring is a LATER (Epoch-7) concern; the prior error-spans chunk did NOT extend `EmissionSpec` (standalone builder), and this chunk follows that precedent.
- `crates/conductor-faults/src/lib.rs` + `Cargo.toml` — **empty stub**: lib is one doc line ("…port-occupier, fingerprint generation."); Cargo deps are `conductor-core` ONLY (does NOT depend on `conductor-emit`). Epoch-4 builds this crate.
- `.andromeda/input.md:47-115` — authoritative P-006/P-017/P-018 capability text + amended clause (c) (see correction above).

## Graph impact (code-graph)
- **Additive — no existing callers displaced.** `error_trace_request` (the sibling builder) is referenced only by its own `#[cfg(test)]` module today; the new exception builder + fingerprint fn are net-new public API with no upstream callers until the Epoch-7 scenario catalog. `EmissionSpec` is `#[non_exhaustive]` so additive extension is non-breaking (not extended this chunk regardless).

## Patterns detected
- **Seeded-builder** (`span_tree.rs:43`): a builder takes `seed: u64`, owns a local `ChaCha8Rng::seed_from_u64`, draws identity via `gen_id`; reads no clock for identity. The exception builder mirrors this for its span identity.
- **Shape-projection determinism test** (`span_tree.rs:100-170`): assert on `(trace_id, span_id, parent_span_id, status_code)` excluding wall-clock timestamps; pair same-seed⇒identical with ≥2-seeds⇒divergent (testing.md 2026-06-17). The fingerprint fn additionally needs same-identity⇒same-fp AND different-identity(type/frame)⇒different-fp.
- **Shared `pub(crate)` raw-struct primitives** (`message.rs`) with thin `pub` builders re-exported from `lib.rs` — extend `span()` (or add an exception-span helper) to carry an `events` vec rather than duplicating `Span` assembly.
- **Golden-locked envelope** (`run_record.rs:99`, `journal.rs:146`): the JSONL line keys are exactly the 11 owned names; any stray field fails the hygiene test — the fingerprint output must be plain `Vec<String>`, no host paths / struct names.

## Conventions to follow
- **Determinism on a shape-projection, never a byte-golden** (testing.md; `span_tree.rs:100`) — span timestamps are wall-clock and would flake a full golden.
- **Cross-platform/version-stable fingerprint** — mirror the ChaCha8-over-StdRng cross-version-stability decision (arch §Determinism RNG): the fingerprint string MUST be stable across Rust versions/platforms, so NO `std::hash::DefaultHasher`/SipHash. A stable scheme (normalized identity string, or a stable digest e.g. FNV-1a/sha2) — impl choice, see Open Q.
- **Module doc comment** citing `architecture §…` + the P-IDs (every emit module has one).
- **OTel exception semantic convention** — event `name = "exception"`, attributes `exception.type` / `exception.message` / `exception.stacktrace` as `KeyValue{StringValue}` (`opentelemetry_proto::tonic::trace::v1::span::Event`).
- **Redaction is already wired** — `fingerprints` is in the `conductor-core::redact` allowlist (`redact.rs:46`); keep stacktrace frame `file` fields free of absolute host paths (security/obs constraint).

## New files to create
- `crates/conductor-emit/src/exception.rs` *(home pending Open Q1 — emit vs faults)* — the exception-event builder (`exception_trace_request`-style: `service_name`, `seed`, an exception/variant spec, → `ExportTraceServiceRequest` with an `exception` span event), the variant model (identical / path / line / type / frame), and the path+line-insensitive `fingerprint(...) -> String` fn.
- `crates/conductor-emit/tests/exception_events.rs` — loopback gRPC-stub integration test for exception-event egress (the `TraceService`-on-`127.0.0.1:0` harness from `tests/egress.rs` / `error_spans.rs`).

## Files to modify
- `crates/conductor-emit/src/lib.rs` — `mod exception;` + re-export the builder + fingerprint + variant types.
- `crates/conductor-emit/src/message.rs` — extend `span()` (or add a sibling) to attach an `events: Vec<span::Event>` so the exception event rides on the span (currently `..Default::default()` leaves it empty).
- *(only if Open Q1 ⇒ faults)* `crates/conductor-faults/{Cargo.toml,src/lib.rs}` — add `conductor-emit` dep + the fingerprint module.
- `conductor-0.1.0/chunks/.../scope.md` — amend the fingerprint-relationship error (path/line/identical all same-fp; type/frame differ).

## Open questions
1. **Crate seam (→ P4 AskUserQuestion):** fingerprint fn in `conductor-emit` (co-located with the exception content it derives from; matches the error-spans "builder-in-emit" precedent; faults is empty + Epoch-4) vs `conductor-faults` (the literal module-map "fingerprint generation", which more plausibly denotes the Epoch-7 fingerprint-STORM fault that will compose this primitive). **Recommend emit.**
2. **Variant breadth:** include type-variant + frame-variant (the different-fp contrast) alongside the same-fp triple, so the fingerprint fn is testable in both directions. **Decided: include** (correctness requirement, not a user choice).
3. **Fingerprint representation/stability scheme** (normalized-string vs stable digest) — impl-level (HOW); plan only pins the cross-version-stability constraint.
