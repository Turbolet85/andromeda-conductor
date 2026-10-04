# Scope — 2026-10-04-real-model-test-surface-corrective

**Working entry** (`conductor-0.3.0/working-route.md:88`, Epoch 5b — Version close, entry 1 of 3):
Real-model test-surface corrective — `real_model_harvest.rs` split by series, its repeated grading in
`tests/real_model_common`, and `secret_scan_gate` skipping cleanly where no `.git` exists.

**Origin:** founder ruling 2026-10-04 (overseer relay, the 2026-10-04T11-45-00 0-pending wrap): one corrective ahead
of the `v3-09` series. Sources: the Epoch 5 code audit
`.andromeda/runs/2026-10-04T09-27-47-code-audit/proposals.md` (M1 · the Informational "Runner portability" bullet) and
the Epoch 5 diagnosis `.andromeda/runs/2026-10-04T09-03-35-evolve-diagnose/proposals.md` P14(b) (the CARRY).

## What it builds

1. **Split `crates/conductor-run/tests/real_model_harvest.rs` by series.** [premise-corrected: the split's SHAPE is
   not given. The rule section `:64-452` is code whose bytes committed captures pin (byte-exact prefix and equality
   arms), and its items are private, so a sibling module cannot reach them and a `pub` would break the pin. Child
   modules of one target, or several targets each `include!`-ing the rule verbatim, is a P4 fork — research.md Open
   questions.] Each dated series and each assertion family moves out of the one file, so no single harvest file
   carries every series' grading.
   - Measured at the audit (HEAD `88de180`): 2142 code lines (tokei), 680 at the Epoch 3 baseline; the repo's
     largest file and `sizes.file_max`'s monotonic driver (M1). Re-read at take-up (HEAD `1208ca5`): 2602 raw lines
     (`wc -l`), 102 tests, 102 passing (`cargo nextest run -p conductor-run --test real_model_harvest --profile ci`).
     No source file changed between the two shas, so the two figures are two counts of one file.
2. **Lift the repeated grading into the existing `crates/conductor-run/tests/real_model_common/` module** (`mod.rs`,
   353 lines at take-up). Measured at the audit: 4 of the duplication top-10 rows are this file's self-clones within
   lines 1916–2422 (31 L at 2147/2422 · 18 L at 2128/2403 · 16 L at 1916/2123 · 16 L at 1916/2398). Research adds
   that the harvest's digest helpers (`:1553-1590`) are private copies of `tests/evidence_pin/mod.rs`'s four, which two
   other harvests already use. [intent-incomplete, val-1: only the RULE-INDEPENDENT repeated grading (the envelope
   key-set arm) can live in `real_model_common`; a sibling module cannot see the rule's private items, and a `pub`
   would break the pinned bytes. So the rule-dependent per-series shape lifts into one harness in
   `tests/real_model_grading/mod.rs`, and the digest helpers reuse `evidence_pin`.]
3. **`crates/conductor-core/tests/secret_scan_gate.rs` skips cleanly where no `.git` exists**, instead of panicking.
   Today `workspace_files()` asserts `git ls-files` succeeded (`secret_scan_gate.rs:185-189`, "the gate has no
   subject"), and `the_workspace_holds_no_secret_shaped_string` reaches it.
4. **CARRY — a gate that runs `cargo check --tests` with the `stub-server` feature set.** `conductor-verify`'s
   `stub-server` feature gates `tests/preflight_spawn.rs` (`#![cfg(feature = "stub-server")]`, line 5) and the
   `stub_pulse_mcp` bin (`required-features = ["stub-server"]`, `crates/conductor-verify/Cargo.toml:29`). Re-verified
   at take-up HEAD `1208ca5`: `.github/workflows/ci.yml`, `scripts/agent-run.sh` and `scripts/agent-run.ps1` each
   name `stub-server` 0 times (the CARRY stated this at `88de180`). Epoch 5 diagnosis P14(b), overseer placement
   2026-10-04. [intent-incomplete, val-1: widened at P4 by the overseer's ruling to BOTH feature sets
   (`stub-server` + `live-pulse`). Basis: testing.md 2026-06-21 as extended 2026-10-03, and the per-feature clippy
   line test-plan §9 owes (`test-plan.md:391`). The gate is the clippy form, placed in `agent-run run`'s bundled
   default in both shells.]

## Boundaries

- Leaves `v3-09`'s matrix status (`deferred` against the founder's NOT-deferred ruling) to the `v3-09` series' chunk —
  the founder's word; never a write here.
- The split and the lift are behaviour-preserving: every one of the 102 tests the file holds today still exists after
  the split, with the same assertion and the same pass/skip/ignore posture, and the rule section between its markers is
  byte-identical. No series is re-run against Pulse and no committed evidence or capture changes; the sha256 digest
  pins over committed captures (security.md, the 2026-09-30 remedy) hold byte-equal.
- [premise-corrected: no live command names the target — `--test real_model_harvest` appears 0 times in
  `scripts/agent-run.{sh,ps1}` and `ci.yml` (python scan of `git ls-files`, research.md Sweep record); a target
  rename happens only if P4 picks separate targets] Every PATH citation of where the rule and the grading live
  (architecture.md:71 · obs-plan.md:221 · security-plan.md:121 · test-plan.md:260 · tests-summary.md:22 · the posture
  contract `:144` · `drift.rs:55` · the scenario's `:27` comment · matrix v3-10's `ref`) is kept true by the chosen
  shape: unchanged if the rule stays in `real_model_harvest.rs`, re-pointed (code/contract sites) or carried as wrap
  amendments (masters) if it moves.
- Out of scope: `delegated_timing_harvest.rs` (1130 code lines, M2's other entrant), `real_model_live.rs` (the 21 L
  clone pair with `span_landing_live.rs`), `scenario.rs` / `execute.rs`. The entry names only
  `real_model_harvest.rs`.
- The secret-scan skip must not let the gate pass vacuously where its subject DOES exist: it triggers only when the
  tree has no git repository, never when `git ls-files` fails inside one, and the gate's vacuity guards (`:284-296`)
  stay. Verified that a skip cannot turn CI green: the `Secret-scan gate` step runs `test -n "$(git ls-files)"`
  (`ci.yml:79`) before the test and exits 1 on an empty subject.
- Both `stub-server` forms compile at HEAD — `cargo check --tests -p conductor-verify --features stub-server` and
  `cargo clippy -p conductor-verify --features stub-server --all-targets -- -D warnings` each exit 0. Whether the
  gate also RUNS `preflight_spawn.rs`, and whether it covers `live-pulse` too, is a P4 decision.

## Causal claims carried (closed at P3)

- "`conductor-core/tests/secret_scan_gate.rs` panics on `git ls-files` exit 128 in cargo-mutants' `.git`-less
  copy, so `conductor-core` was unmeasurable by mutation" — **measured there**, spot-checked: the audit's
  `_mutants-conductor-core-nocopyvcs.log` records the unmutated copy-mode baseline panicking at
  `secret_scan_gate.rs:185:5` ("`git ls-files` exited exit status: 128 — the gate has no subject"), and the assert
  sits at `:185-189` at HEAD `1208ca5`.
- [premise-corrected: compiles at HEAD `1208ca5` — both forms above exit 0; the claim described the state before the
  2026-10-03 fix-loop repaid it (test-plan history, `2026-10-03-p-075-re-round-on-incident-events`)] "the
  stub-server-gated preflight_spawn.rs is built by no CI job or harness verb; uncompilable for seven weeks" — the
  diagnosis's RR case (`2026-10-03T23:30:42Z-c`). The first half still holds (no gate builds it), so the gate is a guard
  against recurrence, not a fix.
- "the real-model harvest test grows one grading arm per series and per assertion" — the audit's suspected shape,
  **verified**: the three dated series (`:1810-1993`, `:1995-2197`, `:2199-2479`) each repeat one eight-test shape
  (capture loader · digest · rule recorded · no fingerprint · `measured_*` · grades as the ledger · eleven keys ·
  v3-09 not met).

## CI verdicts read at Setup (5a; the last wrap's flip `07c8f11` through HEAD)

- `1208ca5` — **verdict not yet available** (in progress, CI#37199673078, checks 3/3)
- `88de180` — green · checks 3/3 · wall 540 s (CI#37169522759)
- `b4bfe7e` — untested — pushed under a later tip
- `07c8f11` — green · checks 3/3 · wall 665 s (CI#37168119947)

No red and no `not green`: nothing to fold.

## Operator directives at take-up (overseer relay, from the Epoch 4/5 triage)

- Any architecture draft is measured with `scripts/arch-registry-check.py measure` BEFORE it is written (the script
  exists at take-up: `cmd_measure`, `measure (--rev REV | --file PATH)`; the arch history records ~124 B of headroom
  left in §Established Decisions at the last wrap).
- Before the supply-chain gate, the local advisory-db residue is self-healed (untracked files cleaned), or the audit runs
  against a fresh `--db` clone (diagnosis L11; security.md's 2026-08-09 corrected rule). Measured at take-up: 0
  porcelain lines, HEAD `ef6173cb` (2026-10-03).
- The code graph is live on this host (rust plane `fresh` at this run's query).
