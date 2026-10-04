# Fan-out results — 2026-10-04-real-model-test-surface-corrective

Seven Explore doc-agents, one parallel batch, prompts sent verbatim (amendment-flow §Fan-out). Detector scoping:
architecture 5 · security-plan 4 · design-system 3 · layout-templates 3 · test-plan 5 · obs-plan 5 · a11y-plan 3 = 28
(21 single-doc detectors + D-platform-claim × 7). Keyed-contract renders attached for architecture · test-plan ·
obs-plan · a11y-plan. Entity probe: the returns carried no `&lt;`/`&gt;`/`&amp;` (entities=0 on every return).

## Verdicts
- architecture — `proposals: []` (stripping removed the agent's per-detector commentary → raw twin `.raw-fanout-architecture.md`)
- security-plan — `proposals: []` (commentary stripped, incl. a note that the expected Secret-scan gate shape amendment sits outside its four detectors → raw twin)
- design-system — `proposals: []` (commentary stripped → raw twin)
- layout-templates — `proposals: []` (commentary stripped → raw twin)
- test-plan — 4 proposals (trailing commentary on D-tests-framework / D-tests-derived-count / D-platform-claim stripped: all no-drift)
- obs-plan — 2 proposals
- a11y-plan — `proposals: []` (commentary stripped → raw twin)

## test-plan
- T1 · D-tests-obs-harness · warning · §3 → 5-command implementation (keyed contract `5-command-implementation.md:11`) —
  the `run` body gains the two feature-gated clippy lines after the workspace clippy; lint only, never run; CI via the
  dogfood step; five verbs unchanged. basis `.andromeda/registries/contracts/test-plan/5-command-implementation.md:11`.
  **Disposition: APPLY** — playbook :308 (accurate this-chunk addition; report Harness / gate surface); check 5 entry 1.
- T2 · D-tests-obs-harness (dependent-of D-tests-obs-harness) · §9 Lint row (`test-plan.md:382`) — names the two
  feature clippy lines beside the workspace clippy. **Disposition: APPLY** — :308; check 5 entry 2.
- T3 · D-tests-obs-harness (dependent-of D-tests-obs-harness) · §9 Live-Pulse scenarios (`test-plan.md:391`) — the owed
  per-feature lint lines now run in `agent-run run`'s bundle; the gated targets are still never RUN in CI.
  **Disposition: APPLY** (text re-derived from the report, not pasted) — :308; check 5 entry 2.
- T4 · D-tests-coverage · §6 Repository-hygiene legs (`test-plan.md:291`) — the skip where no `.git` entry exists, the
  in-repo listing failure still red, both arms named. **Disposition: APPLY** — :308; check 5 entry 3.

## obs-plan
- O1 · D-obs-ci-gates · §10 Build / deploy failure conditions (`obs-plan.md:490`) — replace "clippy warnings treated as CI
  annotations (non-blocking at Minimal…)" with a blocking lint-gate line. basis `scripts/agent-run.sh:20`
  (`set -euo pipefail`), script lines, `obs-plan.md:490`.
  **Disposition: REJECT** — (a) the re-derivation tell: its basis cites harness source the report does not carry (the
  blocking semantics); (b) the detector's invariant holds — the clippy gate KIND is named at §1 :42, §9 :443 and §10
  :490, and the chunk added lines of that same kind; (c) the "non-blocking" wording, if stale, predates this chunk
  (the workspace `clippy … -D warnings` line has sat in the bundle since the harness chunk) — playbook :300's caution:
  routed to its owned channel, a CARRY on the second corrective entry minted at this wrap's route-resolve (measure
  whether a `-D warnings` red fails `agent-run run` and CI's dogfood step, then amend §10 from that chunk's report).
- O2 · D-obs-ci-gates · §9 Pipeline integration, Repository-hygiene gates row (`obs-plan.md:447`) — the secret-scan
  gate's one path-free skip line where no `.git` exists; an in-repo listing failure stays red. basis cites
  `secret_scan_gate.rs:291` / `:193`.
  **Disposition: REJECT as proposed (basis cites source) — the FACT raised by the orchestrator under check 5** (plan
  expected-amendments entry 3 names this row; the report's Symbols / APIs and Harness / gate surface bullets carry the
  skip line text and both arms) → **APPLY (orchestrator raise, routine)**, text re-derived from the report.

## Orchestrator raises (Validate check 5 — expected amendments no detector proposed)
- R1 · security-plan §Secret Management → Secret-scan gate shape (`security-plan.md:262`) — the skip where the workspace
  root has no `.git` entry, decided before any spawn; a listing failure inside a repository still fails; the CI presence
  guard is unchanged. `:225` (REALIZED record — still true inside a repository) and `:260` (Secret scanning in CI — CI
  always has a checkout) state nothing the skip falsifies: no change. **APPLY (routine; report-substantiated).**
- R2 · test-plan §4 Mutation instrument (`test-plan.md:149`) — an ADDITION: cargo-mutants' default copy mode (no `.git`
  in the copy) no longer needs `--copy-vcs true` for `conductor-core`; measured by the `.git`-less copy run, never a
  mutation run. No master states the old dependency (sweep `copy-vcs`: 0; semantic sweep: 2 hits, neither the
  dependency). **APPLY (routine; report-substantiated).**
- O2's fact (above).

## Validate checks
1. Playbook — T1–T4 :308 routine; O1 :300 reject (+ the re-derivation tell); O2 re-derivation reject → raised under
   check 5. No escalate verdict, no two-rule collision, no uneasy no-match.
2. Cross-contradiction — none: T2/T3 and T1 edit different sections in the same direction; O1 rejected.
3. Intent-consistency — the report matches the working-route entry (`:88`) and the plan's acceptance; the deviations
   are in-intent and justified; scope record: none (gate.py scope clean, 0 recorded).
4. Absence-needs-evidence — the R2 "no master states it" claim cites `copy-vcs` (0 hits) and the semantic sweep (2 hits,
   both dispositioned, read whole: architecture:60 and test-plan:291 are the subject's `git ls-files` wording).
5. Expected amendments — entry 1 (keyed contract): T1 · entry 2 (§9 Lint + Live-Pulse): T2 + T3 · entry 3 (security
   :262 · test-plan §6 :291 · obs §9 :447): R1 + T4 + O2-raise · entry 4 (§4 Mutation instrument): R2 · architecture:
   none expected, none proposed. Floor met.
6. Disproved claims — the report's bullet reads `none`.

Escalations: 0.

## Apply outcome
Applied 7: T1 (keyed contract `5-command-implementation.md:11`) · T2 (`test-plan.md:382`) · T3 (`test-plan.md:391`) ·
T4 (`test-plan.md:291`) · R2 (`test-plan.md:149`) · R1 (`security-plan.md:262`) · O2-raise (`obs-plan.md:447`).
Rejected 1: O1 (→ CARRY at route-resolve). Leaves re-derived 4 (cascade-dispositions.md). Sidecar entries: test-plan ·
security-plan · obs-plan, each read back as its file's last entry. `registry.py check` (test-plan): 0 defects.
`arch-registry-check.py measure`: within target (architecture unchanged). Escalations open: 0.
