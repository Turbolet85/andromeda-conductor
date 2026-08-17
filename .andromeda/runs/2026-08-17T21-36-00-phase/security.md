# security extract

## Relevance
Partial — the chunk is mostly semantics/test work, but four security surfaces bind: the blake3 derivation it re-aims, the no-absolute-host-path artifact-hygiene invariant, the `scenarios/fingerprint-storm.toml` `[phases.emission]` config boundary, and the standing `cargo audit` deferral carried as a PREREQ.

## Constraints
- The exception-fingerprint derivation in `conductor-emit/src/exception.rs` must remain the blake3 derivation (first 16 bytes / 32 hex); re-aiming the variant model must not reintroduce FNV-1a or any deprecated/non-cryptographic hash (per security-plan.md §Security Anti-Patterns → Data Protection).
- The run-report `fingerprints[]` column is fed from read-back `Observation#fingerprints` and never from the local derivation — the two must not meet; a redesign that restores single-fingerprint storm grading must not route the derived hex into that column (per security-plan.md §Security Anti-Patterns → Data Protection).
- Emitted stacktraces must carry no absolute host path, and run artifacts (`<run_id>.md`, `runs.db` rows, JSONL journals) must not leak absolute host paths or internal seam-crate struct names — the redesign's variant set must be constructible without an absolute-path base (per security-plan.md §Error Handling, Run-report artifact sanitization; §Security Anti-Patterns → Logging). Whether `stacktrace_carries_no_absolute_host_path` already covers the exact reach the redesign leans on is research's question.
- Any edit to the `[phases.emission]` block in `scenarios/fingerprint-storm.toml` stays inside the validated boundary: `kind` REQUIRED whenever the table exists (no inferred default), `occurrences` bounded by `MAX_OCCURRENCES` (`0` = a declared silence window), non-empty variant/category sets, per-shape cross-field rules enforced by garde `range` + `#[garde(custom)]` at load (per security-plan.md §Input Validation, scenario-config row).
- If the variant model changes the shape of any nested spec struct, every nested spec field must `dive`, never `skip` — a `#[garde(skip)]` nested struct deserializes entirely unvalidated (per security-plan.md §Input Validation; §Security Anti-Patterns → Input).
- The standing red `cargo audit` is an advisory-DATABASE fault, whose only sanctioned remedy is the bounded wait plus the audit↔deny overlap VERIFIED green — never a floor raise, never a `deny.toml` ignore, never a CI edit, never a silent accept (per security-plan.md §Dependency Security, two-fault split).
- `Cargo.lock` must stay committed and un-drifted; if this chunk lands any dependency delta at all, the deferral's basis changes and `cargo deny check advisories bans licenses sources` must be VERIFIED green over the NEW lock (per security-plan.md §Dependency Security, admission-condition bullet; §Security Anti-Patterns → Universal).

## Patterns to follow
- garde validation co-located with the serde structs in their owning seam crate — `#[derive(Validate)]` `range` rules plus `#[garde(custom)]` cross-field, failed `Report` → `ConfigError` via `#[from]` as a harness fault, never a verdict (per security-plan.md §Input Validation).
- The verdict/error wall: typed `thiserror` enums stay inside the seam crates and collapse to `anyhow` only at the `conductor-cli` / `#[tauri::command]` edges; verification outcomes remain typed `Ok(...)` values (per security-plan.md §Error Handling).
- Artifact-hygiene framing for any new failure text: identity/verdict fields only, no absolute paths, no internal struct names — the same discipline the committed-manifest readers use (`e.kind()` only, never the path) (per security-plan.md §Input Validation, committed SUT-facing manifests row; §Error Handling).
- The deny.toml justified-exception mechanism with a per-entry comment is the documented route for a non-actionable advisory or a permissive license — `deny.toml` itself is the authority on the full set (per security-plan.md §Dependency Security, Accepted exceptions).

## Anti-patterns to avoid
- NEVER use a deprecated or non-cryptographic digest (MD5, SHA-1, DES, RC4, ECB, FNV-1a) for the exception fingerprint — the ban is active, not hypothetical (per security-plan.md §Security Anti-Patterns → Data Protection).
- NEVER mark a nested spec field `#[garde(skip)]` — garde does not descend, so the rules silently never run; `dive` is the only correct annotation (per security-plan.md §Security Anti-Patterns → Input).
- NEVER let a scenario TOML, test fixture, or run artifact carry an absolute host path or internal struct/field name into agent-parseable ground truth (per security-plan.md §Security Anti-Patterns → Logging).

## Contract bindings
- Artifact hygiene binds security §Error Handling ↔ tests: `stacktrace_carries_no_absolute_host_path` in `conductor-emit/src/exception.rs` is the enforcement site for the no-absolute-path invariant, so the re-aimed variant tests are where this domain's check lands — and it binds obs, since the same rule governs journal/report emission.
- The dependency gate binds tests §CI Integration — the audit/deny re-check is one job in the single existing workflow; the PREREQ's re-pin is recorded, not re-plumbed.

## Acceptance criteria contributions
- `cargo deny check advisories bans licenses sources` exits 0 over the committed `Cargo.lock`, and the `cargo audit` red is re-pinned as the same advisory-DATABASE fault with no floor raise, no `deny.toml` ignore, and no CI edit; if any dependency delta lands, state the changed basis explicitly (per security-plan.md §Dependency Security).
- The shipped exception-fingerprint derivation is still blake3 (first 16 bytes / 32 hex) after the variant-model redesign, and no MD5/SHA-1/FNV-style digest is introduced (per security-plan.md §Security Anti-Patterns → Data Protection).
- `stacktrace_carries_no_absolute_host_path` still passes, and no re-aimed variant, fixture, or scenario edit introduces an absolute host path into an emitted stacktrace or a run artifact (per security-plan.md §Error Handling).
- Any `[phases.emission]` edit in `scenarios/fingerprint-storm.toml` still loads through garde with `kind` present, `occurrences` ≤ `MAX_OCCURRENCES`, a non-empty variant set, and no nested spec field marked `#[garde(skip)]` (per security-plan.md §Input Validation).

## Relevant amendment history
- `2026-08-16-canary-fingerprint-derivation-aligned` — "deprecated-crypto ban is now active": the ban was re-anchored from a hypothetical to the shipped blake3 derivation (FNV-1a removed), and it recorded that `fingerprints[]` is read-back-fed and never fed from that derivation. Same derivation this chunk re-aims — the ban is the direct constraint on the redesign.
- `2026-08-16-canary-fingerprint-derivation-aligned` — "admitting a dependency under a red audit" + "third accepted deny.toml exception": blake3's `arrayref` BSD-2-Clause allow was recorded, and "no dependency delta" was retired as a standalone deferral basis once a dependency lands. Governs how this chunk's PREREQ re-pin must be worded.
- `2026-08-09-interpretation-correctness-posture` — established the TOOL-fault vs advisory-DATABASE-fault split and the bounded-wait remedy. This is the rule the chunk's 25th-consecutive `cargo audit` deferral executes against.
- `2026-08-11-faithful-emission-dispatcher` — widened the scenario-config boundary to `[phases.emission]` (required `kind`, `MAX_OCCURRENCES` bound, non-empty variant sets) and mandated `dive` never `skip` after finding `PhaseSpec.emission` skipped. Governs the `fingerprint-storm.toml` variant-mix edit.
