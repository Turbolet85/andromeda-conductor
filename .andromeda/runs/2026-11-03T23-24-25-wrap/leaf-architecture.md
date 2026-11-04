group: architecture
read: .claude/docs/commands.md · 48 · «- `cargo modules generate tree` / `cargo tree` — module/depe»
read: .claude/docs/conventions.md · 56 · «- Warnings & gotchas → `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 72 · «- Path-specific rules → `.claude/rules/*.md`. Decision ratio»
read: .claude/docs/stack.md · 56 · «1. Update `architecture.md` §Stack first → 2. update `Cargo.»
read: .claude/docs/workflow.md · 51 · «Reference-based CLAUDE.md: a thin index (≤200 lines) with de»
read: .claude/rules/frontend.md · 27 · «- 2026-09-17: **In a webview spec, an expectation that inter»
read: CLAUDE.md · 139 · «<!-- USER:session-learnings end -->»

- leaf: .claude/docs/stack.md:40   class: retired
  leaf-text: «since 2026-06-18), the mutation-tally gate (`scripts/mutation-gate.py` + `scripts/mutation-roster.toml`, since 2026-09-13; a stdlib `selftest` verb over the committed `scripts/fixtures/mutation-gate/` since 2026-09-30) and the architecture-registry check (`scripts/arch-registry-check.py`, stdlib only, since 2026-09-24).»
  source: fct-22zuwdf4 «the code-graph pipeline (`scripts/code-graph.py` + `scripts/scip_pb2.py` + `scripts/requirements.txt`, since 2026-06-18) and the mutation-tally gate (`scripts/mutation-gate.py`»
  replace-with: «since 2026-06-18) and the mutation-tally gate (`scripts/mutation-gate.py` + `scripts/mutation-roster.toml`, since 2026-09-13; a stdlib `selftest` verb over the committed `scripts/fixtures/mutation-gate/` since 2026-09-30).»
