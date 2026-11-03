# P5 review — 2026-11-03-linux-only-base-ci «Linux-only base CI» (conductor-0.4.0)

The plan to open in an editor: `.andromeda/cache/docs/2026-11-03-linux-only-base-ci/plan.md` (rev 4, rendered from the
store). No plan stands in a chunk's folder. Writing in that file changes no record until a `put`. For a diff of the
chunk's records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-03-linux-only-base-ci`.

## Goal
The one CI job `rust` moves from `windows-latest` to `ubuntu-latest`, keeping every gate. The dogfood step becomes
`bash scripts/agent-run.sh run`, and `scripts/agent-run.ps1` leaves. The three carries ride along:
- the matrix-ledger gate re-bases onto each version's `coverage.md`, both line forms;
- the 46 test-read evidence files move byte-equal to `crates/conductor-run/tests/evidence/`, every pin unchanged
  (the founder's ratification, `inputs#I3`), and the roster's five citations re-point to the U35 test-plan §12 log;
- the chunk `git rm`s the six spine files, the 49 chunk files and the twin (`inputs#I4`), then reconciles the
  `product` row to the architecture pair, gated on the store tool.

## Steps (11)
1. Matrix-ledger gate re-based.
2. Copy the 46 files.
3. Re-point the five readers.
4. Roster citations.
5. `ci.yml` to Linux.
6. `STEP_HEAD`.
7. Two doc comments.
8. `git rm`: the twin, the six, the two chunk dirs.
9. Reconcile, dry-run first. **HALT if `rec.py` still refuses the pair**, the line reported verbatim (refused at P5 by
   `rec.py v1.10`; overseer1 owns the fix, `inputs#I4`).
10. **HALT for the founder's hand step outside git**: `gh cache delete` of the two stale Windows caches. No directory
    outside git was measured.
11. Gates 1-15 once, then stop. The operator pass runs 16-19, and the wrap waits for your word.

## Top acceptance criteria
- `ci.yml`: one job on `ubuntu-latest`, `bash scripts/agent-run.sh run`, no `pwsh` / `windows` / `.ps1` token. The
  pushed HEAD's Linux CI run reads `verdict: green` with every gate step passing.
- `bash scripts/agent-run.sh run` exits 0 on the dev host, its nextest running the `#[cfg(unix)]` first-interrupt
  witness, which the Linux leg now runs in CI as well.
- `cargo test -p conductor-run --locked` passes over the relocated evidence. The 46 copies are byte-equal to
  `8e450f3`, no pin line moved, and no `conductor-0.N.0/chunks/` path stands in `crates/` or `scripts/`.
- The mutation selftest passes and every `citation_home` resolves. The `product` row names the architecture pair
  alone, and the 56 tracked files are out of the index and the tree. `agent-run.sh` is byte-unchanged.

## Touchpoints
- new: `crates/conductor-run/tests/evidence/`
- modify: `.github/workflows/ci.yml`, `matrix_ledger_gate.rs`, five `conductor-run` test files,
  `scripts/mutation-roster.toml`, `workflow_env_gate.rs`, `live_suite.rs`, `real_model_live.rs`, `.andromeda/db/moved`
  (tool write)
- removed in git: `scripts/agent-run.ps1`, the six spine files, `conductor-0.2.0/chunks/`, `conductor-0.3.0/chunks/`

## Test commands
19 entries:
- 1 `cargo fmt --all --check`
- 2 `bash scripts/agent-run.sh run`
- 3 `cargo test -p conductor-run --locked`
- 4 mutation selftest
- 5 the citation probe
- 6-15 the absence, shape, byte-equality, count, pin, row and harness-unchanged probes
- 16 hygiene · 17 merge-base probe · 18 the clean-tree push · 19 `ci.py conclusion --wait 1800` (all operator)

Baselines: 1-2 seen green at the last wrap. Entries 3, 4, 5, 13 and 15 are green. Entries 6-12 and 14 are red on
this chunk's own subject. Every new probe was fired on a scratch clone: unmodified, with the subject built (all green
but 7, whose only hits are in the file step 8 removes), and over planted negatives (5, 13 and 15 read red).
`.andromeda/runs/2026-11-03T05-52-57-phase/p5-controls.md`.

## Capability claims — concretization preview
- None claimed. `v4-07` needs "each can be run on the developer's host before a push", and the next entry
  `Fast feedback on Linux` (`working-route.md:8`) names `v4-07` and carries that leg. `v4-07` stays pooled. After the
  word, a dated `matrix.py note` records the failed concretization: "builds, full gate, a run on Linux, no
  script/leg for another system — delivered by `2026-11-03-linux-only-base-ci`; the run-every-leg-locally half is
  `Fast feedback on Linux`'s".

## Decided in this phase
- P4 fork (the capture move, Boundary-widening pattern) — answered by the founder's own word, relayed verbatim
  (`inputs#I3`, «Разрешить перенос»): the Data Protection exception extends to `crates/conductor-run/tests/evidence/`,
  and test-plan §7 is amended to match at the wrap.
- P5 fork (the reconcile) — "Gate it on the fix", delegate overseer (founder-delegated), 2026-11-03 (`inputs#I4`):
  the `git rm` is the chunk's own, the reconcile follows it, and step 9 halts if the tool refuses. Health check 13's ⚠
  after the twin leaves is known (F411).
- Leans, each set by an artifact:
  - `spawn.rs`'s Windows arms stay, for «Spawned sidecar and shared-machine gate retired».
  - `STEP_HEAD` becomes `ubuntu-latest`, and the pwsh grammar case is kept.
  - The citations point at the U35 §12 log.
  - One reconcile call.
  - No split.
- Validation-1: intent-incomplete, scope amended (rev 5) — the deletions bullet narrowed per `inputs#I4`.
- sampler: off

## Beyond the plan
- A new markerless entry directly after «Linux-only base CI», proposed title «Architecture-registry check retired».
  It retires `scripts/arch-registry-check.py`, drops the `product` row's architecture pair, and cites the founder's
  ruling (item 6 of the masters-classify-pilot `rulings.md`, lines 38-39). The wrap mints it (`inputs#I1` item 4).
- An owner outside this repository: Andromeda's tools (`health.py` check 13's `HARNESS`, setup's harness template)
  learn a Linux-only project. Carried by the overseer; the ⚠ is known meanwhile (F411).
- An owner for a part left out: `spawn.rs`'s `#[cfg(windows)]` arms go with the existing entry «Spawned sidecar and
  shared-machine gate retired».
