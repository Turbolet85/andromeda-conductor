"""Formatting-only invariant for this chunk's .rs diff.

A rustfmt pass may reorder and rewrap tokens; it may never add, drop or alter one.
For each changed file, the multiset of identifiers, string literals and numerals on
the added side must equal the multiset on the removed side. One whole-diff invariant
covering all 60 files, so review cost does not scale with the file count.

Subject: the working tree against HEAD; when that is empty (the chunk has committed),
HEAD~1..HEAD, so the entry stays meaningful at the wrap light-gate re-run.
"""

import collections
import re
import subprocess
import sys

ANSI = re.compile(r"\x1b\[[0-9;]*m")
TOKEN = re.compile(r'[A-Za-z_][A-Za-z0-9_]*|"(?:[^"\\]|\\.)*"|[0-9]+')
HUNK_FILE = re.compile(r"^\+\+\+ b/(.+)$")

ROOT = ["git", "-c", "core.autocrlf=false", "diff", "--unified=0", "--no-color"]


def diff(*revs):
    out = subprocess.run(
        ROOT + list(revs) + ["--", "*.rs"],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    if out.returncode != 0:
        print(out.stderr.strip(), file=sys.stderr)
        sys.exit(2)
    return out.stdout


def collect(text):
    """file -> (added tokens, removed tokens), ANSI stripped before classifying."""
    added = collections.defaultdict(collections.Counter)
    removed = collections.defaultdict(collections.Counter)
    current = None
    for raw in text.split("\n"):
        line = ANSI.sub("", raw)
        m = HUNK_FILE.match(line)
        if m:
            current = m.group(1)
            continue
        if current is None or line.startswith("+++") or line.startswith("---"):
            continue
        if line.startswith("@@"):
            continue
        if line.startswith("+"):
            added[current].update(TOKEN.findall(line[1:]))
        elif line.startswith("-"):
            removed[current].update(TOKEN.findall(line[1:]))
    return added, removed


def main():
    text = diff()
    subject = "working tree vs HEAD"
    if not text.strip():
        text = diff("HEAD~1", "HEAD")
        subject = "HEAD~1..HEAD"

    added, removed = collect(text)
    files = sorted(set(added) | set(removed))

    differing = []
    for path in files:
        if added[path] != removed[path]:
            differing.append(path)

    print(f"subject: {subject}")
    print(f"files with .rs hunks: {len(files)}")
    for path in differing:
        only_add = added[path] - removed[path]
        only_del = removed[path] - added[path]
        print(f"  DIFFERS {path}")
        print(f"    only added:   {dict(only_add)}")
        print(f"    only removed: {dict(only_del)}")
    print(f"token-multiset differences: {len(differing)}")
    return 1 if differing else 0


if __name__ == "__main__":
    sys.exit(main())
