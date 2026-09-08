# Adaptation Record — 0-pending wrap (session 129)

**Run:** `2026-09-08T17-07-54-wrap`
**Path:** Setup step 6 — **0 pending** in `master-route.md`, tree dirty only with expected-transient
bookkeeping → git-CLEAN by the session-state contract → no-op path.
**Session subject:** a `/andromeda-setup-project` re-run absorbing pipeline-template change **CE-2**
(commit `bf7da06`). No chunk was promoted, so there is no chunk to wrap.

---

## Dirt classification (the gate into this path)

| File | Class | Disposition |
|---|---|---|
| `.andromeda/friction-log.ndjson` | expected-transient (evolve telemetry) | absorbed by this commit |
| `.claude/session-handoff.md` | expected-transient (second-writer convention) | absorbed by this commit |

Both are named members of the bookkeeping set in `session-state-contract.md` §The second writer. No
out-of-chunk work in the tree → no HALT.

---

## What ran, and what did not

| Phase | Ran? | Basis |
|---|---|---|
| P1 report | **no** | no chunk — nothing to report on |
| P2 reconcile (fan-out) | **no** | the 0-pending path runs no fan-out |
| P2 carve-out check | **yes** | see below — the one reconcile-shaped act this path permits |
| P3 curation | **yes** | conversation-sourced, not git-sourced — the path requires it |
| P4 code-graph | **not fired** | source-free delta (zero `crates/` or `scripts/` source changes); `tree.db.commit` re-pointed instead |
| P5 route-resolve | **no** | no operator route-adaptation request this session |
| P6 state + handoff | **yes** | `state.yaml` bumped, handoff overwritten |
| P7 gates / master flip / coverage | **no** | 0 pending — no record to flip, nothing claimed |

---

## Carve-out check (facts THIS session measured vs. the masters)

The path permits amendments whose subject is a fact this wrap itself produced or measured — never
drift-DERIVED. The session's setup-project run produced one candidate fact: **CLAUDE.md no longer
imports `architecture.md`.**

Swept all 7 masters. A bare `CLAUDE.md` token sweep returned **0** — but a bare-token count is the
proxy check this project's own learnings warn about, so it was widened to alternate wordings that could
carry the same claim (`Tier 1` · `distillation` · `imports` · `@import` · `session-handoff` ·
`pointer table` · `.claude/`). That returned **9 hits, every one read**:

| Hit | Reading | Verdict |
|---|---|---|
| `architecture.md:163` · `design-system.md:201` | `@import "tailwindcss"` — CSS | false positive |
| `obs-plan.md:187` | "modules `conductor-emit` imports" — Rust | false positive |
| `security-plan.md:6` | "setup-project (materializes `@` imports + secure defaults per D22)" | **still true** — setup still materializes them, one instead of two; the sentence makes no claim about which files |
| `architecture.md:176` · `test-plan.md:502,609,610,612` | citations of `rules/verification-harness.md` and `rules/testing.md` | both files unchanged this session |

**No master is contradicted. Zero amendments applied, zero owed.** No `.andromeda/` sidecar written.

A reality↔spec divergence *noticed* on this path would still wait for its chunk wrap; none was found.

---

## Curation (P3) — 6 candidates, 1 applied

| # | Candidate | Filter outcome |
|---|---|---|
| A | The Bash tool's cwd **persists** across calls; host-win32's body clause saying it "does not persist" is false, and the hazard is inverted (a *remembered* cwd rebases later relative paths) | **APPLIED** — Tier 2, confidence 0.8 |
| B | On a setup re-run, validate the preserved tiers rather than re-render them | Filter 1 **duplicate** — `docs/session-learnings.md` 2026-09-08 already prescribes exactly this ("read each template for its SHAPE… change only what the trigger actually requires"). Followed correctly; not a recurrence |
| C | Run a host-tool probe from the Bash tool's own shell, never a native-process wrapper | Filter 1 **duplicate** — `host-win32.md` Session Additions 2026-09-08. Followed correctly (hook smoke 6/6) |
| D | Read a pattern's hits rather than refining the pattern | Filter 1 **duplicate** — CLAUDE.md Tier 1 (2026-09-02 / 2026-09-06). Followed correctly (the 9-hit sweep above) |
| E | Do not restore write-time `clippy --fix` on a setup re-run | Filter 1 **duplicate** — CLAUDE.md Tier 1 (2026-09-02), which named this exact scenario. Held |
| F | The 134-vs-135 line-count slip | Filter 2 **task-specific** — self-corrected, no durable rule |

### Candidate A — scoring and routing

Confidence **0.8**: verified-by-measurement +0.4 (a two-call `cd`/`pwd` probe falsified the clause) ·
specific-technical-detail +0.2 · **no-other-home +0.2**. The base signals total exactly 0.6 — the
contract's named mass point, which *rejects* — so the conditional signal is load-bearing. Its
preconditions were checked individually and all hold: this wrap runs no P2 (no master, playbook or
drift-base home), the fact is not carried on the route as a PREREQ/CARRY, and it is not a matrix
ledger note.

**Disposition is a §Corrections correction, not an additive learning** — the measurement made an
existing claim false rather than incomplete. It was written to `## Session Additions` rather than
edited in place, because the false clause lives in setup-project's *generated* body: an in-place fix
would be clobbered by the next re-render, and Tier 2 write logic forbids touching that region. This
extends Filter 1's generated-body carve-out — written for the additive-facet case — to the correction
case by analogy; the collision is logged as `contract.skill-reference-drift` for the pipeline's owners.

**Not also logged as `recall.corpus-recurrence`.** The body's *advice* ("prefer absolute paths over
`cd`") existed and was not followed, which has that shape — but the type's own rule states that an entry
this session's measurement disproves is a Corrections correction and not that type. One entry, one
disposition.

---

## Evolve

4 records appended at `2026-09-08T17:07:54Z` (ids `-a`…`-d`), `chunk: null`, epoch copied byte-exact
from `working-route.md:107`:

- `-a` **step** `curation` / `ok` — one `problem` fact (the correction routed around its documented
  in-place path). Four curated rules recorded as *signals*, not friction: each was consulted and
  correctly applied, and evolve-system requires a positive measurement to ride the step record.
- `-b` **friction** `contract.skill-reference-drift` — curation-guide leaves the
  correction-against-generated-body case unowned between two colliding rules.
- `-c` **friction** `ambiguity.filter-borderline` — the sole applied candidate rested on one
  conditional signal at the exact mass point.
- `-d` **friction** `contract.skill-reference-drift` — the shipped rules *template* states the false
  `cd` mechanic; corrected downstream, unfixed upstream.

---

## Route / master / coverage

- **master-route.md** — untouched. 0 pending; wrap's only sanctioned master write is a
  `pending → complete` flip, and there is no record to flip.
- **working-route.md** — untouched. No route-adaptation request; the markerless tail keeps its 2
  entries (*WebView2 runtime 152+…* with its `PREREQ: close rust gate deferral`, then *Release build
  and bundle*).
- **verification-matrix** — untouched. Nothing was claimed this session; coverage stays **28/32
  verified · 4 unclaimed** (`v2-04`, `v2-21`, `v2-24`, `v2-27`).

## Position on exit

Unchanged from the prior wrap: next is `/andromeda-phase` on the first markerless head,
*WebView2 runtime 152+ installed in-job* (`working-route.md:127`).
