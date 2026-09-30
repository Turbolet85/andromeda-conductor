# Fan-out results — 2026-09-30-the-screen-reader-pass-grades-again-on-this-host

Seven Explore doc-agents, one parallel batch, prompt from `amendment-flow.md` §Fan-out sent verbatim; `{contracts_line}`
dropped for all seven (architecture / test-plan / obs-plan / a11y-plan: `registry.py contracts` exit 3 `NOT MIGRATED`;
security-plan / design-system / layout-templates: no keyed-contract section). Every return was `proposals: []` followed by
`#` comment lines; stripping removed only those comments (their substance below). No entity escapes in any return; no
raw twin warranted (no `[]` return changed by stripping beyond commentary, no parse failure).

| doc | verdict | substance stripped |
|---|---|---|
| architecture | proposals: [] | CONDUCTOR_NVDA / CONDUCTOR_A11Y_STRICT already registered (§Occupied Resources :194, :196); lever named-not-built is no stack change; no second owner; no platform verdict retired |
| security-plan | proposals: [] | no new input surface, spawn, or dependency; no platform verdict retired |
| design-system | proposals: [] | no new UI element; 21→24 is a plan forecast, no design tally; no platform verdict |
| layout-templates | proposals: [] | no new surface; SR row references (:139, :154) are dated row measurements, R0-01 still heard |
| test-plan | proposals: [] | no new code path; registered runners; harness unchanged; 0 hits for 154.0.4258.37 / allowInChromium; `:47` and `:307` still hold; Branch-A entries superseded |
| obs-plan | proposals: [] | no new operation, telemetry dep, artifact leak or CI gate; `:621` S1-01 channel unaffected |
| a11y-plan | proposals: [] | no new element, schema unchanged, no retired verdict; notes the Branch-B finding reaches §3 via the plan's expected amendment |

## Validate

- Detector proposals: 0 — nothing to validate under checks 1–4.
- **Check 5 — expected-amendments reconciliation** (the plan's list is the floor):
  - Branch A · test-plan §6 — superseded (Branch B; report Changes → Expected amendments). No raise.
  - Branch A · a11y-plan §3 — superseded (Branch B). No raise.
  - Branch A · test-plan §9 Matrix builds — superseded (Branch B). No raise.
  - Branch B · a11y-plan §3 *Screen reader test pattern* — no detector proposed it → **raised by the orchestrator**
    (O1). Substantiated by the report's *Measured findings* bullet → **apply (routine)**. Playbook: no rule names a
    configuration-bound SR finding; the operator's recorded direction settles it (the plan's Branch-B expected
    amendment, and the relay "Branch B, as the plan forecast") → apply per check 1's recorded-direction branch.
- **Check 6 — disproved claims:** the report's bullet is `none` — nothing to dispose. (The CARRY's hypothesis stays
  untested; it is not a spec claim.)
- Check 3 — intent consistency: the scope record's one line (in-intent, `nvda-pass.defaults.json`, serves step 3) holds
  its `serves`; Branch B is the P4-ruled stop of the intent (scope.md §What this chunk builds #2). No escalation.

### O1 — orchestrator-raised (check 5)
- section: a11y-plan §Screen reader test pattern → *Per-surface test spec* (`:268`)
- change: bind the measured platform set to its configuration — on WebView2 runtime/driver 154.0.4258.37 / 154.0.4258.37,
  Windows 26200.9457, NVDA 2026.2 the agent arm hears focus only in the first burst and no later focus change under both
  NVDA Chromium object models; live regions still speak; cause recorded, not established; the user consequence.
- disposition: **apply** (check 5 · recorded direction)

Escalations: 0.
