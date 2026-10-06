# Codebase Research — 2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09

## Scope
- **Depth:** deep · **Reads:** 24 · **Globs/Greps:** 31
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, Session Additions included
  (auto-loaded whole when the `crates/**/tests/**` reads began); applied: 2026-08-19 (a) no `boot` before a drive,
  2026-08-16 quiet window, 2026-08-18 one `pulse-legs` parent and a letters-only leaf, 2026-08-20 a ~0 s `[BLOCKED]`
  is sidecar resolution, 2026-09-02 a census before and after and a stop form beside the firing form, 2026-09-10
  live-leg atoms from a printed output, 2026-09-23 as corrected 2026-10-01 (`interpretation.incident.skipped`),
  2026-09-30 one harness-defect re-fire, 2026-10-03 `WEBKIT_DISABLE_DMABUF_RENDERER=1` on this host.
  `.claude/rules/testing.md` — read in full (same load); applied: 2026-10-04 child modules under `tests/<name>/`,
  2026-09-07 a bare positional is a nextest FILTER (use `--test`), 2026-06-21 as extended 2026-10-03 (compile every
  feature-gated target). `.claude/rules/host-win32.md` and `.claude/rules/security.md` load unconditionally.
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:**
  - `inputs#I1` — the overseer relay for this phase
  - `inputs#I2` — the phase directive
  - `inputs#I3` — Pulse's master route at `5f77859`
  - `inputs#I4` — Pulse `assembler.rs` (the `TRIGGER: ` line)
  - `inputs#I5` — Pulse `schema.rs` (prompt versions)
  - `inputs#I6` — Pulse `llamacli_inference.rs` (argv, sampling constants, the embedded grammar)
  - `inputs#I7` — Pulse's rank-1 chunk report
  - `inputs#I8` — the leg env script
  - `inputs#I9` — Pulse `prompt.rs` (`TRIGGER_FRAMING_INSTRUCTION`)
  - `inputs#I10` — Pulse `triage/src/contract.rs` (`cue_cause_label`)
  - `inputs#I11` — Pulse `inference_runtime.rs` (the grounded title)
  - `inputs#I12` — Pulse `l4-output.gbnf`
  - `inputs#I13` — Pulse `workspace-detector/src/contract.rs` (`workspace_key`)
  - `inputs#I14` — Pulse `markdown.rs` (the report render)
  - `inputs#I15` — Pulse `triage/build.rs` (the tokenizer source)
  - `inputs#I16` — Pulse `observability.rs` (the field allowlist)
  - `inputs#I17` — the overseer's answers to the P4 fork round (the home-rooted dir, the mask fix after the drives,
    the build and launch authority, the night rule)
  - `inputs#I18` — the overseer's first P5 review (leans ratified; no host rendering lever in the launch)
  - `inputs#I19` — Pulse `render_posture.rs`: the app sets `__NV_DISABLE_EXPLICIT_SYNC` itself on Linux
    (`:16`, `:50-59`; called at `main.rs:279`; landed 2026-10-04, commit `2099998`). Read at the P5 review, so the
    harness rule's 2026-10-03 WebKit lever and the re-round pattern below describe a build BEFORE that fix.

## Files inspected
- `contracts/pulse-real-model-leg-posture.md` (full, 501 lines) — the three series sections (`:213`, `:322`, `:378`)
  are the shape of the new one; the stated limit sits at `:146-152`; the next `## ` after the 2026-10-01 section is
  `## The quiet window and serialization` (`:461`), so the new section lands between them.
- `crates/conductor-run/tests/real_model_grading/series_2026_10_01.rs` (full, 148 lines) — the template for the new
  series module: the `Series` value, the pre-registration digest test, pins, rule-recorded, no-fingerprint-no-key,
  the `measured_*` table, the eleven-key envelope arm, the verdict test.
- `crates/conductor-run/tests/real_model_grading/mod.rs` (full, 170 lines) — `Series`, `v3_09_met` (`:137`),
  `contract_section` (`:142`), `pre_registered` (`:151`, reads the ledger line `pre-registration sha256: `); the
  series modules are declared at `:13-15`.
- `crates/conductor-run/tests/real_model_series/mod.rs` (full, 97 lines) — `Drive`, the `EVIDENCE_*` and `SERIES_*`
  constants; labels, file names and digests only.
- `crates/conductor-run/tests/real_model_harvest.rs` (`:60-230`, `:1262-1391`) — the rule sits between
  `// ---- rule: begin ----` (`:70`) and `// ---- rule: end ----` (`:458`); `identifies_cause` (`:190`) is
  `names_conductor` AND `names_retry`; the series imports (`:64-67`), `GRADING_MODULES` (through `:1290`),
  `graded_captures` (`:1293`) and `the_capture_text_arm_scans_every_grading_module` (`:1358`, an exact-set equality
  between the directory and the list) each need the new series.
- `crates/conductor-run/tests/real_model_grading/capture_population.rs` (full, 68 lines) — `COMMITTED_CAPTURES = 14`
  (`:9`), equal to `ls conductor-0.3.0/chunks/*/evidence/rm-capture*.txt | wc -l` = 14 at HEAD.
- `crates/conductor-run/tests/real_model_grading/scrub_and_sweep.rs` (full, 84 lines) — the mask's three arms; its
  forms are BUILT, never spelled (`:6`).
- `crates/conductor-run/tests/real_model_common/mod.rs` (`:196-375`) — `mask_workspace_key` (`:227`),
  `workspace_rendering` (`:273`), `mask_host_paths` (`:312`), `host_path_starts_at` (`:330`).
- `crates/conductor-run/tests/real_model_live.rs` (`:140-165`, `:536-760`) — the capture's emit chain and
  `print_pulse_witnesses`; nothing in it needs an edit for `5f77859` (below).
- `crates/conductor-run/tests/capture_paths/mod.rs` (full, 45 lines) — `pulse_logs_dir_from` canonicalizes the value
  and requires a directory; no drive-letter or host assumption.
- `scripts/agent-run.sh` (`:60-211`) — `live_leg` (`:79`) prints `[live] leg ${label}: ${scenario}` (`:81`) and
  `[live] leg ${label}: froze … self-obs lines` (`:86`); `live_real_model_leg` (`:164`) is the arm every drive runs.
- `scenarios/real-model-interpretation.toml` (full) — seed 4317033, three 12x bursts on service `conductor`.
- `crates/conductor-run/src/execute.rs` (`:78-100`) — the per-execution span salt.
- `crates/conductor-core/src/redact.rs` (`:74-136`, by grep) — `redact_value`'s token anchors.
- The 2026-10-01 series' `evidence/attempt-ledger.md` (full) and `plan.md` (`## Test Commands`) — the ledger shape and
  the fence precedent; the 2026-10-03 re-round's `evidence/round-ledger.md` (full) and its fence (by grep) — the one
  live leg already driven on this host.
- Pulse at `5f77859`, each by `git show`: the files of `inputs#I4`-`I7` and `I9`-`I16`.

## Graph impact (from the code-graph query; rust plane, trace `tree-query-{marker}.json`)
- **`v3_09_met`** (`real_model_grading/mod.rs:136`, 0-indexed) — called by the three series verdict tests
  (`series_2026_09_29.rs:123`, `series_2026_09_30.rs:118`, `series_2026_10_01.rs:144`; the second query of the trace,
  28 rows in all); the new verdict test is one more caller. The signature does not change.
- **`pre_registered`** — called by the two digest-registered series (`series_2026_09_30.rs:22`,
  `series_2026_10_01.rs:24`). **`contract_section`**, **`committed_captures`** — indexed in the test tree (the first
  query, 7 rows). No signature changes.
- **`mask_host_paths`** — defined in `real_model_common/mod.rs`. The graph names five calling tests: three in
  `scrub_and_sweep.rs` (`:27`, `:39`, `:53`) and two in `workspace_mask.rs` (`:78`, `:91` — the second is
  `a_path_valued_workspace_leaks_neither_its_key_nor_its_path`, which exercises only the Windows long-path form). The
  sixth caller, `real_model_live.rs` `emit_block` (`:158-164`), is in the `live-pulse` target the graph does not
  index (found by grep). Its signature does not change under the temp-root extension; only its match set grows.
- **`rekey_trace_identity`** (`conductor-emit/src/identity.rs:26`) — read only, not changed.
- Companion sweep by NAME (`grep -rn -E '2026_10_01|2026-10-01-interpretation-re-proven' crates --include=*.rs`):
  hits in exactly four files — `real_model_harvest.rs`, `real_model_series/mod.rs`, `real_model_grading/mod.rs`,
  `real_model_grading/series_2026_10_01.rs`. Each site is one the new series mirrors; the 2026-10-01 lines themselves
  take no change.

## Patterns detected
- **A dated series is one child module plus four registrations** (`series_2026_10_01.rs`; `real_model_series/mod.rs:76-97`;
  `real_model_grading/mod.rs:15`; `real_model_harvest.rs:64-67`, `:1278-1281`, `:1301-1306`). Missing the
  `GRADING_MODULES` row fails `the_capture_text_arm_scans_every_grading_module` by exact-set equality.
- **Pre-registration is a ledger line plus a digest pin** (`real_model_grading/mod.rs:151-170`): the test reads the
  FIRST line of the ledger starting `pre-registration sha256: ` and checks the section's digest (heading line to the
  next `\n## `) against the pin.
- **The rule is byte-pinned per drive** (`Series::drives_recorded_the_current_rule`, `mod.rs:58`): each capture opens
  with the rule section as the harness recorded it before the leg, and every prior series asserts that record EQUAL
  to the current rule. An edit between the markers fails all four prior series' arms.
- **The capture's scrub chain** (`real_model_live.rs:158-164`): `mask_workspace_key` → `redact_value` →
  `mask_host_paths` → `elide_fingerprints`, buffered and printed once.
- **The live round on this host** (2026-10-03 `round-ledger.md`): `pulse-app` launched by path from a cwd outside this
  repo with ONE env block echoed back, `WEBKIT_DISABLE_DMABUF_RENDERER=1` in it; the census is `ps` over the pattern
  `pulse-app|andromeda-pulse|conductor|WebKit`; teardown is `kill -TERM` by the recorded PID (it exited within 2 s);
  Pulse's log is `logs/agent-latest.jsonl.<date>` under the data dir, the same name the capture's `pulse_log()` globs.

## Conventions to follow
- **A new series edits nothing between the rule markers** (`real_model_harvest.rs:70`-`:458`).
- **No capture text in test source**: pins are labels, file names and 64-hex digests (`real_model_series/mod.rs:1-5`).
- **A mask form in test source is BUILT, never spelled** (`scrub_and_sweep.rs:6`, `:8-10`): the temp-root arms format
  their roots from parts, as the named-root arm does at `:36-40`.
- **Evidence names no host path**: the ledger records handle NAMES, a leaf, a digest or a boolean
  (security-history 2026-09-04, 2026-09-07). `gate.py hygiene` P1 also refuses a temp-dir path in `evidence/`
  (`gate-contract.md` §Hygiene, the `tmp` form), so the ledger never spells the data dir's parent.
- **A Pulse build's raw output stays out of evidence** (it names host paths): exit and the `Finished` line only
  (the 2026-10-01 ledger `:35`).

## Pulse at `5f77859` (each a transcribed SUT record, read with `git show`)
- **Head and block.** `5f77859f8ebbb18fe01f6394a6f94bfc66b9ef34`, branch `chore/migrate-pulse-to-v3`, 0 ahead and 0
  behind its upstream (`git rev-list --count`, both directions, read 2026-10-06T19:26Z); the master route holds
  `2026-10-04-retry-storm-interpretation-names-its-cause · complete` (`inputs#I3`).
- **The digest.** `render_payload` (`assembler.rs:629`) writes, in order: `WINDOW`, `PROJECT`, `RECENT CHANGES`,
  `OVERALL: {…}` (`:673`), then — when a cue exists — `TRIGGER: {cue_cause_label(first cue)}` (`:680`), `SERVICES`
  (`:685`), `ATTENTION CUES:` (`:697`), `CORPUS MATCHES:` (`:708`) with a framing note (`inputs#I4`).
  `cue_cause_label(RetryStorm)` is `Retry storm` (`contract.rs:211`, `inputs#I10`). So the scenario's digest reads
  `TRIGGER: Retry storm` — the cause label alone, with NO scope — while the cue line still reads
  `[autonomous] retry_storm — retry_storm scope_id=conductor` (`cue_summary`, `assembler.rs:618-623`) and the
  `SERVICES` rows name `conductor`.
- **The prompt.** `TRIGGER_FRAMING_INSTRUCTION` (`prompt.rs:90-98`, `inputs#I9`) is appended to every tier's prompt
  (`:272`, `:365`, `:456`): "the first hypothesis statement must name that signal in the TRIGGER line's own words".
  Versions `v2.5` / `v1.4-fallback` / `v1.4-reflection` (`schema.rs:34`, `:42`, `:55`, `inputs#I5`).
- **What this does to the rule's two facets.** The retry token now reaches the model twice (the TRIGGER line and the
  cue line) under an instruction to put it in rank 1. The `conductor` token reaches it on the cue line and the
  SERVICES rows, under no instruction to name it. The rule needs both.
- **The title, not the hypotheses, is grounded.** `grounded_output` (`inference_runtime.rs`, `inputs#I11`) rewrites
  only `title` to `{cue cause label}: {model title}`; `hypotheses` are stored as the model wrote them. The rule reads
  the rank-1 STATEMENT, so the grounding cannot satisfy it.
- **The render and the struct.** `crates/interpretation/src/markdown.rs` and `schema.json` are byte-unchanged from
  `a2addb3` (`git diff --stat a2addb3 5f77859 -- <those paths>` prints nothing); `schema.rs` moved only its three
  version constants and their comments. `L4Output` keeps its fourteen fields (`schema.rs:170-185`). The rank-1
  extraction needs no change.
- **The grammar.** Generation is constrained by `--grammar-file` (`llamacli_inference.rs:489`); the grammar is
  embedded (`include_str!("l4-output.gbnf")`, `:134`) and written to a per-call temp file, so the launch cwd does not
  matter to it. It admits a `hypotheses` array of 0 to 5 items (`inputs#I12`), so an empty array — the harvest's
  `NoRankedHypotheses` — is still a legal output.
- **argv constants** (`inputs#I6`): `LLAMA_CLI_CTX_SIZE` 8192 (`:114`), `LLAMA_CLI_REASONING` `off` (`:120`),
  `--temp` `1.0` (`:124`), `--top-p` `0.95` (`:126`), `--top-k` `64` (`:128`), `--min-p` `0` (`:130`).
- **The model file.** `gemma-4-E4B-it-Q4_K_M.gguf`, 4 977 171 584 B, sha256
  `85a896a047553e842f25297ee5b031d64ff30147d9c4af17b1e4b394cd1fab87` (`sha256sum`, measured 2026-10-06 at take-up),
  matching the relay's `85a896a0…ab87`.
- **Pulse's own readings** (`inputs#I7`): `Slot 2 FAIL · rank1 34/40` (`:113`) against "the pre-registered bar of 36"
  (`:93`), on the baseline Llama 3.2 3B (`:182`). Re-derived: `git grep -n -i -E 'E4B|gemma' 5f77859 --
  'andromeda-pulse-0.3.0/chunks/2026-10-05-*/evidence/*.md' '…/report.md'` filtered for `/40|retry|rank-1|rank1|storm`
  returns 4 lines, all naming gemma-4-E2B (40/40, record-only) and none naming E4B. No Pulse artifact reads the
  shipped model on retry naming.
- **The log surface.** Every target the capture reads exists at `5f77859` (`git grep -l -F` per quoted target over
  `crates` and `pulse-app/src`: 13 of 13 found), and the fields it prints are in `observability.rs`'s allowlist
  (`inputs#I16`: `skip_reason`, `decision`, `severity`, `digest_kind`, `prompt_version`, `inference_mode`,
  `model_identity`, `workspace_root_basename`, `parse_outcome`, `cue_present` — each ≥ 1 occurrence). A field can
  still read redacted live; d1's capture is where that is measured.
- **The workspace key** is the detected root, else the data dir — a PATH (`workspace_key`, `:77-82`, `inputs#I13`);
  `crates/workspace-detector` is byte-unchanged from `a2addb3`.
- **The MCP tool set.** `ALL_TOOL_NAMES` holds nine, among them the five `contracts/mcp-contract.toml:11-17` requires.
- **The scrubber** moved one line (`sum % 10 == 0` → `sum.is_multiple_of(10)`), no behaviour change.

## The binaries on this host
- `target/release/pulse-app` is dated 2026-10-04 00:12:33 +02:00 and `andromeda-pulse-mcp` 2026-10-03 23:52:23 +02:00;
  `5f77859` was committed 2026-10-05 18:22:00 +02:00. Both binaries PREDATE the series' HEAD: a rebuild of both is
  owed before d1.
- **Content pair for `pulse-app`.** NEW at `5f77859`: `--grammar-file` (0 files at `a2addb3`, 1 at `5f77859`, by
  `git grep -l -F`). RETIRED: `--json-schema-file` as an argv element (`llamacli_inference.rs:430@a2addb3`; at
  `5f77859` the string survives only in a `//!` doc comment, `:18`, which no binary carries). Control, measured on the
  stale binary: `grep -a -c -F` reads `--grammar-file` **0** and `--json-schema-file` **1** — the pair fails a stale
  build in both halves.
- **The sidecar has no discriminating string.** The stale sidecar already carries `retrieve_incident_events` (1), and
  reads 0 for `--grammar-file`, for the corpus framing note and for the trigger-framing words. So the sidecar's proof
  is build provenance — clean inputs, the build command, an mtime after the commit, a sha256 different from the stale
  build's — with the content reading recorded first, as the 2026-10-01 series did.
- **Build env.** `crates/triage/build.rs` (`inputs#I15`) copies a local tokenizer when
  `ANDROMEDA_LLAMA3_TOKENIZER_PATH` is set and otherwise downloads one; the leg env script sets that handle
  (`inputs#I8`). Building with the script sourced keeps the build off the network.
- Pulse's build inputs are clean at take-up (`git status --short` there: 3 modified + 1 untracked, all under
  `.andromeda/` or `.claude/`).

## The host
- **GPU.** Kernel module `610.57.04` (`/proc/driver/nvidia/version`), userspace `nvidia-utils 610.57.04-1`
  (`pacman -Q`), `nvidia-smi` answering with driver `610.57.04` on an RTX 3090, 1 232 MiB of 24 576 MiB used. The
  majors agree now; the check is re-read before d1 because a package upgrade without a reboot splits them.
- **llama-cli** is present and executable at the path the leg env names (`inputs#I8`).
- **Ports and processes at take-up.** No listener on `:4317` or `:4318`; no `pulse-app`, sidecar or `llama-cli`
  process (the pulse-builder terminal is open and idle).
- **The data dir's parent.** The re-round's `/tmp/pulse-legs` no longer exists (the temp dir is cleared at boot).
  More to the point, the capture chain does not mask a temp-rooted path:
  - Pulse stamps the key as a path, and the report's `## Previously Seen` entries carry it as a `({workspace})`
    suffix (`mask_workspace_key`'s own doc, `:219-226`).
  - `mask_workspace_key` replaces the LEAF only, leaving `(<parent>/<workspace-key>)`.
  - `redact_value` masks a whitespace token holding `/home/`, `/Users/`, `/root/`, `~/` or a drive form
    (`redact.rs:130-136`) — which is why the Windows d3 capture reads `<redacted>` there
    (`rm-capture-d3.txt:447`) — and `mask_host_paths` names `/home/`, `/Users/` and `%APPDATA%` (`:346`).
  - Neither names `/tmp/` or `/var/tmp/`. So a home-rooted parent is masked at two stages and a temp-rooted one at
    none, and `gate.py hygiene` would then refuse the committed capture.
  - `grep -c '/tmp/'` over the 14 committed captures: 0 in every one, so extending the mask moves no committed file.
- **Supply chain.** `git -C "${CARGO_HOME:-$HOME/.cargo}/advisory-db" status --porcelain` prints 0 lines;
  `cargo-audit 0.22.2`, `cargo-deny 0.20.2`; `grep -c '^name = ' Cargo.lock` = 562.

## Walks-tree candidates (read by grep for `read_dir|ls-files`; implement confirms each before marking)
- Direct walkers: `crates/conductor-core/tests/secret_scan_gate.rs` (`git ls-files`, `:183`),
  `crates/conductor-core/tests/workflow_env_gate.rs` (`read_dir`, `:141`),
  `crates/conductor-report/tests/matrix_ledger_gate.rs` (`read_dir` of the repo root, `:37`),
  `crates/conductor-run/tests/real_model_grading/capture_population.rs` (`read_dir` over `chunks/*/evidence`, `:16`).
- Walkers through the catalog loader (no `read_dir` of their own; they read every scenario file the loader lists):
  `crates/conductor-core/tests/scenario_audit_gate.rs` (`catalog()`, `:31`), the inline tests of
  `crates/conductor-core/src/drift.rs` (`:357`) and `crates/conductor-core/src/load_envelope.rs` (`:453-462`).
- Not named by the relay, found by the same grep: `real_model_harvest.rs:1360` lists the `real_model_grading/`
  directory (names only; its content reads are compile-time `include_str!`). `journal_conformance.rs`, `cli_smoke.rs`,
  `real_model_live.rs` and `span_landing_live.rs` walk a runs dir, a temp dir or Pulse's logs, never the tree.
- The token's reader is `gate.py run`'s header (`gate-contract.md` §Tool, the walk-class line): a marked file whose
  extension is a code language's is counted, and implement's delta rule reads the line to void a `defer`.

## New files to create
- `conductor-0.3.0/chunks/2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09/evidence/`
- `crates/conductor-run/tests/real_model_grading/series_2026_10_06.rs` — the series block

## Files to modify
- `contracts/pulse-real-model-leg-posture.md` — the new section, add-only
- `crates/conductor-run/tests/real_model_series/mod.rs` — the evidence path and the drive pins
- `crates/conductor-run/tests/real_model_grading/mod.rs` — the module declaration
- `crates/conductor-run/tests/real_model_harvest.rs` — imports, the module list, the graded-capture loop, a walks-tree mark
- `crates/conductor-run/tests/real_model_grading/capture_population.rs` — the population count, a walks-tree mark
- `crates/conductor-run/tests/real_model_common/mod.rs` — the temp-rooted forms in the host-path mask
- `crates/conductor-run/tests/real_model_grading/scrub_and_sweep.rs` — the temp-root arms
- `crates/conductor-run/tests/real_model_grading/workspace_mask.rs` — the POSIX path-valued workspace arm
- `crates/conductor-core/tests/secret_scan_gate.rs` — a walks-tree mark
- `crates/conductor-core/tests/workflow_env_gate.rs` — a walks-tree mark
- `crates/conductor-core/tests/scenario_audit_gate.rs` — a walks-tree mark
- `crates/conductor-report/tests/matrix_ledger_gate.rs` — a walks-tree mark
- `crates/conductor-core/src/drift.rs` — a walks-tree mark
- `crates/conductor-core/src/load_envelope.rs` — a walks-tree mark

## Open questions
- Where does the series' data dir live on this host — home-rooted (masked by the committed chain) or temp-rooted
  (needs the mask extension before d1)? → blocks: plan-decision — ANSWERED at P4 (`inputs#I17`): home-rooted; the
  temp-rooted gap is fixed in this chunk after the drives.
- Does the overseer's grant cover the two Pulse release builds, and who launches `pulse-app`? → blocks: plan-decision
  — ANSWERED at P4 (`inputs#I17`): the grant covers both builds and the launch; the agent does both.
- Which of the eight walks-tree candidates truly walk the tree (the three catalog-loader readers and the
  directory-listing arm are the uncertain ones)? → blocks: implementation-scope
