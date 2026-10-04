# Fan-out results — 2026-10-03-p-075-re-round-on-incident-events

Seven Explore doc-agents, one parallel batch, each sent the amendment-flow prompt verbatim with its doc's scoped
detectors (D-platform-claim included in every batch). No master is registry-migrated (`registry.py contracts` exit 3
`NOT MIGRATED` on all four keyed masters), so every prompt dropped the contracts line. Stripping removed only the
agents' trailing `#` commentary (no-hit notes per detector, summarized per doc below); no return failed the parse,
and no HTML entity reached a YAML value, so no raw twin is warranted.

## Verdict lines
- **architecture** — 8 proposals (D-arch-resources ×6 · D-arch-decisions ×2). No hit: D-arch-collision (no port/socket/env/path/process changed) · D-platform-claim (`:139`, `:160`, `:263` name a host fact; `:213` is the CI arrangement) · D-arch-registry-size (orchestrator tooling, measured after Apply).
- **security-plan** — 2 proposals (D-security-input ×2, escalate severity).
- **design-system** — 0 proposals. D-platform-claim: `:195`, `:282` already name Windows + macOS + Linux; `:273` is a chrome note.
- **layout-templates** — 2 proposals (D-layout-derived-count ×2). Both literals sit in §Surface: cli → Output structure, not §Header / banner (the plan's wording); `:250` "tool count" already names the set.
- **test-plan** — 4 proposals (D-tests-derived-count ×3 · D-tests-coverage ×1). D-platform-claim: the Linux sites are xvfb / CI-arrangement / NVDA-bound and none is retired by this chunk.
- **obs-plan** — 2 proposals (D-obs-instrumentation ×2). The agent named §4's dated re-grade and `:35` as uncovered by any obs invariant.
- **a11y-plan** — 0 proposals. D-platform-claim: `:217`, `:294`, `:597` name a rejected CDP stack; the SR-leg sentences are NVDA-bound and not exercised here.

## Proposals and dispositions

| # | Doc · section · site | Detector | Change (one line) | Disposition · check |
|---|---|---|---|---|
| A1 | arch §Occupied Resources → Interface routes / surfaces · `:153` | D-arch-resources | MCP tools consumed lists five, `retrieve_incident_events` added | **escalate** — Boundary widening (check 1, playbook `:124`) |
| A2 | arch §Occupied Resources → Route prefixes · `:155` | D-arch-resources (dep. A1) | "the four consumed MCP tool names" → the pinned set | **escalate** with its group (A1) |
| A3 | arch [Read-Back Dependency Posture] · `:62` | D-arch-resources (dep. A1) | required-tool enumeration gains the fifth | **escalate** with its group (A1) |
| A4 | arch §Standard Contracts → Readiness gate sample · `:104` | D-arch-resources (dep. A1) | sample `required_tools` map gains the fifth | **escalate** with its group (A1) |
| A5 | arch §Standard Contracts → Readiness gate · `:93` | D-arch-resources (dep. A1) | live surface 8 → 9 tools at S2 (8 at `83d4060`) | **escalate** with its group (A1) |
| A6 | arch §Occupied Resources → Launched child process · `:160` | D-arch-resources (dep. A1) | "all four tools `absent`" → every pinned required tool, 2026-09-01 count kept dated | **escalate** with its group (A1) |
| A7 | arch [Read-Back Dependency Posture] · `:64` | D-arch-decisions | retire "`incident_events` reaches no MCP tool … not Conductor's to build"; `83d4060` kept dated; S2 reads it | **escalate** — Boundary widening (check 1) |
| A8 | arch §Standard Contracts → Readiness gate · `:93` | D-arch-decisions (dep. A7) | the same retirement's restatement | **escalate** with its group (A7) |
| S1 | security §Input Validation → pinned manifest row · `:112` | D-security-input | required-tool set names five; preconditions stay five | **escalate** — Boundary widening (check 1) |
| S2 | security §Input Validation → MCP child stdout row · `:125` | D-security-input | the boundary names the new read, its keys and its typed unknown-id arm | **escalate** — Boundary widening (check 1) |
| L1 | layout §Output structure — `conductor run` · `:198` | D-layout-derived-count | `tools 4/4` → the pinned set, never `5/5` | **apply** — routine, set-naming (playbook `:127`) |
| L2 | layout §Output structure — `conductor suite` · `:233` | D-layout-derived-count (dep. L1) | same | **apply** — routine (`:127`) |
| T1 | test §2 Deterministic bullet · `:124` | D-tests-derived-count | "four registered MCP tools" → the pinned set incl. the events read; the timed probe; the re-round's harvest ids | **apply** — routine (`:127` + `:308`) |
| T2 | test §1 ipc-internal · `:54` | D-tests-derived-count (dep. T1) | the four-name RAW-result list → the pinned set | **apply** — routine (`:127`) |
| T3 | test §6 desktop-webview row · `:307` | D-tests-derived-count (dep. T1) | "all four tools `absent`" → every pinned required tool, date kept | **apply** — routine (`:127`) |
| T4 | test §5 Cross-module patterns · after `:284` | D-tests-coverage | a `retrieve_incident_events` bullet in the three-tier shape | **apply** — routine (`:308`) |
| O1 | obs §6 Boundary-call wrappers · `:499` | D-obs-instrumentation | the must-log tool enumeration names five; the read rides `call_tool` | **apply** — routine (`:308`) |
| O2 | obs §1 Pulse MCP server row · `:40` | D-obs-instrumentation (dep. O1) | the consumed-tool list names five | **apply** — routine (`:308`) |
| R1 | obs §4 Delegated-timing family · `:350` @c≈4300 | orchestrator (check 5) | a dated S2 re-grade BESIDE the `03ec944` one | **apply** — routine (`:308`); the report substantiates it (Counts + Outcome) |

**Basis lines:** several proposals cite a source line (`contracts/mcp-contract.toml:16`, `manifest.rs:18`, `client.rs:195`) the report does not print. Each points at a FACT the report's Changes carries (five `required_tools`, the read riding `call_tool`), so this is not the re-derivation tell, and nothing is rejected on it. The applied text is re-derived from the report regardless.

## Validate checks
1. **Playbook** — as the table shows. The escalated group is one class: Boundary widening (a validated surface, the MCP child stdout, admits a new input class, and a locked decision's "not Conductor's to build" is reversed). Rule `:124` keeps it escalate however recurrent.
2. **Cross-contradiction** — none. A5 and A8 both edit `:93`, different clauses of the same line, consistent with each other.
3. **Intent-consistency** — the report matches the working-route entry and every plan criterion except the workspace-green clause, which is UNMET only by the `red — not this chunk's` gate 27 (routed to route-resolve). Scope record: one `companion` line, `preflight_spawn.rs`, serves `manifest.rs` (the child stub's `tools/list` is built from `READBACK_TOOLS`) and was overseer-accepted — holds.
4. **Absence needs evidence** — every hit carries a line; long lines are resolved by offset window at Apply (arch `:93` / `:64`, obs `:350`, test `:307` are over 2 000 chars).
5. **Expected amendments** — arch §Occupied Resources + both incident-events sites: A1–A8. security `:112` / `:125`: S1/S2; the corpus-access sentences `:46` / `:157` / `:336` read "read-back plus the `mark_incident_resolved` lifecycle write" — the new tool is a read, so they stay true: **no change**; §Universal preconditions stay five — no site states a moved count. test §2 / §1 / §5: T1–T4. obs §1: O2 (`:40`); `:35` states no count and frames the seam as read-back plus the write, still true: **no change**; §6: O1; §4: **R1, raised**. layout: L1/L2. Wrap check (Windows-only host): no stating sentence in any master.
6. **Disproved claims** — "`incident_events` reaches no MCP tool": A7/A8 (escalated). "8 tools": A5 (escalated). Platform: no sentence states the retired verdict in any of the seven (all seven detectors, cited above) — DISPOSED, no master change. The `conductor-tauri` Windows-origin fixture: no master states a host-independence claim for those tests (test-plan agent) — DISPOSED to the owner entry route-resolve proposes.

## Escalation resolution and apply
- **Boundary widening (A1–A8, S1, S2):** put to the operator at the wrap; answered "Ratified — apply": the founder ratified pinning `retrieve_incident_events` live on 2026-10-03 (the overseer put the question to him directly; his answer, "pin it"). All ten applied; each sidecar entry records the ratification on the founder's live word, relayed by the overseer.
- **Applied:** all 18 proposals + R1 (19 amendments across architecture · security-plan · layout-templates · test-plan · obs-plan); 0 rejected; 0 escalations open.
- **Registry size (D-arch-registry-size):** the first apply measured §Established Decisions 38830 B and §Occupied Resources 38392 B, both OVER 38115 B. Remedy per the detector: this wrap's own history (the `83d4060` provenance clause, the evidence pointer, the preconditions-stay-five aside) moved to the sidecar, and the amended PATH-miss sentence tightened. Re-measured 37991 B / 38083 B — within target.
- **Cascade:** `cascade-dispositions.md`; leaves re-derived: `.claude/docs/tests-summary.md`, `.claude/docs/security-summary.md`; one curation-home hit routed to P3 (`.claude/docs/session-learnings.md:320`).
