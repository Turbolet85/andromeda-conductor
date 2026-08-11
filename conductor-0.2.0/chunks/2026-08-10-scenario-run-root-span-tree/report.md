# Report — 2026-08-10-scenario-run-root-span-tree

**Chunk:** scenario.run root span tree — the absent root parenting the must-trace self-observation chain, plus the two orphaned conductor-report spans (report.generate → db.insert_run)
**Date:** 2026-08-10
**Commits:** none yet this chunk (the wrap commit is the chunk's first); prior HEAD `09849ae feat(2026-08-10-pulse-run-contract)`

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-core/src/obs.rs` (+132) · `crates/conductor-report/src/report.rs` (+72) · `crates/conductor-cli/tests/cli_smoke.rs` (+46) · `crates/conductor-run/src/lib.rs` (+17) · `crates/conductor-core/src/redact.rs` (+10) · `crates/conductor-report/src/db.rs` (+6) · `crates/conductor-report/Cargo.toml` (+1) · `Cargo.lock` (+1). No new files, no deletions.

- **Symbols / APIs:**
  - **No public API change.** `conductor_run::execute_scenario`, `conductor_report::RunReport::write` and `conductor_report::RunsDb::insert` keep their exact signatures — each only gained a `#[tracing::instrument]` attribute.
  - New private impls on the existing `JsonObsLayer` (conductor-core, not exported): `on_new_span`, `on_close` (both `Layer` trait methods), plus helpers `base_map` / `write_line`. The `Layer<S>` bound tightened from `S: Subscriber` to `S: Subscriber + for<'lookup> LookupSpan<'lookup>` — satisfied by the already-installed `Registry::default()`.
  - New private helpers in `conductor-report::report`: `join_distinct`, `verdict_field`, `state_field`.
  - **Span names now emitted:** `scenario.run` (conductor-run), `report.generate` + `db.insert_run` (conductor-report). All three were already members of obs-plan §11's bounded span-name set — no name invented, no name renamed.
  - **New self-obs line keys:** `span`, `span_event` (`new`/`close`), `parent` — layer-inserted on span-lifecycle records only.
  - No new ports, no new env vars, no new IPC/Tauri command, no new CLI verb or flag.

- **Crates / modules:** none added or removed. `conductor-report` changed (gained one external dependency edge); `conductor-core` and `conductor-run` changed in place.

- **Dependencies:** `tracing` added to `crates/conductor-report/Cargo.toml` `[dependencies]` as `tracing.workspace = true` — the workspace-pinned 0.1.44 already declared in `[workspace.dependencies]`. **`Cargo.lock` moved by exactly 1 line, inside the existing `conductor-report` `[[package]]` entry; NO new `[[package]]` was added** (verified: `git diff Cargo.lock | grep '^+name = '` is empty). No bumps. No OTel/`opentelemetry-*`/`tracing-opentelemetry` crate entered any crate.

- **Schema / config:** the **self-obs JSONL line** (format owned by test-plan §3; obs-plan §3 derives) gains a span-lifecycle variant: the same flat identity block (`timestamp_ms`, `level`, `target`, the three service fields, `run_id`) plus `span` / `span_event` / optional `parent` and the span's own allowlisted attributes. The pre-existing event line is byte-unchanged in shape. **The Run-report envelope is UNCHANGED** (same eleven fields, same order) and **`runs.db` columns are UNCHANGED** — the two record shapes obs-plan §3 names are still two; this is a variant of the self-obs base line, not a third shape. No migration, no config key.

- **Spec-master edits:** none — this chunk edited no `.andromeda/` master. **Two amendments are EXPECTED at P2** (recorded in `plan.md` §Implementation notes):
  1. `obs-plan.md` §4 Critical Path 1 (Cleanup) — "Root `scenario.run` span closes on `db.insert_run` completion" is unachievable under the shipped call structure; the root is per-scenario at the composition root and the two report-seam spans are run-scoped siblings correlated by `run_id`.
  2. `obs-plan.md` §1 (line 49), §4 (line 136), line 373 — names four Tauri command spans that do not exist (`start_scenario` / `stop_scenario` / `get_run_report` / `operator_pause_go_no_go`) vs the shipped `tauri.command.start_run` / `stop_run` / `run_report` / `resolve_operator_hold`. **Pre-existing drift, not introduced by this chunk**, in the same open section.

- **Counts / qualifiers moved:** **none.** `conductor_core::UNBACKED_AUTO` is unchanged at 9 entries (this chunk names no P-ID and adds no scenario); no coverage tally, roll-up caption, lamp count, precondition count, or wireframe sample changed. The `ALLOWLISTED_FIELDS` const grew 21 → 29, but no document states that number — it is not a documented derived value.

- **Dev-tool versions:** none installed or upgraded. `cargo-audit` 0.22.2 was already the version proven against at the prior chunk; it was re-run, not changed.

- **Reverted / negative API facts:** recording `db.insert_run`'s `row_count` from rusqlite's actual affected-row count (via `tracing::field::Empty` + a later `Span::current().record(...)`) was implemented-in-thought and **deliberately not shipped**: the layer records span attributes on the `new` record only, so a post-open `record()` fires `on_record`, which the layer does not implement — the value would vanish silently. Shipped the literal `row_count = 1`, which is exactly what obs-plan §4 CP1 specifies. Implementing `on_record` was judged out of plan scope.

- **Coverage of new surfaces:**
  - `scenario.run` span (conductor-run) → validation n/a · instrumentation ✓ · PII redacted✓ (allowlist-gated + host-path scrub at the processor stage) · tests unit+e2e · a11y n/a · tokens n/a
  - `report.generate` span (conductor-report) → validation n/a · instrumentation ✓ · PII redacted✓ · tests unit+e2e · a11y n/a · tokens n/a
  - `db.insert_run` span (conductor-report) → validation n/a · instrumentation ✓ · PII redacted✓ · tests unit+e2e · a11y n/a · tokens n/a
  - span-lifecycle log record (conductor-core `JsonObsLayer`) → validation n/a · instrumentation ✓ · PII redacted✓ · tests unit+e2e · a11y n/a · tokens n/a

## Deviations from intent

1. **Allowlist grew by 8 names, not the plan's enumerated 5.** Added `row_count`, `phase_count`, `emission_count`, `record_count`, `mcp_tool` (the plan's five) **plus** `span`, `span_event`, `parent`. *Justification:* the latter three are the span-record keys the layer inserts directly, and `redact.rs` already lists its direct-inserted identity keys in the allowlist; omitting them would falsify that file's own doc claim ("only allowlisted field names appear on a log line"). Same file, same plan step.

2. **`Cargo.lock` moved by 1 line.** The plan predicted byte-unchanged but instructed "verify rather than assume". *Justification:* the dependency edge is recorded inside the existing `conductor-report` package entry. No new package, so the audit surface is unchanged.

3. **`crates/conductor-cli/tests/cli_smoke.rs` edited** — a test file not in the plan's touchpoint table. *Justification:* named explicitly by the plan's Step 7 ("extend the shipped `cli_smoke.rs::agent_mode_routes_self_obs_to_the_log_file_not_stderr` scaffold rather than adding a harness"). In scope.

4. **`report.rs` gained three private helpers** (`join_distinct`, `verdict_field`, `state_field`) the plan did not enumerate. *Justification:* the plan required the span to carry `verdict`/`state` but not how to stringify them, and the obvious accessor is a trap — `Verdict::label()` / `ReportState::label()` are documented **human** labels rendering `CalibrationRegion` as `"HOLD"` and `ManualCheck` as `"Manual"`, which would have silently broken the a11y lamp↔journal-row join the acceptance names. Used the canonical serde spelling, locked by a dedicated test. For a suite, distinct-first-seen values are joined rather than inventing a severity ranking the report seam does not own.

5. **`row_count` shipped as the literal `1`, not the actual affected-row count.** *Justification:* forced by the chosen layer design (attributes recorded at span-open); the literal is what obs-plan §4 CP1 specifies, so no acceptance weakens. See *Reverted / negative API facts*.

## Decisions & corrections

- **Operator correction at phase P5 — the matrix premise-correction channel.** The plan proposed *refining* `v2-31`'s acceptance text at claim time, because the code-graph disproved its "nested beneath it" premise. The operator held the **`v2-18` note-only precedent** from `2026-08-10-pulse-run-contract` instead: set `chunk`, write the PREMISE-CORRECTION into `notes` citing the graph rows, and leave the acceptance prose standing as the record of what was asked. Rationale: `verification-matrix-contract.md` §Affordance honesty is the **only** sanctioned acceptance-refinement class today; "premise disproved" is a **proposed second class awaiting the founder's P6 ruling**, and until that ruling both entries must share one channel or the ledger becomes inconsistent. When P6 is ruled, `v2-18` and `v2-31` take the same retroactive treatment under one rule. *(Applied: `v2-31.acceptance` and `.requirement` are byte-unchanged; the diff touches only `chunk`, `notes`, `ref`, `status`.)*

- **Operator decision at phase P4 — layer scope.** The chunk owns the subscriber-layer span capability (emit `on_new_span` + `on_close` lifecycle records), not just the three annotations. Chosen over folding span context onto events, and over shipping annotations that provably emit nothing.

- **Operator decision at phase P4 — root placement.** `scenario.run` is per-scenario at the composition root (`execute_scenario`), with `report.generate` / `db.insert_run` as run-scoped siblings correlated by `run_id`. Chosen over wrapping both calls at the CLI edge (loses instrument-once-inherit-twice) and over a new run-level span name (would need a bounded-set amendment).

- **Operator wrap directive — the `cargo audit` re-pin rationale changes wording.** This chunk **did** move the dependency tree, so the deferral's standing basis is now **"audit surface unchanged (no new packages) + `cargo deny` verified green"**, *not* "zero dependency delta". The ninth re-pin must carry that wording so the rationale stays literally true.

- **Operator wrap directive — the `on_record` gap is a route-resolve judgment.** Decide from the *Faithful emission dispatcher* entry's actual scope whether it earns a CARRY there; do not default to a pin. The friction record suffices otherwise.

- **Scope amended in place at phase P5 (intent-incomplete).** `scope.md` asserted the three shipped spans "are emitted"; research falsified it (they emit nothing). Amendment recorded in the scope with the empirical evidence.

## Outcome

**All acceptance criteria met.** Gates green in **one fix-loop iteration, no fixes needed**:

- `cargo nextest run -p conductor-core -p conductor-run -p conductor-report` — **290/290**
- `cargo nextest run --workspace --profile ci` — **518/518**, zero retries (+9 from this chunk)
- `cargo test --workspace --doc` — ok
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `ANDROMEDA_PULSE_DATA_DIR='pulse;injection' cargo run -p conductor-cli -- run P-001 --agent-mode` — exit 0, artifact produced and inspected
- `cargo deny check` — advisories · bans · licenses · sources **all ok**
- `cargo audit` — **exit 1**, byte-identical `duplicate advisory ID: RUSTSEC-2026-0244` on 0.22.2 (the latest published). Advisory-**database** fault; bounded wait continues under the operator's L5 ratification. Ninth consecutive.

**Smoke ✓** (boot-path changed — the run pipeline and the obs sink): `bash scripts/agent-run.sh run` exit 0, and `bash scripts/agent-run.sh status <run_id>` returned a truthful envelope with the blocked-row null rule intact (`verdict: null`, `latency_ms: null`, `state: "Blocked"`).

**The empirical proof.** Before this chunk the committed `logs/agent-latest.jsonl` contained **0** span records of any kind. After, a real agent-mode run emits:

```
verify.readback.connect  new / close
scenario.run             new  {seed: 4317001, scenario: "receiver-lifecycle-state", p_ids: "P-001"}
scenario.run             close
db.insert_run            new  {row_count: 1}
report.generate          new  {state: "Blocked", verdict: "null"}
```

`verify.readback.connect` is a **previously-shipped span that had never emitted anything** — it became visible for free, confirming the diagnosis that the gap was missing span emission, not missing correlation. On the no-Pulse spine the gate blocks before the timeline, so `timeline.execute` / `emit.batch` never run; their nesting beneath the root is proven at the layer's own tier (`a_child_span_records_its_parent_so_the_tree_reconstructs`), and the live chain remains the operator-gated leg.

**Verification matrix:** `v2-31` → `ref` (9 test ids) + `status: implemented`, awaiting the P7 flip to `verified`.
