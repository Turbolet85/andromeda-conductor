import json
import re

ROOT = "D:/dev/projects/conductor"
TS = "2026-09-23T09:53:10Z"
OLD = "2026-09-23T09:51:34Z"
EPOCH = open(f"{ROOT}/conductor-0.3.0/working-route.md", encoding="utf-8").read().split("\n")[40][4:]
base = {"v": 1, "ts": TS, "version": "conductor-0.3.0", "epoch": EPOCH,
        "chunk": "2026-09-22-interpretation-proven-live", "skill": "andromeda-wrap-session", "step": "route-resolve"}
note = ("evidence named '.andromeda/runs/2026-09-23T08-03-55-wrap/route-2026-09-22-interpretation-proven-live.json', "
        "which does not exist; route.py ran without --marker, so the trail is 'route-no-marker.json' in the same run dir")
records = [
    dict(base, kind="friction", id=f"{TS}-a", type=None, untyped=True,
         what="three grammar-irregularity records carried an evidence pointer to a trail file that was never written; the verdict lines they quote stand",
         impact={},
         retracts=[{"id": f"{OLD}-d", "scope": "clause", "note": note},
                   {"id": f"{OLD}-e", "scope": "clause", "note": note},
                   {"id": f"{OLD}-f", "scope": "clause", "note": note}]),
]
R = re.compile(r"^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$")
bad = [r for r in records if not R.fullmatch(str(r.get("ts"))) or str(r.get("id", "")).rsplit("-", 1)[0] != r.get("ts")]
if bad:
    raise SystemExit(f"malformed ts/id - nothing written: {bad}")
lines = [json.dumps(r, ensure_ascii=False) for r in records]
with open(f"{ROOT}/.andromeda/friction-log.ndjson", "a", encoding="utf-8", newline="") as f:
    for l in lines:
        json.loads(l)
        f.write(l + "\n")
print(f"appended {len(lines)} record")
