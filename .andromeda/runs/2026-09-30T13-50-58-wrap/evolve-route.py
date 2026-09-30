import json, sys
ts = sys.argv[1]
E = dict(v=1, ts=ts, version="conductor-0.3.0", epoch="Epoch 5 — Polish & ship",
         chunk="2026-09-30-the-sr-cause-isolated-on-this-host", skill="andromeda-wrap-session", step="route-resolve")
rs = [
    dict(E, kind="step", id=ts + "-a", outcome="ok", counts={"halted": 0, "dialogue_rounds": 0},
         consumed=[{"artifact": "working-route", "quality": "ok"}, {"artifact": "report", "quality": "ok"},
                   {"artifact": "operator-directive", "quality": "ok", "note": "relay §2 named the new entry, its two steps and its placement, satisfying the trajectory gate"}],
         produced=[{"artifact": "working-route", "signals": ["entry-minted", "prereq-closed"], "note": "1 entry minted at the tail head with 3 CARRY blocks (the steps, the three routed reds, arm K); the Rust gate deferral since the prior chunk closed; BLOCKED-ON on Interpretation re-verified against Pulse's working-route (entry still markerless) and kept"}],
         problem=None),
    dict(E, kind="friction", id=ts + "-b", type=None, untyped=True,
         what="route.py epoch read the freshly minted line INDETERMINATE because a prose token 'FIRST:' after one space parses as a freight introducer; reworded to 'first —' and the re-read split the line into its three CARRY blocks",
         impact={"retries": 1}, artifacts=["conductor-0.3.0/working-route.md:69"]),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
