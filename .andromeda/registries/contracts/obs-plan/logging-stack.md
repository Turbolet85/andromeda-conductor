### Logging stack

- **Library:** `tracing` 0.1.44 (Rust async tracing facade) + `tracing-subscriber` 0.3.23 (JSON formatter + layer composition)
- **Format:** JSONL (one JSON object per line) with schema from tests binding contract (Section 6)
- **Sink (CLI):** JSON to stderr in dev / non-agent mode, JSON to file `logs/agent-latest.jsonl` under `--agent-mode` — a WRITER choice, never a format choice (one `JsonObsLayer` over every writer); no TTY detection
- **Sink (Tauri backend):** no writer since 2026-10-11 (its caller left with the window; the sink goes at `Panel-shaped types retired`) — file `conductor-tauri.jsonl` under `<runs_dir.parent()>/logs/` (the directory rides `CONDUCTOR_RUNS_DIR` — Log file location below); stderr only as an open-failure fallback
- **Agent-mode flag:** `--agent-mode` CLI flag redirects the JSON stream from stderr to the file (the format is JSON either way); agent mode is triggered by the flag OR the `CONDUCTOR_AGENT_MODE` env — a **read-only trigger** (`agent_mode = flag || env-set`) Conductor never WRITES (avoiding edition-2024 `unsafe std::env::set_var`; the harness/operator exports it). Observable mode is identical to "sets it internally"
