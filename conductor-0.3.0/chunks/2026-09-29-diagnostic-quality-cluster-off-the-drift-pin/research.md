# Codebase Research — 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 31
- **Harness rules consulted:** `.claude/rules/verification-harness.md`, read IN FULL by structural extraction: `grep -n` index of the 5 headers and 28 Session-Additions introducers, then offset reads covering `:1-49`, `:50-58` and `:59-67`. Applied: the quiet window (≥120 s idle + a 30 s resolver tick after ANY preceding canary, `:50`/`:53`); fresh data dir per leg with a `pulse-app` restart (`:50`); `[BLOCKED]` in ~0 s = sidecar resolution (`:54`); the process census twice, with a named stop form (`:58`); model-side classification only on the parse-`ok`-without-`interpretation.incident.created` pair, and only a pipeline fault may be re-fired (`:67`, 2026-09-23); the `--live real-model` firing form (`:19`).
- **Platform issues consulted:** none — no runner-only bullet stands open. The folded CI verdict closed against its recorded run (below), and no CI-reading entry sits outside the operator leg.
- **Pulse coordinates:** read ONLY at committed HEAD `f15536b909af814e35eba43a50988cb863223d16` via `git -C D:/dev/projects/andromeda-pulse show f15536b:{path}` (re-verified 2026-09-29; the worktree holds uncommitted P-025 work and was never read).

## Files inspected
- `contracts/pulse-real-model-leg-posture.md` (full) — the posture, emission profile and grading rule, with dated corrections. `:176-178` states "A failing leg … is never re-driven until it passes. There is no retry-once policy".
- `crates/conductor-run/src/canary.rs` (full) — the storm (`CANARY_STORM_COUNT = 12` on `CANARY_SERVICE_NAME = "conductor-canary"`, `:132`/`:139`); the warm-up (3 benign spans over 45 s, `:259-281`); the marker `ConductorCanary_{now_ms}` (`:236`); the poll `CONDUCTOR_PREFLIGHT_TIMEOUT` raised to the contract floor (`:288-299`); `preflight_for(…, posture)` (`:48`).
- `conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/evidence/rm-capture.txt` (full, 12 355 B) — the one graded drive. The canary storm reached `autonomous` at 07:40:24.999Z. The four `interpretation.json.parse` lines are all `ok`: output_bytes 395 / **1192** (the 07:40:25 cue-bearing digest, prompt token_count 6312) / 393 / **1098** (07:41:41, token_count 6322). `interpretation.incident.created: 0`, `inference.error: 0`, `inference.skipped: 0`. The `preflight:` line names BOTH causes of the third precondition (workspace-key divergence, or "Pulse raised no incident for the canary").
- `crates/conductor-run/tests/real_model_live.rs` (`:1-60`, `:434-484`, `:539-662`) — the capture prints the attributed report's `## Hypotheses` and `## Evidence` ONLY (`report_section` at `:465-473`). Its Pulse-log target set (`:604-624`) carries no `digest.*` line and no corpus-retrieval line.
- `crates/conductor-run/tests/real_model_harvest.rs` (fn index + rule section via the capture) — the grading rule. Its committed-capture pins (`committed_capture` `:790`, `pinned_literals_equal_the_committed_capture` `:872`) bind to the 2026-09-23 capture.
- `scripts/agent-run.sh` (`:157-205`, `:332-341`) — `live_real_model_leg`: preconditions `--for` → capture pre-build → non-recursive clear of `rm.jsonl` / `rm-capture.txt` / `rm-capture.err` → the rule record → the leg → the capture. `:160` says "never re-driven", as prose; no lock or marker enforces it.
- `contracts/pulse-run-contract.toml` (terms) — `[incident_formation]` `warmup_ms = 45000`, `warmup_emissions = 3`, `min_canary_poll_seconds = 90`; seven terms; the `l4-real-model` `shell-absence` term.
- `crates/conductor-core/src/drift.rs` (`:30-63`) — `UNBACKED_AUTO` at `:61-63` (8 ids); the interpretation-correctness doc at `:48-56` still reads "Owner: a conductor-0.3.0 entry".
- `coverage-matrix.md:3` — `43 auto (8 unbacked)`; rows `:37`/`:40`/`:50` for P-031/P-034/P-044.
- `crates/conductor-core/src/coverage.rs:275/287/293/353` — the four `Auto` rows (P-031 Report Structure, P-033 Ranked Hypothesis Generation, P-034 Suggested Investigation Steps, P-044 Retrieval-Augmented Interpretation).
- `crates/conductor-core/src/scenario.rs:1520-1535` — `real_model_interpretation_declares_the_known_cause_under_the_real_model_posture` pins `p_ids == ["P-018"]` (`:1528`).
- `scenarios/real-model-interpretation.toml:1-40` — `p_ids = ["P-018"]`. Its header states that naming P-033 "moves `UNBACKED_AUTO` and the committed coverage matrix together, which is v3-10's change".
- **Pulse `pulse-app/src/inference_runtime.rs@f15536b`** (`:150-380`, `:640-973`):
  - `process_digest` (`:190-273`) routes a parsed success to `create_incident_from_l4_output`, unless `is_resolution_summary` is set.
  - The creation predicate `:755-760` returns SILENTLY on `parsed.is_resolution_summary`, `Decision::Dismiss` or `Severity::None`.
  - A cue-less non-reflection digest also returns silently (`:775-777`).
  - Both creation and dedupe log `interpretation.incident.created` through `emit_incident_outcome` (`:912-933`). Its absence after a parse `ok` therefore means an early return, never an unlogged dedupe.
- **Pulse `crates/interpretation/src/schema.rs@f15536b`** (`:100-176`) — `Decision {Surface, Dismiss, Watch}`; `Severity {Autonomous, Suggested, Curious, None}`. A `Watch` with severity ≠ `None` DOES create an incident.
- **Pulse `crates/interpretation/src/prompt.rs@f15536b`** (`:80-175`) — role and conventions: "surface / dismiss / watch", "none (informational)". All three builder call sites pass `corpus_retrieval = ""` (`inference_runtime.rs` `handle_digest_outcome`).
- **Pulse `pulse-app/src/llamacli_inference.rs@f15536b`** (`build_llama_cli_args`) — `-m -ngl -st --simple-io --no-display-prompt --log-disable -n --json-schema-file -p`. There is NO `--seed`, `--temp` or `--top-k`, so llama.cpp's default stochastic sampling applies. The runner logs no raw output.
- **Pulse `crates/triage/src/digest/assembler.rs@f15536b`** (`:200-340`, `:600-690`):
  - The model sees `WINDOW`, `PROJECT` (+ `RECENT CHANGES`), `OVERALL`, `SERVICES (rate, error%, p99)`, the `ATTENTION CUES` line `[{tier}] {kind} — {kind} scope_id={service}` and `CORPUS MATCHES`. The exception type and message never reach the model.
  - `digest.assemble` logs `cue_kind` / `cue_priority_tier` (`:211`).
  - Corpus retrieval logs `row_count_returned = candidate_count`, the candidates BEFORE selection (`:309-318`). The selected matches reach only the unlogged payload.
- **Pulse `crates/triage/src/digest/retrieval.rs@f15536b`** (`:1-130`) — P-044's digest-side `select_corpus_matches` (same workspace, 30 days, fingerprint or scope match, top-5) and P-036's report-side `select_previously_seen`. Two DIFFERENT selections.
- **Pulse `crates/interpretation/src/markdown.rs@f15536b`** (`:1-60`, `:120-260`) — `serialize_report` always renders the six P-031 sections: Symptom / Timeline / Hypotheses / Investigation Steps / Evidence / Project Context. Each has an explicit empty placeholder and a `DEGRADED_NOTICE` for Hypotheses and Investigation Steps. Investigation steps render numbered (`1. {step}` + `_Expected yield:_`). `## Previously Seen` renders only when non-empty.
- **Pulse records** (`git grep` over `*.md`@f15536b for real-model surfacing) — no measured real-model incident creation anywhere in Pulse's own record. The only real-model mentions are posture/setup text and a driver trap (`.claude/rules/verification-harness.md:126`@f15536b).
- `gh run view 36529121865` / `36529176256` + `.github/workflows/ci.yml:11-13` — the folded CI verdict (closure below).

## Graph impact (rust plane, `db_state: fresh`, trace `.andromeda/runs/2026-09-29T06-16-19-phase/tree-query-2026-09-29-diagnostic-quality-cluster-off-the-drift-pin.json`, `rows: 25`)
- **preflight_for** — 4 sites: `conductor-cli/src/commands/run.rs:21` (the posture-selected run path), `conductor-run/src/canary.rs:42` (`preflight`), the re-export `conductor-run/src/lib.rs:30`, `tests/composition_root.rs:51`. A canary change inside `canary_gate` reaches every scenario run of both postures.
- **canary_gate** — 2 callers, `canary.rs:67` (`preflight_for`) and `canary.rs:89` (`readiness`, the deterministic `boot` gate). A real-model-only canary change must key on `posture`, or it moves the deterministic gate too.
- **emit_canary_storm** — 9 sites. `canary.rs:249` (production), re-export `lib.rs:30`, and the companions `tests/canary_obs_witness.rs:15,46`, `tests/canary_wire.rs:25,38,159,209`, `tests/lifecycle_live.rs:39`.
- **check_scenario_backing** — the production call `drift.rs:368` plus its own unit arms `drift.rs:375-448` and the re-export `lib.rs:41`. It reads the catalog's `p_ids` and nothing else, so ANY committed scenario naming an id backs it.
- **UNBACKED_AUTO** (grep; a `term`, not a call): `conductor-report/src/coverage.rs:29,236,253,264-266` (renders `.len()`); `conductor-cli/src/render.rs:299,794-795` (`.len()`); `conductor-tauri/src/commands.rs:172-175,461` (returns the constant, and the IPC test compares against the constant); `ui/src/App.tsx:145` → `CoverageMatrix.tsx:30-33` (the count from IPC). Every consumer single-sources the constant, and no literal `8` pins it (`grep -rnE '8 unbacked|\(8 unbacked\)' crates` → 0 hits; the one literal is `coverage-matrix.md:3`, the rendered artifact).

## Patterns detected
- **Silent model-side exit** (Pulse `inference_runtime.rs:755-760`@f15536b): the three incident-suppressing branches read only model-authored fields and log nothing. The one witness is the pair "parse `ok` on the cue-bearing digest, no `interpretation.incident.created` after it" (harness rule `:67`).
- **Stochastic sampling by construction** (Pulse `llamacli_inference.rs` `build_llama_cli_args`@f15536b): no seed or temperature argument, so identical prompts need not yield identical decisions.
- **One cue-bearing digest per canary** (capture `rm-capture.txt:255-257`): the 12-occurrence storm raised `suggested@5` then `autonomous@10` once. Only the 07:40:25 digest carried the cue. Every other digest in the window was a cue-less cadence tick, which the predicate skips at `:775-777`. A canary therefore gets ONE model decision per drive.
- **Freshness gate** (`canary.rs:235-251` + `conductor-verify` `run_preflight`): the stamp is taken after the warm-up and before the storm. Any incident opened after it satisfies the gate, and a later surfacing storm would too.
- **Capture printed once, graded at the harvest** (`real_model_live.rs:1-25`; `real_model_harvest.rs` rule section): new graded surfaces extend the capture's printed sections and the harvest's rule together. `the_capture_prints_every_token_the_rule_reads` (`real_model_harvest.rs:688`) holds the capture source to the rule's constants.

## Conventions to follow
- **Pulse at a pinned HEAD**: every SUT coordinate this chunk cites is `f15536b`. The posture contract's `83d4060` coordinates are re-read before being restated (the arch history's precedent).
- **Posture-keyed canary behaviour**: `preflight_for` carries the scenario's `L4Posture` (`canary.rs:48`). A real-model-only change keys on it, so the deterministic suite and `boot` (`readiness`, `canary.rs:89`) stay byte-identical.
- **Harness-owned capture files, cleared non-recursively** (`agent-run.sh:176`): per-drive artifacts keep that idiom (`rm -f` on named files, never a recursive delete).
- **Quiet window, never a sleep inside a test** (harness rule `:50`, test-plan history `2026-09-06-operator-gated-live-suite`): spacing between drives is an operator or harness step.
- **Every committed capture byte-pinned by the harvest** (`real_model_harvest.rs:790-880`): each new drive's capture becomes a committed evidence file with its own pinned-literal test. The 2026-09-23 capture stays pinned as it is.

## New files to create
- derived `conductor-0.3.0/chunks/2026-09-29-diagnostic-quality-cluster-off-the-drift-pin/evidence/*` by `bash scripts/agent-run.sh run --live real-model` — one scrubbed capture per drive plus the attempt ledger, committed

## Files to modify
- `crates/conductor-core/src/drift.rs` — drop P-031/P-033/P-034/P-044 from `UNBACKED_AUTO` and retire the "Owner: a conductor-0.3.0 entry" doc paragraph to the shipped state
- `coverage-matrix.md` — regenerated by `conductor coverage --write` in the same change (roll-up `(8 unbacked)` → `(4 unbacked)`)
- `scenarios/real-model-interpretation.toml` — `p_ids` gains the four ids; the header's v3-10 sentence becomes the shipped state
- `crates/conductor-core/src/scenario.rs` — the `:1528` `p_ids` pin moves with the TOML
- `crates/conductor-run/tests/real_model_live.rs` — print all six report sections plus `## Previously Seen`; add the `digest.assemble` and `digest.corpus_retrieve` Pulse lines to the target set
- `crates/conductor-run/tests/real_model_harvest.rs` — grading arms for P-031/P-034/P-044 and the canary-attempt ledger's pinned literals; the 2026-09-23 capture's pins stay
- `crates/conductor-run/tests/real_model_common/mod.rs` — only if a shared section helper moves there
- `contracts/pulse-real-model-leg-posture.md` — the extended grading rule, fixed BEFORE any drive, and the re-drive policy the P4 fork decides
- `scripts/agent-run.sh` — per-drive capture naming, so a series does not overwrite a capture; the `:160` "never re-driven" prose made true to the decided policy
- `scripts/agent-run.ps1` — the same change at parity
- `crates/conductor-run/src/canary.rs` — CONDITIONAL, only if the measured cause is Conductor-side (for example several spaced storms per real-model canary, keyed on `posture`)
- `crates/conductor-run/tests/canary_wire.rs` — CONDITIONAL companion of a `canary.rs` storm-shape change (pins the storm count and service)
- `crates/conductor-run/tests/canary_obs_witness.rs` — CONDITIONAL companion of the same change
- `crates/conductor-run/tests/severity_harvest.rs` — CONDITIONAL companion (`CANARY_SAMPLE_COUNT = 3 + CANARY_STORM_COUNT` at `:39`)

## Scope premise closure (the scope's `[inferred]` bullets)
- **Three candidate causes** → VERIFIED as all still open, and sharpened, at `f15536b`. The incident-suppressing exits are ALL model-authored fields (`inference_runtime.rs:755-760`). Sampling is stochastic by construction (no `--seed` / `--temp`). The model never sees the canary's exception type or message, only `conductor-canary`'s RED row, the cue line and any `CORPUS MATCHES` (`assembler.rs:600-690`). And NO Pulse surface records the decision. So "nondeterminism", "digest too weak" and "prompt/thresholds" separate only by OUTCOME FREQUENCIES under controlled variation. No single drive can separate them.
- **Whether any surface shows WHY** → VERIFIED: none at `f15536b`. There is no decision or severity log on the silent exits and no raw-output log in the runner, and a dismissed output is persisted nowhere. A direct discriminator needs a Pulse-side observability change (a bounded decision/severity label on the predicate's exits), which is a Pulse route item. The overseer carries it.
- **The acceptance conflict** → VERIFIED. The re-drive ban is stated in FOUR places:
  - the `v3-09` acceptance ("recorded and never re-driven");
  - `contracts/pulse-real-model-leg-posture.md:176-178`;
  - `scripts/agent-run.sh:160` (prose, unenforced);
  - test-plan §9 ("fired once and never re-driven", per the tests extract).

  It is a P4 fork.
- **P-031 / P-034 / P-044 from the same capture** → [premise-corrected: P-031 and P-034 are gradable from the capture once it prints them (six always-rendered sections, a numbered Investigation Steps list — `markdown.rs@f15536b`); P-044's digest-side retrieval is NOT directly observable — Pulse logs the candidate count before selection only (`assembler.rs:309-318`) and the selected matches reach only the unlogged prompt payload, so the one external witness is an inference chain through the report-side `## Previously Seen` (P-036's sibling selection).]
- **Which scenario names the ids** → VERIFIED. `check_scenario_backing` reads catalog `p_ids` only (`drift.rs:368`), so any scenario backs an id. The honest backing is `real-model-interpretation`, the only path exercising interpretation. Its `p_ids` pin at `scenario.rs:1528` moves.
- **CI `17379d6` not green** → VERIFIED against its run. CI#36529121865 (created 06:03:18Z) was cancelled at 06:04:55Z by the `ci-${{ github.ref }}` `cancel-in-progress` group once CI#36529176256 for `a76420a` started at 06:03:59Z. That run concluded `success`, and the diff `17379d6..a76420a` is two run-dir JSON trails. No failing subject exists.
- **CONTEXT "model-side, inferred from an absence"** (verbatim marker "measured from Pulse's own log") → VERIFIED at `f15536b`. `emit_incident_outcome` logs on create AND dedupe, so zero `interpretation.incident.created` after a parse `ok` means an early return, and every early return reads a model field.
- **CONTEXT "the canary is itself a real-model gate"** → VERIFIED at `f15536b` (the same predicate; `preflight_for` runs the canary under either posture).

## Measurement facts for the plan (Problem 1)
- **Per-drive cost when blocked:** 45 s warm-up + the poll (600 s at the prior drive's `CONDUCTOR_PREFLIGHT_TIMEOUT`) + the capture's own `POLL_BOUND` 600 s (`real_model_live.rs:59`) — about 12-20 min each. Between drives on one launch the canary leaves no incident to dedupe against, and each drive's marker is unique. The only spacing owed is Pulse's 60 s storm-retention window, while the 150 s quiet window binds once any incident HAS formed (harness rule `:50`/`:53`).
- **The statistics of a Bernoulli series:** with n identical drives all dismissing, the 95 % upper bound on the per-drive surfacing probability is `1 − 0.05^(1/n)` — about 0.63 at n = 3, 0.45 at n = 5 and 0.31 at n = 8. Zero surfacings in a small series cannot establish "never". One surfacing in any series refutes "systematic".
- **Conductor-side levers on what the model sees** (the only ones Conductor holds):
  - the canary's SERVICE name (`conductor-canary`, registered in arch §Occupied Resources);
  - its RED row shape (3 benign warm-up spans dilute the error %; the storm count sets the rate);
  - the number of cue-bearing digests per canary (several spaced storms with unique markers, each ≥ the 60 s retention apart);
  - the corpus state the digest's `CORPUS MATCHES` reads, through the operator's data dir (fresh vs existing). This is a launch-side lever, not code.
- **Not a Conductor lever:** the prompt, the schema, the sampling and the creation predicate, all Pulse's (directive item 2: stop and report).

## P4 addenda (read during synthesis, Pulse `f15536b`)
- **Generation damper** (`crates/triage/src/digest/damper.rs`@f15536b):
  - Identity = (workspace, digest kind, first cue's kind + scope_id) (`identity`, `:126-133`).
  - The projection includes each cue's `fingerprint` (`:251`) and the Q1 service rows (`:259-273`).
  - `record_generated` fires on every clean parse, a DISMISS included (`inference_runtime.rs` `process_digest`), and a cue identity is evicted after 300 s of absence (`DAMPER_CUE_EVICTION_SECONDS`, `:30`).

  So a second canary storm with a NEW fingerprint changes the projection and generates, while an unchanged one within
  300 s is suppressed. The test `changed_projection_generates_and_reports_the_released_run` (`:355-370`) pins exactly
  that fingerprint case.
- **Cue latch:** `CUE_LATCH_REFRACTORY_NANOS = 60 s` (`crates/triage/src/cue/emitter.rs:28`@f15536b). Storms on one
  service need ≥ 60 s between them to raise a fresh cue, and the plan uses 90 s.
- **Log target strings and allowlist:**
  - `digest.assemble.request` (`crates/triage/src/digest/mod.rs:66`) logs `mode`, `cue_kind`, `cue_priority_tier`.
  - `digest.corpus.retrieve` (`:74`) logs `query_id`, `row_count_returned`, `duration_ms`.
  - `digest.runtime.cadence_tick` (`pulse-app/src/digest_runtime.rs:48`) logs `cue_present` (`:145`).

  All of these fields are in Pulse's default-deny allowlist (`pulse-app/src/observability.rs:1870-1874`,
  `:1919-1923`, `:1947-1948`@f15536b), so they read live and never as `"<redacted>"`.
- **Capture Pulse-target table:** `real_model_live.rs:603-624` — a `(target, keys)` array, so a new target is a new
  row. The `creating digest prompt_version` locator (`:654-672`) already finds "the last prompt assembly before the
  first incident creation at or after the emission instant". P-044's retrieval witness uses the same locator, with
  `digest.corpus.retrieve` in place of `interpretation.prompt.assemble`.

## Open questions
- Re-drive policy: the four "never re-driven" statements against the founder's ruling and the measurement's need for a series. Is the rule re-worded to a bounded, pre-stated drive series whose every drive is recorded, and how does the `v3-09` acceptance travel? → blocks: plan-decision
- Measurement design: how many drives, and which controlled variations (identical repeats · fresh vs existing data dir · a canary identity or multi-storm variant), with the decision rule mapping each outcome to a cause stated before the first drive. Is a Pulse-side decision log requested up front through the overseer, or only if the series stays all-dismiss? → blocks: plan-decision
- P-044 grading basis: the inference chain (candidates ≥ 1 on the cue digest + `## Previously Seen` naming a same-scope incident), or record P-044's digest-side retrieval as not observable from outside and route a Pulse observability ask. This decides whether the pin can drop P-044 honestly in this chunk. → blocks: plan-decision
