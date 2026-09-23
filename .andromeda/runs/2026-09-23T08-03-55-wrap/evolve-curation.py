import json
import re

ROOT = "D:/dev/projects/conductor"
TS = "2026-09-23T09:48:36Z"
EPOCH = open(f"{ROOT}/conductor-0.3.0/working-route.md", encoding="utf-8").read().split("\n")[40][4:]
assert EPOCH.startswith("Epoch 4"), EPOCH
base = {"v": 1, "ts": TS, "version": "conductor-0.3.0", "epoch": EPOCH,
        "chunk": "2026-09-22-interpretation-proven-live", "skill": "andromeda-wrap-session", "step": "curation"}

records = [
    dict(base, kind="step", id=f"{TS}-a", outcome="ok",
         counts={"dialogue_rounds": 0},
         consumed=[{"artifact": "conversation", "quality": "ok"},
                   {"artifact": "report", "quality": "ok", "note": "Decisions & corrections supplied both new entries and three of the five corrections"}],
         produced=[{"artifact": "conversation", "signals": ["corrections-5", "tier2-new-2"],
                    "note": "5 cap-exempt in-place corrections (two fingerprint read-back clauses, ~110 s, the quoted-heredoc backslash clause, the CLAUDE.md and Tier-3 fingerprint clauses) + 2 new Tier-2 entries"}],
         problem=[
             {"nature": "process", "solution": "workaround", "note": "the plan listed three Tier-2 corrections under curation; two sit in GENERATED rule bodies (verification-harness.md:19, security.md's READ SET sentence), so per the curation guide they were re-derived by the P2 cascade from their amended masters, and only the Session Additions one (verification-harness.md:60) was corrected here"},
             {"nature": "environment", "solution": "workaround", "note": "a backslash-bearing grep spot-check of the host-win32 correction read back 0 through the Bash transport (the very collapse the correction records); verified instead by a Write-tool script file, which found the text intact"}]),
    dict(base, kind="friction", id=f"{TS}-b", type="ambiguity.filter-borderline",
         what="both surviving candidates scored exactly 0.6 before Filter 4's conditional signals; the Dismiss-witness entry passed only on the load-bearing call (does 'Diagnostic-quality cluster off the drift pin' need the fact?), a judgment on an entry with no annotations to read it from",
         impact={}),
    dict(base, kind="friction", id=f"{TS}-c", type="recall.corpus-recurrence",
         what="the cd-persistence entry (host-win32.md Session Additions, 2026-09-08) did not prevent recurrence: a cd inside a compound command re-based later calls in implement and three more times in this wrap",
         impact={"retries": 0},
         artifacts=[".claude/rules/host-win32.md"]),
    dict(base, kind="friction", id=f"{TS}-d", type="recall.corpus-recurrence",
         what="the COMPLETION axis (CLAUDE.md 2026-08-09 entry, 2026-08-31 extension: check your own evidence before accepting an ask's premise) did not prevent A17 reaching validate framed 'code reading only, not live-measured' while committed 2026-09-10 envelopes already measured it",
         impact={"extra_reads": 3},
         artifacts=["CLAUDE.md"]),
    dict(base, kind="friction", id=f"{TS}-e", type="contract.skill-reference-drift",
         what="rules-templates/host-win32.md renders 'The quoted heredoc itself is sound below the cut', falsified for backslash pairs (the Bash tool's transport collapses \\\\ to \\ before bash sees a quoted heredoc, measured on three payloads); corrected in place in the rendered host rule with a dated tag, the template source still carries the false clause",
         impact={},
         artifacts=["rules-templates/host-win32.md", ".claude/rules/host-win32.md"]),
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
