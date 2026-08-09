# Scope — 2026-08-09-interpretation-correctness-posture

**Working-route entry (Epoch 1 — Foundation: re-aim at the SUT):**
> Interpretation-correctness posture — one real-model leg with a known root cause and asserted top
> hypothesis, or a recorded deferral naming its owner

**Version:** conductor-0.2.0 · **Capability:** `v2-05` (method `manual`, status `planned`)

---

## The question this chunk answers

Pulse now ships a deterministic L4 mode (`ANDROMEDA_PULSE_L4_DETERMINISTIC=true`): a canned `L4Output`
replaces the Llama-3.2-3B inference so the incident pipeline becomes reproducible. Every live leg
conductor-0.2.0 plans (Epochs 2–4) depends on that mode. The consequence intent §4 **F3** names:

> with `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` the canned `L4Output` replaces the model, so every live leg
> 0.2.0 plans exercises **the plumbing and never the interpretation** — while Pulse's own positioning memo
> names interpretation-correctness ("inject a known root cause, verify the top hypothesis identifies it")
> *"the single most important test in all of Pulse"*.

So a fully green Conductor run under deterministic L4 proves that telemetry flows, cues fire, incidents
form, read-back works and SLOs hold — and proves **nothing** about whether Pulse's explanation is right.
This chunk makes that boundary a **recorded decision** rather than an accident.

## What it builds

The recorded posture, in exactly one of the two forms `v2-05`'s acceptance admits:

- **Branch A — a real-model correctness leg.** Deterministic mode OFF, a *known* root cause injected, the
  **top hypothesis asserted to identify it**, and the leg passes. This is the P-033 (Ranked Hypothesis
  Generation) path.
- **Branch B — a recorded deferral.** The gap is deferred **with a named owner**, together with the
  explicit statement that **"Conductor green" does not mean interpretation is trustworthy**.

Absence of both is a `v2-05` failure. Which branch is taken is an **operator decision at plan time**
(P4), made against the evidence the fan-out and codebase research surface — it is deliberately not
pre-decided here. Whichever branch is taken, the posture must land somewhere **load-bearing** rather than
as prose alone: a reader of a green Conductor artifact must be able to see what that green does not cover.

## Boundaries

- **Not the live-path enablement.** The faithful per-phase dispatcher (`v2-08`), real per-check read-back
  extraction (`v2-09`), the machine-checked Pulse run contract (`v2-18`) and live `ready:true` (`v2-10`)
  are Epoch-2 entries and stay there. This chunk does not build the live path it comments on.
- **Not the five families.** Epoch 3 owns them.
- **Never a CI gate.** Per vision §Method boundary, live legs are operator-gated (`workflow_dispatch` /
  local invocation) — CI has no Pulse. A real-model leg is additionally **non-deterministic by
  construction** and therefore cannot enter the zero-retry CI test set without breaking the standing
  zero-flakiness invariant.
- **Read-only on the specs.** Any spec reconciliation this chunk implies is wrap's amendment path, not a
  phase or implement edit.

## Surfaces and contracts in play

- `.claude/rules/verification-harness.md:47` records this as an **OPEN posture decision (pending,
  cross-codebase)** from the 0.1.0 `live-pulse-e2e-proof` chunk. Its "leading" option — a deterministic
  test-L4 mode in Pulse — has since **landed on the Pulse side**, so that entry is now partly stale: the
  determinism question is answered and the *interpretation* question is what remains open. This chunk is
  the one that closes it.
- `crates/conductor-core/src/coverage.rs:121` classifies **P-033 "Ranked Hypothesis Generation"
  (Diagnostic Quality)** as `CoverageMode::Auto` — i.e. Conductor claims it is auto-verifiable — while
  `scenarios/` contains **no P-033 scenario** and no code path asserts a hypothesis. The posture must be
  consistent with that classification (either the leg justifies `Auto`, or the deferral records why an
  `Auto` row currently has no verifying scenario).
  **Amended at P5 (validation-1, intent-incomplete):** P3 measured this and P-033 is **not a singular
  anomaly** — **eleven** `Auto`-classified capabilities have no scenario at all (`P-031`, `P-033`, `P-034`,
  `P-039`, `P-041`, `P-042`, `P-043`, `P-044`, `P-073`, `P-074`, `P-079`), and no gate in the repo can see
  it: `check_sut_drift` compares classification↔manifest, never classification↔scenarios. Four of the
  eleven (`P-031`/`P-033`/`P-034`/`P-044`) are precisely the diagnostic-quality cluster F3 says
  deterministic L4 bypasses. The recorded posture therefore covers the whole unbacked-`Auto` set rather
  than P-033 alone — a generalization this scope did not anticipate, justified by that measurement.
- The run-report surfaces (`conductor-report` Markdown · `conductor-cli` · the webview) are the candidate
  homes for a caveat that travels with a green result; the run-report envelope is the shared shape.
- `ANDROMEDA_PULSE_L4_DETERMINISTIC` appears **nowhere in Conductor's tree** today — Conductor cannot
  currently observe which mode the SUT under test was in. Whether this chunk changes that is a plan
  question; asserting the mode at preflight is explicitly Epoch 2's `v2-18`.

## PREREQ folded in (from 2026-08-09-out-of-scope-classification-treatment)

**Re-check `cargo audit` at this chunk's gates.** It exited 1 at the previous chunk on
`error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244` — the RustSec
advisory DB itself would not parse (reproduced with a fresh fetch AND with `-n` against the cache),
against a zero-dependency-delta chunk with both lockfiles un-drifted. It was taken as a **bounded
deferral** with `cargo deny check` (all four classes green) as the overlapping signal.

- If it is **green again** → note the heal and close the deferral.
- If it **still fails** → raise the **cargo-audit floor** to the fixed release. Explicitly **not** a
  `deny.toml` entry and **not** a CI edit (`.claude/rules/security.md` 2026-08-09 + `playbook.md`'s
  external-decay rule). CI's own `cargo audit` step hits the same wall until upstream heals.

## Done when

`v2-05`'s acceptance holds: either the real-model leg exists and passes, or the deferral is recorded with
its named owner and the "Conductor green ≠ interpretation trustworthy" statement — and the standing
`cargo audit` deferral is resolved one of the two ways above.
