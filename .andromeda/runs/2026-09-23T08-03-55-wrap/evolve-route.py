import json
import re

ROOT = "D:/dev/projects/conductor"
TS = "2026-09-23T09:51:34Z"
EPOCH = open(f"{ROOT}/conductor-0.3.0/working-route.md", encoding="utf-8").read().split("\n")[40][4:]
assert EPOCH.startswith("Epoch 4"), EPOCH
base = {"v": 1, "ts": TS, "version": "conductor-0.3.0", "epoch": EPOCH,
        "chunk": "2026-09-22-interpretation-proven-live", "skill": "andromeda-wrap-session", "step": "route-resolve"}
TRAIL = ".andromeda/runs/2026-09-23T08-03-55-wrap/route-2026-09-22-interpretation-proven-live.json"

records = [
    dict(base, kind="step", id=f"{TS}-a", outcome="ok",
         counts={"dialogue_rounds": 0, "halted": 0, "extra_reads": 3},
         consumed=[{"artifact": "report", "quality": "ok"},
                   {"artifact": "working-route", "quality": "thin", "note": "two pre-existing tail lines failed the route grammar (a one-space BLOCKED-ON, a CARRY 2 introducer)"}],
         produced=[{"artifact": "working-route", "signals": ["tail-reshaped"],
                    "note": "3 CARRY pins + 1 CONTEXT (Diagnostic-quality cluster x2 CARRY + CONTEXT; Full-gate regression x1 CARRY), 0 PREREQ, 0 reorder, 0 residuals; the line-48 BLOCKED-ON premise re-verified (Pulse HEAD still 83d4060)"}],
         problem=[
             {"nature": "process", "solution": "removed-cause", "note": "repaired two pre-existing route-grammar breaks on the markerless tail while pinning: line 48's BLOCKED-ON after ONE space (now two) and line 57's 'CARRY 2:' introducer (now 'CARRY:')"},
             {"nature": "process", "solution": "deferred", "note": "two surfaced facts had no sanctioned writer on this path and went to the handoff: test-plan :335's 'retrieve_report is permanently degraded_mode' (a master claim architecture retired 2026-09-06, absent from this chunk's report) and residuals.md :11's 'payload fidelity stays unattainable' (a re-carried entry, which route-resolve may not rewrite)"}]),
    dict(base, kind="friction", id=f"{TS}-b", type="contract.carry-no-owner",
         what="the operator-routed 'refuse an ambiguous run <P-ID>' fix had no clean owner entry; pinned to 'Full-gate regression over the moved surfaces' (the scenario corpus is a surface it sweeps) as the nearest plausible owner rather than halting",
         impact={"extra_reads": 1}),
    dict(base, kind="friction", id=f"{TS}-c", type="contract.no-sanctioned-channel",
         what="test-plan :335 'retrieve_report is permanently degraded_mode' (a stale master universal outside this chunk's report) and residuals.md :11 'payload fidelity stays unattainable' (a re-carried residual) are both now contradicted, but neither has a sanctioned writer on this wrap's path; surfaced in the handoff",
         impact={}),
    dict(base, kind="friction", id=f"{TS}-d", type="contract.grammar-irregularity",
         what="INDETERMINATE: working-route.md:48 — after ONE space — freight or prose, no structural split: 'BLOCKED-ON:'",
         impact={}, artifacts=["conductor-0.3.0/working-route.md:48"], evidence=TRAIL),
    dict(base, kind="friction", id=f"{TS}-e", type="contract.grammar-irregularity",
         what="UNPARSED: working-route.md:57 — separator-boundary token outside the letters' introducer shape: 'CARRY 2: `npm run knip…'",
         impact={}, artifacts=["conductor-0.3.0/working-route.md:57"], evidence=TRAIL),
    dict(base, kind="friction", id=f"{TS}-f", type="contract.grammar-irregularity",
         what="INDETERMINATE: working-route.md:46 — after ONE space — freight or prose, no structural split: 'PREFLIGHT:' (this wrap's own new CONTEXT prose, an ALL-CAPS word before a colon; rephrased the same pass)",
         impact={"retries": 1}, artifacts=["conductor-0.3.0/working-route.md:46"], evidence=TRAIL),
]

R = re.compile(r"^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$")
bad = [r for r in records if not R.fullmatch(str(r.get("ts"))) or str(r.get("id", "")).rsplit("-", 1)[0] != r.get("ts")]
if bad:
    raise SystemExit(f"malformed ts/id - nothing written: {bad}")
lines = [json.dumps(r, ensure_ascii=False) for r in records]
for l in lines:
    json.loads(l)
with open(f"{ROOT}/.andromeda/friction-log.ndjson", "a", encoding="utf-8", newline="") as f:
    for l in lines:
        f.write(l + "\n")
print(f"appended {len(lines)} records")
