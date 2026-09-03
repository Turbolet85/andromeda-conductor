# Session Handoff

**Last Updated:** 2026-09-03T06:15:00Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **46 ahead** after this commit)
**Status:** clean
**Last Commit:** `feat(2026-09-02-mutation-tier-restored-for-conductor-tauri): the binary a build graph
finally guarantees, and the survivors its first score revealed`

## Position
- Done: **2026-09-02-mutation-tier-restored-for-conductor-tauri** — the `conductor-tauri` mutation tier
  went from 43 planned / **0 tested** to 43 planned / **43 tested**, by moving the parity test to the
  package that declares the `conductor` bin so `CARGO_BIN_EXE_conductor` binds it.
- Next: **`/andromeda-phase`** to promote + plan **_conductor-tauri survivors dispositioned_** — minted
  this wrap at the Epoch 6a head, ahead of the `conductor-run` sibling. It carries the **standing
  cargo-audit PREREQ, now the 46th**.
- Coverage **26/32 verified · 6 unclaimed** (`v2-04`, `v2-21`, `v2-24`, `v2-26`, `v2-27`, `v2-32`) —
  unchanged; this chunk claimed nothing, so the coverage gate was a no-op.

## Work done
The parity test relocated to `crates/conductor-cli/tests/cross_surface_parity.rs` and its CLI arm now
resolves through `env!("CARGO_BIN_EXE_conductor")` — a compile-time build-graph fact — instead of
`assert_cmd`'s `target/debug` fallback. Production code is **byte-unchanged**: every diff line in
`commands.rs` sits inside `#[cfg(test)]`. `assert_cmd`/`assert_fs` left `conductor-tauri`'s dev-deps
(lock 564 → 564 packages, 2 edge lines, `cargo deny` green over the new lock). Measured both
directions: with `conductor.exe` absent, `-p conductor-tauri` went **exit 100 → exit 0**, and a cold
`CARGO_TARGET_DIR` run passed **49/49** (368 packages compiled — which also settles the `ui/dist`
cold-compile question architecture had left open). `v2-25`'s `ref` followed the test.

**The tier's first score is the finding:** 43 tested in **4 m 18 s** at `--jobs 2` — and **21 stable
standing survivors** (18 missed + 3 timeout). The wrap light gate re-ran the tier on an identical tree
and one mutant (`pause.rs:85 TauriResolver::kind -> "xyzzy"`) moved missed → unviable, so the headline
reads 21 or 22 depending on the run; cargo-mutants' viability classification is not fully run-stable
here. Nothing was dispositioned by default; that is the correct reading of test-plan §10, and the work
is now owned by the entry minted at P5.

## Drift resolved
**7 sites in `test-plan.md`, one claim.** The retired claim: *the parity leg's first arm is a
`tauri::test` mock-runtime run*. It is not — the arm calls `conductor_run::{preflight, drive_run}` and
uses no `tauri::*` item.
- **4 detector-proposed** — `:287` (primary), `:288`, `:370` (duplicates), `:264` (the
  `cargo_bin` → `env!` driver fact). All routine under playbook `:28-30`.
- **3 orchestrator-raised.** `:80` and `:371` state the same mechanism in words carrying none of the
  swept tokens; `:93` carried neither those tokens nor the phrase the mechanism sweep used, and
  surfaced only in the **post-edit verification** pass.
- **1 escalation, resolved with the operator.** `:80`/`:93`/`:371` are Critical Path 7's *Verification
  signal*, sourced from the Creator Brief Must-Work — so playbook `:28-30` did **not** govern them (its
  qualifier is "contract preserved"). Operator chose the **split**: the envelope-equality half is
  recorded PROVEN, the control-panel-LAUNCHED half recorded DEFERRED to the tauri-driver leg and
  explicitly **still owed**.
- Cascade: `.claude/docs/tests-summary.md` re-derived (it carried the retired "Tauri-launched vs
  headless" wording, and was outside the rules/CLAUDE.md sweep — provenance enumeration caught it).
- Six of seven docs returned `proposals: []`.

## Notes
- **Adjacent finding, reported not applied:** `design-system.md:406` (decisions log) still says
  "Tailwind v4.1 `@theme`", which `:201` retires in favour of `:root`. It is surviving residue of the
  2026-06-15 amendment that playbook rule `:28-30` cites as its own founding precedent — pre-existing,
  outside this chunk's Changes. Yours to fold in whenever you like.
- **Pipeline defect worth the founder's attention:** the doc-agent prompt in `amendment-flow.md` tells
  the duplicate sweep to grep *"the WORDING your change retires"* — token-keyed. Three of this wrap's
  seven sites carried no swept token. The cascade's own step-2 text already prescribes sweeping the
  retired MECHANISM's phrasing; the doc-agent prompt was never brought into line with it. Logged to the
  friction ledger as `contract.skill-reference-drift`.
- **Two recurrence-despite-learning items** (logged, not re-curated — both mine, both this session):
  CLAUDE.md's *"graph lines are 0-indexed, grep's are 1-indexed"* — I cited graph rows as editor lines
  in `plan.md` and `research.md`, and you caught it at the P5 review; and `host-win32.md`'s *"no `rm` in
  a launch path"* — I issued exactly that compound and it was denied. Both entries are correct and
  present; neither was consulted at the moment of acting.
- **Disk residue for you (not processes):** the cold-run target dir
  `%LOCALAPPDATA%/Temp/claude/D--dev-projects-conductor/<session>/scratchpad/fresh-target-0903` —
  **3.0 GB**, left because `rm -r` is denied here; safe to delete by hand. Plus the small pre-existing
  `%TEMP%/conductor-core-run-journal-*` class.
- **Process hygiene:** re-measured against the host process list by name — **zero stragglers**. No
  listener opened; no Pulse, WebDriver or screen-reader process involved.
- **Curation:** Tier 2 × 2, both `testing.md` — one new entry (cargo-mutants writes tallies to
  `{dir}/mutants.out/`, so a pre-existing output dir is stale by construction; plus the 4 m 18 s budget
  basis), one in-place extension of the 2026-06-21 `CARGO_BIN_EXE` entry (the cross-package failure
  mode + the artifact-absent probe). CLAUDE.md untouched at **133/200**.
- **Last failed command:** none. (One Bash call was DENIED — a compound with `rm -rf` in a launch path;
  replaced by granular steps with a unique dir. Not a failure to retry.)
