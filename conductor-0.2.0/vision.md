# Conductor 0.2.0 — Vision

_Derived from `conductor-0.2.0/intent.md` (§1–2, §5, §7). The harness is built; 0.2.0 re-aims it at
the Pulse that actually exists, proves it, and ships._

## The problem

0.1.0 delivered the harness — 61 chunks, all complete, 0 pending, CI-green (workspace nextest 420/420,
clippy, doctest). The scenario catalog, the seeded deterministic timeline, the emission journal, the CLI
verbs, the 5-command agent-run harness, the Tauri control panel and the CI gates all shipped.

What it never did is the thing the harness exists for: **prove a real Pulse.** The last chunk
(`2026-06-27-live-pulse-e2e-proof`) shipped Part A only — the canary fingerprint-fidelity bridge — and
deferred the proof because Pulse's incident creation was LLM-in-the-loop non-deterministic. Live
`conductor preflight` ran end-to-end correctly (connect · negotiate `2024-11-05` · 4 tools · emit · poll
all ✓) and still returned `Blocked: incident not found in corpus`.

**That blocker is gone.** Pulse shipped the leading option during its own v0.3.0: a deterministic
env-gated L4 mode (P-073, canned `L4Output`, no GPU/3B) plus Tier-1 storm coalescing so a sustained storm
yields exactly ONE incident (P-074). Both are `verified` in Pulse's matrix, so 0.1.0's route entry
_"Pulse LLM-in-the-loop verification posture — OPEN DECISION"_ is **resolved and retired** — it is not a
chunk and does not migrate.

**But Pulse moved while Conductor stood still.** Over a 41-day pause Pulse shipped 22 new capabilities
(P-061..P-082) and Conductor cannot so much as _name_ them — its P-ID type rejects anything above P-060.
A 0.2.0 that proved every 0.1.0 scenario and shipped a release would still be a harness aimed at a
version of Pulse that no longer exists. Synchronization with the SUT is therefore the first epoch, not a
housekeeping afterthought.

> **Core problem.** Conductor can drive and read back a real Pulse, but **has never actually proven
> one** — and it is now aimed at a SUT 22 capabilities out of date, its verification legs unbuilt, its
> four delegated timing budgets structurally unassertable, and its own delivery untracked by any ledger.

## Who and why

Solo developer, local dev host, Conductor running beside a real Pulse. Conductor exists to answer one
question — _does Pulse actually do what it claims?_ — programmatically via MCP read-back where one
exists, by operator checklist where the claim is visual. Until a live proof lands, that question is
unanswered and the harness itself is unfalsified.

## What is IN

- **Re-aim at the SUT (keystone).** The accepted capability set tracks Pulse instead of a compile-time
  constant, with a drift check that fails loudly when the SUT advances; every current Pulse capability is
  classified; the not-Conductor's boundary is a recorded decision; the SUT's known load envelope bounds
  scenario design.
- **Make the live path real.** A recorded, machine-checked Pulse run contract; detection of the
  workspace-key divergence as a named precondition; a faithful per-phase emission dispatcher; real
  per-check read-back extraction; and the first live `ready: true` with journal + `runs.db` evidence.
- **The proof.** The five families — `fingerprint-storm` (incl. P-074's exactly-one-incident claim),
  `error-baseline-spike`, `restart-suppression` (incl. one bypass case), `pii-scrub`,
  `connection-lifecycle` — plus one full `severity-lifecycle` pass, and the four delegated timing budgets
  asserted at their real values.
- **Verify, then ship.** Desktop a11y run and asserted with its violations gated in CI, cross-surface
  envelope parity, a zero-gap coverage gate over the _current_ SUT capability set, and a release build +
  Tauri 2 bundle with a final SLO pass.

## What is OUT

**Non-goals carried from 0.1.0, unchanged:** not a load-tester; no Pulse process management (launching
Pulse stays an operator-pause step); no UI automation of Pulse; no scenario DSL; no
multi-target / distributed / cloud; **"no scenario without a P-ID."**

**New non-goal — Conductor does not fix Pulse.** The workspace-key divergence is a Pulse defect and its
real remedy is a Pulse chunk; Conductor records the requirement, detects the misconfiguration and fails
loudly with a named precondition.

**Explicit SUT boundary.** Pulse's v0.3.0 UI capabilities (P-061..P-066, P-068..P-071, P-080..P-082) and
its own tooling (P-077 demo injector, P-078 self-verify harness) are **not** Conductor's to verify —
they belong to Pulse's webview suite and its P-076 tauri-driver e2e. Conductor's lane in that range is
**P-067, P-072, P-073, P-074, P-079**; P-075/P-076 are the delegation itself. Recording this as a
decision is what keeps the coverage gate from reading it as a gap.

**Method boundary.** The live legs are **operator-gated** (`workflow_dispatch` / local invocation),
**never a CI gate** — CI has no Pulse. Determinism goldens and stub-backed tests remain the CI-side
coverage.

## What "0.2.0 done" means

Conductor knows the Pulse that actually exists: every current Pulse capability is classified, the ones in
Conductor's lane are covered by scenarios, and the SUT-drift check fails loudly when Pulse advances
again. A real Pulse is launched under a machine-checked run contract; Conductor drives it and —
programmatically, via MCP read-back — proves the five families (including P-074's exactly-one-incident
claim) and one full severity-lifecycle pass, reaching live `ready: true` with journal and `runs.db`
evidence. The four delegated timing budgets (P-025 ≤2s, P-027 ≤5s, P-037 ≤2s, P-045 ≤1s) return real
Pass/Fail at their actual values, closing Pulse's P-075. The interpretation-correctness question has a
recorded answer. Desktop a11y runs and asserts on Linux+xvfb with its violations gated in CI; CLI and
Tauri produce an identical envelope for the same seed; the coverage matrix is gated zero-gap with all CI
gates green; and a release build plus Tauri bundle ships with a final SLO pass.

## Two id spaces (do not blur)

- **`v2-NN`** — Conductor's own capabilities. These are what `verification-matrix.json` tracks.
- **`P-NNN`** — Pulse's capabilities, the things Conductor proves. They appear only inside acceptance
  criteria ("…proving Pulse P-017/P-018"), in `coverage.rs` and in scenario configs — never as
  Conductor's own ids, and never as matrix entries.
