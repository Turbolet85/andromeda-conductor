# Report — 2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive

**Chunk:** a three-drive real-model capture run under the operator's recording pass-through: the leg discloses the
posture and gives each drive's instants, grades are observations, not a `v3-09` verdict
**Date:** 2026-10-07
**Commits:** `15fab57` `chore(…): operator pre-CI commit, for the run this chunk's verdict reads` (the operator's
act, performed by the agent on the overseer's explicit word; basis `git log 6987764..HEAD`, 1 commit)

## Changes (structured — detectors read this)
- **Files** (basis: `git diff --numstat 6987764 -- contracts crates`, 6 rows):
  - `contracts/pulse-real-model-leg-posture.md` — 74 added, 0 deleted: one new section `## The 2026-10-07 capture
    run`, between `## The 2026-10-07 series` and `## The quiet window and serialization`.
  - `crates/conductor-run/tests/real_model_grading/capture_run_2026_10_07.rs` — new, 109 lines, test code.
  - `crates/conductor-run/tests/real_model_series/mod.rs` — 24 added: the run's evidence path and three pins.
  - `crates/conductor-run/tests/real_model_grading/mod.rs` — 1 added: the module line.
  - `crates/conductor-run/tests/real_model_harvest.rs` — 15 added, 4 deleted: four registration sites; the bytes
    between the rule markers are unchanged (the rule diff against `6987764` prints nothing).
  - `crates/conductor-run/tests/real_model_grading/capture_population.rs` — 1 line: the population count.
  - New committed evidence under this chunk's `evidence/`: `attempt-ledger.md`, `rm-capture-d1.txt`, `-d2.txt`,
    `-d3.txt`, and three round listings (`round-122911Z.txt`, `round-123540Z.txt`, `round-124504Z.txt`).
  - No file under `crates/*/src`, `scripts/`, `scenarios/`, `.github/`, no manifest and no lockfile changed (the
    frozen-path diff against `6987764` prints nothing).
- **Symbols / APIs:** test-only, all inside the `real_model_harvest` test target: `pub(crate) fn
  capture_run_2026_10_07(&Drive) -> String`; consts `EVIDENCE_CAPTURE_RUN_2026_10_07` and
  `CAPTURE_RUN_2026_10_07: [Drive; 3]`; six tests named `…2026_10_07_capture_run…`. No shipped symbol, verb, flag,
  selector, IPC method, port, socket or span changed. `pre_registered` gains a fifth calling test; `v3_09_met`
  gains none (the module does not import it and holds 0 occurrences of `v3_09`).
  - **Environment handles — no shipped artifact names a new one.** The contract's handle census reads 6 before and
    after (`grep -oE '(ANDROMEDA|CONDUCTOR|PC)_[A-Z0-9_]+' contracts/pulse-real-model-leg-posture.md | sort -u`).
    The name sweep over `crates scripts .github` for a model handle, the operator's variable, the pass-through,
    its folder or either env file reads 0. Three names appear ONLY in this chunk's own folder (`plan.md`, the
    attempt ledger), never in a shipped artifact: `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH`,
    `ANDROMEDA_LLAMA3_TOKENIZER_PATH` (both also named in the fifth series' ledger) and the operator's own
    variable `PC_L4_REAL_LLAMA_BIN` (new in this chunk's folder). Conductor sets and reads none of them.
- **Crates / modules:** one test child module added (`real_model_grading/capture_run_2026_10_07.rs`); no crate
  added, removed or re-edged.
- **Dependencies:** none. Package count 562 before and after (`grep -c "^name = " Cargo.lock`).
- **Schema / config:** none. The capture's printed line classes, the scrub chain, the envelope and the journal
  are unchanged; the grading rule is byte-identical.
- **Spec-master edits:** none (implement edits no master).
- **Counts / qualifiers moved:**
  - Dated pre-registered records in the posture contract: five series (2026-09-29, 2026-09-30, 2026-10-01,
    2026-10-06, 2026-10-07) → the same five series **plus one capture run that is not a series** (`grep -n '^## The
    20' contracts/pulse-real-model-leg-posture.md`: 5 headings, the last two both dated 2026-10-07). Sites that
    enumerate the series or name the 2026-10-07 series as the latest record (`fifth series|five series|2026-10-07
    series` over the seven masters, `.andromeda/registries/`, `CLAUDE.md`, `.claude/rules`, `.claude/docs`):
    `security-plan.md` 2 lines (`:121` ×3, `:338`), `test-plan.md` 1 (`:260`), `obs-plan.md` 1 (`:221`),
    `.claude/docs/tests-summary.md` 1 (`:22`); `architecture.md:184` enumerates the series by date
    (`pulse-real-model-leg-posture`: `architecture.md` 2 lines `:184`, `:204`; `test-plan.md` 3 lines `:124`,
    `:260`, `:391`; registries 0).
  - Committed real-model captures: 20 → 23 (`COMMITTED_CAPTURES`; 0 hits for the old value's forms in the
    masters, registries, rules and docs).
  - `real_model_harvest` tests: 119 → 125; workspace nextest: 1219 → 1225; grading modules scanned by the
    capture-text arm: 12 → 13 (`\b119\b`, `\b1219\b`, `GRADING_MODULES|twelve`: 0 hits each in the same file
    set).
  - `cargo audit` at this chunk's gate: 1293 advisories loaded (the fifth series' wrap read 1290), 562 crate
    dependencies, 7 allowed warnings (6 `unmaintained`, 1 `unsound`) — external advisory-database movement over a
    byte-unchanged lockfile (`\b1290\b`: 0 hits in the same file set).
  - Architecture §Occupied Resources: 38115 B, unchanged by this chunk (no master edit); it stands AT its
    threshold, so an amendment that grows it frees bytes first.
- **Dev-tool versions:** none — no host tool installed, upgraded or read changed.
- **Harness / gate surface:** none. No harness script, verb, flag, selector, CI step, status or verdict shape
  changed; every drive ran through the existing `bash scripts/agent-run.sh run --live real-model`. The launch, the
  pre-leg checks and the census stayed session-level (a scratchpad script, never committed), so no governed spawn
  form was added.
- **Cross-project / external claims:**
  - andromeda-pulse: the run is pinned to `f70be92`; Pulse HEAD `48714f0` differs from it in no file under
    `crates pulse-app xtask Cargo.toml Cargo.lock` (`git diff --stat`, empty); both release binaries hash as the
    fifth series recorded (`pulse-app` `df167647…ba4a`, `andromeda-pulse-mcp` `6175fc36…d2a9`). Nothing was built.
  - Pulse's own log for this launch: `inference_mode` `real`, `load_status` `loaded`, `model_identity`
    `gemma-4-E4B-it-Q4_K_M`, `prompt_version` `v2.6` on 37 of 37 prompt assemblies, 37 of 37 parses `ok`, 0
    inference errors, 0 inference skips.
  - The operator's vehicle (the overseer's folder, outside this repo): three files recorded by sha256 and byte
    count in the ledger, never copied; nothing under the capture folder was opened, hashed or counted by the leg
    or the agent. The overseer's word at this wrap: the captures are tied and already with the Pulse builder.
  - CI: run CI#37625765074, `completed/success`, `verdict: green`, checks 3/3, wall 745 s, measured on
    `15fab57095a35b7951bc2b099465426d50a4e73c` (the pre-CI commit; this wrap's own commit adds to that tree). The
    overseer verified the same run.
  - RustSec advisory-db: the local copy's `HEAD` equalled its `FETCH_HEAD` and the remote head read live
    (`b0797f54`); three untracked pre-id-assignment leftovers were removed by name before the audit reading kept.
  - Inputs (`inputs.py verify`: 7 entries — unchanged 3 · drifted 0 · vanished 0 · broken 0 · n/a 4 · uncited 2):
    - I1 · `../additional/pc-overseer/relays/conductor-phase-capture-2026-10-07.md` · copy · unchanged
    - I2 · message (the phase invocation) · copy · n/a
    - I3 · `../additional/pc-overseer/relays/conductor-wrap-capture-2026-10-07.md` · copy · unchanged
    - I6 · `../andromeda-pulse:pulse-app/src/llamacli_inference.rs` · pointer @`48714f09` · unchanged
    - I8 · message (the P5 review answer) · copy · n/a
    - I9 · message (the go, 14:28 local) · copy · n/a · UNCITED by scope/research/plan (snapped at implement; the
      attempt ledger cites it)
    - I10 · message (the operator-pass word, an excerpt) · copy · n/a · UNCITED by scope/research/plan (snapped at
      implement; the attempt ledger cites it)
- **Reverted / negative API facts:** none shipped. Two deliberate controls existed only in the working tree and
  were replaced before any commit: a `d1` pin one hex digit wrong, and a `measured` table wrong in every field.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none. One dated observation sits beside two earlier dated records and
  disproves neither: this run's `d3` read `Identified` at the 6-row position where the 2026-10-06 and 2026-10-07
  series' `d3` read `NotIdentified` (each record is of its own drives).
- **Expected amendments (from plan):**
  - architecture §Occupied Resources → On-disk artifacts (the posture-contract entry) — **carried**: Counts /
    qualifiers moved, first bullet (the contract now holds one pre-registered run that is not a series). Sites:
    `pulse-real-model-leg-posture` → `architecture.md` 2 lines (`:184` the entry, which enumerates the series by
    date; `:204` the model-handle line), registries 0. The section is at its byte threshold (38115 B), so the
    entry is re-worded without growing it and dated detail goes to the sidecar.
  - security-plan §Input Validation (real-model capture ingest row) and §Security Anti-Patterns → Data Protection
    — **carried**: Cross-project / external claims (the operator's vehicle) and Coverage of new surfaces below.
    Sites: `2026-10-07 series` → `security-plan.md` 2 lines (`:121`, three hits in the ingest row; `:338` Data
    Protection). Facts for the dated record: the leaf `rm-recorded-run` occurs 0 times across the three captures;
    the host-path probe over the evidence dir reads 0; key rendering `verbatim` on all three; each `## Previously
    Seen` suffix prints `<redacted>` (d2: 1, d3: 2); each capture carries one report body; an operator-owned
    recording of the model's argv ran beside the run, which no Conductor artifact holds or reads.
  - test-plan §6 Scenario: Fingerprint-storm → Real-model interpretation leg and §9 Live-Pulse scenarios —
    **carried**: Counts / qualifiers moved and Outcome. Sites: `pre-stated series` → `test-plan.md` 3 lines
    (`:124` §2, `:260` §6, `:391` §9), `architecture.md` 1 (`:70`), and the distillations
    `.claude/docs/commands.md:12`, `.claude/docs/tests-summary.md:22`. Fact: a pre-stated run that states NO
    verdict now exists beside the series; whether "driven only as the pre-stated series" covers it as worded is
    the question the plan left to this wrap.
  - obs-plan §4 Scenario: Headless deterministic scenario run with MCP read-back verification (the Real-model
    posture paragraph) — **carried**: Cross-project / external claims and Outcome. Site: `2026-10-07 series` →
    `obs-plan.md` 1 line (`:221`). Facts: all three envelopes landed `ManualCheck` with `verdict` null, eleven
    keys; Pulse logged `prompt_version` `v2.6` and `model_identity` `gemma-4-E4B-it-Q4_K_M`; no span name, span
    attribute, log line or allowlist entry was added.
- **Coverage of new surfaces:**
  - the capture run's committed captures (the third use of the real-model capture ingest on this host) →
    validation mechanism✓ (the standing scrub chain `mask_workspace_key` + `redact_value` + `mask_host_paths` +
    `elide_fingerprints`; both capture handles through the `capture_paths` guards, unchanged) · instrumentation
    n/a (no new operation) · PII redacted✓ (leaf 0 times, host paths 0, 0 un-elided `fingerprint_hex` values) ·
    tests unit (six harvest tests; digest pins; fixed point of the elision) · a11y n/a · tokens n/a
  - the contract section `## The 2026-10-07 capture run` → validation n/a (reader-less document; its digest is
    pinned by `the_2026_10_07_capture_run_record_was_fixed_before_d1`) · instrumentation n/a · PII n/a (no host
    path, no value) · tests unit · a11y n/a · tokens n/a

## Deviations from intent
- **The advisory-db residue was cleared outside the repository.** The plan's porcelain entry read red on three
  untracked pre-id-assignment files; after the copy was read current and each leftover diffed against its tracked
  twin (the id line alone differs), the three were removed by name. Justification: scope item 11 names the remedy;
  the first pass's `cargo audit` reading over the unclean copy was discarded.
- **One clause of the pre-registered section is not word-for-word the fifth series' text.** The Re-fires clause
  keeps "only a `pipeline-fault` canary, once, uncounted" and adds that an inference error, an inference skip or
  a failed spawn stops the sitting and is reported before any re-fire; it drops the series' parenthetical
  definition of pipeline-fault. Justification: plan step 10 and its rejected-approaches entry. The section was
  digested before `d1` and is not edited.
- **Three `mcp.tools.call.error` WARN lines per drive fall about 0.2 s after each bracket's end** (the capture's
  own read-back), so the ledger records them as outside the bracket where the fifth series' ledger counted them
  in its leg window. A recording difference, not a behaviour difference.
- **The overseer's reading of the operator's folder was kept out of every committed file.** The operator-pass
  message carried one sentence of it; input I10 holds the directive sentences only, marked as an excerpt.
  Justification: the scope's boundary (no fact derived from a recorded prompt enters a Conductor artifact).
- **A read-only `gate.py hygiene` preview** was run at implement before the operator pass; it is not the entry.
- scope record: none — `gate.py scope` clean, changed 6 · listed 6 · 0 recorded.

## Decisions & corrections
- The overseer's go (I9) and the operator-pass word (I10) were each given in the session and snapshotted.
- The overseer recorded one difference from the fifth series' sitting: Pulse's builder did CPU work in its own
  tree during the sitting (no model, neither port).
- **Wrap directive (the overseer, founder-delegated, 2026-10-07; `{run_dir}/wrap-directive.md`):** report terms —
  a capture run, no `v3-09` verdict; no drive missed, so the d3 position stands at 2 misses of 3 live runs, a
  rate. One observation is recorded beside the grades and no rule changes (Outcome). Whether the rule should
  reject that case is put to the founder before the sixth series, carried on that entry as an open question.
  Route-resolve mints nothing; the sixth series stays `BLOCKED-ON` Pulse.
- Sweep hazards found this chunk:
  - A drive's bracket taken from the frozen self-obs stream ends BEFORE the capture test's own sidecar read-back,
    so Pulse lines the capture causes (`mcp.tools.call.error`) fall just outside it.
  - `cargo audit` exits 0 and loads the same advisory count over a copy holding pre-id-assignment leftovers; only
    the porcelain probe before it shows the copy unclean.
  - The PreToolUse Bash guard refuses a `cat` heredoc with a file target; ledger appends go through Edit.
  - The Read tool clips each multi-KB line of `.claude/rules/verification-harness.md`; a folded scratchpad copy
    read in pages is the full read.

## Outcome
**A capture run, and no `v3-09` verdict.** Three drives fired in one sitting (12:29:13Z to 12:54:07Z), none
re-fired, none replaced, no fourth. Grades, as observations (the harvest's, by the unchanged rule over the pinned
files):

| Drive | Route | Grade (an observation) | Creating digest's corpus rows | `## Previously Seen` lines |
|---|---|---|---|---|
| d1 | `ReadBack`, incident 2, pickup 5607 ms | `Identified` | 1 | 0 |
| d2 | `ReadBack`, incident 5, pickup 13165 ms | `Identified` | 3 | 1 |
| d3 | `ReadBack`, incident 7, pickup 6348 ms | `Identified` | 6 | 2 |

- **No drive missed.** In the overseer's report terms: the d3 position now stands at 2 misses of 3 live runs
  (2026-10-06 miss, 2026-10-07 series miss, this run no miss) — a rate, not a verdict and not a cause.
- **One observation beside the grades; no rule changes** (the overseer's word at this wrap). d2's first
  hypothesis reads, in the committed capture: "retry_storm scope_id=conductor is actively occurring in
  conductor-canary." It quotes the cue's `scope_id` value and still places the storm in the canary service. The
  rank-1 rule asks for `conductor` as a whole word and a retry token; both are present, so the rule grades it
  `Identified`. Whether the rule should reject that case is an open question for the founder before the sixth
  series, carried on that route entry.
- The tie is in the ledger: each drive's start and end from the first and last `timestamp_ms` of its frozen
  self-obs stream, and the scenario's `digest.assemble.request` and `interpretation.prompt.assemble` stamps.

Acceptance criteria, each re-asserted against the diff from `6987764` to the tree:
1. Add-only contract section, digest recorded before d1, pin test agrees — **met** (numstat 74/0; ledger line
   `pre-registration sha256: 51389269…87da` at 12:25:44Z, launch 12:28:21Z; the test green).
2. No new handle named by the contract — **met** (census 6 → 6; `architecture.md` not a touchpoint of implement).
3. No file under `crates`, `scripts`, `.github` names a model handle or the vehicle — **met** (sweep 0).
4. Nothing read from the operator's capture folder enters a committed file — **met** (nothing was read; the ledger
   states the leg neither set the handle nor read what was recorded; the overseer's own reading was kept out).
5. Every drive through `run --live real-model`; no file under `scripts/`, `conductor-cli/`, `conductor-verify/`,
   `conductor-run/src/`, `conductor-tauri/` in the diff — **met** (frozen-path diff empty).
6. Preconditions probe exit 0 before each drive — **met** (three round listings, each `green`).
7. Binaries, model and pass-through digests equal before d1; handle entries green in the capture-env shell; three
   operator files by digest and byte count; `inputs/` holds no copy — **met**.
8. Each capture matches its pin, holds 0 un-elided keyed values, is a fixed point of the elision and holds 0
   occurrences of the leaf; `COMMITTED_CAPTURES` = 23; no capture text in test source — **met**.
9. No verdict stated: 0 `v3_09` in the module, no ref test, matrix unchanged; grades as the ledger records —
   **met** (`matrix.py show`: claimed 0).
10. Rule byte-identical; each capture opens with it — **met**.
11. Tie table with RFC-3339 instants from `timestamp_ms`, start before first `canary:`, end not earlier than
    `read_back_observed_at` at one-second grain — **met** (checked on all three).
12. Eleven-key envelopes, `verdict` null, `ManualCheck`; no span, attribute, log line or allowlist entry added —
    **met**.
13. Host-path probe 0; secret-scan green — **met** (re-fired after the ledger's last edit; secret-scan 7 of 7).
14. Harvest, `cargo test -p conductor-run`, `capture_paths_guard`, the bundled default and fmt green, no retries —
    **met**.
15. Advisory-db porcelain empty, then `cargo audit` and `cargo deny` exit 0; 562 packages — **met** on the second
    pass (first pass: porcelain red on fetch residue, see Deviations).
16. Post-run census empty, process table in the ledger, six frozen chunks byte-unchanged — **met**.
17. CI read `verdict: green` for the pushed HEAD — **met**: CI#37625765074 on `15fab57`.

Gates (by `run` text; implement's second full pass unless noted):
- the three Pulse probes (`git -C … merge-base --is-ancestor f70be92 HEAD` · `… diff --stat f70be92 HEAD -- …` ·
  `… sha256sum pulse-app andromeda-pulse-mcp`) — `leg = 'operator'`, by hand: exit 0, atoms held.
- the five pre-leg checks (`test -f "$ANDROMEDA_PULSE_MODEL_PATH" && …` · the posture check · the two `sha256sum …
  | cut` digests · the NVIDIA majors) — `leg = 'operator'`, by hand in the capture-env shell: exit 0 each, atoms
  held.
- `ls -1 "$HOME/.cache/pulse-legs" …` — operator: `leaf absent`, exit 0.
- the round, three firings (`round: COMPLETE · legs fired 1/1` each, listings in `evidence/`): the census
  (`ps -eo … ; ss -ltn | grep -cE ':4317|:4318'`) green ×3 · `… conductor -- preconditions --for
  real-model-interpretation` green ×3 · `sleep 180` green ×2 · `… bash scripts/agent-run.sh run --live real-model
  && cp … && head … && tail …` green ×3 (362 s each, exit 0, four atoms, artifact fresh, no survivor).
- the post-run census — operator: exit 1, last line 0.
- `cargo nextest run -p conductor-run --test real_model_harvest --profile ci` — green (125 of 125).
- `cargo test -p conductor-run` — green (382 passed, 27 result lines).
- `cargo nextest run -p conductor-run --test capture_paths_guard --profile ci` — green (8 of 8).
- `bash scripts/agent-run.sh run` — green (nextest 1225 of 1225, doctests, three lint lines).
- `cargo fmt --all --check` — green.
- `grep -c "^name = " Cargo.lock` — green (562).
- the rule diff · the contract's deleted-line count (0) · the handle census (6) · the name sweep (0, exit 1) · the
  frozen-path diff · the frozen-chunk diff · the module's `v3_09` count (0, exit 1) — green each.
- `git -C "${CARGO_HOME:-$HOME/.cargo}/advisory-db" status --porcelain` — first pass `red · no output ✗` (fetch
  residue; fixed), then green · `cargo audit` — green (1293 advisories, 7 allowed) · `cargo deny check advisories
  bans licenses sources` — green.
- the host-path probe over `evidence/*` (0, exit 1) · the workspace-key probe (0, exit 1) — green, re-fired after
  the ledger's last edit.
- `bash scripts/agent-run.sh status <id>` — operator, with d3's run id: exit 0, atom held.
- `python -X utf8 "$HOME"/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — operator pass: `hygiene:
  clean` (twice).
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD && echo …` — operator pass:
  `PUSHED_SHA=15fab57…`.
- `… ci.py conclusion --sha HEAD --wait 1800` — operator pass: `verdict: green`, CI#37625765074.
- Smoke: the `status` entry above (mint-then-read); no boot path changed.

Watches: none folded.

Outcome basis: the operator pass ran — one commit (`15fab57`) and the final HEAD's CI run CI#37625765074, recorded
in `evidence/attempt-ledger.md`; implement's report as given in this session; the wrap directive above (it adds
one observation and one route carry, changes no gate verdict).

Process hygiene (implement's census, re-measured at this wrap, 13:19Z): `pulse-app` 71524 and its five WebKit
children — the agent, on the overseer's go — terminated 12:54:21Z; `conductor` and `andromeda-pulse-mcp` per leg —
the legs — exited, no survivor on any round; the model binary's processes — `pulse-app` — exited. Now: 0 listeners
on `:4317` or `:4318`; one `llama-cli` is running under Pulse's builder's own probe (parent in the andromeda-pulse
session), not started by this chunk and left alone.
