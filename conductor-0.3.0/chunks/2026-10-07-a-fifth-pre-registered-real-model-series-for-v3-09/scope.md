# Scope — 2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09

**Working entry (`working-route.md:94`):** A fifth pre-registered real-model series for `v3-09` — the same three-drive
design and rule, against the Pulse sha that ships the fix.

**Matrix target:** `v3-09` (*Real-model interpretation leg*, `dynamic-external`, status `planned`, `chunk: null` at
take-up — the pool's one unclaimed capability). Its acceptance as it stands names the 2026-10-06 series by section,
dir (`rm-trigger-series`) and Pulse HEAD (`5f77859`), and its stated limit is written for prompt `v2.5`. This chunk
re-concretizes it at P5 for the fifth series; nothing of the pass condition moves. Met iff at least one drive is
graded AND every graded drive reads `Identified` (the rank-1 statement names `conductor` as a whole word and a retry
token) with the real-model witnesses; a graded `NotIdentified` is NOT met and is never replaced.

**Chunk base:** `29adafa` (`29adafa143b58d3c296762af6ed59270ec5e4558`), HEAD at take-up. Every diff-shaped gate probe
names it explicitly, because the operator pre-CI commit moves HEAD before the wrap.

**Host:** the Linux dev host (Omarchy), as the fourth series. Its host-shaped terms (binaries without `.exe`, the
`ps` census, `kill -TERM` by the recorded PID, Pulse's log under the data dir) are read from the fourth series'
section and re-verified at P3, not re-derived.

## Gate

- gate: cleared — the block reads «BLOCKED-ON: Pulse "the L4 rank-1 hypothesis names the triggering cue's service"
  (the overseer's working title; Pulse mints its own wording) — clears when the overseer relays the sha shipping it».
  The overseer relayed it (inputs#I2; inputs#I1 §1), and the relayed sha was re-read at take-up, not taken on the
  relay's word:
  - Pulse `f70be92` (`f70be92c2ca13c951330efeffd725e62a484610c`) is HEAD of `chore/migrate-pulse-to-v3`, 0 ahead of
    its upstream, and holds `2026-10-06-l4-first-hypothesis-names-the-triggering-service · complete` in its master
    route (`:102`, inputs#I3).
  - The code commit `1a2e509` is its ancestor, and `git diff --name-only 1a2e509 f70be92` lists no path under
    `crates`, `pulse-app` or `xtask` — only wrap bookkeeping. So a build at `f70be92` is a build of `1a2e509`'s
    product tree.
  - CI on `1a2e509`: `ci#37580684260` 12 of 12 checks `success`, `secret-scan#37580684315` `success`. CI on `f70be92`
    itself (`ci#37585281667`) was still `in_progress` at take-up; its `gitleaks` check had completed `success`.
  - Re-read at P5 (2026-10-07 07:33Z), once that run settled: 12 checks `success`, 1 `failure` — the job `boot smoke
    (ubuntu-22.04)` (job 112673874282). Its log shows `boot: ready`, then the `status` step reading `"verdict":
    "not-running"` with `"ended": "exit 1"`, on a hosted runner under xvfb. The same job read `success` on
    `1a2e509`, and no product file differs between the two commits. It is Pulse's CI and Pulse's to own; this
    chunk neither folds nor disposes it. What it touches here is the launch: the 10 s liveness check is the place
    an app that boots and then exits would show on this host. The overseer's answer at the P5 review (inputs#I17):
    verified on GitHub, the failed job re-run by them, the same class as a 2026-10-05 flake on `2dc099a`; proceed
    as planned, and if the app dies on this host too, stop and report, because then it is not a runner flake.
  - What shipped (inputs#I4): `TRIGGER_FRAMING_INSTRUCTION` now ends «When the cue line under ATTENTION CUES carries
    a scope_id, the first hypothesis statement must name that scope_id value exactly as written there, and must not
    attribute the signal to anything else, including a service whose name merely contains it.»
- What the cleared block does NOT settle, and this series measures: whether the SHIPPED model's rank-1 statement now
  names `conductor` as a whole word in all three drives. So no criterion waits behind a standing block and the chunk
  carries no gated mode: the leg runs inside this chunk's implement and its outcome is the acceptance. A pass is not
  assumed (inputs#I1 §2).

## What this chunk builds

1. **A new pre-registered series section in the posture contract** (`contracts/pulse-real-model-leg-posture.md`),
   add-only beside §The drive series, §The 2026-09-30 series, §The 2026-10-01 series and §The 2026-10-06 series
   (`:213`, `:322`, `:378`, `:461`), rewriting none of them. Fixed BEFORE d1: the drive count, the data dir, the pass
   condition, canary-is-a-measurement, the one uncounted pipeline-fault re-fire, no further drive, the quiet windows,
   the launch posture re-pinned to Pulse `f70be92`, the model and prompt versions graded, the slots. Its sha256 is
   recorded in this chunk's `evidence/attempt-ledger.md` before d1 and recomputed by the harvest.
   - **The design and the rule stay byte-identical to the fourth series** (the entry's own words; inputs#I1 §2
     "Keep the design and the rule byte-identical to the fourth series"). Three drives, one fresh dir, the same
     grading rule. The only terms that move are the ones that name the series: its date, its dir leaf, the Pulse sha,
     the prompt versions, the binary proof, and the stated limit.
   - **The stated limit is restated for prompt `v2.6`.** At `5f77859` the retry facet was instructed and the
     `conductor` facet was not. At `f70be92` both are: the TRIGGER line still names the cause, and the new sentence
     obliges rank 1 to carry the cue's `scope_id` value. So `Identified` now means the shipped model followed two
     instructions over a cue line that carries both facets verbatim. P4 writes the wording; P5 previews it.
     Verified at P3: the instruction is appended to all three tiers' prompts (`prompt.rs:278`, `:371`, `:462`,
     inputs#I4), and the cue line reads `{kind} scope_id={value}` (`cue_summary`, `assembler.rs:618-623`,
     inputs#I6), so for the scenario it carries `scope_id=conductor`.
   - The prior sections' Pulse coordinates are dated records and are not rewritten; the new section re-pins what it
     relies on at `f70be92`.
2. **What the series grades** (inputs#I1 §1, each coordinate re-read at take-up):
   - Model `gemma-4-E4B-it-Q4_K_M.gguf`, 4 977 171 584 B, sha256
     `85a896a047553e842f25297ee5b031d64ff30147d9c4af17b1e4b394cd1fab87` — measured at take-up, equal to the digest
     the 2026-10-06 section pins. The model is unchanged.
   - Prompts `v2.6` / `v1.5-fallback` / `v1.5-reflection` (`schema.rs:35`, `:44`, `:59`, inputs#I5).
   - The argv and the committed GBNF are unchanged since `5f77859` (inputs#I1 §1, marked `measured 2026-10-07 at
     Pulse 1a2e509`). Verified at P3: `git diff --name-only 5f77859 f70be92 -- crates pulse-app xtask Cargo.toml
     Cargo.lock` lists five files — `prompt.rs`, `schema.rs`, `l4_decision_probe.rs`, `unit_inference_runtime.rs`,
     `ui/package-lock.json` — so `llamacli_inference.rs` (inputs#I10) and `l4-output.gbnf` (inputs#I11) are
     byte-identical.
3. **The binaries, and a new content proof** (inputs#I1 §3).
   - "Pulse's `target/release` holds the `5f77859` build today" (inputs#I1 §3). Verified at P3 by digest: `pulse-app`
     hashes `f69be5bb…` and the sidecar `6175fc36…`, the two digests the fourth series' ledger records for its
     rebuilt binaries. Both are rebuilt before d1.
   - The fourth series' proof no longer proves the build: on the `5f77859` `pulse-app` the grammar rule name
     `hypotheses-item-statement-kv` already reads 2 and `--json-schema-file` 0 (`grep -c -a -F`, measured at P3).
   - **This chunk picks a content proof the change introduces, and takes a two-sided control on both binaries BEFORE
     pre-registering it** (inputs#I1 §3; the 2026-10-06 lesson, where a pre-registered literal read 0 on a correct
     build). Old side, measured at P3 on the `5f77859` `pulse-app`: the new sentence's words `must name that
     scope_id value exactly as` read 0 and the retired version literal `v1.4-fallback` reads 1. So the candidate
     pair is the sentence's words present and `v1.4-fallback` absent. The new side is read after the rebuild and
     before the section is written; a string that fails either side is replaced, never pre-registered.
     [premise-corrected: the `v2.6` literal is not a candidate — it is 4 bytes, the class the 2026-10-06 lesson
     warns can sit in a binary as immediates.]
   - The sidecar. `crates/mcp-server`, `crates/triage`, `crates/workspace-detector` and `crates/security` are
     byte-unchanged since `5f77859` (the same diff, 0 lines). [premise-corrected: the sidecar is still rebuilt — it
     links `crates/interpretation` (inputs#I12), whose `prompt.rs` and `schema.rs` moved. No string proves it: all
     eleven candidate strings read 0 on the `5f77859` sidecar, so the moved text is not in that binary and a
     rebuilt sidecar may hash the same. Its proof is build provenance, and the fourth series' term "a sha256
     different from the pre-build binary's" is not required here: both digests are recorded, with whether they
     differ.]
   - Slot (inputs#I2): the model slot and ports 4317/4318 are granted for this chunk; pulse-builder stays idle.
     The plan leans that the agent builds both binaries, launches and stops `pulse-app`: the relay asks for a
     control on an `f70be92` build (inputs#I1 §3) while the directive keeps pulse-builder idle (inputs#I2), and the
     same overseer answered so for the fourth series. The directive does not say it in words, so the lean rode the
     P5 review card. Confirmed as written by the overseer there, with the leaf, the sidecar's proof and the
     handle registration (inputs#I17).
   - Pulse's tree at take-up: 3 modified paths, all wrap bookkeeping (`friction-log.ndjson`, one evolve trail, the
     handoff). No build input is touched.
4. **The pre-leg checks** (inputs#I1 §3 "stand as in the fourth series"): the model env is present (inputs#I7 —
   `ANDROMEDA_PULSE_MODEL_PATH`, `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`), the model's sha256 equals the pinned digest,
   and the NVIDIA driver major equals the runtime's (the driver reads `610.57.04` at take-up). Recorded in the
   attempt ledger before d1.
5. **A fresh home-rooted data dir, a NEW leaf** under `~/.cache/pulse-legs/` (inputs#I1 §3). The leaf carries no
   digit, no `@`, no vendor prefix and no keyed scrubber word; its absence before creation is recorded. The fourth
   series' dir `rm-trigger-series` is the only entry there at take-up and stays untouched as evidence. P4 names the
   leaf.
   - Found at P3: the app is launched with the data dir as its cwd, and the cwd's basename reaches the prompt on
     the `PROJECT:` line (`launch_cwd_clear`, `real_model_harvest.rs:226-232`). So the leaf is model input, and it
     also carries neither word of the grading rule (`conductor`, a retry token). Pulse's keyed scrubber arms are
     the `api_key` family and `password|passwd|secret|token` (`scrubber.rs:296`, `:305`, inputs#I13).
6. **The series**, d1..d3, each through `bash scripts/agent-run.sh run --live real-model` against a pulse-app
   launched per the posture contract. Every drive recorded: capture committed under this chunk's `evidence/`,
   sha256-pinned in the series module, graded from the file by the harvest, one row in `evidence/attempt-ledger.md`
   with its workspace-key rendering witness and Pulse's `skip_reason` for every digest it did not surface.
   - **Order (inputs#I2; inputs#I1 §4):** the founder opened the GPU for today (2026-10-07 07:42 local, his own
     word). The plan orders implement so the three-drive sitting is reached EARLY: only what the leg reads (the
     contract section and its recorded digest, the binaries and their proof, the env check, the dir) precedes d1.
     Everything else follows the drives.
   - **The go is asked before the launch** (inputs#I2 "ask me for the go before the launch"). The three drives run
     in one sitting, and a sitting that would run into the night is not started (inputs#I1 §4).
   - The round's point-in-time steps are keyed `leg = 'round'` in the plan's gate fence, as in the fourth series.
7. **The verdict test and the ref.** A harvest test states the series verdict over the pinned captures, as each prior
   series has one. The separate ref test asserting `Identified` plus the real-model witnesses is written ONLY if the
   pass condition holds. A `NotIdentified` or a zero-graded series is recorded as NOT MET, never replaced.
   - **A pass is read as three drives, no more** (inputs#I1 §2 "do not read a pass as proof beyond its three
     drives"). The report states it beside the verdict either way.
8. **The tenth walker's mark** (the entry's CARRY; inputs#I1 §3). The inline test in
   `crates/conductor-tauri/src/commands.rs` reads a committed fixture scenarios dir through the catalog loader
   (`fixture_scenarios_dir`, `:533`; `the_committed_scenarios_fixture_stays_loadable`, `:556`) and carries no
   `andromeda:walks-tree` token today. It takes the token here, once. Not an input to the leg, so it follows the
   drives.
   - The entry calls it "the tenth"; at take-up the token occurs 8 times in 8 files under `crates` and `scripts`.
     Verified at P3: the ordinal counts walkers found, not marks. The fourth series' report names eight candidates,
     a ninth (`scenario_catalog.rs`, marked) and this tenth (routed); seven of the eight plus the ninth were
     marked. This chunk takes the marked files from 8 to 9.
   - Four tests of that module read the fixture dir through one helper (`fixture_scenarios_dir`: graph callers at
     `commands.rs:560`, `:632`, `:649`, `:667`). The mark goes once, on the helper.
9. **The two unregistered env handles** (inputs#I1 §3, unowned since the fourth series' wrap). Architecture's
   posture-contract entry says the contract names only registered handles, yet `ANDROMEDA_PULSE_MODEL_PATH` and
   `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` are absent from architecture. The plan settles it as an Expected amendment
   for the wrap. Measured at P3: the contract names both at three sites (`:367`, `:442`, `:554`); no file under
   `crates`, `scripts` or `.github` reads or sets either; the registry holds four `ANDROMEDA_PULSE_*` handles and
   neither of these. The playbook already rules this class routine (`.andromeda/playbook.md:137`: an external
   handle Conductor neither sets nor reads, named by a committed contract, is registered). So the lean is to
   register both, not to correct the clause. The seven masters stay read-only in phase and implement.
10. **The supply-chain gate**: clear advisory-db residue or pass a fresh `--db` before reading `cargo audit`. The
    local copy's porcelain reads 0 lines at take-up.

## Folded freight (`working-route.md:94`, three blocks, per `route.py pins`)

- **BLOCKED-ON** (193 chars, folded whole) → §Gate above, cleared.
- **CONTEXT** (816 chars, folded whole): «founder ruling 2026-10-06 (his own word, live, relayed by the overseer;
  inputs I23 of `2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09`) — `v3-09` is neither relaxed nor
  deferred: Pulse is fixed, then this series runs; rejected by him: relaxing the rule to the retry facet, deferring
  `v3-09` at the version close. The fourth series (2026-10-06, Pulse `5f77859`, the shipped
  `gemma-4-E4B-it-Q4_K_M`) graded d1 and d2 `Identified`, d3 `NotIdentified` (its rank 1 names the service only as
  the hyphenated canary identity). Measured at `5f77859`: the digest's `TRIGGER: ` line names the signal and no scope
  (`crates/triage/src/digest/assembler.rs:680`), and prompt `v2.5` obliges rank 1 to name only that signal; the
  service reaches the model on the cue line and the `SERVICES` rows, uninstructed».
  - Re-verified: `I23-conductor-wrap-v309fourth-2026-10-06.md.txt` exists in that chunk's `inputs/`. The matrix's
    2026-10-06 wrap note reads the same grades and the same ruling.
- **CARRY** (311 chars, folded whole) → item 8 above.

## Causal claims carried in (closed at P3, marker kept)

- "Measured at `5f77859`: the digest's `TRIGGER: ` line names the signal and no scope
  (`crates/triage/src/digest/assembler.rs:680`), and prompt `v2.5` obliges rank 1 to name only that signal; the
  service reaches the model on the cue line and the `SERVICES` rows, uninstructed" (the CONTEXT block) — VERIFIED.
  `assembler.rs:680-684` pushes `TRIGGER_LINE_PREFIX` + `cue_cause_label(trigger.kind)` and nothing else
  (inputs#I6); `git diff 5f77859 f70be92 -- crates/triage` is empty; the `v2.5` instruction as the 2026-10-06
  contract section quotes it (`:481-486`) names the signal alone; and the cue line carries the scope
  (`cue_summary`, `assembler.rs:618-623`).
- "Pulse's pre-registered probe reading (240 generations, your `identifies_cause` rule as the grader) read PASS: the
  shipped arm 40/40 on four ordinary storm shapes and 20/20 on two sibling shapes" and "The baseline arm (prompt
  `v2.5`, what your fourth series ran against) read 38/40 and 20/20 on the same probe. So the probe did not
  reproduce the miss your d3 hit, and Pulse's reading does not establish that the sentence fixes it" (inputs#I1 §2,
  marked `measured 2026-10-07 at Pulse 1a2e509`) — VERIFIED in Pulse's own reading (inputs#I8): `arm shipped: bar
  MET · sibling both 20/20 (min 19) · ordinary both 40/40 (min 36)` and `arm ns: bar MET · sibling both 20/20 (min
  19) · ordinary both 38/40 (min 36)`. The baseline's two misses are both `signal_only` on an ordinary shape; its
  sibling half reads 20 of 20, so the probe holds no known-positive for d3's miss. Pulse's attempt to replay d3's
  own digest was not measured (inputs#I9). This series is the only end-to-end reading of the fix.
- "`crates/triage` did not change since `5f77859`, so the digest and every `assembler.rs` coordinate your contract's
  2026-10-06 section pins still hold" (inputs#I1 §1, same marker) — VERIFIED: the diff over `crates/triage` is
  empty, so every line of `assembler.rs` and `contract.rs` is where that section read it.

## Boundaries (out of scope)

- No edit to Pulse's repository. Pulse is read at its committed HEAD (`git show f70be92:<path>`), never its working
  tree. A Pulse build runs in Pulse's tree only inside the overseer's grant.
- No edit to any frozen chunk's `evidence/`, and no rewrite of the posture contract's dated records. The fourth
  series' data dir is not touched.
- No change to the series design or the grading rule: three drives, the rule byte-identical to every prior series'
  recorded rule.
- No widening of security-plan's scoped corpus-text exception: captures enter `evidence/` only through
  `mask_workspace_key` + `redact_value` + `mask_host_paths` + `elide_fingerprints`, digest-pinned, no capture text in
  test source.
- No `v3-09` weakening: every graded drive must be `Identified`, never replaced, the real-model witnesses stand.
- No drive before its pre-registration digest is recorded; no binary proof pre-registered before its two-sided
  control; no retry-until-pass; no drive past the fixed count.
- No GPU run at night, and no launch before the overseer's go.
- The seven spec masters stay read-only.

## CI (Setup 5a)

Both commits from the last wrap's flip through HEAD read green; nothing to fold.

- `29adafa` (the setup `U02` upgrade commit, HEAD): `verdict: green` · checks 3/3 · wall 1106 s · CI#37535326112
  completed/success.
- `fa6a374` (the last wrap's flip commit): `verdict: green` · checks 3/3 · wall 606 s · CI#37534114527
  completed/success.
