import json, sys
TS = sys.argv[1]
E = dict(v=1, ts=TS, version="conductor-0.3.0", epoch="Epoch 5 — Polish & ship",
         chunk="2026-09-30-the-screen-reader-pass-grades-again-on-this-host", skill="andromeda-wrap-session")
def rec(letter, **kw):
    r = dict(E); r["id"] = TS + "-" + letter; r.update(kw); return r
rs = [
    rec("a", kind="step", step="route-resolve", outcome="halted-resolved",
        counts={"dialogue_rounds": 1, "halted": 1, "extra_reads": 3},
        consumed=[{"artifact": "report", "quality": "ok"},
                  {"artifact": "working-route", "quality": "ok"},
                  {"artifact": "operator-directive", "quality": "thin", "note": "relay named the slot as ':65', a line number this chunk's promotion had moved onto the chunk's own frozen line"}],
        produced=[{"artifact": "working-route", "signals": ["tail-reshaped"], "note": "1 new entry ahead (The SR cause isolated on this host) carrying the Rust deferral PREREQ + the relay's cheapest-first CARRY; BLOCKED-ON on Interpretation re-verified against Pulse's route (still markerless)"}],
        problem=None),
    rec("b", kind="friction", step="route-resolve", type="contract.carry-no-owner",
        what="the operator relay placed the successor 'right before :65' — a stale line number now naming this chunk's own frozen line; one AskUserQuestion round resolved the anchor as 'before Full-gate regression over the moved surfaces'",
        impact={"dialogue_rounds": 1}),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
