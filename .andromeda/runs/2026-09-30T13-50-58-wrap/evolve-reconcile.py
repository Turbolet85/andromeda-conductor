import json, sys
ts = sys.argv[1]
E = dict(v=1, ts=ts, version="conductor-0.3.0", epoch="Epoch 5 — Polish & ship",
         chunk="2026-09-30-the-sr-cause-isolated-on-this-host", skill="andromeda-wrap-session", step="reconcile")
docs = ["architecture", "security-plan", "design-system", "layout-templates", "test-plan", "obs-plan", "a11y-plan"]
rs = [
    dict(E, kind="step", id=ts + "-a", outcome="halted-resolved", counts={"retries": 0, "dialogue_rounds": 1, "halted": 1},
         consumed=[{"artifact": "report", "quality": "ok", "note": "every detector ran from the report alone; arch's AR4/AR5 needed the quoted script text in evidence, which the report names but does not restate"}]
                  + [{"artifact": "spec:" + d, "quality": "ok"} for d in docs]
                  + [{"artifact": "drift-base", "quality": "ok"}, {"artifact": "wrap-playbook", "quality": "ok", "note": ":124 escalated four records; :137 failed its precondition on AR1; :149 failed clause (a) on A1/A2"}],
         produced=[{"artifact": "spec:a11y-plan", "signals": ["cascade-wide"], "note": "A1+A2 at :268; leaves rules/a11y.md + docs/a11y-summary.md re-derived"},
                   {"artifact": "spec:test-plan", "signals": ["escalation-shaped"], "note": "T1 pair set"},
                   {"artifact": "spec:security-plan", "signals": ["escalation-shaped"], "note": "S1 rule (b); leaf rules/security.md re-derived"},
                   {"artifact": "spec:architecture", "signals": ["escalation-shaped"], "note": "AR4+AR5 only-committed-reader; registry 38110/38115 B"},
                   {"artifact": "sidecar:a11y-plan", "signals": []}, {"artifact": "sidecar:test-plan", "signals": []},
                   {"artifact": "sidecar:security-plan", "signals": []}, {"artifact": "sidecar:architecture", "signals": []}],
         problem=[{"nature": "process", "solution": "overridden", "note": "E1: the operator rejected three arch proposals (WEBVIEW2_BROWSER_EXECUTABLE_FOLDER env row, :4445 Ports, trust boundary) and the plan's expected amendment 3 with them — arch carries standing committed readers/binders only"}]),
    dict(E, kind="friction", id=ts + "-b", type="ambiguity.playbook-no-match",
         what="A1/A2 (a11y-plan:268 user-consequence and candidate-list retirement) matched no playbook rule — :149 fails clause (a) because the sentence named no precondition measurement; applied under the operator relay's recorded direction and a rule proposed at the wrap card",
         impact={"dialogue_rounds": 0}, artifacts=[".andromeda/a11y-plan.md:268", ".andromeda/playbook.md:149"]),
    dict(E, kind="friction", id=ts + "-c", type="ambiguity.escalation-rounds",
         what="Two escalations (E1 four boundary-widening records, E2 two sole-reader narrowings) resolved in one AskUserQuestion round; the expected-amendments list had forecast an arch registration the operator then rejected",
         impact={"dialogue_rounds": 1, "halted": 1}, artifacts=[".andromeda/runs/2026-09-30T13-50-58-wrap/fanout-results.md"]),
    dict(E, kind="friction", id=ts + "-d", type=None, untyped=True,
         what="The apply script's post-check asserted the old anchor was absent after the write, which cannot hold when the new text extends that anchor (S1 kept 'the count stays seven.' as its prefix); the write had landed, verified by a count of the new text",
         impact={"retries": 1}, artifacts=[".andromeda/runs/2026-09-30T13-50-58-wrap/apply-bodies.py"]),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
