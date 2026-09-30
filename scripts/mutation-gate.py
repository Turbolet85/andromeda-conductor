"""Mutation gate — run a unit's mutation tier and gate on the TALLY, never the exit code.

`cargo mutants` exits non-zero for surviving/timeout CLASSES and exits 0 on `Found 0 mutants to test`
(a no-op, never a pass), so its exit carries no verdict in either direction
(`.claude/rules/testing.md` 2026-08-20 / 2026-09-03). This reads `mutants.out/` instead and grades EVERY
tally the verdict rests on:
  - `missed.txt` and `timeout.txt`, each compared as a coordinate-free multiset against the unit's rows of
    that `tally` class in `scripts/mutation-roster.toml` (a timeout is detection by hang, rostered so a
    caught -> timeout move is graded rather than silent);
  - each of the four tally files' parsed line count held to the tool's own count in `outcomes.json`;
  - `missed + caught + timeout + unviable` held to `total_mutants`, so a truncated tally, an unparsed line
    or a mutant absent from every tally cannot pass unseen. A missing tally file fails closed.
`unviable` is counted but never rostered: it moves on host link contention alone.

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

`selftest` proves the grading with no mutation run: it grades each arm of a committed fixture tree
(`scripts/fixtures/mutation-gate/` by default — `arms.toml` names each arm's expected verdict, finding and
finding count) and validates the real roster's schema. It spawns nothing and writes nothing.

Usage: python -X utf8 scripts/mutation-gate.py <unit> | selftest [--fixtures DIR]
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
from typing import NamedTuple

ROOT = Path(__file__).resolve().parent.parent
ROSTER = ROOT / "scripts" / "mutation-roster.toml"
FIXTURES = Path("scripts/fixtures/mutation-gate")
USAGE = "usage: mutation-gate.py <unit> | selftest [--fixtures DIR]"

ROSTERED = ("missed", "timeout")
CLASSES = ("missed", "caught", "timeout", "unviable")

# `{path}:{line}:{col}: {mutation}` — line and column are captured only to be discarded.
SURVIVOR = re.compile(r"^(?P<file>.+?):\d+:\d+:\s*(?P<mutation>.+?)\s*$")


class Roster(NamedTuple):
    expected: dict[str, Counter[tuple[str, str]]]
    rows: list[dict]
    declaration: dict | None
    total_rows: int


def rel(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def parse_tally(path: Path, notes: list[str]) -> Counter[tuple[str, str]]:
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
            notes.append(f"  UNPARSED: {line}")
            continue
        pairs[(m["file"].replace("\\", "/"), m["mutation"])] += 1
    return pairs


def load_roster(unit: str, roster: Path = ROSTER) -> Roster | str:
    """The unit's expected multiset per rostered tally, or the one FAIL line of an invalid roster.

    A row's `tally` is required and never defaulted: a row the gate cannot place is a roster fault.
    """
    doc = tomllib.loads(roster.read_text(encoding="utf-8"))
    entries = doc.get("entry", [])
    bad = next((e for e in entries if e.get("tally") not in ROSTERED), None)
    if bad is not None:
        return (f"MUTATION GATE {unit}: FAIL — roster row lacks a valid tally: "
                f"{bad.get('file')}: {bad.get('mutation')}")
    rows = [e for e in entries if e["unit"] == unit]
    declaration = next((u for u in doc.get("unit", []) if u["name"] == unit), None)
    expected = {t: Counter((r["file"], r["mutation"]) for r in rows if r["tally"] == t) for t in ROSTERED}
    return Roster(expected, rows, declaration, len(entries))


def complete(out: Path) -> tuple[bool, str, int]:
    """The tool's own completion markers — never a live read, never the lock's absence."""
    outcomes, mutants = out / "outcomes.json", out / "mutants.json"
    if not outcomes.is_file():
        return False, f"no outcomes.json under {rel(out)}", 0
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


def grade(tally: Path, unit: str, roster: Path = ROSTER) -> tuple[bool, list[str]]:
    """Grade a finished tally directory. Returns the verdict and every line to print, verdict last.

    Findings are two-space-indented, one per line; every finding is collected before the verdict.
    """
    loaded = load_roster(unit, roster)
    if isinstance(loaded, str):
        return False, [loaded]

    ok, why, total = complete(tally)
    lines = [f"completion: {why}"]
    if not ok:
        return False, lines + [f"MUTATION GATE {unit}: FAIL — tally not readable"]
    # `Found 0 mutants to test` exits 0 with only a WARN, so an empty population reads as a clean
    # sweep. A mis-declared scope path is exactly how that happens now that scoping exists.
    if total == 0:
        return False, lines + [f"MUTATION GATE {unit}: FAIL — 0 mutants generated; a no-op run is never a pass"]

    counts = json.loads((tally / "outcomes.json").read_text(encoding="utf-8"))
    findings: list[str] = []
    present = {c: (tally / f"{c}.txt").is_file() for c in CLASSES}
    parsed = {c: parse_tally(tally / f"{c}.txt", findings) for c in CLASSES}

    for c in CLASSES:
        if not present[c]:
            findings.append(f"  TALLY MISSING: {c}.txt")
    for c in CLASSES:
        n = sum(parsed[c].values())
        if present[c] and n != counts.get(c):
            findings.append(f"  COUNT MISMATCH {c}: {c}.txt has {n}, outcomes.json says {counts.get(c)}")
    summed = sum(counts.get(c) or 0 for c in CLASSES)
    if summed != total:
        findings.append(f"  NOT CONSERVED: missed+caught+timeout+unviable = {summed}, total_mutants {total}")

    # An absent tally is already a finding; comparing it to its roster would restate that one fault per row.
    expected = loaded.expected
    if present["missed"]:
        for (f, m), n in sorted((parsed["missed"] - expected["missed"]).items()):
            findings.append(f"  SURVIVED, not in roster (x{n}): {f}: {m}")
        for (f, m), n in sorted((expected["missed"] - parsed["missed"]).items()):
            findings.append(f"  ROSTERED, did not survive (x{n}): {f}: {m}")
    if present["timeout"]:
        for (f, m), n in sorted((parsed["timeout"] - expected["timeout"]).items()):
            findings.append(f"  TIMED OUT, not in roster (x{n}): {f}: {m}")
        for (f, m), n in sorted((expected["timeout"] - parsed["timeout"]).items()):
            findings.append(f"  ROSTERED, did not time out (x{n}): {f}: {m}")

    got = {c: sum(parsed[c].values()) for c in CLASSES}
    lines.append(
        f"missed {got['missed']} · caught {got['caught']} · timeout {got['timeout']} · "
        f"unviable {got['unviable']} · total {total} · expected missed {sum(expected['missed'].values())} · "
        f"expected timeout {sum(expected['timeout'].values())}"
    )
    lines += findings
    if findings:
        return False, lines + [f"MUTATION GATE {unit}: FAIL"]
    return True, lines + [f"MUTATION GATE {unit}: PASS"]


def selftest(fixtures: Path) -> int:
    manifest_path = ROOT / fixtures / "arms.toml"
    if not manifest_path.is_file():
        print("selftest: the fixtures dir holds no arms.toml")
        return 2
    manifest = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
    root_forms = (str(ROOT), ROOT.as_posix())
    missed: list[str] = []
    for arm in manifest.get("arm", []):
        name = arm["name"]
        ok, lines = grade(ROOT / fixtures / "arms" / name, arm["unit"],
                          ROOT / fixtures / arm.get("roster", "roster.toml"))
        verdict = "PASS" if ok else "FAIL"
        findings = sum(1 for line in lines if line.startswith("  "))
        failed: list[str] = []
        if verdict != arm["verdict"]:
            failed.append(f"verdict {verdict}, expected {arm['verdict']}")
        if not any(arm["contains"] in line for line in lines):
            failed.append(f"no line contains {arm['contains']!r}")
        if findings != arm["findings"]:
            failed.append(f"{findings} findings, expected {arm['findings']}")
        if any(form in line for line in lines for form in root_forms):
            failed.append("a printed line carries the absolute repo path")
        if failed:
            print(f"  {name}: NOT detected — {'; '.join(failed)}")
            missed.append(name)
        else:
            print(f"  {name}: detected")

    real = load_roster("real-roster")
    if isinstance(real, str):
        print(f"  real roster: NOT detected — {real}")
        missed.append("real roster")
    else:
        print(f"  real roster: {real.total_rows} rows, schema ok")

    if missed:
        print(f"selftest: ARM NOT detected — {', '.join(missed)}")
        return 1
    print("selftest: every arm detected")
    return 0


def run_unit(unit: str) -> int:
    roster = load_roster(unit)
    if isinstance(roster, str):
        print(roster)
        return 1
    # Registration is the DECLARATION, never the row count — an all-killed unit legitimately carries
    # zero rows. Rows alone still register, so a roster predating the [[unit]] block keeps working.
    if roster.declaration is None and not roster.rows:
        print(f"MUTATION GATE {unit}: FAIL — unit is not declared in {ROSTER.name} and has no roster rows")
        return 1

    dangling = [r["citation_home"] for r in roster.rows if not (ROOT / r["citation_home"]).exists()]
    if dangling:
        print(f"MUTATION GATE {unit}: FAIL — citation_home does not resolve: {sorted(set(dangling))}")
        return 1

    scope: list[str] = list((roster.declaration or {}).get("files", []))
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
    print(f"output: {rel(output)}")

    cmd = ["cargo", "mutants", "-p", unit, "--test-tool=nextest", "--jobs", "2",
           "--output", str(output)]
    for path in scope:
        cmd += ["-f", path]
    proc = subprocess.run(cmd, cwd=ROOT)
    print(f"cargo mutants exit {proc.returncode} (carries no verdict — the tally decides)")

    ok, lines = grade(output / "mutants.out", unit)
    print("\n".join(lines))
    return 0 if ok else 1


def main() -> int:
    args = sys.argv[1:]
    if args[:1] == ["selftest"]:
        if len(args) == 1:
            return selftest(FIXTURES)
        if len(args) == 3 and args[1] == "--fixtures":
            fixtures = Path(args[2])
            if fixtures.is_absolute() or fixtures.anchor or ".." in fixtures.parts:
                print("selftest: --fixtures must be repo-relative (absolute or .. rejected)")
                return 2
            return selftest(fixtures)
        print(USAGE)
        return 2
    if len(args) != 1:
        print(USAGE)
        return 2
    return run_unit(args[0])


if __name__ == "__main__":
    sys.exit(main())
