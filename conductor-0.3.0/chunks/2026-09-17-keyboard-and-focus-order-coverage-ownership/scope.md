# Scope — Keyboard and focus-order coverage ownership

**Marker:** `2026-09-17-keyboard-and-focus-order-coverage-ownership`
**Version:** conductor-0.3.0 · **Epoch 3** — The a11y capability's terminal
**Working entry:** `conductor-0.3.0/working-route.md:39`
**Capability targeted:** `v3-03` — *Keyboard and focus-order coverage has a stated owner*.
**`[amended at P5 validation-1, 2026-09-17: v3-03 is NOT CLAIMED by this chunk — it stays pooled.]`** Its
acceptance quantifies over *every* keyboard/focus claim ("…and the claim is **actually asserted** in that
suite"), and P4 measured **three §5 claims asserted by no suite at all** — `:354` and `:356` coverage-matrix row
navigation and `:370` the first-class shortcuts. Closing those is new keyboard coverage, out of this chunk's
scope. Per `verification-matrix-contract.md` §Acceptance lifecycle a cap not provable by THIS chunk stays
pooled with one `notes` line; concretizing the acceptance to admit a third "recorded gap" terminal would weaken
the outcome. See `plan.md` §Verification ledger for the derivation.

---

## The chunk in one line

Every keyboard-reachability and focus-order claim the requirement makes gets exactly one named owning
suite, and the claims whose owner is a suite CI never runs get a stated carve-out saying what gates in
their place.

## What this chunk builds

1. **A per-claim ownership statement.** For each keyboard/focus claim the requirement makes, the owning
   suite is named, and the claim is actually asserted in that suite. The entry scopes the *work* to the
   half that is currently unattributed — **the hold-dependent trap and restoration claims** — but the
   acceptance is quantified over *every* claim, so the unstated-but-already-correct half must be
   attributed too rather than left silently implicit. `[inferred: the "for every … claim" quantifier is
   v3-03's acceptance text, not the entry's line; the entry names only the hold-dependent half.]`
2. **A CI carve-out statement.** The hold-dependent claims' owner is `operator-hold.e2e.ts` — a suite CI
   does not run and by design cannot (it needs a live preflight-ready Pulse). The requirement must state
   what gates in its place rather than leaving those claims unattributed. `[inferred: the suite's identity
   and its never-a-CI-gate status are read from `operator-hold.e2e.ts:1-7` this phase.]`
3. **The CARRY 1 dittography repair** — folded below, operator-directed, coordinates re-verified.

**`[amended at P5 validation-1, 2026-09-17 — intent-incomplete, justified: the DELIVERABLE grew a mechanical
half the scope did not anticipate.]`** On the operator's P4 ruling the ownership statement is not prose-only:
/implement commits a claim→owner **enumeration** beside the suites (`claim-ownership.ts`) plus a **checker**
(`check-claim-ownership.ts`, run by `tsx`) that holds it to reality — every named owner's spec must exist and
must contain at least one assertion, so a title-only spec can never stand as an owner. The `a11y-plan.md` prose
(the §5 per-claim sentences, the §11 carve-out, CARRY 1, F3, F4) rides wrap's amendment flow, since
`plan-template.md` §Discipline bars every spec master from a chunk's modify-set. This is architecture's standing
shape for a coverage claim — a statement plus a gate holding it to exact-set equality.

## The claim population this chunk must attribute `[inferred]`

Read this phase from `.andromeda/a11y-plan.md` §5 *Keyboard Navigation* (`:348-376`) and §3 *Keyboard test
harness* (`:112`). None of these bullets currently names an owning suite — that absence IS the gap:

> **`[premise-corrected` at P3, 2026-09-17: two rows below were wrong. See research.md §Scope premise closure.]`**
> `:353` is **already attributed** (it carries "**Asserted UNATTENDED on the routine arm since 2026-09-07**"),
> so the genuinely unattributed set is `:354`, `:355`, `:362`, `:366`, `:370` — five rows, not seven.

| Claim group | Site | Present owner |
|---|---|---|
| Focus order per layout — `run-console-idle` | `:353` | **already attributed to the routine arm** `[premise-corrected]` |
| Focus order per layout — `run-console-live` | `:354` | unnamed |
| Focus order per layout — `run-console-HOLD` | `:355` | unnamed (asserted in the driven suite) |
| Focus order per layout — `idle-with-report` | `:356` | unnamed |
| Skip links | `:359` | N/A by construction (no repeated nav blocks) |
| Focus trap — HOLD `alertdialog` (SC 2.1.2, SC 2.4.3) | `:362-363` | unnamed (driven suite) |
| Focus restoration — dialog close → triggering control | `:366` | unnamed (driven suite) |
| Focus restoration — no route-change surface | `:367` | N/A by construction |
| Per-surface keyboard shortcuts | `:370` | unnamed |

The hold-free Operable pair is already asserted in the routine arm — `accessibility.e2e.ts:408` (SC 2.1.1
reachability) and `:424` (SC 2.4.3 idle focus order) — ~~so for those two the work is attribution, not
relocation.~~ **`[premise-corrected at P3, 2026-09-17: the easy reading is FALSIFIED. The routine arm also
carries the hold-dependent specs at `:450` and `:456`, and `:450` is VACUOUS BY CONSTRUCTION — its title names
four claims (alertdialog role, focus trap, Escape resolves NoGo, focus restores) and its body is the
`this.skip()` guard alone, 0 `expect(` calls over `:450-455`. So attribution of those four claims to the
routine arm would not merely be unnecessary — it would be FALSE. research.md F1.]`**

## Boundaries — what this chunk is NOT

- **Not a relocation of the hold-dependent assertions into the routine arm.** They cannot run there: the
  hold fires only behind a READY preflight gate with a live Pulse. The acceptance offers *two* honest
  terminals — routine-arm ownership OR named-driven-suite ownership plus a carve-out — and only the
  second is reachable here. `[inferred]`
- **Not new keyboard or focus-trap assertions.** The assertions exist in both suites; what is missing is
  the requirement's statement of who owns what. Writing new coverage would be a different chunk.
- **Not a change to what CI runs.** The `a11y` job's spec set reached its measured terminal at the
  predecessor; widening it is out of scope and would re-open a just-closed capability. `[inferred]`
- **Not CARRY 3's remediation** — see the fold below: it is re-measured and carried, not actioned, and
  the entry does not ask this chunk to fix it.
- **No Rust surface, no dependency delta expected.** `[inferred]`

---

## An open question this scope records rather than settles

**Which channel may write the ownership statement?** The requirement is a spec master
(`.andromeda/a11y-plan.md`), and `/andromeda-phase` and `/andromeda-implement` are both read-only against
spec sources — spec amendments are wrap's flow. v3-03's acceptance turns on the *requirement's* text, so
a naive plan would claim a capability whose decisive artifact only wrap can author, which the matrix
contract's claim-reachability rule forbids (the coverage gate would HALT by construction).

CARRY 1 is an operator-ratified precedent for this chunk editing `a11y-plan.md` directly — but its stated
justification is narrow ("a master-body defect with no report fact behind it and no detector that covers
it"), which does not obviously extend to substantive spec content that the amendment flow does cover.

This is a genuine fork, not a detail: it decides whether v3-03 is claimable by this chunk at all, and if
so through which artifact. **It is carried to P4 for resolution and, if contestable there, to the
operator.** `[inferred — the fork is derived from the skill contract + the ledger's claim-reachability
rule, and is stated nowhere on the entry.]`

---

## Measured this phase — coordinate re-verification (promotion.md's fold-as-hypotheses rule)

Both CARRYs arrived as hypotheses; both named coordinates were re-verified against the artifacts before
they shaped this scope, and **both hold exactly**:

- **CARRY 1's coordinates hold.** `a11y-plan.md:115` is **3691 characters** (3728 bytes — the char count
  is the CARRY's basis, and the 37-byte gap is this line's multibyte `·`/`—` run, not drift). The
  duplicated clause `the one webview-automation stack running on the measured platform SET — ` sits at
  offsets **2555 and 2627**, exact-phrase, the 72-character delta matching the clause's own length.
- **CARRY 3's census holds.** Re-run this phase: **6 `msedgewebview2` survivors, every one at StartTime
  `2026-09-16 18:33:31`** (pids 11112, 22612, 27104, 30520, 33428, 33580). The new-session pass and this
  phase added none.
- **`v3-03`'s acceptance and `observed_gap` read as the entry implies**, confirmed from the ledger via the
  matrix tool rather than by loading the JSON.

**The CI fold (promotion.md's second fold source, Setup 5a):** `git log -1 --format=%H` is
`3ddd405c2556003006f3747804173fbf951414b5`; the check-runs API answers **`No commit found for SHA`
(HTTP 422)** — the wrap's commit is **not on the remote yet**, exactly as the handoff states ("this wrap's
commit leaves it 1 ahead. The operator pushes"). This is the documented not-on-the-remote reading, a
**no-op**: there is no completed conclusion to disposition, and nothing folds into this scope from CI.

---

## Folded annotations (verbatim freight from `working-route.md:39`)

### CARRY 1 — `a11y-plan.md:115` verbatim dittography

> `a11y-plan.md:115` carries a verbatim dittography — the clause "the one webview-automation stack running
> on the measured platform SET — " stands twice inside one line, at offsets 2555 and 2627 of a
> 3691-character `:115` — **coordinates RE-MEASURED 2026-09-17** (they were 866 and 938 in a 2002-character
> line; this wrap's §1 CI-integration amendment moved them, and a stale offset is exactly what this CARRY
> exists to prevent — re-measure before using it, never trust the last number). Fix it while this entry has
> that file open. It is a master-body defect with no report fact behind it and no detector that covers it,
> so it has no routine amendment channel; a line-granular read shows one hit and cannot see it, which is
> why the offsets are carried here (operator directive, 2026-09-12 wrap).

**Absorbed as work.** The entry directs the fix into this chunk. The coordinates re-verify (above), so the
repair is a bounded one-clause deletion inside a single multi-KB line — which per the host rule file is a
python read-modify-write by path, never an anchored Edit.

### CARRY 3 — `msedgewebview2` orphan hygiene

> **CARRY 3** — `msedgewebview2` orphan hygiene, RE-MEASURED 2026-09-17 and still open: **6 survivors,
> every one at `StartTime 2026-09-16 18:33:31`**, i.e. ~17 hours older than that session and NOT produced
> by it. The same session's THREE `--e2e` leg runs (a red baseline, the post-fix green, and a reverted
> perturbation control) left ZERO survivors of their own — wdio's `onComplete` teardown held on every one —
> so the defect is NOT the routine arm's teardown path and a fix aimed there would be aimed at the wrong
> thing. Inherit the dated count, not the description: re-census by `StartTime` before concluding anything,
> since a live leg's own children are indistinguishable from these by name alone.

**Absorbed as a carried measurement, not as work.** The entry asks for a re-census before any conclusion,
not for a remediation, and this chunk's own surface (a requirement statement) cannot produce or reap these
processes.

- `[inferred — causal-mechanism claim preserved verbatim per promotion.md's marker-text rule]` **"so the
  defect is NOT the routine arm's teardown path and a fix aimed there would be aimed at the wrong thing"**
  — carried at its RE-MEASURED marker. This phase's re-census reproduces the count and the StartTime
  exactly, which is consistent with the claim but does not independently establish *where* the defect is;
  P3's premise closure sets the re-verification depth.
- This chunk runs no `--e2e` leg of its own, so if survivors appear during it that is itself new evidence —
  re-census by `StartTime` before attributing any of them to this chunk.
