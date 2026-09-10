# Adaptation Record — 0-pending wrap · 2026-09-10

**Path:** Setup step 6 — 0 pending, tree dirty only with expected-transient bookkeeping
(`.andromeda/friction-log.ndjson`, `.claude/session-handoff.md`), conversation carries an operator
route-adaptation request. No chunk to wrap: no P1 report, no P2 fan-out, no P7 gates.

**HEAD at entry:** `13a176f` · branch `build/conductor-0.2.0` · upstream `origin/build/conductor-0.2.0`,
1 ahead and unpushed (the prior chunk commit; not load-bearing — `ci.yml` byte-unchanged).

---

## Item 1 — new route entry (applied)

**Disposition: APPLIED.** One entry authored and inserted directly before *Release build and bundle*
(`working-route.md:133` at entry), which stays last.

**Entry core as authored** (route register: WHAT-not-HOW, 22 words, no implementation verbs):

> Live-Pulse in-lane scenario round — P-067, P-072 and P-079 each yielding a non-blocked live verdict,
> with journal and runs.db evidence per scenario

Authored in the route's own grammar rather than transcribed from the directive's intent line, per
route-resolve §Operator-requested adaptation. The three P-IDs are named in the core so the next reader
gets the subject without opening the matrix.

**Annotation:** one `CONTEXT (…)` introducer carrying the directive's three pre-verified facts and
nothing more. The placement rationale is the operator's and is recorded below, not in the annotation.

**Trajectory gate:** satisfied without a HALT. This is a new chunk ahead — trajectory by the P5 gradient —
but the operator's directive names BOTH the entry and its disposition (content, placement, and the
reason for the placement), which is the recorded pre-direction route-resolve accepts. The direction is
cited in the applied annotation.

### Directive facts re-verified against the artifacts this wrap

The directive presented these as pre-verified. Each was still checked against the artifact it names —
a dictated citation is where drift hides, and all four checks are cheap and in-repo. **All verified
exact; nothing to correct.**

| Fact | Check | Result |
|---|---|---|
| v2-04 acceptance text | `verification-matrix.json` | Matches the directive verbatim; `chunk:null`, `status:planned`, `method:dynamic-external` |
| `run --live` refusal string + no-leg behavior | `scripts/agent-run.sh:96` | Exact string match; the `conductor preconditions` probe leads and short-circuits before any leg. pwsh twin at `scripts/agent-run.ps1:161` (hyphen, not em-dash — a console-encoding difference, not a divergence) |
| P-067 scenario | `scenarios/live-only-service-truth.toml:20` | `p_ids = ["P-067"]` at that exact line |
| P-072 scenario | `scenarios/investigate-actions-functional.toml:21` | `p_ids = ["P-072"]` at that exact line |
| P-079 scenario | `scenarios/constellation-severity-live-wiring.toml:22` | `p_ids = ["P-079"]` at that exact line |

### Placement — the operator's reasoning, recorded so no reader re-derives it

*Release build and bundle* promises a final SLO verification pass "recorded against the shipped
artifact". A final pass over a binary whose in-lane live round has not run is final in name only, and a
defect the live round surfaced afterwards would stale the bundle that pass certified — costing a re-run
of the release gate on a later chunk. Running the round first also meets the version's last real risk
before packaging rather than after it.

### Annotation migration — checked, none owed

route-resolve requires that an insertion ahead of the previous first markerless entry re-pin that
entry's next-entry `PREREQ`s onto the new first entry, origins preserved. **Measured: *Release build and
bundle* carries 0 `PREREQ` and 0 `BLOCKED-ON`** — so nothing migrates. Its three `CARRY` annotations are
owner-pinned to that entry by their own text ("Land it with this entry's final verification pass";
"Pinned HERE because this entry's final SLO verification pass is the plausible owner") and correctly
stay with it. One of the three is already `DISCHARGED` (the `--e2e` exit-code leak, closed at
`2026-09-07-a11y-ci-gate`); the panic-hook race and the SR announcement variance remain open against it.

### Freeze contract

**Held.** The diff is exactly 2 insertions (the entry line + one `   ↓` separator). All **58** frozen
`[{marker}]` lines are byte-identical to `HEAD` — verified by differencing the frozen-line sets, not by
reading the diff stat. The new annotation's first strip boundary resolves to `   CONTEXT (`, so it will
flip-compact cleanly at its own P7 with marker and title inviolate.

---

## Epoch-growth valve — pre-directed, no halt

Epoch 6b now holds **14 entries** (12 frozen + 2 markerless). The valve's threshold (~10) is exceeded, so
it surfaces. The operator **pre-directed the answer in this directive: NO split** — the epoch closes at
*Release build and bundle*. Surfaced and answered in one pass; no second halt, per the directive's
pre-direction clause.

---

## Phases not run, with their bases

- **P1 report / P2 fan-out** — no chunk in flight (0 pending). A reality↔spec divergence noticed on this
  path would still wait for its chunk wrap; none was noticed.
- **P3 curation** — **did not fire.** The path runs curation only if the conversation carried
  corrections; the operator's directive opens by stating none is owed, and no candidate matches any of
  the five qualifying categories in `curation-guide.md` §What qualifies. The one friction event this
  session (a `cat`-heredoc script author blocked by the PreToolUse hook, re-authored via the Write tool)
  is **evolve telemetry**, which §Analysis scope excludes from curation by name.
- **Standing-amendment door** — the path MAY apply amendments whose subject is a fact this wrap itself
  measured. Four facts were measured (above); **none contradicts any master's claim**, so no amendment
  is owed and no sidecar is written.
- **P4 code-graph refresh** — not fired. This path's delta is source-free, so `tree.db.commit` is
  re-pointed to the new HEAD after the commit instead (Setup step 6), leaving the index fresh.
- **P7 gates / master flip / flip-compaction** — P7 does not run on this path. No master record is
  touched; no working-route line is compacted.

---

## Version-close arithmetic (carried to the handoff)

`matrix.py coverage` reads **28/32 verified · 1 deferred · 3 unclaimed** (`v2-04`, `v2-21`, `v2-27`),
`done-test: NO`. The operator has ruled that **nothing moves to 0.3.0** — Conductor is finished inside
this version, because the next work is Pulse and it waits on that.

The version now closes in **two chunks**:

1. **Live-Pulse in-lane scenario round** → claims `v2-04` (this adaptation's entry — previously ownerless)
2. **Release build and bundle** → claims `v2-27`

`v2-21` is `by-construction` and claimable at either chunk's P5. `done-test` flips only when all three
read `verified` or `deferred`.
