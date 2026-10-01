# Report — 2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix

**Chunk:** Interpretation re-proven after Pulse's incident-surfacing fix — a third pre-registered real-model series for
v3-09 on a fresh letters-only data dir, against Pulse a2addb3
**Date:** 2026-10-01
**Commits:** `3791d37` chore(2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix): operator pre-CI commit,
for the run this chunk's verdict reads (the only commit since `last_wrap` 2026-10-01T00:13:10Z after `1fe46a1`)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-run/tests/real_model_live.rs` · `crates/conductor-run/tests/real_model_common/mod.rs` ·
  `crates/conductor-run/tests/real_model_harvest.rs` · `crates/conductor-run/tests/real_model_series/mod.rs` ·
  `contracts/pulse-real-model-leg-posture.md` (add-only, new `## The 2026-10-01 series`) · new
  `chunks/{marker}/evidence/{attempt-ledger.md, rm-capture-d1.txt, rm-capture-d2.txt, rm-capture-d3.txt}` ·
  `chunks/{marker}/scope-record.md` · two notes `.andromeda/runs/2026-10-01T18-42-55-phase/p5-dryrun{,-2}.MOVED.md`
  (basis: `git diff --name-only 1fe46a1` + `git status --short`).
- **Symbols / APIs:** all TEST-binary code, no shipped surface.
  - `real_model_live.rs` `print_pulse_witnesses`: its fixed Pulse-log target list gains
    `interpretation.incident.skipped`, printed fields-only (`skip_reason`, `decision`, `severity`, `digest_kind`)
    through the unchanged `emit` scrub chain (`mask_workspace_key` → `redact_value` → `mask_host_paths` →
    `elide_fingerprints`). The capture now prints `pulse-log interpretation.incident.skipped: {n}` plus one line per hit.
  - `real_model_common/mod.rs`: new private const `INCIDENT_SKIPPED = "interpretation.incident.skipped"` (distinct from
    the existing `SKIPPED` = `interpretation.inference.skipped`, a pipeline skip); `canary_attempts` (signature
    unchanged; its 7 call sites are all in `real_model_harvest.rs`) appends ` skip_reason={first such line's
    skip_reason | none}` as the LAST field of every `canary:` line. The token decision is unchanged.
  - `real_model_harvest.rs` (nothing between the rule markers changed): 3 pairing/source arms
    (`a_dismissal_carries_pulse_s_stated_skip_reason`, `a_surfacing_and_the_pre_fix_dismissal_carry_no_skip_reason`,
    `the_capture_prints_pulse_s_no_incident_outcome`); the 2026-10-01 series block (`capture_2026_10_01`,
    `SERIES_2026_10_01_RULE_SHA256`, `KEY_2026_10_01 = "rm-surfacing-series"`, `measured_2026_10_01`, and the tests
    `the_2026_10_01_series_rule_was_fixed_before_d1`, `each_2026_10_01_capture_matches_its_pinned_digest`,
    `each_2026_10_01_drive_recorded_the_current_rule_before_it_fired`,
    `the_2026_10_01_captures_carry_no_fingerprint_and_no_workspace_key`,
    `each_2026_10_01_drive_grades_as_the_ledger_records`, `the_2026_10_01_envelopes_carry_the_eleven_keys`,
    `v3_09_is_not_met_by_the_2026_10_01_series`); `no_committed_capture_text_sits_in_test_source` loops the new series.
    The target holds 100 tests (was 93; basis: nextest `Summary 100 tests run`).
  - `real_model_series/mod.rs`: `EVIDENCE_2026_10_01` + `SERIES_2026_10_01` (three `Drive` pins: labels, file
    names, sha256 of LF-normalized content).
  - No new env handle, verb, flag, selector, lamp, caption, port or envelope key.
- **Crates / modules:** none added or removed.
- **Dependencies:** none (`grep -c "^name = " Cargo.lock` = 562 at `1fe46a1` and now; `Cargo.lock` untouched).
- **Schema / config:** the committed real-model capture's grammar gains two TRAILING/ADDED forms (the
  `interpretation.incident.skipped` witness lines and the `canary:` line's last ` skip_reason=` field); the grading
  rule reads only the first token after `canary: ` and is byte-identical to `1fe46a1` (rule-diff probe: no output).
  Scrub shape unchanged: no new elision code. One STATED RESIDUAL committed on the overseer's ruling (option (a),
  2026-10-01): `rm-capture-d3.txt` carries one all-digit 8-character `fingerprint_hex` value on two lines (the
  suggested and autonomous detection of the preflight CANARY's second storm, `scenario_storm=false`) — synthetic
  content Conductor emitted, no real data; `elide_fingerprints` keeps all-digit runs by design; the harvest counts it
  exactly (2 in d3, 0 in d1/d2).
- **Spec-master edits:** none (implement touched no master; `contracts/` is not a master).
- **Counts / qualifiers moved:** the real-model drive series count: two series recorded (2026-09-29, 2026-09-30) →
  three (+ 2026-10-01) — stated at `architecture.md:182` ("the drive series (2026-09-29, then 2026-09-30 — each an
  add-only section …)", grep `2026-09-30 series` 1 hit) and `test-plan.md:336` (1 hit). The harvest test count 93 →
  100 (no master states it: `grep -c "93 tests\|93 passed" .andromeda/*.md` 0). Workspace nextest 1140 → 1147 (no
  master states either figure — verified: grep `1140` / `1147` over the seven masters 0 hits each).
- **Dev-tool versions:** none — no host tool installed or upgraded (cargo-audit / cargo-deny re-read at their
  installed versions; the advisory-db copy current, HEAD = FETCH_HEAD `46826f29`).
- **Harness / gate surface:** none in `scripts/`, `.github/` or any `agent-run` verb (numstat probe against
  `1fe46a1` printed nothing). The `run --live real-model` capture prints the added lines above (test-binary output
  only).
- **Cross-project / external claims:**
  - andromeda-pulse `a2addb3` (`a2addb3755b3029cb79809b96efdd2522749b179`, on `origin/chore/migrate-pulse-to-v3`, 0
    unpushed, build inputs clean — read with `git -C` against the local tracking ref, no fetch): read with `git show`
    at committed state — `interpretation.incident.skipped` (`pulse-app/src/inference_runtime.rs:96`, message
    `incident producer skipped`, fields allowlisted at `pulse-app/src/observability.rs:2225-2231`; `skip_reason` ∈
    {`model_resolution_summary`, `decision_dismiss`, `severity_none`, `no_cue`}); `PROMPT_VERSION_PRIMARY = "v2.3"`
    (`crates/interpretation/src/schema.rs:31`); OVERALL line `OVERALL: {overall} ({} active incident(s); {} cue(s))`
    with `anomalous` for a cue-bearing tier-1 digest (`crates/triage/src/digest/assembler.rs:652-662`); the
    `L4Output` struct, the `## Hypotheses` render and `crates/workspace-detector` unchanged from `fcc31b2`.
  - Pulse binaries built from `a2addb3` in the operator's build slot: `pulse-app.exe` sha256 `6476568e…` (content
    proof: `incident producer skipped` 1, ` active-bypass incident(s); ` 0); `andromeda-pulse-mcp.exe` sha256
    `29f35540…` (content 0/0/0 → PROVENANCE arm: clean inputs, exit 0, mtime after the commit, sha256 ≠ `2179caab…`).
  - The live series, measured from Pulse's own log (`agent-latest.jsonl.2026-10-01`, posture confirmed before d1:
    `inference_mode real`, `workspace_root_basename rm-surfacing-series`, 0 override lines): see Outcome.
  - CI: `3791d37` → CI#36921742915 `verdict: green · checks 3/3 · wall 572 s` (overseer-verified); this wrap's commit
    adds to that tree.
  - **The d2 span-identity gap (measured):** Conductor's exception span identity is a pure function of the scenario
    seed (`crates/conductor-emit/src/exception.rs:177-187`, `exception_trace_request`; seed 4317033). d1 attributed on
    its first poll, so d2's scenario emission (19:46:20.822Z) came ~7 min after d1's while Pulse's buffer still held
    d1's rows; every append of d2's scenario spans was refused (`duckdb.append` `reject_reason=append_failed` ×36;
    `buffer.tick` `append_rejections` 36), no scenario storm formed. Pulse evicted d1's rows 19:48-19:51Z
    (`buffer.tick` `eviction_count_since_last_tick`, `memory_bytes` 0 from 19:51:19Z); d3 (20:01Z) landed. Routed
    forward on the overseer's word (relay §2 entry 1); no change in this chunk.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - `.claude/rules/verification-harness.md:67` (2026-09-23): "Pulse logs NO line when its model DISMISSES a digest" —
    true to Pulse `fcc31b2`, superseded at `a2addb3` by `interpretation.incident.skipped` (measured live: 15 skip lines
    across the three leg windows). A rules file, not a master: a curation (time-axis correction) item.
  - The pre-registered drive spacing (posture contract §The drive series (c), carried into §The 2026-10-01 series: ≥
    150 s after the last incident, else ≥ 90 s) presumes a drive's emission cannot be voided by the PREVIOUS drive's
    emission; measured false when the previous drive attributes fast (d2 above). The contract section is
    pre-registered and never edited after d1; the remedy is the routed entry. No master states the spacing as
    sufficient (grep `quiet window` in test-plan/architecture: the hits describe Pulse's incident dedupe, not span
    identity) — disposition: routed, no amendment.
- **Expected amendments (from plan):**
  - architecture §Occupied Resources, the posture-contract row (`architecture.md:182`, grep `2026-09-30 series` 1 hit,
    `fcc31b2` 1 hit): carried — Counts bullet (three series) + Cross-project bullet (Pulse `a2addb3`); trim to the
    registry-size target.
  - test-plan §6 Real-model interpretation leg (`test-plan.md:336`, grep `2026-09-30 series` 1 hit, `fcc31b2` 1 hit):
    carried — the 2026-10-01 series and its measured outcome (Outcome below) beside the dated list.
  - security-plan §Input Validation, the real-model capture ingest row (`security-plan.md:121`, grep `fcc31b2` 1 hit):
    carried — Schema / config bullet (the `interpretation.incident.skipped` lines fields-only, the `canary:` line's
    `skip_reason`, the stated residual) + Cross-project (`workspace_key` coordinate `fcc31b2` → `a2addb3`, unchanged in
    content). ALSO carried — `security-plan.md:335` (the corpus-text exception, grep `2026-09-30 series` hit 2 of 2):
    it enumerates which series' captures carry corpus-rendered report text ("the 2026-09-29 series' b2 capture
    carries one report body; the 2026-09-30 series read none"). This chunk moved it: the 2026-10-01 series' d1 and
    d3 captures each carry ONE report body (attributed incidents 2 and 10, printed from the first `## ` line through
    the four-stage chain, digest-pinned, no capture text in test source — `no_committed_capture_text_sits_in_test_source`
    loops the series); d2 read none. The same site is the class the 2026-10-01 stated residual belongs to (the frozen
    2026-09-22 file's one `fingerprint_hex` prefix is named there).
  - obs-plan §4 Real-model posture: not carried — every drive's envelope is `ManualCheck` / `verdict` null, the shape
    the bullet already names; no new span/line (obs-plan grep `incident.skipped` 0, `2026-09-30 series` 0).
- **Coverage of new surfaces:**
  - `capture: interpretation.incident.skipped witness lines` → validation n/a (fields-only from a closed allowlisted
    set) · instrumentation n/a (test output) · PII redacted✓ (the four-stage scrub chain; d3's all-digit fingerprint
    prefix is the stated residual) · tests unit (harvest source arm + pinned-capture grades) · a11y n/a · tokens n/a
  - `capture: canary line skip_reason field` → validation n/a · instrumentation n/a · PII n/a (closed enum) · tests
    unit (two pairing arms) · a11y n/a · tokens n/a

## Deviations from intent
- **d2 not graded** — its scenario emission was refused by Pulse (span-identity gap above). Under the pre-registered
  contract it is a fired, counted drive (its canary surfaced, so it is not the pipeline-fault the re-fire rule
  names); nothing re-fired, no fourth drive.
- **d3's all-digit fingerprint prefix committed as a stated residual** — overseer option (a), 2026-10-01; reason, in
  their words: "the token is an 8-digit prefix of a SYNTHETIC scenario fingerprint (Conductor-emitted storm, no real
  data); no new elision code in this chunk". Precision recorded in the ledger: the storm is the preflight canary's,
  also Conductor-emitted.
- **The operator pre-CI commit and the guarded push were made by the agent on the operator's explicit word** ("ON MY
  WORD you make the operator pre-CI commit with that subject and run the guarded push yourself, as in every earlier
  chunk") — the operator's act carried out by the agent, as in every chunk this version (relay §1).
- **`measured_2026_10_01` was read off the rule** with a temporary print-only test, deleted before the gate run — the
  table records the rule's output, never a hand reading.
- **Scope record** (`gate.py scope` at P1: `clean — changed 5 · listed 5 · recorded 0`; the record's two lines are in
  the bookkeeping tree the read excludes, printed `record: not changed`):
  - widening: `.andromeda/runs/2026-10-01T18-42-55-phase/p5-dryrun.MOVED.md` and `…/p5-dryrun-2.MOVED.md` — serve step
    13 (hygiene) — word: "yes, move both P5 dry-run transcripts per the 2026-09-30 precedent (gitignored p5-controls,
    a .MOVED.md note each naming sha256 and the forms in words), record it in scope-record.md on my word, re-run
    hygiene to clean" — the overseer. The two transcripts (real host paths in the gate tool's header) now sit in the
    gitignored `.andromeda/cache/p5-controls/2026-10-01T18-42-55-phase/`, sha256 unchanged.

## Decisions & corrections
- Overseer: the d2 span-identity gap is routed forward at this wrap (placement directed: relay §2, entry 1); no change
  in this chunk.
- Overseer: d3 residual → option (a), stated residual like 2026-09-22, no new elision code.
- Overseer (relay §1): `v3-09`'s next step is the FOUNDER's call, asked 2026-10-01 and unanswered — options: a fourth
  series after the d2 fix; deferring `v3-09` at the version close on the three measured series; revisiting whether
  the retry-token bar is the right interpretation test. Recorded as awaiting his word; no fourth-series entry minted.
- Overseer (relay §2): mint two entries at the head of the markerless tail before "Version close on measured
  evidence": (1) per-run span identity in the real-model harness; (2) the P-075 assert round against Pulse,
  BLOCKED-ON Pulse "Conductor return" relaying sha S.
- Sweep hazards found this chunk:
  - `elide_fingerprints` keeps an all-digit hex-looking run by design, so a fingerprint prefix made only of digits
    passes the scrub chain (measured on d3: one 8-digit value, two lines). A grep for `fingerprint_hex=[0-9a-f]*[a-f]`
    misses it; count all-digit runs after `fingerprint_hex=` instead.
  - A live run's "drive spacing" keyed on incident formation cannot see a SUT-side buffer keyed on span identity: a
    seeded identity replayed inside the buffer window is silently refused (`append_failed`), and the drive reads as a
    plain `attribution: none` — the discriminator is Pulse's `duckdb.append` `reject_reason` beside `buffer.tick`
    `append_rejections`.
  - A gate tool's dry-run TRANSCRIPT committed into a run dir carries the tool's own header host paths (shell, root,
    log dir) — the hygiene gate refuses it; same class as 2026-09-30's moved control.

## Outcome
- **v3-09 — NOT MET by the 2026-10-01 series, measured under the byte-identical rule.** d1 `Identified` (rank 1
  `retry_storm scope_id=conductor is anomalous` — it restates the cue line, the stated limit); d2 not graded
  (`NoAttributableIncident`, the scenario's spans refused); d3 `NotIdentified` (rank 1 "Error Rate Spike in
  <workspace-key> indicates a potential issue with the Conductor service." — names `conductor`, no retry token).
  Further grades: d1 P-031 Pass / P-034 Pass / P-044 Blocked (no prior same-scope incident); d3 Pass / Pass / Pass;
  d2 Blocked ×3. Pass condition (≥ 1 graded AND every graded `Identified`) fails on d3 → `v3_09_is_not_met_by_the_2026_10_01_series`.
  The matrix entry stays `deferred`, `ref` null; no ref test written. Its next step awaits the founder's word.
- **Beside the verdict, never softening it — the Pulse fix is visible live:** the real model surfaced 6 of 6
  cue-bearing canary digests (`skip_reason=none` each) and the scenario's own storm digest both times it formed (d1
  created 19:39:44.530Z; d3 created 20:04:57.902Z) — 0 of 2 on 2026-09-30. Inside the three leg windows Pulse's own
  skip lines were all cue-less tier-3 (`no_cue` 14, `model_resolution_summary` 1). `prompt_version` `v2.3` throughout.
- **Acceptance criteria, re-asserted against the diff:**
  - (tests) harvest green under both runners (nextest 100/100, `cargo test` 100 passed); the pairing arms and the
    source arm hold; prior series' pins and grades unchanged — MET.
  - (tests) rule byte-identical to `1fe46a1` (rule-diff probe: no output) — MET.
  - (arch) the series fixed add-only before d1 (removed-line probe 0; pre-registration sha256 `0232afb1…` recorded
    in the ledger after the builds and before d1 fired 19:35:32Z; `the_2026_10_01_series_rule_was_fixed_before_d1` green); no Rust reader of the
    contract, no handle/verb/selector — MET.
  - (arch) no change under `crates/conductor-{verify,core,cli,emit,faults,report,timeline,tauri}`,
    `crates/conductor-run/src/`, manifests, `scripts/`, `scenarios/`, `.github/` (numstat: no output); `Cargo.lock`
    562 — MET.
  - (security) both binaries rebuilt from `a2addb3` after a clean gate re-check and proven before d1, which arm held
    recorded (`pulse-app` content, sidecar provenance) — MET.
  - (security) no host path / workspace key in committed evidence (probes 0 / 0); `no_committed_capture_text_sits_in_test_source`
    covers the series; each capture passes `elide_fingerprints` unchanged — MET, with the d3 all-digit prefix as the
    overseer-ratified stated residual (counted exactly by the harvest).
  - (layouts) every drive fired as `bash scripts/agent-run.sh run --live real-model`, each after the non-priming
    probe whose `[PRECONDITION] every live-Pulse precondition is satisfied` is recorded — MET.
  - (tests) the series ran as pre-registered: exactly d1-d3 in the operator's slot after the gate re-check, the
    posture confirmation and the probe; each `rule_record` equals the current rule; one ledger row each; no re-fire,
    no fourth drive; skip-line header count 3 — MET.
  - (obs) every envelope eleven keys, `verdict` null, `state` `ManualCheck`; no new span/attribute/key — MET.
  - (obs) Pulse's `skip_reason` and `prompt_version` recorded in the ledger as witnesses, never grade inputs — MET.
  - (tests) `v3-09` graded by the pre-stated rule over the three digest-pinned captures; ref test only on pass — MET
    as a measurement (the capability itself NOT MET).
  - (CI) push → `verdict: green`, CI#36921742915 on `3791d37` — MET.
- **Gates (implement's final run over the final tree, re-run by this wrap's light gate):**
  - `cargo nextest run -p conductor-run --test real_model_harvest --profile ci` — green (100/100).
  - `cargo test -p conductor-run --test real_model_harvest` — green (100 passed).
  - `cargo nextest run -p conductor-run --test capture_paths_guard --profile ci` — green.
  - `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` — green.
  - `cargo nextest run --workspace --profile ci` — green (1147/1147, `secret_scan_gate` included).
  - `cargo test --workspace --doc` — green.
  - `cargo clippy --workspace --all-targets -- -D warnings` — green.
  - `cargo fmt --all --check` — green.
  - `grep -c "^name = " Cargo.lock` — green, last line 562.
  - the rule diff against `1fe46a1` — green, no output.
  - `git diff --numstat 1fe46a1 -- contracts/pulse-real-model-leg-posture.md | awk …` — green, 0.
  - `git diff --numstat 1fe46a1 -- crates/conductor-verify/ … .github/` — green, no output.
  - `git diff --numstat 1fe46a1 -- …frozen series chunks…` — green, no output.
  - `git -C "$CARGO_HOME/advisory-db" status --porcelain` — green, no output (HEAD = FETCH_HEAD checked beside it).
  - `cargo audit` — green, exit 0 (1278 advisories · 562 crates · 7 allowed warnings).
  - `cargo deny check advisories bans licenses sources` — green (advisories ok, bans ok, licenses ok, sources ok).
  - the four Pulse gate re-check probes (ancestor · `rev-list --count` · build-input porcelain · `rev-parse HEAD`) —
    leg operator, fired by hand: exit 0 · 0 · 0 lines · `a2addb3…` (evidence ledger §Gate re-check).
  - the two Pulse release builds — leg operator, in the overseer's build slot: exit 0 each (`Finished` 8m 16s / 3m 04s).
  - the content probes `incident producer skipped` (1, exit 0) · ` active-bypass incident(s); ` (0, exit 1) · the
    sidecar reading (0/0/0, recorded) · sha256 + mtime (recorded) — leg operator, by hand.
  - `ls -1 "$TEMP/pulse-legs"` — leg operator, recorded (22 entries, the leaf absent).
  - the preconditions probe — leg operator, fired before each drive: exit 0, atom held ×3.
  - the three drives `… agent-run.sh run --live real-model && cp …/rm-capture-dN.txt` — leg operator, in the
    overseer's model-run slot: exit 0 each, both atoms held each, artifacts fresh.
  - `bash scripts/agent-run.sh status <id>` (d3's `2026-10-01T20-01-03-329`) — leg operator: exit 0, atom held (1 hit).
  - the skip-line header count — green, last line 3.
  - the host-path probe over every evidence file — green, 0 at exit 1.
  - the key probe — green, 0 at exit 1.
  - `gate.py hygiene` — leg operator: first `refused 2 files` (the P5 dry-run transcripts), moved on the overseer's
    word; then `hygiene: clean`.
  - the guarded push — leg operator, on the overseer's word: `PUSHED_SHA=3791d370…`.
  - `ci.py conclusion --sha HEAD --wait 1500` — leg operator: `verdict: green` (CI#36921742915).
  - Smoke: `status` mint-then-read on d3's run — exit 0, envelope `ManualCheck` / null.
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran — the pre-CI commit `3791d37` (Setup 4: the only commit since its parent
  `1fe46a1`) and its CI run CI#36921742915 recorded in `evidence/attempt-ledger.md`; implement's conversation and its
  P4 report (this session) for everything else; the ledger's CI line is the one post-commit edit in the tree.
- **Process hygiene:** `pulse-app` PID 34792 + 10 descendants (agent-launched on the overseer's word) — terminated
  20:08Z, `:4317`/`:4318` released, census equal to the pre-series baseline; the two Pulse release builds — exited 0;
  each drive's `conductor` / sidecar / `llama-cli` — terminated (no survivor after any drive). Census 20:38:03Z: a
  `cargo clean --profile dev` pair (PIDs 54924/38624, another session's shell snapshot) — not this chunk's, left
  running.
