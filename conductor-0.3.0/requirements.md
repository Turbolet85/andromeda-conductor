# Conductor 0.3.0 — requirements

_Capabilities this version adds or retires. Ids continue the project scheme (`v2-NN` ran to 32 in 0.2.0);
the `refs/` capability spec keeps its own `P-NNN` space and is never minted into. Derived from
`conductor-0.3.0-incubator/intent.md`; each line carries that finding's EXPECT, and the matching OBSERVED
clause travels into `verification-matrix.json` as `observed_gap`._

## Theme 1 — The deferred a11y capability reaches an honest terminal (F1)

- **v3-01** · Hosted-runner cause measured, not listed — the reason the WebView2 remote-debugging endpoint never appears on the hosted image is established by a probing pass with named one-line probes, whose reading is recorded whatever it says (per intent §F1.1).
- **v3-02** · A11y CI gate at an honest terminal — the routine a11y arm either runs green in the `a11y` CI job, or its CI half is closed as a ratified permanent exclusion resting on the measured cause and carrying a named owner (per intent §F1 EXPECT; absorbs residual `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` and the prior matrix's one `deferred` entry `v2-24`).
- **v3-03** · Keyboard and focus-order coverage has a stated owner — the requirement and the suites agree about where SC 2.1.1 and SC 2.4.3 coverage lives: either the routine arm carries it, or the requirement names the driven suite as owner and states what CI gates instead (per intent §F1.2; a11y-plan §5 `run-console-idle`).

## Theme 2 — Committed scenarios assert only what can pass (F2)

- **v3-04** · Structurally-dead assertions retired as a class — every committed scenario's declared checks are satisfiable by construction against a live Pulse, or are retired to declare-only in ONE pass with their pinning tests updated in the same change (per intent §F2.1–2; precedent `architecture-amendments.md:403`; absorbs residual `2026-09-10-live-pulse-in-lane-scenario-round`).
- **v3-05** · Every scenario's tier is honest about its own duration — each committed scenario's declared SLO tier either fits its summed phase duration or carries a stated reason why it does not, with the three situations kept distinct rather than collapsed into one defect (per intent §F2.3).
- **v3-06** · The scenario-assertion audit is mechanical and re-runnable — a fresh reader can re-run one check that establishes both outcomes above rather than re-deriving them from prose (per intent §F2 EXPECT).

## Theme 3 — The fourth delegated budget gets an instrument (F3)

- **v3-07** · P-025 measurement contract stated for Pulse — Conductor states which Pulse-emitted observable, at what resolution, over what window, and what would constitute a hard grade, in a form Pulse can implement (per intent §F3 EXPECT, unblocked half).
- **v3-08** · P-025 graded hard at its real value — once Pulse emits the contracted observable, Conductor re-drives `halo-hue-encoding` and grades the hue-shift budget hard, restoring the fourth delegated budget narrowed out of `v2-20` (per intent §F3 EXPECT + §SEQUENCING — BLOCKED-ON a Pulse-side emission Conductor cannot satisfy alone).

## Theme 4 — One live leg proves interpretation, not plumbing (F4)

- **v3-09** · Real-model interpretation leg — a live leg runs with deterministic mode OFF, injects a known root cause, and asserts that the top hypothesis identifies it, with the non-determinism handled as a stated property of the leg (operator-gated, never a CI gate, its grading stated in advance) rather than by retrying until it passes (per intent §F4 EXPECT; absorbs residual `2026-08-09-interpretation-correctness-posture`; prerequisite `v2-09` verified in 0.2.0).
- **v3-10** · Diagnostic-quality cluster backed by an exercised path — P-031 / P-033 / P-034 / P-044 are backed by a path the harness actually drives instead of remaining pinned in the `UNBACKED_AUTO` drift set (per intent §F4 EXPECT).

## Theme 5 — The unrealized security bootstrap item (operator-absorbed at the 0.3.0 route review)

- **v3-11** · Secret-scanning CI gate — no secret-shaped string survives in the `conductor-*` workspace, the `*.p12` / `*.pem` / `*.cer` ignores are present, and the build goes red on any hit (per security-plan §Bootstrap phases + §Secret Management). Absorbed from the 0.2.0 carried-residual `secret-scanning-ci-gate` on the operator's ruling at this version's Phase 4 review, discharging that entry's own "Revisit at 0.3.0 scoping". Not intent-sourced: the authored intent neither names nor excludes it.

## Carried residuals (NOT 0.3.0 capabilities — deliberately unnumbered, no matrix entry)

- **`incident_events`-through-MCP — an EXTERNAL repo's capability gap.** Pulse's corpus persists incident lifecycle events in `incident_events`, the one corpus table whose content is not L4-authored and therefore the only remaining candidate for payload-level read-back fidelity under deterministic L4. Measured 2026-09-01 at Pulse HEAD `83d4060`: zero references in `crates/mcp-server`, so no MCP tool surfaces it at any width and **Conductor cannot reach it**. Re-carried, not dropped: the intent names it out of scope for 0.3.0 while explicitly keeping it a residual here, and its target is `pulse-0.4.0` — an external repo's entry, recorded so the next intake re-checks it rather than re-derives it. Conductor asserts runtime-STATE fidelity meanwhile (`mark_incident_resolved` + active-set membership, live-proven).

- **Unbuilt console surfaces — the footer `contentinfo` status strip and the report-site operator-checklist render.** Surfaced independently by the design and a11y validators during this route run from different plans: layout-templates §Component — Footer marks the status strip "DESIGNED, NOT SHIPPED … route-owned gap" (the only per-surface layout primitive with no shipped scaffold), §Component — Operator-checklist and design-system §Component Patterns record the report-site render as unbuilt, and a11y-plan §4's landmark table plus §11's ban on a landmark-less window reach the same two surfaces. Deliberately excluded at the Phase 4 review as outside the authored intent — **but coupled to `v3-02`, and that coupling is the reason this line exists**: if the routine arm starts gating in CI, the landmark ban could turn the unshipped footer into a RED inside the very gate `v3-02` is trying to make green. So `v3-02`'s planning must MEASURE whether the routine specs pass against the console as it stands today before promising a green terminal — a promise made without that measurement is the failure this entry predicts.

- **Control-panel-launched parity — Critical Path 7's owed half.** test-plan §1 Critical Path 7 / §6 Scenario 7 record the control-panel-LAUNCHED half as OWED and assign it to the tauri-driver leg, whose fate Epoch 3 decides. Surfaced by the tests validator during this route run and excluded at the Phase 4 review as a capability no intent asked for; recorded here because it is genuinely coupled to this version's a11y terminal and should be re-checked at the next intake rather than re-derived.
