import json
import re

ROOT = "D:/dev/projects/conductor"
TS = "2026-09-23T09:59:23Z"
EPOCH = open(f"{ROOT}/conductor-0.3.0/working-route.md", encoding="utf-8").read().split("\n")[40][4:]
assert EPOCH.startswith("Epoch 4"), EPOCH
base = {"v": 1, "ts": TS, "version": "conductor-0.3.0", "epoch": EPOCH,
        "chunk": "2026-09-22-interpretation-proven-live", "skill": "andromeda-wrap-session", "step": "gates"}

records = [
    dict(base, kind="step", id=f"{TS}-a", outcome="ok",
         counts={"halted": 0, "deferred": 0},
         consumed=[{"artifact": "plan", "quality": "ok"}, {"artifact": "matrix", "quality": "ok"},
                   {"artifact": "master-route", "quality": "ok"}],
         produced=[
             {"artifact": "matrix", "signals": ["unclaim-valve"], "note": "v3-09 claimed with ref null (B4: the ref is written only on Identified) — resolved by the un-claim valve plus defer under plan D1, the drive's measurement and the 2026-09-18 claim-(2) correction in notes; the flip then read claimed 0 — no-op; version 6/11 verified, 1 deferred"},
             {"artifact": "master-route", "signals": ["desc-rewritten"], "note": "route.py flip exit 0, desc rewritten from --desc-file (the outcome falsified the proposal's 'top hypothesis asserted' promise); flip-compaction stripped the one complete line and archived it verbatim"},
             {"artifact": "git-state", "signals": ["pushed"], "note": "commit edc0af8 on build/conductor-0.3.0, add -A staged exactly the expected set (0 secret-shaped paths), tree.db.commit stamped, pushed 74a0872..edc0af8, 0 ahead; code-graph refresh done at Setup (rust 2781n/13372e, ts 807n/1690e)"}],
         problem=[
             {"nature": "environment", "solution": "removed-cause", "note": "light-gate entry 1 (advisory-db currency probe) read red: an untracked placeholder crates/connectrpc/RUSTSEC-0000-0000.md left in the local $CARGO_HOME/advisory-db clone after upstream renamed it to RUSTSEC-2026-0304; moved aside (not deleted), probe clean, full block re-run 19 green"},
             {"nature": "process", "solution": "workaround", "note": "flip-compaction ran as a scratchpad python whole-file rewrite (one read, one write, read-back asserting only the compacted line moved) because route.py has no compaction subcommand; the reference names a single Write"}]),
    dict(base, kind="friction", id=f"{TS}-b", type="tooling.light-gate-red",
         what="entry 1 `git -C \"$CARGO_HOME/advisory-db\" status --porcelain` red at the wrap (42 B: `?? crates/connectrpc/RUSTSEC-0000-0000.md`) where implement's run of the same entry was green hours earlier — an upstream add-then-rename fetched into the existing clone left the placeholder untracked; cargo audit itself stayed green (an extra untracked advisory can only add findings)",
         impact={"retries": 1},
         artifacts=["$CARGO_HOME/advisory-db"],
         evidence="wrap-2026-09-23T08-03-55 gate logs (1.log); re-run all green"),
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
