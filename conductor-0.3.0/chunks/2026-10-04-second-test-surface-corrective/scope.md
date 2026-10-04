# Scope — 2026-10-04-second-test-surface-corrective

**Working entry** (`conductor-0.3.0/working-route.md:90`, Epoch 5b — Version close, entry 2 of 4):
Second test-surface corrective — `delegated_timing_harvest.rs` split under the size line, and the Epoch 5 audit's new
emit/run mutation survivors killed.

**Origin:** founder ruling 2026-10-04 ("A", agreed with overseer1; overseer relay at the
`2026-10-04-real-model-test-surface-corrective` wrap) — the second corrective, right after that chunk and ahead of the
`v3-09` series. Source: the Epoch 5 code audit `.andromeda/runs/2026-10-04T09-27-47-code-audit/proposals.md` (M2 and the
Coverage/Mutation bullet "new survivors this epoch").

## What it builds

1. **Split `crates/conductor-run/tests/delegated_timing_harvest.rs` under the size line** (`sizes.over_800`, the
   audit's M2: every file over 800 code lines). Measured at the audit (HEAD `88de180`): 1130 code lines (tokei), 623 at
   the Epoch 3 baseline; grew +558/−36 across `2026-09-29-hue-shift-budget-graded-hard`,
   `2026-10-02-p-075-assert-round-against-pulse` and `2026-10-03-p-075-re-round-on-incident-events`. Re-read at take-up
   (HEAD `dab66dc`): 1130 code lines (tokei 14.0.0), 1222 raw lines (`wc -l`) — unchanged. The audit's proposal:
   "structure the two harvest files per scenario or assertion family, as for M1". [verified at P3: the shape is child
   modules of ONE target (`tests/delegated_timing_grading/`, the prior corrective's precedent). No committed evidence
   pins this file's own bytes — it reads evidence only through `evidence_pin` digests over the evidence files — and it
   reads no `CONDUCTOR_*` / `ANDROMEDA_*` handle. Every test a live document cites by path (the P-075 round and
   re-round families, the 2026-08-21 legs, the 2026-09-07 re-driven leg, the 2026-09-29 graded leg) stays in the root's
   `mod tests`; the grading helpers and the three uncited synthetic families move out (research.md §Split sizing).]
2. **Kill the Epoch 5 audit's new mutation survivors** (roster members identified by mutation description, never by
   file:line:col; coordinates as the audit gave them at `88de180`):
   - `crates/conductor-emit/src/identity.rs:49:15` — `replace ^= with |= in xor_in_place` · `replace ^= with &= in
     xor_in_place` (the per-run span identity code, `2026-10-01-per-run-span-identity-in-the-real-model-harness`).
   - `crates/conductor-run/src/canary.rs:203:14` — `replace > with >= in emit_canary_storms`.
   - `crates/conductor-run/src/canary.rs:206:61` — `replace * with +` · `replace * with /` (in `emit_canary_storms`).
   Re-verified at HEAD `dab66dc`: `identity.rs:49` is `*byte ^= m;`, `canary.rs:203` is `if n > 0 {`, `canary.rs:206`
   is `base.wrapping_add(n * CANARY_STORM_COUNT)` — the columns still land on the named operators.
   [verified at P3 — measured LIVE on this host at HEAD `dab66dc`, cargo-mutants 27.1.0, scoped by file and function:
   all five are in `missed.txt` (identity 2 of 3 mutants, canary 3 of 6), tallies conserving and equal to
   `outcomes.json` (research.md §Live survivor measurement).]
3. **CARRY — obs-plan §10's clippy failure-condition line.** `obs-plan.md:490` reads "`cargo clippy` warnings treated as
   CI annotations (non-blocking at Minimal, but visible to agent)", while the bundled default of `agent-run run`
   (`scripts/agent-run.sh:321-325`, re-verified at take-up) runs three `clippy … -- -D warnings` lines since
   `2026-10-04-real-model-test-surface-corrective`. [premise-corrected: the hypothesis "a lint warning fails the run and
   CI's dogfood step" holds per SURFACE, not uniformly. `agent-run.sh` runs under `set -euo pipefail`, so a red clippy
   line ends the bundled default non-zero (to be MEASURED at implement with a planted, never-committed warning).
   `agent-run.ps1` checks no `$LASTEXITCODE` after its cargo lines and never sets `$PSNativeCommandUseErrorActionPreference`,
   so it blocks only when its CALLER sets that preference. CI's dogfood step does, and recorded run 34689135760 (sha
   `e3ff4e5f`) shows a native non-zero there aborting the step (`NativeCommandExitException`, exit 1). A bare local ps1
   invocation is unmeasurable on this Linux host (no `pwsh`). The `--e2e` arm (the `a11y` job's entry point) runs no
   clippy line at all.] The previous wrap
   [intent-incomplete, val-1: widened at P4 by the overseer's founder-delegated answer — `agent-run.ps1`'s `''` arm
   gains the file's own `if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }` after each of its five cargo lines, so a red
   line ends the bundled default non-zero whoever calls it; the ps1 red path is stated UNMEASURED on this Linux host
   (no `pwsh`) and CI proves only the green path.] The previous wrap
   rejected the amendment (D-obs-ci-gates proposal O1) because its basis was harness source the report did not carry
   (`.andromeda/runs/2026-10-04T12-32-24-wrap/fanout-results.md`) — so this chunk's report must CARRY the measurement,
   and the §10 amendment rides this chunk's wrap.

## Boundaries

- Behaviour-preserving split: every test `delegated_timing_harvest.rs` holds today still exists after it, with the same
  assertion and the same pass/skip/ignore posture; no scenario is re-run against Pulse; no committed evidence or
  capture changes. No pulse-app is needed for this chunk (overseer note at take-up).
- Mutation kills are TEST additions or tightenings; production code changes only if a survivor proves the code itself
  is equivalent-mutant-shaped and the plan says so explicitly. [intent-incomplete, val-1: the kill PROOF is inverse
  controls only — each mutation hand-applied once at implement, the new test failing, the file restored — per the
  overseer's P4 answer; test-plan §9's founder ruling (mutation at the epoch-boundary audit ONLY) stays literal, the
  P3 live measurement is not an exception, and the next audit confirms the kills.]
- Out of scope: `real_model_harvest.rs` (stays 1391 raw lines so matrix v3-10's by-file citations hold; M2 is not
  cleared for it), `scenario.rs` / `execute.rs` (standing, steady), the line-drifted standing `execute.rs` survivors
  (`:126/:169`, not new), `real_model_live.rs`.
- Leaves `v3-09`'s matrix status (`deferred` against the founder's NOT-deferred ruling) to the `v3-09` series' chunk.

## Causal claims carried (closed at P3)

- [premise-corrected: per surface — see item 3] "a lint warning fails the run and CI's dogfood step" — the CARRY's
  hypothesis, marked hypothesis.
- [verified at P3] "`delegated_timing_harvest.rs` grew across three chunks" — its families are the P-075 round
  (`2026-10-02`), the P-075 re-round (`2026-10-03`) and the P-025 graded-leg / hard-grade synthetics (`2026-09-29`),
  beside the older 2026-08-21 / 2026-09-07 legs.
- [verified at P3] the five mutants survive at HEAD — measured live at `dab66dc` (item 2).
- Kill mechanisms, re-derived at HEAD as the equalities the criteria need (research.md §Files inspected): span identity
  is a function of the seed alone (`exception.rs:185-187`), so with base 7 the correct seeds are 7..42 (36 distinct
  identities), `*`→`+` yields seeds 19..32 (14 distinct) and `*`→`/` yields 7..18 three times (12 distinct); `>`→`>=`
  puts a 90 s sleep before storm 0, so storm 0 arrives 90 s after the call instead of at it; `^=`→`|=` / `&=` makes a
  second re-key under the same salt fail to restore the original ids (`x|m|m = x|m`, `x&m&m = x&m`).

## CI verdicts read at Setup (5a; the last wrap's flip `dab66dc` through HEAD)

- `dab66dc` — **verdict not yet available** (in progress, CI#37203295198, checks 3/3; oldest running: the A11y gate at
  113 s). Lands as read, not green.

No red and no `not green`: nothing to fold.

## Operator directives at take-up (overseer note)

- Measure the three survivor sites as LIVE on this host before planning their kills (item 2 above).
- Measure any architecture draft before writing it (P5): `scripts/arch-registry-check.py measure (--rev REV | --file
  PATH)` exists at take-up (`cmd_measure`, `:381`).
- Supply-chain residue, measured at take-up: the local advisory-db holds 0 porcelain lines, HEAD `ef6173cb`
  (2026-10-03).
