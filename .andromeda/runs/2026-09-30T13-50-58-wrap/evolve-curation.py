import json, sys
ts = sys.argv[1]
E = dict(v=1, ts=ts, version="conductor-0.3.0", epoch="Epoch 5 — Polish & ship",
         chunk="2026-09-30-the-sr-cause-isolated-on-this-host", skill="andromeda-wrap-session", step="curation")
rs = [
    dict(E, kind="step", id=ts + "-a", outcome="ok", counts={"dialogue_rounds": 0},
         consumed=[{"artifact": "conversation", "quality": "ok"}, {"artifact": "report", "quality": "ok"}],
         produced=[{"artifact": "conversation", "signals": []}],
         problem=None),
    dict(E, kind="friction", id=ts + "-b", type="ambiguity.filter-borderline",
         what="Three measured host-tool gotchas (PS5 [ushort], Edge background relaunch, Write-tool backslash-u decode) each scored exactly 0.6 before a conditional +0.2; with five 0.8 survivors the max-3 cap chose by consequence (a failed slot start) and deferred two",
         impact={"deferred": 2}, artifacts=[".andromeda/runs/2026-09-30T13-50-58-wrap/curation.md"]),
    dict(E, kind="friction", id=ts + "-c", type="recall.corpus-recurrence",
         what="host-win32.md's 2026-09-23 backslash-pair transport correction did not prevent two more doubled-backslash commands this wrap (a sed regex and a grep alternation); the bash guard blocked both before they ran",
         impact={"retries": 2}, artifacts=[".claude/rules/host-win32.md"]),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
