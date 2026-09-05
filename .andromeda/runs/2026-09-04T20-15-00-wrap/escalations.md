# Escalations — 2026-09-04-sidecar-spawn-without-a-console-window

Two escalations, both raised before any apply, both resolved WITH the operator.

## E1 — arch §Established Decisions [Module Boundaries]

**Proposal:** A7 (D-arch-decisions, warning).

**Why it escalated:** the proposal qualifies a LOCKED §Established Decision on the strength of a
defect this chunk did not cause and does not fix. No playbook rule matched; the nearest
(`:97`, reversing a locked decision because the live SUT contradicts it) is `escalate` and its
subject is the SUT, not the build graph. Orchestrator was uneasy — the "no rule, but this is
structural" branch.

**The measured fact:** arch states the workspace "gives independent `cargo build -p`/`test -p` per
seam". `cargo check -p conductor-verify --lib` is RED at HEAD — `tokio::time::sleep` at
`preflight.rs:336` with tokio's `time` feature present only in `[dev-dependencies]`
(`Cargo.toml:31`) and not `[dependencies]` (`:10`). Workspace feature unification hides it.
Pre-existing (`git show HEAD` confirms both the call site and the identical feature list).
`cargo test -p conductor-verify` DOES pass — precisely because dev-deps supply the feature — so
only the non-test half of the claim is falsified.

**Resolution (operator, 2026-09-04):** **qualify the arch body now.** The compiler-enforced
forbidden-edge property stands; the standalone per-seam BUILD claim gains the measured qualifier
with its evidence pointer. The route CARRY still owns the FIX (directive item 3 → *Dependency
polish*); this amendment only stops the body overstating what the workspace guarantees.

**No playbook rule minted** — a single instance, and the operator declined the rule-minting option.

## E2 — test-plan §9 Matrix builds + Pipeline structure (E2E row)

**Proposals:** T4 (primary) + T5 (dependent-of), both D-platform-claim, warning.

**Why it escalated:** the proposals would retire a stated 3-OS CI matrix and a separate
`ubuntu-latest` + `xvfb` webview job. Measured: `.github/workflows/ci.yml` has exactly two jobs,
both `runs-on: windows-latest`. But §9 reads as TARGET state, the block already distinguishes CI
ARRANGEMENT from CAPABILITY (and already carries a dated 2026-09-01 measurement about Windows),
and the markerless route entry *A11y CI gate* explicitly owns "the a11y specs as a CI gate on
whatever runner the project gets". Retiring the claim would DELETE a plan rather than correct a
falsehood — a materially different act from what the detector proposed.

**Resolution (operator, 2026-09-04):** **dismiss — unimplemented plan, not drift.** Neither T4 nor
T5 applied. The same shape playbook `:22` dismisses as build-sequencing: a plan that sequences a
concern as its own later phase is not stale for not having arrived. The measured two-job reality is
recorded in the chunk report's Outcome row and carried in the handoff, not written into §9.

**No playbook rule minted** — the existing `:22` reasoning covers the class by analogy; a rule
naming CI-matrix specifically would risk suppressing a genuine future CI drift.
