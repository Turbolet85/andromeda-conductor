# Report — 2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09

**Chunk:** a fourth pre-registered real-model series for `v3-09`, grading the shipped model's rank-1 hypothesis at Pulse `5f77859`
**Date:** 2026-10-06
**Commits:** `9df683d` chore(…): operator pre-CI commit, for the run this chunk's verdict reads (the only commit since `0b07b2c`)

**Headline.** The series ran as pre-registered. **`v3-09` is NOT MET** under the byte-identical rule: three graded
drives, d1 `Identified`, d2 `Identified`, d3 `NotIdentified`. Recorded, never replaced; no ref test; no fourth drive.
Beside that verdict, and never as a softening of it: 6 of 6 classified canary digests and 3 of 3 scenario digests
surfaced, and all three rank-1 statements carry the retry facet the trigger framing asks for.

## Changes (structured — detectors read this)

- **Files** (basis: `git diff --name-only 0b07b2c` less run dirs and the chunk folder; 15 source/contract files):
  - `contracts/pulse-real-model-leg-posture.md` — new section `## The 2026-10-06 series` (111 added, 0 deleted).
  - `crates/conductor-run/tests/real_model_series/mod.rs` — `EVIDENCE_2026_10_06`, `SERIES_2026_10_06` (three sha256 pins).
  - `crates/conductor-run/tests/real_model_grading/series_2026_10_06.rs` — NEW child module, 8 tests.
  - `crates/conductor-run/tests/real_model_grading/mod.rs` — the module declaration.
  - `crates/conductor-run/tests/real_model_harvest.rs` — imports, `GRADING_MODULES` 10 → 11 rows, a loop in
    `graded_captures`, one walks-tree mark. Nothing between the rule markers moved (rule diff vs `0b07b2c` empty).
  - `crates/conductor-run/tests/real_model_grading/capture_population.rs` — `COMMITTED_CAPTURES` 14 → 17, one mark.
  - `crates/conductor-run/tests/real_model_common/mod.rs` — `mask_host_paths`' named roots gain `/tmp/` and `/var/tmp/`.
  - `crates/conductor-run/tests/real_model_grading/scrub_and_sweep.rs`, `workspace_mask.rs` — the temp-root arms.
  - Walks-tree marks, one comment line each (numstat 1/0): `crates/conductor-core/tests/secret_scan_gate.rs`,
    `workflow_env_gate.rs`, `scenario_audit_gate.rs`, `crates/conductor-core/src/drift.rs`, `load_envelope.rs`,
    `scenario_catalog.rs`.
  - Chunk folder: `evidence/attempt-ledger.md`, `evidence/rm-capture-d1.txt` / `-d2.txt` / `-d3.txt`, three
    `evidence/round-*.txt` listings, `scope-record.md`, `inputs/` (I20-I24 added by implement and this wrap).
- **Symbols / APIs:** no production symbol, IPC method, endpoint, port, socket or env handle added or changed.
  Test-tree only: `series_2026_10_06` (`pub(crate)` child module), `capture_2026_10_06`, the two series constants.
  `mask_host_paths` keeps its signature and its six callers (five harvest arms and `real_model_live.rs` `emit_block`);
  only its match set grew.
- **Crates / modules:** none added or removed. One test child module added under `tests/real_model_grading/`.
- **Dependencies:** none. `grep -c "^name = " Cargo.lock` = 562, unchanged; no manifest touched.
- **Schema / config:**
  - Scrub shape: `mask_host_paths` now masks a path opening with `/tmp/` or `/var/tmp/`, beside `/home/`, `/Users/`
    and `%APPDATA%`, with the same match semantics (basis: `real_model_common/mod.rs` `host_path_starts_at`). No
    committed capture holds a temp-rooted path (`grep -c '/tmp/'` over the 17 files: 0 in each), so no pin moved.
  - Posture contract: a fourth dated series section, add-only. It carries ONE in-section `[corrected 2026-10-06 …]`
    note (the binary proof, below). It names no handle the contract did not already name and no host rendering lever.
  - No scenario, workflow, run-contract or capability-manifest change.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - Pre-registered real-model series in the posture contract: 3 → 4 dated sections (2026-09-29, 2026-09-30,
    2026-10-01, 2026-10-06). Stated in: test-plan `:260` (the §6 Real-model interpretation leg's series enumeration,
    which ends at the 2026-10-01 series), security-plan `:338` (names the 2026-09-30 series' re-elision record).
  - Posture contract's Pulse pin list: `e98d838`, `fcc31b2`, `a2addb3` → plus `5f77859` (2026-10-06). Stated in:
    architecture `:184` ("re-pinned `fcc31b2` 2026-09-30, `a2addb3` 2026-10-01").
  - Committed real-model captures: 14 → 17 (`ls conductor-0.3.0/chunks/*/evidence/rm-capture*.txt | wc -l`). No master
    states the literal (grep `COMMITTED_CAPTURES|14 captures|fourteen` over the seven and the registries: 0 hits).
  - `mask_host_paths` named roots: 3 → 5. Masters naming the function: security-plan `:121`, `:338` (2 hits each line
    group; neither enumerates the roots). `%APPDATA%` appears at architecture `:201`, security-plan `:120`, `:123`,
    obs-plan `:459`, `:523` — each about `redact_value` or the path handles, a DIFFERENT function; none is this mask.
  - `real_model_harvest` tests: 104 → 112 (7 series tests + 1 POSIX arm). Workspace nextest: 1212 run, 1212 passed.
    No master states either literal (grep `(109|110|112) tests`: 0 hits).
  - Tests marked `andromeda:walks-tree`: 0 → 8 files (`grep -rl` over `crates scripts`). No master names the token
    (grep `walks-tree|walk-class`: 0 hits).
- **Dev-tool versions:** none changed. `cargo-audit` and `cargo-deny` re-read unchanged; `cargo audit` loaded 1290
  advisories over 562 crate dependencies, 7 allowed warnings (6 `unmaintained`, 1 `unsound`). The two Pulse binaries
  rebuilt at `5f77859` are the SUT, not a dev tool (Cross-project, below).
- **Harness / gate surface:** none. `git diff --numstat 0b07b2c -- scripts/ .github/ crates/conductor-cli/
  crates/conductor-verify/ crates/conductor-run/src/` prints nothing: no verb, flag, selector, CI step, bracket label,
  `ReportState` or envelope field. The walks-tree token is a comment read by the pipeline's gate tool header, not by
  any project gate.
- **Cross-project / external claims:**
  - **Pulse at `5f77859`** (`../andromeda-pulse`, read with `git show`, HEAD level with its upstream, build inputs
    clean before and after both builds). The shipped L4 model is `gemma-4-E4B-it-Q4_K_M`
    (sha256 `85a896a0…fab87`, equal to the pin; Pulse's own log read `model_identity` `gemma-4-E4B-it-Q4_K_M`,
    `inference_mode` `real`). Prompt `v2.5` on all 38 prompt assemblies of the three leg windows. The digest carries a
    `TRIGGER: ` line naming the cue's cause and no scope; the prompt obliges rank 1 to name that signal.
    `crates/workspace-detector` (the workspace key's derivation) is byte-identical from `a2addb3` to `5f77859`
    (`git diff --stat a2addb3 5f77859 -- crates/workspace-detector` in the Pulse repo: empty), so the key is still a
    PATH whose leaf is the data dir's. On this series — the first on the Linux dev host, on a home-rooted data dir —
    the leaf occurs 0 times across the three captures, each capture's rendering witness reads `verbatim`, and each
    `## Previously Seen` suffix (a POSIX path) prints `<redacted>`, taken whole by `redact_value` after the key mask.
  - **Pulse sets its own render posture on Linux.** The launch carried no host rendering lever; Pulse logged
    `app.boot.render.posture` `lever` `__NV_DISABLE_EXPLICIT_SYNC`, `posture` `applied`, and the app stayed up for the
    whole round. The 2026-10-03 rule-file entry that a launch on this host carries `WEBKIT_DISABLE_DMABUF_RENDERER=1`
    holds only for a Pulse build older than `2099998` (2026-10-04).
  - **The real-model canary fires three storms; the capture prints two `canary:` lines.** The third storm lands at
    the scenario's emission instant, so its digest's tick falls after it. Measured on all three drives from Pulse's
    log; each third storm parsed `ok` and logged `deduped`.
  - **CI:** run CI#37528717687 on sha `9df683d14b6c92b27e7f250fed3549e4a72fb3eb`. Attempt 1 `failure`: the A11y job's
    step `Upload a11y violation record` (`Failed to FinalizeArtifact … ECONNRESET`), with the Rust and Frontend jobs
    `success` and the a11y arm itself passing (19 passing, 2 skipped). The overseer re-ran the failed job. Attempt 2
    `success`: `verdict: green · checks 3/3`. Both readings are in `evidence/attempt-ledger.md`.
  - **Inputs** (`inputs.py verify`: 24 entries — unchanged 17 · drifted 0 · vanished 0 · broken 0 · n/a 7 · uncited 5
    · unparsed 0):
    - I1 · `../additional/pc-overseer/relays/conductor-phase-v309fourth-2026-10-06.md` · copy · unchanged
    - I2, I17, I18 · overseer messages (phase directive, P4 answers, P5 review) · copy · n/a (a message has no live source)
    - I3-I7, I9-I16, I19 · `../andromeda-pulse` files at `5f77859` (master route, `assembler.rs`, `schema.rs`,
      `llamacli_inference.rs`, the rank-1 chunk report, `prompt.rs`, `triage/src/contract.rs`, `inference_runtime.rs`,
      `l4-output.gbnf`, `workspace-detector/src/contract.rs`, `markdown.rs`, `triage/build.rs`, `observability.rs`,
      `render_posture.rs`) · pointer committed · unchanged (14 entries)
    - I8 · `../additional/pc-overseer/l4-env.sh` · copy · unchanged
    - I20, I21, I22 · overseer messages at implement (the binary-proof ruling and the GPU go; the operator-pass word;
      the word after the red CI reading) · copy · n/a · UNCITED by scope/research/plan (they postdate all three; cited
      in the ledger and here)
    - I23 · `../additional/pc-overseer/relays/conductor-wrap-v309fourth-2026-10-06.md` · copy · unchanged · UNCITED
      (this wrap's relay)
    - I24 · overseer message, the wrap directive · copy · n/a · UNCITED
- **Reverted / negative API facts:**
  - The contract section's FIRST `pulse-app` content proof (`--grammar-file` present) was written, digest-recorded
    (`6c4590ac…cd220`), then replaced BEFORE any drive on the overseer's ruling (inputs#I20). The final digest is
    `00173912…8097`; both are in the ledger; the harvest pins the final one.
  - The plan's one-relaunch arm (`WEBKIT_DISABLE_DMABUF_RENDERER=1`) was never used.
  - No ref test `v3_09_ref_identified_with_the_real_model_witnesses_2026_10_06` exists: the met-condition is false.
- **Insufficient fixes (written, kept, not the remedy):** none in this repo. (Pulse's trigger framing made the retry
  facet reliable and left the service facet uninstructed; that is Pulse's, and the route owns it — P5.)
- **Spec claims disproved by measurement:**
  1. **The `pulse-app` content proof by the argv literal.** Stated in `verification-matrix.json#v3-09` acceptance
     ("`pulse-app` by content (the argv element `--grammar-file` present, the retired `--json-schema-file` absent)"),
     the plan's security criterion and its gate entry `grep -c -a -F -e '--grammar-file' pulse-app`. Measured: the
     rebuilt binary reads **0** for that string, as the pre-build binary does; the compiler builds the 14-byte literal
     from two immediates (`--gramma` and `mar-file` each read 1 on the new binary, 0 on the old). The measured proof:
     `hypotheses-item-statement-kv` 0 → 2 and `--json-schema-file` 1 → 0. No master states the literal (grep
     `grammar-file|json-schema-file` over the seven and the registries: 0 hits). Owner: P7.3 (the matrix acceptance).
  2. **architecture names the earlier model as Pulse's L4 model** at three sites (grep `Llama` in architecture: 7
     occurrences on lines `:70`, `:93`, `:200`; 0 in the other six and the registries):
     - `:200` (§Occupied Resources, the `ANDROMEDA_PULSE_L4_DETERMINISTIC` entry): "swaps a canned `L4Output` for the
       Llama-3.2-3B inference" — a STANDING description; the model the flag replaces is now `gemma-4-E4B-it-Q4_K_M`.
     - `:70` (§Established Decisions [Read-Back Dependency Posture]): "L4 llama.cpp Llama-3.2-3B inference decides
       Dismiss/Severity" and "the 3B LLM" — inside a decision DATED 2026-06-27.
     - `:93` (§Standard Contracts — Readiness gate): "L4 Llama-3.2-3B" — inside a NOTE "verified live 2026-06-27".
  3. **`.claude/rules/verification-harness.md` 2026-10-03** ("a `pulse-app` launch carries
     `WEBKIT_DISABLE_DMABUF_RENDERER=1`") — false for a build at or after Pulse `2099998`. A rule file, not a master:
     P3 curation (the overseer's note, inputs#I23 §3).
  4. **Research's walks-tree candidate list was a floor.** It named eight; a ninth (`conductor-core`
     `scenario_catalog.rs` inline tests) and a tenth (`conductor-tauri/src/commands.rs`, an inline test over a
     committed fixture dir) exist. No master states the count. The tenth is routed (P5 CARRY).
- **Expected amendments (from plan):**
  1. architecture §Occupied Resources — On-disk artifacts, the posture-contract entry (a fourth series, the `5f77859`
     re-pin) — **carried**: Counts / qualifiers moved, second bullet. Site: grep `pulse-real-model-leg-posture` →
     architecture 1 hit (`:184`), test-plan 3 (`:124`, `:260`, `:391`). Headroom measured
     (`scripts/arch-registry-check.py measure`): §Occupied Resources 38083 B of 38115 (32 B spare), §Established
     Decisions 37991 B (124 B spare). Dated detail goes to the sidecar.
  2. architecture [Read-Back Dependency Posture], §Standard Contracts — Readiness gate, §Occupied Resources — the
     `ANDROMEDA_PULSE_L4_DETERMINISTIC` entry (each sentence naming the earlier Llama model, attributed per sentence)
     — **carried**: Spec claims disproved, item 2, with the per-sentence attribution.
  3. No amendment on architecture's names-only-registered-handles clause — **not carried, as planned**: the section
     names no new handle and no rendering lever (grep `WEBKIT|EXPLICIT_SYNC` in the contract: 0).
  4. Curation candidate, `verification-harness.md` 2026-10-03 — **carried** to P3: Cross-project, second bullet.
  5. security-plan §Input Validation, the real-model capture ingest row (the temp roots; a fourth series on the Linux
     host) — **carried**: Schema / config and Counts. Site: grep `mask_host_paths` → security-plan `:121`, `:338`.
  6. test-plan §6 Real-model interpretation leg (the fourth series' dated verdict; the temp-root arms) — **carried**:
     Headline and Counts. Site: grep `2026-10-01 series` → test-plan 1 line (`:260`).
  7. obs-plan §4 Real-model posture (a dated live observation at `5f77859`, only for what the drives measured) —
     **carried**: Cross-project (the model identity and prompt version Pulse logged; the three-storm canary). Site:
     grep `Real-model posture` → obs-plan 1 line (`:221`).
- **Coverage of new surfaces:**
  - `mask_host_paths` temp roots (a scrub stage of the test-binary capture ingest; no new external surface) →
    validation n/a · instrumentation n/a · PII redacted✓ (two arms, each seen red before the extension) · tests unit
    (`the_mask_covers_the_named_roots`, `a_posix_path_valued_workspace_leaks_neither_its_key_nor_its_path`) · a11y n/a
    · tokens n/a
  - The fourth series' committed captures (corpus-rendered model text entering `evidence/`) → validation the four-stage
    chain✓ · instrumentation n/a · PII redacted✓ (key probe 0; host-path probe 0; zero un-elided `fingerprint_hex`
    values; each `## Previously Seen` suffix prints `<redacted>`) · tests unit (digest pins, fixed point of the
    elision, population 17) · a11y n/a · tokens n/a
  - No new production operation, log line, span, UI element or external input.

## Deviations from intent

1. **The pre-registered section was amended once, before any drive.** The plan says the section is never edited after
   its digest is recorded. Entry 8 measured its proof unsatisfiable; implement stopped before the launch and the
   overseer ruled to amend with both digests and the reason in the ledger (inputs#I20). The grading rule, the pass
   condition, the drive design and the launch posture were not touched.
2. **The two Pulse builds ran concurrently with plan steps 2-4**, to reach the GPU leg inside the founder's window.
   Pre-registration preceded d1.
3. **Step 12's red-before-green reading:** every measured value came from the grading arm's own failure output
   except d3's route, which was pre-set to the value d1 and d2 measured and then held by the arm's assertion.
4. **Step 14:** seven of the eight candidates marked. `matrix_ledger_gate.rs` is NOT marked: it reads only
   `requirements.md` and `verification-matrix.json`, both named by literals a name grep finds. A ninth walker was
   marked (scope record). A tenth was left unmarked: the plan's gate holds `crates/conductor-tauri/` byte-unchanged.
5. **The launch env block** also carried the leg env script's CPU-binary and tokenizer handles, as sourced.
6. **The operator pass** (hygiene, the pre-CI commit, the guarded push, both CI reads) was performed by the agent on
   the operator's explicit word (inputs#I21, #I22), as in every chunk of this version; the re-run of the failed CI job
   was the overseer's own act.

Scope record (`gate.py scope`: `clean — changed 15 · listed 14 · recorded 1`):
- in-intent — `crates/conductor-core/src/scenario_catalog.rs` · serves step 14 · self

## Decisions & corrections

- **Overseer, before the launch (inputs#I20):** amend the binary proof pre-d1, both digests in the ledger; GPU go for
  d1-d3 in one sitting; Part C the same night (only GPU runs wait for daytime).
- **Overseer, after implement (inputs#I21, #I22):** the operator pass on their word; the red CI reading verified by
  them as a runner network flake and the failed job re-run by them.
- **FOUNDER RULING 2026-10-06, live, his own word, relayed by the overseer (inputs#I23 §2):** `v3-09` is neither
  relaxed nor deferred — Pulse is fixed, then a fifth series runs. Rejected by him: relaxing the rule to the retry
  facet; deferring `v3-09` at the version close.
- **Sweep hazards found:**
  - A `grep -a -F` for a short flag literal in a release binary can read 0 on a correct build (the compiler builds a
    literal of 16 bytes or fewer from immediates). A control taken only on the STALE binary cannot show it; take the
    control on both binaries before pre-registering a content proof.
  - `inputs.py snap` refuses any file under the temp dir; a relayed message goes to the run dir first and is
    snapped with `--message-file --origin`.
  - The capture's `canary:` count (2) is not the canary's storm count (3) under the real-model posture.
  - The Bash working directory persisted after a `cd` again, at this wrap's own Setup (a sixth consecutive session).

## Outcome

Acceptance criteria, each re-asserted against the diff:
- (capability) `v3-09` — the series ran as its section fixes it and the harvest states its verdict from the pinned
  captures: **the criterion holds; the capability is NOT MET** (`v3_09_is_not_met_by_the_2026_10_06_series`: graded
  `[d1 Identified, d2 Identified, d3 NotIdentified]`), with no ref test and no further drive → P7.3 (the claimed cap
  cannot verify; its premise correction and its return to the pool) and P5 (the founder's ruling).
- (arch) section add-only, digest in the ledger, pin equal — **met** (numstat 111/0;
  `the_2026_10_06_series_rule_was_fixed_before_d1` green).
- (arch, layouts) every drive through `run --live real-model`; no file under `scripts/`, `conductor-cli`,
  `conductor-verify`, `conductor-run/src` — **met**.
- (arch) grading in the test tree; 562 packages; no manifest — **met**.
- (arch, security) the non-priming probe exit 0 before each drive — **met** (round listings, three probes green).
- (security) both binaries built from a clean `5f77859` tree and proven before d1; the model sha equal — **met by the
  amended pair; UNMET as the criterion's own words name the pair** (`--grammar-file` present) → P7.3, pre-ratified by
  the overseer (inputs#I23 §1). The sidecar: the provenance arm, recorded.
- (security, tests) pins, zero un-elided values, fixed point, zero key occurrences, population 17, no capture text in
  test source — **met**.
- (security) host-path probe 0 over `evidence/`; secret-scan gate green — **met**.
- (security) the temp-root mask: red before, green after, POSIX arm, prior arms unchanged — **met**.
- (tests) the rule byte-identical; each capture opens with it — **met**.
- (tests) harvest and `cargo test -p conductor-run` green, no retries; `capture_paths_guard` green — **met**.
- (tests, obs) `agent-run.sh run` exit 0; `cargo fmt --all --check` clean — **met**.
- (obs) eleven-key envelopes, `verdict` null, `state` `ManualCheck` on all three — **met**.
- (security) advisory-db porcelain empty; audit and deny exit 0 — **met**.
- (tests) marks add-only; every candidate dispositioned — **met** (plus a ninth marked and a tenth routed).
- (process) post-series census empty; the process table; four frozen chunks unchanged — **met**.
- (ci) `verdict: green` for the pushed HEAD, run id named — **met on the second reading** (CI#37528717687 attempt 2;
  the first reading red on an artifact-upload connection reset, recorded).

Gates (by `run`, in block order; the operator pass's final state is the basis):
- `git -C … merge-base --is-ancestor 5f77859 '@{u}'` · `rev-list --count '@{u}..HEAD'` · `status --porcelain -- crates
  pulse-app Cargo.toml Cargo.lock` · `rev-parse HEAD` — operator, by hand: exit 0 each; atoms held (`0`, no output,
  the full sha).
- `sha256sum pulse-app andromeda-pulse-mcp && ls -l …` (before the builds) — operator, `recorded`.
- `cargo build --release -p pulse-app` · `… -p mcp-server --bin andromeda-pulse-mcp --features mcp-server` —
  operator: exit 0 each (`Finished` in 2m 25s, 2m 24s).
- `grep -c -a -F -e '--grammar-file' pulse-app` — operator: **red by construction** (0 at exit 1); a plan defect,
  superseded by the amended proof on the overseer's word. Recorded in the ledger; never re-run.
- `grep -c -a -F -e '--json-schema-file' pulse-app` — operator: held (`0` at exit 1).
- the sidecar's three-string reading · the after-build digests — operator, `recorded`.
- `test -f "$ANDROMEDA_PULSE_MODEL_PATH" && …` · `sha256sum "$ANDROMEDA_PULSE_MODEL_PATH" | cut …` · the NVIDIA
  major check · `ls -1 "$HOME/.cache/pulse-legs" …` — operator: exit 0 each; atoms held.
- The round (`leg = 'round'` and `'live'`), three `--live-legs --entry` firings, listings in `evidence/round-200628Z.txt`,
  `round-201255Z.txt`, `round-202228Z.txt`: each census, each `conductor … preconditions --for
  real-model-interpretation`, both `sleep 180`, and the three `bash scripts/agent-run.sh run --live real-model && cp …`
  legs `green`; each firing `round: COMPLETE · legs fired 1/1`; no survivor.
- the post-series census — operator: held (exit 1, last line `0`).
- `cargo nextest run -p conductor-run --test real_model_harvest --profile ci` — green (112/112).
- `cargo test -p conductor-run` — green. `cargo nextest run -p conductor-run --test capture_paths_guard --profile ci`
  — green (8/8).
- `bash scripts/agent-run.sh run` — green (1212/1212, doctests, three lint lines). `cargo fmt --all --check` — green.
- `grep -c "^name = " Cargo.lock` — green (`562`). The rule diff — green (no output). The contract numstat — green
  (`0`). The no-touch numstat — green (no output). The `conductor-core` deleted-lines probe — green (`0`). The four
  frozen chunks — green (no output).
- advisory-db porcelain · `cargo audit` · `cargo deny check advisories bans licenses sources` — green.
- the host-path probe over `evidence/*` · the key probe over the captures — green (`0` at exit 1 each).
- `bash scripts/agent-run.sh status <id>` on d3's run id — operator: exit 0, atom held.
- `gate.py hygiene` — operator: `hygiene: clean`.
- `git diff --quiet && … git push origin HEAD && echo "PUSHED_SHA=…"` — operator, on the operator's word: exit 0,
  `PUSHED_SHA=9df683d1…`.
- `ci.py conclusion --sha HEAD --wait 1500` — operator: first reading `verdict: red` (attempt 1, the artifact-upload
  reset, not a test); second reading `verdict: green · checks 3/3` (attempt 2, after the overseer's re-run).

Watches: none folded.

Outcome basis: the operator pass ran — Setup's commit list is `9df683d` alone, and the final HEAD's CI run is
recorded in `evidence/attempt-ledger.md`. Implement's P4 report is used as given (the conversation is present), with
four overseer directives after it (inputs#I21-#I24) and the founder's ruling (inputs#I23).

Process hygiene (implement's census, re-measured at this wrap: the pattern
`pulse-app|andromeda-pulse|llama-cli|conductor|WebKit` matches 0 processes; 0 listeners on `:4317` or `:4318`):

| Process | Started by | Final state |
|---|---|---|
| `pulse-app` and its five WebKit children | implement, on the overseer's go | terminated (SIGTERM, gone within 2 s) |
| `conductor` and `andromeda-pulse-mcp`, one pair per leg | the legs | terminated; no survivor on any firing |
| `llama-cli`, one per inference | `pulse-app` | terminated |
| the two Pulse release builds; the gate block's cargo | implement | terminated |
