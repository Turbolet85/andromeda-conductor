import json, sys
TS = sys.argv[1]
E = dict(v=1, ts=TS, version="conductor-0.3.0", epoch="Epoch 5 — Polish & ship",
         chunk="2026-09-30-the-screen-reader-pass-grades-again-on-this-host", skill="andromeda-wrap-session")
def rec(letter, **kw):
    r = dict(E); r["id"] = TS + "-" + letter; r.update(kw); return r
rs = [
    rec("a", kind="step", step="report", outcome="ok",
        counts={"extra_reads": 4},
        consumed=[{"artifact": "conversation", "quality": "ok"},
                  {"artifact": "implement-outcome", "quality": "ok", "note": "Branch B outcome, deviations and census all carried by implement's P4 report in this window"},
                  {"artifact": "operator-directive", "quality": "ok", "note": "relay conductor-wrap-sr-2026-09-30: supplemented the Outcome framing and the route successor; changed no implement fact"},
                  {"artifact": "git-state", "quality": "ok"},
                  {"artifact": "plan", "quality": "ok"}],
        produced=[{"artifact": "report", "signals": []}],
        problem=None),
    rec("b", kind="friction", step="report", type="contract.detector-fact-gap",
        what="a configuration-bound measured finding that disproves no spec claim yet is the fact a Branch-B expected amendment carries had no stock Changes bullet (not Spec claims disproved, not Spec-master edits); added a 'Measured findings' bullet",
        impact={"extra_reads": 1}),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
