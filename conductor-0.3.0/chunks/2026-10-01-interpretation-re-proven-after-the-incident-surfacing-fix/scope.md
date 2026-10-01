# Scope — 2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix

**Working entry (`working-route.md:75`):** Interpretation re-proven after Pulse's incident-surfacing fix — a third
pre-registered real-model series for `v3-09` on a fresh letters-only data dir.

**Matrix target:** `v3-09` (*Real-model interpretation leg*, `dynamic-external`, status `deferred`, `chunk: null` at
take-up — un-claimed to the pool by the 2026-09-30 wrap, owned by this entry). Its acceptance as it stands names the
2026-09-30 series by section (`contracts/pulse-real-model-leg-posture.md` §The 2026-09-30 series), three drives on ONE
fresh letters-only data dir, a pulse-app from "a Pulse HEAD carrying its scrubber fix", digest-pinned captures graded
from their files, a pre-registration sha256 recomputed by `real_model_harvest.rs`, at most one uncounted
pipeline-fault re-fire, no fourth drive; met iff ≥ 1 drive is graded AND every graded drive is `Identified` (rank-1
statement names `conductor` as a whole word + a retry token) with the real-model witnesses
(`inference_mode = real`, no `det-` evidence ref); a graded `NotIdentified` is NOT met and never replaced; the stated
limit (the cue line itself satisfies the rule) unchanged.

**Chunk base (W182):** `1fe46a1` (`1fe46a10948647938168d45da4b21d34bd47de3e`), HEAD at take-up (operator: "Base
1fe46a1"). Every diff-shaped gate probe names it explicitly, because the operator pre-CI commit moves HEAD before the
wrap.

## What this chunk builds

1. **A new pre-registered series section in the posture contract**, add-only beside §The drive series (2026-09-29) and
   §The 2026-09-30 series — never rewriting either record. Fixed BEFORE d1: the drive count, the data dir, the pass
   condition (§The drive series (a), unchanged), the canary-is-a-measurement rule, the one-uncounted-pipeline-fault
   re-fire rule, no further drive, the quiet windows, the launch posture re-pinned to the Pulse fix HEAD, the slot.
   Its sha256 (LF-normalized, heading to next `## `) is recorded in the chunk's `evidence/attempt-ledger.md` before d1
   and recomputed by `crates/conductor-run/tests/real_model_harvest.rs`.
   - The count stays three, on the 2026-09-30 design (P3: no research finding moves it — the entry says "same
     discipline as that series — a fixed drive count"; P4 states the lean, the overseer ratifies at P5).
   - The GRADING RULE does not change: nothing between `real_model_harvest.rs:61` and `:449` is edited, because both
     prior series hold their recorded rule EQUAL to the current one (`:1735`, `:1958`) — and P3 found nothing at
     Pulse `a2addb3` the rule must absorb (the hypothesis render is unchanged, `markdown.rs:153-171`).
2. **The binaries (CARRY).** Build BOTH `pulse-app` and the MCP sidecar from the fix HEAD and prove each by content
   before d1 — the sidecar renders the graded report through the same scrubber (measured at `fcc31b2`
   `crates/interpretation/src/markdown.rs`).
   - **Slot (operator directive):** building Pulse binaries runs in the Pulse tree, where pulse-builder also builds —
     implement STOPS and asks the operator for that slot. No build without it.
   - `pulse-app`'s content probe — VERIFIED at P3: NEW at `a2addb3` (0 hits at `fcc31b2` by `git grep -F`):
     `incident producer skipped`, `interpretation.incident.skipped`, ` active incident(s); `; OLD control
     ` active-bypass incident(s); ` (0 hits at `a2addb3`). A runtime witness rides too: Pulse logs
     `prompt_version` `v2.3` (`schema.rs:31@a2addb3`, was `v2.2`) on every prompt assembly the capture prints.
   - [premise-corrected: the sidecar has NO source-determinable new/old string — its source references none of the
     surfacing symbols and the redaction commits add no non-test literal (research.md §Pulse at `a2addb3`); whether
     the linked triage OVERALL strings survive its link is measurable only on the built exe.] So the sidecar's proof
     is MEASURED FIRST: probe the built exe for the same new/old pair; where the new string is absent, the proof is
     build provenance — a clean `a2addb3` tree, the build command, the exe's sha256 and mtime after the commit — plus
     a sha256 different from the `fcc31b2` build's (`2179caab…`, prior ledger `:29`). Recorded as which arm held.
3. **The key confirmation (CARRY).** Confirm from Pulse's `app.boot.workspace_key` line that the key's leaf is the data
   dir's, since the capture's key mask derives the key from the data dir.
4. **A fresh letters-only data dir** — a NEW leaf under `%TEMP%/pulse-legs/`, not `rm-clean-series` (used by the
   2026-09-30 series), carrying no digit, no vendor prefix, no `@`; its absence before creation is recorded.
5. **The series itself** (d1..dN, LAST, gated): each drive through `bash scripts/agent-run.sh run --live real-model`
   against a pulse-app launched per the posture contract. Gated on: (a) the Pulse fix committed and pushed — VERIFIED
   at take-up (below); (b) the operator's slot — **model runs and `:4317` are the operator's slots**: implement stops
   and asks. Every drive recorded: capture committed under this chunk's `evidence/`, sha256-pinned in
   `crates/conductor-run/tests/real_model_series/mod.rs`, graded from the file by `real_model_harvest.rs`, one row in
   `evidence/attempt-ledger.md` with its workspace-key rendering witness.
6. **The verdict test + the ref.** A harvest test states the series verdict over the pinned captures (as
   `v3_09_is_not_met_by_the_2026_09_30_series` does for the prior series); the separate ref test asserting
   `Identified` + the real-model witnesses is written ONLY if the pass condition holds.
   - VERIFIED at P3: the no-incident outcome is `interpretation.incident.skipped` (`inference_runtime.rs:96,955-964
     @a2addb3`), fields `skip_reason` ∈ {`model_resolution_summary`, `decision_dismiss`, `severity_none`, `no_cue`} ·
     `decision` · `severity` · `digest_kind` — closed enums Pulse's code writes, all four allowlisted
     (`observability.rs:2225-2231`), none `<redacted>`. So the capture prints it (hermetic, before d1): the window list
     gains the target, and each `canary:` line gains its paired inference's `skip_reason` as a TRAILING field — the
     rule reads only the first token (`real_model_harvest.rs:433-448`), so no rule edit. A dismissal is then recorded
     WITH Pulse's stated cause. This supersedes the 2026-09-23 harness-rule clause "Pulse logs NO line when its model
     DISMISSES a digest" for Pulse `a2addb3` onward (a wrap curation item, never edited here).
   - [premise-corrected: the L4 STRUCT is unchanged — only `schema.json`'s property order moved (`decision`/`severity`
     after `hypotheses`) and the prompt versions bumped; the `## Hypotheses` render is byte-unchanged
     (`markdown.rs:153-171`) and `retrieve_report` still parses the struct (`tools.rs:377-381`).] No parsing gap: the
     rank-1 extraction needs no hermetic fix. Degraded mode is LESS likely than at `fcc31b2`, because the stored
     summary is now scrubbed per string leaf (`inference_runtime.rs:675-690@a2addb3`).

## Folded freight (`working-route.md:75`, three blocks, all per `route.py pins`)

- **BLOCKED-ON** (285 chars, folded whole): "Pulse's "Real-model incident surfacing" entry committed and pushed —
  clears when a Pulse commit carrying it is on Pulse's pushed branch (founder 2026-09-30, "fix in Pulse 0.3.0"; minted
  on relay `conductor-wrap-56-2026-09-30` §2, placement before Version close named there)".
  - **CLEARED, verified at take-up** (overseer's measurement re-read by the agent, Pulse read at committed state, no
    fetch): `a2addb3` (`a2addb3755b3029cb79809b96efdd2522749b179`), "feat(2026-10-01-real-model-incident-surfacing):
    the real model surfaces a storm digest for a measured cause", 2026-10-01 20:39:04 +02:00, is on
    `origin/chore/migrate-pulse-to-v3`; Pulse's checked-out branch is `chore/migrate-pulse-to-v3`, HEAD = that ref =
    `a2addb3`, 0 ahead (read against the local tracking ref).
  - Pulse moved 17 commits `fcc31b2..a2addb3`, among them `1dfca74` (credit_card arm Luhn-gated over whole-group
    windows), `09d0809` (span-level redaction — "a secret inside a larger value redacts only its matched span"),
    `fb93fca` (a service registered at first sighting, not the 15 s tick), and the surfacing chunk's two pre-CI commits
    `69f0b93`, `f37cd3e`. Closed at P3 (research.md §Pulse at `a2addb3`):
    [premise-corrected: `1dfca74` carries no code — the Luhn arm landed in `fcc31b2` itself; `f37cd3e` touches only
    `pulse-app/ui` packages. The code-bearing set is `69f0b93` (surfacing), `9d14166`+`7949d81` (span redaction),
    `87fe658` (first sighting), `5fbf762`/`d708ad7` (perf).] Span redaction now masks only a secret's span instead of
    the whole value; a letters-only leaf is touched only after a keyed word (`password|passwd|secret|token` …)
    directly followed by whitespace, `=` or `:`, which the leaf avoids. First sighting moves only the registry
    listing time — no `crates/triage/src/cue` or `pattern` reference, so no change to when the canary's cue fires.
    The workspace key and its publication are unchanged (`git diff fcc31b2 a2addb3 -- crates/workspace-detector`
    empty).
- **CONTEXT** (476 chars, folded whole): "the 2026-09-30 series graded 0 drives — d1 canary-blocked, and in d2 and d3 the
  real model dismissed the scenario's own storm digest (measured at
  `conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/evidence/attempt-ledger.md`);
  `v3-09` returns to the pool at that wrap and this entry owns it; same discipline as that series — a fixed drive count
  and rule in the posture contract before d1, digest-pinned captures, no retry-until-pass".
  - Re-verified: the attempt ledger exists and its series verdict reads "Graded drives: 0" (`attempt-ledger.md:56`);
    the matrix `notes` 2026-09-30 wrap note matches.
- **CARRY** (440 chars) → items 2 and 3 above, verbatim: "build BOTH `pulse-app` and the MCP sidecar from the fix HEAD
  and prove each by content (the sidecar renders the graded report through the same scrubber, measured at `fcc31b2`
  `crates/interpretation/src/markdown.rs`); confirm from Pulse's `app.boot.workspace_key` line that the key's leaf is
  the data dir's, since the capture's key mask derives the key from the data dir".

## Causal claims carried in (P3 closes, marker kept)

- "in d2 and d3 the real model dismissed the scenario's own storm digest" (marker: "measured at …
  `attempt-ledger.md`") — VERIFIED: the ledger's d2/d3 rows (`attempt-ledger.md:51-52`) read the scenario's tier-1
  digest prompted, parse `ok`, then no incident outcome before the next prompt; series verdict "Graded drives: 0"
  (`:56`).
- Operator relay, verbatim: "Pulse shipped A5 (decision after the analysis) + a truthful OVERALL line; synthetic
  29/30." — VERIFIED as relayed and located: A5 is the `schema.json` reorder (arm matrix A5 30/30); the shipped tree
  read `would_create 29/30 · decision 19/0/11 · severity_none 1`, pre-registered minimum 27, PASS
  (`andromeda-pulse-0.3.0/chunks/2026-10-01-real-model-incident-surfacing/evidence/slot2.md:13-15@a2addb3`, arm A0
  24/30 at `arm-matrix.md:16`). Its own scope note: synthetic Tier-1 storm digests, n = 30, "bounds nothing broader".
  Whether the real model surfaces the SCENARIO's digest live is exactly what this series tests; nothing here settles
  it.

## Boundaries (out of scope)

- No edit to Pulse's repository. Pulse is read at its committed HEAD (`git show <sha>:<path>`), never its worktree. The
  Pulse BUILD runs in Pulse's tree only on the operator's slot.
- No edit to any frozen chunk's `evidence/` (2026-09-22, 2026-09-29, 2026-09-30 series), and no rewrite of the
  posture contract's 2026-09-29 or 2026-09-30 records.
- No widening of security-plan's scoped corpus-text exception; captures enter `evidence/` only through
  `mask_workspace_key` + `redact_value` + `mask_host_paths` + `elide_fingerprints`, digest-pinned, no capture text in
  test source.
- No `v3-09` acceptance weakening: every-graded-must-be-`Identified`, never-replaced, the real-model witnesses and the
  stated limit stand.
- No drive before both gates; no retry-until-pass; a pipeline-fault re-fire only under the rule fixed before d1; no
  drive past the fixed count.
- The seven spec masters stay read-only.
- `/andromeda-evolve-diagnose` waits (founder) — not part of this chunk.

## Operator directives (take-up, 2026-10-01)

- The BLOCKED-ON is cleared, measured by the overseer (re-verified above).
- Building Pulse binaries runs in the Pulse tree where pulse-builder also builds: ask the operator for that slot.
- Model runs and `:4317` are the operator's slots.
- Founder: evolve-diagnose waits.
- Base `1fe46a1`.
- P5 approval (overseer): the plan and the `v3-09` concretization approved (rule byte-identical to every prior
  series, ref only on met, no fourth drive); leans 1-5 stand; "The Pulse build slot and the model-run/4317 slot are
  mine: stop and ask with lengths; pulse-builder is in the same tree, so expect a short hold."

## CI (Setup 5a)

- `1fe46a1` (the 2026-09-30 full-gate wrap commit, the only sha from the last flip through HEAD): **`verdict: green` ·
  checks 3/3 · wall 682 s** — CI#36796579534 push completed/success. Nothing to fold.
