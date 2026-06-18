# Report — 2026-06-18-exception-events-fingerprint-control

**Chunk:** Exception events + fingerprint control — OTel `exception` span events (type/message/stacktrace) + seeded line-insensitive expected-fingerprint primitive over identical/path/line variants (conductor-emit, P-006/P-017/P-018)
**Date:** 2026-06-18
**Commits:** none yet this session — the chunk is uncommitted; the commit lands in this wrap's P7. (Prior commit `beeb82e` error-spans was at `last_wrap`.)

## Changes (structured — detectors read this)
- **Files:**
  - NEW `crates/conductor-emit/src/exception.rs` — exception/variant model + `fingerprint()` + `exception_trace_request()` + FNV-1a + 7 unit tests
  - NEW `crates/conductor-emit/tests/exception_events.rs` — loopback gRPC-stub integration test
  - MOD `crates/conductor-emit/src/lib.rs` — `mod exception` + re-exports + module doc
  - MOD `crates/conductor-emit/src/message.rs` — `span_with_events()`, `string_kv()`, `service_resource` refactor, `unix_nanos`→`pub(crate)`, `Event` import
  - MOD `crates/conductor-emit/src/span_tree.rs` — `gen_id`→`pub(crate)` (1 token)
- **Symbols / APIs:** new crate-root public re-exports on `conductor-emit`: `exception_trace_request(service_name: &str, seed: u64, spec: &ExceptionSpec) -> ExportTraceServiceRequest` · `fingerprint(spec: &ExceptionSpec) -> String` · `ExceptionSpec` · `Frame` · `FingerprintVariant` (Identical/PathVariant/LineVariant/TypeVariant/FrameVariant). New `pub(crate)`: `message::span_with_events`, `message::string_kv`, `message::unix_nanos` (vis), `span_tree::gen_id` (vis). **No new IPC method / endpoint / socket / port / env var.**
- **Crates / modules:** `conductor-emit` gains module `exception`. No crate added/removed. `conductor-faults` untouched — stays an empty stub; the fingerprint primitive landed in `conductor-emit` per the crate-seam decision the user **ratified in /andromeda-phase** (vs CLAUDE.md/arch §Modules attributing "fingerprint generation" to faults — see Decisions).
- **Dependencies:** none added, none bumped. `Cargo.lock` un-drifted.
- **Schema / config:** none. The fingerprint output feeds the **existing** `conductor-core::RunRecord.fingerprints: Option<Vec<String>>` envelope field (unchanged). `EmissionSpec` (`#[non_exhaustive]`) **not** extended — scenario-config wiring is deferred to Epoch-7.
- **Coverage of new surfaces:**
  - `conductor-emit::exception_trace_request` (OTLP exception-event construction → PRODUCT egress to `:4317`) → validation **n/a** (programmatic builder, no external input; scenario-config garde validation is Epoch-7) · instrumentation **n/a** (PRODUCT emission, not self-obs; self-obs of emission is the existing `emit.batch` seam, not this builder) · PII **redacted✓** (synthetic stacktrace with relative module paths — unit test asserts no `C:\`/`/Users/`/`/home/`; fingerprint is opaque hex) · tests **unit+integ✓** · a11y **n/a** · tokens **n/a**
  - `conductor-emit::fingerprint` (path+line-insensitive expected fingerprint) → validation **n/a** · instrumentation **n/a** · PII **n/a** (FNV-1a hex, content-derived, carries no paths) · tests **unit✓** (5 variant relationships + pure-function determinism) · a11y **n/a** · tokens **n/a**

## Deviations from intent
1. **`span_tree.rs` edited though plan's "Files to modify" listed only `lib.rs` + `message.rs`.** A 1-token visibility change (`gen_id` → `pub(crate)`) to enable the plan's explicit gen_id reuse (step 3). Justified: minimal enabling change vs duplicating the seeded-id primitive; the plan's file list was under-enumerated, no spec gap.
2. **`string_kv` `pub(crate)` helper added in `message.rs` + `service_resource` refactored onto it.** Beyond the literal "attach events" step; justified DRY (3 exception attributes + the existing `service.name` attribute share one constructor). Within the listed file, additive.

## Decisions & corrections
- **Crate-seam (user-ratified in /andromeda-phase via AskUserQuestion):** the `fingerprint()` fn lives in `conductor-emit` (co-located with the exception content), NOT `conductor-faults` — even though CLAUDE.md / arch §Modules say faults does "fingerprint generation". Rationale: the error-spans "builder-in-emit" precedent; faults is empty + Epoch-4; the Epoch-7 fingerprint-storm fault will depend on emit and compose this primitive. **Potential arch drift** — arch §Modules may need to clarify: the per-exception fingerprint PRIMITIVE is in emit; faults will own the fingerprint-STORM FAULT (Epoch-7).
- **Fingerprint is content-derived, NOT seed-derived** — the seed drives only the exception span's identity bytes (like `error_trace_request`); the fingerprint is a pure function of (exception type + frame functions).
- **Stable-hash choice: FNV-1a 64-bit** (hand-rolled, no new dep) for cross-platform/version stability — explicitly NOT `std` `DefaultHasher`/SipHash (mirrors the ChaCha8-over-StdRng cross-version-stability rationale, arch §Determinism RNG).
- **Amended P-017 clause (c) (from input.md):** identical / path-variant / line-variant ⇒ **SAME** fingerprint; type-variant / frame-variant ⇒ **DIFFERENT**. (Corrected an initial scope.md error that said path-variant → different; reconciled in scope.md during the phase.)
- **Deferred follow-up (not this chunk):** `opentelemetry-proto` `default-features = false` to drop the dormant transitive `opentelemetry_sdk` — no deps added this chunk; the dormant SDK stays behavioral-clean (never initialized), consistent with the 2026-06-17 playbook rule.

## Outcome
- **All acceptance criteria met.** Gates green:
  - `cargo nextest run -p conductor-emit` → 18/18 (8 new: 7 unit + 1 integration)
  - `cargo nextest run --workspace --profile ci` → 99/99
  - `cargo clippy --workspace --all-targets -- -D warnings` → clean
  - `cargo test --doc -p conductor-emit` → ok (0 doctests)
  - `cargo audit` → clean · `cargo deny check` → advisories/bans/licenses/sources ok
  - `Cargo.lock` → un-drifted (no new deps)
- **Smoke:** skipped — no boot-path change (library-only; no `agent-run.sh` in Test Commands).
