# Drift Base — Conductor

<!--
Drift detectors operated by /andromeda-wrap-session (its fan-out runs each detector against the chunk
report). One detector per entry: { id · doc · invariant · check · severity }. `doc` is one of the 7 spec
sources (arch + the 6 plans). severity: warning (routine — the playbook may auto-apply the amendment) |
escalate (halt + ask the user). Format owned by /andromeda-wrap-session (`references/amendment-flow.md`).

STARTER SET — covers roughly what v2 drift-detection checked, across all 7 greenfield artifacts:
v2 D3 (plan-to-code: crates / IPC / auth-library / test-framework / logging-library mismatches) +
v2 D4 (plan-to-plan: specialist-vs-arch decisions, tests↔obs harness bind, a11y↔obs schema bind).
NOT here (v3 makes them structural, so no drift can accumulate): v2 D1/D2 (living-artifact staleness —
living docs reconcile every wrap) · v2 D6 (chunk-progression — the cursor is marker-derived) · v2 D5
(distillation staleness — wrap's cascade re-derives CLAUDE.md/rules/docs on every amendment).
GROWS from dogfood: a resolved escalation that recurs becomes a new detector. §-refs point at the standard
greenfield plan sections.

Report contract: structural detectors read the report's `## Changes` lists (crates / IPC / deps / schema);
the presence detectors (input-validation · instrumentation · PII · tests · a11y · design-tokens) read the
report's `## Coverage of new surfaces` flags. A detector that needs a fact the report doesn't carry must wait
for the report to carry it (extend report-template) — never re-derive from git/code inside the fan-out.
-->

## Detectors

# — architecture —
- id: D-arch-resources
  doc: arch
  invariant: every new IPC method / endpoint / event / socket / port / env var / workspace crate the chunk lands is registered in arch §Occupied Resources / §Standard Contracts / §Inherited Defaults workspace crates.
  check: agent-read — for each new symbol / API / crate in the report's Changes, confirm it appears in the arch registry sections; an unregistered resource is drift.
  severity: warning

- id: D-arch-decisions
  doc: arch
  invariant: the chunk uses only the stack / runtime / patterns arch §Stack + §Established Decisions allow.
  check: agent-read — compare the report's Dependencies + symbols against §Stack / §Established Decisions; a new library, runtime, or a contradicted locked decision is drift.
  severity: warning

# — security —
- id: D-security-input
  doc: security-plan
  invariant: every new external-input surface (scenario config / CONDUCTOR_* path / MCP child stdout / deserialized struct) the chunk adds is validated per §Input Validation.
  check: agent-read — for each new external-input symbol in the report, confirm the validation §Input Validation mandates (garde derive / canonicalize / bounded decode) is present; an unvalidated boundary is drift.
  severity: escalate

- id: D-security-subprocess
  doc: security-plan
  invariant: the MCP sidecar spawn / data-dir handling matches §Security Anti-Patterns (fixed path, `.env(...)` only, no argv/shell interpolation, negotiate-down to 2024-11-05).
  check: agent-read — if the report touches the sidecar spawn / preflight / data-dir, confirm it matches §Security Anti-Patterns; a mismatch is drift.
  severity: escalate

- id: D-security-deps
  doc: security-plan
  invariant: every new dependency the chunk adds is allowed by §Dependency Security (audit-green; toolchain ≥1.94.1; tauri ≥2.10.3; Cargo.lock committed).
  check: agent-read — check the report's new Dependencies against §Dependency Security; a banned/unvetted dependency or a missed required bump is drift.
  severity: escalate

# — design-system —
- id: D-design-tokens
  doc: design-system
  invariant: new UI the chunk renders uses design tokens, not hardcoded values; status is never color-alone.
  check: agent-read — read the Coverage `tokens` flag of each new UI element in the report; `hardcoded✗` or color-only state is drift against §Color Palette / §Spacing / §Typography.
  severity: warning

# — layout-templates —
- id: D-layout-surface
  doc: layout-templates
  invariant: a new user-facing surface / region the chunk adds is described in §Primary Surfaces / the wireframes (desktop-webview or cli).
  check: agent-read — if the report adds a UI surface or region, confirm it maps to a §Wireframe entry; an undocumented surface is drift.
  severity: warning

- id: D-layout-derived-count
  doc: layout-templates
  invariant: a count / range / qualifier the doc states in a sample caption, wireframe or selector label matches the value the chunk's code now produces.
  check: agent-read — for each entry in the report's `Counts / qualifiers this chunk moved` bullet, grep the doc for the OLD value in a caption / sample / wireframe / token label; a hit is drift (the doc bakes a value the code no longer produces). Prose naming the SET rather than a literal is correct and NOT a hit — the fix is always to name the set or update the sample, never to substitute a fresh literal that re-stales.
  severity: warning

# — test-plan —
- id: D-tests-coverage
  doc: test-plan
  invariant: new code paths the chunk adds carry tests at the tier the test-plan requires (§2 Test Strategy); determinism is seeded (no real network/time).
  check: agent-read — compare the report's new symbols / modules against its Outcome (tests run); a new path with no test at the mandated tier, or non-deterministic test, is drift.
  severity: warning

- id: D-tests-framework
  doc: test-plan
  invariant: the test framework / runner the chunk uses matches the test-plan (§2 + §4 — cargo-nextest, rstest, proptest, insta, assert_cmd).
  check: agent-read — compare the report's test commands / runner against §2 / §4; an off-spec framework or runner is drift.
  severity: warning

- id: D-tests-obs-harness
  doc: test-plan
  invariant: the 5-command harness / envelope shape / JSONL log format stays consistent between test-plan §3 and obs-plan §3.
  check: agent-read — if the report changes the harness, status-read, or log format, confirm §3 ↔ obs-plan §3 still agree; a one-sided change is drift.
  severity: warning

- id: D-tests-derived-count
  doc: test-plan
  invariant: a count / range / qualifier the doc states in a selector label, fixture count or tier table matches the value the chunk's code now produces.
  check: agent-read — for each entry in the report's `Counts / qualifiers this chunk moved` bullet, grep the doc for the OLD value in a selector label / fixture count / tier table; a hit is drift (the doc bakes a value the code no longer produces). Prose naming the SET rather than a literal is correct and NOT a hit — the fix is always to name the set or update the sample, never to substitute a fresh literal that re-stales.
  severity: warning

# — obs-plan —
- id: D-obs-instrumentation
  doc: obs-plan
  invariant: new must-trace operations the chunk adds carry the spans / structured logs §4–§6 require; every log line carries run_id; journal stamps from std::time.
  check: agent-read — for each new operation symbol in the report, confirm instrumentation per §4 / §6; an uninstrumented critical path, missing run_id, or virtual-clock journal stamp is drift.
  severity: warning

- id: D-obs-stack
  doc: obs-plan
  invariant: the logging library the chunk uses matches the obs harness §3 — tracing JSON only, NO OTel SDK / OTLP exporter for self-observation.
  check: agent-read — compare the report's telemetry Dependencies / symbols against §3; an OTel SDK / network OTLP exporter for self-obs is drift.
  severity: escalate

- id: D-obs-redaction
  doc: obs-plan
  invariant: the chunk does not leak absolute host paths / internal struct names into logs / runs.db / report (§6 + §11 redaction).
  check: agent-read — if the report adds logging / artifact writes, confirm field-allowlist redaction per §6; a leaked path / struct name is drift.
  severity: escalate

# — a11y-plan —
- id: D-a11y-surface
  doc: a11y-plan
  invariant: a new interactive UI element the chunk adds carries the WCAG / focus / keyboard coverage §4–§6 require (four must-be-accessible paths).
  check: agent-read — if the report adds an interactive UI element, confirm a11y coverage per §4 / §5 / §6; an uncovered element is drift.
  severity: warning

- id: D-a11y-obs-schema
  doc: a11y-plan
  invariant: the a11y violation JSON schema stays consistent with obs §6 (the structured-log envelope a11y emits to).
  check: agent-read — if the report changes the a11y violation schema or the obs log schema, confirm the two still match; a divergence is drift.
  severity: warning

- id: D-design-derived-count
  doc: design-system
  invariant: a count / range / qualifier the doc states in a palette row, ANSI-map entry, token label or reuse tally matches the value the chunk's code now produces.
  check: agent-read — for each entry in the report's `Counts / qualifiers this chunk moved` bullet, grep the doc for the OLD value in a palette row / ANSI-map entry / token label / reuse tally; a hit is drift (the doc bakes a value the code no longer produces). Prose naming the SET rather than a literal is correct and NOT a hit — the fix is always to name the set or update the sample, never to substitute a fresh literal that re-stales.
  severity: warning

# — cross-doc (scoped to no single source: any of the seven may state a platform verdict) —
- id: D-platform-claim
  doc: arch | security-plan | design-system | layout-templates | test-plan | obs-plan | a11y-plan
  invariant: a PLATFORM / runner / driver VERDICT a doc STATES — a sentence asserting, in the doc's own voice, where the harness can or cannot run ("Linux only", "headless only", "requires X", "no driver on Y") — matches what the harness is demonstrably able to run on. A doc that merely NAMES a platform, shell, script or tool (a host mention, an entry-point pointer, a recipe's shell, an enumeration of surfaces) states no verdict and is never a hit; neither is a plan's TARGET-state CI arrangement (a matrix row or job the plan sequences for later) — an unimplemented plan, not a capability claim.
  check: agent-read — for each entry in the report's `Spec claims disproved by measurement` or `Harness / gate surface` bullets that RETIRES a platform, runner, or driver verdict, find in the doc a sentence that STATES the retired verdict — the token alone (`Linux`, `agent-run.sh`, `xvfb`) is never the trigger; QUOTE the stating sentence in the proposal, and where no sentence states it the detector has no hit. Distinguish the CI ARRANGEMENT (which runner the pipeline uses — a choice, not a capability) from the CAPABILITY claim (where the leg CAN run): only the capability claim is falsified by a measurement elsewhere. Retire the verdict by naming the measured platform SET, never by substituting a fresh single-platform literal.
  severity: warning
  # Added 2026-09-01 (webview-self-verify-windows-host, operator-approved): a11y-plan's own detectors
  # structurally could not see this class — it returned `proposals: []` while carrying six stale sites —
  # and test-plan's site list under-ran the doc (6 planned, 8 present). Caught only by the plan's
  # expected-amendments floor, which is exactly the recurrence condition for minting a detector.
  # Tightened 2026-09-05 (0-pending adaptation, operator-directed): the original check keyed on report TOKENS and
  # mis-fired five proposals in one day — design-system ×3 at the 2026-09-04T17-15-00 wrap (sites that NAME the
  # harness / a shell without stating any verdict; E2 dismissed, and NO playbook rule matched) and test-plan §9 ×2
  # (T4 + dependent T5) at the 2026-09-04T20-15-00 wrap (a TARGET-state CI matrix — an unimplemented plan, dismissed
  # under playbook :22). The trigger is now a STATING sentence quoted in the proposal; playbook :46 was widened the
  # same day so the dismissal has a rule.
