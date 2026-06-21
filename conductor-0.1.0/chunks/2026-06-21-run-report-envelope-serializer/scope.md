# Scope — Run-report envelope serializer

**Marker:** 2026-06-21-run-report-envelope-serializer
**Epoch:** 6 — Run report & persistence (chunk 1 of 4)
**Working entry:** "Run-report envelope serializer — canonical shape shared by Markdown + runs.db + JSONL"

## What it builds
The canonical, single-source-of-truth **run-report envelope** — the typed Rust struct(s) + their
(de)serialization — representing one **per-scenario-check** result. This is the shared shape the three
downstream Epoch-6 outputs all derive from:
- the per-run **Markdown** run report (Epoch 6 chunk 3),
- the **runs.db** SQLite index row (Epoch 6 chunk 2),
- the **JSONL** machine-readable ground-truth record.

The envelope carries the fields fixed in architecture §Standard Contracts ("Run report envelope"):
- `run_id` — filesystem-safe hyphen-delimited stamp `YYYY-MM-DDTHH-MM-SS-<suffix>`
- `seed` — u64
- `scenario` — scenario key
- `p_ids` — array of Pulse P-IDs (`P-001`..`P-060`)
- `verdict` — `{Pass, Fail, CalibrationRegion}` (re-use conductor-core `Verdict`)
- `state` — `{Pass, Fail, ManualCheck, KnownResidual, Blocked}` (re-use conductor-core `ReportState`)
- `journal_emitted_at` / `read_back_observed_at` — instant fields
- `latency_ms` — integer milliseconds (nullable)
- `slo_tier` — closed enum over exactly `<5s` / `<20s` / `<90s`
- `fingerprints` — array of fingerprint strings

## Boundaries (what this chunk does NOT do)
- NOT the runs.db schema / rusqlite bound-parameter writes — Epoch 6 chunk 2.
- NOT the Markdown rendering / status-line glyphs — Epoch 6 chunk 3.
- NOT the coverage-matrix generator — Epoch 6 chunk 4.
- NOT new verdict/classification logic — `Verdict` / `ReportState` (and `Assessment`) already exist upstream
  (conductor-core / conductor-verify); this chunk only *consumes & serializes* outcomes.
- NOT live-run wiring — producing real envelopes from a running scenario timeline is Epoch 8.

## Contracts / surfaces it touches
- **Run report envelope** (architecture §Standard Contracts) — this chunk IS the canonical realization of
  that contract shape; `verdict ∈ {Pass,Fail,CalibrationRegion}`, `state ∈ {Pass,Fail,ManualCheck,
  KnownResidual,Blocked}`.
- **Blocked-row null rule:** a `Blocked` envelope populates only the identity fields (`run_id`, `seed`,
  `scenario`, `p_ids`, `slo_tier`); `verdict`, `journal_emitted_at`, `read_back_observed_at`, `latency_ms`,
  and `fingerprints` serialize as JSON `null` (and become NULL in runs.db downstream) — never phantom values.
- **Timestamp encodings:** `run_id` is constrained to the hyphen form (it is the runs.db PK + artifact stem);
  in-payload instants use colon-delimited RFC-3339 in the human/JSON serialization; the runs.db columns are
  integer-millisecond journal offsets (that runs.db mapping is chunk 2, but the envelope representation must
  not preclude it).
- **Verdict/error wall:** the envelope holds outcome VALUES; producing/serializing an envelope is infallible
  with respect to SUT outcomes — `Result::Err` stays reserved for harness faults.
- **Artifact hygiene:** no absolute host paths or internal struct/field names leak into the serialized form
  (canonical-name serde).

## Open questions (for the plan / val-1 to resolve)
- **Crate placement:** conductor-core (shared cross-seam contract type, alongside `Verdict`/`ReportState`)
  vs conductor-report (the storage-seam owner that consumes it). Resolve in P4 (likely AskUserQuestion).
- **JSONL relationship:** how this per-check envelope record relates to the existing per-run *emission*
  journal (`2026-06-16-emission-journal-writer`) — a distinct report record vs an appended line — and whether
  the "JSONL" here is a new report stream.
- **Canonical field naming** + the `run_id` `<suffix>` derivation rule.

## Resolution (P3 research + P4 user decisions) — refined scope
P3 found the original framing incomplete: the **envelope already exists**. `conductor-core::RunRecord` is the
canonical 11-field shape with golden-locked serde, `::blocked()` + the null rule, JSON round-trip, and
`conductor-report::JournalWriter` already appends it to `runs/<run_id>.jsonl` (built early by
`2026-06-16-emission-journal-writer`). So the open questions resolve as:
- **Crate placement:** the envelope stays in **conductor-core** (already there, correct — a shared type); no move.
- **JSONL relationship:** the per-check envelope IS the existing JSONL emission-journal line (`RunRecord`); not
  a new stream.
- **Field naming / run_id suffix:** already fixed by the existing golden + `mint_run_id()`.

**Refined deliverable (P4 decision = "re-scope to the producer side"):** make the existing envelope the
canonical *produced* shape — (1) `RunRecord::measured(...)` constructor (mirror `::blocked()`), (2) a default
`Verdict → ReportState` mapping helper in core (`Pass→Pass`, `Fail→Fail`, **`CalibrationRegion→ManualCheck`**
— P4 decision; widens `ManualCheck`'s doc, no 6th state), and (3) the `CheckOutcome → RunRecord` producer
bridge in **conductor-verify**. This supersedes the original "build the envelope struct + serde" framing
(that work pre-existed). The boundaries above (no runs.db, no Markdown, no live wiring) still hold.
