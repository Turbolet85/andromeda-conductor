# security extract

## Relevance
Partial — the chunk builds infrastructure for run-history artifacts; security covers error sanitization, wall-clock determinism, and artifact hygiene only.

## Constraints
1. Wall-clock timestamp source MUST be `std::time::SystemTime`/`Instant` only, never tokio's virtual clock, for journal-relative SLO math to remain sound (per security-plan §Error Handling — "Determinism discipline").
2. Run-report journal artifacts MUST NOT leak absolute host paths (canonicalized `CONDUCTOR_RUNS_DIR`, `ANDROMEDA_PULSE_DATA_DIR`) or internal seam-crate struct/field names into JSONL lines (per security-plan §Error Handling — "Run-report artifact sanitization").
3. Journal I/O failures MUST surface as harness faults (`Result::Err`), never as verdict outcomes, to maintain the verdict/error wall separation (per security-plan §Error Handling — "The verdict/error wall keeps `Result::Err` strictly for harness faults").
4. Per-run JSONL file MUST be append-mostly and never overwritten, ensuring append-safety and audit-trail immutability (per security-plan §Error Handling — "Run-report artifact sanitization" + Threat Model Summary § Data classification for run-metadata "append-mostly").

## Patterns to follow
1. Field-allowlist redaction already in place (per scope §Artifact hygiene) — journal entry schema owned by tests/obs specialists; sealing that contract prevents future path-leak mutations.
2. `thiserror` typed enums collapse to `anyhow` only at the `conductor-cli` binary edge; journal I/O errors stay as `Result::Err` within the seam (per security-plan §Error Handling — "module-internal `thiserror` typed enums collapsed to `anyhow` only at the `conductor-cli` edge").
3. `run_id` filesystem-safe format (YYYY-MM-DDTHH-MM-SS-<suffix>) with no user-controlled components prevents path traversal in the runs directory (per security-plan §Input Validation — Threat Model Summary attack surface "run_id is the filesystem-safe hyphen-delimited stamp").

## Anti-patterns to avoid
1. NEVER write wall-clock stamps from tokio's virtual clock — only `std::time` for `journal_emitted_at` (per security-plan §Logging — "NEVER write the journal/report wall-clock stamps from tokio's virtual clock").
2. NEVER let absolute paths or internal struct names leak into JSONL lines (per security-plan §Error Handling — "Run-report artifacts must record verdict/state/identity fields WITHOUT leaking absolute host paths or internal struct/field names").

## Contract bindings
- **obs ↔ journal-entry field schema:** obs §log/JSON schema owns the line shape; tests §run-report / journal envelope verify the produced artifact.
- **tests ↔ journal envelope harness:** test CI gates the append-safe, never-overwritten invariant.

## Acceptance criteria contributions
1. (security) Wall-clock timestamps in journal JSONL are sourced from `std::time::SystemTime` or `std::time::Instant`, never tokio's virtual clock — per security-plan §Error Handling.
2. (security) No absolute filesystem paths from `CONDUCTOR_RUNS_DIR` / `ANDROMEDA_PULSE_DATA_DIR` / canonicalized directories appear in journal JSONL lines — per security-plan §Error Handling.
3. (security) No internal seam-crate struct/field names leak into journal JSONL lines — per security-plan §Error Handling.
4. (security) Journal I/O errors surface as `Result::Err` (harness fault), never verdicts — per security-plan §Error Handling.

## Relevant amendment history
- **2026-06-15-structured-logging-stack** (obs identity env-handles): `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` are non-path labels stamped into self-obs JSON log values (JSON-escaped by serde_json), not journal-JSONL; they require no validation. Clarifies the journal's own artifact-hygiene boundary.