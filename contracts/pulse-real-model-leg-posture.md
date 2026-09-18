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

Read the provenance line as a bound on trust. The transcribed clauses will need re-checking if the SUT moves,
and the one carried measurement is a point-in-time reading that the drive chunk confirms rather than inherits.
Nothing here asserts a property of Pulse that Conductor cannot observe from outside it.

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

- **Deterministic L4 absent or falsy** — the defining term, and a `shell-declaration`. Conductor observes
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

**The outcomes**, each drawn from the closed verdict triad and the closed five-valued report state, and each a
returned value rather than an error — an error is reserved for Conductor's own harness faults:

| Outcome | Verdict | Report state |
|---|---|---|
| The top-ranked hypothesis identifies the injected cause | `Pass` | `Pass` |
| It does not, and the report rendered hypotheses to judge | `CalibrationRegion` | `ManualCheck` |
| The report rendered no hypotheses (degraded, or the second generation never landed) | — | `Blocked`, with the named precondition |
| The read-back call itself failed | — | `Blocked`, with the named precondition |

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

**No reported state is an error exit.** Blocked, manual-check and known-residual are reported states; only a
hard failure is a non-zero process exit.

### Disposition instead of an SLO tier

A real-model leg **is not graded against an SLO tier**, and cannot be. The tier ladder is closed at three
values whose coarsest deadline is 90 000 ms (`crates/conductor-core/src/scenario.rs:52-72`), while real-model
incident formation alone was measured at roughly 110 s — above the ladder's ceiling before any read-back is
attempted. Latency is measured journal-relative across the scenario's whole emission window, so no tier
assignment could be met and every real-model leg would grade red on timing regardless of what the model said.

The leg therefore lands **declare-only** — no expected checks, a null verdict and a known-residual state — and
the interpretation claim is graded hard one tier out, in a harvest-style test over the leg's verbatim
captures. This is the shape the system already uses for a claim the run-report envelope cannot carry, and it
keeps the timing fact and the interpretation fact from contaminating each other.

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
