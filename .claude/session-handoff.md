# Session Handoff

**Last Updated:** 2026-08-09T14:25:00Z
**Branch:** build/conductor-0.2.0
**Status:** clean
**Last Commit:** 2026-08-09-current-sut-coverage-classification — Current-SUT coverage classification (four modes, 82 capabilities, KNOWN_UNCLASSIFIED retired)

## Position
- Done: **2026-08-09-current-sut-coverage-classification** — every capability the SUT manifest accepts is classified into exactly one of **four** modes (`CoverageMode::NotConductors` is new, wire `not-conductors`): **82 = 43 auto · 16 drive+observe · 7 static-only · 16 not-conductors**. The not-Conductor's set is explicitly enumerated (Pulse's UI/visual set + its own tooling), `KNOWN_UNCLASSIFIED` is retired to `[]`, and no literal count or P-ID range survives in any workspace assertion. `v2-03` verified — coverage **3/32**.
- Next: **Out-of-scope classification treatment** (`/andromeda-phase` to promote + plan). It carries a CARRY: the mode itself already shipped, so that chunk owns only the VISUAL treatment, and `layout-templates.md` §Component block 1 already documents the Mode cell + four values — extend it, don't re-author.

## Work done
9 source files across 4 crates + the webview mirror; 0 new files, **zero dependency delta** (`Cargo.lock` un-drifted). Gates green on the **first run, 0 fix-loop iterations**: nextest workspace **441/441** (+6, zero retries), core 192/192, doctest ok, `clippy -D warnings` clean across 9 crates, `cargo audit` exit 0, `cargo deny` all four classes ok, `agent-run.sh run` exit 0. Smoke drove the real binary: `conductor coverage` renders 82 rows with a tally summing to 82 and zero identifier leaks.

## Drift resolved
7 detectors → **3 proposals** (arch · layout-templates · test-plan), **1 escalation** resolved with the operator, **5 amendments applied** (3 proposals + 2 found by the W12 citation grep), **4 sidecar entries**, cascade closed to `CLAUDE.md GENERATED:setup:warnings`. Full record: `.andromeda/runs/2026-08-09T14-17-38-wrap/fanout-results.md`.

## Notes
- **Instance repair completed this wrap.** Three spec bodies (`architecture.md`, `design-system.md`, `layout-templates.md`) were edited during `/implement` because the approved plan listed them as touchpoints. The pipeline fix is encoded upstream (plans can no longer route spec masters through implement); this wrap completed the half that flow skipped — sidecar entries authored on the *did-this-body-change* trigger (not on the proposal set), cascade driven from the report's spec-change list, and the W12 grep anchored on the diff. That grep earned its keep: it found `test-plan.md:39` quoting arch as saying "60 claimed capabilities P-001..P-060", wording arch dropped on 2026-08-08 — a staleness **no detector reported**, because a detector reads only its own doc.
- **Operator decisions this chunk:** classification stays **code-native** (manifest ids-only, so `check_sut_drift` remains a genuine two-source comparison rather than a self-check) · `P-075` is **drive+observe**, since three of the four timing budgets it aggregates are already drive+observe · `layout-templates.md:222`'s sample caption folded in ("illustrative-sample status doesn't exempt a baked count") · D-layout-surface **applied now** rather than carried, accepting that the next chunk revisits the same block.
- **arch's re-aiming claim is now qualified.** "A manifest edit with no Rust change" became "a manifest edit **plus** a matching classification row" — the accepted set is still data, but `check_sut_drift` holds it set-equal to the code-native classification, so an unmatched id is a `CoreError::SutDrift`, never a silent gap. `CLAUDE.md`'s warning block cascaded to match.
- **Open question pinned to Epoch-6 (CARRY on *Coverage completeness gate*):** `coverage-matrix.md` has never existed in this repo, though arch §Occupied Resources registers it and test-plan §1 Path 6 names it as the gate's surface. That entry owns the call — commit it and gate on it, keep it on-demand and re-word the two specs, or gitignore it.
- **Worth knowing before the next Pulse re-aim:** the 22 capability titles are **not reproducible from this repo**. `P-061`..`P-078` came from Pulse's `andromeda-pulse-0.3.0/requirements.md`; `P-079`..`P-082` exist **only** in its `verification-matrix.json`, minted mid-build as operator-surfaced caps. This scored below the curation confidence threshold, so it lives only in the chunk report — noted here so it is not lost.
- **Curation:** T1 ×0 · T2 ×0 · T3 ×1 (`session-learnings.md` — illustrative samples are not exempt from stale-fact repair, and a sample's coupled counts must move together) · filtered 4 (3 tool-level pipeline rulings with no home in the project ecosystem, 1 below confidence). No conflicts, no deferrals.
- **`build/conductor-0.2.0` still has no upstream** — push with `git push -u origin build/conductor-0.2.0` when ready.
- **Last failed command:** none.
