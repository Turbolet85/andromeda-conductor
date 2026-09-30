import json, sys

route = open("conductor-0.3.0/working-route.md", encoding="utf-8").read().splitlines()
epoch = [l[4:] for l in route if l.startswith("### Epoch 5")][0]
TS = sys.argv[1]
M = "2026-09-30-the-sr-pass-regrades-on-the-os-input-path"
base = {"v": 1, "ts": TS, "version": "conductor-0.3.0", "epoch": epoch, "chunk": M, "skill": "andromeda-wrap-session"}
rs = [
    dict(base, id=TS + "-a", kind="step", step="report", outcome="ok", counts={"extra_reads": 4},
         consumed=[{"artifact": "conversation", "quality": "ok"},
                   {"artifact": "implement-outcome", "quality": "ok", "note": "implement's P4 report carried every deviation with its measured fix and the census; it stood"},
                   {"artifact": "operator-directive", "quality": "thin", "note": "relay conductor-wrap-osinput-2026-09-30: its 31-id accepted list matched the record exactly; one premise mis-stated — T-01 listed with the reason 'no OS-path browse key' though it is a live-class notRun row by design; its K 'stays a desk task' line was superseded the same wrap by the founder's ruling retiring arm K"},
                   {"artifact": "git-state", "quality": "ok"}, {"artifact": "plan", "quality": "ok"}],
         produced=[{"artifact": "report", "signals": []}],
         problem=[{"nature": "process", "solution": "workaround",
                   "note": "the expected-amendment site sweep first used grep -E with an escaped pipe, which ERE reads literally, and returned false zeros; re-run with -oiE and wc -l"}]),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
