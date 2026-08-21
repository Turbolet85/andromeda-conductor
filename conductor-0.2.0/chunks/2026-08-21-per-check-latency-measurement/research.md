# Codebase Research — 2026-08-21-per-check-latency-measurement

## Scope
- **Depth:** deep (mature codebase, the chunk edits a Standard Contract) · **Reads:** 12 · **Globs/Greps:** 6 · **Graph queries:** 8

## Files inspected
- `crates/conductor-core/src/scenario.rs` (40–100, 600–660) — `SloTier` is a closed 3-variant enum, serde-renamed to `<5s`/`<20s`/`<90s`, `deadline_ms()` → `5_000`/`20_000`/`90_000`. `Scenario` carries `slo_tier` as `#[garde(skip)]` (a closed enum has nothing to range-check) and `phases` carries the one-altitude-up cross-field validator `fault_phases_are_silent` — the shape the budget invariant must copy.
- `crates/conductor-core/src/expected.rs` (full) — `ExpectedCheck { kind, class, expected }`, `#[derive(Validate)]`, only `expected` validated (`length(min = 1)`). **It has no name/id and no reference to the scenario's tier.**
- `crates/conductor-core/src/run_record.rs` (1–60) — the eleven-field envelope, declaration-order-serialized, five measurement fields `Option` so a blocked row emits JSON `null`. Doc explicitly cites test-plan §3 / obs-plan §3 as schema owners.
- `crates/conductor-core/src/redact.rs` (40–90) — `ALLOWLISTED_FIELDS` already admits all eleven envelope field names, including `latency_ms` and `slo_tier`. `is_allowlisted` is a flat `contains`.
- `crates/conductor-verify/src/slo.rs` (full) — `evaluate_slo(tier, emitted, observed)` → `SloOutcome { latency_ms, within_tolerance }`; `evaluate_check` folds comparison + SLO into `CheckOutcome { assessment, slo, slo_tier }`. **The deadline comes only from `tier.deadline_ms()`** — the single place a budget must intercept.
- `crates/conductor-verify/src/record.rs` (22–46) — `CheckOutcome::to_run_record` copies `self.slo.latency_ms` and `self.slo_tier` into `RunRecord::measured`.
- `crates/conductor-verify/src/extract.rs` (1–120) — **the decisive file.** See Graph impact / Patterns below.
- `crates/conductor-run/src/lib.rs` (330–560, 637–652, 726–734) — `execute_scenario`: `emitted_ms`/`journal_emitted_at` stamped once before the timeline, `observed_ms`/`read_back_observed_at` stamped once after ONE `observe(client)` call; the `expected` iteration maps every check over the SAME instant pair, then `.max_by_key(severity_rank)` keeps one `CheckOutcome` and discards the rest; `persist(runs_dir, run_id, &records, envelope)` writes journal + db + report.
- `crates/conductor-report/src/db.rs` (15–45, 110–200) — `runs` PK `(run_id, scenario)`, eleven columns; `run_envelope` is the second-table precedent, and its doc comment states the reasoning verbatim.
- `crates/conductor-report/src/journal.rs`, `report.rs:147`, `crates/conductor-cli/src/render.rs:185` — the three render/serialize sinks of `latency_ms`.
- `scenarios/high-severity-log-capture.toml` (32–43) — the only catalog scenario with two `[[expected]]` blocks (`Contains "ERROR"` + `Absent "WARN"`); plain array-of-tables, no per-check keys today.

## Graph impact (from the code-graph query; trace `tree-query-2026-08-21-per-check-latency-measurement.json`)
- **`SloTier` / `deadline_ms`** — **150 refs across 18 files**, densest in `conductor-core/src/scenario.rs` (31), `conductor-verify/src/slo.rs` (27), `conductor-verify/tests/expected_slo.rs` (26), then `run_record.rs` (10), `db.rs` (7), `report.rs` (7). *Meaning:* widening the enum is a large, cross-crate change; adding a budget beside it is not. This is the quantitative backing for the "beneath, not wider" lean.
- **`evaluate_slo`** — exactly **one production caller** (`evaluate_check`, `slo.rs:74`); everything else is tests. Additive-parameter blast radius is one call site.
- **`evaluate_check`** — one production caller: `conductor-run/src/lib.rs:417`. Test pins in `slo.rs`, `record.rs`, `expected_slo.rs`, `lib.rs:848`.
- **`to_run_record`** — one production caller: `conductor-run/src/lib.rs:427`. Test pin at `record.rs:60`.
- **`crate_edges`** — `conductor-run → conductor-verify`, `conductor-run → conductor-report`, `conductor-cli → {core, report, verify}`, `conductor-tauri → conductor-core`. **No new edge is required** by any design considered.
- **Graph gaps — stated, not glossed.** `SELECT … FROM symbol WHERE symbol LIKE '%execute_scenario%'` returned **0 rows**, so `calls`/`refs` cannot resolve it either; the tightened descriptor `'%/persist().%'` likewise returned 0. My first `persist` query used the loose predicate `'%persist%'` and returned ~25 rows that are **false positives** — they match `persistence_seconds`, the Pulse cue field the harvest tests parse, not this crate's `persist`. **For these two symbols the caller list below is grep-derived and labelled as such**; the `SloTier` / `evaluate_*` / `to_run_record` / `crate_edges` results above are genuine graph results.

## Patterns detected
- **One read-back pass per scenario, projected per check** (`conductor-verify/src/extract.rs:1-8`, `:76-120`): `observe()` calls `query_incident_list` once, then loops the incidents calling `retrieve_report` + `retrieve_telemetry_slice`, composing a **single** `Observation { text, evidence_count, degraded, fingerprints }`. `Observation::observed_for(kind)` then hands each check a *projection* of that one observation — `text` for `Exact`/`Contains`/`Absent`, `evidence_count.to_string()` for `CountAtLeast`. The module doc says it outright: *"One pass over the corpus composes an `Observation` … and `observed_for` hands each check the slice its `ComparisonKind` needs."*
- **Worst-check collapse discards the rest** (`conductor-run/src/lib.rs:414-437`): `.max_by_key(severity_rank)` keeps one `CheckOutcome`; the other checks' verdicts, observed values and SLO outcomes never reach any sink.
- **Second-table-for-a-different-grain** (`conductor-report/src/db.rs:113-130`): `insert_envelope`'s doc states the rule verbatim — *"Deliberately NOT a column on `runs`: the qualifier is run-level, and the eleven-column check row plus its `(run_id, scenario)` key stay exactly as they are."*
- **One-altitude-up cross-field validation** (`scenario.rs:96`): `#[garde(length(min = 1), dive, custom(crate::phase_spec::fault_phases_are_silent))]` on `Scenario::phases` — a field-level `custom` at the container that owns both sides of the invariant.
- **Closed-enum fields carry `#[garde(skip)]` legitimately** (`expected.rs:57-61`, `scenario.rs:52`): `skip` on a closed *enum* is correct and is NOT the banned `skip`-on-a-nested-*struct*. A new `Option<u32>` budget takes `range`, not `skip`.

## Conventions to follow
- **Journal-relative instants from `std::time`**: `now_ms()` is `SystemTime::now().duration_since(UNIX_EPOCH)` (`conductor-run/src/lib.rs:520`), `now_rfc3339()` its RFC-3339 twin. Both already satisfy the wall-clock mandate — no clock change is needed or wanted.
- **Integer-milliseconds, `Eq`-preserving**: the envelope's `latency_ms: Option<i64>`; a budget declared as an integer keeps `Scenario`/`ExpectedCheck` `Eq` (the `error_percent: u32` precedent).
- **Bound parameters only** in `conductor-report/src/db.rs` (`rusqlite::params![…]` at every site).
- **Allowlist admission is explicit**: `redact.rs` already lists all eleven envelope names under a "reserved Run-report envelope fields" comment — a new field name joins that block deliberately.

## New files to create
- None required. (If the plan adds a per-check table, it lands in the existing `crates/conductor-report/src/db.rs` `SCHEMA` const beside `run_envelope`.)

## Files to modify
Seam files:
- `crates/conductor-core/src/expected.rs` — the optional per-check budget field + its `range` bound.
- `crates/conductor-core/src/scenario.rs` — the one-altitude-up `custom` validator asserting budget ≤ `slo_tier.deadline_ms()` on `Scenario::expected`.
- `crates/conductor-verify/src/slo.rs` — `evaluate_slo`/`evaluate_check` take the effective deadline (budget-or-tier) rather than reading `tier.deadline_ms()` directly.
- `crates/conductor-verify/src/record.rs` — per-check row projection beside `to_run_record`.
- `crates/conductor-run/src/lib.rs` — retain all `CheckOutcome`s instead of discarding the non-worst; thread them to `persist`.
- `crates/conductor-report/src/db.rs` — the per-check grain (additive `CREATE TABLE IF NOT EXISTS`, bound params).
- `crates/conductor-report/src/report.rs` — per-check lines in the Markdown report.
- `crates/conductor-core/src/redact.rs` — admit any new field name.

Caller threading (grep-derived for `execute_scenario`/`persist`; the graph has no symbol row for either):
- `crates/conductor-cli/src/commands/run.rs:24,26` — calls `execute_scenario` then `persist`.
- `crates/conductor-cli/src/commands/suite.rs:31,37` — same pair, in a loop.
- `crates/conductor-run/src/lib.rs:731,727,734` — `drive_run`'s own `execute_scenario` + two `persist` sites.
- `crates/conductor-tauri/src/commands.rs:260` — calls `drive_run` (graph-confirmed); touched only if `drive_run`'s signature moves.

Crate-local companions that PIN the changed surfaces (a caller query misses these — data pins and test calls):
- `crates/conductor-verify/tests/expected_slo.rs` — 26 `SloTier` refs; calls `evaluate_slo` at :41 and `evaluate_check` at :49/:60/:68/:69/:76.
- `crates/conductor-verify/src/slo.rs` inline tests (:116–:148) and `record.rs` inline tests (:51–:62).
- `crates/conductor-run/src/lib.rs` inline tests (:848–:849, :1083–:1101).
- `crates/conductor-core/src/run_record.rs:135` — the **exact-string** envelope golden; any envelope shape change breaks it by design.
- `crates/conductor-report/src/journal.rs:146` + `db.rs:293,368` — field-order and blocked-row-NULL pins.
- `scenarios/high-severity-log-capture.toml` — the only two-check scenario; the natural fixture for a differing-budget case.

## Scope premise closure
1. **Keep `SloTier` closed, add the budget beneath — VERIFIED.** arch §Data model conventions pins the closed TEXT enum; the graph measures 150 refs / 18 files behind it; security, design and layouts each independently forbid widening the tier's value set. `[inferred]` dropped.
2. **Budget ≤ tier `deadline_ms()` as a garde cross-field invariant — VERIFIED, with a placement correction.** `ExpectedCheck` cannot see the tier (it has no back-reference), and garde 0.22.1's `custom` is field-level only, so the rule must sit on `Scenario::expected` one altitude up, exactly like `fault_phases_are_silent` on `Scenario::phases`. `[inferred]` dropped; placement now specified.
3. **Thin catalog observability — VERIFIED by direct count.** 11 of 35 scenarios declare any `[[expected]]`; only `high-severity-log-capture` declares two. `[inferred]` dropped.
4. **Zero dependency delta — VERIFIED.** The PREREQ probe reproduced the signature byte-identically this session (`cargo audit` exit 1, first diagnostic `duplicate advisory ID: RUSTSEC-2026-0244`; `cargo deny check advisories bans licenses sources` exit 0; `Cargo.toml`/`Cargo.lock` byte-untouched). No design under consideration needs a crate. `[inferred]` dropped.
5. **`[premise-corrected: one read-back pass per scenario means one observation instant, so per-check latency cannot mean per-check read-back instants]`** — scope's §2 framing ("each check carries its own measured latency") implied each check could be timed independently. `observe()` is called **once** per scenario (`conductor-run/src/lib.rs:371`) and composes **one** `Observation`; every check grades a projection of it (`extract.rs:53-62`). Per-check *instants* would require per-check read-back calls — a read-back-architecture change far outside a `method: unit` capability, and one that would multiply MCP round-trips per scenario. The achievable and genuinely-different property is stated in Open questions #1.

## Open questions
1. **What makes per-check latency non-vacuous?** → blocks: **plan-decision.** With one shared instant pair, per-check `latency_ms` values are *equal* unless the **deadline** differs per check. So the budget must be declarable **per check** (on `ExpectedCheck`), not per scenario — then checks in one scenario share a latency but differ in whether they met their own budget, which is a real, unit-provable difference and matches the entry's two halves ("sub-5s budgets" + "each check carries its own `latency_ms`"). A scenario-level-only budget would make the per-check half vacuous. P4 must resolve this before synthesis.
2. **Which grain carries per-check outcomes?** → blocks: **plan-decision.** Row-per-check in `runs` breaks the `(run_id, scenario)` PK, arch's eleven-column pin, a11y's single-row-per-P-ID matrix and the lamp→journal-row join; a 12th column contradicts "column types fixed on first write"; a separate additive table keyed `(run_id, scenario, check_index)` is the exact `run_envelope` precedent (`db.rs:113-119`) at a check-level grain. The evidence points one way, so P4 states the lean with its citation rather than asking.
3. **Does the Markdown/CLI render change count as a breaking output contract?** → blocks: **implementation-scope.** layouts pins the cli results table at exactly 6 columns and calls a new column "a breaking change" absent a `--format` flag; design's indented-detail-line precedent surfaces a per-row fact without a new column. The file list above assumes the detail-line route; a decision to add a column would widen it.
