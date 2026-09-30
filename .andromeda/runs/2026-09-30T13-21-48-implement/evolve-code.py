import json
ts = "2026-09-30T13:27:04Z"
env = dict(v=1, ts=ts, version="conductor-0.3.0", epoch="Epoch 5 — Polish & ship",
           chunk="2026-09-30-the-sr-cause-isolated-on-this-host", skill="andromeda-implement", step="code")
rs = [
    dict(env, kind="step", id=ts + "-a", outcome="ok", counts={"extra_reads": 5, "soft_exit": 0},
         consumed=[{"artifact": "plan", "quality": "ok"},
                   {"artifact": "research", "quality": "ok", "note": "lists: 2 new evidence files, none modified; scripts gitignored under runs/"},
                   {"artifact": "scope", "quality": "ok"},
                   {"artifact": "source", "quality": "ok", "note": "wdio.conf.ts NVDA spawn/readiness/stop, activate-window.ps1, screen-reader.e2e.ts, tauri.conf.json title read to mirror the leg's forms"}],
         produced=[{"artifact": "source", "signals": ["style-inferred"], "note": "two gitignored session scripts mirror wdio.conf.ts startNvda/stopNvda and activate-window.ps1; parse-checked under PowerShell 5, ASCII-only"}],
         problem=[{"nature": "environment", "solution": "workaround", "note": "CONDUCTOR_NVDA unset in the agent shell (as the handoff states); the one portable NVDA copy located by a bounded PowerShell search and set per command"}]),
    dict(env, kind="friction", id=ts + "-b", type=None, untyped=True,
         what="The Write tool decoded a JSON backslash-uE004 escape inside a single-quoted PowerShell string into the raw U+E004 character, making the ASCII-only .ps1 non-ASCII; caught by a byte check, rewritten to build the escape from [char]92",
         impact={"iterations": 1}, artifacts=["runs/sr-control/edge-webdriver-walk.ps1"]),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
