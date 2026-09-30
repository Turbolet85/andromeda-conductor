
## 2026-09-30-mutation-gate-grades-every-tally-it-rests-on — the mutation gate's selftest and fixture tree registered
**Section:** §Stack and Technologies — "Operator instruments (host runtime)" row · §Infrastructure Patterns — Directory structure
**Change:**
- §Stack row: the mutation-tally gate's roster rows each carry a required `tally` (`missed` | `timeout`). Since 2026-09-30 a stdlib-only `selftest [--fixtures DIR]` verb grades the committed fixture tree `scripts/fixtures/mutation-gate/`, spawning no process and running no `cargo mutants`. The row's closing sentence stands: no CI step invokes any instrument, and none adds a sixth `agent-run.{sh,ps1}` command.
- Directory structure: the `scripts/` block gains `mutation-gate.py` (operator-local, `<unit>` | `selftest`, no CI step, neither harness shell, no 6th command), `mutation-roster.toml` (the expected `missed` / `timeout` multisets per unit) and `fixtures/mutation-gate/` (the `selftest` fixture tree).
**Why:** the chunk added the verb and the fixture tree to an already-registered operator instrument. The tree had no row for either gate file. Neither registry section was edited (38 111 / 38 028 B of 38 115, within target).
**Kept:** the tree still has no rows for the other operator instruments (`arch-registry-check.py`, `code-graph.py` and its companions). That gap predates this chunk and is left for the chunk that owns those files.
**Ref:** .andromeda/runs/2026-09-30T08-33-23-wrap/
