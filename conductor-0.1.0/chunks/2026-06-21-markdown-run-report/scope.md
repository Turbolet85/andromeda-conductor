# Scope — Markdown run report

**Marker:** `2026-06-21-markdown-run-report`
**Working-route entry:** _Markdown run report — per-scenario Pass/Fail/ManualCheck/KnownResidual/Blocked render_
**Epoch:** 6 (Run report & persistence) — chunk 3 of 4
**Crate(s):** `conductor-report` (render+write seam); a shared lamp helper likely lands in `conductor-core`.

## What it builds
The per-run **Markdown run report** writer — the human-facing third consumer of the run-report
envelope, a sibling of the JSONL `JournalWriter` and the `runs.db` `RunsDb` storage seam (all three
share the canonical `RunRecord` shape from the envelope-serializer chunk). Given a run's collection of
per-scenario `RunRecord`s plus run-level identity, it renders a deterministic Markdown document and
writes it to `runs/<run_id>.md` — run_id-stemmed, never overwritten (the established artifact-hygiene
pattern from `JournalWriter`/`RunsDb`).

Each scenario check renders as a row classified into the five report states
(Pass / Fail / ManualCheck / KnownResidual / Blocked), with its **status lamp chosen verdict-first**:
- `verdict = Some(Pass|Fail)` → `[PASS]` / `[FAIL]`
- `verdict = Some(CalibrationRegion)` → **`[HOLD]`** (NOT `[MANUAL]`) — the verdict-first precedence rule
- `verdict = None` (blocked row) → fall back to `state` → `[BLOCKED]`

This is the **first** surface where verdict-first lamp precedence is realized (carried follow-up (c));
coverage-matrix (ch4), cli (Epoch 8) and desktop (Epoch 9) reuse the same rule, so the precedence logic
should be a shared, reusable helper rather than report-local.

## In scope
- A Markdown render module in `conductor-report` (e.g. `report.rs`/`markdown.rs`) + `pub use` in `lib.rs`.
- A `RunReport`-style writer mirroring `JournalWriter`/`RunsDb`: takes an already-resolved `runs_dir`
  (the cli edge owns `CONDUCTOR_RUNS_DIR` canonicalization), writes `<run_id>.md`, never overwrites.
- A run-level summary section (run_id, seed, per-state counts, generated-at) + a per-scenario detail
  table/section listing identity (scenario, p_ids, seed) and the measured envelope fields (verdict,
  state, latency_ms, slo_tier, journal_emitted_at, read_back_observed_at, fingerprints).
- **Verdict-first lamp precedence** as a shared helper (verdict when present, else state), honoring the
  status-never-color-alone invariant via the ASCII `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/
  `[BLOCKED]` prefix (Markdown has no color, so the prefix IS the status encoding).
- **Blocked-row null rule** at the render: a blocked row shows only identity + slo_tier; the five
  never-measured fields render as an em-dash/absent marker — never the literal `null` or a struct name.
- **Artifact hygiene:** no absolute host paths, internal struct names, or stack traces leak into the
  `.md` artifact (CLAUDE.md universal invariant; sanitize at the render edge).
- Typed seam error (harness-fault only — IO/write failures); verdicts/states are rendered values, never
  `Result::Err` (the verdict/error wall).
- Deterministic output (same records ⇒ identical Markdown) — unit + golden (`insta`) tests; the only
  wall-clock field (a generated-at stamp, if any) must be excluded from / injected into the golden.

## Boundaries (out of scope)
- Does NOT define the envelope shape (`RunRecord` — envelope-serializer chunk) or verdict/state logic
  (`conductor-verify`); it is a pure consumer/renderer.
- Does NOT own `CONDUCTOR_RUNS_DIR` resolution/canonicalization (cli edge, Epoch 8).
- NOT the coverage-matrix generator (Epoch 6 ch4), NOT cli line-rendering (Epoch 8), NOT desktop lamps
  (Epoch 9) — but it establishes the shared verdict-first lamp helper those three reuse.
- No cross-run aggregation/queries (that is the `runs.db`/`get` surface, Epoch 7/8).
- No operator-checklist ManualCheck authoring (Epoch 7+/9) — only correct rendering of whatever
  `RunRecord`s it is handed.

## Surfaces / contracts touched
- **Run report envelope** (arch §Standard Contracts) — the Markdown serialization of the same shape
  shared with `runs.db` + JSONL.
- **Verdict-first lamp precedence** (arch §Probabilistic-Assertion Policy; carried follow-up (c)).
- **Status-never-color-alone** (CLAUDE.md universal invariant) — ASCII prefix.
- **Blocked-row null rule** (arch §Standard Contracts) — render as absent, not `null`/struct.
- **Artifact hygiene** (CLAUDE.md) — no host-path / struct-name leak into `runs/<run_id>.md`.
- **runs/ artifact directory** — `<run_id>.md`, run_id-stemmed, never overwritten.

## Acceptance (intent level — refined in plan.md)
- A measured Pass/Fail/CalibrationRegion record and a Blocked record each render with the correct
  verdict-first lamp ([PASS]/[FAIL]/[HOLD]/[BLOCKED]); a CalibrationRegion row shows `[HOLD]`, never
  `[MANUAL]`.
- A blocked row shows identity + slo_tier only; measurement fields render as em-dash, never `null`.
- `runs/<run_id>.md` is written run_id-stemmed and is never overwritten on a repeat run_id.
- Output is deterministic for a fixed record set (golden-frozen).
- No absolute host path or internal struct name appears in the rendered Markdown.
