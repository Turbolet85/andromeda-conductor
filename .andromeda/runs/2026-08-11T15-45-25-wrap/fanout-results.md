# Fan-out results — 2026-08-10-scenario-run-root-span-tree

7 Explore doc-agents, one per spec source, one parallel batch. **2 proposals · 5 clean · 0 escalations.**

| doc | verdict | detail |
|---|---|---|
| arch | `proposals: []` | D-arch-resources: no new port/socket/endpoint/IPC/env-var/crate; `logs/agent-latest.jsonl` already registered, arch delegates its schema to obs-plan §3 and keeps no span-name registry. D-arch-decisions: `tracing` 0.1.44 already in §Stack; no new library, no OTel crate. |
| security-plan | `proposals: []` | D-security-input: no new external-input surface (the span record is *emitted* output, not input). D-security-subprocess: no file touches `conductor-verify`; `verify.readback.connect` only became *visible*. D-security-deps: one edge, no new `[[package]]`, `cargo deny` green; the `cargo audit` failure is the advisory-DATABASE class §Dependency Security already documents. |
| design-system | `proposals: []` | D-design-tokens: no rendered UI element (all four new surfaces `tokens n/a`). D-design-derived-count: report's *Counts moved* is `none`; grepped for 21/29 in palette rows / ANSI map / token labels — no hits. |
| layout-templates | `proposals: []` | D-layout-surface: no new cli stdout line or webview region. D-layout-derived-count: `(9 unbacked)` caption still matches; the doc carries no allowlist content. |
| test-plan | **1 proposal** | **D-tests-obs-harness** (warning) → §3 Log format. |
| obs-plan | **1 proposal** | **D-obs-instrumentation** (warning) → §4 CP1 Cleanup. D-obs-stack + D-obs-redaction clean. |
| a11y-plan | `proposals: []` | D-a11y-surface: no interactive element. D-a11y-obs-schema: the §6 envelope is unchanged (11 fields); deviation 4 actively *preserves* the §6 crosswalk by shipping canonical serde spellings. |

## Validation (5 checks)

1. **Playbook** — both proposals `routine`. D-obs-instrumentation matches the 2026-06-27 *operationalizing-a-spec'd-step* rule (this chunk is the wiring chunk the 2026-06-21 deferred-span rule named) **and** the 2026-06-15 *wording→sound-impl* rule. D-tests-obs-harness: the 2026-06-21 *pre-existing-bind dismiss* rule does **not** apply — its precondition ("the report changes the harness / log format") is **met**, since test-plan §3 owns the format this chunk changed.
2. **Cross-contradiction** — none; the two proposals sit in different docs and describe the same shipped reality consistently.
3. **Intent-consistency** — aligned. The layer-capability divergence was already classified intent-incomplete and `scope.md` amended at /andromeda-phase P5.
4. **Absence-needs-evidence** — the five clean returns each cite the search establishing the absence (line numbers, greps). Accepted.
5. **Expected-amendments reconciliation** — the plan listed **two**. Entry 1 (§4 CP1 cleanup wording) was proposed by D-obs-instrumentation. **Entry 2 (stale Tauri command names) was proposed by NO detector** — the obs agent explicitly excluded it as pre-existing. Orchestrator-raised as routine: the report substantiates it (line numbers verified first-hand at :49/:136/:373) and an explicit operator directive covers it. The coverage floor held.

**Escalations: 0.** No HALT.

## Applied (4 body edits · 4 sidecar entries)

| # | Doc · section | Change |
|---|---|---|
| A1 | obs-plan §4 CP1 Cleanup (:304) | root closes when the scenario returns; report-seam spans are run-scoped **siblings** correlated by `run_id` |
| A2 | obs-plan §4 Known-residual Cleanup (:354) | **same false claim found by first-hand grep, not proposed** — fixed in the same amendment (multi-section, both landed) |
| B | obs-plan §1 :49 · §4 :136 · :373 | `start_scenario`/`stop_scenario`/`get_run_report`/`operator_pause_go_no_go` → the shipped `start_run`/`stop_run`/`run_report`/`resolve_operator_hold` |
| C | test-plan §3 Log format (:188) | the self-obs stream's **two line variants** recorded (owner side) |
| D | obs-plan §3 (:229) | the same variant recorded on the **deriving** side — the lateral §3↔§3 bind; omitting it would have been exactly the one-sided change D-tests-obs-harness guards |

## Cascade

- **Lateral binds:** test-plan §3 ↔ obs-plan §3 both amended in lockstep (edit D). a11y↔obs schema bind untouched (envelope unchanged).
- **Cross-master citations:** grepped all seven masters + `.claude/` for the amended CP1 wording and the stale command names — the only hits outside obs-plan are in `.andromeda/runs/` historical run-dirs, which are immutable records, correctly untouched.
- **Leaf re-derivation:** `rules/observability.md` and `docs/obs-summary.md` (both restated the "two record shapes" line, now carrying the two line variants). `docs/tests-summary.md` · `rules/testing.md` · `rules/verification-harness.md` re-checked and **correctly need nothing** — they describe the *emission journal* and Run-report envelope, not the self-obs line shape. `architecture.md` unchanged ⇒ no CLAUDE.md `GENERATED:setup:*` re-derive.
- **Method note:** `obs-summary.md` says "Two **shapes**" where the rule file says "Two **record shapes**" — a wording-scan for the amended phrase would have missed it. Found by re-computing from the changed source, which is what the cascade rule mandates.

## Observed, NOT amended (out of report scope)

obs-plan :49 also states Tauri command spans are instrumented "via `#[tracing::instrument]`", while the shipped convention is a manual `info_span!(...).entered()` — the attribute does not stack with `#[tauri::command]` (`.claude/rules/observability.md`, 2026-06-26). Verified first-hand, but **not amended**: the operator directive scoped entry 2 to the command *names*, and the report substantiates only those. Recorded here and in the handoff for a future pass rather than silently widened.
