"""Architecture registry check — hold `architecture.md`'s two registry sections under the Read cap, fact-whole.

The Read tool returns a partial page past its token cap, and §Established Decisions and §Occupied Resources grow
at every wrap. This measures each section in UTF-8 bytes against a threshold derived from three stated constants,
and checks a drafted replacement of the two sections against the revision it replaces, arm by arm:

  (a) size        each AFTER section is within the threshold
  (b) labels      every `- **[Label]` / `**Label:**` of BEFORE, byte-identical, in BEFORE order, exactly once
  (c) registry    every leading handle, crate name and port of BEFORE stays in its section, and every QUALIFIERS
                  phrase a BEFORE bullet carries stays on the AFTER bullet of the same handle
  (d) span floor  every BEFORE backtick span survives somewhere: AFTER, the REV sidecar, or the moved history
  (e) hygiene     no drafted file carries a host path
  (f) ledger      exactly one disposition per BEFORE sentence, each verified — `kept` / `moved` verbatim,
                  `in-sidecar` to span grain inside the named marker's entries, `rewritten` to an AFTER anchor

Arm (d) is a floor, never the no-fact-lost proof: most BEFORE spans already occur in the sidecar. Arm (f) carries
the proof, except for what no predicate decides — whether a `rewritten` row restates its sentence faithfully, and
whether a span-less `in-sidecar` row's content is in the entry it names. Those are JUDGMENT rows: counted and
printed, never passed as proven.

A sentence ends at `.` `;` `:` followed by whitespace outside a backtick span, or at a bullet / paragraph break.
The splitter need only be deterministic: `ledger-init` and arm (f) share it, so they agree on the sentence set.

Every set and size is derived from the revision at run time; the threshold is the only number.

Usage:
  python -X utf8 scripts/arch-registry-check.py measure (--rev REV | --file PATH)
  python -X utf8 scripts/arch-registry-check.py status --drafts DIR
  python -X utf8 scripts/arch-registry-check.py check --before-rev REV --drafts DIR
  python -X utf8 scripts/arch-registry-check.py selftest --before-rev REV --drafts DIR
  python -X utf8 scripts/arch-registry-check.py ledger-init --before-rev REV --out PATH
  python -X utf8 scripts/arch-registry-check.py ledger-show --before-rev REV --drafts DIR [--judgment]

Exit: 0 pass / within target · 1 findings / OVER target · 2 harness fault (a path-free reason, never a PASS).
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
import tomllib
from collections import Counter
from dataclasses import dataclass, field, replace
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ARCH = ".andromeda/architecture.md"
SIDECAR = ".andromeda/architecture-amendments.md"
SECTIONS = ("Established Decisions", "Occupied Resources")
DRAFT_FILES = {"Established Decisions": "established-decisions.md", "Occupied Resources": "occupied-resources.md"}
MOVED_FILE = "moved-history.md"
LEDGER_FILE = "disposition-ledger.toml"

READ_CAP_TOKENS = 25_000
# Measured on architecture.md itself: 148 508 B over the Read tool's own 58 443-token count.
BYTES_PER_TOKEN = 2.541
TARGET_FRACTION = 0.60
THRESHOLD = round(READ_CAP_TOKENS * BYTES_PER_TOKEN * TARGET_FRACTION)

QUALIFIERS = (
    "not in the reserved namespace",
    "dev-only",
    "CI-JOB-SCOPED",
    "No inbound listener",
    "No `DATABASE_URL`",
)
DISPOSITIONS = ("kept", "moved", "in-sidecar", "rewritten")
MIN_ANCHOR = 24
CRATE_LABEL = "Crate names (workspace members):"

REV = re.compile(r"^[A-Za-z0-9_.~^/-]+$")
MARKER_ENTRY = re.compile(r"^## (\d{4}-\d{2}-\d{2}-[a-z0-9-]+) — ")
DECISION_LABEL = re.compile(r"^- \*\*(\[[^\]\n]+\])", re.M)
REGISTRY_LABEL = re.compile(r"^\*\*([^*\n]+:)\*\*", re.M)
SPAN = re.compile(r"`([^`\n]+)`")
CRATE = re.compile(r"^conductor-[a-z]+$")
HOST_PORT = re.compile(r"\b(?:127\.0\.0\.1|localhost):\d{4,5}\b")
BARE_PORT = re.compile(r"(?<![^\s`(])(:\d{4,5})\b")
LIST_MARKER = re.compile(r"^\s*(?:[-*]|\d+\.)\s+")
# A drive-letter path (never a `scheme://`), or a POSIX home root.
HOST_PATH = re.compile(r"\b[A-Za-z]:[\\/](?!/)|/Users/|/home/")


class Fault(Exception):
    """A harness fault: exit 2 with a path-free reason, never a verdict."""


@dataclass(frozen=True)
class Finding:
    arm: str
    section: str
    name: str


@dataclass
class Inputs:
    before: str
    sidecar: str
    after: dict[str, str]
    moved: str
    drafted: dict[str, str]
    ledger: list[dict[str, str]] = field(default_factory=list)


def norm(text: str) -> str:
    return " ".join(text.split())


def git_show(rev: str, path: str, label: str) -> str:
    if not REV.fullmatch(rev):
        raise Fault("the revision is not a plain ref")
    proc = subprocess.run(["git", "show", f"{rev}:{path}"], cwd=ROOT, capture_output=True, check=False)
    if proc.returncode != 0:
        raise Fault(f"revision {rev} does not carry {label}")
    return proc.stdout.decode("utf-8")


def repo_path(arg: str) -> Path:
    candidate = Path(arg)
    if candidate.is_absolute() or ":" in arg or arg.startswith(("/", "\\")) or ".." in candidate.parts:
        raise Fault("a path argument must be repo-relative, with no `..` component")
    return ROOT / candidate


def read_file(path: Path, label: str) -> str:
    if not path.is_file():
        raise Fault(f"{label} is absent")
    return path.read_text(encoding="utf-8")


def section_span(text: str, name: str) -> tuple[int, int]:
    m = re.search(rf"^## {re.escape(name)}\n.*?(?=^## |\Z)", text, re.S | re.M)
    if not m:
        raise Fault(f"section §{name} is absent")
    return m.start(), m.end()


def section(text: str, name: str) -> str:
    start, end = section_span(text, name)
    return text[start:end]


def paragraphs(body: str) -> list[str]:
    out: list[str] = []
    cur: list[str] = []
    for line in body.split("\n"):
        if not line.strip():
            if cur:
                out.append(" ".join(cur))
                cur = []
            continue
        marker = LIST_MARKER.match(line)
        if marker:
            if cur:
                out.append(" ".join(cur))
            cur = [line[marker.end():]]
        else:
            cur.append(line.strip())
    if cur:
        out.append(" ".join(cur))
    return out


def split_sentences(par: str) -> list[str]:
    pieces: list[str] = []
    start, code = 0, False
    for i, ch in enumerate(par):
        if ch == "`":
            code = not code
        elif not code and ch in ".;:" and (i + 1 == len(par) or par[i + 1].isspace()):
            pieces.append(par[start : i + 1])
            start = i + 1
    pieces.append(par[start:])
    return [s for s in (norm(p) for p in pieces) if any(c.isalnum() for c in s)]


def sentence_id(name: str, text: str) -> str:
    return hashlib.blake2b(f"{name}\0{text}".encode("utf-8")).hexdigest()[:12]


def sentences(sec_text: str, name: str) -> dict[str, str]:
    """id → normalized sentence, in document order; the heading line is not a sentence."""
    body = sec_text.split("\n", 1)[1] if "\n" in sec_text else ""
    out: dict[str, str] = {}
    for par in paragraphs(body):
        for text in split_sentences(par):
            out.setdefault(sentence_id(name, text), text)
    return out


def labels(sec_text: str) -> list[str]:
    return DECISION_LABEL.findall(sec_text) + REGISTRY_LABEL.findall(sec_text)


def ports(text: str) -> set[str]:
    return set(HOST_PORT.findall(text)) | set(BARE_PORT.findall(text))


def crate_names(before: str) -> set[str]:
    for line in section(before, "Occupied Resources").split("\n"):
        if line.startswith(f"**{CRATE_LABEL}**"):
            return {s for s in SPAN.findall(line) if CRATE.fullmatch(s)}
    raise Fault(f"the `{CRATE_LABEL}` registry is absent")


def bullets(sec_text: str) -> dict[tuple[str, str | None], str]:
    """(sub-registry, leading handle) → the bullets' joined text. A span-less bullet keys by sub-registry alone."""
    out: dict[tuple[str, str | None], str] = {}
    current = ""
    for line in sec_text.split("\n"):
        label = REGISTRY_LABEL.match(line)
        if label:
            current = label.group(1)
        elif line.startswith("- "):
            spans = SPAN.findall(line)
            key = (current, spans[0] if spans else None)
            out[key] = out.get(key, "") + "\n" + line
    return out


def registry_names(before: str, name: str) -> set[str]:
    text = section(before, name)
    names = {c for c in crate_names(before) if re.search(rf"(?<![\w-]){re.escape(c)}(?![\w-])", text)}
    names |= ports(text)
    if name == "Occupied Resources":
        names |= {handle for (_, handle) in bullets(text) if handle is not None}
    return names


def sidecar_entries(sidecar: str) -> dict[str, str]:
    """marker → the union of the text of every entry headed `## {marker} — …`."""
    out: dict[str, str] = {}
    marker: str | None = None
    for line in sidecar.split("\n"):
        if line.startswith("## "):
            m = MARKER_ENTRY.match(line)
            marker = m.group(1) if m else None
        if marker is not None:
            out[marker] = out.get(marker, "") + line + "\n"
    return out


def evaluate(inp: Inputs) -> tuple[list[Finding], Counter[str]]:
    """Every finding of the six arms, plus the judgment tally (keys `rewritten` / `spanless`)."""
    findings: list[Finding] = []
    judgment: Counter[str] = Counter()
    before = {name: section(inp.before, name) for name in SECTIONS}
    after_norm = {name: norm(inp.after[name]) for name in SECTIONS}

    for name in SECTIONS:
        size = len(inp.after[name].encode("utf-8"))
        if size > THRESHOLD:
            findings.append(Finding("a size", name, f"{size} B over {THRESHOLD} B"))

    for name in SECTIONS:
        want, got = labels(before[name]), labels(inp.after[name])
        counts = Counter(got)
        for label in want:
            if counts[label] != 1:
                findings.append(Finding("b labels", name, f"{label} present {counts[label]}x"))
        if [x for x in got if x in want] != want:
            findings.append(Finding("b labels", name, "label order differs from BEFORE"))

    for name in SECTIONS:
        for reg in sorted(registry_names(inp.before, name)):
            if reg not in inp.after[name]:
                findings.append(Finding("c registry", name, reg))
    was, now = bullets(before["Occupied Resources"]), bullets(inp.after["Occupied Resources"])
    for key, text in was.items():
        for qual in QUALIFIERS:
            if qual in text and qual not in now.get(key, ""):
                findings.append(Finding("c registry", "Occupied Resources", f"{qual!r} off `{key[1] or key[0]}`"))

    pool = inp.after["Established Decisions"] + inp.after["Occupied Resources"] + inp.sidecar + inp.moved
    for name in SECTIONS:
        for span in sorted(set(SPAN.findall(before[name]))):
            if span not in pool:
                findings.append(Finding("d span floor", name, f"`{span}`"))

    for draft, text in inp.drafted.items():
        if HOST_PATH.search(text):
            findings.append(Finding("e hygiene", draft, "host path"))

    entries = sidecar_entries(inp.sidecar)
    moved_norm = norm(inp.moved)
    reg_by_section = {name: registry_names(inp.before, name) for name in SECTIONS}
    expected: dict[str, tuple[str, str]] = {}
    for name in SECTIONS:
        for sid, text in sentences(before[name], name).items():
            expected[sid] = (name, text)
    seen = Counter(str(row.get("id", "")) for row in inp.ledger)
    for sid in sorted(set(expected) - set(seen)):
        findings.append(Finding("f ledger", expected[sid][0], f"{sid} has no row"))
    for sid, n in sorted(seen.items()):
        if sid not in expected:
            findings.append(Finding("f ledger", "-", f"{sid} is no BEFORE sentence"))
        elif n > 1:
            findings.append(Finding("f ledger", expected[sid][0], f"{sid} has {n} rows"))
    for row in inp.ledger:
        sid = str(row.get("id", ""))
        if sid not in expected or seen[sid] > 1:
            continue
        name, text = expected[sid]
        disp = row.get("disposition", "")
        spans = SPAN.findall(text)
        if disp not in DISPOSITIONS:
            findings.append(Finding("f ledger", name, f"{sid} disposition {disp!r}"))
        elif disp == "kept":
            if text not in after_norm[name]:
                findings.append(Finding("f ledger", name, f"{sid} kept but absent from AFTER"))
        elif disp == "moved":
            if text not in moved_norm:
                findings.append(Finding("f ledger", name, f"{sid} moved but absent from the moved history"))
        elif disp == "in-sidecar":
            marker = row.get("marker", "")
            if marker not in entries:
                findings.append(Finding("f ledger", name, f"{sid} names no sidecar entry {marker!r}"))
            else:
                lost = [s for s in spans if s not in entries[marker]]
                if lost:
                    findings.append(Finding("f ledger", name, f"{sid} span `{lost[0]}` not in {marker}"))
                elif not spans:
                    judgment["spanless"] += 1
        else:
            anchor = norm(row.get("anchor", ""))
            if len(anchor) < MIN_ANCHOR:
                findings.append(Finding("f ledger", name, f"{sid} anchor under {MIN_ANCHOR} chars"))
            elif anchor not in after_norm[name]:
                findings.append(Finding("f ledger", name, f"{sid} anchor absent from AFTER"))
            else:
                lost = [s for s in spans if s in reg_by_section[name] and s not in inp.after[name]]
                if lost:
                    findings.append(Finding("f ledger", name, f"{sid} rewrite drops `{lost[0]}`"))
                else:
                    judgment["rewritten"] += 1
    return findings, judgment


def load(before_rev: str, drafts: str) -> Inputs:
    folder = repo_path(drafts)
    if not folder.is_dir():
        raise Fault("the drafts directory is absent")
    before = git_show(before_rev, ARCH, "the architecture master")
    sidecar = git_show(before_rev, SIDECAR, "the architecture sidecar")
    after: dict[str, str] = {}
    drafted: dict[str, str] = {}
    for name, fname in DRAFT_FILES.items():
        text = read_file(folder / fname, f"draft {fname}")
        if not text.startswith(f"## {name}\n"):
            raise Fault(f"draft {fname} does not open with its `## {name}` heading")
        after[name], drafted[fname] = text, text
    moved = read_file(folder / MOVED_FILE, f"draft {MOVED_FILE}")
    drafted[MOVED_FILE] = moved
    raw = read_file(folder / LEDGER_FILE, f"draft {LEDGER_FILE}")
    drafted[LEDGER_FILE] = raw
    try:
        rows = tomllib.loads(raw).get("sentence", [])
    except tomllib.TOMLDecodeError as e:
        raise Fault(f"the ledger does not parse ({e.args[0]})") from None
    return Inputs(before, sidecar, after, moved, drafted, rows)


def assemble(before: str, after: dict[str, str]) -> str:
    text = before
    for name in SECTIONS:
        start, end = section_span(text, name)
        text = text[:start] + after[name] + text[end:]
    return text


def size_line(label: str, size: int) -> str:
    tokens = size / BYTES_PER_TOKEN
    verdict = "within target" if size <= THRESHOLD else "OVER target"
    return f"  {label}: {size} B · ~{tokens:.0f} tokens · {tokens / READ_CAP_TOKENS:.1%} of cap · {verdict}"


def cmd_measure(args: argparse.Namespace) -> int:
    text = git_show(args.rev, ARCH, "the architecture master") if args.rev else read_file(repo_path(args.file), "the file")
    over = False
    print(f"threshold {THRESHOLD} B ({TARGET_FRACTION:.0%} of {READ_CAP_TOKENS} tokens at {BYTES_PER_TOKEN} B/token)")
    for name in SECTIONS:
        size = len(section(text, name).encode("utf-8"))
        over |= size > THRESHOLD
        print(size_line(f"§{name}", size))
    print(f"registries: {'OVER' if over else 'within'} target")
    return 1 if over else 0


def cmd_status(args: argparse.Namespace) -> int:
    folder = repo_path(args.drafts)
    master = read_file(ROOT / ARCH, "the architecture master")
    for name, fname in DRAFT_FILES.items():
        draft = read_file(folder / fname, f"draft {fname}")
        current = section(master, name)
        applied = "yes" if current == draft else "no"
        print(f"§{name}: master {len(current.encode('utf-8'))} B · draft {len(draft.encode('utf-8'))} B · applied: {applied}")
    return 0


def report(findings: list[Finding], judgment: Counter[str], after: dict[str, str]) -> int:
    for name in SECTIONS:
        print(size_line(f"AFTER §{name}", len(after[name].encode("utf-8"))))
    for f in findings:
        print(f"  [{f.arm}] §{f.section}: {f.name}")
    total = judgment["rewritten"] + judgment["spanless"]
    print(f"judgment rows: {total} (rewritten {judgment['rewritten']} · span-less in-sidecar {judgment['spanless']})")
    if findings:
        print(f"arch-registry-check: FAIL ({len(findings)} findings)")
        return 1
    print("arch-registry-check: PASS")
    return 0


def cmd_check(args: argparse.Namespace) -> int:
    inp = load(args.before_rev, args.drafts)
    findings, judgment = evaluate(inp)
    return report(findings, judgment, inp.after)


def mutations(inp: Inputs) -> list[tuple[str, Inputs]]:
    """One in-memory mutation per arm (three for the ledger), each over the REAL drafts and ledger."""
    ed, orr = "Established Decisions", "Occupied Resources"
    out: list[tuple[str, Inputs]] = []
    out.append(("a size", replace(inp, after={**inp.after, ed: inp.after[ed] + "x" * (THRESHOLD + 1)})))
    first = labels(section(inp.before, ed))[0]
    out.append(("b labels", replace(inp, after={**inp.after, ed: inp.after[ed].replace(first, "[Renamed]", 1)})))
    handle = sorted(h for (_, h) in bullets(section(inp.before, orr)) if h is not None and len(h) > 8)[0]
    out.append(("c registry", replace(inp, after={**inp.after, orr: inp.after[orr].replace(handle, "")})))
    solo = sorted(
        s for name in SECTIONS for s in SPAN.findall(section(inp.before, name))
        if s not in inp.sidecar and len(s) > 3
    )[0]
    out.append((
        "d span floor",
        replace(inp, after={k: v.replace(solo, "") for k, v in inp.after.items()}, moved=inp.moved.replace(solo, "")),
    ))
    out.append(("e hygiene", replace(inp, drafted={**inp.drafted, MOVED_FILE: inp.moved + "\nC:\\Users\\x\n"})))
    out.append(("f ledger", replace(inp, ledger=inp.ledger[1:])))
    before_ids = {}
    for name in SECTIONS:
        before_ids.update(sentences(section(inp.before, name), name))
    absent = next(
        i for i, row in enumerate(inp.ledger)
        if row.get("disposition") != "kept"
        and before_ids.get(str(row.get("id")), "") not in norm(inp.after[str(row.get("section"))])
    )
    flipped = [dict(r) for r in inp.ledger]
    flipped[absent]["disposition"] = "kept"
    out.append(("f ledger", replace(inp, ledger=flipped)))
    entries = sidecar_entries(inp.sidecar)
    repoint = [dict(r) for r in inp.ledger]
    target = next(
        (i for i, r in enumerate(repoint) if r.get("disposition") == "in-sidecar"
         and SPAN.findall(before_ids.get(str(r.get("id")), ""))), None,
    )
    if target is None:
        sid, text = next((k, v) for k, v in before_ids.items() if SPAN.findall(v))
        repoint = [r for r in repoint if r.get("id") != sid] + [{"id": sid, "disposition": "in-sidecar"}]
        target = len(repoint) - 1
    span = SPAN.findall(before_ids[str(repoint[target]["id"])])[0]
    repoint[target]["marker"] = next(m for m, body in entries.items() if span not in body)
    out.append(("f ledger", replace(inp, ledger=repoint)))
    return out


def cmd_selftest(args: argparse.Namespace) -> int:
    inp = load(args.before_rev, args.drafts)
    baseline = set(evaluate(inp)[0])
    missed: list[str] = []
    for arm, mutated in mutations(inp):
        new = set(evaluate(mutated)[0]) - baseline
        hit = any(f.arm == arm for f in new)
        print(f"  {arm}: {'detected' if hit else 'NOT detected'} ({len(new)} new findings)")
        if not hit:
            missed.append(arm)
    if missed:
        print(f"selftest: ARM NOT detected — {', '.join(missed)}")
        return 1
    print("selftest: every arm detected")
    return 0


def cmd_ledger_init(args: argparse.Namespace) -> int:
    out = repo_path(args.out)
    if out.exists():
        raise Fault("the ledger already exists — ledger-init never overwrites")
    before = git_show(args.before_rev, ARCH, "the architecture master")
    lines = [f"# Disposition ledger — one row per BEFORE sentence of the two registry sections at {args.before_rev}.",
             "# disposition: kept | moved | in-sidecar (+ marker) | rewritten (+ anchor)", ""]
    for name in SECTIONS:
        for sid, text in sentences(section(before, name), name).items():
            lines += ["[[sentence]]", f'id = "{sid}"', f"section = {json.dumps(name)}",
                      f"head = {json.dumps(text[:72], ensure_ascii=False)}", 'disposition = ""', ""]
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text("\n".join(lines), encoding="utf-8", newline="\n")
    print(f"ledger-init: {sum(1 for x in lines if x == '[[sentence]]')} rows")
    return 0


def cmd_ledger_show(args: argparse.Namespace) -> int:
    inp = load(args.before_rev, args.drafts)
    text_of: dict[str, str] = {}
    for name in SECTIONS:
        text_of.update(sentences(section(inp.before, name), name))
    headings: dict[str, list[str]] = {}
    for line in inp.sidecar.split("\n"):
        m = MARKER_ENTRY.match(line)
        if m:
            headings.setdefault(m.group(1), []).append(line[3:])
    tally: Counter[str] = Counter()
    shown = 0
    for row in inp.ledger:
        disp, sid = row.get("disposition", ""), str(row.get("id", ""))
        text = text_of.get(sid, "<not a BEFORE sentence>")
        tally[disp] += 1
        spanless = disp == "in-sidecar" and not SPAN.findall(text)
        if args.judgment and not (disp == "rewritten" or spanless):
            continue
        shown += 1
        if disp == "rewritten":
            target = f"AFTER anchor: {row.get('anchor', '')}"
        elif disp == "in-sidecar":
            target = "sidecar: " + " | ".join(headings.get(row.get("marker", ""), ["<no such entry>"]))
        else:
            target = disp
        print(f"[{disp}{' span-less' if spanless else ''}] §{row.get('section', '')} {sid}\n  BEFORE: {text}\n  → {target}")
    print(f"ledger: {len(inp.ledger)} rows · shown {shown} · " + " · ".join(f"{k} {tally[k]}" for k in DISPOSITIONS))
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(prog="arch-registry-check")
    sub = parser.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("measure")
    g = p.add_mutually_exclusive_group(required=True)
    g.add_argument("--rev")
    g.add_argument("--file")
    sub.add_parser("status").add_argument("--drafts", required=True)
    for name in ("check", "selftest", "ledger-show"):
        p = sub.add_parser(name)
        p.add_argument("--before-rev", required=True)
        p.add_argument("--drafts", required=True)
        if name == "ledger-show":
            p.add_argument("--judgment", action="store_true")
    p = sub.add_parser("ledger-init")
    p.add_argument("--before-rev", required=True)
    p.add_argument("--out", required=True)
    args = parser.parse_args()
    handler = {
        "measure": cmd_measure, "status": cmd_status, "check": cmd_check, "selftest": cmd_selftest,
        "ledger-init": cmd_ledger_init, "ledger-show": cmd_ledger_show,
    }[args.cmd]
    try:
        return handler(args)
    except Fault as e:
        print(f"arch-registry-check: HARNESS FAULT — {e}")
        return 2


if __name__ == "__main__":
    sys.exit(main())
