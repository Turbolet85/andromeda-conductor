# layouts extract

## No domain coverage

This chunk adds `tracing` self-observation spans (`fault.silence` / `fault.ramp` / `fault.port_occupier`) to `logs/agent-latest.jsonl` on a headless, no-SUT run path with no rendered surface — it touches neither the desktop-webview console nor the cli's printed output structure, and the scope explicitly states the GUI surfaces are untouched.
