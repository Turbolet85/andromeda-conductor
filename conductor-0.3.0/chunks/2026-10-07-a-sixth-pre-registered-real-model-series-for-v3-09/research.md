# Codebase Research — 2026-10-07-a-sixth-pre-registered-real-model-series-for-v3-09

## Scope
- **Depth:** deep on the series surface (the posture contract, the harvest's series tree, the fifth series' chunk
  folder) and on Pulse at `9bfefb8`; nothing else read · **Reads:** 19 · **Globs/Greps:** 24
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — the file is past the read cap, so it was
  read structurally: an index of its four section headers and 39 dated entries (`grep -n -E '^## |^- [0-9]{4}-'`),
  then 24 entries read whole (`:48-54`, `:60-71`, `:73-77`); the other 15 (`:40-47`, `:55-59`, `:72`, `:78`) were
  read by their index line only. Applied here: the sidecar built and on `PATH`, and the app launched from a cwd
  outside this repository (2026-08-10); mint-then-read for `status` (2026-08-11); one launch serves several drives
  behind a quiet window (2026-08-16 as extended 2026-09-04); one `pulse-legs` parent, a letters-only leaf
  (2026-08-18 as refined); never `boot` before a drive (2026-08-19); a `[BLOCKED]` in about 0 s is sidecar
  resolution (2026-08-20); the `run --live` mechanics — each leg's stream frozen before the next, a `Blocked` row
  exits 0 (2026-09-06); a live leg's atoms are read from what the runner prints (2026-09-10); a no-incident outcome
  is read from `interpretation.incident.skipped` (2026-09-23 as corrected 2026-10-01); a harness-defect re-fire is
  once and on the operator's word (2026-09-30); the 10 s liveness check, the WebKit lever only as one recorded
  relaunch (2026-10-03 as corrected 2026-10-06); a binary proof is two-sided before it is pre-registered
  (2026-10-06); one shared dir confounds the drive ordinal with what accumulates there, so the corpus-row line
  stays in the capture (2026-10-07); a drive's bracket ends before the capture's own read-back (2026-10-07). The
  process-census rule (2026-09-02, `:58`) was read by its index line and reaches this plan through the fifth
  series' census entries.
- **Platform issues consulted:** none — the one not-green CI row Setup read (`717bbf3`) is a concurrency cancel
  with no failed check and so no failure signature to search (`.github/workflows/ci.yml:11-13`; scope.md §CI), and
  this chunk lists no CI-reading entry outside the operator leg.
- **External inputs:**
  - `inputs#I1` — the overseer's relay for the sixth series: what shipped at `9bfefb8` and how it was measured, the
    founder's rulings of 20:08, what is carried in, the pass-through not used, the GPU closed tonight.
  - `inputs#I2` — the phase directive: promote `:98`, pin Pulse `9bfefb8`, Part A tonight, the sitting waits for
    daytime and the go.
  - `inputs#I3` — Pulse's master route holds the remedy chunk `pending` at `:103`.
  - `inputs#I4` — Pulse's working route carries the stamped entry at `:180`, the artifact the block names.
  - `inputs#I5` — `retrieval.rs` at `9bfefb8`: `select_corpus_matches` narrows its scope arm to the triggering
    scope, before the cap, and only removes.
  - `inputs#I6` — `assembler.rs` at `9bfefb8`: the assembler passes the triggering cue's `scope_id`.
  - `inputs#I7` — the plain `l4-env.sh`: four exported handles, the L4 flag left unset.
  - `inputs#I8` — Pulse's record of the shipped remedy `CX`, its counts and what it does not cover.
  - `inputs#I9` — Pulse's replay reading: 0, 1 and 11 service misses of 20 on the three captured prompts.
  - `inputs#I10` — Pulse's dry run: the product selection composes the bytes the `CX` arm composed.
  - `inputs#I11` — Pulse's root `Cargo.toml`: the release profile strips symbols.
  - `inputs#I12` — the sidecar's manifest: it links `crates/triage` and `crates/interpretation`.
  - `inputs#I13` — `schema.rs` at `9bfefb8`: prompt versions `v2.6`, `v1.5-fallback`, `v1.5-reflection`.
  - `inputs#I14` — the operator's answers to the two plan forks, with the timing fact that the build waits for
    the word that Pulse's wrap is committed.

## Files inspected
- `contracts/pulse-real-model-leg-posture.md` (`:572-761`, the fifth series' section and the capture run's) — the
  form the new section repeats clause for clause; the new section lands between `## The 2026-10-07 capture run`
  and `## The quiet window and serialization` (`:761`). The file is 63646 B and names six env handles
  (`grep -oE '(ANDROMEDA|CONDUCTOR)_[A-Z0-9_]+' … | sort -u`: 6).
- `crates/conductor-run/tests/real_model_grading/mod.rs` (full) — the `Series` shape every dated series grades
  through; `contract_section` finds a section by its whole heading line (`:146`) and ends it at the next `## `
  heading (`:148`); `pre_registered` reads the ledger line `pre-registration sha256: ` (`:160`).
- `crates/conductor-run/tests/real_model_grading/series_2026_10_07.rs` (full) — the module the new one mirrors
  test for test: seven tests, the `Series` constant, the digest pin, the key constant, the `measured` table.
- `crates/conductor-run/tests/real_model_series/mod.rs` (`:100-168`) — pins are labels, file names and sha256
  digests only; the last constants are the capture run's (`:148`, `:152`).
- `crates/conductor-run/tests/real_model_harvest.rs` (`:46-72`, `:1250-1342`) — the registration sites: the `use`
  lines (`:62-67`), the constants list (`:68-71`), `GRADING_MODULES` at length 13 (`:1256`), `graded_captures`
  (`:1309-1342`). The rule sits between its markers at `:74` and `:462`.
- `crates/conductor-run/tests/real_model_grading/capture_population.rs` (full) — `COMMITTED_CAPTURES` is 23
  (`:9`), walked from every chunk's `evidence/`.
- `scripts/agent-run.sh` (`:76-88`, `:196`) — the live arm's two printed lines and the leg's label and scenario.
- `conductor-0.3.0/chunks/2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09/` (`scope.md`, `plan.md`
  full; `evidence/attempt-ledger.md` `:30-98`, `:244-276`) — the plan this one repeats, its measured timings, its
  binaries table and its recorded `status` smoke.
- `.github/workflows/ci.yml` (`:11-13`) — the concurrency group that cancelled the `717bbf3` run.
- Pulse at `9bfefb8`, read with `git show` and `git diff`, never its working tree: the product diff against
  `f70be92` (inputs#I5, inputs#I6), three evidence files (inputs#I8, inputs#I9, inputs#I10), the release profile
  (inputs#I11), the sidecar's manifest (inputs#I12), the prompt versions (inputs#I13).

## Pulse at `9bfefb8`
- `9bfefb8` (`9bfefb812297bbdea610423a21568b3262bdb7ed`, committed 2026-10-07T20:38:39+02:00) is HEAD of
  `chore/migrate-pulse-to-v3`, 0 ahead of its upstream. Its CI run `ci#37668429742` reads `success` on attempt 2,
  12 of 12 jobs.
- Product files that differ from `f70be92` (`git diff --stat f70be92 9bfefb8 -- crates pulse-app/src
  pulse-app/Cargo.toml xtask Cargo.toml Cargo.lock`): two, `crates/triage/src/digest/assembler.rs` and
  `crates/triage/src/digest/retrieval.rs`. Two more files differ under `pulse-app/examples/`, the probe; an example
  is not linked into `pulse-app`.
- Unchanged since `f70be92` (the diff named in scope.md item 2 prints nothing): the prompt and its versions, the
  workspace key, the scrubber, the sidecar's own crate, everything under `pulse-app/src` (the argv, the grammar,
  the field allowlist, the grounded title), the cue kinds and labels, both manifests.
- The symbols the fifth section names still exist: `TRIGGER_LINE_PREFIX`, `cue_summary` and `render_payload` in
  `assembler.rs`; `select_corpus_matches` in `retrieval.rs` now takes `triggering_scope`.
- Pulse's wrap was running in its own tree during this phase (untracked run dir and report, three modified
  bookkeeping files); HEAD stayed `9bfefb8` at every read. Its commit is expected on top and is checked by diff
  before the build.

## The binaries on this host
- On disk in Pulse's `target/release`: `pulse-app` `df167647…ba4a` (built 2026-10-07 09:40 local) and
  `andromeda-pulse-mcp` `6175fc36…d2a9` (09:41), equal to the fifth series' recorded rebuilt digests. They are the
  `f70be92` builds.
- **No named content token discriminates an `f70be92` build from a `9bfefb8` one.** The change adds no string
  literal to non-test code, and the release profile sets `strip = true`, so `nm` lists 0 symbols on both binaries
  (`nm pulse-app | grep -c .`: 0; the same on the sidecar). Both binaries are stripped PIE executables (`file`).
- What can be measured on both builds: the sha256 (old side known now), the mtime against the commit time, and
  the fifth series' four strings as a lineage reading (they read 1, 1, 0, 0 on the `f70be92` `pulse-app` and
  discriminate nothing between these two builds).
- Rebuild determinism on this host has one data point: the sidecar rebuilt at `f70be92` over a recompiled
  dependency hashed identically to its `5f77859` build (the fifth series' ledger, `:62`, `:80`). No same-source
  rebuild of `pulse-app` has been compared.
- The series has no in-drive witness of the remedy either. The capture's `creating digest corpus retrieval rows:`
  line prints `row_count_returned`, which Pulse logs as the CANDIDATE count before the selection
  (`assembler.rs` at `9bfefb8`, the `tracing::info!` after `select_corpus_matches`), and the prompt is not
  recorded in this series. So the binary proof is the only evidence that the remedy is in the launched binary.

## Graph impact (rust plane; `tree-query` trace in the phase run dir, 21 rows)
- **`v3_09_met`** (`real_model_grading/mod.rs:140`) — five calling verdict tests, at `series_2026_09_29.rs:124`,
  `series_2026_09_30.rs:119`, `series_2026_10_01.rs:145`, `series_2026_10_06.rs:131`,
  `series_2026_10_07.rs:131`. The new verdict test is a sixth caller; no signature changes.
- **`pre_registered`** (`real_model_grading/mod.rs:154`) — five calling tests, at `series_2026_09_30.rs:23`,
  `series_2026_10_01.rs:25`, `series_2026_10_06.rs:25`, `series_2026_10_07.rs:25`,
  `capture_run_2026_10_07.rs:26`. The new digest test is a sixth.
- **`contract_section`** (`real_model_grading/mod.rs:145`) — one caller, `pre_registered` at `:156`.
- The other ten rows are the modules' `use` lines. No ref test exists in code
  (`grep -rn 'v3_09_ref\|fn v3_09_is_met' crates/conductor-run/tests`: 0 hits): every series so far read NOT MET,
  so the ref test's shape comes from the acceptance and the fifth plan's wording, not from a precedent.

## Patterns detected
- **One child module per dated series** (`real_model_grading/series_2026_10_07.rs:1-134`): `use crate::*;`, a
  `Series` constant, seven tests, the verdict test last. A child sees the harvest's private rule items, so the
  rule's bytes never move.
- **Registration at four sites** (`real_model_grading/mod.rs:11-18`; `real_model_harvest.rs:62-71`, `:1256`,
  `:1309-1342`): a module missing from `GRADING_MODULES` is caught by the directory arm.
- **The capture run's label form** (`real_model_harvest.rs:1337`): `format!("2026-10-07 capture {}", …)` — a
  second record of one date takes a kind word beside the date.

## Conventions to follow
- **Add-only contract section** opening with a dated `[added …]` note, each term its own bolded clause in the
  prior section's order (`contracts/pulse-real-model-leg-posture.md:574-685`).
- **Pins hold digests, never text** (`real_model_series/mod.rs:124-144`); the workspace key constant is the leaf
  (`series_2026_10_07.rs:43`).
- **The `measured` table is written from what the rule measures** — red before green, the grades read from the
  arm's own failure output (the fifth plan's step 14).
- **The ledger names handles, the leaf, digests, counts and booleans, never a host path** (the fifth ledger,
  `:6`).

## Sweeps
- `pulse-real-model-leg-posture` over `crates`, `scripts`, `.github` (`grep -rln`): 9 hits · 3 changed
  (`real_model_series/mod.rs`, `real_model_grading/mod.rs`, `real_model_harvest.rs`, for the new series'
  registration) · 6 no-change (doc-comment citations of sections this chunk does not edit, in `drift.rs`,
  `run_contract.rs`, `canary.rs`, `real_model_live.rs`, `agent-run.sh`, `agent-run.ps1`).
- The two code readers of the contract (`real_model_grading/mod.rs:155`, `:167`) read one section by heading, so
  an added section moves no existing digest.

## Counts at the chunk base (`902d12c`)
- `Cargo.lock` packages: 562 (`grep -c '^name = ' Cargo.lock`).
- Harvest tests: 125 (`cargo nextest list -p conductor-run --test real_model_harvest | grep -c real_model_harvest`).
- Committed captures: 23 (`capture_population.rs:9`). Grading modules: 13 (`real_model_harvest.rs:1256`).
- Tree-walker marks: 9 files (`grep -rl --include='*.rs' 'andromeda:walks-tree' crates | wc -l`); none is owed
  here.
- `cargo fmt --all --check` exits 0.
- NVIDIA kernel-module major 610, userspace major 610.
- `~/.cache/pulse-legs/` holds three leaves: `rm-trigger-series`, `rm-fifth-series`, `rm-recorded-run`.
- Architecture registries: §Occupied Resources 38114 B, §Established Decisions 38068 B, threshold 38115 B.

## Scope premise closure
- Verified, tag dropped: the section's name collision; the stop rule and the outcomes it hands the wrap (as
  relayed); the stated limit's basis; the model; the prompts, argv and grammar; the corpus selection; the on-disk
  builds; the sidecar's link to `crates/triage`; the covariate; the relay's selection and lineage claims.
- Corrected in scope.md:
  - the fifth section pins symbols, not line numbers; what lapses at `9bfefb8` is its "byte-unchanged under
    `crates/triage`" clause;
  - no named content token proves a `9bfefb8` build, so the acceptance's string-set term cannot be met as
    written — a plan fork;
  - the relay's "11 of 20" for the second prompt is the miss count: 9 of 20 named the right service.
- Not verified and not relied on: Pulse's doc-comment sentence that another service's corpus lines led the model
  to misplace the signal. Pulse's own record states what its counts do not show.
- One scope edit beyond the tags: the folded CARRY's quoted rank-1 statement was replaced by a description,
  because it is model text read from the corpus and scope.md is outside any `evidence/` tree.

## New files to create
- `conductor-0.3.0/chunks/2026-10-07-a-sixth-pre-registered-real-model-series-for-v3-09/evidence/` — the attempt ledger, the three captures, the round listings
- `crates/conductor-run/tests/real_model_grading/series_2026_10_07_sixth.rs` — the series' child module

## Files to modify
- `contracts/pulse-real-model-leg-posture.md` — one add-only section before the quiet-window section
- `crates/conductor-run/tests/real_model_series/mod.rs` — the series' evidence path and three digest pins
- `crates/conductor-run/tests/real_model_grading/mod.rs` — one module declaration
- `crates/conductor-run/tests/real_model_harvest.rs` — the registration sites, never between the rule markers
- `crates/conductor-run/tests/real_model_grading/capture_population.rs` — the capture count

## Open questions
- none — the two plan-decision questions research raised were asked at P4 and answered by the operator
  (inputs#I14): the `pulse-app` proof is the moved digest with build provenance, and the `717bbf3` CI row is taken
  up recorded unowned.
