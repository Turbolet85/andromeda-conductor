# Real-model leg posture and grading rule

**A posture fixed before the first drive.** A real-model leg runs Conductor against a live Pulse with the
deterministic L4 mode OFF, so that what the leg exercises is Pulse's *interpretation* rather than its plumbing
carrying a canned answer. This document fixes, in advance of any such leg, the launch posture it runs under,
the emission profile it must produce, the rule by which its outcome is graded, and the quiet interval it
requires. It is stated in advance deliberately: a leg driven first and graded afterwards satisfies the letter
of the capability and defeats it, because the grading can then be chosen to fit what happened.

    sut_version  = "andromeda-pulse, pinned below"
    captured_at  = "2026-09-18"
    pinned_at    = "Pulse HEAD 83d4060"
    provenance   = "MIXED, per clause. Every Pulse coordinate below is a TRANSCRIBED SUT RECORD — a reading
                    of the SUT's own source at the pinned HEAD, tracked tree clean. The ~110 s formation
                    figure is a CONDUCTOR MEASUREMENT carried from a prior leg, dated in place and NOT
                    re-measured here. Every Conductor coordinate is a reading of this repo at the commit
                    that carries this file."
                   [corrected 2026-09-22 (2026-09-22-interpretation-proven-live, P3 research, before any
                    drive): the ~110 s figure is a DETERMINISTIC-L4 measurement, not a real-model one. Its
                    only witness, crates/conductor-run/tests/lifecycle_live.rs:20 (first committed at
                    f1584b1), ran under its own firing form's ANDROMEDA_PULSE_L4_DETERMINISTIC=true and is
                    recorded "deterministic L4" at
                    conductor-0.2.0/chunks/2026-08-31-p-075-assert-round/report.md:161; it was
                    re-attributed to the real model at
                    conductor-0.2.0/chunks/2026-09-06-operator-gated-live-suite/plan.md:102. No
                    real-model formation figure exists in Conductor's record.]
                   [measured 2026-09-23 (2026-09-22-interpretation-proven-live, the one graded drive, Pulse
                    HEAD 83d4060): NO pickup figure. The real model answered the preflight canary's one
                    cue-bearing digest (inference succeeded, its JSON parsed) and did not surface it, so no
                    incident formed, the preflight blocked after its 600 s poll, and the scenario never
                    emitted. Real-model pickup and formation both remain unmeasured.]

Read the provenance line as a bound on trust. The transcribed clauses will need re-checking if the SUT moves,
and the one carried measurement is a point-in-time reading that the drive chunk confirms rather than inherits.
Nothing here asserts a property of Pulse that Conductor cannot observe from outside it.
[corrected 2026-09-23 (2026-09-22-interpretation-proven-live, P5 review, before any drive): no drive can
confirm the carried figure, because it is a deterministic-L4 measurement (the provenance correction above).
The first drive measures PICKUP — the interval from Conductor's emission instant to Pulse's `opened_at`
stamp, which Pulse takes at digest pickup, before inference — and records it under that name. Real-model
formation stays unmeasured.]

## Regime

This is a **reader-less** member of `contracts/`, following the precedent set by the P-025 measurement
contract. No Rust code reads it: it has no `default_path()`, no resolution through the repo-relative resolver,
no bounds check at load, and deliberately no environment-variable override handle. It is read by people and by
the chunk that drives the leg. Nothing in this file is parsed at runtime, so none of the committed-manifest
input-boundary duties attach to it; were a future chunk to make it runtime-read, those duties would attach in
full and this line is the notice that they do.

## The launch posture

Each term is stated in the run contract's own vocabulary, so that what Conductor can and cannot observe is
explicit rather than implied.

- **Deterministic L4 absent or falsy** — the defining term, and a `shell-absence` (the run contract's
  `l4-real-model` term: unmet iff the handle IS declared, by either side's truthiness rule). Conductor observes
  whether *its own environment* declares `ANDROMEDA_PULSE_L4_DETERMINISTIC`; it never sets the handle, never
  launches Pulse, and cannot inspect a running `pulse-app`. A real-model leg therefore requires that this
  handle be absent or falsy in the shell that launches both processes, and the leg records that it observed
  its own environment rather than claiming a measurement of the SUT.
- **MCP read-back enabled** — `ANDROMEDA_PULSE_MCP_ENABLED` declared in Conductor's own environment, the
  second `shell-declaration` the run contract observes.
- **Shared data directory** — `ANDROMEDA_PULSE_DATA_DIR` set to the data directory of the live `pulse-app`
  the leg drives, never a directory of Conductor's own. Pointing Conductor elsewhere empties every read-back
  while resetting nothing on the SUT's side. The value is the operator's and is never recorded in this file
  or in any committed artifact.
- **Sidecar resolvable** — the MCP sidecar built and resolvable by name through the inherited search path,
  expressed in POSIX form when the launching shell is the POSIX one. An unresolvable sidecar produces a
  blocked row in roughly zero seconds that is indistinguishable in shape from a genuine SUT-side gate
  failure; elapsed time is the discriminator, and the check belongs before the leg rather than after it.
- **Emission witness** — the self-observation level paired with the global level, never a bare per-target
  directive, which replaces the default and silences every other target.
- **Bootstrap window** — `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` is resolved ONCE at the SUT's boot, so
  it is a **suite-wide posture and cannot be chosen per leg**. The real-model suite states one disposition and
  runs under it throughout. Which posture actually booted is confirmed by reading the SUT's own log for the
  bootstrap-window override target — never for the name of the function that emits it, which appears nowhere
  in the log. Conductor neither sets nor reads this handle; it rides the operator's launch.
- **Operator-invoked, never a CI gate** — the leg is reachable only from the operator-gated arm of the
  harness and from no default path, no bundled gate, and no CI job. This is a property of the invocation
  path, not of a prompt: the headless source-of-truth path is never blocked on an interactive prompt.
  What CI runs in place of this leg — the workspace unit and integration tiers, the supply-chain gates, the
  routine accessibility arm — gates the code the leg exercises but is **not a substitute for the leg's own
  assertion**, and nothing in CI observes Pulse's interpretation.
- **Sanctioned invocation** — the leg is driven through the existing operator-gated harness arm, which is
  already a member of the sanctioned live-leg set. This posture adds no new member to that set and no sixth
  harness command.

## The emission profile the observable requires

This clause is the reason the posture is worth writing down, and it is the one a leg designed the obvious way
gets wrong.

The hypotheses a real-model leg must assert on are reachable through exactly one surface: the diagnostic
report the MCP tool returns, whose rendered markdown carries a `## Hypotheses` section. That section renders
only when the incident's resolution-summary text parses as a model output; the tool computes its degraded flag
as precisely the negation of that parse succeeding (`crates/mcp-server/src/tools.rs:360-387`). Neither writer
of that field fires when an incident is first created:

- the live writer attaches the latest cleanly-parsed interpretation to an **already-active** incident, and
  only inside the dedupe re-generation branch — an existing active incident matching the same kind, scope and
  scope id whose re-emission was observed (`pulse-app/src/inference_runtime.rs:823`);
- the other writer attaches a resolution summary and **rejects any incident not already resolved**
  (`crates/triage/src/incident/registry.rs:128-143`).

**Therefore a single-storm leg has no hypothesis observable at all.** The first incident a storm forms carries
no resolution-summary text, the report comes back degraded, the hypotheses section is absent, and there is
nothing to grade. A leg that reads that result as a model failure has measured its own design.
[corrected 2026-09-22 (2026-09-22-interpretation-proven-live, P3 research, before any drive): FALSE at the
pinned HEAD. Pulse attaches the parsed interpretation AT CREATION — `create_incident_from_l4_output` writes
`resolution_summary_text: scrubbed_l4_json(parsed)` (`pulse-app/src/inference_runtime.rs:884`, commit `b2e4cb3`
of 2026-08-27, the same commit as the `:823` dedupe attach cited above) — so a single-storm incident renders its
hypotheses, and Conductor's own 2026-09-06 leg b1 read a single-storm canary incident non-degraded
(`ManualCheck`). The second cue-bearing digest below is therefore sufficient, not necessary. The `## Hypotheses`
section always renders (`crates/interpretation/src/markdown.rs:153`); what separates the cases is its body — the
degraded notice, the empty placeholder, or ranked entries — so "rendered hypotheses" means at least one ranked
entry. A report is degraded only when no L4 output attached or the attached JSON fails to parse, which includes
Pulse's whole-field scrub replacing the entire JSON when any part trips a scrubber pattern
(`inference_runtime.rs:665-671`).]

A real-model leg must therefore produce **at least a second cue-bearing digest on one identity while the
incident is still active**, and must confirm it reached the live-writer branch rather than forming a second
incident. Two SUT-side descriptions state the opposite of the behaviour above and must not be planned
against: the MCP tool's own description (`crates/mcp-server/src/jsonrpc.rs:189`) and the report renderer's
header contract (`crates/interpretation/src/markdown.rs:20-27`) both say that only resolved incidents render
hypotheses, which the live writer exists precisely to defeat.

## The grading rule

Fixed in advance, and stated so that it decides a verdict without reference to the run that produced it.

**What is asserted.** That the top-ranked hypothesis identifies the root cause introduced through the
harness's normal emission path. Rank is position: the first entry of the model's hypotheses array is the
top-ranked one. The assertion is a substring comparison against the composed observation text, which already
carries the report markdown (`crates/conductor-verify/src/extract.rs:102`) — so the rule needs no new
comparison kind, no new verdict word, no new envelope field, no new span name and no non-allowlisted span
attribute.
[corrected 2026-09-23 (2026-09-22-interpretation-proven-live, P5 review, before any drive): the substring
comparison is FALSE as a mechanism. No Conductor artifact persists the report text — the observation lives in
memory, and the read-back self-obs line logs key names only (`crates/conductor-verify/src/extract.rs:99`) — and
the composed text unions every active incident's report with the list fields (`extract.rs:90-103`), so it can
express neither rank nor attribution. The rule is applied to the rank-1 statement of the incident attributed to
the scenario's emission, re-read over MCP by a `live-pulse`-gated capture after the leg and graded in
`crates/conductor-run/tests/real_model_harvest.rs`. The sentence's consequence still holds: no new comparison
kind, verdict word, envelope field, span name or span attribute.]
[corrected 2026-09-23 (2026-09-22-interpretation-proven-live, P5 review, before any drive): a stated limit on
what "identifies" can mean. The digest's cue line reaches the model verbatim —
`[autonomous] retry_storm — retry_storm scope_id=conductor` (andromeda-pulse
`crates/triage/src/digest/assembler.rs:664-673`, one cue per digest at `:261-276`, inserted whole by
`crates/interpretation/src/prompt.rs:229-235`) — and that line itself satisfies the rule. So a pass means the
real, non-canned model carried the cue's scope and kind into rank 1, n=1: not inference of an unstated cause,
and not a choice among competing causes.]

**The outcomes**, each drawn from the closed verdict triad and the closed five-valued report state, and each a
returned value rather than an error — an error is reserved for Conductor's own harness faults:

| Outcome | Verdict | Report state |
|---|---|---|
| The top-ranked hypothesis identifies the injected cause | `Pass` | `Pass` |
| It does not, and the report rendered hypotheses to judge | `CalibrationRegion` | `ManualCheck` |
| The report rendered no hypotheses (degraded, or the second generation never landed) | — | `Blocked`, with the named precondition |
| The read-back call itself failed | — | `Blocked`, with the named precondition |

[corrected 2026-09-23 (2026-09-22-interpretation-proven-live, P5 review, before any drive): the third row's
"or the second generation never landed" does not arise. No second generation is needed, because the
interpretation attaches at creation (the emission-profile correction above).]

**Why a miss is the calibration region and not a failure.** Hypothesis quality is a model-interpretive claim,
and the standing policy is that such claims are reported for a human and never hard-failed on exact values.
The miss row above therefore takes the default mapping rather than overriding it, and this is stated
explicitly so that no future reader mistakes it for an oversight. The consequence is deliberate and worth
naming: a real-model leg **cannot go red on the model being wrong**. What it can go red on is the harness —
a leg that never reached its assertion is blocked, which is a different row, lexically and visually distinct
from a graded miss, and carries the precondition that was not met.

**A failing leg grades as a failure.** It is never re-driven until it passes. There is no retry-once policy,
no soft verdict, and no quarantine-and-rerun path. A leg whose verdict the operator disputes is re-examined,
not re-rolled.
[corrected 2026-09-29 (2026-09-29-diagnostic-quality-cluster-off-the-drift-pin, implement, before any drive):
"never re-driven until it passes" is replaced by the pre-stated drive series below, the overseer's ruling
(founder-delegated, 2026-09-29). What stands: there is no retry-once policy, no soft verdict and no
quarantine-and-rerun path, and a graded miss is never replaced by a further drive. What changed: the number of
drives is fixed in advance at up to five, each one recorded, and a canary-blocked drive is a measurement, never a
graded outcome.]

**No reported state is an error exit.** Blocked, manual-check and known-residual are reported states; only a
hard failure is a non-zero process exit.

### Disposition instead of an SLO tier

A real-model leg **is not graded against an SLO tier**, and cannot be. The tier ladder is closed at three
values whose coarsest deadline is 90 000 ms (`crates/conductor-core/src/scenario.rs:52-72`), while real-model
incident formation alone was measured at roughly 110 s — above the ladder's ceiling before any read-back is
attempted. Latency is measured journal-relative across the scenario's whole emission window, so no tier
assignment could be met and every real-model leg would grade red on timing regardless of what the model said.
[corrected 2026-09-22 (2026-09-22-interpretation-proven-live, P3 research, before any drive): the roughly 110 s
is a deterministic-L4 measurement (the provenance correction above). The disposition stands on the ground that
survives: real-model timing is unmeasured and non-deterministic, so no tier can be declared honestly before a
drive.]

The leg therefore lands **declare-only** — no expected checks, a null verdict and a known-residual state — and
the interpretation claim is graded hard one tier out, in a harvest-style test over the leg's verbatim
captures. This is the shape the system already uses for a claim the run-report envelope cannot carry, and it
keeps the timing fact and the interpretation fact from contaminating each other.
[corrected 2026-09-22 (2026-09-22-interpretation-proven-live, P3 research, before any drive): the success path
lands `ManualCheck`, not a known-residual state. A declare-only read-back that is non-degraded takes
`state_for(…, ManualCheck)` (`state_for` in `crates/conductor-run/src/execute.rs`), as the 2026-09-06 leg b1
measured; `KnownResidual` arises only from a degraded report or an emptied active set.]
[corrected 2026-09-23 (2026-09-22-interpretation-proven-live, P5 review, before any drive): the harvest grades a
capture that a `live-pulse`-gated tool (`crates/conductor-run/tests/real_model_live.rs`) re-reads over MCP after
the leg. It never grades the leg's own captures, which carry no report text.]

## The drive series

[added 2026-09-29 (2026-09-29-diagnostic-quality-cluster-off-the-drift-pin, implement, before any drive)]

**Provenance of these rules.** The fixed series, its pass condition and the exclusion of canary-blocked drives
from grading are the OVERSEER's rulings, made under founder delegation on 2026-09-29. The founder's own ruling is
only the principle they serve: nothing is skipped or deferred, the route runs in order, and a problem met is
solved now. Every Pulse coordinate in this section was read at andromeda-pulse committed HEAD `f15536b`
(`f15536b909af814e35eba43a50988cb863223d16`), never from its working tree, and RE-PINNED at `e98d838`
(`e98d8384512a300127305925c9e762f69e937ca5`, Pulse's P-025 wrap, the HEAD the drives run against): every file
cited here is byte-identical between the two commits (`git diff --stat f15536b e98d838` over them is empty), and
P-025's triage changes are additive (a whole-workspace incident listing and an optional tier-change instant for
the services view), touching neither incident creation, dedupe, the active list nor the MCP read-back. The rest of
this file keeps its own `83d4060` pin.

**Why a series and not one drive.** The only graded drive (2026-09-23) blocked at the preflight canary: the real
model answered the canary's one cue-bearing digest and formed no incident. At `f15536b` every exit that suppresses
an incident reads a model-authored field (`pulse-app/src/inference_runtime.rs:756-760`), sampling is stochastic
(no seed or temperature argument in `pulse-app/src/llamacli_inference.rs` `build_llama_cli_args`, `:405-435`), and no surface
records the model's decision. The candidate causes — model nondeterminism, a digest too weak to warrant an
incident, Pulse's incident prompt or thresholds — therefore separate only by outcome frequencies under controlled
variation. No single drive can tell them apart.

**(a) The pass condition.** `v3-09` is met only if at least one drive is GRADED and every graded drive reads
`Identified` under the unchanged rank-1 rule above. That is stricter than one drive, and there is no
retry-until-pass: a `NotIdentified` graded drive means not met, is recorded, and is never replaced by a further
drive.
- A drive is graded when its scenario emitted and was read back (route `ReadBack`).
- A drive whose scenario emitted but attributed no incident is a scenario-side dismissal: recorded, not graded.
- **A canary-blocked drive is a measurement, never graded.**
- A series that ends with zero graded drives leaves `v3-09` not met.

**(b) The series and its decision rule**, applied mechanically after the drives it names. At most five drives,
each the full leg through the existing operator-gated arm — there is no canary-only entry point, so a canary that
passes also yields a graded drive. `k_A` is the number of Stage A drives whose capture carries no
`emission: none` line, i.e. whose preflight reached ready; `k_B` is the same count over Stage B.
- **Stage A:** drives `a1`, `a2`, `a3`, identical, with the canary as shipped, on the operator's long-lived data
  dir.
- **`k_A = 3`:** the series ends. The 2026-09-23 block is recorded as one dismissal in a stochastic process and
  no canary change is made: the cause is the model's sampling, which the gate absorbs at the observed rate.
- **`1 ≤ k_A ≤ 2`:** the cause includes per-decision nondeterminism. The multi-storm canary below lands, then
  Stage B drives `b1`, `b2` on the SAME long-lived data dir, to confirm.
- **`k_A = 0`:** the multi-storm canary lands, then `b1`, `b2` on ONE FRESH data dir launched by the operator.
  That removes prior canary incidents' `CORPUS MATCHES` from the canary digest
  (`crates/triage/src/digest/assembler.rs:676-679`).
  - `k_B ≥ 1`: the cause is Conductor-side digest conditions, and the fix stands. Stated limit: the two levers
    are applied together, so Stage B does not attribute the cause to either one.
  - `k_B = 0`: **Pulse-side — the series stops.** A Pulse-side report goes to the overseer: the measured
    frequencies (0 of 3, then 0 of 6 decisions), the damper, latch and creation-predicate coordinates at
    the drives' Pulse HEAD, and the ask — a bounded decision/severity label on the silent exits, and a review of the prompt's
    surface/dismiss guidance. Nothing further is driven, and no Pulse file is touched.
- **The multi-storm canary**, landed only when the rule calls Stage B, and only under the real-model posture:
  three storms of the shipped occurrence count, each carrying its own marker, so its fingerprint differs and the
  damper's projection changes (`crates/triage/src/digest/damper.rs:251`); 90 s apart, clearing the 60 s cue
  latch (`crates/triage/src/cue/emitter.rs:28`); the freshness stamp taken before the first. The deterministic
  posture's canary — the `boot` gate's and every deterministic leg's — is unchanged.
- **A `pipeline-fault` canary** (no parse `ok` for its cue-bearing digest, or an inference error or skip) is an
  environment fault: it may be re-fired once and does not count toward the five. No other outcome is re-fired.

**(c) The quiet window between drives:** at least 150 s after the LAST incident any earlier drive formed, and
otherwise at least 90 s. A dismissed canary leaves no incident to dedupe against, and each drive's marker is
unique, so the 90 s floor covers Pulse's 60 s storm-retention window.

**(d) The launch posture per stage.** Stage A uses the operator's long-lived data dir, because P-044's witness
needs a prior same-scope incident. Stage B uses the dir the decision rule names. Every term of the launch posture
above binds every drive, and `pulse-app` is the operator's: never started, restarted or stopped by the agent.

**(e) The slot.** Before every drive the overseer confirms the host is free — Pulse runs its own live legs on the
same OTLP port — and the confirmation is recorded against the drive's label in the chunk's attempt ledger.

**(f) Three further grades**, fixed with the rank-1 rule in the harvest's rule section and applied to the same
capture. Each is a returned value from the closed triad and five-valued state. P-033's rule is unchanged. A
capture with no attributable incident, or whose read-back failed, is `Blocked` on all three with that reason.

| Capability | `Pass` / `Pass` | `CalibrationRegion` / `ManualCheck` | — / `Blocked` |
|---|---|---|---|
| P-031 Report Structure | the six sections (Symptom, Timeline, Hypotheses, Investigation Steps, Evidence, Project Context) in that order, Symptom and Timeline each carrying a narrative | the six in order, and Symptom or Timeline is its empty placeholder | the report is degraded |
| P-034 Suggested Investigation Steps | at least one numbered entry under Investigation Steps | anything else under a non-degraded section, the empty placeholder included | the report is degraded, or the section is absent |
| P-044 Retrieval-Augmented Interpretation | `## Previously Seen` lists at least one incident opened BEFORE the attributed one, AND the creating digest's corpus retrieval returned at least one row | — | otherwise: "no prior same-scope incident to retrieve" |

P-031 has a fourth row: a section missing or out of order under a non-degraded report is `Fail` / `Fail`. The
six sections are rendered by Pulse's serializer, never by the model (`crates/interpretation/src/markdown.rs`), so a
broken structure is a deterministic SUT fault and not a model-interpretive miss.

Stated limits, fixed with the grades:
- **P-031** — a pass says the model supplied Symptom and Timeline narratives into the rendered structure, never
  that either is correct.
- **P-034** — a pass says the model proposed at least one numbered step, never that a step is useful.
- **P-044** — proven by an inference chain, and only to the model's INPUT. Pulse logs the digest-side retrieval's
  candidate count BEFORE selection (`assembler.rs:309-318`) and the selected matches reach only the unlogged
  prompt payload; `## Previously Seen` is the report-side sibling selection (P-036, `retrieval.rs`), with the
  same workspace, the same 30-day window and a fingerprint-or-scope match. A pass therefore shows that corpus
  retrieval had a same-scope prior incident to put in front of the model, never that the model used it.
- **The canary classification** — the capture prints one `canary:` line per retry-storm cue-bearing digest before
  the scenario's emission instant (the whole window when it never emitted), following Pulse's serial inference:
  the digest's inference is the first prompt assembly after its tick (before the next retry-storm tick), and its
  outcome is what lies between that prompt and the next. `surfaced` when an incident outcome follows its parse
  `ok` (Pulse logs one on creation and on dedupe alike, so either is the model surfacing), `dismissed` when its
  parse `ok` has none, and `pipeline-fault` when there is no prompt, a skip before it, no parse `ok`, or an
  inference error. Pulse logs no line on a dismissal, so `dismissed` is read from an absence and cannot say which
  model field (a dismiss decision, severity none, the resolution-summary flag) caused it. Pulse's lines carry no
  digest identity, so the pairing rests on that order: a digest Pulse's queue replaces before inference pairs with
  the one that replaced it.
  [corrected 2026-09-29 (2026-09-29-diagnostic-quality-cluster-off-the-drift-pin, implement, after the series): the
  pairing first ended each digest's lines at the next cadence tick of ANY kind. Measured at b1 and b2, a tier-2
  `error_rate_spike` tick 0.95 s and 3 s after a canary tick closed its segment before the parse, so a surfacing
  (b1) and a dismissal (b2) printed as `pipeline-fault`. Those captures keep what they printed; the attempt ledger
  records each storm's outcome from Pulse's own log. The decision rule counts emissions, never these tokens.]

## The 2026-09-30 series

[added 2026-09-30 (2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir, implement, before any drive of
this series). The 2026-09-29 record above is unedited.]

**Provenance.** The design below — three identical drives on one fresh letters-only data dir — is the OVERSEER's
ruling, made under founder delegation on 2026-09-30. The founder's own ruling is the principle it serves: fix the
cause and run a NEW series; `v3-09` stays recorded NOT MET by the 2026-09-29 series and is never closed as deferred.
The cause fixed is the one the 2026-09-29 series measured: Pulse's `credit_card` scrubber arm matched the
digit-bearing data-dir name that is the workspace key, and the model read `[redacted: credit_card]`.

**Pulse coordinates**, re-pinned for this series to andromeda-pulse committed HEAD `fcc31b2`
(`fcc31b21666df70f3bbbaf124c4a6cd8597fa586`), read with `git show`, never from its working tree. That commit is the
scrubber fix: the `credit_card` arm (`crates/security/src/scrubber.rs`) now redacts a digit-group run only when some
window of whole groups with 13-19 digits passes Luhn. `pulse-app` AND the MCP sidecar are both built from that
HEAD — the sidecar renders the read-back report through the same scrubber (`crates/interpretation/src/markdown.rs`
`assemble_report`) — and each binary is proven by content to carry the new pattern and not the old before the
first drive. Pulse's workspace key is a PATH (its detected workspace root, else the data dir:
`crates/workspace-detector/src/contract.rs` `workspace_key`), so under this launch its last component is the dir's
leaf.

**The dir.** ONE fresh data dir, `%TEMP%/pulse-legs/rm-clean-series`, under the one-parent convention. Its leaf
carries no digit, so it matches neither numeric scrubber arm even without the fix, and no vendor prefix or `@`, so
it matches no other arm. All three drives use it.

**The drives.** Exactly three identical drives, `d1`, `d2`, `d3`, each the full leg through the existing
operator-gated arm (`run --live real-model`), with the canary and the capture as committed at this section's
landing.

**The pass condition** is §The drive series (a), unchanged: `v3-09` is met only if at least one drive is GRADED
(route `ReadBack` with an attributed incident) and every graded drive reads `Identified` under the unchanged rank-1
rule. A graded `NotIdentified` means not met, is recorded, and is never replaced.

**The canary.** A canary-blocked drive is a measurement, never graded.

**Re-fires.** Only a drive whose canary reads `pipeline-fault` from Pulse's own log (no parse `ok` for its
cue-bearing digest, or an inference error or skip) may be re-fired, once, uncounted. No other outcome re-fires.

**No fourth drive**, whatever `d1`-`d3` read.

**The quiet windows** are §The drive series (c): at least 150 s after the LAST incident any earlier drive formed,
and otherwise at least 90 s.

**The launch posture** is §The launch posture, every term, plus a `pulse-app` built from a HEAD carrying the fix.
One launch serves all three drives: `ANDROMEDA_PULSE_DATA_DIR` names the new dir, the model paths are set
(`ANDROMEDA_PULSE_MODEL_PATH`, `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` — absent, every inference errors
`model_not_configured`), deterministic L4 is absent, and no bootstrap-window override is set. On the overseer's
word (2026-09-30) the AGENT launches `pulse-app` for this series and stops it after the last drive; the booted
posture is confirmed from Pulse's own log before `d1`.

**The slot.** The overseer granted the slot for `d1`, `d2`, `d3` back to back, with the quiet windows and no
confirmation between drives (2026-09-30); the grant is recorded against each drive in the chunk's attempt ledger.

**The key rendering.** On a graded drive the capture's `pulse-report workspace rendering:` witness must read
`verbatim` or `absent`; any other reading is recorded beside that drive's grade as contamination.

## The 2026-10-01 series

[added 2026-10-01 (2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix, implement, before any drive
of this series). The 2026-09-29 and 2026-09-30 records above are unedited.]

**Provenance.** The founder's ruling of 2026-09-30, relayed by the overseer: the real model's dismissals are fixed in
Pulse 0.3.0, and a third pre-registered series follows. The 2026-09-30 series graded no drive — its `d1` was
canary-blocked, and in `d2` and `d3` the real model dismissed the scenario's own storm digest. The design below
repeats that series' design (the overseer's ruling, made under founder delegation) and was ratified by the overseer
at this chunk's plan review on 2026-10-01.

**Pulse coordinates**, re-pinned for this series to andromeda-pulse committed HEAD `a2addb3`
(`a2addb3755b3029cb79809b96efdd2522749b179`), read with `git show`, never from its working tree:
- **The surfacing fix.** `crates/interpretation/src/schema.json` orders `decision` and `severity` after `hypotheses`,
  so constrained generation emits the decision after the analysis; `PROMPT_VERSION_PRIMARY` is `v2.3`
  (`crates/interpretation/src/schema.rs`); and the digest's OVERALL line reads
  `OVERALL: {degraded|anomalous|nominal} ({n} active incident(s); {m} cue(s))`
  (`crates/triage/src/digest/assembler.rs`), so a cue-bearing tier-1 digest reads `anomalous` where it read `nominal`.
- **The no-incident outcome.** Every parsed generation logs exactly one of created, deduped or skipped. The skip is
  the target `interpretation.incident.skipped` (`pulse-app/src/inference_runtime.rs`, message
  `incident producer skipped`) with the fields `skip_reason` (`model_resolution_summary`, `decision_dismiss`,
  `severity_none` or `no_cue`, written by Pulse's code, never by the model), `decision`, `severity` and
  `digest_kind`, all four allowlisted (`pulse-app/src/observability.rs`). A `watch` decision creates an incident.
- **Unchanged:** the `L4Output` struct and its `Hypothesis` fields (`schema.rs`), and the `## Hypotheses` render
  (`crates/interpretation/src/markdown.rs`), so the rank-1 rule reads the same entry shape; `retrieve_report` still
  derives `degraded_mode` from parsing that struct.
- **The workspace key** and its publication under the data dir are unchanged (`crates/workspace-detector` is
  byte-identical from `fcc31b2`), and `app.boot.workspace_key` logs its `workspace_root_basename`.
- **The scrubber** keeps its eight arms with unchanged sources (`crates/security/src/scrubber.rs`) and now masks only
  a secret's matched span inside a larger value. A letters-only leaf is touched only after a keyed word (`password`,
  `passwd`, `secret`, `token` and the like) followed directly by whitespace, `=` or `:`.

**The dir.** ONE fresh data dir, `%TEMP%/pulse-legs/rm-surfacing-series`, under the one-parent convention. Its leaf
carries no digit, no `@`, no vendor prefix and no keyed word. All three drives use it. None reuses `rm-clean-series`,
whose corpus holds the 2026-09-30 incidents and would reach the model as `## Previously Seen` matches.

**The binaries.** `pulse-app` AND the MCP sidecar are built from a clean `a2addb3` tree — the build inputs (`crates`,
`pulse-app`, `Cargo.toml`, `Cargo.lock`) carry no uncommitted or untracked change — and each is proven before `d1`:
- `pulse-app` by content: `incident producer skipped` present, ` active-bypass incident(s); ` absent;
- the sidecar by the same pair where its exe carries the new string, and otherwise by build provenance: the clean
  inputs, the build command, its sha256 and an mtime after the commit, and a sha256 different from the `fcc31b2`
  build's.

Which arm held for the sidecar is recorded in the chunk's attempt ledger.

**The drives.** Exactly three identical drives, `d1`, `d2`, `d3`, each the full leg through the existing
operator-gated arm (`run --live real-model`), with the canary and the capture as committed at this section's
landing.

**The pass condition** is §The drive series (a), unchanged: `v3-09` is met only if at least one drive is GRADED
(route `ReadBack` with an attributed incident) and every graded drive reads `Identified` under the unchanged rank-1
rule. A graded `NotIdentified` means not met, is recorded, and is never replaced.

**The canary.** A canary-blocked drive is a measurement, never graded.

**Re-fires.** Only a drive whose canary reads `pipeline-fault` from Pulse's own log (no parse `ok` for its
cue-bearing digest, or an inference error or skip) may be re-fired, once, uncounted. No other outcome re-fires.

**No fourth drive**, whatever `d1`-`d3` read.

**The quiet windows** are §The drive series (c): at least 150 s after the LAST incident any earlier drive formed,
and otherwise at least 90 s.

**The launch posture** is §The launch posture, every term: `ANDROMEDA_PULSE_DATA_DIR` names the new dir, the model
paths are set (`ANDROMEDA_PULSE_MODEL_PATH`, `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`), deterministic L4 is absent, and
no bootstrap-window override is set. One launch serves the three drives. Before `d1` the booted posture is confirmed
from Pulse's own log: `inference_mode` `real`, `app.boot.workspace_key`'s `workspace_root_basename` equal to the
leaf, and 0 `triage.baseline.bootstrap_window.override` lines. Who launches `pulse-app` is the operator's word at the
slot.

**The slots.** The Pulse build and the model runs on `:4317` are the operator's slots. Each grant is recorded in the
chunk's attempt ledger against what it covered.

**Recorded, never graded.** Pulse's `skip_reason` for each digest it did not surface, and the `prompt_version` it
logged, witness the fix live and are never inputs to a grade. The capture prints Pulse's
`interpretation.incident.skipped` lines fields-only through its scrub chain, and each `canary:` line ends with its
inference's `skip_reason`; the rule reads only the first token after `canary: ` and is byte-identical to the one the
2026-09-30 series recorded.

**The key rendering.** §The 2026-09-30 series' clause, unchanged: on a graded drive the capture's
`pulse-report workspace rendering:` witness must read `verbatim` or `absent`; any other reading is recorded beside
that drive's grade as contamination.

## The 2026-10-06 series

[added 2026-10-06 (2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09, implement, before any drive of
this series). The 2026-09-29, 2026-09-30 and 2026-10-01 records above are unedited. The series keeps this name
whatever day its drives fire.]

**Provenance.** The founder's rulings this series serves: 2026-10-02, nothing planned for 0.3.0 is carried over, so
`v3-09` is not deferred; and 2026-10-04, a blocked route head is taken up and its block is the acceptance gate. The
design below repeats the 2026-10-01 series' design and was ratified by the overseer, under founder delegation, at
this chunk's plan review on 2026-10-06. It is the first series on the Linux dev host; the three before it ran on the
Windows host.

**Pulse coordinates**, re-pinned for this series to andromeda-pulse committed HEAD `5f77859`
(`5f77859f8ebbb18fe01f6394a6f94bfc66b9ef34`), read with `git show`, never from its working tree. The earlier
sections' coordinates are dated records and keep their own pins.
- **The trigger line.** The digest carries a `TRIGGER: ` line under `OVERALL`, rendered from the FIRST cue's cause
  label (`crates/triage/src/digest/assembler.rs` `render_payload`, `TRIGGER_LINE_PREFIX`;
  `crates/triage/src/contract.rs` `cue_cause_label`). For a retry-storm cue it reads `TRIGGER: Retry storm` — the
  cause label alone, with no scope.
- **The trigger-framing instruction**, appended to every tier's prompt (`crates/interpretation/src/prompt.rs`
  `TRIGGER_FRAMING_INSTRUCTION`): "When the digest carries a TRIGGER line, that line names the signal this output
  describes: the title, the symptom and the first hypothesis must be about that signal, and the first hypothesis
  statement must name that signal in the TRIGGER line's own words. Treat any other abnormal metric on the same
  service as a cause or an effect of that signal, never as a separate first hypothesis. CORPUS MATCHES lines are
  OTHER incidents, past or still open on another signal, given for context only; never describe one of them as the
  current signal."
- **The grounded title.** `grounded_output` (`pulse-app/src/inference_runtime.rs`) rewrites only the incident's
  `title`, to the cue's cause label, a colon and the model's own title. `hypotheses` are stored as the model wrote
  them, so the grounding cannot satisfy the rank-1 rule.
- **Unchanged from `a2addb3`:** the report render (`crates/interpretation/src/markdown.rs`) and `schema.json` are
  byte-identical; `L4Output` keeps its hypothesis fields (`crates/interpretation/src/schema.rs`, which moved only
  its three version constants); and the no-incident outcome still logs `interpretation.incident.skipped` with its
  closed `skip_reason`.
- **The workspace key** is a PATH — the detected workspace root, else the data dir
  (`crates/workspace-detector/src/contract.rs` `workspace_key`, byte-identical from `a2addb3`) — so under this launch
  its last component is the dir's leaf.
- **The argv** (`pulse-app/src/llamacli_inference.rs` `build_llama_cli_args`): `-c 8192`, `-rea off`,
  `--temp 1.0 --top-p 0.95 --top-k 64 --min-p 0`, and `--grammar-file` naming a per-call copy of the embedded
  grammar (`l4-output.gbnf`). The grammar admits a `hypotheses` array of zero to five entries.

**What is graded.** The model file `gemma-4-E4B-it-Q4_K_M.gguf`, sha256
`85a896a047553e842f25297ee5b031d64ff30147d9c4af17b1e4b394cd1fab87`, under the prompt versions `v2.5` (primary),
`v1.4-fallback` and `v1.4-reflection` (`crates/interpretation/src/schema.rs`). This is the first reading of that
model on retry naming: no Pulse artifact reads it on that question, and a pass is not assumed.

**The rule and the trigger line.** The rank-1 rule is byte-identical to the one every prior series recorded: the
rank-1 statement must name `conductor` as a whole word AND a retry token. Stated limit for this series: the retry
token reaches the model on the trigger line and on the cue line, under an instruction to put the trigger's words in
the first hypothesis; `conductor` reaches it on the cue line and on the `SERVICES` rows, under no instruction. So
`Identified` means the shipped model followed that instruction AND carried the cue's scope into rank 1. It is not
inference of an unstated cause. A rank 1 that names the retry storm without the service grades `NotIdentified`, as
it always would have.

**The dir.** ONE fresh data dir, leaf `rm-trigger-series`, under the one-parent convention's `pulse-legs` in the
user's cache dir. It is home-rooted on this host because the capture's scrub chain masks a home-rooted path and no
temp-rooted one. The leaf carries no digit, no `@`, no vendor prefix and no keyed word. All three drives use it, and
no prior series' dir is reused.

**The binaries.** `pulse-app` AND the MCP sidecar are built from a clean `5f77859` tree — the build inputs (`crates`,
`pulse-app`, `Cargo.toml`, `Cargo.lock`) carry no uncommitted or untracked change — and each is proven before `d1`:
- `pulse-app` by content: `hypotheses-item-statement-kv` present — a rule name of the embedded grammar the
  `--grammar-file` argument passes (`pulse-app/src/l4-output.gbnf`, in 0 files at `a2addb3`) — and
  `--json-schema-file` absent;
- the sidecar by the same pair where its binary carries the new string, and otherwise by build provenance: the clean
  inputs, the build command, an mtime after the commit, and a sha256 different from the pre-build binary's.

Which arm held for the sidecar is recorded in the chunk's attempt ledger.
[corrected 2026-10-06 (2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09, implement, before any drive of
this series, on the overseer's word): the present half first read `--grammar-file`. The rebuilt binary does not hold
that 14-byte argument as one run of bytes — the compiler materializes it from two immediates — so a correct build
reads 0 for it, as the pre-build binary does, and it discriminates nothing. The grammar rule name reads 0 on the
pre-build binary and 2 on the rebuilt one. The digest of the section as first written and the digest of this text
are both in the chunk's attempt ledger; the grading rule and the drive design are untouched.]

**The drives.** Exactly three identical drives, `d1`, `d2`, `d3`, each the full leg through the existing
operator-gated arm (`run --live real-model`), with the canary and the capture as committed at this section's
landing, fired in one sitting.

**The pass condition** is §The drive series (a), unchanged: `v3-09` is met only if at least one drive is GRADED
(route `ReadBack` with an attributed incident) and every graded drive reads `Identified` under the unchanged rank-1
rule. A graded `NotIdentified` means not met, is recorded, and is never replaced.

**The canary.** A canary-blocked drive is a measurement, never graded.

**Re-fires.** Only a drive whose canary reads `pipeline-fault` from Pulse's own log (no parse `ok` for its
cue-bearing digest, or an inference error or skip) may be re-fired, once, uncounted. No other outcome re-fires.

**No fourth drive**, whatever `d1`-`d3` read.

**The quiet windows** are §The drive series (c): at least 150 s after the LAST incident any earlier drive formed,
and otherwise at least 90 s.

**The launch posture** is §The launch posture, every term: `ANDROMEDA_PULSE_DATA_DIR` names the new dir, the model
paths are set (`ANDROMEDA_PULSE_MODEL_PATH`, `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`), deterministic L4 is absent, and
no bootstrap-window override is set. One launch serves the three drives, from a working directory outside this
repository. Before `d1` the booted posture is confirmed from Pulse's own log: `inference_mode` `real`, the
`model_identity` it logged, `app.boot.workspace_key`'s `workspace_root_basename` equal to the leaf, and 0
`triage.baseline.bootstrap_window.override` lines. On the overseer's word (2026-10-06) the AGENT launches `pulse-app`
for this series and stops it after the last drive.

**The slots.** The overseer granted the two Pulse builds, the launch and the model runs on `:4317` for this chunk
(2026-10-06); each grant is recorded in the chunk's attempt ledger against what it covered. A GPU drive never runs
at night.

**Recorded, never graded.** Pulse's `skip_reason` for each digest it did not surface, the `prompt_version` and the
`model_identity` it logged, and pickup. None is an input to a grade.

**The key rendering.** §The 2026-09-30 series' clause, unchanged: on a graded drive the capture's
`pulse-report workspace rendering:` witness must read `verbatim` or `absent`; any other reading is recorded beside
that drive's grade as contamination.

## The 2026-10-07 series

[added 2026-10-07 (2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09, implement, before any drive of
this series). The 2026-09-29, 2026-09-30, 2026-10-01 and 2026-10-06 records above are unedited. The series keeps
this name whatever day its drives fire.]

**Provenance.** The founder's ruling this series serves: 2026-10-06, `v3-09` is neither relaxed nor deferred —
Pulse is fixed, then this series runs. The design below repeats the 2026-10-06 series' design and was ratified by
the overseer, under founder delegation, at this chunk's plan review on 2026-10-07. It is the second series on the
Linux dev host.

**Pulse coordinates**, re-pinned for this series to andromeda-pulse committed HEAD `f70be92`
(`f70be92c2ca13c951330efeffd725e62a484610c`; its code commit is `1a2e509`, and no product file differs between the
two), read with `git show`, never from its working tree. The earlier sections' coordinates are dated records and
keep their own pins.
- **The trigger line**, unchanged. The digest carries a `TRIGGER: ` line under `OVERALL`, rendered from the FIRST
  cue's cause label (`crates/triage/src/digest/assembler.rs` `render_payload`, `TRIGGER_LINE_PREFIX`;
  `crates/triage/src/contract.rs` `cue_cause_label`). For a retry-storm cue it reads `TRIGGER: Retry storm` — the
  cause label alone, with no scope.
- **The trigger-framing instruction**, appended to every tier's prompt (`crates/interpretation/src/prompt.rs`
  `TRIGGER_FRAMING_INSTRUCTION`). It keeps the text the 2026-10-06 section quotes and ends with one new sentence:
  "When the digest carries a TRIGGER line, that line names the signal this output describes: the title, the symptom
  and the first hypothesis must be about that signal, and the first hypothesis statement must name that signal in
  the TRIGGER line's own words. Treat any other abnormal metric on the same service as a cause or an effect of that
  signal, never as a separate first hypothesis. CORPUS MATCHES lines are OTHER incidents, past or still open on
  another signal, given for context only; never describe one of them as the current signal. When the cue line under
  ATTENTION CUES carries a scope_id, the first hypothesis statement must name that scope_id value exactly as written
  there, and must not attribute the signal to anything else, including a service whose name merely contains it."
- **The cue line.** Under `ATTENTION CUES` a cue that has a scope renders as its kind label, a space, and
  `scope_id=` followed by the value (`crates/triage/src/digest/assembler.rs` `cue_summary`; the label from
  `crates/triage/src/cue/classify.rs` `cue_kind_label`). For the scenario's storm it reads
  `retry_storm scope_id=conductor`.
- **The grounded title.** `grounded_output` (`pulse-app/src/inference_runtime.rs`) rewrites only the incident's
  `title`, to the cue's cause label, a colon and the model's own title. `hypotheses` are stored as the model wrote
  them, so the grounding cannot satisfy the rank-1 rule.
- **Byte-unchanged from `5f77859`:** everything under `crates/triage`; the report render
  (`crates/interpretation/src/markdown.rs`); the no-incident outcome, which still logs
  `interpretation.incident.skipped` with its closed `skip_reason`; the workspace key
  (`crates/workspace-detector/src/contract.rs` `workspace_key`); the argv (`pulse-app/src/llamacli_inference.rs`
  `build_llama_cli_args`); and the embedded grammar (`pulse-app/src/l4-output.gbnf`). Between the two commits the
  product tree moves in five files: `prompt.rs` and `schema.rs` of `crates/interpretation`, one example, one unit
  test and the UI's package lock.

**What is graded.** The model file `gemma-4-E4B-it-Q4_K_M.gguf`, sha256
`85a896a047553e842f25297ee5b031d64ff30147d9c4af17b1e4b394cd1fab87` (unchanged from the 2026-10-06 series), under the
prompt versions `v2.6` (primary), `v1.5-fallback` and `v1.5-reflection` (`crates/interpretation/src/schema.rs`).
Pulse's own reading of the change graded 240 generations by this contract's rank-1 rule: its shipped arm met its
bar, and so did its baseline arm on the 2026-10-06 prompt. So that reading does not show the new sentence fixes the
2026-10-06 `d3` miss, and a pass is not assumed.

**The rule and the stated limit.** The rank-1 rule is byte-identical to the one every prior series recorded: the
rank-1 statement must name `conductor` as a whole word AND a retry token. Stated limit for this series: both facets
now reach the model under an instruction. The retry token is on the trigger line and on the cue line, with rank 1
told to use the trigger's words; `conductor` is on the cue line as its `scope_id` value, with rank 1 told to name
that value exactly as written and not a service whose name merely contains it. So `Identified` means the shipped
model followed both instructions over a cue line that carries both facets verbatim. It is not inference of an
unstated cause. A rank 1 that names the service only inside the canary identity `conductor-canary` grades
`NotIdentified`, as it did on 2026-10-06. A met series is three drives and is not read as proof beyond them.

**The dir.** ONE fresh data dir, leaf `rm-fifth-series`, under the one-parent convention's `pulse-legs` in the
user's cache dir, home-rooted on this host as the 2026-10-06 series' dir was. The app is launched from the dir, so
its leaf reaches the prompt: the leaf carries no digit, no `@`, no vendor prefix, no keyed word and neither word of
the grading rule. All three drives use it, and no prior series' dir is reused.

**The binaries.** `pulse-app` AND the MCP sidecar are built from a clean `f70be92` tree — the build inputs (`crates`,
`pulse-app`, `Cargo.toml`, `Cargo.lock`) carry no uncommitted or untracked change — and each is proven before `d1`.
The proof was measured on both builds, the `5f77859` binaries and then the rebuilt ones, before this section was
written:
- `pulse-app` by content, each string counted as one run of bytes in the binary. Present — two strings of the
  sentence the commit adds: `must name that scope_id value exactly as` reads 0 on the `5f77859` build and 1 on the
  rebuilt one, and `whose name merely contains it` reads 0 and 1. Absent — the two retired version literals:
  `v1.4-fallback` reads 1 on the `5f77859` build and 0 on the rebuilt one, and `v1.4-reflection` reads 1 and 0;
- the sidecar by build provenance: the clean inputs at `f70be92`, its build command at exit 0, and an mtime after
  `1a2e509`'s commit time. None of the moved text is in that binary — all four strings read 0 on both builds — so
  its sha256 is not required to move, and it did not: the rebuilt sidecar hashes the same as the `5f77859` one.

Both binaries' digests, before and after, are in the chunk's attempt ledger.

**The drives.** Exactly three identical drives, `d1`, `d2`, `d3`, each the full leg through the existing
operator-gated arm (`run --live real-model`), with the canary and the capture as committed at this section's
landing, fired in one sitting.

**The pass condition** is §The drive series (a), unchanged: `v3-09` is met only if at least one drive is GRADED
(route `ReadBack` with an attributed incident) and every graded drive reads `Identified` under the unchanged rank-1
rule. A graded `NotIdentified` means not met, is recorded, and is never replaced.

**The canary.** A canary-blocked drive is a measurement, never graded.

**Re-fires.** Only a drive whose canary reads `pipeline-fault` from Pulse's own log (no parse `ok` for its
cue-bearing digest, or an inference error or skip) may be re-fired, once, uncounted. No other outcome re-fires.

**No fourth drive**, whatever `d1`-`d3` read.

**The quiet windows** are §The drive series (c): at least 150 s after the LAST incident any earlier drive formed,
and otherwise at least 90 s.

**The launch posture** is §The launch posture, every term: `ANDROMEDA_PULSE_DATA_DIR` names the new dir, the model
paths are set (`ANDROMEDA_PULSE_MODEL_PATH`, `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`), deterministic L4 is absent, and
no bootstrap-window override is set. One launch of the `f70be92` build serves the three drives, from a working
directory outside this repository. Before `d1` the booted posture is confirmed from Pulse's own log:
`inference_mode` `real`, the `model_identity` it logged, `app.boot.workspace_key`'s `workspace_root_basename` equal
to the leaf, and 0 `triage.baseline.bootstrap_window.override` lines. On the overseer's word (2026-10-07) the AGENT
launches `pulse-app` for this series and stops it after the last drive.

**The slots.** The overseer granted the two Pulse builds, the launch and the model runs on `:4317` for this chunk
(2026-10-07), and the founder opened the GPU for 2026-10-07; each grant is recorded in the chunk's attempt ledger
against what it covered. A GPU drive never runs at night.

**Recorded, never graded.** Pulse's `skip_reason` for each digest it did not surface, the `prompt_version` and the
`model_identity` it logged, and pickup. None is an input to a grade.

**The key rendering.** §The 2026-09-30 series' clause, unchanged: on a graded drive the capture's
`pulse-report workspace rendering:` witness must read `verbatim` or `absent`; any other reading is recorded beside
that drive's grade as contamination.

## The 2026-10-07 capture run

[added 2026-10-07 (2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive, implement, before
any drive of this run). The 2026-09-29, 2026-09-30, 2026-10-01, 2026-10-06 and 2026-10-07 series records above are
unedited. The run keeps this name whatever day its drives fire.]

**Provenance.** The founder's ruling this run serves: 2026-10-07, Pulse's builder must be able to read the digest
itself — the prompt each generation received — and the corpus-key read stays refused. The founder picked the
vehicle the same day: the operator's wrapper around the model binary, not a Conductor feature. The run's shape
below is the overseer's, under founder delegation, fixed at this chunk's plan review on 2026-10-07.

**Not a verdict.** This run is a capture. Its drives are graded by the unchanged rank-1 rule, and each grade is an
observation recorded beside its capture. No reading of those grades makes `v3-09` met or not met: three
`Identified` do not meet it, and a `NotIdentified` does not count against it. §The drive series (a) is not applied
to this run, and the verdict stays a later pre-registered series. If no drive misses, that is a finding, and the
run is still delivered.

**The launch posture note.** For this run the launching shell's model-binary handle
(`ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`) names the operator's recording pass-through: an operator-owned file that
records the argv Pulse hands the model binary and then runs the real `llama-cli` with the same argv. The leg
neither set that handle nor read what the pass-through recorded. What it recorded sits in the operator's folder, is
read by Pulse's builder alone and enters no Conductor artifact. Every other term is §The launch posture:
`ANDROMEDA_PULSE_DATA_DIR` names the new dir, `ANDROMEDA_PULSE_MODEL_PATH` is the 2026-10-07 series' model,
deterministic L4 is absent, and no bootstrap-window override is set. One launch serves the three drives, from a
working directory outside this repository. Before `d1` the booted posture is confirmed from Pulse's own log:
`interpretation.model.load` `load_status` `loaded`, `inference_mode` `real`, the `model_identity` it logged,
`app.boot.workspace_key`'s `workspace_root_basename` equal to the leaf, and 0
`triage.baseline.bootstrap_window.override` lines.

**Pulse coordinates and what is graded** are §The 2026-10-07 series', unchanged: andromeda-pulse `f70be92`
(`f70be92c2ca13c951330efeffd725e62a484610c`); the model file `gemma-4-E4B-it-Q4_K_M.gguf`, sha256
`85a896a047553e842f25297ee5b031d64ff30147d9c4af17b1e4b394cd1fab87`; the prompt versions `v2.6` (primary),
`v1.5-fallback` and `v1.5-reflection`; and the rank-1 rule, byte-identical. Nothing is built for this run. The two
binaries are that series' `f70be92` builds, held by digest and re-read before `d1`:
- `pulse-app`, sha256 `df1676477222776d3e95edae7d219a4d421f2311ea8f17863233630c1ed8ba4a`;
- `andromeda-pulse-mcp`, sha256 `6175fc36be6577b195470f136a124fe8690e4029745f3651ab46ce19043ad2a9`.

**The dir.** ONE fresh data dir, leaf `rm-recorded-run`, under the one-parent convention's `pulse-legs` in the
user's cache dir, home-rooted on this host as the 2026-10-07 series' dir was. The app is launched from the dir, so
its leaf reaches the prompt: the leaf carries no digit, no `@`, no vendor prefix, no keyed word and neither word of
the grading rule. All three drives use it, and no earlier dir is reused.

**The drives.** Exactly three identical drives, `d1`, `d2`, `d3`, each the full leg through the existing
operator-gated arm (`run --live real-model`), with the canary and the capture as committed at this section's
landing, fired in one sitting.

**The canary.** A canary-blocked drive is a measurement, never graded.

**Re-fires.** Only a drive whose canary reads `pipeline-fault` from Pulse's own log may be re-fired, once,
uncounted. No other outcome re-fires. Under this launch an inference error, an inference skip or a failed spawn in
a drive's window stops the sitting and is reported to the overseer before any re-fire, because it may be the
pass-through's.

**No fourth drive**, whatever `d1`-`d3` read, and no drive is replaced.

**The quiet windows** are §The drive series (c): at least 150 s after the LAST incident any earlier drive formed,
and otherwise at least 90 s.

**Recorded, never graded.** Pulse's `skip_reason` for each digest it did not surface, the `prompt_version` and the
`model_identity` it logged, and pickup. None is an input to a grade.

**The key rendering.** §The 2026-09-30 series' clause, unchanged: on a graded drive the capture's
`pulse-report workspace rendering:` witness must read `verbatim` or `absent`; any other reading is recorded beside
that drive's grade as contamination.

**The tie.** Each drive's start and end instants are recorded in the attempt ledger, read from the first and the
last `timestamp_ms` of that drive's frozen self-obs stream; each committed capture prints the stamp of every prompt
assembly in its window. Time is the whole tie between a drive and what the pass-through recorded: the ledger gives
brackets and stamps, never a count of what the operator's folder holds.

**The slots.** The overseer's go precedes the launch. The agent launches `pulse-app` for this run and stops it
after the last drive. A GPU drive never runs at night. Each grant is recorded in the chunk's attempt ledger against
what it covered.

## The quiet window and serialization

Real-model legs are **serialized**, never merely ordered: each rides its own quiet window, because the SUT
dedupes a new incident against any open one and every run fires its own preflight canary.

The window's floor is the SUT's idle interval plus a full resolver tick, measured from the **last** incident
the leg's own preflight formed, which can be more than one. The per-leg budget derives from the committed run
contract's warm-up and canary-poll terms rather than from a literal restated here; a lowered environment value
clamps up to the contract floor and never down.

**The existing suite's arithmetic does not transfer.** Its window and per-leg budgets are calibrated to
canned-L4 incident formation of roughly 2 s. At roughly 110 s, a real-model leg's formation falls outside the
SUT's idle window entirely, which is why the deterministic handle is load-bearing for that suite's dedupe leg
and why a real-model leg cannot be obtained by flipping it. A real-model leg requires its own budgets,
computed from the same contract terms against the real formation figure — and that figure is confirmed at the
first drive rather than taken from this file.
[corrected 2026-09-22 (2026-09-22-interpretation-proven-live, P3 research, before any drive): the roughly 110 s
above is a deterministic-L4 measurement (the provenance correction above); no real-model formation figure exists.]
[corrected 2026-09-23 (2026-09-22-interpretation-proven-live, P5 review, before any drive): so no drive can
confirm it. The first drive measures PICKUP (inference excluded) and records it under that name; real-model
formation stays unmeasured.]

The window is a harness-level wait **between** legs. It is a firing-form precondition, not a synchronisation
device, and nothing inside a test sleeps to synchronise.

## The process census

A real-model leg boots an external process, so it ends with a real process census taken **twice** — once
before the leg as the baseline that makes the second reading mean anything, and once after — with the census
pattern derived from what the leg can start rather than from what it usually leaves behind. Each row names the
process, who started it, and its final state. The leg names its stop form beside its firing form, and stops
only what it started: `pulse-app` is the operator's and is always recorded as left running, with the operator
named as the one who stops it.

## What this posture does not do

- It does not drive a leg, and it grades nothing. The drive is a separate route entry, bound by this file.
- It does not re-scope, re-grade or re-posture the existing deterministic-L4 legs or the operator-only
  accessibility arm that runs beside them. Their posture and their results stand unchanged.
- It mints no harness command, no verdict word, no report-state, no bracket label and no colour.
- It states no property of `pulse-app` that Conductor cannot observe from its own side of the boundary.
