# Fan-out results — 2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration

Seven Explore doc-agents, one per spec source, in one parallel batch. Each read the chunk report plus its own
document and the drift-base detectors scoped to it. No return needed stripping or an entity decode, and no
return carried a re-derivation tell (no proposal cited a source, manifest or lockfile location the report does
not carry), so no `.raw-fanout-{doc}.md` twin is warranted and this file is the audit artifact for all seven.

| doc | verdict | proposals |
|---|---|---|
| `obs-plan.md` | clean | `proposals: []` — no new must-trace operation, no telemetry dependency, no logging/artifact path; the doc states no platform verdict (0 occurrences of the swept tokens) |
| `design-system.md` | clean | `proposals: []` — no new UI element, no token/hex/ms delta; its literals are unrelated to the counts this chunk moved |
| `layout-templates.md` | clean | `proposals: []` — no new surface or region; its counts are Conductor run-domain numbers this chunk did not touch |
| `architecture.md` | 13 | D-arch-resources ×4 (Ports · Trust boundary · `RUNNER_TEMP` · `EDGEWEBDRIVER`), D-arch-decisions ×5, D-platform-claim ×4 |
| `security-plan.md` | 8 | D-security-deps ×4, D-security-subprocess ×2 (the sixth and seventh governed spawn forms), D-security-input ×1, D-platform-claim ×1 |
| `test-plan.md` | 10 | D-platform-claim ×5, D-tests-derived-count ×5 |
| `a11y-plan.md` | 8 | D-platform-claim ×8 (one primary, seven `dependent-of`) |

**Total: 39 proposals across 4 documents; 3 documents clean.**

## Validation (the six named checks)

1. **Playbook.** `:171` (measured-scalar with its basis) and `:149` (a master's own explicitly-provisional claim
   retired by the measurement its sentence names) govern the version and verdict retirements — routine. `:143`
   (an external handle a shipped artifact READS) governs the `EDGEWEBDRIVER` registration — routine; `:146`
   (SETS but never READS) is NO MATCH, its qualifier failing now that the job both writes and reads the handle,
   so it is not a collision participant. `:97` (a chunk reverses a locked arch Established Decision on live
   evidence, user-confirmed at P4) and `:124` (boundary widening — always a human's call, never routine
   however often it recurs) both resolve on the operator's ratification, given at the chunk's P4 and P5 and
   recorded in the sidecars.
2. **Cross-contradiction.** None. Several proposals edit one section (arch `[CI/CD]` takes six), all in the same
   direction — retire a stale claim to current truth.
3. **Intent-consistency.** Aligned. The one divergence — the chunk widening to `:397` and the shared walk
   helper beyond the entry's `:384` framing — is justified and was already absorbed into `scope.md` at phase P4
   on measurement, so val-1 verified a closure rather than discovering one.
4. **Absence needs evidence.** Every absence claim was re-derived here, not taken from a proposal. The
   security agent's "security-plan.md contains NO runner-label token" measured 0 for both `windows-2025` and
   `windows-2022` — confirmed. **Every hit on an over-cap line was resolved BY OFFSET, never from a grep
   view**: `splice.py summary` first (security-plan longest line 363 at 6664c with 4 lines over 2000;
   architecture longest 60 at 11135c with 13; test-plan longest 307 at 10078c with 14; a11y-plan longest 115
   at 3263c with 4), then `index --min-chars 2000`, then bounded windows around each match. `security-plan.md:363`
   alone carried both governed spawn forms at offsets 2748 and 3700 — a line-granular read would have shown one
   hit and hidden the other.
5. **Expected-amendments reconciliation.** All nine plan entries dispositioned: eight carried by proposals
   (architecture ×4, security-plan ×2, a11y-plan, test-plan), and `matrix#v3-02 notes` is the ledger-note form,
   owned by P7.3. No entry under-ran.
6. **Disproved-claims disposition.** All five report entries end DISPOSED: the `≥152` floor (security-plan +
   architecture), the float's non-existent exit mechanism (security-plan), the `windows-2025` verdicts
   (architecture, test-plan, a11y-plan), the seventh spawn form (ESCALATED, resolved), and `:397`'s falsified
   wrap-insensitivity (a11y-plan §11, plus the scope correction already made at P4).

## Escalations — 2, both resolved with the operator before any apply

Rule (b)'s class escalates rather than going routine, so neither was applied as drift.

- **The SEVENTH governed spawn form.** The launcher left the asserting step but still ships and still runs (4
  `ci.yml` call sites, down from 5). **Resolved: RETAIN, scope corrected to the dev-only driver-alone
  diagnostics** — a registry silent about a spawn a committed workflow still performs fails in the same way as
  one describing a spawn it no longer performs.
- **The SIXTH governed spawn form.** Raised by the orchestrator beyond what the detector proposed: the detector
  read the CI locus as now "fetching and verifying without executing any fetched program", which is true of the
  gate STEP and false of the JOB — the leg's driver stack executes the very binary the gate fetched.
  **Resolved: retire the step description, TRANSFER its network-arriving-PROGRAM property to the driver-stack
  spawn** — retiring it outright would have orphaned the Authenticode control's stated justification.

No playbook rule was proposed for either: `:124` forbids minting one for the boundary-widening class, precisely
so the recurrence keeps reaching the operator.

## Cascade

Step 1 applied 39 bodies across 4 masters. Step 2 swept all seven masters, the derived tier, the three
preserve-verbatim curation homes and the two judgment bases — patterns derived from every amendment of the pass
before the first grep. Step 3 re-derived the leaves: `CLAUDE.md` (warnings block), `.claude/rules/security.md`,
`.claude/rules/a11y.md`, `.claude/docs/security-summary.md`, `.claude/docs/a11y-summary.md`,
`.claude/docs/tests-summary.md`.

Routed OUT of the cascade rather than edited: `.claude/rules/testing.md`'s `## Session Additions` and
`.claude/docs/session-learnings.md` (preserve-verbatim → P3 curation); `.claude/rules/host-win32.md:134`'s
`runas /trustlevel` clause (KEPT — it states what the HOST IS, measured, not what the project does);
`playbook.md:168`/`:170` (a rule's own provenance example, correctly standing); and `master-route.md`, the seven
`*-amendments.md` sidecars and `residuals.md` (non-targets by contract).

**Drift = 0 on exit:** a final sweep for ten retired phrasings across the seven masters and every derived body
returns clean on all ten.
