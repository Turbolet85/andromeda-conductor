group: architecture
read: .claude/docs/conventions.md · 56 · «- Warnings & gotchas → `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 72 · «- Path-specific rules → `.claude/rules/*.md`. Decision ratio»
read: .claude/docs/workflow.md · 51 · «Reference-based CLAUDE.md: a thin index (≤200 lines) with de»

- leaf: .claude/docs/conventions.md:9   class: retired
  leaf-text: «`scripts/` (`agent-run.{sh,ps1}`)»
  source: fct-uhih6b6b «├─ scripts/⏎│  ├─ agent-run.sh            # headless source-of-truth entrypoint»
  replace-with: «`scripts/` (`agent-run.sh`)»
