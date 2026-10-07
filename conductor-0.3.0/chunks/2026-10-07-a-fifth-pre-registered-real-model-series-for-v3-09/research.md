# Codebase Research — 2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09

## Scope
- **Depth:** deep · **Reads:** 27 · **Globs/Greps:** 34
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full by structural extraction (75
  lines, 82 516 B; lines 1-46, 47-57 and 58-75 in three offset reads; 36 Session Additions). Applied:
  - 2026-08-19 (a): no `boot` before a drive;
  - 2026-08-16 as extended 2026-09-04: one launch serves several legs, so the inter-leg hygiene is the quiet window;
  - 2026-08-18: one `pulse-legs` parent and a letters-only leaf;
  - 2026-08-20: a `[BLOCKED]` in about 0 s is sidecar resolution;
  - 2026-09-02: a census before and after, a stop form beside the firing form, the pattern derived from what the
    leg can start;
  - 2026-09-10: a live leg's atoms come from a printed output;
  - 2026-09-23 as corrected 2026-10-01: a no-incident outcome is read from `interpretation.incident.skipped`;
  - 2026-09-30: one harness-defect re-fire on the operator's word;
  - 2026-10-03 as corrected 2026-10-06: no WebKit lever on a build at or after Pulse `2099998`; keep the 10 s
    liveness check;
  - 2026-10-06: a two-sided control on both binaries before a content proof is pre-registered, and a long embedded
    text over a short literal.
  `.claude/rules/testing.md` loaded whole on the first `crates/**/tests/**` read; applied: 2026-10-04 (child modules
  under `tests/<name>/`), 2026-09-07 (a bare positional is a nextest FILTER; use `--test`).
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:**
  - `inputs#I1` — the overseer relay for this phase
  - `inputs#I2` — the phase directive
  - `inputs#I3` — Pulse's master route at `f70be92` (the fix chunk reads `complete`, `:102`)
  - `inputs#I4` — Pulse `prompt.rs` (`TRIGGER_FRAMING_INSTRUCTION` and its three append sites)
  - `inputs#I5` — Pulse `schema.rs` (prompt versions)
  - `inputs#I6` — Pulse `assembler.rs` (the `TRIGGER: ` line, `cue_summary`)
  - `inputs#I7` — the leg env script (four exported handles)
  - `inputs#I8` — Pulse's probe reading for the fix chunk
  - `inputs#I9` — Pulse's d3 replay record (not measured)
  - `inputs#I10` — Pulse `llamacli_inference.rs` (argv; byte-unchanged since `5f77859`)
  - `inputs#I11` — Pulse `l4-output.gbnf` (byte-unchanged since `5f77859`)
  - `inputs#I12` — Pulse `crates/mcp-server/Cargo.toml` (the sidecar links `interpretation`)
  - `inputs#I13` — Pulse `crates/security/src/scrubber.rs` (the keyed arms)
  - `inputs#I14` — Pulse `observability.rs` (the field allowlist; byte-unchanged since `5f77859`)
  - `inputs#I15` — Pulse `workspace-detector/src/contract.rs` (`workspace_key`; byte-unchanged since `5f77859`)
  - `inputs#I16` — Pulse `inference_runtime.rs` (the grounded title; byte-unchanged since `5f77859`)
  - `inputs#I17` — the overseer's P5 review answer (the four leans confirmed; the sidecar term replaced, not
    weakened; Pulse's red `boot smoke` on `f70be92` read as a runner flake; a death of the app on this host stops
    and is reported)

## Files inspected
- `contracts/pulse-real-model-leg-posture.md` (`:461-610`, and the heading index by grep) — §The 2026-10-06 series
  (`:461-570`) is the shape of the new one; the next `## ` after it is `## The quiet window and serialization`
  (`:572`), so the new section lands between them.
- `crates/conductor-run/tests/real_model_grading/series_2026_10_06.rs` (full, 134 lines) — the template for the new
  series module, test for test.
- `crates/conductor-run/tests/real_model_grading/mod.rs` (`:18-171`) — `Series` and its five arms (`:28-119`),
  `graded` (`:123`), `v3_09_met` (`:138`), `contract_section` (`:143`), `pre_registered` (`:152`); the series modules
  are declared at `:13-16`.
- `crates/conductor-run/tests/real_model_series/mod.rs` (`:96-120`, constants by grep) — `EVIDENCE_2026_10_06`
  (`:101`) and `SERIES_2026_10_06` (`:104`): labels, file names and digests only.
- `crates/conductor-run/tests/real_model_harvest.rs` (`:60-70`, `:218-232`, `:1253-1318`, markers by grep) — the rule
  sits between `// ---- rule: begin ----` (`:71`) and `// ---- rule: end ----` (`:459`); the series imports
  (`:62-69`), `GRADING_MODULES` (`:1253`, 11 rows), `graded_captures` (`:1298`, one loop per series);
  `real_model_witnessed` (`:218`) is `inference_mode` `real` AND no `det-` ref in `## Evidence`; `launch_cwd_clear`
  (`:229`).
- `crates/conductor-run/tests/real_model_grading/capture_population.rs` (by grep) — `COMMITTED_CAPTURES = 17` (`:9`),
  equal to `ls conductor-0.3.0/chunks/*/evidence/rm-capture*.txt | wc -l` = 17 at HEAD.
- `crates/conductor-tauri/src/commands.rs` (`:526-560`, tests by grep) — `fixture_scenarios_dir` (`:533`, doc
  comment `:530-532`) is the one helper through which the module's tests read the committed fixture dir.
- `crates/conductor-core/src/scenario_catalog.rs` (`:89-92`), `crates/conductor-core/src/drift.rs` (`:353-356`) — the
  mark's shipped form: one `// andromeda:walks-tree — {why}` line directly above the item's doc comment.
- `crates/conductor-run/src/execute.rs` (`:84-94`) — the per-execution span salt is in place.
- `scripts/agent-run.sh` (the `[live] leg` print sites by grep, `:81`, `:86`), `crates/conductor-cli/src/commands/
  preconditions.rs` (`:48`) — the atoms the round's entries assert.
- `.andromeda/architecture.md` (`:184`, and a handle-name census by grep) and `.andromeda/playbook.md` (`:96-125`,
  `:137-139`) — the posture-contract entry's clause, the env registry's members, the three `verdict: escalate`
  patterns and the routine rule for an external handle.
- The fourth series' chunk folder: `scope.md`, `research.md`, `plan.md` and `report.md` (each full), `evidence/
  attempt-ledger.md` (`:19-148`) — the plan and ledger this series repeats, and its measured timings.
- Pulse at `f70be92`, each by `git show` or `git diff`: the files of `inputs#I3`-`I6` and `I8`-`I16`.

## Graph impact (from the code-graph query; rust plane, trace `tree-query-{marker}.json`, 21 rows)
- **`v3_09_met`** (`real_model_grading/mod.rs:138`) — called by the four series verdict tests
  (`series_2026_09_29.rs:124`, `series_2026_09_30.rs:119`, `series_2026_10_01.rs:145`, `series_2026_10_06.rs:131`);
  the new verdict test is a fifth. No signature changes.
- **`pre_registered`** (`real_model_grading/mod.rs:152`) — called by three digest tests
  (`series_2026_09_30.rs:23`, `series_2026_10_01.rs:25`, `series_2026_10_06.rs:25`); the new one is a fourth.
- **`capture_2026_10_06`** — called from its own module (`:94`) and from `graded_captures`
  (`real_model_harvest.rs:1315`), imported at `:65`; the new accessor takes the same two call sites.
- **`fixture_scenarios_dir`** (`conductor-tauri/src/commands.rs:533`) — four calling tests:
  `the_committed_scenarios_fixture_stays_loadable` (`:560`), `resolve_selection_loads_the_whole_suite_for_the_sentinel`
  (`:632`), `resolve_selection_loads_exactly_one_for_a_named_scenario` (`:649`),
  `load_all_reads_every_committed_fixture_scenario` (`:667`). A mark on the helper covers all four; no code line moves.
- Companion sweep by NAME (`grep -rlE '2026_10_06|2026-10-06-a-fourth' crates --include=*.rs`): 4 hits · 3 changed
  (`real_model_harvest.rs`, `real_model_series/mod.rs`, `real_model_grading/mod.rs`, each gaining the new series
  beside the old lines) · 1 no-change (`series_2026_10_06.rs`, the template; its lines stay as they are).

## Patterns detected
- **A dated series is one child module plus four registrations** (`series_2026_10_06.rs`;
  `real_model_series/mod.rs:99-120`; `real_model_grading/mod.rs:16`; `real_model_harvest.rs:65`, `:67-68`,
  `:1283-1286`, `:1312-1317`). A missing `GRADING_MODULES` row fails the exact-set arm at `:1368`.
- **Pre-registration is a ledger line plus a digest pin** (`real_model_grading/mod.rs:152-171`): the test reads the
  FIRST ledger line starting `pre-registration sha256: ` and checks the section's digest against the pin.
- **The digest recipe is validated.** A scratchpad script in the `contract_section` form (the LF-normalized text
  from the heading line up to and including the newline before the next `\n## `) prints
  `00173912ffa684a4eebe120e45a9afb61c1766fff888bada4b5e713bb26f8097` (8663 bytes) for `## The 2026-10-06 series` and
  `0232afb1c302c49e408c92246ebfb6090c64c06ef81dea782a4af7656422e841` for `## The 2026-10-01 series`: the two digests
  those series' ledgers record.
- **The rule is byte-pinned per drive** (`Series::drives_recorded_the_current_rule`, `mod.rs:59`): an edit between the
  rule markers fails every prior series' arm.
- **The launch cwd is model input** (`real_model_harvest.rs:226-232`): `pulse-app`'s cwd basename reaches the prompt
  as `PROJECT:`, and the fourth series launched with the data dir as cwd, so the dir's leaf is read by the model.
- **The fourth series' timings** (its ledger, `:22-34`, `:139-143`): the two release builds 2 m 25 s and 2 m 24 s;
  each drive 362-363 s; d1's start to d3's end 25 m 02 s with two 180 s quiet windows.

## Conventions to follow
- **A new series edits nothing between the rule markers** (`real_model_harvest.rs:71`-`:459`).
- **No capture text in test source**: pins are labels, file names and 64-hex digests (`real_model_series/mod.rs`).
- **Evidence names no host path**: the ledger records handle NAMES, a leaf, a digest or a boolean; `gate.py hygiene`
  also refuses a temp-dir path in `evidence/`.
- **A Pulse build's raw output stays out of evidence** (it names host paths): the exit and the `Finished` line only.
- **A walks-tree mark is one comment line above the item's doc comment** (`scenario_catalog.rs:90`, `drift.rs:354`).

## Pulse at `f70be92` (each a transcribed SUT record, read with `git show`)
- **Head and block.** `f70be92c2ca13c951330efeffd725e62a484610c`, HEAD of `chore/migrate-pulse-to-v3`, 0 ahead of its
  upstream (`git rev-list --count '@{u}..HEAD'`, read 2026-10-07 at take-up). The code commit `1a2e509` is its
  ancestor and `git diff --name-only 1a2e509 f70be92` lists no path under `crates`, `pulse-app` or `xtask`.
- **What moved since `5f77859`.** `git diff --name-only 5f77859 f70be92 -- crates pulse-app xtask Cargo.toml
  Cargo.lock` lists five files: `crates/interpretation/src/prompt.rs`, `crates/interpretation/src/schema.rs`,
  `pulse-app/examples/l4_decision_probe.rs`, `pulse-app/tests/unit_inference_runtime.rs`,
  `pulse-app/ui/package-lock.json`. The same diff over `crates/triage`, `crates/mcp-server`,
  `crates/workspace-detector`, `crates/security`, `crates/corpus` and `pulse-app/src` prints 0 lines.
- **The instruction** (`prompt.rs:93-107`, the new sentence at `:104-107`, `inputs#I4`). It keeps the `v2.5` text and ends with one new sentence:
  "When the cue line under ATTENTION CUES carries a scope_id, the first hypothesis statement must name that scope_id
  value exactly as written there, and must not attribute the signal to anything else, including a service whose name
  merely contains it." It is appended to all three tiers' prompts (`:278`, `:371`, `:462`).
- **The cue line** reads `{kind label} scope_id={value}` when the cue has a scope (`cue_summary`,
  `assembler.rs:618-623`, `inputs#I6`), so the scenario's reads `retry_storm scope_id=conductor`. The `TRIGGER: ` line
  still carries the cause label alone (`:680-684`).
- **What this does to the rule's two facets.** Both are now instructed for rank 1: the retry token by the TRIGGER
  clause, the service by the new sentence. The canary's identity (`conductor-canary`) is exactly the "service whose
  name merely contains it" the sentence rules out, and it is what the fourth series' d3 named.
- **Versions** (`schema.rs:35`, `:44`, `:59`, `inputs#I5`): `v2.6` / `v1.5-fallback` / `v1.5-reflection`.
- **Unchanged, by the empty diffs above:** the argv (`inputs#I10`), the GBNF (`inputs#I11`), the grounded title
  (`inputs#I16`), the workspace key (`inputs#I15`), the field allowlist (`inputs#I14`), the report render, the skip
  line, the render posture and the MCP tool set. The fourth series read `prompt_version` and `model_identity`
  unredacted from Pulse's live log through that same allowlist.
- **The model file.** `gemma-4-E4B-it-Q4_K_M.gguf`, 4 977 171 584 B, sha256
  `85a896a047553e842f25297ee5b031d64ff30147d9c4af17b1e4b394cd1fab87` (`sha256sum`, measured 2026-10-07 at take-up),
  equal to the digest the 2026-10-06 section pins.
- **Pulse's own reading of the fix** (`inputs#I8`): 240 generations, graded by Conductor's `identifies_cause` rule.
  `arm shipped: bar MET · sibling both 20/20 (min 19) · ordinary both 40/40 (min 36)`; `arm ns: bar MET · sibling both
  20/20 (min 19) · ordinary both 38/40 (min 36)`. The baseline arm's two misses are `signal_only` on an ordinary
  shape; on the sibling shapes it reads 20 of 20. So the probe never reproduced d3's miss, and its PASS does not show
  the sentence fixes it. The replay of d3's own digest was not measured (`inputs#I9`).
- **The scrubber's keyed arms** (`scrubber.rs:296`, `:305`, `inputs#I13`): the `api_key` family and
  `password|passwd|secret|token`. The file is byte-unchanged since `5f77859`.

## The binaries on this host
- `target/release/pulse-app` is dated 2026-10-06 22:00 and hashes `f69be5bb8643a479…`; `andromeda-pulse-mcp` is dated
  22:02 and hashes `6175fc36be6577b1…`. Both digests equal the fourth series' recorded rebuilt binaries (its ledger,
  `:32`, `:34`), so they are the `5f77859` builds. `1a2e509` was committed 2026-10-07 08:16:59 +02:00. Both are
  rebuilt before d1.
- **Old-side control, measured on the `5f77859` binaries** (`grep -c -a -F -e`, one string per count):

  | String | `pulse-app` | sidecar |
  |---|---|---|
  | `whose name merely contains it` | 0 | 0 |
  | `must name that scope_id value exactly as` | 0 | 0 |
  | `the first hypothesis statement must name that signal` | 1 | 0 |
  | `hypotheses-item-statement-kv` | 2 | 0 |
  | `--json-schema-file` | 0 | 0 |
  | `v2.6` | 0 | 0 |
  | `v2.5` | 2 | 0 |
  | `v1.5-fallback` | 0 | 0 |
  | `v1.4-fallback` | 1 | 0 |
  | `v1.5-reflection` | 0 | 0 |
  | `v1.4-reflection` | 1 | 0 |

- **The fourth series' pair no longer discriminates:** it reads 2 and 0 on the old `pulse-app`, exactly what a new
  one must read.
- **The candidate pair for `pulse-app`.** Present: `must name that scope_id value exactly as` — 40 bytes inside one
  `const` string, the class the 2026-10-06 lesson says survives as a run of bytes (the `v2.5` clause of the same
  constant reads 1 on the old binary). Absent: `v1.4-fallback`, which reads 1 on the old binary and is retired at
  `f70be92`. The new side of both is unmeasured until the rebuild. `v2.6` and `v2.5` are not candidates: four bytes
  each, and `v2.5` reading 2 shows a short literal can match more than its own constant.
- **The sidecar has no discriminating string**: 0 for all eleven. It links `crates/interpretation`
  (`inputs#I12`, for the report render), so a build at `f70be92` recompiles it, but none of the moved text is in
  the binary. A rebuilt sidecar may therefore hash the same as the old one. Its proof is build provenance — clean
  inputs at `f70be92`, the build command, an mtime after the commit — with both digests recorded and no requirement
  that they differ.
- **Build env.** The leg env script sets the tokenizer handle (`inputs#I7`), which keeps `crates/triage/build.rs` off
  the network; the fourth series built with the script sourced.
- Pulse's build inputs are clean at take-up (`git status --short` there: 3 modified paths, all wrap bookkeeping).

## The host
- **GPU.** Kernel module `610.57.04` (`/proc/driver/nvidia/version`), `nvidia-smi` answering with driver `610.57.04`
  on an RTX 3090. The majors agree now; the check is re-read before d1.
- **Ports and processes at P3.** No listener on `:4317` or `:4318` (`ss -ltn | grep -cE ':4317|:4318'` = 0) and no
  `pulse-app`, sidecar, `llama-cli` or WebKit process (`ps` over that pattern: 0 rows).
- **The data dir's parent** `pulse-legs` under the user's cache dir holds one entry, `rm-trigger-series`, dated
  2026-10-06. A Pulse session read that dir on 2026-10-07 and did not write it (`inputs#I9`).
- **The Bash guard** (`.claude/settings.json`, landed at `29adafa`) blocks a top-level `cd` that would move the
  session cwd and passes a subshell `( cd DIR && … )`. Every Pulse-tree entry of the plan is a subshell.
- **Supply chain.** `git -C "${CARGO_HOME:-$HOME/.cargo}/advisory-db" status --porcelain` prints 0 lines;
  `grep -c '^name = ' Cargo.lock` = 562.
- **Formatter.** `cargo fmt --all --check` exits 0 at HEAD, so a marked file's diff should be one line.

## The env handles (scope item 9)
- **What the contract names** (`grep -oE '(ANDROMEDA|CONDUCTOR)_[A-Z0-9_]+' contracts/pulse-real-model-leg-posture.md
  | sort | uniq -c`): six handles — `ANDROMEDA_PULSE_DATA_DIR` 4, `ANDROMEDA_PULSE_MODEL_PATH` 3,
  `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` 3, `ANDROMEDA_PULSE_L4_DETERMINISTIC` 2, `ANDROMEDA_PULSE_MCP_ENABLED` 1,
  `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` 1. The two model handles sit at `:367`, `:442` and `:554`, one line
  in each of the last three series sections.
- **What architecture registers** (the same grep over `.andromeda/architecture.md`): four — the data dir, the L4
  flag, the MCP flag and the bootstrap seconds. Neither model handle occurs.
- **No Conductor reader or setter.** `grep -rn` for the four handle names the leg env script exports, over `crates`,
  `scripts`, `.github` and `contracts`: 3 hits, all in the posture contract. So the per-reader duty of security-plan
  §Input Validation does not attach, and neither handle is a run-contract term or a `conductor preconditions`
  subject.
- **The playbook rules this class routine** (`.andromeda/playbook.md:137-139`): an external handle Conductor neither
  sets nor reads is registered when a SHIPPED artifact names it, "a committed contract" among the examples; the
  entry states that Conductor neither sets nor reads it, which artifact names it, and whether it is a run-contract
  term or a probe subject. Every qualifying clause holds here. The three `verdict: escalate` patterns (`:97`,
  `:109`, `:124`: a reversed locked decision, an obs misattribution, a boundary widening) do not: nothing new
  crosses a boundary, since no code reads the handles.
- **The size bound.** `scripts/arch-registry-check.py measure --file .andromeda/architecture.md` reads §Occupied
  Resources at 38086 B of a 38115 B target (29 B spare) and §Established Decisions at 37980 B. Two new registry
  entries do not fit in 29 B, so the wrap's amendment must move dated detail to the sidecar as it lands them.
- The leg env script exports two more handles (`ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH`,
  `ANDROMEDA_LLAMA3_TOKENIZER_PATH`, `inputs#I7`). No committed Conductor artifact names either, so the rule's
  shipped-artifact clause excludes them; the new section keeps to the two the earlier sections name.

## New files to create
- `conductor-0.3.0/chunks/2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09/evidence/`
- `crates/conductor-run/tests/real_model_grading/series_2026_10_07.rs` — the series block

## Files to modify
- `contracts/pulse-real-model-leg-posture.md` — the new section, add-only
- `crates/conductor-run/tests/real_model_series/mod.rs` — the evidence path and the drive pins
- `crates/conductor-run/tests/real_model_grading/mod.rs` — the module declaration
- `crates/conductor-run/tests/real_model_harvest.rs` — imports, the module list, the graded-capture loop
- `crates/conductor-run/tests/real_model_grading/capture_population.rs` — the population count
- `crates/conductor-tauri/src/commands.rs` — the walks-tree mark, one comment line

## Open questions
- Does the overseer's grant cover the two Pulse release builds and the launch, with the agent doing both? → blocks:
  plan-decision — leaned yes (the relay asks for a control on an `f70be92` build while the directive keeps
  pulse-builder idle; the fourth series' answer). ANSWERED at the P5 review (`inputs#I17`): confirmed as written.
- Does the candidate pair hold on the rebuilt `pulse-app`? → blocks: implementation-scope — the new-side control is
  implement's first act after the build; a string that fails is replaced from the table above before the section is
  written.
