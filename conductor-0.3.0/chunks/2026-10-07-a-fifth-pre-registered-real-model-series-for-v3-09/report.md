# Report — 2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09

**Chunk:** a fifth pre-registered real-model series for `v3-09`, the same three-drive design and rule at Pulse `f70be92` (prompt `v2.6`)
**Date:** 2026-10-07
**Commits:** `29adafa` chore(setup-project): upgrade conductor — U02 (the chunk base; not this chunk's) · `ca52914`
chore(2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09): operator pre-CI commit, for the run this
chunk's verdict reads (basis: `git log --since=2026-10-06T21:27:00Z`, 2 commits after the previous wrap's)

**The verdict: `v3-09` is NOT MET, 2 of 3.** d1 `Identified`, d2 `Identified`, d3 `NotIdentified`. Pulse pinned at
`f70be92`, prompt `v2.6`; the design and the grading rule are byte-identical to the 2026-10-06 series. Recorded,
never replaced; no ref test; no fourth drive.

## Changes (structured — detectors read this)
- **Files:** 7 outside the chunk folder, all inside research's lists (basis: `gate.py scope` at this wrap, base
  `29adafa`: `scope: clean — changed 7 · listed 7 · recorded 0`):
  - `contracts/pulse-real-model-leg-posture.md` — one add-only section, `## The 2026-10-07 series` (115 added, 0
    deleted; basis `git diff --numstat 29adafa`);
  - `crates/conductor-run/tests/real_model_grading/series_2026_10_07.rs` — NEW, the series' grading module;
  - `crates/conductor-run/tests/real_model_grading/mod.rs` — one `mod` line;
  - `crates/conductor-run/tests/real_model_grading/capture_population.rs` — `COMMITTED_CAPTURES` 17 → 20;
  - `crates/conductor-run/tests/real_model_series/mod.rs` — `EVIDENCE_2026_10_07`, `SERIES_2026_10_07` (three
    sha256 pins);
  - `crates/conductor-run/tests/real_model_harvest.rs` — one `use`, the `GRADING_MODULES` row (11 → 12), one loop in
    `graded_captures`; nothing between the rule markers;
  - `crates/conductor-tauri/src/commands.rs` — one added comment line, the `andromeda:walks-tree` mark.
  Chunk folder: `evidence/attempt-ledger.md`, `evidence/rm-capture-d1.txt` · `-d2.txt` · `-d3.txt`,
  `evidence/round-074555Z.txt` · `round-075216Z.txt` · `round-080141Z.txt`, `inputs/` (19 entries).
- **Symbols / APIs:** none shipped. Test-tree only: the constants and the module above, and seven tests in
  `series_2026_10_07.rs` (`the_2026_10_07_series_rule_was_fixed_before_d1`,
  `each_2026_10_07_capture_matches_its_pinned_digest`,
  `each_2026_10_07_drive_recorded_the_current_rule_before_it_fired`,
  `the_2026_10_07_captures_carry_no_fingerprint_and_no_workspace_key`,
  `each_2026_10_07_drive_grades_as_the_ledger_records`, `the_2026_10_07_envelopes_carry_the_eleven_keys`,
  `v3_09_is_not_met_by_the_2026_10_07_series`). No verb, flag, selector, port, socket, env-handle reader, span, log
  line, field, `ReportState` or envelope key (basis: the frozen-path gate below, no output).
- **Environment handles — a registration fact, not a new reader.** The posture contract NAMES six handles before and
  after this chunk (basis: the handle-census gate, `last line 6`). Two of the six,
  `ANDROMEDA_PULSE_MODEL_PATH` and `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`, are absent from architecture (basis:
  `grep -o -F` for each name over the seven masters and `.andromeda/registries/`: 0 hits each). Three facts hold for
  both: Conductor neither sets nor reads either (the plan's `grep -rn` over `crates`, `scripts` and `.github`: 0
  hits); the committed posture contract names both; neither is a run-contract term or a `conductor preconditions`
  subject. They are PULSE's launch handles, set by the launching shell for `pulse-app`. Architecture's
  posture-contract entry (`architecture.md:184`, @c1020) says the contract "NAMES only already-registered
  environment handles"; that clause is false until the two are registered. The gap predates this chunk (the
  2026-10-06 handoff surfaced it); the plan carries its registration as an Expected amendment.
- **Crates / modules:** no crate added, removed or changed in its manifest. `conductor-run`'s test tree gains one
  module. `conductor-tauri` gains one comment line.
- **Dependencies:** none. `grep -c "^name = " Cargo.lock` reads 562, as at the chunk base; no manifest changed.
- **Schema / config:** none. No scenario, no manifest, no scrub or redaction shape changed. The capture chain
  (`mask_workspace_key` → `redact_value` → `mask_host_paths` → `elide_fingerprints`) is unchanged and was exercised
  by three more captures.
- **Spec-master edits:** none before this wrap.
- **Counts / qualifiers moved** (each with the docs stating it; basis for every "0 hits" is `grep -o -F` over the
  seven masters and `.andromeda/registries/` at this wrap):
  - the posture contract's dated series: four (2026-09-29, 2026-09-30, 2026-10-01, 2026-10-06) → five, adding
    2026-10-07. Stated at `architecture.md:184` @c236 (the series list) — 1 site;
  - the contract's latest per-series Pulse pin: `5f77859` at 2026-10-06 → `f70be92` at 2026-10-07. Stated at
    `architecture.md:184` @c852 — 1 site. `5f77859` also stands at `security-plan.md:121` @c3838, `test-plan.md:260`
    @c4545 and `obs-plan.md:221` @c1504, each inside a DATED record of the 2026-10-06 series, which stays true as
    written (`5f77859`: 4 hits — architecture 1 · security-plan 1 · test-plan 1 · obs-plan 1; `f70be92`: 0 hits);
  - the series whose workspace-key derivation and leaf rendering are recorded: "the 2026-09-30, 2026-10-01 and
    2026-10-06 series" (2 hits, both `security-plan.md:121`, @c3968 and @c4360) → the 2026-10-07 series joins them;
  - the capture exception's per-series inventory (`security-plan.md:338` @c1302) ends at the 2026-10-06 series → the
    2026-10-07 series' d1, d2 and d3 captures carry one report body each;
  - `COMMITTED_CAPTURES` 17 → 20, `GRADING_MODULES` 11 → 12, the real-model harvest 112 → 119 tests, marked
    tree-walking `.rs` files 8 → 9, workspace nextest 1219: none is stated in a master (`COMMITTED_CAPTURES`,
    `GRADING_MODULES`, `walks-tree`, `112 tests`, `1212`, `1219`: 0 hits each). `119` has 1 hit in security-plan; it
    is not read here as this count and is left to that doc's detector.
- **Dev-tool versions:** none — no host tool installed or upgraded. `cargo audit` re-read at the gate: 1290
  advisories loaded, 562 crate dependencies, 7 allowed warnings (6 `unmaintained`, 1 `unsound`), exit 0, on the dev
  host, 2026-10-07 (the previous recorded reading: 1243 · 562 · 7 on 2026-09-10; the advisory count moved on
  external database movement alone, zero dependency delta).
- **Harness / gate surface:** none. Every drive ran through the existing `bash scripts/agent-run.sh run --live
  real-model`; `scripts/`, `.github/`, `scenarios/`, `conductor-cli`, `conductor-verify`, `conductor-run/src` are
  byte-unchanged against `29adafa` (the frozen-path gate, no output).
- **Cross-project / external claims:**
  - **Pulse (`../andromeda-pulse`)** at HEAD `f70be92c2ca13c951330efeffd725e62a484610c`, on its pushed branch, build
    inputs clean (the four Pulse git probes, all green; re-read at this wrap: HEAD `f70be92`). Both release binaries
    were built by the agent from that tree on the overseer's grant. `pulse-app` is proven by content, measured on
    both builds before the section was written: the two strings of the sentence `f70be92` adds read 0 / 0 on the
    `5f77859` build and 1 / 1 on the rebuilt one, and the two retired version literals read 1 / 1 then 0 / 0;
    sha256 `f69be5bb…` → `df167647…`. The sidecar `andromeda-pulse-mcp` is proven by build provenance; its sha256
    `6175fc36…` did NOT move, because none of the moved text is in that binary.
  - **Pulse's own log, per drive** (read fields-only): `inference_mode` `real`, `model_identity`
    `gemma-4-E4B-it-Q4_K_M`, `prompt_version` `v2.6` on all 37 prompt assemblies of the three leg windows (10 · 11 ·
    16), 37 of 37 parses `ok`, no `interpretation.inference.error`, no refused append. The model file's sha256
    equalled the pinned `85a896a0…fab87` before d1.
  - **What the Pulse sentence did and did not do:** d1 and d2 now quote `scope_id=conductor` in rank 1's rationale
    and name `conductor` as a whole word in the rank-1 statement; d3 is unchanged in kind — its rank-1 statement
    reads "Active Retry Storm Detected on conductor-canary." (`evidence/rm-capture-d3.txt:426`), the hyphenated
    canary identity the rule separates from the service.
  - **Pulse CI's `boot smoke` red on `f70be92`** (the app booting, then exiting) did not reproduce on this host: the
    app stayed up at its own render posture, with no relaunch.
  - **CI of this repo:** run CI#37593230851, `completed/success`, checks 3/3, on
    `ca529144a0f587f5872c58650836a36c92de6a7a` (the operator pre-CI commit), first reading, no re-run. The sha is
    the record: this wrap's own commit lands on top of it.
  - **The inputs** (`inputs.py verify` at this wrap: `19 entries — unchanged 15 · drifted 0 · vanished 0 · broken 0 ·
    altered 0 · unreachable 0 · n/a 4 · uncited 2 · unparsed 0`):
    - I1 · `../additional/pc-overseer/relays/conductor-phase-v309fifth-2026-10-07.md` · copy · unchanged;
    - I2 · message, the overseer, the phase invocation · copy · n/a;
    - I3-I6, I8-I16 · `../andromeda-pulse` (`.andromeda/master-route.md`, `crates/interpretation/src/prompt.rs`,
      `…/schema.rs`, `crates/triage/src/digest/assembler.rs`, the Pulse chunk's `reading.md` and
      `post-hoc-d3-replay.md`, `pulse-app/src/llamacli_inference.rs`, `l4-output.gbnf`,
      `crates/mcp-server/Cargo.toml`, `crates/security/src/scrubber.rs`, `pulse-app/src/observability.rs`,
      `crates/workspace-detector/src/contract.rs`, `pulse-app/src/inference_runtime.rs`) · pointer
      `committed@f70be92c` · unchanged, all 13;
    - I7 · `../additional/pc-overseer/l4-env.sh` · copy · unchanged;
    - I17 · message, the overseer, the P5 review answer · copy · n/a;
    - I18 · message, the overseer, 2026-10-07 09:45 local, the go for d1, d2 and d3 · copy · n/a — UNCITED by scope,
      research and plan (it arrived at implement); cited here and in the ledger;
    - I19 · message, the overseer, after the implement report (the covariate direction and the operator-pass word) ·
      copy · n/a — UNCITED likewise; cited here and in the ledger.
    No entry drifted, vanished or broke.
  - **Read live at this wrap, not snapshotted** (`inputs.py snap` takes no wrap step):
    `../additional/pc-overseer/relays/conductor-wrap-v309fifth-2026-10-07.md`, sha256 `90c0780d…b1fa0a`, the
    overseer's wrap relay. Its covariate figures were recounted from the six captures before they were written
    below; all twelve match.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none in this repo. Recorded as a cross-project
  observation only: Pulse's `v2.6` sentence is followed on two drives of three and the miss it was written for
  stands on d3; that remainder is Pulse's and the founder's.
- **Spec claims disproved by measurement:**
  - `architecture.md:184` @c1020 — "It NAMES only already-registered environment handles". Measured false for two
    of six (0 hits for either name in architecture). Disposition owed: the registration (Expected amendment 2).
  - `plan.md` step 6 — the new section's dir clause was to cite "the 2026-10-06 section's reason" for a home-rooted
    dir (the mask covers no temp-rooted path). That reason no longer holds: `mask_host_paths` names `/tmp/` and
    `/var/tmp/` since the 2026-10-06 chunk (`crates/conductor-run/tests/real_model_common/mod.rs:348`;
    `security-plan.md:121` @c1793 already says so). The section was written "home-rooted on this host as the
    2026-10-06 series' dir was". A plan clause, not a master claim; no master needs an edit for it.
- **Expected amendments (from plan):**
  1. architecture §Occupied Resources → On-disk artifacts (the posture-contract entry): a fifth series date and the
     latest pin `f70be92` in place of `5f77859` — **carried**: Counts / qualifiers moved, lines 1-2. Sites:
     `2026-10-06` in architecture 2 hits, both `:184` (@c236, @c864); `5f77859` 1 hit (`:184` @c852). The section
     holds 38086 B of a 38115 B threshold (`scripts/arch-registry-check.py measure` at this wrap), so 29 B of
     headroom; the pin swap is length-neutral and the date adds 12 B.
  2. architecture §Occupied Resources → Environment variables: register `ANDROMEDA_PULSE_MODEL_PATH` and
     `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` under the playbook's routine rule (`.andromeda/playbook.md:137-139`) —
     **carried**: the Environment handles bullet above, with the rule's three facts. Sites: 0 hits for either name
     in any master or key file; the clause it makes true is `architecture.md:184` @c1020. Same 29 B bound, so the
     registration needs bytes freed inside the section or its dated detail in the sidecar.
  3. security-plan §Input Validation (the real-model capture ingest row) and §Security Anti-Patterns → Data
     Protection: the fifth series' dated record, the workspace-key derivation read unchanged at `f70be92`, the
     exception's per-series inventory — **carried**: Counts / qualifiers moved, lines 3-4, and Coverage below. Sites:
     `security-plan.md:121` (@c3838, @c3968, @c4360, @c4447) and `:338` (@c1302). The derivation's basis is
     inputs#I15 (`crates/workspace-detector/src/contract.rs`, `committed@f70be92c`, unchanged). No handle inventory
     moves.
  4. test-plan §6 Real-model interpretation leg: the fifth series' dated verdict — **carried**: the verdict line and
     Outcome. Site: `test-plan.md:260`, after the 2026-10-06 record (@c4545).
  5. obs-plan §4 Real-model posture: a dated live observation at `f70be92` — **carried**: Cross-project (Pulse's own
     log) and Coverage. Site: `obs-plan.md:221`, after the 2026-10-06 record (@c1504). What the drives measured and
     nothing wider: `model_identity` `gemma-4-E4B-it-Q4_K_M`, `prompt_version` `v2.6`, all three envelopes
     `ManualCheck` with `verdict` null and the eleven keys, no span, log line or field added.
  6. architecture §Established Decisions [Read-Back Dependency Posture]: only if the series reads MET — **not
     carried**: the series reads NOT MET, so the section's one `v3-09` verdict statement stands as written.
- **Coverage of new surfaces:** no new external surface, hot-path op or UI element.
  - `series_2026_10_07.rs` (test module) → validation n/a · instrumentation n/a · PII redacted✓ (captures are
    digest-pinned and graded from file; no capture text in test source; 0 un-elided `fingerprint_hex=`, 0
    occurrences of the workspace key) · tests integ (7 tests, the harvest 119 of 119) · a11y n/a · tokens n/a
  - three committed captures (untrusted SUT output, the existing ingest) → validation mechanism✓ (the unchanged
    four-step chain) · instrumentation n/a · PII redacted✓ (host-path probe 0, key probe 0; each `## Previously
    Seen` suffix prints `<redacted>`; key rendering `verbatim` on all three, so no drive is contaminated) · tests
    integ · a11y n/a · tokens n/a
  - `commands.rs` comment mark → all n/a · tests unit (`cargo test -p conductor-tauri` 27 of 27)

### What the series measured, and nothing wider

- Three drives in one sitting, 07:45:55Z to 08:10:45Z (09:45 to 10:10 local), on the founder's own word of
  2026-10-07 07:42 local that GPU runs were allowed, the overseer's go at 09:45 local (inputs#I18). 362 · 362 ·
  363 s; each `ReadBack`, attributed on the first poll, `degraded_mode=false`; each envelope `ManualCheck`,
  `verdict` null, eleven keys, `seed` 4317033.
- 6 of 6 classified canary digests surfaced (`skip_reason=none`); the scenario's own digest surfaced in 3 of 3. No
  canary read `pipeline-fault`; nothing was re-fired.
- Every rank-1 statement carries a retry token. The service facet split: d1 and d2 name `conductor` as a whole
  word; d3 names only `conductor-canary`. d3's rank-1 rationale line does carry `scope_id=conductor`; the rule
  grades the statement.
- The same split, on the same drive, was recorded on 2026-10-06 at prompt `v2.5`. Three drives per series do not
  separate the two prompts.

### A measured covariate across the two series — not a cause

Recorded on the overseer's word (inputs#I19; the wrap relay repeats it). Recounted from all six committed captures
at this wrap: each capture's `creating digest corpus retrieval rows:` line and its `pulse-log
digest.corpus.retrieve:` line.

| Series | d1 | d2 | d3 |
|---|---|---|---|
| fourth (2026-10-06, `5f77859`, `v2.5`) | 1 / 10 · `Identified` | 3 / 13 · `Identified` | 6 / 16 · `NotIdentified` |
| fifth (2026-10-07, `f70be92`, `v2.6`) | 1 / 10 · `Identified` | 3 / 13 · `Identified` | 6 / 16 · `NotIdentified` |

Each cell: the creating digest's corpus rows / the `digest.corpus.retrieve` count Pulse logged in the leg window.
The miss is the six-row drive both times (2 of 2); the one- and three-row drives pass (4 of 4). It is a covariate
and nothing more: the row count rises with the drive ordinal, the earlier incidents in the corpus, the app's uptime
and the canary's history, and no drive varied one without the others, so the series cannot separate them. Only a
probe that varies the row count at everything else equal can.

## Deviations from intent
- **The dir clause's reason (plan step 6)** — written "as the 2026-10-06 series' dir was" in place of "for the
  2026-10-06 section's reason", because that reason is no longer true (above). The dir is still home-rooted, as the
  plan fixes. Reported at implement.
- **Plan entry 5 fired with `command grep`** — the tool shell's `grep` is a shell function; entries 8-13 were then
  fired verbatim from scratchpad scripts under plain bash. Same commands, same readings.
- **The ledger's walker's-mark heading** was lost when the covariate section was inserted, and the pre-CI commit
  `ca52914` carries the ledger without it. Restored at this wrap (one heading line; no figure moved). The ledger
  also gained the entry 47 and 48 records after the push. Both ride this wrap's commit.
- scope record: none — `gate.py scope` clean, 0 recorded (changed 7 · listed 7 · excluded 63).

## Decisions & corrections
- **The overseer's go** (inputs#I18, delegate the overseer, 2026-10-07 09:45 local, on the founder's word of 07:42
  local): d1, d2, d3 in one sitting.
- **The covariate is a measured covariate, not a cause** (inputs#I19, relayed by the overseer, 2026-10-07): the
  table is recorded with its collinearity stated beside it, in the ledger and here.
- **The operator pass was the operator's act, performed by the agent on the overseer's explicit word**
  (inputs#I19): hygiene, the pre-CI commit `ca52914`, the guarded push, the CI read. Not a skill bypass.
- **The sidecar's digest not moving is not a failure**: a relinked binary whose inputs did not change is
  byte-identical. The plan had already replaced a digest-must-differ term with build provenance (inputs#I17).
- **A correction to the agent's own evolve record**: a fix-loop record said "20 operator entries by hand"; the count
  is 15 (entries 1-13, 25, 45). Retracted in the friction ledger at implement.
- **Sweep hazards found:** `grep -c -F 'Previously Seen'` over a capture counts the rule text's mentions too (2 · 3
  · 3 where the `## Previously Seen` list lines number 0 · 1 · 2), so the covariate table is keyed on the two
  counted lines the capture prints, never on that token. `grep -c` at a zero count exits 1 and reads as a failed
  command in a compound call.
- **Curation candidates the overseer offered** (the wrap relay §4; the agent's to word or drop): a series whose
  drives share one growing data dir confounds the drive ordinal with everything that accumulates in it; a capture
  line that counts an input the model reads is cheap to add before a series and expensive to reconstruct after.

## Outcome
Acceptance criteria, each re-asserted against the diff and the evidence:
- (capability) `verification-matrix.json#v3-09` — the series ran as its pre-registered section fixes it and the
  harvest states its verdict from the pinned captures: **NOT MET** (three graded drives, one `NotIdentified`). The
  criterion's own second arm holds: recorded NOT MET, no ref test, no further drive. The CAPABILITY is unmet, so
  `v3-09` gets no ref and stays `planned`; its claim by this chunk is P7's to resolve attended (the overseer's wrap
  relay §2: un-claimed, never deferred, per the founder's ruling of 2026-10-06).
- (arch) the section is add-only (0 deleted); its digest `57b5e96e…7376` is the ledger's `pre-registration sha256:`
  line and the test's pin — **met**.
- (arch) the handle census reads 6 before and after; the two model handles' registration is carried as an Expected
  amendment — **met**.
- (arch, layouts) every drive through `run --live real-model`; no file under `scripts/`, `conductor-cli/`,
  `conductor-verify/` or `conductor-run/src/` changed — **met**.
- (arch) grading in `conductor-run`'s test tree; package count 562; no manifest changed — **met**.
- (arch, security) `conductor preconditions --for real-model-interpretation` exit 0 before each drive, the sidecar
  resolved through the firing form's `PATH` — **met** (three round listings).
- (security) both Pulse binaries built from a clean `f70be92` tree and proven before d1, the proof measured on both
  builds before the section was written; the model digest equal — **met**.
- (security, tests) each capture matches its pin, 0 un-elided keyed `fingerprint_hex=`, a fixed point of the
  elision, 0 occurrences of `rm-fifth-series`; `COMMITTED_CAPTURES` 20; no capture text in test source — **met**.
- (security) host-path probe over `evidence/` 0; secret-scan gate green — **met** (re-read after this wrap's ledger
  edit at P7).
- (tests) the rule is byte-identical to the chunk base; each capture opens with that rule — **met**.
- (tests) harvest, `cargo test -p conductor-run`, `cargo test -p conductor-tauri`, `capture_paths_guard` green, no
  retries — **met**.
- (tests, obs) `bash scripts/agent-run.sh run` exit 0; `cargo fmt --all --check` clean — **met**.
- (obs) each envelope has the eleven keys, `verdict` null, `state` `ManualCheck`; each landing state in the ledger;
  the verdict test keys on no envelope field — **met**.
- (security) advisory-db porcelain empty; `cargo audit` and `cargo deny` exit 0 — **met**.
- (tests) nine marked `.rs` files; `commands.rs` 1 added, 0 deleted; the rest of `conductor-tauri/` unchanged —
  **met**.
- (process) post-series census: 0 processes, 0 listeners; the ledger's process table names each process with its
  final state; the five frozen chunks byte-unchanged — **met**.
- (ci) the operator pass's CI read printed `verdict: green` for the pushed HEAD; the run id is CI#37593230851 —
  **met**.

Gates, by `run`, in block order (implement's verdicts; the operator pass's for the last three):
- `git -C "$HOME/dev/projects/andromeda-pulse" merge-base --is-ancestor f70be92 '@{u}'` — leg operator, by hand:
  exit 0.
- `git -C … rev-list --count '@{u}..HEAD'` — leg operator: exit 0, `last line 0` held.
- `git -C … status --porcelain -- crates pulse-app Cargo.toml Cargo.lock` — leg operator: exit 0, `no output` held.
- `git -C … rev-parse HEAD` — leg operator: exit 0, `last line f70be92c…610c` held.
- the old-side binary table (`sha256sum pulse-app andromeda-pulse-mcp && …`, before the builds) — leg operator,
  `recorded`: `pulse-app` 0 / 0 / 1 / 1, the sidecar 0 / 0 / 0 / 0; both digests the 2026-10-06 builds'.
- `cargo build --release -p pulse-app` (in the Pulse tree, the leg env sourced) — leg operator: exit 0, 1m 38s.
- `cargo build --release -p mcp-server --bin andromeda-pulse-mcp --features mcp-server` — leg operator: exit 0,
  9.01s.
- the asserting sentence probe (`… echo "sentence-a=$a sentence-b=$b"; test "$a" -ge 1 || test "$b" -ge 1`) — leg
  operator: `sentence-a=1 sentence-b=1`, exit 0.
- the new-side binary table (the same command, after the builds) — leg operator, `recorded`: `pulse-app` 1 / 1 /
  0 / 0, the sidecar 0 / 0 / 0 / 0; both mtimes after the Pulse commit.
- `test -f "$ANDROMEDA_PULSE_MODEL_PATH" && test -x "$ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH" && echo "model env
  present"` — leg operator: exit 0, the atom held.
- `sha256sum "$ANDROMEDA_PULSE_MODEL_PATH" | cut -d" " -f1` — leg operator: exit 0, the pinned digest held.
- the NVIDIA major probe (`kernel=$k userspace=$u`) — leg operator: `kernel=610 userspace=610`, exit 0.
- `ls -1 "$HOME/.cache/pulse-legs" …; test ! -e …/rm-fifth-series && echo "leaf absent"` — leg operator: exit 0,
  the atom held.
- the round, three firings recorded in `evidence/round-074555Z.txt`, `round-075216Z.txt`, `round-080141Z.txt`: each
  census (`ps -eo … ; ss -ltn | grep -cE ':4317|:4318'`) green, each `cargo run -q -p conductor-cli --bin conductor
  -- preconditions --for real-model-interpretation` green (the `[PRECONDITION]` atom held), each `sleep 180` green,
  each `bash scripts/agent-run.sh run --live real-model && cp runs/live-suite/rm-capture.txt …/rm-capture-d{1,2,3}.txt`
  green (exit 0, both `[live] leg rm:` atoms, the capture fresh), no survivor on any firing.
- the post-series census (the same `ps … ; ss …` command, after the stop) — leg operator: exit 1, `last line 0`
  held.
- `cargo nextest run -p conductor-run --test real_model_harvest --profile ci` — green, 119 of 119.
- `cargo test -p conductor-run` — green, 376 passed.
- `cargo test -p conductor-tauri` — green, 27 of 27.
- `cargo nextest run -p conductor-run --test capture_paths_guard --profile ci` — green, 8 of 8.
- `bash scripts/agent-run.sh run` — green, exit 0 (workspace nextest 1219 of 1219, doctests, the workspace lint and
  both feature-gated lint lines).
- `cargo fmt --all --check` — green.
- `grep -c "^name = " Cargo.lock` — green, `last line 562`.
- the rule diff (`diff <(git show 29adafa:…real_model_harvest.rs | sed -n …) <(sed -n … real_model_harvest.rs)`) —
  green, no output.
- `git diff --numstat 29adafa -- contracts/pulse-real-model-leg-posture.md | awk '{d+=$2} END {print d+0}'` —
  green, `last line 0`.
- the handle census (`grep -oE '(ANDROMEDA|CONDUCTOR)_[A-Z0-9_]+' contracts/pulse-real-model-leg-posture.md | sort
  -u | wc -l`) — green, `last line 6`.
- the frozen-path numstat (`git diff --numstat 29adafa -- crates/conductor-verify/ … .github/`) — green, no output.
- `grep -rl --include='*.rs' 'andromeda:walks-tree' crates | wc -l` — green, `last line 9`.
- `git diff --numstat 29adafa -- crates/conductor-tauri/src/commands.rs | awk '{print $1 "/" $2}'` — green,
  `last line 1/0`.
- the five frozen chunks' numstat — green, no output.
- `git -C "${CARGO_HOME:-$HOME/.cargo}/advisory-db" status --porcelain` — green, no output.
- `cargo audit` — green, exit 0 (1290 · 562 · 7 allowed).
- `cargo deny check advisories bans licenses sources` — green, exit 0.
- the evidence host-path probe (`ls …/rm-capture-d1.txt >/dev/null && cat …/evidence/* | grep -cE …`) — green,
  exit 1, `last line 0`.
- the workspace-key probe (`… | grep -c "rm-fifth-series"`) — green, exit 1, `last line 0`.
- `bash scripts/agent-run.sh status <id>` — leg operator, by hand on d3's `run_id` `2026-10-07T08-04-42-988`: exit
  0, the `"scenario": "real-model-interpretation"` atom held. Smoke: this is the chunk's smoke; no boot-path
  changed.
- `python -X utf8 "$HOME"/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — leg operator, the operator
  pass: exit 0, `hygiene: clean`.
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD && echo "PUSHED_SHA=$(git rev-parse HEAD)"`
  — leg operator, the operator pass: exit 0, `PUSHED_SHA=ca529144…6a7a` (`29adafa..ca52914`).
- `python -X utf8 "$HOME"/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — leg
  operator, the operator pass: exit 0, `verdict: green · checks 3/3 · wall 549 s`, CI#37593230851.
No entry deferred; no red.

Re-run at the wrap's light gate (2026-10-07, after the amendments and the route edit): the 19 non-leg entries green
again, harvest 119 of 119, workspace nextest 1219 of 1219; `cargo audit` then loaded 1293 advisories (562
dependencies, 7 allowed), three more than at implement, on external database movement alone.

Watches: none folded.

Outcome basis: the operator pass ran. Its one commit is `ca52914` (basis `git log 29adafa..HEAD`), and the final
HEAD's CI run is CI#37593230851, recorded in `evidence/attempt-ledger.md` §The operator pass. Implement's report,
given in this same conversation, is the basis for everything only it holds. Between implement and this report the
overseer directed the covariate table and the operator pass (inputs#I19), and the wrap relay fixed this report's
terms; neither changed a verdict. Post-implement artifacts: the ledger's entry 46-48 records and its restored
heading.

Process hygiene: implement's census — `pulse-app` PID 3117394 (the agent's, on the overseer's go) terminated by
SIGTERM at 08:11:04Z with its five WebKit children; `conductor` and `andromeda-pulse-mcp` exited with each leg;
`llama-cli` with each inference; the two Pulse builds' `cargo` and `rustc` with the builds. Re-measured at this
wrap (09:48Z): 0 matching processes, 0 listeners on `:4317` or `:4318`.
