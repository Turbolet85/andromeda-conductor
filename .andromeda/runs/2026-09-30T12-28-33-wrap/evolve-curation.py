import json, sys
TS = sys.argv[1]
E = dict(v=1, ts=TS, version="conductor-0.3.0", epoch="Epoch 5 — Polish & ship",
         chunk="2026-09-30-the-screen-reader-pass-grades-again-on-this-host", skill="andromeda-wrap-session")
def rec(letter, **kw):
    r = dict(E); r["id"] = TS + "-" + letter; r.update(kw); return r
rs = [
    rec("a", kind="step", step="curation", outcome="ok",
        counts={"dialogue_rounds": 0},
        consumed=[{"artifact": "conversation", "quality": "ok"}, {"artifact": "report", "quality": "ok"}],
        produced=[{"artifact": "conversation", "signals": []}],
        problem=None),
    rec("b", kind="friction", step="curation", type="recall.corpus-recurrence",
        what="host-win32.md's 2026-09-23 transport correction (backslash pairs go through a scratchpad file) was correct and loaded, yet two log-reading commands were written with a doubled backslash and blocked by the PreToolUse guard before being moved to scratchpad scripts",
        impact={"reformulations": 2}),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
