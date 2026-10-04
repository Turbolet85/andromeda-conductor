### Log file location

- **CLI:** `logs/agent-latest.jsonl` (project root, relative to `CONDUCTOR_RUNS_DIR`) in agent mode; stderr in dev / non-agent mode
- **Tauri backend:** `<runs_dir.parent()>/logs/conductor-tauri.jsonl` — the file NAME is fixed and the DIRECTORY rides `CONDUCTOR_RUNS_DIR` (`tauri_log_path()` = `runs_dir.parent()/logs`), so the sink moves with whoever set the handle: with it unset and the launch cwd at the repo root, the project-root `logs/conductor-tauri.jsonl` (measured 2026-09-01); on the a11y suites, per suite — `runs/logs/` (routine `--e2e` + `sr-empty`), `runs/driven/logs/` (the driven arm) and `runs/sr-leg/logs/` (`sr` / `sr-error`), each measured 2026-09-02
- **Tauri frontend:** browser console JSON (paste-to-AI; no file persistence)
- **Rotation:** N/A — Minimal tier, no retention/compliance requirement; logs are paste-to-AI artifacts, not persisted archives
- **Paste-to-AI workflow:** user tails CLI output (`scripts/agent-run.sh | tail -f`) or copies browser console JSON to Claude Code / Claude web
