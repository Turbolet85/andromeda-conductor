# Report — 2026-09-18-real-model-leg-posture-and-grading-rule

**Chunk:** Real-model leg posture and grading rule — the launch posture, the grading rule and the per-leg quiet
window for a non-deterministic L4 live leg fixed and committed BEFORE any drive, operator-gated and never a CI
gate; the posture half of Epoch 4's pair, so `v3-09` is partially advanced and deliberately NOT claimed; folds
the `PREREQ` closing the rust gate deferral at re-pin ordinal 2.
**Date:** 2026-09-18
**Commits:** none since `last_wrap` — this wrap's own commit is the chunk's first.

## Changes (structured — detectors read this)

- **Files:** `contracts/pulse-real-model-leg-posture.md` (**new**, 181 lines). Pipeline artifacts beside it:
  `conductor-0.3.0/chunks/2026-09-18-real-model-leg-posture-and-grading-rule/{scope,research,plan,report}.md`,
  `conductor-0.3.0/verification-matrix.json` (a `notes` write on `v3-09` only),
  `conductor-0.3.0/working-route.md` (the freeze stamp), `.andromeda/master-route.md` (the pending record),
  `.andromeda/friction-log.ndjson`, and the phase/implement/wrap run dirs.
- **Symbols / APIs:** **none.** Zero `.rs` delta. The new file is a **reader-less** `contracts/` member — no
  Rust code reads it, it has no `default_path()`, no resolution through `resolve_under`, no bounds check and
  deliberately no `CONDUCTOR_*` override handle. No new env var, port, socket or export; the document NAMES
  existing registered handles (`ANDROMEDA_PULSE_L4_DETERMINISTIC`, `_MCP_ENABLED`, `_DATA_DIR`,
  `_BASELINE_BOOTSTRAP_SECONDS`) and mints none.
- **Crates / modules:** none added, removed or changed.
- **Dependencies:** none added or bumped. `Cargo.lock` and `package-lock.json` byte-unchanged; `cargo audit`
  scanned the same 562 crate dependencies as the prior reading.
- **Schema / config:** none. No envelope key, no span name, no span attribute, no violation-schema change.
- **Spec-master edits:** none applied at P1 (P2's fan-out may propose; see Expected amendments below).
- **Counts / qualifiers moved:** **none — verified.** No documented derived value changed: the workspace test
  count held at 986 and the coverage roll-up held at 6/11 verified · 5 unclaimed. (The `cargo audit` advisory
  figure is a host-tool reading and belongs to Dev-tool versions, not here — filing it under both bullets has
  previously drawn two detectors with opposing remedies.)
- **Dev-tool versions:** **none — `cargo audit` re-read at 1247 advisories · 562 crate dependencies · 7 allowed
  warnings (6 `unmaintained` + 1 `unsound`), UNCHANGED.** The baseline is the most recent recorded reading,
  `chunks/2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration/report.md:292`, which records the
  identical triple (1247 · 562 · 7) one day earlier. The advisory database did not move between the two
  readings. `cargo-audit` and `cargo-deny` themselves were not installed, upgraded or re-versioned this chunk.
  Basis: `grep -rnoE '1(2[0-9]{2}) advisories' conductor-*/chunks/*/report.md` over the whole corpus, bare.
- **Harness / gate surface:** **none — deliberately.** No `agent-run` verb, flag, arm or exit rule changed; the
  5-command surface is untouched and the operator-gated `--live` suite is byte-unchanged. The posture document
  *describes* a future leg's firing form without registering one, which is the chunk's stated boundary.
- **Cross-project / external claims:** several, all read at **`andromeda-pulse` HEAD `83d40601`** (tracked tree
  clean; only that repo's own bookkeeping modified):
  - `crates/mcp-server/src/tools.rs:360-387` — `dispatch_retrieve_report` parses `resolution_summary_text` as
    an `L4Output` and computes `degraded_mode = parsed_l4.is_none()`, returning `{markdown, degraded_mode}`.
  - `crates/interpretation/src/markdown.rs:153` — the literal `## Hypotheses` section header; `:41`
    `HypothesisView`, described at `pulse-app/src/incidents_router.rs:84` as a "Single **ranked** hypothesis".
  - `crates/triage/src/incident/registry.rs:128-160` — the two writers of `resolution_summary_text`:
    `attach_resolution_summary` (Resolved only) and `attach_interpretation_summary` (live incidents).
  - `pulse-app/src/inference_runtime.rs:823` — the live writer fires only inside the **dedupe re-generation**
    branch; `:250` — the resolution writer fires only on a resolution-summary digest.
  - `pulse-app/src/deterministic_inference.rs:68-72` — `evidence_refs` holds a populated `det-*` triple.
  - **CI run `35255156862`** on sha `e75fcb9dc19f9fa3ae97965bc4ec1cef452f43c5` (the tree this chunk started
    from), read at Setup 5a: all three jobs **completed / success** — Rust gate, Frontend gate, and the A11y
    gate (routine arm). No red to disposition. The sha is the record: this wrap's commit adds to that tree.
- **Reverted / negative API facts:** none written-then-removed. One surface was deliberately NOT written and is
  recorded as a plan-level rejection rather than a revert: a `--live`-sibling real-model harness arm, withheld
  because a live leg's `expect` atoms must be authored from a log that exists and this chunk drives nothing.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** **one, and it is a rule file rather than a spec master.**
  `.claude/rules/verification-harness.md`'s 2026-08-16 entry states that the deterministic-L4 fixture "pins
  [`evidence_refs`] to `[]` (`deterministic_inference.rs:35`)". Measured false at Pulse HEAD `83d40601`: the
  array sits at `pulse-app/src/deterministic_inference.rs:68-72` and holds a populated triple
  (`det-span-9f2c4a7e1b6d0358` / `det-template-0007` /
  `det-fingerprint-4a7f2b91c6e05d3849b1e7a2c5f08d63`); `:35` is now a doc-comment line about scrubber bounds.
  **It was TRUE when written and went stale the next day**: the triple landed 2026-08-17 in Pulse commit
  `efabe8e` (`feat(2026-08-17-conductor-e2e-verification-closure)`), whose diff changes
  `"evidence_refs": []` to the three values above. **No spec master carries the stale claim** — the only
  master citing that file is `architecture.md:64`, which already states the corrected reading ("freshness stays
  the carrier because the values never vary with what Conductor emitted, *not because the field is empty*").
  Basis: `grep -rnoE '.{130}deterministic_inference\.rs.{60}'` across all seven masters — one hit, current.
  **Owner: P3 curation** (a Tier-2 rule-file correction), not a P2 amendment.
  Two SUT-side descriptions are likewise stale — `crates/mcp-server/src/jsonrpc.rs:189` and
  `crates/interpretation/src/markdown.rs:20-27` both say only resolved incidents render hypotheses, which
  `attach_interpretation_summary` exists to defeat — but those are the SUT's own artifacts and are recorded
  here as a planning hazard, not as anything this repo amends.
- **Expected amendments (from plan):** one entry.
  - `architecture.md` §Occupied Resources — On-disk artifacts — register
    `contracts/pulse-real-model-leg-posture.md` as a second reader-less `contracts/` member. **Carried**: its
    fact is the Files and Symbols/APIs bullets above (a new on-disk artifact under `contracts/`, reader-less
    regime). Site search: `grep -c 'pulse-p025-measurement-contract' .andromeda/architecture.md` → **1 hit**,
    at `architecture.md:181` — the precedent's registry row in §Occupied Resources — so the owning master and
    the insertion point are both named on evidence rather than by assumption. Note for the applier: the P-025 precedent was recorded n=1 with no
    playbook rule minted, so a second reader-less member is a judgement to surface, not a settled pattern.
- **Coverage of new surfaces:**
  - `contracts/pulse-real-model-leg-posture.md` (a committed document, not an external surface) → validation
    `n/a` (nothing parses it) · instrumentation `n/a` (no runtime path) · PII `redacted✓` (asserted by gate 7:
    zero host-path matches, count 0) · tests `unit/integ` (the workspace suite ran green over the tree
    carrying it; two probes assert its presence, structure and host-path freedom) · a11y `n/a` (no UI) ·
    tokens `n/a` (no UI).

## Deviations from intent

**None.** All seven implementation steps executed as written; both touchpoint lists were exactly right; no step
was skipped, reordered or reinterpreted, and no plan-listed rejection was crossed.

## Decisions & corrections

- **Operator correction (advisory baseline), applied and then extended by one step.** The implement report
  compared today's 1247 against **1243** (2026-09-10) and framed the delta as "wrong by four". The directive
  corrected the baseline to **1246** (`chunks/2026-09-16-scenario-assertion-audit-gate/report.md:229`, which
  itself records the move 1243 → 1246). Re-deriving the corpus bare rather than accepting either enumeration
  found a **newer** reading still: `chunks/2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration/report.md:292`
  records 1247 · 562 · 7 — the same triple as today, one day earlier. **The true delta is ZERO.** Both the
  original and the corrected figure were artifacts of reaching past a newer recorded reading; the directive's
  own stated principle applied one step further than the directive itself reached.
- **Operator correction (the 986 attribution), applied.** The implement report attributed the recorded count to
  "the prior chunk". The immediately prior chunk (`2026-09-17-keyboard-and-focus-order-coverage-ownership`,
  = HEAD) records nextest green at exit 0 **without** the count (`report.md:74`). The count is carried by four
  reports — `grep -rln '986 tests' conductor-0.3.0/chunks/*/report.md`, bare — of which the directive's cited
  `2026-09-16-a11y-ci-gate-at-an-honest-terminal/report.md:180` is one and the most recent is
  `2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration/report.md:288`. The comparison
  986 → 986 across a zero-`.rs`-delta span stands; only the attribution was wrong.
- **Sweep hazard found this chunk (self-inflicted, and the corpus already warned of it).** While checking the
  986 attribution, a sweep run as `grep -rn '986' … | grep -iE 'test|nextest|pass' | tail -8` returned four
  carriers and appeared to show the directive's cited file absent — because `tail -8` had clipped it, the
  alphabetically-earliest hit. Reading the cited line directly is what corrected it. The pattern to distrust is
  any clipped multi-path sweep used to assert an ABSENCE: the honest form is `grep -rln` bare, and the count is
  read from the bare command.
- **Class finding — a cross-repository citation has no re-check mechanism anywhere in either pipeline.** The
  stale entry above cites another repository's `file:line`. Nothing in Conductor's pipeline re-verifies such a
  citation when the other repository moves, and nothing in Pulse's pipeline knows Conductor cites it. This one
  was true when written, went stale the next day in Pulse commit `efabe8e`, and survived a month — surfacing
  only because this chunk happened to read the cited file for an unrelated reason. Note the asymmetry that
  makes it worse than an internal staleness: the wrap cascade sweeps the seven masters and re-derives their
  leaves, so `architecture.md` was corrected; a **rule file** carrying the same kind of citation is swept by
  nothing. Worth stating as a class, not only fixing the instance.
- **Correction to a finding this chunk carried: the code-graph gap was a STALE INDEX, not a missing symbol.**
  P3 recorded `execute_scenario` as absent from the rust plane's `symbol` view (a `symbol`-by-name probe
  returned empty while `grep` found 15 hits), and implement's report carried that forward. After this wrap's
  full `code-graph.py refresh` (rust 2620 nodes / 12514 edges), the same probe returns the symbol at
  `crates/conductor-run/src/execute.rs` and the impact query returns **9 call sites / 7 distinct callers**.
  The distinction that matters for the next planner: P3's query ran against a database the query path had
  regenerated on miss and which reported `db_state: fresh`, yet still lacked the symbol; the wrap's full
  refresh has it. So a query-time regenerate is not equivalent to a full refresh, and a 0-row result on a
  `fresh`-stamped plane is not by itself evidence of a leaf. CLAUDE.md's 2026-09-02 figure of 7 sites /
  5 callers matches neither reading and remains stale — the caution against re-quoting it stands, for this
  corrected reason.
- **A criteria tension resolved at the step that owns it.** The security criterion forbids "internal seam-crate
  struct names" in the committed artifact while the arch/obs/design criteria require naming the closed
  `Verdict`/`ReportState` sets. The redaction boundary's own scope settles it: it governs Conductor-GENERATED
  artifacts (run report, `runs.db`, journal, self-obs), and a committed `contracts/` document is not one — the
  P-025 precedent itself names SUT type and field names. The document therefore uses public contract vocabulary
  and cites `file:line` where a crate-private mechanism needed reference.

## Outcome

Acceptance criteria, each re-asserted against the diff rather than against the plan's text:

- (arch) Reader-less regime declared explicitly, with `sut_version` / `captured_at` / `pinned_at` /
  `provenance` — **MET** (`## Regime` + the metadata block; provenance is per-clause MIXED, naming which
  clauses are transcribed SUT records and which is a carried Conductor measurement).
- (arch) Every outcome from the closed `Verdict`(3)/`ReportState`(5) sets as a returned value, miss mapping
  reconciled explicitly against the `CalibrationRegion → ManualCheck` default — **MET** (the outcome table plus
  the "why a miss is the calibration region" paragraph, which states the consequence that the leg cannot go red
  on the model being wrong).
- (security) The L4 handle stated as a declaration-only observation of Conductor's own environment; no clause
  asserts a measurement of `pulse-app` — **MET**.
- (security) Zero absolute host paths / data-dir values / secret-shaped strings — **MET**, gate-proven (count 0).
- (security) `cargo audit` + `cargo deny` green over an un-drifted `Cargo.lock`, porcelain probe clean first —
  **MET**.
- (tests) Operator-invoked, never-a-CI-gate footing stated, invocation named as a member of the sanctioned
  live-leg set, adding no member — **MET**.
- (tests) `cargo clippy --workspace --all-targets -- -D warnings` runs green, not deferred — **MET**.
- (tests) No re-drive-until-pass; no reported state produces a non-zero exit — **MET**.
- (obs) A non-tier disposition named, with the 90 000 ms ceiling cited against the ~110 s figure — **MET**.
- (obs) No new span name, span attribute, envelope key or grade word — **MET** (asserted against the diff: the
  document adds no code and the Schema/config bullet is empty).
- (design) An unmeasured leg grades `Blocked` with its named precondition, lexically distinct from a graded
  miss — **MET** (the outcome table separates the rows).
- (a11y) The `driven-a11y` leg keeps its owner, posture and results unchanged — **MET** (harness byte-unchanged).
- (layouts) No sixth command, no new bracket label, ANSI entry or token — **MET**.

**Gates** (`[[gate]]` entries by `run`, in order; /implement's run, re-run by this wrap's light gate):

- `git -C "$CARGO_HOME/advisory-db" status --porcelain` — **green** · exit 0 · no output (the local
  advisory-db copy is clean, so the audit below is trusted rather than assumed).
- `cargo audit` — **green** · exit 0 · 1247 advisories · 562 crate dependencies · 7 allowed warnings.
- `cargo deny check advisories bans licenses sources` — **green** · exit 0 · advisories/bans/licenses/sources ok.
- `cargo clippy --workspace --all-targets -- -D warnings` — **green** · exit 0. **The PREREQ discharge.** The
  log is one line (`Finished dev profile … in 0.36s`), i.e. fully cached: the green re-asserts over unchanged
  sources rather than being a fresh analysis. Stated plainly because the deferral chain closes on it.
- `cargo nextest run --workspace --profile ci` — **green** · exit 0 · 986 tests run, 986 passed, 0 skipped.
  Ran un-deferred: `conductor-0.3.0/verification-matrix.json` carries an uncommitted delta and three Rust
  sources read it, two of them test binaries (`conductor-core/tests/scenario_audit_gate.rs`,
  `conductor-report/tests/matrix_ledger_gate.rs`; basis `grep -rln 'verification-matrix.json' --include=*.rs crates/`).
- `test -f contracts/pulse-real-model-leg-posture.md && grep -cE "^## " …` — **green** · exit 0 · 7 sections.
  `new = true`; P5 baseline `red — exit 1` on the absent subject, with a both-directions control.
- `grep -cE "[A-Za-z]:[\\/]|/home/|/Users/|%APPDATA%" …` — **green** · exit 1 · last line 0 (both atoms held).
  `new = true`; P5 baseline `red — exit 2`, with a control proving a planted drive-letter path FAILS the entry
  and a clean file carrying a loopback address does not match.

**Smoke:** skipped — no boot-path or UI-surface change (the only touchpoint is a `contracts/` document; the
plan lists neither a `smoke` nor a `self-verify` entry, and states the absence in its prose).

**Outcome basis:** /implement's P4 report as given in this session's conversation, plus this wrap's operator
directive, which corrected two figures before the report was authored (the advisory baseline and the 986
attribution — both re-derived here, one of them corrected a step beyond the directive). The friction log was
NOT read as a basis: the implement conversation is present.

**Process hygiene:** re-measured here against the host process list (`tasklist` per image): `cargo`, `rustc`,
`git` — all started by this chunk's gate runs, all **terminated**, 0 instances. Zero instances also for
`conductor`, `conductor-tauri`, `andromeda-pulse-mcp`, `msedgedriver`, `tauri-driver` and `pulse-app`. No leg
ran and no external process was booted, so nothing was left for the operator to stop.
