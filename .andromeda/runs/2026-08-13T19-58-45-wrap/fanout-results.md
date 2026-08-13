# Fan-out results — 2026-08-13-per-check-read-back-extraction

7 doc-agents, one per spec source, all returned clean YAML (no stripping needed → no raw twins warranted).

| doc | verdict | detail |
|---|---|---|
| architecture.md | `proposals: []` | D-arch-resources: only a *module* was added (below the crate grain arch registers); `conductor-verify` already listed; no port/socket/endpoint/IPC/env-var/crate. D-arch-decisions: Dependencies "none added, none bumped", `Cargo.lock` zero lines. Independently affirmed arch:60 already scopes `KnownResidual` response-side, so deviation 3 owes no amendment. |
| security-plan.md | `proposals: []` | D-security-input: the one new external-input consumer (`observe`) reuses the shipped bounded decode, "adds no second decode path". D-security-subprocess: `preflight.rs` touched only by a signature-preserving MOVE; spawn/data-dir/negotiation untouched. D-security-deps: zero lock movement; the audit re-pin's bounded-wait remedy is already the documented §Dependency Security baseline. |
| design-system.md | `proposals: []` | D-design-tokens: all 4 new surfaces carry `tokens n/a` / `a11y n/a` (no UI). D-design-derived-count: grepped all three moved values — 558/573 zero hits; `fingerprints` prose names the SET not a literal (and the doc becomes *more* true now); `retrieve_report` absent from the doc. |
| layout-templates.md | `proposals: []` | D-layout-surface: no new rendered region; degraded rows render through the already-documented residual lamp. D-layout-derived-count: no baked OLD value survives — the cli suite sample already shows populated `fingerprints` for measured rows, so the code moved *toward* the documented sample. |
| test-plan.md | `proposals: []` | D-tests-coverage: every new symbol has a matching coverage row at the mandated tier; 573/573 zero retries. D-tests-framework: all runners are the ones §2/§3/§4/§9 name. D-tests-obs-harness: Schema/config "none" — envelope stays 11 fields, so §3 ↔ obs-plan §3 stay in agreement and the obs-side reword is not one-sided. D-tests-derived-count: 558 absent; no "always empty" qualifier baked for `fingerprints`. |
| obs-plan.md | **1 proposal** (warning) | D-obs-instrumentation — §4 CP5's degraded-mode span wording describes a call Conductor cannot make and attributes that would be dropped. See below. |
| a11y-plan.md | `proposals: []` | D-a11y-surface: all new symbols are Rust backend, all 4 surfaces `a11y n/a`; cli/tauri diffs empty. D-a11y-obs-schema: envelope unchanged at 11 fields; `fingerprints` "may be empty" stays true, name/type/cardinality unchanged. |

## The single proposal

- **detector:** D-obs-instrumentation · **severity:** warning · **doc:** obs-plan
- **section:** §4 Span/Trace Coverage — Known-residual classification path (CP5), plus the `mcp_method`
  attribute naming in §4's `verify.readback` / `verify.readback_fingerprints` entries and the §1 CP5 row.
- **change:** reword CP5's must-trace span from `verify.readback_degraded_mode` (MCP call with
  `degraded_mode=true`) carrying `degraded_mode_requested` / `response_received` to the shipped
  response-observed shape; state that a §4 span attribute must be in `conductor-core::redact::ALLOWLISTED_FIELDS`
  or it emits nothing; rename `mcp_method` → `mcp_tool`.
- **rationale:** report *Reverted / negative API facts* + Deviation 1 — two artifact-verified disqualifiers.

## Validation (5 checks, orchestrator-direct)

1. **Playbook** — matches `playbook.md:28` (spec-illustration → sound-impl reconciliation, where the report
   shows the invariant still holds) AND `:88` (a chunk OPERATIONALIZING a spec'd path for the first time
   surfaces that the spec's own description of it names a stale/wrong field-list). This chunk is
   `retrieve_report`'s first caller in the workspace, so it is exactly the operationalizing chunk.
   → **routine**, stage to apply. No escalation.
2. **Cross-contradiction** — n/a (one proposal).
3. **Intent-consistency** — the report matches the chunk's working-route entry and `plan.md` acceptance
   criteria; all three deviations carry justifications. No unjustified divergence.
4. **Absence needs evidence** — every absence claim cites its search: design ("grep for both values: zero
   hits"), tests ("grepped all three moved values"), layouts ("nothing near 558/573 anywhere in it"). The
   arch/security/a11y absence claims are report-sourced, not inferred from a partial view.
5. **Expected-amendments reconciliation** — `plan.md`'s `Expected amendments (wrap)` named the
   `degraded_mode_requested` reword + the `mcp_method`/`mcp_tool` mismatch. The proposal covers both **plus**
   the `response_received`/allowlist facet. Coverage floor met and exceeded; nothing under-ran.

**Operator directive 3 check:** no detector proposed encoding a scenario-allowlist for `KnownResidual` — the
guard did not need to fire. Three agents independently confirmed `architecture.md:60` already scopes the state
response-side ("a `retrieve_report` result returned under `degraded_mode`"), matching the shipped `state_for`.

**Outcome: 1 routine amendment applied · 0 escalations · 0 open.**
