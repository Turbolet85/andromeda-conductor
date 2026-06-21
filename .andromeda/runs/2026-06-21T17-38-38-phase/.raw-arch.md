# arch extract

## Relevance
Relevant — envelope types are the foundational Standard Contract the architecture locks; this chunk IS the realization.

## Constraints
- Per architecture §Standard Contracts, the run-report envelope carries exactly the specified fields (`run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `journal_emitted_at`, `read_back_observed_at`, `latency_ms`, `slo_tier`, `fingerprints`) with locked type signatures and semantics.
- Per architecture §Conventions (Interface surfaces / Error handling), the envelope is infallible with respect to SUT outcomes — envelope production/serialization involves only values, never `Result::Err` (which is reserved strictly for harness faults).
- Per architecture §Data model conventions (SQLite / `runs.db`), timestamps are integer-millisecond journal offsets as the envelope's internal representation, with RFC-3339 colon-delimited encoding only in human/JSON serialization; `run_id` is the filesystem-safe hyphen form (`YYYY-MM-DDTHH-MM-SS-<suffix>`).
- Per architecture §Standard Contracts (Blocked-row null rule), a `Blocked` state envelope populates only identity fields (`run_id`, `seed`, `scenario`, `p_ids`, `slo_tier`); `verdict`, `journal_emitted_at`, `read_back_observed_at`, `latency_ms`, `fingerprints` serialize as JSON `null` (and store as database NULL downstream) — never phantom values.
- Per architecture §Conventions (Config conventions), `slo_tier` is a closed enum over exactly `<5s` / `<20s` / `<90s` with no extensibility.
- Per architecture §Inherited Defaults (Serialization), the canonical serialization is serde_json 1.0 with canonical-name (de)serialization to prevent internal struct/field names leaking.

## Patterns to follow
- **Verdict/ReportState reuse:** `verdict ∈ {Pass, Fail, CalibrationRegion}` and `state ∈ {Pass, Fail, ManualCheck, KnownResidual, Blocked}` are typed enums already defined in conductor-core (per Established Decisions [Outcomes are values, errors are harness faults]); this chunk consumes and serializes them, never redefines them.
- **serde + serde_json pipeline:** The envelope struct derives `Serialize`/`Deserialize` with canonical naming; serde_json 1.0 is registered in §Stack and already present in workspace.dependencies (per 2026-06-15 amendment).
- **Timestamp encoding duality:** Internal representation is integer milliseconds (journal offsets); serialization to JSON/human uses RFC-3339 with colons. The mapping rule is documented in §Data model conventions and applies uniformly across all serialization boundaries.

## Anti-patterns to avoid
- Do not introduce a new verdict type or redefine `Pass`/`Fail`/`CalibrationRegion`/`ManualCheck`/`KnownResidual`/`Blocked` — they are locked upstream and must be reused from conductor-core.
- Do not emit absolute host paths or internal Rust field/struct names in the serialized JSON — canonical-name serde (rename attributes if needed) is mandatory per §Conventions (Artifact hygiene).
- Do not allow non-null values in `verdict`, `journal_emitted_at`, `read_back_observed_at`, `latency_ms`, `fingerprints` fields when `state == Blocked` — the null rule is a strict invariant per §Standard Contracts.

## Contract bindings
**Downstream consumers (determined in Epoch 6 chunks 2/3):**
- runs.db index row (Epoch 6 chunk 2) — consumes the envelope's field types and applies the timestamp-to-integer mapping.
- Markdown run report (Epoch 6 chunk 3) — consumes the envelope and formats it for human readability with status glyphs.
- JSONL journal (Epoch 6 chunk 1, this chunk) — appends per-check records with the envelope shape.

**Upstream producers (Epoch 8 & prior):**
- conductor-verify (Verdict production) — supplies `verdict` and `state` outcome values (per §Established Decisions [Outcomes are values]).
- conductor-timeline / conductor-emit (journal emission) — supply `journal_emitted_at` timestamps and eventual `read_back_observed_at` (post-verification).

## Acceptance criteria contributions
- (arch) Envelope struct members exactly match §Standard Contracts fields with locked types (`run_id`: filesystem-safe hyphen form; `seed`: u64; `verdict`/`state` as typed enums from conductor-core; `slo_tier`: closed enum `<5s`/`<20s`/`<90s`).
- (arch) Envelope serialization/deserialization is canonical-name serde_json 1.0 with no host paths or internal names leaked (per §Conventions Artifact hygiene).
- (arch) Blocked-state rows enforce null invariant: `verdict`, `journal_emitted_at`, `read_back_observed_at`, `latency_ms`, `fingerprints` all serialize as JSON `null` when `state == Blocked`.
- (arch) Timestamps are integer-millisecond journal offsets internally; RFC-3339 colon-delimited format is applied at serialization boundary (serde serializer custom logic per §Data model conventions).

## Relevant amendment history
- **2026-06-15-conductor-core-shared-types** — serde_json 1.0 added to §Stack and workspace.dependencies (run-report envelope + JSONL journal dependency); this chunk is the canonical consumer.
- **2026-06-18-exception-events-fingerprint-control** — fingerprint primitive placed in conductor-emit (the `fingerprints` array in the envelope will reference these primitives' output); envelope does not redefine fingerprint logic, only carries the strings.
