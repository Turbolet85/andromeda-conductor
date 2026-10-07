# Curation log — 0-pending wrap, 2026-10-07 (correction after the host leaf's re-seed)

```
CLAUDE.md ecosystem curated (operator-directed corrections; no new learning this session):
  Tier 1 (CLAUDE.md USER:session-learnings):  1 entry rewritten in place to a one-sentence lead + pointer, corrected
      2026-08-22
  Tier 2 (.claude/rules/*):                   2 citations corrected in place
      testing.md (the 2026-09-16 sweep-form fixture entry) · verification-harness.md (the 2026-09-12 RED-baseline entry)
  Tier 3 (.claude/docs/session-learnings.md): + 1 entry, "Tier-1 entry of 2026-08-22, moved whole: …", corrected
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred · 1 recurrence-despite-learning (→ handoff)
  CLAUDE.md size: 138/200 · T1 6.1 KB, 0 over 600 B
```

Proof (Tier 1 and Tier 3, the 2026-08-22 entry): `uname -s` reads `Linux` at this wrap; the host leaf on disk is
`.claude/rules/host-linux.md` (setup commit `717bbf3`); `.github/workflows/ci.yml:18`, `:245` and `:289` read
`windows-latest`, `windows-latest` and `windows-2022`. The move's read-back against `HEAD`'s copies is in
`adaptation-record.md` beside this file: line 132 the only changed `CLAUDE.md` line, the old text contained whole
in the Tier-3 file with the tag removed, the Tier-3 file's prior content intact.

Proof (Tier 2, `testing.md`): the cited drive-letter-anchor entry heads `.claude/docs/session-learnings.md:87`,
"host leaf entry of 2026-09-11, as extended 2026-09-12 and 2026-09-14"; `.claude/rules/host-win32.md` is absent
from the tree.

Proof (Tier 2, `verification-harness.md`): the cited clause is `717bbf3^:.claude/rules/host-win32.md:23`, in the
old leaf's body; `host-linux.md` holds no `UTF-16` text; `verification-harness.md:78` states "PowerShell redirects
write UTF-16/BOM that greps read as empty".

Corrections are exempt from Filter 5's three-entry cap. The conflict the prior handoff carried on the 2026-08-22
entry ("This host is Windows-only" against a Linux dev host) is closed by this correction, on the directive in
this wrap's arguments.

Size figures are the `check 1` row of `health.py check`, before (10.6 KB, 1 of 17 over) and after.
