# Scope — 2026-10-07-a-sixth-pre-registered-real-model-series-for-v3-09

**Working entry (`working-route.md:98`):** A sixth pre-registered real-model series for `v3-09` — the same three-drive
design and rule, against the Pulse sha that ships the measured remedy.

**Matrix target:** `v3-09` (*Real-model interpretation leg*, `dynamic-external`, status `planned`, `chunk: null` at
take-up — the pool's one unclaimed capability). Its acceptance as it stands names the fifth series by section
(§The 2026-10-07 series), dir (`rm-fifth-series`), Pulse HEAD (`f70be92`) and harvest module
(`real_model_grading/series_2026_10_07.rs`). This chunk re-concretizes it at P5 for the sixth series; nothing of the
pass condition moves. Met iff at least one drive is graded AND every graded drive reads `Identified` (the rank-1
statement names `conductor` as a whole word and a retry token) with the real-model witnesses; a graded
`NotIdentified` is NOT met and is never replaced.

**Chunk base:** `902d12c` (`902d12c84beaf140937eca1b1d08d187462e2dea`), HEAD at take-up. Every diff-shaped gate probe
names it explicitly, because the operator pre-CI commit moves HEAD before the wrap.

**Host:** the Linux dev host (Omarchy), as the fourth and fifth series. Its host-shaped terms are read from the fifth
series' section and re-verified at P3, not re-derived.

## Gate

- gate: cleared — the block reads «BLOCKED-ON: Pulse "The L4 probe reproduces the canary-history miss"
  (andromeda-pulse `andromeda-pulse-0.3.0/working-route.md:180`, minted at Pulse `48714f0`; read there at the
  2026-10-07 wrap, already promoted in Pulse's working tree) — clears when the overseer relays the sha that ships
  it». The sha was relayed (inputs#I2 names `9bfefb8`; inputs#I1 §1), and it was re-read at take-up, not taken on
  the relay's word:
  - Pulse `9bfefb8` (`9bfefb812297bbdea610423a21568b3262bdb7ed`, "operator pre-CI commit" of
    `2026-10-07-l4-probe-reproduces-the-canary-history-miss`) is HEAD of `chore/migrate-pulse-to-v3`, 0 ahead of its
    upstream and contained in `origin/chore/migrate-pulse-to-v3`.
  - CI on that sha: `ci#37668429742`, attempt 2, `completed/success`, 12 of 12 jobs `success`, head sha equal to
    the full sha above (read through `gh run view` at take-up; the run's `updatedAt` reads 2026-10-07T20:27:16Z). The relay says attempt 1 held an unrelated
    boot-smoke death and a coverage job that hung in a runner's apt step (inputs#I1 §1); attempt 1 was not read at
    take-up.
  - `git diff --stat f70be92 9bfefb8 -- crates pulse-app xtask` lists four files: `crates/triage/src/digest/
    assembler.rs`, `crates/triage/src/digest/retrieval.rs` (inputs#I6, inputs#I5) and two probe files under
    `pulse-app/examples/`. So `9bfefb8` differs from the fifth series' `f70be92` in product code.
  - Pulse's master route still reads the record `pending` at `:103` (inputs#I3), and its working route carries the
    stamped entry at `:180` (inputs#I4). The block's clearing event is the relayed sha, not the flip. Pulse's wrap
    was running in its own tree at take-up (an untracked `.andromeda/runs/2026-10-07T20-29-42Z-wrap/`, an untracked
    `report.md`, three modified bookkeeping and evidence files; no build input touched). The relay says that wrap's
    commit lands on top of `9bfefb8` and changes no product file (inputs#I1 §1) — a hypothesis until the diff over
    `crates pulse-app xtask` between `9bfefb8` and the HEAD the build runs at reads empty.
  - Re-read at P5 (2026-10-07, after the plan was written): Pulse's wrap commit landed. HEAD is `f18c631`
    (`f18c631bd545c50466c6177c5ae658dc61f7d2e7`, committed 2026-10-07T22:58:10+02:00), on top of `9bfefb8`, on
    the upstream with 0 unpushed; `git diff --name-only 9bfefb8 HEAD -- crates pulse-app xtask Cargo.toml
    Cargo.lock` prints nothing; the master route now reads the record `complete` at `:103` (inputs#I15). Two
    bookkeeping files were modified in that tree at the read, no build input. This is the agent's own reading;
    the build still waits for the operator's word (inputs#I14), and implement's step 1 re-reads all of it.
- What the cleared block does NOT settle, and this series measures: whether the SHIPPED model's rank-1 statement
  names `conductor` as a whole word in all three drives on the shipped tree. No generation has run on that tree
  (inputs#I1 §1), so a pass is not assumed. No criterion waits behind a standing block and the chunk carries no
  gated mode: the leg runs inside this chunk's implement and its outcome is the acceptance.
- What does wait, and is not a block: the sitting. The GPU is closed tonight; the three drives wait for the next
  daytime go (inputs#I2; inputs#I1 §4).

## What this chunk builds

1. **A new pre-registered series section in the posture contract** (`contracts/pulse-real-model-leg-posture.md`),
   add-only beside §The drive series, §The 2026-09-30 series, §The 2026-10-01 series, §The 2026-10-06 series,
   §The 2026-10-07 series and §The 2026-10-07 capture run (`:213`, `:322`, `:378`, `:461`, `:572`, `:687`),
   rewriting none of them. Fixed BEFORE d1: the drive count, the data dir, the pass condition,
   canary-is-a-measurement, the one uncounted pipeline-fault re-fire, no further drive, the quiet windows, the
   launch posture re-pinned to Pulse `9bfefb8`, the model and prompt versions graded, the slots. Its sha256 is
   recorded in this chunk's `evidence/attempt-ledger.md` before d1 and recomputed by the harvest.
   - **The design and the rule stay byte-identical to the fifth series** (the entry's own words: "the same
     three-drive design and rule"). The terms that move are the ones that name the series: its date and section
     name, its dir leaf, the Pulse sha, the binary proof, the stated limit — and one term that is new, the stop
     rule below.
   - **The section's name.** Prior sections are named by date, and both `2026-10-07` names are taken
     (the fifth series and the capture run). The sitting waits for a daytime go, so the drive date is not known at
     take-up. P4 names the section and its harvest child module so that neither collides. Verified at P3: the
     contract carries `## The 2026-10-07 series` (`:572`) and `## The 2026-10-07 capture run` (`:687`), the harvest
     carries `series_2026_10_07.rs` and `capture_run_2026_10_07.rs`, and `contract_section` finds a section by its
     whole heading line (`real_model_grading/mod.rs:146`), so any heading that differs as a line is safe.
   - **The stop rule is written into the pre-registration BEFORE the launch, so the outcome cannot shape
     it** (inputs#I1 §2, the founder's rulings of 2026-10-07 20:08 local, by dialog, his own picks, relayed by the
     overseer; verified at P3 as relayed — the ruling's content rests on the relay alone, and Pulse's own record
     independently dates a founder dialog relayed by the pc overseer at the same time, inputs#I8): this is the one
     last series. MET if every graded drive reads `Identified`. If it does not pass,
     there is no seventh: `v3-09` then leaves 0.3.0 for 0.4.0, and Conductor 0.3.0 closes at 10 of 11 with that as
     the measured basis and a named owner. His words on what 0.4.0 should judge instead, kept verbatim for that
     owner: «чтоб кондуктор умел спавнить продолжительную серию разных событий и логировать как на них реагирует
     модель, а потом логи анализировать».
   - **The stated limit is restated for Pulse `9bfefb8`.** The prompt is unchanged (`v2.6`, inputs#I13), so both
     facets stay instructed; what moved is the digest's corpus block (item 2). P4 writes the wording; P5 previews
     it. Found at P3, for that wording: under the remedy, on the third captured prompt, 13 of 40 first hypotheses
     still name the sibling beside the right service (inputs#I8). The rule grades such a statement `Identified`
     when `conductor` stands in it as a whole word — the weakness item 9 records and leaves unpatched.
   - The prior sections' Pulse coordinates are dated records and are not rewritten; the new section re-pins what
     it relies on at `9bfefb8`. [premise-corrected: the fifth section names files and symbols, never line numbers
     (`contracts/pulse-real-model-leg-posture.md:587-613`), and each symbol it names exists at `9bfefb8`
     (`TRIGGER_LINE_PREFIX`, `cue_summary`, `render_payload`, read with `git show`). What no longer holds there is
     its clause "Byte-unchanged from `5f77859`: everything under `crates/triage`"; the new section states what is
     unchanged since `f70be92` instead.]
2. **What the series grades** (inputs#I1 §1, each coordinate re-read at P3):
   - The model is unchanged: `gemma-4-E4B-it-Q4_K_M.gguf`, the digest the 2026-10-07 section pins. Measured at
     P3: 4 977 171 584 B, sha256 `85a896a047553e842f25297ee5b031d64ff30147d9c4af17b1e4b394cd1fab87`, equal to
     that pin. Its sha256 is measured again before d1.
   - Prompts `v2.6` / `v1.5-fallback` / `v1.5-reflection`, the argv and the committed GBNF are unchanged since
     `f70be92`. Verified at P3: `git diff --name-only f70be92 9bfefb8 -- crates/interpretation
     crates/workspace-detector crates/security crates/mcp-server pulse-app/src crates/triage/src/contract.rs
     crates/triage/src/cue Cargo.toml Cargo.lock` prints nothing, and the three version constants read as named
     (inputs#I13). So the workspace key, the report render, the skip line and the field allowlist are unchanged
     too.
   - What changed is which corpus rows the digest carries. Verified at P3 in Pulse's own record: on the three
     prompts the capture run recorded, the selection keeps 0 of 1, 1 of 3 and 2 of 5 corpus lines (inputs#I10).
     Read at take-up from the diff:
     `select_corpus_matches` takes a `triggering_scope` (the triggering cue's `scope_id`); with one, the scope arm
     keeps only that scope's own incidents, the fingerprint arm still keeps a match whatever its scope, and the
     narrowing runs before the cap and only removes; with `None` every active scope's matches stay
     (inputs#I5). The assembler passes `triggering_cue.and_then(|c| c.scope_id.as_deref())` (inputs#I6). For the
     scenario the cue carries `scope_id=conductor`, so a `conductor-canary` incident no longer reaches the corpus
     block through the scope arm.
3. **The binaries, and a new binary proof** (inputs#I1 §3).
   - "Pulse's `target/release` holds the `f70be92` builds until you rebuild" (inputs#I1 §3). Verified at P3 by
     digest: `pulse-app` hashes `df1676477222776d3e95edae7d219a4d421f2311ea8f17863233630c1ed8ba4a` and the sidecar
     `6175fc36be6577b195470f136a124fe8690e4029745f3651ab46ce19043ad2a9`, the two digests the fifth series' ledger
     records for its rebuilt binaries (`attempt-ledger.md:61-62`) and the capture-run section holds (`:721-722`).
     Both are rebuilt before d1, from a CLEAN Pulse tree at the pinned sha.
   - **The fifth series' string set no longer proves the build**, since the `f70be92` binary already carries it.
     The relay asks for a content proof this change introduces, with the two-sided control taken on both builds
     before it is pre-registered (inputs#I1 §3). [premise-corrected: no named content token discriminates the two
     builds. The non-test product change is one parameter, one closure condition, one call-site argument and
     comments, so it adds no string literal (inputs#I5, inputs#I6); and Pulse's release profile sets `strip = true`
     (inputs#I11), so neither on-disk binary carries a symbol table (`nm` lists 0 symbols on each). What a build at
     `9bfefb8` can show is that the binary's bytes moved under a build of the pinned tree: its sha256 against the
     recorded `f70be92` digest, with build provenance. That replaces the acceptance's "proven by content, by a
     string set" term, so it is a fork the plan asks, never a silently weaker term.] Answered by the operator at
     P4 (inputs#I14): the section carries the moved digest with its provenance and states plainly that no named
     token exists; no determinism control (it would delete inside Pulse's build cache) and no marker commit (it
     would break the pin the remedy was measured against).
   - The sidecar. Verified at P3: `andromeda-pulse-mcp` links `crates/triage` and `crates/interpretation` by path
     (inputs#I12), so a build at `9bfefb8` recompiles it over the moved digest module; its own crate is unchanged
     since `f70be92`. Whether its digest moves is not knowable before the rebuild: at the fifth series it did not
     move over a recompiled dependency. Its proof stays build provenance with both digests recorded, as the fifth
     series' was.
   - The agent builds both binaries ("until you rebuild; build from a CLEAN Pulse tree at the pinned sha",
     inputs#I1 §3), and launches and stops `pulse-app` inside the overseer's grant, as in the fourth and fifth
     series. The build is CPU work and belongs to tonight's part (inputs#I2; inputs#I1 §4).
   - **The build waits for the operator's word that Pulse's wrap is committed** (inputs#I14): Pulse's builder is
     inside its wrap in that tree, so the tree is dirty until the wrap commit lands on top of `9bfefb8`. The
     build then runs from a tree whose product inputs equal `9bfefb8`.
4. **The pre-leg checks**, as in the fifth series: the model env is present, the model's sha256 equals the pinned
   digest, the NVIDIA driver major equals the runtime's. Recorded in the attempt ledger before d1.
   - **The pass-through is NOT used** (inputs#I1 §3; inputs#I2): the launch sources the plain `l4-env.sh`
     (inputs#I7), never `l4-env-capture.sh`. Nothing records prompts in this series. Read at P3: the plain file
     exports four handles — the model path, the CUDA and CPU runner paths and the build's local tokenizer path —
     and leaves `ANDROMEDA_PULSE_L4_DETERMINISTIC` unset, which is what the real-model posture's absence term
     needs of the launching shell.
5. **A fresh home-rooted data dir, a NEW leaf** under `~/.cache/pulse-legs/` (inputs#I1 §3). The leaf carries no
   digit, no `@`, no vendor prefix, no keyed scrubber word and neither word of the grading rule; its absence before
   creation is recorded. The three earlier leaves there at take-up — `rm-trigger-series`, `rm-fifth-series`,
   `rm-recorded-run` — stay untouched as evidence. P4 names the leaf.
6. **The series**, d1..d3, each through `bash scripts/agent-run.sh run --live real-model` against a pulse-app
   launched per the posture contract. Every drive recorded: capture committed under this chunk's `evidence/`,
   sha256-pinned in the series module, graded from the file by the harvest, one row in `evidence/attempt-ledger.md`
   with its workspace-key rendering witness and Pulse's `skip_reason` for every digest it did not surface.
   - **Order (inputs#I2; inputs#I1 §4):** the phase and Part A are CPU work for tonight; the sitting waits for
     daytime and the go. The plan splits implement there: everything the leg reads (the contract section and its
     recorded digest, the binaries and their proof, the dir) is done first and may run at any hour; the three
     drives are one sitting, reached only after the go. Amended at P5's plan-versus-scope read: the env check is
     fired in the sitting's own shell, still before d1, because Part A and the sitting fall on different days here
     and a reboot between them could move the driver reading.
   - **The go is asked before the launch.** A sitting that would run into the night is not started. No bound and
     no hurry: the founder asked for a measured pace (inputs#I1 §4).
   - The round's point-in-time steps are keyed `leg = 'round'` in the plan's gate fence, as in the fifth series.
7. **The verdict test and the ref.** A harvest test states the series verdict over the pinned captures, as each
   prior series has one. The separate ref test asserting `Identified` plus the real-model witnesses is written ONLY
   if the pass condition holds. A `NotIdentified` or a zero-graded series is recorded as NOT MET, never replaced.
   - A pass is read as three drives, no more. The report states it beside the verdict either way.
8. **What each outcome hands the wrap** (inputs#I1 §2; verified at P3 as relayed). MET: the ref lands and `v3-09` reads verified.
   NOT MET: recorded by the pre-registered stop rule — no seventh series is minted, `v3-09` leaves 0.3.0 for 0.4.0
   with a named owner. Either way the next entry is the version close (`working-route.md:100`): "Conductor closes
   first". The deferral and its owner are the wrap's and the version close's acts on the ledger and the route; this
   chunk's phase and implement mint no entry and write no deferral.
   - This supersedes, for the not-met case only, the founder's earlier rulings that `v3-09` is neither relaxed nor
     deferred (2026-10-06) and that nothing planned for 0.3.0 is carried over (2026-10-02), both recorded in the
     matrix notes. The later ruling is his own and is recorded as such.
9. **The grading rule is not changed** (inputs#I1 §2). The entry's open question — should the rank-1 rule reject a
   statement that quotes the cue's `scope_id` value yet places the storm in the canary service — is closed for
   0.3.0 as "unchanged": the overseer's decision, told to the founder before his picks, not objected to, his to
   overrule. The weakness is recorded for 0.4.0 (a structured answer field or a semantic grader), not patched
   here. The rule stays byte-identical to every prior series' recorded rule.
10. **The architecture byte threshold** (the entry's CARRY; inputs#I1 §3). §Occupied Resources stood at 38114 of
    38115 B and §Established Decisions at 38068 B at the capture run's wrap. Any Expected amendment that grows
    either first frees bytes there. Re-measured at P3 (`python -X utf8 scripts/arch-registry-check.py measure
    --file .andromeda/architecture.md`): §Occupied Resources 38114 B, §Established Decisions 38068 B, threshold
    38115 B — unchanged. The seven masters stay read-only in phase and implement.
11. **The supply-chain gate**: clear advisory-db residue or pass a fresh `--db` before reading `cargo audit`. The
    local copy's porcelain reads 0 lines at take-up.

## Folded freight (`working-route.md:98`, five blocks, per `route.py pins`)

- **BLOCKED-ON** (290 chars, folded whole) → §Gate above, cleared.
- **CONTEXT** (909 chars, folded whole): «founder ruling 2026-10-07 11:49 local (his own pick of the option titled
  "reproduce and fix" in a question dialog; relayed by the overseer, its content in summary) — a Pulse chunk first
  reproduces the miss in its probe, then measures a remedy, then this series runs; relax, defer and redesign were
  not taken. The fifth series (2026-10-07, Pulse `f70be92`, prompt `v2.6`) graded d1 and d2 `Identified`, d3
  `NotIdentified` — the same split on the same drive as the fourth (2026-10-06, `5f77859`, `v2.5`). Measured across
  both series, a covariate and not a cause: the miss is the drive whose creating digest retrieved 6 corpus rows (2
  of 2) and the 1- and 3-row drives pass (4 of 4), while the row count rises with the drive ordinal, the app's
  uptime and the canary's history, which no drive varied apart
  (`conductor-0.3.0/chunks/2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09/report.md`)».
  - Re-verified: that report exists in the fifth series' chunk folder, and the matrix's 2026-10-07 wrap note reads
    the same grades, the same covariate and the same ruling.
- **CARRY** (437 chars, folded whole) → item 10: «architecture §Occupied Resources stands at its byte threshold —
  38114 of 38115 B, and §Established Decisions at 38068 B, `scripts/arch-registry-check.py measure` at the capture
  run's wrap (2026-10-07) — so an amendment that grows either first frees bytes there; the posture-contract entry
  now names the record kinds and holds the series dates in the sidecar (origin
  `2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09`)».
- **CARRY** (357 chars, folded whole), a record this chunk does not act on: «the 2026-10-07 capture run (the fifth
  series' Pulse, prompt and design; a capture, no `v3-09` verdict) read d1, d2 and d3 `Identified` at 1, 3 and 6
  corpus rows, so the d3 position stands at 2 misses of 3 live runs — a rate, not a cause
  (`conductor-0.3.0/chunks/2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive/report.md`)».
  The capture run's facts stand as recorded: its drives were observations; this series is the verdict
  (inputs#I1 §3).
- **CARRY** (574 chars, folded for its whole substance) → item 9, closed as "unchanged": an open question for the
  founder before this series is pre-registered — should the rank-1 rule reject a statement that quotes the cue's
  `scope_id` value yet places the storm in the canary service? It was observed at that capture run's d2, whose
  rank-1 statement the rule as written grades `Identified` (the whole word `conductor` and a retry token are both
  present); no rule changed at that wrap (delegate pc overseer, 2026-10-07, the capture run's wrap directive;
  that chunk's `evidence/attempt-ledger.md`, Grading).
  - The block on the route line quotes that rank-1 statement. It is model text read from the corpus, so it is
    not repeated here: security-plan's scoped exception admits such text to a chunk's `evidence/` tree only.
    The statement stands in that capture and its ledger. Whether the one-sentence quote on the route line is
    inside that exception is not this chunk's to rule; it is named on the P5 review card.

## Causal claims carried in (closed at P3, marker kept; the closure is `research.md` §Scope premise closure)

- "Measured across both series, a covariate and not a cause: the miss is the drive whose creating digest
  retrieved 6 corpus rows (2 of 2) and the 1- and 3-row drives pass (4 of 4), while the row count rises with the
  drive ordinal, the app's uptime and the canary's history, which no drive varied apart" (the CONTEXT block) —
  VERIFIED against the fifth series' ledger table (`attempt-ledger.md:249-262`: rows 1, 3, 6 in both series, the
  6-row drive `NotIdentified` both times). The capture run's three captures print 1, 3 and 6 rows too
  (`rm-capture-d1.txt:529`, `-d2.txt:548`, `-d3.txt:579`) and all three graded `Identified`, which is the CARRY's
  "2 misses of 3 live runs" at that position.
- "on a digest whose cue carries a `scope_id`, the corpus block keeps that scope's own matches and
  whatever the fingerprint arm keeps, and drops other scopes'. A digest with no cue, or a cue with no `scope_id`,
  selects as before. The prompt lineage is unchanged (`v2.6`); the trigger line still carries the kind label alone;
  the model file, the argv and the grammar are unchanged." (inputs#I1 §1, marked `measured 2026-10-07`) —
  VERIFIED: the selection in inputs#I5 and inputs#I6, the versions in inputs#I13, the rest by the empty diff
  named in item 2 and the model digest measured there.
- "replays of the three scenario prompts your capture run recorded, through Pulse's probe. Before the
  remedy the three read 36 of 40, 11 of 20 and 36 of 40 first hypotheses that name the right service; with the
  remedy 40, 40 and 40 of 40. One capture run, three prompts, measured on the probe's arm, which composes byte for
  byte what the product path composes on those prompts. **No generation has run on the shipped tree: your series is
  its first live reading.**" (inputs#I1 §1, marked `measured 2026-10-07`) — VERIFIED in part, one figure
  corrected. [premise-corrected: for the second prompt the relay's "11 of 20" is the MISS count. Pulse's replay
  reading records 11 service misses of 20 under the shipped arm, so 9 of 20 named the right service (inputs#I9).
  The first and third prompts read 4 misses of 40 each, 36 of 40 right; under the remedy all three read 0 misses
  of 40 (inputs#I8).] The byte-for-byte composition holds on all three prompts by a dry run, with two things
  taken as given that no capture holds (inputs#I10). No generation ran on the shipped tree, in Pulse's own words
  (inputs#I8, inputs#I10).
  - Found at P3 and not in the relay: the remedy that shipped, `CX`, is the founder's choice of 20:08. Pulse's
    own pre-registered order had selected another candidate, `CO`, and that selection stands as measured
    (inputs#I8).
  - Found at P3: no captured drive missed live, so Pulse's known-positive is the second drive's PROMPT replayed,
    not a live miss; its record says the reading "does not show why the third drive missed live in two series"
    (inputs#I9).
- Pulse's own sentence in the shipped doc comment: "A digest that listed another service's incidents led
  the L4 model to place the signal there" (inputs#I5) — NOT VERIFIED as a mechanism, and not relied on. Pulse's
  evidence is counts on three prompts of one capture run, and its record names what those counts do not show
  (inputs#I8, inputs#I9). The series neither assumes nor tests this mechanism; it reads the outcome on three
  drives.

## Boundaries (out of scope)

- No edit to Pulse's repository. Pulse is read at its committed sha (`git show 9bfefb8:<path>`), never its working
  tree. A Pulse build runs in Pulse's tree only inside the overseer's grant, and only from a clean tree.
- No edit to any frozen chunk's `evidence/`, and no rewrite of the posture contract's dated records. The three
  earlier data dirs are not touched.
- No change to the series design or the grading rule: three drives, the rule byte-identical to every prior series'
  recorded rule.
- No prompt recording: the pass-through env file is not sourced.
- No widening of security-plan's scoped corpus-text exception: captures enter `evidence/` only through
  `mask_workspace_key` + `redact_value` + `mask_host_paths` + `elide_fingerprints`, digest-pinned, no capture text
  in test source.
- No `v3-09` weakening: every graded drive must be `Identified`, never replaced, the real-model witnesses stand.
- No drive before its pre-registration digest is recorded; no stop rule written after the launch; no binary proof
  pre-registered before its two-sided control; no retry-until-pass; no drive past the fixed count; no seventh
  series whatever this one reads.
- No GPU run at night, and no launch before the go.
- No route entry minted and no deferral written by phase or implement.
- The seven spec masters stay read-only.

## Review

The plan and the `v3-09` concretization were approved at the P5 review by the operator's word (inputs#I16), after
the two plan forks were answered (inputs#I14). The capability is claimed by this chunk in the ledger, ungated.

## CI (Setup 5a)

Three commits from the last wrap's flip through HEAD. Two read green; one reads `not green`.

- `902d12c` (the 0-pending correction wrap, HEAD): `verdict: green` · checks 3/3 · wall 578 s · CI#37645033497
  completed/success.
- `717bbf3` (the setup `U04` upgrade commit): `verdict: not green` · checks 3/3 · wall 564 s · CI#37643984555
  completed/cancelled. Two checks read `cancelled` — "A11y gate (routine arm · axe · contrast · violation JSON)" and
  "Rust gate (build · test · lint · supply-chain · coverage)"; "Frontend gate (npm audit · build)" read `success`.
  No check read `failure`.
  - Measured cause: `.github/workflows/ci.yml:11-13` sets `concurrency: group: ci-${{ github.ref }}` with
    `cancel-in-progress: true`. The run was created 15:26:49Z and settled `cancelled` at 15:36:18Z; CI#37645033497
    on `902d12c`, the same ref, was created 15:34:44Z. That later run read 3/3 green over a tree that contains
    `717bbf3`'s change, and `ci.yml` carries no `paths` filter, so no job there is scoped to its own commit's diff.
  - Disposition: not folded into this chunk, and not read as green. It has no failing subject, so nothing of it
    intersects what this chunk builds. It is recorded here as read and unowned. Asked as a plan fork at P4 rather
    than as a halt at take-up, and answered so by the operator (inputs#I14).
- `920a1e7` (the last wrap's flip commit): `verdict: green` · checks 3/3 · wall 683 s · CI#37629834826
  completed/success.
