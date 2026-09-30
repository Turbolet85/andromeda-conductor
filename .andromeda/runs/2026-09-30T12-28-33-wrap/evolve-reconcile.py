import json, sys
TS = sys.argv[1]
E = dict(v=1, ts=TS, version="conductor-0.3.0", epoch="Epoch 5 — Polish & ship",
         chunk="2026-09-30-the-screen-reader-pass-grades-again-on-this-host", skill="andromeda-wrap-session")
def rec(letter, **kw):
    r = dict(E); r["id"] = TS + "-" + letter; r.update(kw); return r
docs = ["architecture", "security-plan", "design-system", "layout-templates", "test-plan", "obs-plan", "a11y-plan"]
rs = [
    rec("a", kind="step", step="reconcile", outcome="ok",
        counts={"retries": 1, "dialogue_rounds": 0, "halted": 0},
        consumed=[{"artifact": "report", "quality": "ok", "note": "all seven detectors ran from the report alone; a11y-plan's agent pointed to the expected-amendment route itself"}]
                 + [{"artifact": "spec:" + d, "quality": "ok"} for d in docs]
                 + [{"artifact": "drift-base", "quality": "ok"},
                    {"artifact": "wrap-playbook", "quality": "thin", "note": "no rule for a dated configuration-bound finding that extends a measured platform set; settled by recorded direction"}],
        produced=[{"artifact": "spec:a11y-plan", "signals": ["orchestrator-raised"], "note": "O1 via Validate check 5: the plan's Branch-B expected amendment, 0 detector proposals over 7 docs"},
                  {"artifact": "sidecar:a11y-plan", "signals": [], "note": "splice append read back, last line = this entry's Ref"},
                  {"artifact": "cascade", "signals": ["control-refused-pattern"], "note": "cascade.py refused sr-heard whose control never fired pre-pass (this pass's own phrasing); dropped and re-run; leaves rules/a11y.md and a11y-summary.md re-derived"}],
        problem=None),
    rec("b", kind="friction", step="reconcile", type="ambiguity.playbook-no-match",
        what="the one amendment (a configuration-bound SR finding extending a11y-plan §3's measured platform set) matched no playbook rule; applied on the recorded direction (plan Branch-B expected amendment + operator relay); rule to be proposed at the wrap card",
        impact={}),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
