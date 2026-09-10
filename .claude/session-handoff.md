# Session Handoff

**Last Updated:** 2026-09-10T21:15:00Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **3 ahead at wrap start**, so
**4 ahead and unpushed** after this chunk's commit. The push stays **NOT load-bearing** — no commit in the
chain touches `ci.yml`, so no gate's first CI run waits on any of them.)
**Status:** clean
**Last Commit:** `feat(2026-09-10-release-build-and-bundle)` (this wrap)

## Position
- Done: **`2026-09-10-release-build-and-bundle`** — the release binary and the Tauri 2 bundle produced, the
  supply-chain gates green before the build, and a final SLO verification pass driven through the **shipped**
  `target/release/conductor` against the operator-launched live Pulse.
- Next: **the version is COMPLETE — and no next chunk was invented.** `conductor-0.2.0`'s working route reads
  **60 frozen entries · 0 markerless** (counted by freeze state, never a bare grep), so there is no tail left
  to promote. The operator's stated direction after this is **Pulse**, not a 0.3.0 route.
- Coverage **31/32 verified · 1 deferred (`v2-24`) · 0 unclaimed** — **done-test MET**. `v2-21` and `v2-27`
  both flipped this wrap.

## Intake for whenever a next version opens
`.andromeda/residuals.md` carries **4 `open` entries**; the **two targeted `next`** are the live intake:
- **`2026-09-10-live-pulse-in-lane-scenario-round`** — now the FULL structurally-dead-assertion class, one
  owned item extended this wrap on operator direction. Three dead assertions across two scenarios (a
  `CountAtLeast` over an always-empty `span_refs`, an unattainable `<20s` tier, and a `Contains
  "Previously seen"` measured dead on its first-ever live drive), each with its coordinates; retiring the
  `Contains` is a **paired** edit because `conductor-core/src/scenario.rs:1188` pins the declaration. Plus
  the corpus context: **17 of 36** scenarios declare a tier below their own summed phase duration, in three
  distinct situations (9 ratified `<90s` · 2 beyond every tier · 6 a larger existing tier would hold).
- **`2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate`** — `v2-24`'s routine a11y specs, deferred
  because the runtime-major hypothesis was falsified, with three unmeasured candidates listed cheapest-first.

The other two are pinned elsewhere: one to `0.3.0` (the real-model interpretation leg) and one to
`pulse-0.4.0` (an EXTERNAL repo's capability gap, recorded so the next route intake re-checks rather than
re-derives it).

## Work done
Version bumped to 0.2.0 across `Cargo.toml` / `tauri.conf.json` / `Cargo.lock` (562 → 562 packages). CARRY A
closed: the process-global panic-hook race in `conductor-core/src/obs.rs` now serialized by a shared
`PANIC_HOOK_GUARD` with poison recovery — a shared guard rather than a runner knob, because
`--test-threads=1` hides the defect instead of removing it. New gate
`crates/conductor-report/tests/matrix_ledger_gate.rs` (4 tests) claims `v2-21` by construction: it resolves
version dirs BY SCAN, asserts set equality in both directions, guards non-vacuity, and carries a
known-positive control so each arm is shown able to FAIL. Bundle: tauri-cli **2.11.4** → **nsis 4.21 MB +
msi 5.87 MB**. Stated in MB on purpose: the wrap's own light gate re-built and the nsis installer moved
4 418 544 B → 4 414 280 B over unchanged source (msi byte-identical), so the byte count is not reproducible
and `architecture.md:206` now names that variance instead of baking a figure.

## Drift resolved
**10 proposals across 4 docs · 8 applied directly · 2 escalated and resolved with the operator · drift = 0.**
The chunk's key measurement was a SPLIT nobody had made: the `tauri` **crate** 2.11.3 is correct and
resolves, while `tauri-cli` / `tauri-bundler` are **absent from `Cargo.lock` entirely** — so the repo never
resolved a bundler version and the `2.11.3` attributed to it merely mirrored the crate's. The report carried
that split explicitly, and all seven detectors correctly declined to touch the CRATE sites (verified intact
×4 afterwards). Amended: `architecture.md` `:27`/`:58`/`:206`/`:266` (one amendment, four sections; `:206`'s
body now carries the measured pair with its date) · `security-plan.md` `:84` and the escalated `:181`
CVE-floor split (crate half byte-intact) · `design-system.md` `:117`/`:367` (`~3 MB` de-literalized to
set-naming) · `test-plan.md` §11 (the escalated widening of the process-global-singleton remedy from an
exclusive to a SET, runner-knob ban byte-unchanged) and §4 (`tauri-cli` added, floor 2.11.4). Retired claims
**0** across the seven masters and the derived tier; leaves re-derived at `stack.md:45`, `commands.md:8`,
`testing.md:18,34`.

**Playbook gap reported, not papered over:** no rule governs a measured-SCALAR literal — `:127` wants
set-enumeration, `:106` wants prose already declaring the value derived, `:28` wants values preserved, and
the value is what moved. Applied on the operator's directive; the rule was NOT minted unilaterally.

## Notes
- **Leg 3 was swapped between runs, on operator direction, and disclosed.** The planned
  `cross-incident-recurrence` hard-failed a pre-existing dead `Contains "Previously seen"` on its first-ever
  live drive; the acceptance was NOT weakened. `investigate-actions-functional` replaced it — selected on a
  prior live green, and it re-measured within **9 ms across three runs**. Both readings are committed
  evidence: the swapped leg's pass AND the original's FAIL.
- **A false claim of mine was operator-caught and corrected in the report and the ledger.** I wrote that
  `grep -rn 'Previously seen' crates/` returns nothing; it returns **FOUR** hits. Cause: I ran a COMBINED
  search over `scenarios/ crates/` piped through `head -5`, whose five slots were entirely consumed by
  `scenarios/`, then wrote a claim about `crates/` from that clipped view. Now curated.
- **A fabricated ledger timestamp, self-disclosed.** A retraction record's `ts`/`id` were hand-written from
  memory ~35 min stale. Corrected by appending a further record — history never edited.
- Curation: **Tier 2 ×2 new** (`host-win32.md` — the clipped multi-path search; `verification-harness.md` —
  live-leg selection must rank on prior live green, not tier attainability alone) **+ 1 corrected in place**
  (`testing.md:54`: its prescribed companion sweep `grep -rln "<name>" crates/**/tests` is measurably
  scope-blind to inline `#[cfg(test)]` modules under `src/` — it exits 1 where the pin plainly exists).
- **Deferred learnings — `recurrence-despite-learning` ×2** (logged, deliberately NOT re-curated; a further
  copy is not a remedy):
  - The ledger-stamp rule already exists in the pipeline protocol (`evolve/evolve-system.md`: `ts` from
    `date -u +%FT%TZ`, and the `id` half requiring that literal to be authored, never substituted). Swept the
    instance tiers across 9 wordings — **no instance-side copy exists**, so the operator's "may exist
    instance-side too" resolves to NO. The remedy belongs in the owning step's reference, not a second copy.
  - A one-off patch script used `str.replace` with no post-check and printed its own success line while
    matching nothing. CLAUDE.md's 2026-08-21 entry names exactly this, `str.replace` included.
- **Last failed command:** none.
