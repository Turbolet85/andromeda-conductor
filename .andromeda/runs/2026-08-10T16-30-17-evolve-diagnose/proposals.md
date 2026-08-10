# Evolve Diagnosis — Conductor · conductor-0.2.0 Epoch 1 "Foundation: re-aim at the SUT" · 2026-08-10

Obligation-free. Accept, reject, defer or modify anything here with no mechanism-side consequence —
nothing is applied, queued, or remembered. A re-run recomputes from the ledger alone.

## Mechanism health

- **225 records** — 111 `step` / 114 `friction` · **0 unparseable** · 8 chunks · 7 session starts.
- **Step coverage: complete on every chunk.** 8/8 carry phase 5/5, implement 3/3, wrap 5/5. No
  checkpoint failed to fire anywhere in the epoch.
- **Untyped rate 29/114 (25%)** — highest at `phase/take-up` (4/7), `new-session/orientation` (5/8),
  `phase/plan` (4/13). A high untyped rate this early is the trap working, not a defect.
- **Problem-fact fill 40/111 (36%)** — 45 facts total.
- **Outcomes:** 105 `ok` · 5 `halted-resolved` · 1 `ok-degraded`.
- **Calibration boundary in range:** the deviation scan is live from Pulse record 26, i.e. before this
  epoch — every record here was captured under the current scan, so `problem: null` means "no
  deviation observed", not "not yet instrumented".

---

## Proposals (typed patterns)

### P1 — wrap-session/reconcile · `ambiguity.playbook-no-match` — 4 cases · weight 18 (top score)
**Pattern:** four escalation halts, each because an existing dismiss/reversal rule was **scoped to one
document family** and the misfire arrived in another — the rule's *reasoning* fit, its *scope* did not.

| chunk | what | impact | evidence |
|---|---|---|---|
| sut-capability-manifest | reversal rule 27 is scoped to a tech choice; this reversal was the SUT advancing past a locked constant | dlg 1 | q-typed.json |
| current-sut-coverage-classification | D-layout-surface on a long-undocumented Mode cell; all seven not-this-chunk's-drift rules scope elsewhere | dlg 1 · halt 1 | q-typed.json |
| interpretation-correctness-posture | D-security-input reproduced the known command-name over-reach, but that dismiss family is scoped to **arch** registries | dlg 1 · halt 1 | q-typed.json |
| sut-load-envelope | D-security-input escalate; nearest rule preconditioned on "added NO new external-input boundary" — this chunk added one | dlg 1 · halt 1 | q-typed.json |
**Proposal:** the playbook's rules carry an implicit `doc:`/`detector-family:` scope inside their prose.
A direction: give `playbook.md` entries an explicit **scope field** (which detectors / docs the rule
governs) and let the validate step match on it, so a rule written for one family can be *deliberately*
widened once rather than re-escalated per family. Three of the four halts were the same reasoning
arriving at a new doc.

### P2 — wrap-session/report · `contract.detector-fact-gap` — 8 cases · rate **1.0** (every chunk)
**Pattern:** the report template's Changes families had **no natural home** for a detector-relevant fact
— eight times, once per chunk, a different fact family each time.

| chunk | what | impact | evidence |
|---|---|---|---|
| sut-capability-manifest | an env handle written then deliberately reverted — a *negative* API fact | — | q-typed.json |
| dependency-advisory-remediation | two facts parked under Decisions & corrections for want of a bullet | — | q-typed.json |
| current-sut-coverage-classification | three changed files are **spec masters** — no bullet for that family | — | q-typed.json |
| out-of-scope-classification-treatment | first wrap where the plan records `Expected amendments`; no template slot | — | q-typed.json |
| interpretation-correctness-posture | Coverage row vocabulary is ✓/✗/n-a with no "assertable but unrunnable here" | — | q-typed.json |
| interpretation-correctness-posture | a dev CLI **tool** version change is neither dependency nor config key | — | q-typed.json |
| in-lane-sut-scenarios | no home for "a documented derived COUNT moved" | — | q-typed.json |
| sut-load-envelope | derived-count class recurred on a doc with no detector scoped to it | extra 1 | fanout-results.md |
**Proposal:** the template already tells the author to extend the bullets; what recurs is *which*
extensions. A direction: promote the recurring four to first-class bullets — **spec-master edits ·
derived counts/qualifiers moved · dev-tool version changes · reverted/negative API facts** — and widen
the Coverage row vocabulary beyond ✓/✗/n-a to include an "unrunnable in this environment" value.
(The derived-count bullet was added mid-epoch and is already paying: it fed three detectors at the
final chunk. This proposal is the generalization of that one success.)

### P3 — wrap-session/curation · `ambiguity.filter-borderline` — 8 cases · rate **1.0** (every chunk)
**Pattern:** every single chunk hit a Filter-1 or Filter-4 edge, and the recurring shape is one-sided:
**Filter 4 has no positive signal for a learning the orchestrator discovered and verified itself.**

| chunk | what | impact | evidence |
|---|---|---|---|
| sut-capability-manifest | 3 candidates ≈0.4, rejected, each load-bearing for the next chunks | defer 3 | q-typed.json |
| sut-drift-check | trace-fidelity learning dismissed as duplicate against a Tier-1 entry | — | q-typed.json |
| current-sut-coverage-classification | two-file SUT ledger finding scored ≈0.2 | defer 1 | q-typed.json |
| out-of-scope-classification-treatment | a gotcha discovered **by a real gate failure** fell below the bar | defer 1 | q-typed.json |
| out-of-scope-classification-treatment | survivor over Filter 1's token bar on shared vocabulary alone | — | q-typed.json |
| interpretation-correctness-posture | two real gotchas dropped below 0.6 | — | q-typed.json |
| in-lane-sut-scenarios | **no positive signal exists** for an orchestrator-discovered, codebase-verified convention | — | q-typed.json |
| sut-load-envelope | Filter-1 additive-facet call + a novel failure mode rejected at ≈0.2 | — | fanout-results.md |
**Proposal:** Filter 4's large positive signals are all **user-sourced** (+0.4 correction, +0.5 explicit
request, +0.3 repetition/always-never). A direction: add a positive signal for
**agent-discovered-and-verified** — e.g. "+0.3 when the candidate was proven against the codebase or
surfaced by a real gate failure, with the proof cited". Note this proposal is self-serving by
construction (the mechanism proposing a lower bar for its own findings) and deserves that scrutiny.

### P4 — phase/plan · `ambiguity.scope-question` — 3 cases · weight 7
**Pattern:** every case is "two questions asked in one round", and in each, **one was genuinely the
operator's and one was arguably answerable from the materials**.

| chunk | what | impact | evidence |
|---|---|---|---|
| sut-drift-check | Q1 gate-hardness genuinely operator's (materials actively conflict); Q2 leaned | dlg 1 | q-typed.json |
| current-sut-coverage-classification | representation choice operator's; the language question only leaned | dlg 1 | q-typed.json |
| in-lane-sut-scenarios | P-079 check shape operator's; the second constrained by the materials | dlg 1 | q-typed.json |
**Proposal:** no change is obviously warranted — asking a leaning question costs one round and buys
ratification, which the epoch's own escalations suggest is cheap insurance. Recorded so a *fourth*
occurrence can be read against a deliberate decision rather than drift.

### P5 — phase/distill · `retry.distiller-respawn` — 3 cases · weight 6
**Pattern:** the *same* per-extract check fails the *same* way each time — **Constraints anchored,
Acceptance-criteria bullets unanchored** — and always in `security` (3/3), sometimes with a second doc.

| chunk | what | impact | evidence |
|---|---|---|---|
| sut-drift-check | security + obs both failed check 3; Acceptance bullets carried no § anchor | retries 2 | q-typed.json |
| current-sut-coverage-classification | security + design, identical shape; both passed on re-spawn | retries 2 | q-typed.json |
| in-lane-sut-scenarios | security again: every Constraint anchored, no Acceptance bullet anchored | retries 1 | q-typed.json |
**Proposal:** the distiller prompt says "every Constraint + Acceptance bullet cites a plan §anchor" once,
in prose, while the *format examples* under Constraints show anchors and the Acceptance section's
example does not. A direction: add an anchored example to the Acceptance-criteria line of the prompt's
own format block — the cheapest possible fix for a 3/3-reproducible miss.

### P6 — phase/validate · `contract.matrix-claim` — 4 cases · weight 3
**Pattern:** the verification matrix's route-authored `acceptance` text repeatedly does not survive
contact with the chunk — four different failure modes, none of them affordance-honesty (the only
refinement the contract anticipates).

| chunk | what | impact | evidence |
|---|---|---|---|
| sut-drift-check | a P4 operator decision changed what the cap's pass condition *means* | — | q-typed.json |
| out-of-scope-classification-treatment | **no capability corresponds to this route entry** — 29 unclaimed read, none fits | extra 2 | q-typed.json |
| in-lane-sut-scenarios | sole matching cap needs live-Pulse, blocked on three Epoch-2 caps | defer 1 | q-typed.json |
| sut-load-envelope | the acceptance rests on an assumption research falsified | extra 1 · dlg 1 | fanout-results.md |
**Proposal:** the contract permits refining `method`/`acceptance` only for affordance honesty. A
direction: name a second sanctioned refinement — **"acceptance rests on a premise the chunk's research
disproved"** — with the same discipline the affordance rule uses (refine the text, never weaken the
assertion), plus an explicit answer for a route entry that maps to **no** capability.

### P7 — phase/plan · `input.research-thin` — 4 cases · weight 1 (extra_reads 7)
**Pattern:** research.md is formally fine but missing the *one fact the P4 decision needs* — callers,
an error surface, a table schema, a script name.

| chunk | what | impact | evidence |
|---|---|---|---|
| sut-capability-manifest | named the fn, did not enumerate its callers — the deciding fact | extra 2 | q-typed.json |
| sut-drift-check | listed only lib.rs; the error surface was never read | extra 1 | q-typed.json |
| interpretation-correctness-posture | on a branch-dependent chunk the decisive facts exist only after the branch is chosen | extra 2 | q-typed.json |
| sut-load-envelope | runs.db schema + the ui script name read during synthesis | extra 2 | fanout-results.md |
**Proposal:** see **X1** — this group is one end of a chain and is better read there.

### P8 — implement/fix-loop · `tooling.environmental` — 3 cases · weight 3
**Pattern:** all three are the **same external supply-chain decay**, and the mechanism handled it
correctly each time; the record is of an unclosed condition, not a defect.

| chunk | what | impact | evidence |
|---|---|---|---|
| sut-capability-manifest | 4 advisories published against a byte-identical tree | — | q-typed.json |
| out-of-scope-classification-treatment | advisory DB will not parse, fresh fetch and cached alike | retries 2 | q-typed.json |
| sut-load-envelope | fifth consecutive, byte-identical, on the latest published tool | defer 1 | fanout-results.md |
**Proposal:** none for the handling (the playbook's external-decay rule was written mid-epoch and worked).
See **L5** for the level reading of its non-closure.

### P9 — phase/validate · `contract.mechanical-check` — 3 cases · weight 1
**Pattern:** mechanical check 4's UI-gate WARN misfires on this project — twice as a false positive, once
correctly.

| chunk | what | impact | evidence |
|---|---|---|---|
| sut-capability-manifest | WARN on a chunk with **no UI** at all | — | q-typed.json |
| current-sut-coverage-classification | WARNs for omitting a harness that is display-gated and unrunnable here | — | q-typed.json |
| interpretation-correctness-posture | correctly caught a criterion whose evidence no listed invocation produces | extra 1 | q-typed.json |
**Proposal:** a direction — make the UI-gate heuristic condition on the *modify-set* containing a
frontend path (it already knows how to test this for the size rule) rather than on "a layout/a11y/design
extract is non-empty", which is true of nearly every chunk. The third case shows the check's other half
is earning its place.

### P10 — implement/code · `tooling.hook-friction` — 3 cases · weight 1
**Pattern:** identical every time — PostToolUse rust-analyzer diagnostics fire against a **mid-edit**
state during a multi-file signature change and report already-fixed lines.

| chunk | what | impact | evidence |
|---|---|---|---|
| sut-capability-manifest | six diagnostic batches, all artifacts of edit ORDER | extra 1 | q-typed.json |
| current-sut-coverage-classification | fired twice against mid-edit state (60-vs-82 array length) | extra 1 | q-typed.json |
| interpretation-correctness-posture | E0061/E0107 at line numbers already fixed by later edits | extra 1 | q-typed.json |
**Proposal:** a direction — either debounce the hook until an edit burst settles, or add a line to the
code-writing discipline stating that mid-sequence diagnostics on a signature change are expected and
should be resolved by a compile at the end of the burst, not per-write. Cost is small and constant
(1 extra read each), but it is 3/3 reproducible.

### P11 — implement/smoke · `tooling.headless-skip` — 3 cases · weight 1
**Pattern:** every webview-touching chunk in the epoch skipped its headful verification.

| chunk | what | impact | evidence |
|---|---|---|---|
| current-sut-coverage-classification | CoverageMatrix.tsx changed; no real-world assertion possible | defer 1 | q-typed.json |
| interpretation-correctness-posture | roll-up qualifier + App.tsx wiring got no headful assertion | defer 1 | q-typed.json |
| sut-load-envelope | gate listed, skipped: dist unchanged **and** display-gated | defer 1 | fanout-results.md |
**Proposal:** see **L4** — the level reading (a harness that never ran once) is the substantive finding.

### P12 — wrap-session/reconcile · `contract.cascade-miss` — 3 cases · weight 1
**Pattern:** three different escapes from the amendment flow, all found by hand, none by a detector.

| chunk | what | impact | evidence |
|---|---|---|---|
| current-sut-coverage-classification | cross-master citation grep found staleness no detector reported | extra 1 | q-typed.json |
| interpretation-correctness-posture | **second consecutive** chunk where detectors under-ran the plan's Expected-amendments list | — | q-typed.json |
| sut-load-envelope | a sidecar records an amendment whose **body edit never landed** | extra 2 | fanout-results.md |
**Proposal:** the third case is the sharpest: the sidecar is treated as evidence that a body edit
happened, and once it was wrong. A direction: at apply time, **verify the body actually changed** before
appending the sidecar entry (a trivial post-edit assertion), and consider a one-off audit of prior
sidecar claims against their bodies — this epoch found one; nothing establishes it is the only one.

---

## Cross-step chains (starting heuristics)

### X1 — `research` →research.md→ `code` / `plan` / `fix-loop` — 3 / 2 / 2 chunks (5 distinct chunks)
**Both ends.** Producer: `phase/research`, outcome **`ok`**, carrying signal **`unresolved-questions`**
— every time. Consumers: `implement/code` (capability-manifest, interpretation-correctness,
sut-load-envelope), `phase/plan` (capability-manifest, sut-drift-check), `implement/fix-loop`
(capability-manifest, sut-load-envelope) — each rating the artifact **`thin`**.
**Hypothesis.** `research.md` passes its own step formally while signalling open questions, and three
*different* downstream steps independently rate it thin. The typed groups see only the ends
(`input.research-thin` n=4 at plan, `input.research-files-wrong` n=2 at code); the chain shows one
producer feeding three consumers across five of eight chunks — a stronger signal than either end.
**Proposal direction.** `unresolved-questions` is currently a descriptive signal. A direction: treat it
as a **routing** signal — when research emits it, require the open questions to name *which downstream
step* they block (plan-decision vs implementation-scope), so P4 knows to resolve them before synthesis
and implement knows the file list is provisional. The 0–3 open-questions band is by design; what is
missing is who each one is for.

### X2 — `take-up` →scope.md→ `plan` — 2 chunks
**Both ends.** Producer `phase/take-up`, outcome `ok`, signals `carry-folded` + `scope-inferred`;
consumer `phase/plan` rating scope `thin` (current-sut-coverage-classification) and **`wrong`**
(in-lane-sut-scenarios).
**Hypothesis.** `scope-inferred` marks exactly the scope.md content that was not stated by the working
entry — and it is that content the plan step later finds thin or wrong. The signal already localizes
the risk; nothing consumes it.
**Proposal direction.** Have P4 read the producer's `scope-inferred` signal and re-verify the inferred
portions first. Low cost; the signal is already being emitted faithfully.

---

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — 9 facts — **the largest theme in the epoch**
**Facts (all `solution: prohibition`):** phase/research `do-not-touch coverage.rs` (capability-manifest) ·
phase/plan three do-not directives (capability-manifest) · phase/take-up "lands as a wrap amendment, not
an implement edit" (out-of-scope) · phase/plan one-spelling-across-surfaces guard (out-of-scope) ·
implement/code two `deliberately-NOT` source comments (out-of-scope) · phase/plan
`Do-not-author-a-P-033-scenario` (interpretation-correctness) · phase/plan
`Do-not-mirror-the-pin-in-TypeScript` (interpretation-correctness) · phase/plan stop-and-surface guard on
RunRecord (sut-load-envelope) · wrap/report two guards steering the doc-agents (sut-load-envelope).
Natures: 8 `process`, 1 `product-logic`.
**Level hypothesis.** The pipeline has **no channel that carries a "don't do X" constraint forward**, so
each step hand-authors one into whatever artifact it happens to own — plan prose, scope prose, source
comments, even the report's Changes bullet. The constraints are real and were honored; the *authoring*
is the absorption. Nine times in eight chunks, across four steps and three skills.
**Proposal direction.** A first-class **constraint carrier** — e.g. a `## Constraints` block in
`scope.md`/`plan.md` that /implement is contractually required to read and echo, the way CARRY/PREREQ
already work for *work* rather than *prohibitions*. CARRY proved the pattern works; prohibitions have no
equivalent.

### L2 — band-aid — 8 facts
**Facts (all `solution: workaround`, nature `process`/`environment`):** take-up active-version rule
resolves to the wrong version · orientation `&&`-chain aborts on an optional path · curation used Edit
over the prescribed `.tmp`-then-rename · reconcile consolidated 7 near-empty raw twins the rule demanded ·
orientation read the friction log the mechanics forbid · research declined an extract's constraint after
reading the source first-hand · report added an `Expected amendments` section outside the template ·
report used a non-enumerated Coverage value.
**Level hypothesis.** In each case the reference's **literal prescription** did not fit and the step
deviated to serve its evident intent. The deviations look individually sound; collectively they are the
references being over-specified relative to the situations they meet.
**Proposal direction.** Rather than eight edits, a direction: mark which reference rules are
**invariants** (never deviate) versus **defaults** (deviate with a recorded reason). Several of these —
the raw-twin rule, the atomic-write rule, the Coverage vocabulary — read as defaults being enforced as
invariants, which is what produces the workaround.

### L3 — band-aid — 4 facts — nature `environment`
**Facts:** research — Bash cwd persisted across calls, forcing an explicit `cd` back · fix-loop — piping
through `tail` truncated the nextest summary, evidence recovered by re-running standalone · smoke —
`\U` incomplete-escape inside a quoted heredoc, regex abandoned for substring matching · research — a
python heredoc ate backslashes, script written to the scratchpad and run by path.
Corroborated by untyped cluster **U4** (n=3, independent records).
**Level hypothesis.** The cause is the **Git-Bash-on-Windows invocation path**, not any project artifact;
the fixes all landed as per-invocation workarounds and none generalized.
**Proposal direction.** A short host-specific note in the code-writing / research discipline —
non-trivial Python goes to the scratchpad and runs by path, never inline through a quoted heredoc;
prefer `;` over `&&` for optional probes; never read a summary line through `tail`. Three of the four
were already solved this way ad hoc.

### L4 — chronic-degrade — the headful UI/a11y self-verify **never ran once in the epoch**
**Facts:** `tooling.headless-skip` ×3 (current-sut-coverage, interpretation-correctness, sut-load-envelope)
· `deferred/environment` at implement/smoke ×2 explicitly naming the display gate · `contract.mechanical-check`
P9 case 2, where validate WARNs for omitting a harness that cannot run here.
**Level hypothesis.** Textbook chronic-degrade: **8 chunks, 0 runs, 0 halts**. Every individual skip was
correctly recorded and correctly reasoned; the halt policy structurally never surfaces the aggregate.
Three webview-touching chunks shipped with zero headful assertion, and a fourth (sut-load-envelope)
deferred a user-visible webview feature partly *because* it could not be verified here.
**Proposal direction.** The cause is environmental (Linux+xvfb harness on a Windows dev host) and the
fixes so far are per-chunk recorded skips. A direction: decide the harness's real home — CI-side on
Linux, or a documented "this project cannot verify webview changes locally" posture that phase reads
when scoping UI work. What should not continue silently is UI shipping unverified while each chunk's
record looks clean.

### L5 — deferred-forever — deferrals accumulate monotonically; **no closure appears anywhere in the epoch**
**Facts:** gates `deferred` per chunk, in order: 0 · 0 · 0 · 0 · 1 · 1 · 1 · 2 — with `deferral-open`
signals on the last three. `cargo audit` red on 5 consecutive chunks (P8), re-pinned as a PREREQ each
time. One `unresolved/environment` fact records upgrading the tool to 0.22.2 purely to prove the
prescribed floor-raise remedy **unexecutable**.
**Level hypothesis.** The deferral machinery is working exactly as designed — bounded, re-pinned, with
the overlapping gate verified green each time — and *that is the finding*: nothing in the loop can close
a deferral whose cause is outside the project, so the count only rises. The mechanism has an open
condition it can carry indefinitely without ever escalating.
**Proposal direction.** A direction: give a deferral an **age or count trigger** — at N consecutive
re-pins, the wrap surfaces it as an escalation rather than a routine re-pin, so a bounded wait cannot
quietly become permanent. Note the epoch also produced a case where the closure deadline itself was
missed and nobody noticed (L6).

### L6 — deferred-forever — a deferral whose **promised landing chunk has passed**
**Facts:** one `deferred/process` fact at sut-load-envelope reconcile: dismissing D-obs-instrumentation
surfaced that `playbook.md`'s 2026-06-20/21 entries record the `db.insert_run` and `report.generate`
spans as landing "with the Epoch-8 cli/timeline caller" — **Epoch 8 (conductor-0.1.0) is complete and
`conductor-report` still contains zero `tracing::` calls.**
**Level hypothesis.** The deferral lived in a **playbook `note`** — a judgment aid consulted only when a
detector fires. A note explains a dismissal; it owns no work. The obligation was therefore invisible to
every route, matrix and handoff surface for an entire version, and resurfaced only by accident.
**Proposal direction.** A direction: when a playbook rule dismisses a detector **because the work is
route-sequenced elsewhere**, require the rule to name the owning route entry — and treat a dismissal
whose named owner is already `complete` as an escalation rather than a routine dismiss. (The project-side
half is already recorded in this chunk's handoff; the pipeline-level generalization is the proposal.)

### L7 — override — 3 facts, all nature `process`
**Facts:** phase/validate — the operator blocked apply at review over a factual misattribution in plan.md
(capability-manifest) · implement/code — the operator **widened research's Files-to-modify from 3 to 6**
after a plan-vs-reality conflict was surfaced (capability-manifest) · wrap/reconcile — the operator
directed sidecar instance-repair regardless of the proposal set (current-sut-coverage).
**Level hypothesis.** Not the same rule three times, so the override signature is weak — but two of the
three concern **research's boundary list**, and they align with X1 and P7. The operator widened it once
explicitly; implement then crossed it twice more on its own judgment (`input.research-files-wrong` n=2).
**Proposal direction.** Read together with X1: the recurring correction is that research's file list is
treated as a boundary while being produced as a best-effort inventory. Making its provisional status
explicit would remove both the overrides and the gray-area judgment calls.

### L8 — observation — remit-widening to remove a cause (3 `removed-cause` facts)
**Facts:** reconcile — de-hardcoded the number rather than updating it, extending beyond the four
previewed docs (capability-manifest) · validate — corrected the error in research.md too, a P3 artifact
outside P5's planned work (sut-drift-check) · smoke — created `coverage-matrix.md` at the repo root to
verify the surface, then deleted it (out-of-scope).
**Observation, not a proposal.** In each case the step went *outside its own remit* to remove a cause
rather than patch a symptom, and each looks locally right. Recorded because "the step widened its remit"
is the same observable act whether the judgment was good or bad, and three instances is where a pattern
would start.

---

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — new-session/orientation — 3 cases → proposed type `contract.reference-conflict`
**Cluster.** Three separate session starts recorded the same conflict: `evolve-system.md` states the
friction ledger is append-only and must **never** be read or have prior records loaded, while
new-session's evolve-nudge precondition ("friction-log.ndjson carries records for it") can only be
evaluated by reading it. Reported at three consecutive orientations, unresolved between them; one
orientation recorded parsing all 118 prior records to derive the nudge.
**Draft criteria line:** *"Two references the step must both obey give contradictory instructions on the
same artifact or action — record which references, and which one the step followed."*

### U2 — cross-skill — 5 cases → proposed type `contract.reference-misdescribes-mechanism`
**Cluster.** `fan-out.md` lists `{output_path}` among the variables to substitute while no prompt
placeholder exists (**twice**, two chunks apart) · the pending-exists Setup guard greps for the literal
word "pending", which the master-route template legend also contains · the report's `Commits` field is
sourced "since `last_wrap`" but wrap writes `last_wrap` in P6 and commits in P7, so the previous chunk's
commit always falls in the window · new-session's `chunk:null & status:planned` shorthand does not match
the matrix, which nests the field at `verification.status`.
**Draft criteria line:** *"The reference specifies a variable, guard, field path or derivation window
that does not match the artifact it operates on — record the reference site and the actual shape."*

### U3 — cross-skill — 7 cases → proposed type `contract.derived-from-nonauthoritative-view`
**Cluster.** A code-graph under-reporting claim later **publicly retracted** (2 records) · a crate-edge
direction split reconstructed from a tail-truncated console view and carried into the user review · a
hand-written regex undercounting Auto rows 42-vs-43 · a tests-extract literal-60 inventory that was
correct but incomplete (8 further sites found first-hand) · two plan.md citations pointing at documents
that do not hold the cited fact, caught only by the operator · a lock-delta script keyed by package name
where the lockfile legitimately holds one name at several versions, producing a self-contradictory
559-vs-514 result.
**Draft criteria line:** *"A count, inventory or citation was derived from a truncated, hand-rolled or
assumed-shape view rather than the authoritative source, and reached a decision, an artifact or the user
before being caught — record the derived value, the authoritative value, and what caught it."*
**Note.** The project already absorbed two curated rules for this class (the 2026-08-08 code-graph
reading entry and the 2026-08-09 "grep A before asserting A says X"). It still recurred five more times
after those landed — which is itself the argument for a typed record rather than another rule.

### U4 — cross-skill — 3 cases → proposed type `tooling.invocation-mechanics`
**Cluster.** An `&&`-chained probe batch aborting wholesale on an expected-absent optional path,
discarding the remaining checks · backslash sequences not surviving a quoted bash heredoc into Python on
Git-Bash/Windows, with the reported error position not matching the source · `cargo tree -i … -e
normal,build` printing "nothing to print" plus a hint that misdirected diagnosis toward a target-platform
cause when the real cause was the filter excluding a dev edge.
**Draft criteria line:** *"The invocation's own mechanics — chaining, quoting, or a filter flag —
silently produced a wrong, truncated or aborted result that misdirected the work; record the mechanism
and the corrected invocation."*
**Overlaps** L3, which sees the same cause through the deviation scan. Two independent detectors reaching
the same conclusion is corroboration, not duplication.

### U5 — cross-skill — 3 cases → proposed type `contract.input-misrouted`
**Cluster.** A verification-matrix capability bore directly on a P4 scope decision, but the matrix is in
no distiller's input set, is not in research's remit, and the skill schedules its read at P5 **after**
the decision it informs · an obs acceptance contribution required the posture be machine-parseable from
an obs artifact while the chosen mechanism is a Rust const plus a nextest gate · an acceptance criterion
referenced evidence (`logs/agent-latest.jsonl` field assertions) that no listed Test Command produces.
**Draft criteria line:** *"A fact or artifact the step's decision depends on is outside every agent's
input set, or is scheduled to be read after the decision it informs — record the fact, its owner, and
when the step actually needed it."*

---

## Below threshold — no action

**Typed groups (27).** implement/code `input.plan-step-ambiguous` (2) · implement/fix-loop
`ambiguity.scope-pressure` (1) · phase/validate `contract.intent-divergence` (1) · wrap/reconcile
`contract.false-positive-proposal` (2) · new-session `input.handoff-git-mismatch` (2) · phase/distill
`contract.binding-contradiction` (2) · wrap/reconcile `contract.proposal-format` (2) · wrap/curation
`ambiguity.tier-routing` (2) · phase/research `input.extract-signal-gap` (2) · implement/code
`input.research-files-wrong` (2 — but see X1/L7) · wrap/gates `tooling.gate-deferral` (2) ·
wrap/gates `tooling.commit-mechanics` (1) · phase/plan `retry.synthesis-rework` (1) · implement/fix-loop
`retry.fix-iterations` (1) · phase/research `input.cookbook-gap` (1) · implement/fix-loop
`contract.test-expectation` (1) · phase/take-up `ambiguity.version-derivation` (1) · phase/research
`ambiguity.scope-boundary` (1) · wrap/route-resolve `ambiguity.trajectory-halt` (1) · phase/take-up
`input.working-entry-thin` (1) · phase/take-up `contract.promotion-mechanics` (1) · new-session
`tooling.health-false-red` (1) · phase/distill `contract.extract-format` (1) · phase/plan
`input.extracts-conflict` (1) · implement/fix-loop `contract.spec-reality-gap` (1) · implement/fix-loop
`tooling.gate-deferral` (1) · wrap/route-resolve `contract.carry-no-owner` (1).

**Untyped singletons (8) — emerging, watch next epoch.**
- gates: the light gate passed green while `cargo audit`/`cargo deny` were RED on four live advisories — correct per the skill, recorded as a gate-scope observation.
- take-up: working entry **and** handoff both stated a specifically *wrong* remediation mechanism (not thin — incorrect), costing 10 extra reads.
- curation: Filter 4's signal set structurally caps a purely agent-discovered learning below the apply threshold (the single sharpest statement of P3).
- code: a post-edit residual sweep found two stale sites outside the plan's Files-to-modify; left unedited per the scope rule and carried.
- fix-loop **and** smoke (2 records, consecutive chunks): verifying the Markdown surface requires creating `coverage-matrix.md` at the repo root — untracked, non-ignored, and `git add -A` would commit it; created and deleted both times. **n=2, one short of threshold; the closest emerging cluster.**
- reconcile: the report named four Expected amendments and the fan-out surfaced only two — no detector invariant covers an *existing documented element gaining a treatment*.
- take-up: one P-ID carries two different names across the project's own artifacts (`single-sourced workspace key` vs `Constellation severity live-wiring`).

**Level facts below threshold.** `unresolved` (1) · `overridden` (3, but not on the same rule — see L7).
