# Session Handoff

**Last Updated:** 2026-09-11T12:51:30Z
**Branch:** `build/conductor-0.3.0` — **NEW this session**, cut from 0.2.0's final commit `b54e6ec`. Tracks
`origin/build/conductor-0.3.0`. The operator pushed mid-session (commit `b7ef1bd`), so the branch was **0
ahead at wrap start** and is **1 ahead and unpushed** after this wrap's commit. That push was **load-bearing**
— it is what produced the CI run this chunk's whole deliverable reads.
**Status:** clean
**Last Commit:** `feat(2026-09-11-hosted-runner-endpoint-cause-probed)` (this wrap)

## Position
- Done: **`2026-09-11-hosted-runner-endpoint-cause-probed`** — three read-only probes added to the `a11y`
  job as a `continue-on-error` diagnostic, driven on the hosted `windows-2025` runner at CI run
  `34586959536`, and the readings recorded.
- Next: **`Hosted-runner endpoint cause closed`** — minted this wrap, directly after this chunk in Epoch 1
  on the operator's directive. It owes exactly the two measurements this chunk could not make. It carries
  `PREREQ: close rust gate deferral (deferred since 2026-09-11-hosted-runner-endpoint-cause-probed)`.
- Coverage **0/11 verified · 11 unclaimed** — `v3-01` was **UN-CLAIMED** (see below), so it returns to the
  pool at `planned`. Version `conductor-0.3.0` is 1 chunk in.

## What the chunk measured

Two of three candidates **retired**, one new lead **named**, one probe **unread**:

| candidate | reading (hosted runner vs dev-host control, same script) |
|---|---|
| WebView2 / Edge **policy** | **RETIRED** — all five keys ABSENT on *both* sides; no difference exists to explain one |
| **Session** property | **PARTLY RETIRED** — `SessionId 2`, `UserInteractive: True`: not session 0, not a service context |
| **Module version** loaded | **UNREAD** — no live process; the preceding isolation step stops the app **0.645 s** earlier |

The new lead: the runner runs **elevated** (`IsElevatedAdmin: True`), the dev host does not. A **difference,
not a demonstrated cause** — no probe varied it, and nothing here says de-elevating opens the endpoint.

## v3-01 — UN-CLAIMED, on operator directive
Not a refine. The acceptance clause ("the readings are together sufficient to **decide** remediable vs
permanent") is what makes the capability worth holding, and this version exists to retire closures that rest
on wording. The chunk's reading stands as committed evidence; the follow-up entry claims `v3-01` when the
measurement is whole. The WHY is recorded in the ledger `notes`.

## Drift resolved
**6 proposals across 2 docs · all 6 applied · 0 escalations · drift = 0.** Five of seven detectors returned
`proposals: []`.
- `architecture.md` ×3 — §Established Decisions [CI/CD]'s "hosted-image policy or session property the
  leading unmeasured candidate" **retired** (routine under playbook `:149`; clause (c)'s PARTIAL-result arm
  tested and correctly did not fire) · `TEMP` + `LOCALAPPDATA` registered on the shipped-artifact-READS
  basis (**pre-existing gap**, not introduced here) · the probe script in the directory tree.
- `test-plan.md` ×3 — the same retirement at §9 Matrix builds, **plus its §6 duplicate restatement** caught
  by the `dependent-of` mechanism (a single-site apply would have left the disproved pair alive in the
  driver table) · the "two `continue-on-error` DIAGNOSTIC steps" literal de-literalized to a **SET**.
- Cascade: **0 leaves re-derived**, verified not assumed. Claim sweep returns 0 across all seven masters
  **with a known-positive control still firing** in a sidecar.

**A report defect a detector caught:** the report named `a11y-plan.md` §9 as a co-owner of the
diagnostic-step-set fact with **0 hits** in it. The fact has one owner, `test-plan.md`. Corrected at Validate
check 5 before it could become a hand-raised amendment against a doc that states nothing.

## Curation
**T1 0 · T2 2 · T3 0** (cap 3 not reached; 3 filtered).
- `verification-harness.md` — **extended in place**: a chunk with no producing leg should take the plan
  template's *omit* arm, not mint-then-read; a listed-but-vacuous smoke is worse than a stated absence.
- `host-win32.md` — **new**: a drive-letter host-path anchor also matches the PowerShell registry provider
  form; fix the OUTPUT (colon-free reg.exe rendering), never the pattern.

### Deferred learnings — `recurrence-despite-learning` ×3 (logged, deliberately NOT re-curated)
Each already stated correctly in the corpus; a further copy is not a remedy.
- The evolve heredoc append died on `Invalid \escape` over backslash values — `evolve-system.md` already
  REQUIRES the python-dict raw-literal form for exactly that case.
- The report named a spec master as an owner with zero hits in it — the report template already bans this
  in those words.
- A `-o` pattern clipped its matches and a raw count was misread before the hits were printed — CLAUDE.md
  and `host-win32.md` both already cover it.

## Notes
- **The commit and push happened mid-implement and were the OPERATOR's act.** `v3-01`'s acceptance needs a
  reading from a CI run of committed code, and CI fires only on push, while `/andromeda-implement` reserves
  committing for wrap. The skill **correctly refused to do it itself** and surfaced the tension. Recorded as
  a deviation with that attribution, not as a defect.
- **A ledger claim did not reproduce.** `residuals.md` said the routine arm's red had *moved* to tauri-driver
  never listening on `:4444`. Run `34586959536` holds **zero `ECONNREFUSED`**; the driver connected and
  reached session creation, and the red is the endpoint. The annotation is now re-scoped to say the refusal
  is **intermittent** and that `v3-02` re-measures rather than inherits. Operator re-derived the zero count
  independently.
- **A pipeline gap with no owner, surfaced not patched:** a `residuals.md` entry measured stale has **no
  routine correction channel** — that file scopes route-resolve to *appending* open entries and gives
  dispositions to route Phase A. The working-route's `BLOCKED-ON` clause has exactly this re-verify-and-rewrite
  mechanism; residuals has no counterpart. Corrected this time only by explicit directive.
- **Gate deferral open:** `cargo nextest --workspace --profile ci`, deferred under the source-delta rule
  (zero `.rs` / `Cargo.*` delta — re-verified against the actual diff, not echoed). Pinned as a PREREQ on the
  next entry with its origin.
- **Last failed command:** none.
