R = ""


def edit(path, old, new):
    p = R + path
    s = open(p, encoding="utf-8", newline="").read()
    assert s.count(old) == 1, (path, s.count(old), old[:60])
    s = s.replace(old, new)
    open(p, "w", encoding="utf-8", newline="").write(s)
    print(path, "ok")


# Correction 1 — CLAUDE.md USER:session-learnings, 2026-08-22 entry, clause (3)
edit("CLAUDE.md",
     "is a MISSING KEY PATH, routed as a CARRY to make it agent-driven — not a manual arm. (confidence 0.9)",
     "is a MISSING KEY PATH, routed as a CARRY to make it agent-driven — not a manual arm **[corrected 2026-09-30: the "
     "key path now exists — the `sr*` legs send Tab / Shift+Tab / the browse keys through the OS input path, so only a "
     "row with no OS key in its window stays unreached]**. (confidence 0.9)")

# Correction 2 — verification-harness.md Session Additions 2026-09-02, clause (5)
edit(".claude/rules/verification-harness.md",
     "(5) Injected keys reach its focus/live handling, never its browse mode (an OS keyboard hook): a browse-class row is "
     "a missing KEY PATH, not a missing driver.",
     "(5) Injected keys reach its focus/live handling, never its browse mode (an OS keyboard hook): a browse-class row is "
     "a missing KEY PATH, not a missing driver. **[corrected 2026-09-30: under the leg's own driver launch injected keys "
     "reach focus handling only in the window's first burst — in one session injected Tabs were heard 0/5 and 0/4 while "
     "OS `SendInput` Tabs were heard 5/5 — so the leg now sends Tab / Shift+Tab / the browse keys through the OS input "
     "path, which reaches both focus and browse mode]**")

# L1 — a11y.md Session Additions (Tier 2), appended at the file's end
p = R + ".claude/rules/a11y.md"
s = open(p, encoding="utf-8", newline="").read()
assert s.endswith("\n")
s += ("- 2026-09-30: **Send a modifier through `SendInput` in its OWN call and release it only after NVDA has handled the "
      "key it modifies** — one batched Shift+Tab left NVDA holding Shift, so every later OS key arrived as `shift+tab` and "
      "its browse-mode Shift+Tab moved focus backwards; confirm the OS key state reads up afterwards.\n")
open(p, "w", encoding="utf-8", newline="").write(s)
print(".claude/rules/a11y.md appended")

# L6 — verification-harness.md Session Additions (Tier 2), appended at the file's end
p = R + ".claude/rules/verification-harness.md"
s = open(p, encoding="utf-8", newline="").read()
assert s.endswith("\n")
s += ("- 2026-09-30: **A slot-gated leg that throws on a HARNESS defect (the leg's own code, not a grade) may be re-fired "
      "once, on a new slot, as a fix-loop re-fire; a second harness defect on the same leg stops and reports, and a fix "
      "to a live leg's key path is proven by a short probe over the same launch before the live re-fire** (operator "
      "rule, 2026-09-30).\n")
open(p, "w", encoding="utf-8", newline="").write(s)
print(".claude/rules/verification-harness.md appended")

# L4 — Tier 3, docs/session-learnings.md, inserted at the top (reverse chronological)
p = R + ".claude/docs/session-learnings.md"
s = open(p, encoding="utf-8", newline="").read()
anchor = "## 2026-09-29 — A rule fixed before a drive"
assert s.count(anchor) == 1
entry = (
    "## 2026-09-30 — A sweep over the masters needs `-oiE … | wc -l`, and an escaped pipe under `-E` is a literal\n\n"
    "Two grep mechanics returned confident wrong counts in one site sweep over the spec masters. First, under `grep -E` "
    "the sequence backslash-pipe is a LITERAL pipe character, not alternation, so a pattern written in BRE habit "
    "(`a\\|b`) matched nothing and every master read 0 — a false absence. Second, `grep -c` counts LINES, and the "
    "masters carry multi-KB single lines holding several occurrences each, so a count read as sites under-counted "
    "(the Tier-1 2026-09-17 entry already names this, and it recurred). The form that answers the site question is "
    "`grep -oiE 'a|b' {file} | wc -l` for occurrences, plus `grep -noiE '.{0,90}(a|b).{0,60}'` to read each hit in "
    "context before dispositioning it.\n\n---\n\n")
s = s.replace(anchor, entry + anchor)
open(p, "w", encoding="utf-8", newline="").write(s)
print(".claude/docs/session-learnings.md inserted")
