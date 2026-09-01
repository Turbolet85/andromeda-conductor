# Fan-out results — 2026-09-01-desktop-a11y-sweep

7 Explore doc-agents, one per spec source, one parallel batch. `D-platform-claim` was sent to ALL SEVEN
(its `doc:` field is the cross-doc union, not `arch` alone).

**Note on twins:** every return parsed successfully after the mechanical entity-decode, so the raw returns
are consolidated verbatim here rather than split into six `.raw-fanout-{doc}.md` files. Two returns needed
handling before parsing: `test-plan` carried a preamble line ("I have everything I need."), and
`security-plan` + `a11y-plan` arrived HTML-escaped (`&gt;-` folded-scalar markers, `&lt;body&gt;`,
`=&gt;`). `entities=0` after decode.

## Verdicts

| doc | detectors | proposals | primaries | dependents |
|---|---|---|---|---|
| arch | D-arch-resources, D-arch-decisions, D-platform-claim | 7 | 3 | 4 |
| security-plan | D-security-input/-subprocess/-deps, D-platform-claim | 5 | 1 | 4 |
| design-system | D-design-tokens, D-design-derived-count, D-platform-claim | 4 | 1 | 3 |
| layout-templates | D-layout-surface, D-layout-derived-count, D-platform-claim | 2 | 1 | 1 |
| test-plan | D-tests-coverage/-framework/-obs-harness/-derived-count, D-platform-claim | 7 | 2 | 5 |
| obs-plan | D-obs-instrumentation/-stack/-redaction, D-platform-claim | **0** | — | — |
| a11y-plan | D-a11y-surface, D-a11y-obs-schema, D-platform-claim | 17 | 3 | 14 |
| **total** | | **42** | **11** | **31** |

## Clean returns (no drift), with the basis each cited

- **obs-plan — all four detectors clean.** No runtime operation added (zero `.rs` delta; every Coverage row
  reads instrumentation `n/a`), no dependency (`Cargo.lock`/`package-lock.json` byte-unchanged, so no OTel
  SDK enters), no redaction-shape change (the two diagnostic-output changes are wdio test-console output,
  not the tracing self-obs line / journal / runs.db / run-report artifacts §6 and §11 govern — and the DOM
  bound narrows rather than widens what is printed). `D-platform-claim`: grep for
  `webview2|webdriver|wdio|tauri-driver|axe|emulate|prefers-reduced-motion|a11y-harness` returns no verdict
  line; obs-plan's only platform statements are the multi-platform exporter set (:147) and GitHub Actions as
  a CI ARRANGEMENT (:42, :506), neither a capability verdict.
- **security-plan — D-security-input / -subprocess / -deps clean.** No new external-input symbol; the
  sidecar/preflight material CONFIRMS the doc (fixed program NAME from PATH matches §Anti-Patterns:310
  including its already-recorded "indistinguishable at row level" measurement; `.env(...)` data-dir matches
  §Input Validation:117); no dependency delta, and the audit record is exactly the bounded-wait remedy
  §Dependency Security:172-174 prescribes.
- **design-system — D-design-tokens, D-platform-claim clean.** Every changed UI element reads tokens
  `design-token✓` or `n/a`; no `hardcoded✗`, no colour-only state. Its only platform lines (§Surface
  desktop-webview / cli) are untouched.
- **layout-templates — D-layout-surface, D-platform-claim clean.** No new user-facing region: Start already
  appears in §Wireframe — Run console (idle) (:35) and the go/no-go dialog at §Wireframe — HOLD /
  §Component — Hero. Its platform sentences (:104, :321) are OS-convention design statements; its `--e2e`
  description (:183) matches the report.
- **test-plan — D-tests-framework, D-tests-derived-count clean.** No runner substitution (the
  `mochaOpts.timeout` raise is runner config); grep returns zero hits for all three moved hexes and the
  shared-hex qualifier — they land in design-system, layout-templates:130 and rules/frontend.md:25.
  test-plan's only literal "six" (:51) is the **cli stdout** label set, byte-unchanged this chunk.
- **a11y-plan — D-a11y-obs-schema clean.** No violation-schema or obs-side delta, so the envelope stays
  aligned with obs §6 as reproduced at :94-110 and :232-248.

## The proposal set, by theme

**Theme A — the `--e2e`-only attribution is stale now that a second arm shares the same `wdio.conf.ts`
driver stack** (arch 4, security-plan 5, test-plan 3, a11y-plan 3). Every proposal widens an enumeration
to name BOTH arms; none loosens a ban. arch: Ports / Trust boundary / Scope law / `CONDUCTOR_MSEDGEDRIVER`.
security-plan: attack-surface vector, the inbound-listener ban carve-out, the CLI-input entry point, the
`CONDUCTOR_MSEDGEDRIVER` Input-Validation row, the harness-spawn rule (b) scope label. test-plan: §6
drivers row (the FULL firing form), §9 live-leg invocation inventory (a fourth sanctioned path), §11 ban
parenthetical. a11y-plan: §9 pipeline, §3 Command, §1 CI-integration.

**Theme B — three token VALUES moved and falsified one derived qualifier** (design-system 4,
layout-templates 2). design-system: Tertiary row, Muted row (+ dropping the `(blocked slate-violet)`
identity), the dark `:root` block, the light block. layout-templates: the Mode-cell reason (the ruling
stands; only its hex-identity justification died) + its cascaded restatement in `rules/frontend.md:25`.

**Theme C — the reduced-motion drive path does not exist on any driver** (a11y-plan 5). §6 Motion
Verification (:419) primary; dependents at §12 open question (:592), §3 Configuration (:218), §11 CI ban
(:560), §10 perf budget (:492).

**Theme D — focus restoration is not the Radix default for a Trigger-less Channel-opened dialog**
(a11y-plan 8). §5 Focus restoration (:365) primary; dependents at §5 focus-trap bullet (:361), §3 focus
harness pattern (:253), §3 bootstrap focus-library phase (:295), §1 harness spec (:111), §1 operator-pause
critical path (:131), §12 decisions log (:590), §4 Button row (:331), plus one cascade site.

**Theme E — the self-obs log landing site moved** (arch 2). Under the tauri-driver `cwd: repoRoot` spawn
the app's cwd-relative `logs/conductor-tauri.jsonl` lands at the workspace root, covered by the
root-anchored ignore rule; `crates/conductor-tauri/ui/logs/` becomes the legacy site.
**Orchestrator-verified before staging:** root `logs/conductor-tauri.jsonl` = 27163 B at 00:15 (the
post-cwd driven run); `ui/logs/conductor-tauri.jsonl` = 3188 B at 23:21 (pre-cwd). Claim holds.

**Theme F — the driven arm is a real-clock live-Pulse leg** (test-plan 4, a11y-plan 1). §2 determinism
bullet + §11 real-network ban loopback parenthetical + §4 ESM-loadability clause + §3 CI stage selector;
a11y §10 perf budget. Records the arm as the existing operator-local-gate exception, never a CI leg.

## Orchestrator-raised (expected-amendments floor, amendment-flow §Validate check 5)

- **`layout-templates.md` operator-pause dialog fade, 200ms vs design-system's amended 150ms** — named in
  the chunk's `plan.md` `Expected amendments (wrap)` list, proposed by NO detector. The report does not
  substantiate it (this chunk changed neither fade value; the divergence dates to
  `2026-06-26-component-primitives-library`, whose cascade reached design-system:255 but not
  layout-templates). **ESCALATED** — see the escalation record.

## Cascade-forbidden proposal (routed, not applied)

- a11y-plan's `D-a11y-surface` dependent targeting **`.claude/rules/frontend.md` § Session Additions
  (:47)** — the 2026-06-26 entry stating "Radix supplies … focus-restore". `## Session Additions` is a
  preserve-verbatim curation home the cascade must NEVER edit (amendment-flow §Cascade step 2). Routed to
  **P3 curation as a correction** (curation-guide §Corrections — edit in place with a dated tag, exempt
  from the Filter-5 cap). The detector was right about the content and right to flag it; only the channel
  differs.
