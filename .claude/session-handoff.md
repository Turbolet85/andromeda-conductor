# Session Handoff

**Last Updated:** 2026-09-07T17:15:21Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **0 ahead at Setup** — the
session's own push landed mid-session, so this wrap's commit makes it **1 ahead and unpushed**. The push
is the operator's act and is load-bearing this time: it triggers the a11y job's first run.)
**Status:** clean
**Last Commit:** `feat(2026-09-07-a11y-ci-gate): …`

## Position
- Done: **`2026-09-07-a11y-ci-gate`** (master `complete`).
- Next: **`/andromeda-phase`** on the first markerless head — **_SR findings fixed_**
  (`working-route.md:123`), minted at this wrap by operator directive. It carries CARRY 3's three
  measured SR findings AND the still-owed **verify the a11y job's first CI run** clearing event.
  No `BLOCKED-ON` on it — phase will not halt. The sibling is *Release build and bundle* (`:125`).
- Coverage **28/32 verified · 4 unclaimed** (`v2-04`, `v2-21`, `v2-24`, `v2-27`) — **unchanged: this
  chunk claimed NOTHING.** `v2-24` was deliberately left pooled at phase P5 with a failed-concretization
  note, because its requirement names keyboard PASS/FAIL *gated in CI* and the Operable carve-out puts
  that out of reach.
- **Evolve:** Epoch 6b at 9 chunks (7 frozen + 2 markerless) — under the ~10 split threshold, no nudge.

## Work done
The a11y specs are a CI gate: a third `ci.yml` job (`a11y`, `runs-on: windows-2025`) running the routine
webview arm with no live Pulse, plus the reused `journal_conformance` gate over `runs/a11y` and the
violation-record upload. Two structural properties are the substance, both measured rather than argued.
**The gate cannot key on `$?`** — measured 2026-09-02, `run --e2e` exits 0 on a total skip — so both shells
now assert the PRINTED verdict (`Spec Files:` failed count · the per-spec skip tally against the expected
set · the `[webview2 … windows]` banner). **The unset-handle skip cannot stay green in CI** — the new
`CONDUCTOR_A11Y_STRICT` inverts it to a non-zero exit, as an ADDED arm with the existing guards
byte-unchanged. Proven both ways on this host: strict + no driver → exit 1, lax + no driver → exit 0.

Five folded CARRYs landed with it: `knip.json`, the README platform reword (3 sites), the `[role="status"]`
→ `[role="alertdialog"]` guard, CARRY 4's a11y-plan `:565` clause, and the sibling's printed-verdict CARRY
(discharged here rather than at the release entry).

## Drift resolved
**36 amendments across 6 masters · 2 escalations resolved · cascade closed.**
- `a11y-plan` ×10 · `architecture` ×10 · `test-plan` ×9 · `security-plan` ×3 · `layout-templates` ×2 ·
  `obs-plan` ×2. `design-system` returned `proposals: []`, correctly.
- **Escalation 1 — `CONDUCTOR_A11Y_STRICT` had no governing rule.** It subject-matched two playbook rules
  and failed both their narrowing clauses (`:115` wants a PATH handle read SOLELY by wdio behind the
  isFile guard in an array-form spawn — four false; `:121` wants it SET by wdio and READ by a test binary
  — both false). Ratified as a **fourth handle class**; a bounding rule was minted, including the explicit
  NON-widening of the two PATH-handle clauses (`:210`, `:314`), which a flag handle does not belong in.
- **Escalation 2 — arch's `[CI/CD]` locked decision.** Rule `:97` failed its causal clause (no live SUT
  contradicted anything); ruled routine under `:28`, the live-Pulse invariant preserved and asserted.
- **The cascade caught what no detector could:** one of my edits landed INSIDE a11y-plan's verbatim
  quotation of arch, briefly attributing my wording to arch. Repairing it surfaced **four more** verbatim
  citations of the retired "build + test gating only" phrasing (3 a11y-plan, 1 security-plan) — a detector
  reads its own doc for ITS drift, so a quotation of another master's retired wording is drift in the
  CITING doc and nobody proposes it. All clear now.
- Leaves re-derived: `a11y-summary` ×3 · `tests-summary` · `stack.md` · `commands.md` · `rules/a11y.md`.

## Notes
- **The (obs) eleven-key criterion is UNMET AS WORDED, not silently met.** The artifact carries **13**
  top-level keys locally (15 under CI) — the eleven plus `service.name` + `deployment.environment`, which
  plan step 4 itself required. The keys are admitted by design (`journal_conformance`: *"extra keys are
  allowed — obs-plan §3 keeps the extension point open"*), so the artifact is correct and the criterion
  wording was not. a11y-plan §3 now records the real shape; obs-plan §3 records the extension point as
  **exercised for the first time**.
- **CARRY 5's premise was false and is corrected.** knip's "20 findings, all 20 one class" is wrong:
  only 8 were the wdio-discovery class. `lighthouse` and `axe-core` are TRUE positives of a different kind
  (a11y-plan §3-mandated by name and pin, neither imported anywhere), `@wdio/local-runner` a third
  sub-class. **Baseline for boundary #5's A5 `dead-code-web` column: 12** (3 exports + 9 types in
  `parse-nvda-log.ts` / `rows.ts` / `ScenarioPicker` / `CoverageMatrix`) — exported-but-unimported symbols,
  a usable series now. Nothing deleted.
- **Corrected mid-wrap:** the implement-stage record claimed mocha prints no skip summary. It prints
  `2 skipped`; the grep was for `pending`, mocha's internal term. Retraction is in the friction ledger and
  the T1 proxy-token entry gained the "grep what the tool PRINTS" facet.
- **One `hypothesis:` ships unmeasured** — the WebView2 Evergreen runtime on the `windows-2025` image. The
  Edge Driver itself is confirmed at `EDGEWEBDRIVER`; the first CI run measures the runtime.
- **Curation:** T1 0 new (2 extended) · T2 1 new (1 extended, 1 corrected) · T3 0. `CLAUDE.md` **134/200**.
- **Hermetic apart from the a11y legs.** Process census after both arms: 0 `conductor-tauri` · 0
  `msedgedriver` · 0 `node`; no `4444`/`4445` listeners. The six `msedgewebview2.exe` on this host belong
  to `SearchHost.exe` (Windows Search), not to the leg.
- **Last failed command:** none.
