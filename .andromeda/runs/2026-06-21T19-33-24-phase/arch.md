# arch extract

## Relevance
Partial — the chunk is a report-rendering surface (narrative, layout, Markdown artifact) within the established run-report envelope contract, but does not introduce new architectural decisions.

## Constraints
- Render per run-report envelope shape per architecture §Standard Contracts (run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, journal_emitted_at, read_back_observed_at, fingerprints)
- Status lamp obeys verdict-first precedence per architecture §Probabilistic-Assertion Policy (verdict when present, else state) — do not introduce color-only encoding
- Blocked rows render identity + slo_tier only; measurement fields (latency_ms, journal_emitted_at, read_back_observed_at) render as absent marker, never literal null (architecture §Standard Contracts, Blocked-row null rule)
- Module boundary: code lives in `conductor-report` crate per architecture §Inherited Defaults (workspace crate-per-seam)
- Timestamp formats use colon-delimited RFC-3339 in the Markdown per architecture §Standard Contracts (journal_emitted_at / read_back_observed_at rendering), not filesystem-safe hyphens
- Artifact hygiene: no absolute host paths, internal struct names, or stack traces leak into the `.md` output (CLAUDE.md universal invariant)

## Patterns to follow
- Pure renderer over `RunRecord`s (consumer of the envelope shape, not definer) — mirror the `JournalWriter` / `RunsDb` pattern of taking already-resolved `runs_dir` and writing `<run_id>.md` deterministically without overwrites
- Verdict-first lamp helper as reusable shared code (not report-local) — used downstream by coverage-matrix (ch4), cli (Epoch 8), and desktop (Epoch 9)
- Deterministic golden-frozen output for fixed record sets (unit + insta golden tests, wall-clock stamps injected/excluded at test boundary)
- Typed seam error for harness faults (IO/write); verdicts/states are rendered values, never `Result::Err` (verdict/error wall per architecture §Cross-cutting Patterns)

## Anti-patterns to avoid
- Do not render verdict-as-color without an ASCII status prefix; do not render CalibrationRegion verdict as `[MANUAL]` (always `[HOLD]`)
- Do not leak absolute host paths, struct-internal names, or error stack traces into the artifact
- Do not overwrite an existing `<run_id>.md` on a repeat run_id (follow artifact-hygiene rule from JournalWriter/RunsDb)

## Contract bindings
- **run-report envelope** ↔ envelope-serializer chunk (RunRecord shape, Verdict/ReportState enums — you consume, do not define)
- **verdict-first lamp precedence** ↔ a11y/design-system (escalated user-confirmed rule, reused by coverage-matrix + cli + desktop)
- **runs/ artifact directory** ↔ cli Epoch 8 (CONDUCTOR_RUNS_DIR resolution/canonicalization owned by cli edge, you write into resolved dir)

## Acceptance criteria contributions
- Per-scenario check renders with verdict-first lamp ([PASS]/[FAIL]/[HOLD]/[BLOCKED]/[RESIDUAL]/[MANUAL]) determined by verdict presence and CalibrationRegion→[HOLD] not [MANUAL] rule
- Blocked row displays identity fields and slo_tier; latency_ms, journal_emitted_at, read_back_observed_at render as absent (em-dash or equivalent), never the string "null"
- Markdown file `runs/<run_id>.md` is written deterministically and never overwritten on repeat run_id (artifact-hygiene gate)
- Golden-frozen unit + insta tests confirm identical output for fixed record sets (wall-clock generated-at timestamp injected/excluded at test boundary)

## Relevant amendment history
- 2026-06-21-run-report-envelope-serializer — ManualCheck definition broadened to include auto-measured calibration-region checks; default Verdict→ReportState mapping recorded (`CalibrationRegion→ManualCheck`); verdict-first lamp precedence (CalibrationRegion renders [HOLD], not [MANUAL]) escalated and user-confirmed as the canonical a11y-safe rule applied consistently across report + coverage-matrix + cli + desktop