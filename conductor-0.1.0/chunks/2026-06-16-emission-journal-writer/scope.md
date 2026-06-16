# Scope — Emission-journal writer

**Marker:** 2026-06-16-emission-journal-writer
**Version:** conductor-0.1.0 · Epoch 2 (Timeline engine) · chunk 3 of 4
**Working entry:** "Emission-journal writer — per-run JSONL, std::time wall-clock stamps, tests/obs-owned schema"

## What it builds
The per-run **JSONL emission journal** writer — the on-disk, append-mostly, agent-parseable
ground truth that records each scenario-result as one JSON line (the **run-report envelope**) in
`runs/<run_id>.jsonl`. Each line
carries a wall-clock `journal_emitted_at` stamp taken from `std::time` (`SystemTime`/`Instant`),
**never** tokio's virtual clock. This journal is the canonical source for the journal-relative SLO
math (`read_back_observed_at − journal_emitted_at`) that downstream verification (Epoch 5/6)
consumes; the future `runs.db` SQLite index is a thin convenience layer over it.

Concretely:
- A serde-serializable **journal-entry type** (one JSON object per line) whose field schema is
  owned by the tests/obs specialists (obs-plan log/JSON schema; test-plan run-report / journal
  envelope).
- A **journal writer** that opens/creates `runs/<run_id>.jsonl` (run_id-stemmed, **never
  overwritten**), appends one line per event, and flushes durably.
- **std::time wall-clock stamping** at write time — the `journal_emitted_at` source of truth,
  taken from `std::time`, not the scheduler's virtual clock.
- The writer + stamper ready for the **emit/verify/cli** seams to call; the **live call-site is
  deferred** — `run_timeline` emits nothing and no verdict producer exists until verify (Epoch 5),
  so a real envelope cannot be assembled at Epoch 2 (see §Amendment).

## Boundaries (what it does NOT build)
- **NOT** the `runs.db` rusqlite index (later Epoch-6 chunk) — file artifact first; SQLite is a
  later thin layer over the JSONL.
- **NOT** the Markdown run report (`<run_id>.md`, later Epoch-6 chunk).
- **NOT** the read-back / SLO comparison math (Epoch 5/6) — this chunk is the WRITE side + schema
  only; it produces `journal_emitted_at`, it does not consume `read_back_observed_at`.
- **NOT** the self-observation `tracing` JSON log stream (already built in structured-logging-stack)
  — the emission journal is a DISTINCT artifact from the run_id-tagged tracing self-obs log; the two
  must not be conflated.
- **NOT** actual OTLP emission to `:4317` (Epoch 3) — the journal writer is the infrastructure;
  emission primitives extend the journal later (`EmissionSpec` is `#[non_exhaustive]` for exactly
  this forward-compat).

## Surfaces / contracts it touches
- **Crate seam:** the `conductor-report` seam owns the JSONL journal (architecture §Modules /
  directory layout). The journal-entry *type* may need to be a `conductor-core` shared type if the
  timeline must construct entries without depending on report — the exact placement must respect the
  crate-per-seam dependency edges (a forbidden cross-seam dep won't compile); resolve in planning.
- **Artifact path:** `runs/<run_id>.jsonl`, overridable via `CONDUCTOR_RUNS_DIR`; `run_id` is the
  filesystem-safe hyphen-delimited stamp (`YYYY-MM-DDTHH-MM-SS-<suffix>`).
- **Timestamp contract** (architecture §Standard Contracts §Timestamp formats): the journal's stored
  values are the integer-millisecond journal offsets the SLO math consumes; human-facing RFC-3339 is
  a render concern, not the stored form. `journal_emitted_at` originates here.
- **Determinism invariant:** wall-clock stamps from `std::time` only; the virtual clock stays
  scheduling-only so journal-relative SLO math is sound.
- **Artifact hygiene:** no absolute host paths or internal struct/field names leak into journal lines
  (consistent with the field-allowlist redaction already in place).
- **Verdict/error wall:** journal I/O failure is a harness fault (`Result::Err`), never a verdict.

## Intent anchor (validation-1)
A plan satisfies this scope when it builds a **runtime-agnostic per-run JSONL journal writer** with a
**tests/obs-owned line schema**, **std::time** wall-clock stamping for `journal_emitted_at`,
**run_id-stemmed never-overwritten** files under a caller-supplied `runs/` directory
(**`CONDUCTOR_RUNS_DIR` + `resolve_under` resolution lands at the cli edge**), **ready for the
emit/verify/cli seams to call** — and explicitly **defers** the live call-site, `runs.db`, the
Markdown report, and read-back SLO math to their later chunks.

## Amendment (P5 validation-1 — intent-incomplete)
The original scope asserted the writer would be **"wired into the timeline (a journal entry per
phase transition / emission event)."** Planning established this is not achievable at Epoch 2: the
journal line is the **scenario-result run-report envelope** (10-field tests/obs schema), not a
per-emission/per-phase event; `run_timeline` emits nothing; and no verdict/state producer exists
until verify (Epoch 5) — `ReportState` has no "pending" variant. The corrected scope delivers the
**envelope type + std::time stamper + writer + tests** as a cohesive unit and **defers the live
call-site** to the emit/verify/cli epochs. Also corrected: `CONDUCTOR_RUNS_DIR` resolution is a
cli-edge concern (the writer takes an already-resolved dir), per `config_path::resolve_under`.
