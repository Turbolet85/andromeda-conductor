import sys

P = "D:/dev/projects/conductor/conductor-0.3.0/working-route.md"
ANCHOR = "Full-gate regression over the moved surfaces"
CARRY = (
    "  CARRY (2026-09-30-the-screen-reader-content-findings-fixed; operator wrap directive): SR row E0-10 is graded "
    "subject-absent by its row (\"the seeded fixture records no run_envelope row\"), yet the ENVIRONMENT-SUSPECT banner "
    "was HEARD in E0-07's window in every sr-empty session of 2026-09-30 (measured at "
    "`conductor-0.3.0/chunks/2026-09-30-the-screen-reader-content-findings-fixed/evidence/nvda-pass.json`; the "
    "operator review graded E0-10 a FINDING) — owed: grade E0-10 against its expected content (the banner's label as "
    "text, never a colour) instead of its absent reason. hypothesis: `runs/e2e-fixture`, which sr-empty reads, now "
    "carries the envelope row the `--e2e` arm's seed persists — unmeasured; confirm from the seeded dir before choosing "
    "between re-tokening the row and seeding sr-empty without it."
)

with open(P, encoding="utf-8", newline="") as f:
    lines = f.read().split("\n")
hits = [i for i, l in enumerate(lines) if l.startswith(ANCHOR)]
if len(hits) != 1:
    sys.exit(f"anchor matched {len(hits)} lines")
i = hits[0]
if "SR row E0-10" in lines[i]:
    sys.exit("already carried")
before = list(lines)
lines[i] = lines[i].rstrip("\r") + CARRY + ("\r" if lines[i].endswith("\r") else "")
out = "\n".join(lines)
with open(P, "w", encoding="utf-8", newline="") as f:
    f.write(out)
with open(P, encoding="utf-8", newline="") as f:
    back = f.read().split("\n")
assert len(back) == len(before), "line count moved"
assert all(back[j] == before[j] for j in range(len(before)) if j != i), "another line moved"
assert back[i].endswith(CARRY.rstrip()) or CARRY in back[i]
print(f"carried on line {i + 1}; {len(back)} lines, all others byte-identical")
