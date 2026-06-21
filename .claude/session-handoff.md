# Session Handoff

**Last Updated:** 2026-06-21T22:57:37Z
**Branch:** build/conductor-0.1.0
**Status:** clean
**Last Commit:** 2026-06-21-connection-lifecycle-scenarios — feat: connection-lifecycle scenarios (P-001..P-004) — 4 scenarios/*.toml + Scenario.expected TOML wiring (conductor-core/timeline)

## Position
- Done: **2026-06-21-connection-lifecycle-scenarios** — **Epoch 7 (Scenario catalog) ch1/8.** First `scenarios/*.toml` catalog entries (P-001 `receiver-lifecycle-state`, P-002 `last-span-ago-tracking`, P-003 `receiver-failed-port-conflict`, P-004 `orthogonal-health-domains`) + additive `Scenario.expected: Vec<ExpectedCheck>` TOML wiring (`#[serde(default)]` + `#[garde(dive)]`, reusing existing `ExpectedCheck`). Closes carried follow-up (d); `holds` still deferred.
- Next: **Epoch 7 ch2 — Hard-signals scenarios (P-005..P-008)** → `/andromeda-phase` to promote + plan.

## Work done
3 files MOD (`conductor-core/src/scenario.rs`: +`expected` field +8 tests · `error.rs` + `conductor-timeline/src/convert.rs`: test struct-literals); 4 NEW `scenarios/*.toml` (P-001..P-004). +8 tests, **no new dependency**. Gates: core+timeline 117/117 · workspace **309/309** (301→309) · clippy `-D` clean · doctest 0 · smoke `agent-run.sh status` exit 0 (read-only — no boot-path change). Code-graph 1003n/4161e.

## Drift resolved
1 proposal · **0 applied · 1 routine dismiss + 1 playbook rule appended · 0 escalations** (drift = 0). **arch** D-arch-resources (warning) proposed registering the 4 `scenarios/*.toml` in §Occupied Resources → routine dismiss (`scenarios/` already registered in §Cross-cutting [Config management] + the directory tree; per-file config artifacts are content within a registered directory, not new occupied resources). **Broadened the playbook** (user-approved) so the 7 remaining Epoch-7 scenario chunks don't re-fire. 6/7 detectors clean; all escalate-severity detectors (security-input/subprocess/deps · obs-stack/redaction) clean.

## Notes
- **Key decisions (P4 AskUserQuestion):** (1) **4 files, one P-ID each** — 1:1 with the spec's distinct verification recipes; (2) **wire `expected` now** — establishes the catalog's declarative read-back shape, closes follow-up (d). `holds` deferred (drive+observe needs no go/no-go; the visual badge is a `ManualCheck` report-state). `ComparisonKind` NOT extended for tolerance windows (P-002 ±1s is the Epoch-8 evaluator's concern). Realization (emit/silence + `:4317` bind + live verify) deferred to the Epoch-8 driver. Per-P-ID params from `refs/pulse-capability-spec.md`.
- **Curation:** 1 Tier-3 (scenario-catalog scope + `expected` carrier); 0 Tier-1/2. 0 conflicts, 0 deferred.
- **Follow-up (carried):** (a) `coverage-matrix.md` not yet materialized at repo root (Epoch-8 cli drives it; completeness gate Epoch 10). (b) test-plan §3 ↔ obs-plan §3 two-record-shapes doc-reconcile (still deferred). (c) `report.generate`/coverage obs span → Epoch-8 caller. (d) **CLOSED** — `Scenario.expected` TOML wiring done (`holds` still pending until a scenario gates a non-Conductor step). (e) `opentelemetry-proto default-features=false` trim. (f) Epoch 8/9 coverage/scenario surfaces reuse `coverage_matrix()` + `Lamp::for_record`. (g) **NEW:** the Epoch-8 CLI driver realizes the connection-lifecycle runtime leg — scenario emit/silence on/off, the P-003 port-occupier `:4317` bind, and live MCP verify against `scenario.expected`.
- **Last failed command:** none.
