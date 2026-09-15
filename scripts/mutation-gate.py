"""Mutation gate — run a unit's mutation tier and gate on the TALLY, never the exit code.

`cargo mutants` exits non-zero for surviving/timeout CLASSES and exits 0 on `Found 0 mutants to test`
(a no-op, never a pass), so its exit carries no verdict in either direction
(`.claude/rules/testing.md` 2026-08-20 / 2026-09-03). This reads `mutants.out/` instead and compares
`missed.txt` against `scripts/mutation-roster.toml`'s expected set for the unit.

The tally is read ONLY once the invocation is complete by the tool's own markers: `outcomes.json`'s
top-level `end_time` is set AND its `total_mutants` equals the length of `mutants.json` — the list THAT
invocation was given. cargo-mutants rewrites `outcomes.json` incrementally, so a live read can score a
unit 100.0 that finishes at 96.81.

Output goes to a FRESH `target/mutation-gate/{unit}-{stamp}/` generated at invocation: the tool nests its
real output at `{output}/mutants.out/` and rotates a pre-existing dir to `.old`, so a reused path is stale
by construction.

A unit is REGISTERED by its `[[unit]]` declaration in the roster, never by its row count: a registered
unit whose accepted-deliberate set is legitimately empty — every survivor killed — must be able to pass
with an empty `missed.txt`. The declaration may carry `files`, scoping the tier to those paths via `-f`
(workspace-root-relative, per `.claude/rules/testing.md`); an empty or absent `files` runs the crate whole.

Usage: python -X utf8 scripts/mutation-gate.py <unit>
"""

from __future__ import annotations

import json
import re
import subprocess
import sys
import tomllib
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ROSTER = ROOT / "scripts" / "mutation-roster.toml"

# `{path}:{line}:{col}: {mutation}` — line and column are captured only to be discarded.
SURVIVOR = re.compile(r"^(?P<file>.+?):\d+:\d+:\s*(?P<mutation>.+?)\s*$")


def parse_tally(path: Path) -> Counter[tuple[str, str]]:
    """A multiset of (file, mutation). Identical pairs at different coordinates are distinct rows."""
    if not path.is_file():
        return Counter()
    pairs: Counter[tuple[str, str]] = Counter()
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line:
            continue
        m = SURVIVOR.match(line)
        if not m:
            print(f"  UNPARSED: {line}")
            continue
        pairs[(m["file"].replace("\\", "/"), m["mutation"])] += 1
    return pairs


def expected_for(unit: str) -> tuple[Counter[tuple[str, str]], list[dict], dict | None]:
    """The unit's accepted-deliberate multiset, its rows, and its `[[unit]]` declaration if any."""
    roster = tomllib.loads(ROSTER.read_text(encoding="utf-8"))
    rows = [e for e in roster.get("entry", []) if e["unit"] == unit]
    declaration = next((u for u in roster.get("unit", []) if u["name"] == unit), None)
    return Counter((r["file"], r["mutation"]) for r in rows), rows, declaration


def complete(out: Path) -> tuple[bool, str, int]:
    """The tool's own completion markers — never a live read, never the lock's absence."""
    outcomes, mutants = out / "outcomes.json", out / "mutants.json"
    if not outcomes.is_file():
        return False, f"no outcomes.json under {out}", 0
    data = json.loads(outcomes.read_text(encoding="utf-8"))
    if not data.get("end_time"):
        return False, "outcomes.json end_time unset — the invocation is still running", 0
    total = data.get("total_mutants") or 0
    if mutants.is_file():
        planned = len(json.loads(mutants.read_text(encoding="utf-8")))
        if total != planned:
            return False, f"total_mutants {total} != len(mutants.json) {planned} — partial run", total
    else:
        return True, f"end_time set; mutants.json absent, total_mutants {total} (disclosed)", total
    return True, f"end_time set; total_mutants {total} == len(mutants.json)", total


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: mutation-gate.py <unit>")
        return 2
    unit = sys.argv[1]
    expected, rows, declaration = expected_for(unit)
    # Registration is the DECLARATION, never the row count — an all-killed unit legitimately carries
    # zero rows. Rows alone still register, so a roster predating the [[unit]] block keeps working.
    if declaration is None and not rows:
        print(f"MUTATION GATE {unit}: FAIL — unit is not declared in {ROSTER.name} and has no roster rows")
        return 1

    dangling = [r["citation_home"] for r in rows if not (ROOT / r["citation_home"]).exists()]
    if dangling:
        print(f"MUTATION GATE {unit}: FAIL — citation_home does not resolve: {sorted(set(dangling))}")
        return 1

    scope: list[str] = list((declaration or {}).get("files", []))
    absent = [f for f in scope if not (ROOT / f).is_file()]
    if absent:
        print(f"MUTATION GATE {unit}: FAIL — declared scope file does not exist: {sorted(absent)}")
        return 1
    if scope:
        print(f"scope: {len(scope)} file(s) — {', '.join(scope)}")

    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    output = ROOT / "target" / "mutation-gate" / f"{unit}-{stamp}"
    # cargo-mutants creates the output LEAF itself and errors if the parent chain is missing; the
    # leaf must stay fresh (a pre-existing one is stale by construction and rotates to `.old`).
    output.parent.mkdir(parents=True, exist_ok=True)
    print(f"output: {output.relative_to(ROOT).as_posix()}")

    cmd = ["cargo", "mutants", "-p", unit, "--test-tool=nextest", "--jobs", "2",
           "--output", str(output)]
    for path in scope:
        cmd += ["-f", path]
    proc = subprocess.run(cmd, cwd=ROOT)
    print(f"cargo mutants exit {proc.returncode} (carries no verdict — the tally decides)")

    out = output / "mutants.out"
    ok, why, total = complete(out)
    print(f"completion: {why}")
    if not ok:
        print(f"MUTATION GATE {unit}: FAIL — tally not readable")
        return 1
    # `Found 0 mutants to test` exits 0 with only a WARN, so an empty population reads as a clean
    # sweep. A mis-declared scope path is exactly how that happens now that scoping exists.
    if total == 0:
        print(f"MUTATION GATE {unit}: FAIL — 0 mutants generated; a no-op run is never a pass")
        return 1

    missed = parse_tally(out / "missed.txt")
    caught = parse_tally(out / "caught.txt")

    unexpected = missed - expected
    absent = expected - missed
    print(f"missed {sum(missed.values())} · caught {sum(caught.values())} · expected {sum(expected.values())}")

    if unexpected or absent:
        for (f, m), n in sorted(unexpected.items()):
            print(f"  SURVIVED, not in roster (x{n}): {f}: {m}")
        for (f, m), n in sorted(absent.items()):
            print(f"  ROSTERED, did not survive (x{n}): {f}: {m}")
        print(f"MUTATION GATE {unit}: FAIL")
        return 1

    print(f"MUTATION GATE {unit}: PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
