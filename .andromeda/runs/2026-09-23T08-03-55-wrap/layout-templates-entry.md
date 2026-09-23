
## 2026-09-22-interpretation-proven-live — `preconditions --for` and the `--live real-model` selector

**Section:** Surface: cli › Primary screens (commands) — the `conductor preconditions` bullet (`:187`) and the `scripts/agent-run.{sh,ps1}` bullet (`:190`)

**Change:**
**(L2 + L3)** the `preconditions` synopsis becomes `conductor preconditions [--for <SCENARIO>] [--json]`; the three handles are read for an affirmative declaration under the deterministic posture (the flagless default); `--for` takes a scenario NAME (a `P-NNN` value is refused at parse, exit 2, because a P-ID can name several scenarios) and grades the handles under that scenario's declared `l4_posture` — under `real-model` the L4 handle is read for ABSENCE, its unmet line reading `declared in this environment, but the real-model posture requires it absent or falsy`.
**(L4)** the per-KIND grading sentence keeps its deterministic arm and gains the real-model one: the L4 flag is unmet when EITHER side's truthiness rule declares it (Conductor's `true`/`1` ∪ Pulse's `1`/`true`/`yes`), every other handle graded as before; exit 0 was measured reachable under that posture too (2026-09-23).
**(L1)** the agent-run bullet: a third-token selector — `--live real-model` runs the ONE operator-gated real-model leg instead of the suite, led by `preconditions --for real-model-interpretation` and refusing the same way; an unknown selector prints usage and exits 2 before any probe, in both shells — a selector of `--live`, neither a sixth command nor a new stage flag.

**Why:** report §Changes (Symbols/APIs — `conductor preconditions --for`, the statement text; Harness — the selector and its usage line; Outcome — the `--for` probe green under the live env, both shells' bogus-selector refusals) and the plan's Expected amendment "layout-templates §cli Primary screens". The detector's `basis` fields cited source lines the report does not carry, so every applied sentence was re-derived from the report's facts rather than pasted.

**Sweep (full ledger `.andromeda/runs/2026-09-23T08-03-55-wrap/reconcile-sweep.md`):**
- **G3 grading** — `:187` ×4 (the amended bullet); no other layout-templates hit states the probe's grading.
- **G9 `--live`** — `:190` (the amended bullet) is the only layout-templates site describing `--live`'s shape.
- **G1 `per-P-ID`** — `:26`, `:40`, `:127`, `:144`, `:145`, `:183`, `:187`, `:282` ×2, `:310`: P-ID-keyed rows, lamps and verdict lines, not a scenario cardinality — left.
- Leaves: `docs/design-summary.md` and `.claude/rules/frontend.md` carry no cli command list — checked, no re-derivation needed; the cli command reference (`docs/commands.md`) was re-derived from architecture + these bullets.
