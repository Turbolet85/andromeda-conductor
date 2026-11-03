group: architecture+test-plan
read: .claude/docs/commands.md · 48 · «- `cargo modules generate tree` / `cargo tree` — module/depe»

- leaf: .claude/docs/commands.md:9   class: retired
  leaf-text: «## Agent harness (source of truth — `scripts/agent-run.{sh,ps1}`)»
  source: fct-vwyevac2 «In `agent-run.sh`, the one harness, the short-circuit is environment-CONDITIONAL, firing only on an actually-unmet subject»
  replace-with: «## Agent harness (source of truth — `scripts/agent-run.sh`)»

- leaf: .claude/docs/commands.md:13   class: stale
  leaf-text: «remove run artifacts (`rm -f` in the shells) then `conductor cleanup <run_id>`»
  source: fct-lpebvclu «`rm -f runs/<run_id>.jsonl runs/<run_id>.md` in `agent-run.sh`, then `conductor cleanup <run_id>` — **the harness script issues no SQL** (2026-09-06)»
  replace-with: «remove run artifacts (`rm -f` in `agent-run.sh`) then `conductor cleanup <run_id>`»

- leaf: .claude/docs/commands.md:13   class: stale
  leaf-text: «The shells issue no SQL — the deletes are rusqlite bound-parameter statements inside the binary (2026-09-06).»
  source: fct-apqmcxio «the `conductor cleanup <run_id>` verb is the only caller and the harness script `scripts/agent-run.sh` issues no SQL of its own.»
  replace-with: «The harness script `scripts/agent-run.sh` issues no SQL — the deletes are rusqlite bound-parameter statements inside the binary (2026-09-06).»
