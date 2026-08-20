# Fan-out results — 2026-08-20-latency-regression-re-proof

7 doc-agents, one per spec source. Report: `conductor-0.2.0/chunks/2026-08-20-latency-regression-re-proof/report.md`.

| doc | verdict | proposals |
|---|---|---|
| arch | drift | 3 (D-arch-decisions ×3, one `dependent-of`) |
| security-plan | drift | 2 (D-security-subprocess ×2, one `dependent-of`) — **escalate** severity |
| design-system | clean | `proposals: []` |
| layout-templates | clean | `proposals: []` |
| test-plan | clean | `proposals: []` |
| obs-plan | clean | `proposals: []` |
| a11y-plan | clean | `proposals: []` |

## arch — 3 proposals (all validated ROUTINE, applied)

1. **D-arch-decisions · §Established Decisions [Read-Back Dependency Posture]** — record the shipped
   read-back routing carve-out: after a GREEN preflight, an empty active list routes a declare-only
   scenario to the auto-resolve residual (`KnownResidual` / verdict null) instead of `Blocked`;
   checks-bearing scenarios and every `CallFailed` keep `Blocked`. Must carry the honest limit (the
   arm did not fire on the live leg; unit-pinned only).
2. **D-arch-decisions · §Standard Contracts run-report envelope** (`dependent-of` #1) — the six-family
   declare-only sentence attributes the `KnownResidual` landing solely to "the degraded read-back";
   widen to name both mechanisms. **Matches the plan's Expected-amendments entry.**
3. **D-arch-decisions · §Occupied Resources — `contracts/pulse-load-envelope.toml`** — record that
   `max_sustained_rate_spans_per_s` is computed `occurrences / gap_ms` (dispatches/s), not wire spans/s;
   note the 50× divergence this chunk introduces, still ~200× under the bound, and that the fix is
   SURFACED and unowned. (Directive 3: the owner lands at route-resolve.)

Validation: playbook rule 28 (reconcile spec wording to the sound implementation, invariant preserved)
covers #3; #1 and #2 register a P5-ratified shipped decision in the registry that owns it. No
cross-contradiction; intent-consistent; no absence-claim without evidence.

## security-plan — 2 proposals, ESCALATED to the operator

Primary + its `dependent-of` twin: re-word the sidecar-spawn ban from "a fixed, hard-coded program
**path** only" (§Security Anti-Patterns → Input, line 307; restated in → Code Patterns) to a fixed
hard-coded program **name resolved through the inherited `PATH`**, and name `PATH` as a
spawn-resolution input.

Verified at source: `crates/conductor-verify/src/spawn.rs:15` is
`PULSE_MCP_PROGRAM: &str = "andromeda-pulse-mcp"` — a program NAME, whose own doc comment says
"resolved from `PATH`". The doc's wording is imprecise; the ban's INTENT (never an operator-chosen
command; a compile-time constant) holds and is preserved by the re-wording. Live evidence this chunk:
a `PATH` whose entry was split on a drive-letter colon resolved the sidecar nowhere and the leg
returned `[BLOCKED]` in ~0s.

**Pre-existing, not introduced by this chunk** — the spawn code is untouched here. Escalated because
the proposal goes beyond wording reconciliation: it adds a threat-surface statement (`PATH` is an
environment-controlled resolution channel), which is the operator's call in a security doc.

## Expected-amendments reconciliation (plan's list = this chunk's coverage floor)

| plan entry | disposition |
|---|---|
| arch §Standard Contracts run-report note | **proposed + applied** (arch #2) |
| test-plan §6 latency-regression scenario + §1 Critical Path — two-site re-base | **NO TARGET** — `grep -c latency-regression .andromeda/test-plan.md` = **0**; the statistical-anomaly E2E scenario is keyed on `error-baseline-spike`, which this chunk did not move. The plan predicted a target that does not exist. Dismissed with evidence, not silently. |
| security-plan §Input Validation read-back row — declare-only empty-corpus carve-out | **NO GENUINE TARGET** — security-plan's `blocked`-on-empty sentences (lines 118, 310, 360) all govern the **canary / preflight** round-trip, which this carve-out does not touch (it applies to the post-green-preflight SCENARIO read-back). The existing wording remains true. Folded into the operator escalation for a ruling rather than applied unilaterally. |
| obs-plan §4 statistical-anomaly row — measured cue offsets | **NO TARGET** — `grep -c "latency-regression\|statistical-anomaly" .andromeda/obs-plan.md` = **0**. Dismissed with evidence. |

## Disproved-claims disposition (report bullet → channel)

1. TOML header's "5-min window" → **self-corrected in the chunk** (the header now states the measured 60s-rotation mechanism).
2. `input.md:105`'s 90/90 recipe → **already recorded disproved at 2026-08-18** (v2-12 `notes`); this chunk supplies the measured replacement, carried by the v2-12 notes ADDENDUM. `input.md` is not wrap-amendable.
3. Envelope spans-vs-dispatches → **arch amendment #3** + an OWNER at route-resolve (directive 3).
4. `research.md`'s mid-leg "no rotation" reading → **corrected in `evidence/leg-verdict.md`** (chunk artifact).

## Escalation resolution (operator, 2026-08-20)

**Security spawn wording → APPLIED with the PATH statement.** Both linked proposals landed:
§Security Anti-Patterns → Input and → Code Patterns now read "a fixed, hard-coded program NAME
resolved through the inherited `PATH`", and → Input names `PATH` as a spawn-resolution input plus the
measured `[BLOCKED]`-in-~0s diagnostic. The ban's intent is preserved verbatim.

**security-plan carve-out line → NOT ADDED.** arch §Established Decisions [Read-Back Dependency
Posture] owns the read-back routing decision; security-plan's `blocked`-on-empty sentences govern the
canary/preflight and remain true, so no duplication was introduced.

## Cascade (step 2 cross-master citations + step 3 leaves)

Grepped all seven masters + the three preserve-verbatim curation homes for the retired spawn wording:

| site | class | action |
|---|---|---|
| `test-plan.md` §Test Anti-Patterns (stack-specific) | cross-master citation | re-based + sidecar |
| `obs-plan.md` §Obs Anti-Patterns | cross-master citation | re-based + sidecar |
| `.claude/rules/security.md` | leaf of security-plan | re-derived (Session Additions preserved) |
| `CLAUDE.md` §Critical Warnings | leaf of arch/security | re-derived |
| `security-plan.md` Decisions Log (2026-06-14 entry) | immutable history | LEFT — line 385 declares historical entries immutable |

Leaves checked and found CURRENT (no edit owed): `docs/conventions.md` + `docs/gotchas.md` +
`docs/tests-summary.md` + `rules/testing.md` + `rules/verification-harness.md` (their
`Blocked`/`KnownResidual` statements describe the unchanged five-state enum and the unchanged preflight
integrity rule, neither touched by a post-green-preflight carve-out); `docs/stack.md` (arch §Stack
untouched). `docs/session-learnings.md` hits are preserve-verbatim curation territory and are not stale.

**Drift = 0:** 7 amendments applied (arch ×3 · security-plan ×2 · test-plan ×1 · obs-plan ×1) + 2 leaf
re-derivations · 2 escalations resolved with the operator · 0 open.
