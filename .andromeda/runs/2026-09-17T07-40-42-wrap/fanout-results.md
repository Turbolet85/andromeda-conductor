# Fan-out results — 2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm

**Run:** `.andromeda/runs/2026-09-17T07-40-42-wrap` · 7 doc-agents, one batch · **29 proposals**

| doc | verdict |
|---|---|
| design-system | `proposals: []` — no UI rendered (no Rust/TS source touched); its two `**Platform:**` lines are surface enumerations, excluded by D-platform-claim's own carve-out |
| layout-templates | `proposals: []` — `:190` names the expected-skip **SET**, not a literal, so the tally movement is not a hit |
| obs-plan | `proposals: []` — swept independently for a stating sentence; zero hits. CI/CD already classified Not-instrumentable at `:42` |
| **test-plan** | **5** (1 primary + 4 dependent), all D-platform-claim |
| **security-plan** | **7**, all `escalate` (6 D-security-deps + 1 D-security-subprocess) |
| **a11y-plan** | **5** (4 warning + 1 escalate), all D-platform-claim |
| **arch** | **12** (4 D-platform-claim + 6 D-arch-decisions + 2 D-arch-resources), most `escalate` |

## Proposal index (doc · site · detector · severity · subject)

### test-plan — D-platform-claim ×5
- `:470` **primary** · warning · §9 Matrix builds — retire the unconditional hosted-image endpoint verdict; state the measured image×runtime SET; close the elevation remedy line as MEASURED INSUFFICIENT; CI arrangement rows untouched (win22 was probe-scoped)
- `:56` dependent · §1 Surfaces under test — same claim restated
- `:123` dependent · §2 Agent-runnable invariants — same claim in a parenthetical
- `:307` dependent · §6 Drivers per surface table — fullest statement; also bound "elevation is the established cause" to its measured configuration
- `:373` dependent · §6 Both-surface parity scenario step — fourth verbatim restatement

### security-plan — all `escalate`
- `:185` · D-security-deps · §Dependency Security third class — retire the `≥152` floor as a POSTURE; subject is driver↔runtime major COHERENCE
- `:86` dependent · Threat Model → Networking — the below-152 fetch trigger
- `:87` dependent · Threat Model → CI/CD — the conditional's narrowing property
- `:185` · D-security-deps · float exit condition — "versioned Standalone Installer" names a mechanism that does not exist
- `:185` · D-security-deps · class scope — msedgedriver pin is a SECOND CI-time-fetched third-party binary, admitting control UNSTATED
- `:86` dependent · Threat Model → Networking — **the egress COUNT claim** "The one non-loopback egress…" is falsified
- `:363` · D-security-subprocess · §Anti-Patterns Code Patterns rule (b) — the SEVENTH governed form's mechanism changed AND it no longer describes the asserting step

### a11y-plan — D-platform-claim ×5
- `:457` **primary** · warning · §9 Platform — bind the RED verdict to its configuration
- `:115` dependent · §1 CI integration — carries BOTH retired claims ("RED at session creation" + "never been green") in one clause
- `:465` dependent · §9 E2E table row
- `:516` dependent · §11 Strategy carve-out — CI proof now PART-DELIVERED (`:397` passes, `:384` red on a counting basis)
- `:471` · **escalate** · §9 Pipeline integration — stop naming the launcher as the routine gate's launch path (Escalation 3)

### arch
- `:60` **primary** · D-platform-claim · warning · [CI/CD] — retire "endpoint remains CLOSED" / "Still never a green run" / "runnability measured-unproven"; state the measured SET
- `:246` dependent · Infrastructure Patterns CI/CD approach — "fails at WebView2 session creation" scoped to win25/152
- `:209` dependent · Build system — **the exit condition is still UNMET, but its stated BASIS ("DevToolsActivePort never appearing") is retired**; unmet now because the configuration is unratified and the arm is not green
- `:60` · D-platform-claim · warning · [CI/CD] — retire "must not run elevated" + the medium-integrity remainder; the launcher shipped, lowered the label as claimed, and was measured insufficient
- `:60` · **escalate** · D-arch-decisions — ci.yml no longer matches the locked arrangement (label · direct High call · floor bypass · driver pin), all four PROBE-SCOPED/UNRATIFIED
- `:246` dependent · escalate · CI/CD approach — launcher no longer in the asserting step
- `:209` dependent · escalate · Build system — driver now PINNED at job time, not resolved from the image; floor bypassed
- `:225` dependent · escalate · directory tree — `a11y-limited-token-launch.ps1` mechanism is the token API, not `runas`
- `:226` dependent · warning · directory tree — `a11y-token-witness.ps1` gained two modes, two transports, exits 96/97
- `:148` dependent · escalate · Occupied Resources Ports — Evergreen fetch condition qualified by the win22 bypass
- `:148` · **escalate** · D-arch-resources — register the SECOND egress `msedgedriver.microsoft.com`; retire "the project's only non-loopback outbound target"
- `:254` dependent · escalate · Trust boundary — a FOURTH CI-only outbound surface

## Validation performed so far (main)

- **Re-derivation tell** — test-plan's proposals cite windows-2025 at runtime `151.0.4129.101`, which the report does NOT carry. **CLEARED:** `grep -c '151\.0\.4129\.101' .andromeda/test-plan.md` → **2**; it is the doc's own prior baseline being preserved, not a re-derivation from source.
- **Count claims confirmed present** — `grep -cE 'only non-loopback outbound target|THIRD outbound surface' .andromeda/architecture.md` → **2**; `grep -c 'one non-loopback egress' .andromeda/security-plan.md` → **1**. Both match the proposals.
- **Coordinate drift** — the arch `:209` proposal's quoted phrase is not at that exact line; the claim exists but the line must be re-derived at apply time. Same check owed for every `basis` line before applying.
- **Distillations correctly excluded** — the security-plan agent explicitly declined to propose against `.claude/rules/security.md:27`, recognising it as cascade-derived. Correct.

## NOT YET DONE — resume here

1. Validate checks 2–6 (cross-contradiction · intent-consistency · absence-needs-evidence · expected-amendments reconciliation · disproved-claims disposition).
2. **The four escalations are UNRESOLVED and gate everything** (see report §Deviations 5 and the run's escalation list). No proposal may be applied until they are ruled.
3. Apply + cascade · P3 curation · P4 graph · P5 route-resolve (mint the successor) · P6 state+handoff · P7 gates/commit.
