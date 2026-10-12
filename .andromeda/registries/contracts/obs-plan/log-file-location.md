### Log file location

- **CLI:** `logs/agent-latest.jsonl` (project root, relative to `CONDUCTOR_RUNS_DIR`) in agent mode; stderr in dev / non-agent mode
- **Rotation:** N/A — Minimal tier, no retention/compliance requirement; logs are paste-to-AI artifacts, not persisted archives
- **Paste-to-AI workflow:** user tails CLI output (`scripts/agent-run.sh | tail -f`) to Claude Code / Claude web
