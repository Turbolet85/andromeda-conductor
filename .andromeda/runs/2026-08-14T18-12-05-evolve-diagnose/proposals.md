# Evolve Diagnosis — Conductor · conductor-0.2.0 Epoch 2 "Live-path enablement" · 2026-08-14

Target epoch chosen in dialogue: the latest fully-complete epoch (8 chunks, all frozen, all master
records `complete`). Obligation-free: accept, reject, defer or modify anything here with no
mechanism-side consequence. Nothing was applied; nothing is remembered.

---

## Mechanism health

**220 records** (112 step / 108 friction) across **8 chunks** + 15 new-session records ·
**0 unparseable** (447-line ledger read whole with explicit UTF-8).

- **Checkpoint coverage: complete.** Every chunk fired all 13 expected step records
  (phase 5 · implement 3 · wrap 5). No gaps — no checkpoint silently failed to fire this epoch.
- **Problem-fact fill:** 45/112 step records (40%) carry ≥1 deviation fact; **53 facts** total —
  workaround 25 · prohibition 13 · overridden 8 · removed-cause 6 · deferred 1.
- **Untyped rate: 31%** (34/108 friction records), spread across all 10 step types — highest at
  new-session/orientation (6), implement/smoke (5), wrap/reconcile (5), phase/research (4).
  A third of observed friction does not fit the shipped vocabularies; §Playbook-extension
  candidates proposes four types that would absorb 24 of the 34.
- **Calibration boundaries in range:** none. The deviation scan carries a `problem` key from
  ledger line 1 (2026-08-08), so the whole epoch is inside the scan's era — no record here may be
  read as "nothing occurred".

A note on this document's own method: the cross-epoch recurrence checks below were run as a
**keyword scan over Epoch-1 `what` lines**, which is indicative, not exact. Where a claim rests on
it, it says so. (Proposal P-U2 is precisely about claims made from a narrower basis than the
claim — the diagnosis should not commit the pattern it reports.)

---

## Proposals (typed patterns)

11 of 34 groups cleared the F-2 thresholds. Ordered by frequency × impact weight.

### P1 — phase/plan · `ambiguity.scope-question` — 5 cases · weight 11 · rate 0.62
**Pattern:** five of eight chunks needed an operator dialogue round at plan synthesis; in four of
the five, the records state that only *some* of the questions asked were genuinely undecidable
from the materials — the rest had already been leaned on decisively by an artifact.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-10-workspace-key-divergence-probe | detection approach unsettleable from materials — Conductor cannot observe the app-side key (no tool argument, no log, corpus off-limits), a genuine operator trade-off | dialogue 1 | q-typed.json |
| 2026-08-10-pulse-run-contract | 2 questions, 1 round; the L4-observation fork genuinely the operator's, the P-073-backing fork **already leaned on decisively by `UNBACKED_AUTO`** | dialogue 1 | q-typed.json |
| 2026-08-10-scenario-run-root-span-tree | 2 questions, both genuinely the operator's (subscriber-layer depth = a cost/scope judgment) | dialogue 1 | q-typed.json |
| 2026-08-13-dispatcher-determinism-goldens | 3 questions; envelope-term genuinely operator's, **golden tier leaned dispatch-tier by the materials**, third a judgment | dialogue 1 | q-typed.json |
| 2026-08-13-per-check-read-back-extraction | 3 questions, 1 round; **only one genuinely undecidable** — the other two had material leans | dialogue 1 | q-typed.json |

**Proposal:** the skill already prescribes marked-recommendation shape for leaning questions, and
the records show it being used. What recurs is *bundling* — 2–3 questions per round where one is a
true fork and the others are confirmations of a material lean. A direction: have P4 separate
"decide" from "confirm" in the question set (or state the lean and proceed unless contradicted),
so the operator's attention lands on the genuine fork. Note this is a cost-shape change, not a
defect fix — every one of these rounds resolved in a single exchange.

### P2 — wrap/route-resolve · `ambiguity.trajectory-halt` — 2 cases · weight 11 (halt-qualified)
**Pattern:** both halts are the same shape — the markerless tail could not be adapted factually
because a *blocked-elsewhere* condition made the ordering question a judgment.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-10-workspace-key-divergence-probe | new-chunk-ahead-vs-pin: the orphaned `conductor-report` spans (zero `tracing::` calls across five files vs obs-plan §4 CP1) needed an owner | dialogue 1 · **halt 1** | q-level-raw.json |
| 2026-08-13-first-live-green-preflight | every Epoch-3+ live entry blocked on the same three Pulse-side pieces — insert Conductor-side work, advance non-live entries, or leave for Pulse | dialogue 1 · **halt 1** | q-level-raw.json |

**Proposal:** both halts were correct and cheap (one round each). The recurring input is
"downstream entries are blocked on a dependency outside this project" — a state the route grammar
cannot express, so it reaches the operator as an open-ended trajectory question every time. A
direction: let a working-route entry carry a *blocked-on* annotation naming the external
dependency, so the halt presents a known-shape choice rather than being re-derived. (The third
occurrence just happened at this epoch's close, recorded as untyped — see U-appendix.)

### P3 — wrap/reconcile · `contract.cascade-miss` — 4 cases · weight 4 · rate 0.5
**Pattern:** in half the epoch's chunks the fan-out's proposal set was incomplete, and the gap was
closed only by the orchestrator's own first-hand grep — twice because the amended wording was
**duplicated inside the same master**, once because a detector returned a bare `proposals: []`
against an amendment the plan had already listed as expected.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-10-scenario-run-root-span-tree | obs-plan carried the SAME falsified cleanup claim twice (§4 CP1 :304 and Known-residual :354); the detector proposed one | extra_reads 2 | q-typed.json |
| 2026-08-11-faithful-emission-dispatcher | the cross-master grep found the amended wording verbatim in **five masters + two distillations**, where the single proposal named one | dialogue 1 | q-typed.json |
| 2026-08-13-per-check-read-back-extraction | verbatim apply would have shipped an internally inconsistent obs-plan (retired span wording duplicated at the §1 table row) | iterations 1 | q-typed.json |
| 2026-08-13-first-live-green-preflight | obs-plan detector returned bare `proposals: []`, missing the §6 amendment the plan had listed as expected | — | q-typed.json |

**Proposal:** the mechanism that caught every one of these is already in `amendment-flow.md` — the
cascade's same-master duplicate grep and the expected-amendments floor. Both fired reliably; the
*detectors* are what under-produce. A direction: since the doc-agent sees only the report and its
own doc, a proposal naming a section could be paired with a mandatory "grep this doc for the old
wording, report every hit" step inside the agent prompt, moving the duplicate-catch upstream of
the orchestrator's manual pass. (This epoch's own wrap hit it a fifth time — see L4.)

### P4 — phase/validate · `contract.intent-divergence` — 3 cases · weight 4
**Pattern:** validation-1 classified intent-incomplete and amended `scope.md` in three chunks;
in the third, **the amendment was itself wrong** and needed re-amending after operator correction.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-10-pulse-run-contract | scope named the storm shape as contract content to add; the plan found it already satisfied and redirected the term | — | q-typed.json |
| 2026-08-11-faithful-emission-dispatcher | plan modifies `conductor-timeline` while scope's surfaces list named only run/emit/core/scenarios | — | q-typed.json |
| 2026-08-14-canary-fingerprint-feed-capture | the intent-incomplete amendment inferred the observed zero was consistent with a working path **without checking committed evidence that refuted it** | dialogue 1 · iterations 1 | q-typed.json |

**Proposal:** see X1 — the structural driver is that `scope.md` is authored at P1 *before* research
runs at P3, so it necessarily carries unverified premises. The direction belongs at the chain level,
not here.

### P5 — wrap/curation · `ambiguity.filter-borderline` — 8 cases · **rate 1.0**
**Pattern:** **every single curation run in the epoch produced a borderline call.** Five distinct
filter mechanics are implicated, and three cases record a high-value learning scoring *below* the
apply threshold.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-10-workspace-key-divergence-probe | two survivors scored **exactly 0.6**, the threshold; Filter 5's cap then forced a tiebreak at equal confidence | deferred 1 | q-typed.json |
| 2026-08-10-pulse-run-contract | Filter-1 duplicate rejection against a target the filter does not name (the cascade had written it into a rule's **main body**, not a Session Additions block) | — | q-typed.json |
| 2026-08-10-scenario-run-root-span-tree | Filter 4's "repeated pattern (3+ instances)" **does not define the grain of an instance** | — | q-typed.json |
| 2026-08-11-faithful-emission-dispatcher | the serde-flatten gotcha scored 0.2 — one technical-detail signal, no correction, no repetition — despite being a real trap | — | q-typed.json |
| 2026-08-11-faithful-emission-dispatcher | the garde skip-vs-dive finding rejected at Filter 1 **only because P2's amendment had written it minutes earlier** — ordering decided the outcome | — | q-typed.json |
| 2026-08-13-dispatcher-determinism-goldens | the tokio timer-granularity gotcha scored 0.5 against the 0.6 bar | — | q-typed.json |
| 2026-08-13-first-live-green-preflight | a high-value SUT fact (corpus.db has no span table, so span-level post-mortem is structurally impossible) scored **~0.2** | — | q-typed.json |
| 2026-08-14-canary-fingerprint-feed-capture | the gauge-vs-cumulative learning brushed Filter 3's conflict test; resolved as refinement under Filter 1's tie-breaker | — | q-typed.json |

**Proposal:** a rate of 1.0 says the filters are not calibrated for this project's learning
profile. The consistent shape across cases 4, 6 and 7: **an agent-discovered technical gotcha with
no user correction and no repetition structurally cannot clear 0.6** — Filter 4's large positive
signals are all user-sourced (+0.4 correction, +0.5 explicit request), so a fact discovered by
verification alone tops out at ~0.4. A direction: add a signal for *verified-by-measurement*
(a finding that changed the chunk's design or falsified a spec claim), and define the grain of a
"repeated instance". Two secondary directions the records name: Filter 1 should consider rule
main-bodies (not only Session Additions) as dedup targets, and its outcome should not depend on
whether P2's cascade ran before P3.

### P6 — implement/fix-loop · `contract.spec-reality-gap` — 4 cases · weight 2 · rate 0.5
**Pattern:** in half the chunks a gate surfaced a spec claim that implementation reality
contradicts. All four were surfaced, none authored — the wall held.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-10-pulse-run-contract | v2-18's acceptance reads as though preflight asserts both terms; only the L4 declaration is observable, the shared-data-dir term is not | — | q-typed.json |
| 2026-08-11-faithful-emission-dispatcher | the load envelope names *this chunk* as where its exemptions retire; the plan declined it | deferred 1 | q-typed.json |
| 2026-08-13-dispatcher-determinism-goldens | `scheduler.rs` documents a phase's elapsed time as exactly its jittered gap — true for the reported stream, false for clock consumption | iterations 1 | q-typed.json |
| 2026-08-13-per-check-read-back-extraction | obs-plan §4 names span attributes absent from the redact allowlist — they could never have emitted | — | q-typed.json |

**Proposal:** the surfacing discipline works. The observation worth the founder's eye is the
*source* distribution: three of four gaps are a spec asserting something about **the SUT or the
runtime that no one measured** before writing it. The project has independently curated this lesson
three times (testing.md's "measure the mechanism" chain). A pipeline-side direction: at P4, a spec
claim that a plan step depends on could carry an explicit "verified against artifact / transcribed
unverified" mark, making the unverified ones visible before implement meets them.

### P7 — implement/code · `input.plan-step-ambiguous` — 3 cases · weight 2
**Pattern:** a plan step prescribed something that could not be executed as written — twice because
the prescribed shape conflicts with the crate's actual dependency graph.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-10-workspace-key-divergence-probe | "leave the five shipped legs untouched" while two of them assert the exact string the chunk replaces | — | q-typed.json |
| 2026-08-10-pulse-run-contract | step 3 prescribed the binary edge resolve the contract path, but the two entrypoints take no contract path | extra_reads 1 | q-typed.json |
| 2026-08-14-canary-fingerprint-feed-capture | the step-3 seam could not compile — `opentelemetry-proto` is dev-only in `conductor-run` and `conductor-emit` re-exports no proto type | iterations 1 · extra_reads 1 | q-typed.json |

**Proposal:** all three are the plan prescribing an API shape without the dependency/visibility
facts that decide it. The code-graph is queried at P3 for *callers*; a direction is to also record
the target crate's **dependency kind** (normal vs dev) and re-export surface for any type a plan
step names in a signature — the fact that decided all three cases.

### P8 + P9 — `tooling.gate-deferral` at wrap/gates (5) and implement/fix-loop (4)
**These are one phenomenon counted at both ends.** All nine cases are the same `cargo audit`
advisory-database fault, re-verified byte-identically each time. Full lifecycle analysis in **L3**;
no separate proposal here.

### P10 — wrap/report · `contract.detector-fact-gap` — 4 cases · weight 1 · rate 0.5
**Pattern:** in half the chunks the report carried a fact no shipped Changes bullet fits, so the
detectors structurally could not see it.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-10-workspace-key-divergence-probe | a SUT-side binary built as a probe precondition is neither a Conductor dev tool nor a lockfile dep | — | q-typed.json |
| 2026-08-10-pulse-run-contract | three moved derived counts, but derived-count detectors exist only for layout/test/design — two of the three are stated in `architecture.md` | — | q-typed.json |
| 2026-08-13-dispatcher-determinism-goldens | the chunk inverted *which envelope term is asserted* — a qualifier all three derived-count detectors structurally cannot see | extra_reads 1 | q-typed.json |
| 2026-08-13-first-live-green-preflight | the headline outcome was **a spec claim disproved by measurement**; the nearest bullet (Spec-master edits) is for edits *applied* | — | q-typed.json |

**Proposal:** two concrete gaps the cases name: (a) a **derived-count detector for `architecture.md`**
— three exist, arch is not among them, and arch demonstrably states moved counts; (b) a Changes
bullet family for **a spec claim disproved by measurement**, distinct from an applied edit. Both are
drift-base/report-template additions the founder would apply.

### P11 — phase/research · `input.extract-signal-gap` — 3 cases · weight 1
**Pattern:** the extracts did not name the file or blocker that decided the chunk; research found it
by going beyond the signalled set.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-10-pulse-run-contract | no extract named `conductor-run/src/lib.rs`, which holds the entire canary emission path | extra_reads 1 | q-typed.json |
| 2026-08-10-scenario-run-root-span-tree | no extract predicted the central blocker — `JsonObsLayer` implements only `on_event`, so the shipped spans emit nothing | extra_reads 3 | q-typed.json |
| 2026-08-11-faithful-emission-dispatcher | the obs extract asserted the span topology was "already fixed"; code showed otherwise | extra_reads 5 | q-typed.json |

**Proposal:** see X4. All three share one cause — the distillers read **specs, which describe target
state**, and cannot see whether the code already does it. This is the same lesson the project
curated on 2026-08-09/-08-11; the pipeline-side generalization is the proposal.

---

## Cross-step chains (starting heuristics)

55 hypotheses; **8 shapes recur across ≥2 chunks**. Full detail in `q-chains.json`.

### X1 — phase/take-up →`scope`→ phase/plan (5 chunks) · →`scope`→ phase/validate (4 chunks)
The widest structural finding. `scope.md` is authored at **P1, before research runs at P3**, so it
carries premises nothing has verified; P3 then falsifies them and P5's validation-1 absorbs the
correction as "intent-incomplete". Consumer verdicts are explicit: *"repeats the intent's
marker-less-temp-dir claim, which research falsified"* · *"asserted the three shipped spans 'are
emitted'; research falsified it"* · *"Term C named the storm shape as missing; research found
`emit_canary` already emits 6"*.

**Hypothesis:** this is ordering, not carelessness. The mechanism already anticipates it (`[inferred]`
tags at promotion; P4's "re-verify inferred portions first"), and the tags were used — yet val-1
still classified intent-incomplete in 3 chunks and amended scope in 4.
**Proposal direction:** either accept it explicitly (rename the val-1 outcome so a routine
post-research scope refresh is not an anomaly), or add a cheap P3 step that re-reads scope's
`[inferred]` bullets against research findings and rewrites them *before* P4 consumes scope.

### X2 — phase/plan →`plan`→ implement/smoke (4 chunks)
Every case is the same defect: **the Test Commands omit what actually produces the evidence.**
*"listed the bare `agent-run.sh run`, which does not fire the scenario leg"* (×2) · *"omitted its
hard precondition — the sidecar binary"* · *"listed the bare verb as the producer for the
emission_count criterion"*.
**Note:** the project curated a Tier-2 rule for exactly this on 2026-08-11 — and it recurred twice
afterwards, including in this epoch's final chunk (the `RUST_LOG` form and the
`logs/agent-latest.jsonl` criterion). A curated rule did not stop it.
**Proposal direction:** make it mechanical rather than remembered — P5's mechanical checks already
WARN when a criterion names an artifact no listed invocation produces; that check could be
promoted from WARN to a required-resolution item, since 4 chunks show the WARN is not enough.

### X3 — phase/research →`research`→ implement/code (6 chunks — the widest)
The Files-to-modify boundary list under-covers what implement needs. Correlates exactly with the
band-aid theme L1c (5 workarounds where implement edited outside the list as an "in-scope enabler").
**Proposal direction:** the recurring omission class is narrow and nameable — **manifests**
(`Cargo.toml` dev-deps) and **allowlist/registry files** (`redact.rs`) that a listed change
transitively requires. A P3 rule to include the manifest whenever a step adds a test-only dependency
would have covered 3 of the 5.

### X4 — phase/distill →`extracts`→ phase/research (4 chunks)
The extracts are formally valid (7/7 every time) but describe **target state**, so they cannot tell
research whether the code already does it. See P11.

### X5–X8 — `scope`→implement/code (5) · `plan`→implement/code (5) · and two re-joins
These are X1 and X3 observed at a further hop; the heuristics do not do transitive chains, so they
appear separately. No independent proposal.

---

## Level candidates (systemic-masked-as-project)

### L1 — **band-aid** — four themes at threshold
Pass-A note-theme clustering over the 53 problem facts; each theme crosses steps and skills.

**L1a — the shell-discipline convention is knowingly routed around (5 facts, process/environment).**
`codebase-research.md` prescribes inline Python → a scratchpad file run by path. Four facts record
using `python - <<PY` heredocs *anyway* (phase/research ×2, phase/plan, phase/validate); a fifth
records four chained heredocs **failing at bash parse time** and being redone as scratchpad files.
**Hypothesis:** the convention is correct and the cost of honoring it (a file write per probe) is
high enough that it is skipped until it breaks. The cause lives in the ergonomics, not the discipline.
**Direction:** a tiny helper that runs a heredoc body as a temp file transparently would remove the
trade-off rather than restating the rule.

**L1b — Windows cp1252 on the Bash boundary (3 facts + 6 untyped frictions, environment).**
Facts: routed around after a `UnicodeEncodeError` killed a route command · master record appended
via a Python UTF-8 write instead of a shell append · a friction append believed corrupted and
hand-repaired. Frictions: cp1252 mis-decode of the ledger · epoch-key instability · two heredoc
backslash failures · a subshell-scoping slip.
**Hypothesis:** Andromeda's route/spec text is em-dash and arrow dense, and every project artifact
is UTF-8, but the host shell defaults to cp1252 on both ends. **This is a pipeline/environment
cause absorbed one chunk at a time, in nine separate places.**
**Direction:** one environment fix (force UTF-8 for the tool's shell + Python I/O) would retire the
whole theme; the per-step workarounds cannot.

**L1c — research's boundary list, patched by implement (5 facts).** See X3.

**L1d — do-not guards authored into plan Implementation notes (9 prohibitions).** Six at phase/plan,
two at phase/validate, one each at implement/smoke and wrap/reconcile. Each steers implement off a
design the plan rejected (*"do not re-implement Pulse's `workspace_detector::detect`"* · *"do not add
a ReadyState field"* · *"do not build degraded_mode as a request"* · *"never accept a regenerated
insta snapshot"*).
**Hypothesis:** `plan.md` says WHAT, and the *rejected-alternatives reasoning* has no home — so it
is re-encoded ad hoc as prohibitions, once per chunk.
**Direction:** a plan-template section for rejected options + why (one line each), making the guard
a structured field rather than prose the next chunk re-derives.

### L2 — **chronic-degrade** — `tooling.*` recurring across both epochs without halting
`tooling.gate-deferral` E1=3 → **E2=9** (total 12) · `tooling.environmental` E1=3, E2=3.
Neither ever halted. The first is L3; the second is the L1b environment theme surfacing as typed
friction. **Observation, not a proposal:** the halt policy structurally cannot surface either, which
is exactly the class this signature exists to name.

### L3 — **deferred-forever** — the `cargo audit` deferral, 16 consecutive re-pins
**Facts:** 7 `solution: deferred` facts (6 in E1, 1 in E2) · **16 `deferral-open` signals** spanning
both epochs · `wrap/gates` emitted one at **every wrap in the epoch** · `counts.deferred` E2:
fix-loop 7, gates 7. Nine typed friction records (P8+P9) carry the byte-identical evidence:
`duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1, on 0.22.2 — the latest published.
**Hypothesis:** this is the signature working as designed *and* the case it was written for. The
deferral is correct at every step (an advisory-DATABASE fault has no floor to raise to; the
audit↔deny overlap is verified green each time; the operator ratified it under L5 on 2026-08-10).
**Its closure has never appeared, and cannot: nothing in the loop can heal an upstream data file.**
The mechanism has absorbed 16 re-verifications of a fact that has not changed since 2026-08-08.
**Direction (the founder's call, and the one with real cost):** the bounded wait's re-verification
is now the most-repeated single action in the ledger. A direction: replace per-chunk re-verification
with a **dated re-check interval** (e.g. weekly) plus a cheap upstream probe, so the standing basis
is confirmed without spending a gate cycle in every chunk. The overlap signal (`cargo deny`) already
runs independently and green.

### L4 — **override** — the same steps corrected across both epochs
`phase/validate` n=3 (E1+E2) · `wrap/reconcile` n=3 (E1+E2) · `wrap/route-resolve` n=2 (E2).
Two distinct shapes hide under one solution word:
- **Genuine rule miscalibration (validate, ×2):** the operator corrected the plan's
  Expected-amendments routing when it named *immutable* files (intent.md / requirements.md), and
  held the note-only precedent over refining a matrix acceptance. Both are the same underlying
  question — which channel absorbs a correction — and the second was resolved by a precedent the
  mechanism does not encode.
- **Pre-settlement, not correction (5 facts, wrap ×4 in one chunk + the L5 ratification):** the
  operator supplied directives *at invocation* that pre-decided steps the wrap would otherwise halt
  on. This is not the operator correcting a bad call; it is the pipeline's halt points being
  predictable enough to answer in advance.
**Hypothesis:** the first shape says the amendment-channel rule is under-specified (which artifacts
are immutable, and when a matrix note beats a text refinement). The second says the halt design is
working but front-loadable.
**Direction:** encode the immutable-artifact list and the note-vs-refine precedent into
`playbook.md` (the first shape has now recurred across two epochs); and consider whether wrap should
*accept* pre-declared dispositions as a first-class input rather than as prose the orchestrator
interprets.

---

## Playbook-extension candidates (untyped patterns, F-4)

34 untyped records (31% of friction). Four clusters clear the threshold and would absorb 24 of them.

### U1 — host shell / encoding trap — **8 cases** → proposed type `tooling.host-shell`
Cluster: `grep -c` exits 1 on zero matches, truncating an `&&` chain · a `tail` pipe reported the
pipeline's exit code, not the tool's, and `TMPDIR` is unset in Git Bash · bare `open()` mis-decoded
the ledger as cp1252 · `UnicodeEncodeError` on U+2193 killed a route command · the ledger's epoch key
is encoding-unstable · two heredoc backslash failures · a subshell-scoping slip.
Recurs from Epoch 1 (≥1 clear precedent: the heredoc backslash case).
**Draft criteria line:** *"Did the host shell's semantics, quoting, or encoding defaults corrupt,
truncate, or mis-decode a command or its output? (count reformulations; name the mechanism, not the
command)"* — owning playbooks: all four (it fires everywhere).

### U2 — claim asserted from a narrower basis than the claim — **7 cases** → proposed type `contract.narrow-basis-claim`
Cluster: a fanout-results count stated 4/1 where its own list showed 3/2 · a graph row count read off
the screen instead of the trace's `rows` field · an Expected-amendments entry naming three docs
**asserted from memory rather than a grep** (only one was true) · a mechanical check reporting four
false MISSes from an over-broad command · a coverage query reading `status` at the wrong nesting
level and returning a plausible 0-verified · a non-anchored regex matching commented-out blocks and
returning a plausible-but-wrong tally · a step reported blocked after searching only the repo when
the artifact was on disk elsewhere.
**Every one produced a plausible-looking wrong answer** and was caught by re-derivation, not by the
result looking wrong.
**Draft criteria line:** *"Was a count, absence, or availability claim made from a source narrower or
less authoritative than the claim itself — and would it have passed unnoticed if not re-derived?"*
**Note:** the project curated this lesson at Tier 1 on 2026-08-09 ("Before asserting that document A
says X, grep A"); it recurred 7 times in the following epoch. A curated rule did not stop it, which
is itself the finding.

### U3 — an authored premise falsified by verification — **5 cases** → proposed type `contract.premise-falsified`
Cluster: intent F10's acceptance premise (the workspace detector records the marker as a *field*, not
a success condition) · matrix cap v2-08 asserting an acceptance over a model that does not exist ·
a CARRY predicting both exempt scenarios would pass, computed false over all 35 scenarios · obs-plan
§4 describing a read-back shape the live SUT does not have · a proposed post-mortem read that is
structurally impossible (no span table in `corpus.db`).
Partially covered by the typed `contract.spec-reality-gap` (implement) and `contract.intent-divergence`
(validate) — but these five were recorded at **phase/research and phase/plan**, where neither type is
offered.
**Draft criteria line:** *"Did verification falsify a premise an authored artifact (intent, scope,
matrix acceptance, CARRY, spec) states — and which artifact owns the correction?"*

### U4 — a mechanism's structural blind spot — **4 cases** → proposed type `contract.structural-blind-spot`
Cluster: the cascade DAG cannot reach stale content inside a `## Session Additions` block · a leaf
distillation stated the amended fact in *different words*, so the grep for the amended phrase
returned clean · health-criteria Check 4 has no rule for a rule file that deliberately carries no
frontmatter · P2's aggregate check C compares extracts to each other and cannot catch extract-vs-SUT
conflicts.
**Draft criteria line:** *"Did a documented mechanism fail to reach something by construction —
where no amount of correct execution would have caught it?"* These are the highest-value records in
the ledger for improving Andromeda itself, and they currently have no type.

---

## Below threshold — no action

**Typed groups (23), n=1–2, no halt impact:** implement/fix-loop `contract.test-expectation` (2) ·
implement/smoke `retry.smoke-reentry` (2) · implement/fix-loop `tooling.environmental` (2) ·
implement/smoke `tooling.harness-friction` (2) · implement/code `input.research-files-wrong` (2) ·
implement/code `ambiguity.scope-pressure` (2) · phase/validate `contract.mechanical-check` (1) ·
phase/validate `contract.matrix-claim` (1) · wrap/reconcile `ambiguity.playbook-no-match` (1) ·
wrap/reconcile `tooling.environmental` (1) · implement/smoke `contract.spec-reality-gap` (1) ·
phase/research `input.cookbook-gap` (1) · wrap/curation `contract.marker-write` (1) ·
new-session/orientation `input.handoff-git-mismatch` (1) · phase/take-up `input.working-entry-thin` (1) ·
wrap/route-resolve `contract.carry-no-owner` (1) · phase/distill `input.spec-source-gap` (1) ·
implement/code `input.conventions-gap` (1) · wrap/curation `ambiguity.tier-routing` (1) ·
phase/plan `input.research-thin` (1) · phase/plan `input.extracts-conflict` (1) ·
wrap/report `recall.change-reconstruction` (1) · wrap/reconcile `input.report-insufficient` (1).

**Untyped clusters below F-4:**
- **large-artifact read exceeds the output cap** — 2 problem facts + 2 untyped frictions, all on
  `working-route.md` (32.9KB; `cat` and a grep pipeline both overflowed). **Borderline:** 2 facts is
  below the n≥3 bar, but 4 total observations describe one obstacle, and no Epoch-1 precedent was
  found. Worth watching next epoch — the file only grows.
- **plan named work already satisfied** — 2 cases (a StubConfig knob shipped tests already provide;
  a contract term `emit_canary` already meets).
- **plan Test Commands under-specify a precondition** — 2 cases; subsumed by X2.
- **singletons (6):** the obs layer's `on_record` blind spot · a superseded-not-dismissed proposal
  class · an operator-directed CARRY decision needing extra reads · a near-miss hollow `implemented`
  · the trajectory question that surfaced without a halt (this epoch's close — the third occurrence
  of P2's shape) · a matrix cap outside every distiller's input set.

**Level themes below threshold:** none — the four Pass-A themes at threshold are listed in L1; the
output-cap theme appears above.
