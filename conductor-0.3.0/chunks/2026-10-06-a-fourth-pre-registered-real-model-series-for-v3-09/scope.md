# Scope — 2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09

**Working entry (`working-route.md:92`):** A fourth pre-registered real-model series for `v3-09` — interpretation
re-proven once Pulse's retry-storm interpretation names its retry cause.

**Matrix target:** `v3-09` (*Real-model interpretation leg*, `dynamic-external`, status `deferred`, `chunk: null` at
take-up). Its acceptance as it stands names the 2026-10-01 series by section, dir (`rm-surfacing-series`) and Pulse
HEAD (`a2addb3`): three drives on one fresh letters-only data dir, both binaries proven before d1, the booted posture
confirmed from Pulse's log, a pre-registration sha256 recomputed by the harvest, digest-pinned captures graded from
their files, one uncounted pipeline-fault re-fire, no fourth drive. Met iff at least one drive is graded AND every
graded drive reads `Identified` (the rank-1 statement names `conductor` as a whole word and a retry token) with the
real-model witnesses; a graded `NotIdentified` is NOT met and never replaced. The status `deferred` stands against
the founder's NOT-deferred ruling (handoff note; the CONTEXT block below) and is reconciled by this chunk at P5
(`matrix.py undefer`, then the claim).

**Chunk base:** `0b07b2c` (`0b07b2c475576b523ecc467c9a6ffec36e91216c`), HEAD at take-up. Every diff-shaped gate probe
names it explicitly, because the operator pre-CI commit moves HEAD before the wrap.

**Host:** the Linux dev host (Omarchy). The three prior series ran on the Windows host. Every host-shaped term the
prior series fixed has a Linux form, read from the 2026-10-03 P-075 re-round and from the tree (research.md §The
host, §Patterns detected): binaries without `.exe`, a `ps` census over `pulse-app|andromeda-pulse|conductor|WebKit`,
`kill -TERM` by the recorded PID, Pulse's log at `logs/agent-latest.jsonl.<date>` under the data dir.
[premise-corrected: the re-round's `WEBKIT_DISABLE_DMABUF_RENDERER=1` is NOT part of this launch — a `pulse-app`
built at `5f77859` sets `__NV_DISABLE_EXPLICIT_SYNC` itself on Linux (`render_posture.rs:50-59`, `main.rs:279`,
inputs#I19), a fix that landed 2026-10-04, after the re-round. On the overseer's review (inputs#I18) the WebKit lever
is one recorded relaunch, only if the app dies in its first 10 s.] `scripts/agent-run.sh` and the capture's path guards carry no
host assumption that needs an edit.

## Gate

- gate: cleared — the block reads «Pulse "retry-storm interpretation names its retry cause" (the founder's wording) —
  clears when Pulse relays a sha shipping it». Pulse `5f77859` (HEAD of `chore/migrate-pulse-to-v3`, level with its
  upstream, read at take-up) holds `2026-10-04-retry-storm-interpretation-names-its-cause · complete` in its master
  route (inputs#I3), relayed by the overseer (inputs#I1 §1). The clearing event the line names has happened.
- What the cleared block does NOT settle, and this series measures: whether the SHIPPED model's rank-1 hypothesis
  names the retry cause. The directive words this "its block as the acceptance gate" (inputs#I2). So no criterion is
  deferred behind a standing block and the chunk carries no gated mode: the leg runs inside this chunk's implement,
  and its outcome is the acceptance. A pass is not assumed (inputs#I1 §1).
- The two wordings the handoff asked to reconcile: the route line's wording is the Pulse chunk named above; the
  overseer's wrap note "Pulse's prompt-framing entry and the restored model" is Pulse's TRIGGER-framing chunk
  (`2026-10-04-l4-interpretation-names-its-triggering-cue`, complete at `5f77859`, inputs#I3) plus a settled model,
  where "the restored model" now reads "the shipped model" because the restored Llama was replaced (inputs#I1 §1).

## What this chunk builds

1. **A new pre-registered series section in the posture contract** (`contracts/pulse-real-model-leg-posture.md`),
   add-only beside §The drive series (2026-09-29), §The 2026-09-30 series and §The 2026-10-01 series, rewriting none
   of them. Fixed BEFORE d1: the drive count, the data dir, the pass condition (§The drive series (a), unchanged),
   canary-is-a-measurement, the one uncounted pipeline-fault re-fire, no further drive, the quiet windows, the launch
   posture re-pinned to Pulse `5f77859`, the model and prompt versions graded, the slots. Its sha256 is recorded in
   this chunk's `evidence/attempt-ledger.md` before d1 and recomputed by the harvest.
   - The count stays three on the 2026-10-01 design. No research finding moves it; it is P4's stated lean, ratified
     by the overseer at P5.
   - The GRADING RULE stays byte-identical to every prior series' recorded rule. Verified at `5f77859`: the report
     render and `schema.json` are byte-unchanged from `a2addb3`, `L4Output` keeps its hypothesis fields, and the
     grammar-file generation emits the same keys (research.md §Pulse at `5f77859`). A rule edit would also fail every
     prior series' `drives_recorded_the_current_rule` arm.
   - **How the rule reads the `TRIGGER: ` line** (inputs#I1 §3). Verified (inputs#I4, inputs#I9, inputs#I10): the
     scenario's digest carries `TRIGGER: Retry storm` under `OVERALL` — the cause label alone, with no scope — and
     the v2.5 prompt obliges the first hypothesis statement to name that signal in the line's own words. The cue line
     `[autonomous] retry_storm — retry_storm scope_id=conductor` and the `SERVICES` rows still carry `conductor`. So
     the rule's retry facet is now instructed and its `conductor` facet is not. P4 decides the reading and the new
     section states it before d1.
   - The prior sections' Pulse coordinates that moved (contract `:148-149`, `:257`, `:392`, inputs#I1 §3) are dated
     records and are not rewritten; the new section re-pins what it relies on at `5f77859`.
2. **What the series grades** (inputs#I1 §2, each coordinate re-read):
   - Model `gemma-4-E4B-it-Q4_K_M.gguf`, 4 977 171 584 B, sha256
     `85a896a047553e842f25297ee5b031d64ff30147d9c4af17b1e4b394cd1fab87` (measured at take-up; it matches the relay).
   - Prompts `v2.5` / `v1.4-fallback` / `v1.4-reflection` (`schema.rs:34`, `:42`, `:55`, inputs#I5).
   - argv `-c 8192`, `-rea off`, `--temp 1.0 --top-p 0.95 --top-k 64 --min-p 0`, `--grammar-file` with the embedded
     GBNF (`llamacli_inference.rs:114-134`, `:462-490`, inputs#I6; the grammar is inputs#I12).
   - No Pulse artifact reads the shipped model on retry naming: the relay's search, re-run at `5f77859`, returns 4
     lines, all gemma-4-E2B (research.md §Pulse at `5f77859`). This series is the first reading. Pulse's own bar
     read `FAIL · rank1 34/40` against 36 on the baseline Llama 3.2 3B (inputs#I7 `:113`, `:93`, `:182`).
3. **The binaries.** [premise-corrected: the binaries on this host PREDATE `5f77859` — `pulse-app` 2026-10-04 00:12
   and the sidecar 2026-10-03 23:52 against a commit of 2026-10-05 18:22 — so both are rebuilt before d1.]
   `pulse-app` is proven by content: `--grammar-file` present and `--json-schema-file` absent (the stale binary
   reads 0 and 1). The sidecar has no discriminating string, so its content reading is recorded first and its proof
   is build provenance (research.md §The binaries on this host).
   - Slot (inputs#I2): the model slot and 4317/4318 are granted for this chunk; pulse-builder stays idle. Asked at
     P4 and answered by the overseer (inputs#I17): the grant covers both release builds in the Pulse tree and the
     launch, and the AGENT builds, launches and stops `pulse-app`.
   - Pulse's tree at take-up: 3 modified + 1 untracked path, all wrap bookkeeping; no build input is touched.
4. **The pre-leg checks** (carried pre-directions, inputs#I1 §4): the model env is present (`. …/l4-env.sh`,
   inputs#I8 — `ANDROMEDA_PULSE_MODEL_PATH`, `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`), and the NVIDIA driver major
   equals the runtime's major (both `610` at take-up). Recorded in the attempt ledger before d1.
5. **A fresh letters-only data dir** — a NEW leaf (never `rm-clean-series` or `rm-surfacing-series`), carrying no
   digit, no `@`, no vendor prefix and no keyed scrubber word; its absence before creation recorded.
   [premise-corrected: the parent is not a free choice on this host. The capture's scrub chain masks a home-rooted
   path at two stages and a temp-rooted one at none (`redact.rs:130-136`; `real_model_common/mod.rs:346`), and the
   report's `## Previously Seen` suffix carries the workspace path — so a temp-rooted data dir would commit its
   parent into a capture, which `gate.py hygiene` then refuses.] Asked at P4 and answered by the overseer
   (inputs#I17): the dir is HOME-ROOTED — `pulse-legs/rm-trigger-series` under the user's cache dir.
6. **The series**, d1..d3, each through `bash scripts/agent-run.sh run --live real-model` against a pulse-app launched
   per the posture contract. Every drive recorded: capture committed under this chunk's `evidence/`, sha256-pinned in
   the series module, graded from the file by the harvest, one row in `evidence/attempt-ledger.md` with its
   workspace-key rendering witness and Pulse's `skip_reason` for every digest it did not surface.
   - **Order (inputs#I2; inputs#I1 §2):** the GPU leg runs TONIGHT before the founder sleeps, or tomorrow in
     daytime, never at night. The plan orders implement so the leg starts as early as its inputs allow: only what
     the leg reads (the contract section and its recorded digest, the capture tool as committed, the binaries, the
     env check, the dir) precedes d1. Everything else follows the drives.
   - **Only GPU runs are barred at night.** [premise-corrected: the directive's "no heavy runs at night"
     (inputs#I2) was corrected by the overseer on the founder's word the same evening (inputs#I17): only the model
     drives are barred at night; cargo builds, tests and the pre-push gate may run at any hour, so nothing but the
     drives is deferred.] No mutation run is owed either way: test-plan §9 runs that instrument at the
     epoch-boundary code audit only (tests-history 2026-09-30-mutation-gate-grades-every-tally-it-rests-on).
   - The round's point-in-time steps are keyed `leg = 'round'` in the plan's gate fence (inputs#I1 §4).
7. **The verdict test and the ref.** A harvest test states the series verdict over the pinned captures, as each prior
   series has one. The separate ref test asserting `Identified` plus the real-model witnesses is written ONLY if the
   pass condition holds. A `NotIdentified` or a zero-graded series is recorded as NOT MET, never replaced.
8. **The supply-chain gate** (inputs#I1 §4): clear advisory-db residue or pass a fresh `--db` before reading
   `cargo audit`. The local copy's porcelain reads 0 lines at take-up.
9. **The walks-tree marks** (inputs#I1 §5; 0 in Conductor today — `grep -rl 'andromeda:walks-tree' crates scripts`
   returns 0 files, measured at take-up). Tests that read files no name grep finds take the comment token
   `andromeda:walks-tree`, once each. The relay's seven candidates were read (research.md §Walks-tree candidates):
   four walk the tree directly, three walk it through the scenario catalog loader, and one unnamed candidate
   (`real_model_harvest.rs:1360`) lists a directory. Implement confirms each before marking. The token's reader is
   `gate.py run`'s header line. Not an input to the leg, so it follows the drives.
10. **Any architecture draft** this chunk produces is measured with `scripts/arch-registry-check.py measure`
    (inputs#I1 §4) — applies only where the plan has such a step. The chunk drafts none: the posture row's move is a
    wrap amendment.
11. **The temp-rooted gap in the capture's host-path mask** (found by research, not on the entry): `mask_host_paths`
    gains the temp-rooted forms with their arms, after the drives. Ratified by the overseer at P4 (inputs#I17: "Fix
    the temp-rooted gap in this chunk after the drives").

## Folded freight (`working-route.md:92`, two blocks, per `route.py pins`)

- **BLOCKED-ON** (137 chars, folded whole): «BLOCKED-ON: Pulse "retry-storm interpretation names its retry cause"
  (the founder's wording) — clears when Pulse relays a sha shipping it» → §Gate above, cleared.
- **CONTEXT** (324 chars, folded whole): «founder ruling 2026-10-02 (as above), entry 3 of 3 — `v3-09` is NOT
  deferred; no series has met it yet (the third, 2026-10-01 at Pulse `a2addb3`, graded d1 `Identified` and d3
  `NotIdentified`, its d2 spans refused for an identity replay, since repaid by
  `2026-10-01-per-run-span-identity-in-the-real-model-harness`)».
  - Re-verified: "as above" is the ruling archived with entry 1 of 3 (`route-archive.md`, «Так все что планировали на
    0.3.0 делаем сразу как положено, ничего не переносим», founder, live, relayed by the overseer). The matrix's
    2026-10-01 wrap note reads the same grades (d1 `Identified`, d3 `NotIdentified`, d2 not graded,
    `append_failed` ×36). `2026-10-01-per-run-span-identity-in-the-real-model-harness` is a chunk folder in this
    version.

## Causal claims carried in (closed at P3, marker kept)

- "its d2 spans refused for an identity replay, since repaid by
  `2026-10-01-per-run-span-identity-in-the-real-model-harness`" (the CONTEXT block) — VERIFIED at HEAD:
  `execute_scenario` passes `Some(emitted_ms as u64)` as the dispatcher's identity salt
  (`crates/conductor-run/src/execute.rs:88-93`), so a second same-seed drive on one data dir replays no span id.
- "the restored Llama was then REPLACED by the founder's pick" and "Generation is no longer constrained through
  `--json-schema-file`" (inputs#I1 §1-§2, "measured 2026-10-06 at Pulse 5f77859") — VERIFIED: the argv carries
  `--grammar-file` (`llamacli_inference.rs:489`, inputs#I6) and the old flag survives only in a doc comment; the
  grammar's root keys are `L4Output`'s (inputs#I12). Which model actually loads is read from Pulse's own
  `interpretation.model.load` line before d1.
- "Pulse's own rank-1 bar read `FAIL · rank1 34/40` (bar 36) on the previous model" (inputs#I1 §1, "measured") —
  VERIFIED in the same report: `:113` the reading, `:93` "the pre-registered bar of 36", `:182` the baseline
  Llama 3.2 3B (inputs#I7).

## Boundaries (out of scope)

- No edit to Pulse's repository. Pulse is read at its committed HEAD (`git show 5f77859:<path>`), never its working
  tree. A Pulse build runs in Pulse's tree only on the overseer's word.
- No edit to any frozen chunk's `evidence/`, and no rewrite of the posture contract's dated records.
- No widening of security-plan's scoped corpus-text exception: captures enter `evidence/` only through
  `mask_workspace_key` + `redact_value` + `mask_host_paths` + `elide_fingerprints`, digest-pinned, no capture text in
  test source.
- No `v3-09` weakening: every graded drive must be `Identified`, never replaced, the real-model witnesses stand.
- No drive before its pre-registration digest is recorded; no retry-until-pass; no drive past the fixed count.
- No GPU run at night (inputs#I17).
- The seven spec masters stay read-only.
- The `U02` setup re-run waits for the seam after this chunk's wrap (inputs#I1 §6, inputs#I2).

## CI (Setup 5a)

- `0b07b2c` (the last wrap's flip commit, and the only sha from the flip through HEAD): `verdict: green` · checks 3/3
  · wall 698 s · CI#37211238237 push completed/success. Nothing to fold.
