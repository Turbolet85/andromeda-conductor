# Fan-out results — 2026-06-21-operator-pause-orchestration

7 doc-agents, one per spec source. 5 clean · 2 proposals (both warning).

## Clean (proposals: [])
- **security-plan** — D-security-input PASS (HoldPoint has garde `length`+`dive`); D-security-subprocess N/A (no sidecar touched); D-security-deps PASS (tokio dev-only, already vetted, `time` feature justified).
- **design-system** — D-design-tokens PASS (no UI element; all surfaces `tokens n/a`).
- **layout-templates** — D-layout-surface PASS (no new surface; the operator-pause UI is already wireframed; this is the backend primitive feeding it).
- **test-plan** — D-tests-coverage PASS (15 tests, deterministic via `start_paused`); D-tests-framework PASS (nextest + `#[tokio::test]` on-spec); D-tests-obs-harness PASS (no harness/envelope/log-format change).
- **a11y-plan** — D-a11y-surface PASS (no interactive UI element); D-a11y-obs-schema PASS (no schema change).

## Proposals
### P1 — arch / D-arch-resources (warning)
- **change:** register the new conductor-core public symbols (Decision, HoldPoint, HoldResolution, PauseResolver, HeadlessResolver, resolve_hold) in arch §Occupied Resources / §Standard Contracts.
- **validation verdict:** DISMISS (routine) — exact match to the established playbook rule (line 37-39): "D-arch-resources proposes registering a public library API symbol in ANY arch registry section, where the chunk's actual occupied resources are already registered → over-reach." This chunk added NO ports/sockets/crates/env vars (the `:4317` socket + workspace crates already registered); arch deliberately omits per-crate library API. 3rd recurrence; rule already covers §Occupied Resources AND §Standard Contracts. No amendment, no playbook change.

### P2 — obs / D-obs-instrumentation (warning)
- **change:** add a `pause.resolve` span for `resolve_hold` to obs-plan §4 (the agent flagged a new async library boundary with no span).
- **validation verdict:** DISMISS (recommended) + propose playbook generalization. The must-trace OPERATION is the live hold-await under a run (Epoch 8 timeline-wiring), not the resolver's definition; the headless resolve is immediate, emits no span, leaks nothing (D-obs-redaction PASS). The report + plan deliberately defer `hold.wait_resolve` to Epoch 8 (where it adds the obs-plan §11 bounded-set entry). Same deferred-span build-sequencing pattern as the fault-helper primitives (playbook line 40-42) — but that rule is scoped to `conductor-faults`; this is `conductor-core`. → escalate to user: confirm dismiss + broaden the rule to any-seam primitives.
