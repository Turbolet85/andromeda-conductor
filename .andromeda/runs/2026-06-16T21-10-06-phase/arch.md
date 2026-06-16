# arch extract

## Relevance
relevant — this chunk builds the runtime-agnostic JSONL journal writer infrastructure that every scenario depends on for `journal_emitted_at` source-of-truth stamping.

## Constraints
1. Journal writer lives in the `conductor-report` seam (per architecture §Occupied Resources / §Crate names). The journal-entry type may need to be a `conductor-core` shared type if timeline must construct entries without depending on report — resolve dependency edges per architecture §Inherited Defaults (compiler-enforced module seams).
2. Artifact path is `runs/<run_id>.jsonl`, run_id-stemmed and never overwritten; `CONDUCTOR_RUNS_DIR` env var overrides the `runs/` directory (per architecture §Occupied Resources).
3. Timestamp values stored in journal are integer-millisecond journal offsets (not RFC-3339 ISO strings); `journal_emitted_at` originates from `std::time::SystemTime`/`Instant`, never tokio's virtual clock (per architecture §Data model conventions, §Determinism discipline).
4. Determinism invariant: wall-clock stamps only; virtual clock remains scheduling-only so journal-relative SLO math (`read_back_observed_at − journal_emitted_at`) is sound (per architecture §Design Philosophy "Journal-relative ground truth").
5. Journal-entry schema (field names, types, required/optional) is owned by tests/obs specialists (per scope §Surfaces contracts, test-plan / obs-plan); this chunk implements the writer against that contract, not vice versa.
6. Journal I/O failure is a harness fault (`Result::Err`); never a verification verdict (per architecture §Conventions / Error handling, §Verdict/error wall cross-cutting pattern).

## Patterns to follow
1. **Runtime-agnostic library usage:** Timeline engine (from chunks 1–2) records entries; journal writer is invoked from timeline without forcing a tokio::main or Tauri dependency, preserving the headless-drivable-core pattern (per architecture §Design Philosophy "Headless-drivable core, thin shells").
2. **Determinism-discipline stamping:** Use `std::time::SystemTime` for wall-clock `journal_emitted_at` at write time, not the scheduler's seeded clock, so journal-relative SLO comparison in Epoch 5/6 measures real latency, not virtual time (per architecture §Design Philosophy, §Timing-Tolerance Model).
3. **Typed error handling:** Journal I/O faults surface as `Result::Err` via a `conductor-report` seam error enum (thiserror 2.0.18); timeline/core do not catch-and-classify them (per architecture §Conventions error handling, §Verdict/error wall).

## Anti-patterns to avoid
1. Do not use tokio's virtual clock (`tokio::time::Instant`) for `journal_emitted_at`; use `std::time` only. The journal must record real wall-clock instants so SLO math can compare to real read-back times.
2. Do not introduce a separate self-observation tracing JSON log to the journal artifact; the emission journal is distinct from the structured `tracing` self-obs log (per scope §Boundaries). Do not conflate them.
3. Do not emit `runs.db` SQLite index, Markdown run report, or read-back SLO comparison logic from this chunk; these defer to Epoch 6 chunks (per scope §Boundaries).

## Contract bindings
- **Timeline engine (chunks 1–2)** ↔ **Emission-journal writer:** Timeline must call the journal writer's append method per phase transition / emission event, passing the `journal-entry` data; timeline supplies the data but does not own the serialization/I/O.
- **Tests/obs specialists** (test-plan §4 / obs-plan §3) ↔ **Emission-journal writer:** Tests and obs define the journal-entry field schema (line structure, required fields, validation rules); this chunk implements the write side against that frozen schema.

## Acceptance criteria contributions
1. (arch) Code lives in `conductor-report` seam per workspace boundary rules (arch §Inherited Defaults / §Occupied Resources).
2. (arch) Journal writer persists entries to `runs/<run_id>.jsonl` (never overwritten, `CONDUCTOR_RUNS_DIR`-overridable), with integer-millisecond `journal_emitted_at` from `std::time`, per architecture §Data model conventions.
3. (arch) No new env var beyond `CONDUCTOR_RUNS_DIR` (already registered in arch §Occupied Resources).
4. (arch) Timestamp origin is `std::time::SystemTime` or `std::time::Instant`, never tokio virtual clock — verified by absence of `tokio::time::Instant` in the journal writer module.

## Relevant amendment history
- **2026-06-15-conductor-core-shared-types:** `serde_json 1.0` registered in §Stack; the run-report envelope + per-run JSONL journal consume it at runtime in Epoch 6 (this chunk does the write; json serialization is part of the stack now).
- **2026-06-16-seeded-phase-scheduler:** rand_chacha + rand_core registered; the timeline scheduler's seeded RNG is the sole non-determinism source — wall-clock stamping (this chunk) is deterministic and independent of that seeding.