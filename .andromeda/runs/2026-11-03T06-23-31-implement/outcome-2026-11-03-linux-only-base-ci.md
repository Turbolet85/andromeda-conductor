# Implement outcome — 2026-11-03-linux-only-base-ci

**Outcome: green** (gates 1-15, one firing, 15 green · 0 red) · smoke skipped — no boot-path / UI-surface change.
Next → the OPERATOR PASS: entries 16 (`gate.py hygiene`), 17 (merge-base probe), 18 (the push — history-moving,
the operator's) and 19 (`ci.py conclusion`), fired as written; only then the wrap. Both wait for the overseer's
word (`inputs#I1` item 8, `inputs#I7`, `inputs#I8`).

## Gate run (`gate-2026-11-03-linux-only-base-ci.json`, one call)
`entries 19 · green 15 · red 0 · recorded 0 · timeout 0 · not-run 4` — 16-19 `not run — leg operator`.
- 2 `bash scripts/agent-run.sh run`: `1218 tests run: 1218 passed, 0 skipped`; inside it
  `commands::suite::tests::the_first_interrupt_sets_the_stop_flag_and_the_process_lives` and the four
  `matrix_ledger_gate` tests (`every_ledger_declares_a_capability`, `capability_ids_are_unique`,
  `no_pulse_p_id_occupies_an_id_position`, `the_gate_discriminates`) PASS; doctests and the three clippy lines exit 0.
- 3 `cargo test -p conductor-run --locked`: every target `ok`, 0 `FAILED`; `every_committed_capture_carries_no_un_elided_fingerprint_value`
  (the 26-capture walk) and `no_committed_capture_text_sits_in_test_source` ok — read over the copies, with both
  `chunks/` directories absent from the tree.
- 4-15 probes green on their atoms: roster `0`; no chunk path and no twin/`windows-latest` reader in
  `crates`/`scripts`/`.github`; `ci.yml` `last line 2` / `last line 0`; the retired set `last line 0`; the 46 copies
  byte-equal to `8e450f3` (no output); `46` files; no pin line moved since `8e450f3`; the product row
  `.andromeda/architecture-amendments.md,.andromeda/architecture.md`; `agent-run.sh` unchanged since `8e450f3`.
- No deferral: every gate ran. Matrix: no cap is owned by the chunk (`coverage where chunk = …` → 0 rows), so no
  ref was written, as the plan states (`v4-07` stays pooled).

## Step 9 (the reconcile), as fired
- `inputs#I7` (the overseer, founder-delegated): two per-family reconciles, `--product none`, each after its own
  dry-run, never naming the architecture pair; it supersedes the plan's "never `--product none`" constraint.
- Spine: the dry-run read `dry-run · every render equals the journal`. The live call printed `render: … behind
  (absent), healed from the journal` for each of the six, then `render ok`. The row went from 57 paths to 51, but the
  six retired files were written back into the tree (F412: the reconcile heals what it drops, and its dry-run is
  silent on it). Halted; no `rm`.
- `inputs#I8` (the recovery word, confirmed by overseer1), git only:
  - the six: `git restore --staged` then `git rm -f`;
  - the 49: `git restore --source=HEAD --staged --worktree` (the list in `the-49.txt`). The docs dry-run read
    `docs's product paths would read: none`, the live call `docs: product 0 path(s)` and touched no tree file, then
    `git rm` the 49;
  - `rec.py check --root .` → `check: clean`; the product row holds only the architecture pair.

## Step 10
- Raised as one dialog naming `gh cache delete v0-rust-a11y-Windows_NT-x64-59a01e7d-ae7453cb` and
  `gh cache delete node-cache-Windows-x64-npm-1bef902bde0e18535ee54e9700c699e839dda954c65ab543d48dbece089ff30f`,
  and no directory outside git. Answered done (`inputs#I6`): overseer1 ran both on the founder's word, exit 0; the
  caches API reads `total_count 0`. Implement ran neither.

## Files changed
- `crates/conductor-report/tests/matrix_ledger_gate.rs` — re-based onto each version's `coverage.md`, both line
  forms; `no ledger` skipped by its line; arms: declares-one, unique, no-P-NNN; the set-equality arm and its JSON
  machinery removed; `the_gate_discriminates` rewritten (step 1)
- `crates/conductor-run/tests/evidence/` — 46 byte-equal copies in 11 per-chunk folders (step 2); untracked until
  the pre-CI commit
- `crates/conductor-run/tests/real_model_series/mod.rs` (7 constants) · `delegated_timing_harvest.rs` (3) ·
  `lifecycle_harvest.rs` (2) · `real_model_harvest.rs` (2) — prefix swap; file names and pins unchanged; rustfmt
  un-wrapped four constants (formatting only) (step 3)
- `crates/conductor-run/tests/real_model_grading/capture_population.rs` — the walk reads
  `crates/conductor-run/tests/evidence/*/rm-capture*.txt`; `COMMITTED_CAPTURES` stays 26 (step 3)
- `scripts/mutation-roster.toml` — five `citation_home` values point at
  `.andromeda/runs/2026-10-04T01-45-46-wrap/u35/logs/test-plan.md` (step 4)
- `.github/workflows/ci.yml` — `ubuntu-latest`; dogfood step `shell: bash` / `run: bash scripts/agent-run.sh run`;
  two comments re-stated (step 5)
- `crates/conductor-core/tests/workflow_env_gate.rs` — `STEP_HEAD` on `ubuntu-latest` (step 6)
- `crates/conductor-run/tests/live_suite.rs` · `real_model_live.rs` — `scripts/agent-run.sh` (step 7)
- removed in git (step 8 + the recovery): `scripts/agent-run.ps1`, the six spine files, the 49 chunk files — 56 `D`
- `.andromeda/db/moved` — the `product` row is the architecture pair alone; `.andromeda/db/log/` — the tool journals
  (step 9)

## Deviations
- Step 8: `git rm --` of the six refused on `conductor-0.4.0/verification-matrix.json`'s uncommitted change, a render
  of phase P5's `v4-07` note. The note sits in the store's `cap:conductor-0.4.0/v4-07` rev 2 (compared equal), so the
  removal was re-fired with `-f`.
- Step 9: run as two per-family `--product none` reconciles on the overseer's word (`inputs#I7`), not the plan's one
  architecture-pair call; the heal it caused was recovered by git alone (`inputs#I8`).
- Step 11: gates run after the recovery, not directly after the plan's one call.

## Scope
`gate.py scope` → `scope: clean — changed 58 · listed 58 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 ·
widening 0) · absorbed 46 · excluded 109`. No scope-record line needed.

## Guard entries
None in the plan.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo · cargo-nextest · rustc · conductor · andromeda-pulse-mcp | this run (gate entries 2-3) | `terminated` — `census: none running` |

## Decisions & corrections
- The overseer (founder-delegated), the invocation: steps 1-8, halt at step 10, nothing of step 9 until its word;
  gates once each; diff probes anchored to `8e450f3`.
- `inputs#I6`: the hand step is done (both caches deleted, `total_count 0`).
- `inputs#I7`: `inp-tdlsudwg`'s never-none line superseded; each family clears only its own.
- `inputs#I8`: F412, the reconcile heals what it drops and its dry-run is silent; the recovery is git only — no `rm`, no
  `git clean`.
- A `git rm` refusal (step 8), resolved by a store read and `-f`. Guard/hook refusals: none.
