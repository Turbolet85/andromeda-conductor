### Log file location

- **CLI:** `logs/agent-latest.jsonl` (project root, relative to `CONDUCTOR_RUNS_DIR`) in agent mode; stderr in dev / non-agent mode
- **Tauri backend:** `<runs_dir.parent()>/logs/conductor-tauri.jsonl` — the file NAME is fixed and the DIRECTORY rides `CONDUCTOR_RUNS_DIR`; no writer since 2026-10-11 (its caller, `tauri_log_path()`, left with the window), and the sink goes at `Panel-shaped types retired`
- **Rotation:** N/A — Minimal tier, no retention/compliance requirement; logs are paste-to-AI artifacts, not persisted archives
- **Paste-to-AI workflow:** user tails CLI output (`scripts/agent-run.sh | tail -f`) to Claude Code / Claude web
