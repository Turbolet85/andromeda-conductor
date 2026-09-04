# Adaptation Record — 0-pending wrap · 2026-09-04

**Path:** Setup step 6 no-op (0 pending; tree dirty only with expected-transient bookkeeping) +
P5 route-resolve §Operator-requested adaptation + P3 curation.
**HEAD at entry:** `480bc66` · branch `build/conductor-0.2.0` · 49 ahead of `origin` (`dd15dc3`).
**Directive:** operator ADAPTATION directive, 4 items, overseer facts measured at HEAD 480bc66.

## Context correction — verified first-hand before applying

The directive's premise was re-measured rather than inherited. All three facts hold:

| Claim | Measurement | Verdict |
|---|---|---|
| A Windows CI runner exists | `.github/workflows/ci.yml:18` (`rust` job) and `:164` (`frontend` job) are both `runs-on: windows-latest` | **CONFIRMED** |
| The a11y JOB is what is open | no a11y/axe/wdio/e2e job in that workflow; the sole match at `:182` is a comment inside the npm-audit step ("the a11y harness never ships") | **CONFIRMED** |
| CI has not run in a month | `origin/build/conductor-0.2.0` = `dd15dc3`, dated 2026-08-09; local branch 49 commits ahead, unpushed | **CONFIRMED** |

So the standing `BLOCKED-ON: a CI runner (none exists; Windows-only host by operator ruling
2026-08-22)` was **measurably false**. Its wording was inherited from the 2026-08-22 Linux-runner
block, conflating the HOST verdict (Windows-only — true) with a CI-runner claim nobody had checked.
The real dependency is an EVENT: **one green CI run of the a11y job after a push**, where the push
is the operator's act.

## Items applied

| # | Item | Disposition |
|---|---|---|
| 1 | MOVE *A11y CI gate* 6a head → 6b, directly before *Release build and bundle*; keep CONTEXT; rewrite `BLOCKED-ON` | **APPLIED** — now at line 117, immediately above *Release build and bundle* (119). Prose head kept verbatim; `BLOCKED-ON` rewritten to the measurable event with both measurements cited inline. |
| 2 | RE-PIN the two SR CARRYs (2a Guidepup-weighable · 2b OS-level key injection, 14 browse-mode rows) onto *SR findings remediation* | **APPLIED** — both moved **byte-identically** (verified against `HEAD:conductor-0.2.0/working-route.md`). They did not travel to 6b. |
| 3 | Standing cargo-audit PREREQ (the 49th) migrates to the NEW 6a head, *SR findings remediation*, unchanged in form and count | **APPLIED** — moved **byte-identical**, 850 chars, `this is the 49th` intact. No chunk ran this wrap, so the ordinal correctly does not advance. |
| 4 | Anchor-only: nothing else moves | **APPLIED** — 6a: *SR findings remediation* → *Sidecar spawn without a console window*. 6b: … → *Dependency polish* → *A11y CI gate* → *Release build and bundle*. Zero frozen `[{marker}]` lines in the diff. |

**Reading note on items 1 ∩ 4.** Item 1 says "directly BEFORE *Release build and bundle*"; item 4
lists "A11y CI gate → Release build and bundle → the rest as is". Read as a full 6b ordering, item 4
would move the other five entries, contradicting its own "nothing else moves". Read as the ADJACENCY
it creates, both items are satisfied by one placement — position 6 of 7, directly above *Release
build and bundle*, the other five untouched. That placement was applied; it also matches item 1's
stated rationale (the CI/release neighbourhood).

## Escalation raised and resolved WITH the operator

Item 4 kept a THIRD annotation on the moved line: the `CARRY (… plan §Implementation notes)` to
reword `crates/conductor-tauri/ui/test/README.md` ("This leg runs only on Linux + xvfb"). Its own
text said to do it *"here, the entry that next touches the ui test tree"* — true while *A11y CI gate*
headed 6a, **false** once it moved behind *SR findings remediation*, which re-runs the `sr*` leg in
6a. Left alone this would have shipped exactly the class of self-contradicting annotation this wrap
was convened to correct.

Surfaced as a single armed question (three dispositions, lean stated). **Operator selected: reword
the self-description in place.** Applied — the CARRY stays on *A11y CI gate* per item 4; only its
locating phrase changed:

- was: `Reword it here, the entry that next touches the ui test tree, to the measured platform set.`
- now: `Reword it at THIS entry, which touches the ui test tree, to the measured platform set.`

Consequence recorded honestly: the stale README survives the *SR findings remediation* chunk and is
fixed at *A11y CI gate* in 6b.

## Verification of the applied edit

- `git diff --stat conductor-0.2.0/working-route.md` → **3 insertions, 3 deletions**.
- Frozen-line guard: **0** `[{marker}]`-prefixed lines in the diff (freeze contract intact).
- Annotation fidelity re-derived from `git show HEAD:…` rather than trusted from the splice: PREREQ
  byte-identical (850 → 850); both SR CARRYs present in old and new; old `BLOCKED-ON` string absent
  from the new file.
- Structural re-read confirms the epoch layout above and that no entry was lost (markerless tail
  count unchanged at 9).
- Backup of the pre-edit file retained in the session scratchpad for the duration of the session.

## Curation — one Tier 1 in-place extension (recurrence, not a duplicate)

Candidate: *a route `BLOCKED-ON` annotation is claim surface that expires, and nothing sweeps it.*

Filter 1 returned a hard dedup hit on the Tier 1 entry `2026-08-22: **This host is Windows-only…**`
— and that hit **is the finding**. That entry's own clause (1) prescribes "annotate it `BLOCKED-ON`
and move it behind the entries that can actually run"; the false annotation was authored at the SAME
wrap that minted the learning. Under Filter 1's *Recurrence, not a duplicate* clause plus the
additive-facet tie-breaker, it lands as an **in-place extension of the matched entry** (same tier,
same location) rather than a near-duplicate sibling.

Confidence 0.8 (explicit operator correction +0.4 · verified by measurement +0.4). Tier 1 ×1,
Tier 2 ×0, Tier 3 ×0; 0 filtered. `CLAUDE.md` stays **133/200 lines** — line 131 extended in place,
2391 → 3834 chars, `USER:session-learnings` markers untouched.

The new facet the matched entry lacked: drift detection sweeps the seven `.andromeda/` masters and
**nothing sweeps the working-route**, so a false route annotation is invisible by construction and
its only reader is the promotion that HALTs on it. Hence: state a `BLOCKED-ON` as the measurable
EVENT that clears it, never as a standing state; carry the measurement pointer inline; re-verify on
every re-pin or move.

## Not for this wrap (operator-scoped out)

`v2-24`'s acceptance text still names "Linux+xvfb" — a stale premise for the claiming chunk's
concretization. Recorded in the handoff only, per directive. No matrix write this wrap.

## Not run on this path

No P1 report, no P2 fan-out, no P4/P7 gates — the no-op path runs none of them. No chunk was
wrapped, no master record written, no verification-matrix entry flipped, nothing spawned.
