# security extract

## Relevance
Relevant (partial) — the chunk touches the scenario-config validation boundary, synthetic-telemetry artifact hygiene, and the standing dependency gate; no auth / secret / network-listener surface is introduced.

## Constraints
- Any field the fork adds to `EmissionSpec` (rate · occurrence count · error fraction · severity mix · fingerprint/exception shape) is a scenario-config trust-boundary field and MUST carry garde `range` rules plus `#[garde(custom)]` for cross-field bounds at load — and `PhaseSpec.emission`'s current `#[garde(skip)]` must become a dive, or nested rules never execute and the boundary is silently bypassed (per security-plan §Input Validation, scenario-config row; §Security Anti-Patterns → Input, "NEVER deserialize scenario config without garde validation at load").
- A failed garde `Report` collapses to `ConfigError` via `#[from]` and surfaces as `Result::Err` (harness fault) — never as a `Verdict`/`ReportState`; likewise a refused OTLP transport during dispatch is a harness fault, not a `Fail` (per security-plan §Error Handling, verdict/error wall).
- PII-family dispatch MUST source its payload from seeded synthetic construction only (`PiiCorpus::seeded`), never from host identity, env, or Pulse's `corpus.db`; no corpus content may be persisted into journals/reports (per security-plan §Data Protection; §Anti-Patterns → Data Protection).
- Dispatcher egress stays loopback-only OTLP/gRPC to `127.0.0.1:4317`; it opens no socket of its own and does not promote the `conductor-faults` `:4317` occupier bind into a listener (per security-plan §Threat Model Summary → Attack surface; §Anti-Patterns → Universal / API).
- Artifacts the dispatcher newly writes or annotates — journal lines, `runs.db` rows, and the `timeline.execute` / `emit.batch` span fields incl. `emission_count` — carry verdict/state/identity values only: no absolute host paths (canonicalized `CONDUCTOR_*`, `ANDROMEDA_PULSE_DATA_DIR`) and no internal seam-crate struct/field names (per security-plan §Error Handling, run-report artifact sanitization; §Anti-Patterns → Logging).
- Any randomness the dispatcher introduces comes from the seeded `ChaCha8Rng`; wall-clock stamps on journal/report use `std::time::SystemTime`/`Instant`, never tokio's virtual clock (per security-plan §Anti-Patterns → Logging, clock-source ban).
- The 10th-consecutive `cargo audit` re-check is remedied by the **bounded wait alone**: re-run, record the byte-identical result, and verify `cargo deny check advisories bans licenses sources` actually ran green as the overlapping signal — no floor raise, no `deny.toml` ignore, no CI edit. If the fork's resolution pulls a new dependency, the deferral's "audit SURFACE unchanged" basis stops holding literally and must be re-stated at wrap, not re-pinned verbatim (per security-plan §Dependency Security, two-fault split).

## Patterns to follow
- The existing bounded-range garde precedent co-located with the serde structs in `D:\dev\projects\conductor\crates\conductor-core\src\phase_spec.rs` (`MAX_GAP_MS` / `MAX_JITTER_MS` consts + `#[garde(range(max = …))]`) — new shape fields extend that same shape in the same owning seam crate, per security-plan §Input Validation.
- `D:\dev\projects\conductor\crates\conductor-emit\src\pii.rs` — `PiiCorpus::seeded(n)` is the sanctioned synthetic-identity generator (determinism-proven in its own unit tests); consume it as-is rather than sourcing identities anywhere else.
- Module-internal `thiserror` typed errors per seam (`EmitError`, `ConfigError`) collapsing to type-erased `anyhow` only at the `conductor-cli` edge (per security-plan §Error Handling).
- If the fork reads any new artifact from disk, reuse the committed-manifest pattern: fixed `default_path()` through `resolve_under`, no new `CONDUCTOR_*` override handle, read faults carrying `e.kind()` only and never the path (per security-plan §Input Validation, committed SUT-facing manifests row).

## Anti-patterns to avoid
- A widened `EmissionSpec` deserialized without garde rules reaching it — including the subtle form where the field has a rule but the parent's `#[garde(skip)]` prevents descent (per security-plan §Anti-Patterns → Input).
- String-formatted SQL if emission counts/shape land in `runs.db` — rusqlite bound parameters only, even for self-generated synthetic data (per security-plan §Anti-Patterns → Input).
- Scenario-TOML migration that introduces or retains a scenario without a Pulse P-ID (per security-plan §Anti-Patterns → Universal, scope law).

## Contract bindings
- **obs ↔ security:** the `emission_count` field on `timeline.execute` (obs-plan §4 CP1) is an artifact-hygiene surface — closed low-cardinality values, no paths, no struct names; the CARRY's `on_record` decision must not make a late-recorded value the place a path leaks in (security-plan §Error Handling + §Anti-Patterns → Logging).
- **tests ↔ security:** dispatcher fixtures/goldens carry seeded-synthetic PII only, never real identifiers; the `cargo audit` / `cargo deny` re-check is the security job of the single existing CI workflow, not a new pipeline (security-plan §Dependency Security → CI integration).
- **arch ↔ security:** if the fork adds a rate/occurrence field, the load envelope's `declared-not-derivable` justification stops holding — surface that consequence; do not widen `check_load_envelope` into a claimed measurement (security-plan §Anti-Patterns → Universal, never-claim-a-measurement / `declared-not-observable` clause).

## Acceptance criteria contributions
- (security) Every new emission-shape field has a garde rule that actually executes — `PhaseSpec.emission` dives rather than skips, and a malformed scenario TOML (out-of-range fraction, inverted percentile order, non-summing severity mix) fails at load as `ConfigError`/`Result::Err`, never as a verdict (per security-plan §Input Validation).
- (security) `cargo audit` re-run and its result recorded; `cargo deny check advisories bans licenses sources` verified green rather than assumed; no floor raise, no `deny.toml` ignore, no CI edit — and if a dependency was added, the deferral basis is re-stated at wrap (per security-plan §Dependency Security).
- (security) No dispatcher-produced artifact (JSONL journal line, `<run_id>.md` row, `timeline.execute` / `emit.batch` span) contains an absolute host path or internal seam-crate struct name, and all PII-family payloads trace to seeded synthetic construction (per security-plan §Error Handling + §Anti-Patterns → Logging / Data Protection).
- (security) The dispatcher binds no port and adds no inbound surface — egress remains the loopback OTLP client to `127.0.0.1:4317` (per security-plan §Threat Model Summary → Attack surface).

## Relevant amendment history
- `2026-06-15-config-validation-surface` — garde pinned **0.22.1** (0.23.0's derive crate is absent from the registry); any new `EmissionSpec` rules must be written against 0.22.1's API, and the "path handles validate OUTSIDE garde" split is unchanged.
- `2026-08-09-interpretation-correctness-posture` — established the TOOL-fault vs advisory-DATABASE-fault split; this is the amendment that governs the chunk's 10th-consecutive `cargo audit` re-check (`duplicate advisory ID: RUSTSEC-2026-0244` is committed advisory-db data, so there is no release to raise a floor to).
- `2026-06-15-dependency-audit-gate` — audit-tool versions are minimum **floors**, not lockable pins; relevant only if the fork's resolution pulls a new dependency and the audit surface changes.
- `2026-08-10-pulse-run-contract` — introduced the "a term whose truth lives on the SUT's side is recorded `declared-not-observable` and never blocks" precedent; the same never-claim-a-measurement discipline applies to the load-envelope consequence this chunk surfaces.
- `2026-06-27-live-pulse-e2e-proof` — `corpus.db` is plaintext SQLite and out of scope; corpus access is MCP read-back only and its content is never persisted into Conductor artifacts, which bounds what the PII-family dispatch may ever carry.
