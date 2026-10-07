# 0-pending wrap — correction after the host leaf's re-seed (2026-10-07)

**Path:** Setup step 6, the no-op path. 162 master records, all `complete`; 0 pending, 0 gated (`route.py cursor`
and an anchored status-word grep agree). The tree at entry was dirty only with bookkeeping: `friction-log.ndjson`
(one record), the handoff's session-end stamp, and the prior wrap's evolve trail (a tracked run-dir file the
trailing friction append extends after its commit).

**The request,** as this wrap's invocation arguments give it (2026-10-07; the giver is not named there): after
setup commit `717bbf3` the host leaf is `host-linux.md`, on the founder's 17:22 approval. Correct what still says
otherwise: the 2026-08-22 Tier-1 bullet (re-tier it under the 600 B cap, lose no fact), `testing.md:91`,
`verification-harness.md:66`, and the handoff's two mentions of the old leaf. Leave `scripts/agent-run.ps1:230`
and `scripts/a11y-token-witness.ps1:160` as product files and put a CARRY on the version-close entry. Route
unchanged.

## Measured before and after (`health.py check`, check 1)

| | lines | Tier-1 region | bullets over 600 B |
|---|---|---|---|
| before | 138/200 | 10.6 KB | 1 of 17 (+4.4 KB) |
| after | 138/200 | 6.1 KB | 0 of 17 |

## Items and dispositions

| # | Subject | Disposition |
|---|---|---|
| 1 | `CLAUDE.md:132`, the 2026-08-22 Tier-1 bullet | Re-tiered and corrected. 5078 B to a 482 B lead; full text to Tier 3. |
| 2 | `.claude/rules/testing.md:91` | Citation corrected in place, tagged. |
| 3 | `.claude/rules/verification-harness.md:66` | Citation corrected in place, tagged. |
| 4 | `.claude/session-handoff.md`, two mentions | Corrected by the handoff's full overwrite at this wrap. |
| 5 | `scripts/agent-run.ps1:230`, `scripts/a11y-token-witness.ps1:160` | Left byte-identical. `CARRY:` on `working-route.md:100`. |

### 1 — the 2026-08-22 bullet

- **Form,** as for the other eight (`.andromeda/runs/2026-10-07T10-14-47-wrap/adaptation-record.md`): a
  read-modify-write by path, never retyped. The bullet's text after its leading `- ` landed under
  `## 2026-10-07 — Tier-1 entry of 2026-08-22, moved whole: …` in `.claude/docs/session-learnings.md`, split into
  four paragraphs before its three dated extensions. It sits in the moved-entries block in origin-date order,
  between the 2026-09-02 and 2026-08-21 entries.
- **The one change to the moved text:** a `[corrected 2026-10-07 …]` tag (633 B) after the opening sentence. The
  entry opened "This host is Windows-only — no Linux runner exists or is planned". Measured at this wrap:
  `uname -s` reads `Linux`; the host leaf is `.claude/rules/host-linux.md`; `.github/workflows/ci.yml` names
  three `runs-on` values, `windows-latest` at `:18` and `:245` and `windows-2022` at `:289`. So a Linux host
  exists and a Linux CI runner does not. The tag says that, and says which legs the Linux host can run was not
  measured.
- **The lead** keeps the origin date, states the rule in one sentence, carries a short correction tag and ends
  with the pointer to the Tier-3 heading. The confidence figures and the three extensions stay in the full text.
- **Read-back, against `HEAD`'s copies** (`717bbf3`): `CLAUDE.md` 138 lines before and after, line 132 the only
  changed line. The old bullet's text, whitespace collapsed, is contained whole in the Tier-3 file with the tag
  removed. The Tier-3 file's prior content is intact around the inserted block (6226 B inserted, 165914 B to
  172140 B). No paragraph leaves a bold span open. Both files stay LF.
- **Recovery:** the old bullet is `git show 717bbf3:CLAUDE.md`, line 132 (sha256 of the line, first 16:
  `3964045ee6914d89`).

### 2 and 3 — the two rule-file citations

Each is a clause inside a `## Session Additions` entry, so it was corrected in place with a dated tag
(curation-guide, Corrections), by one anchored Edit on a unique string. `git diff --numstat` reads 1/1 for each
file.

- **`testing.md:91`** cited `.claude/rules/host-win32.md` (2026-09-12, the drive-letter-anchor entry). Setup
  moved that entry whole to Tier 3; it now heads `.claude/docs/session-learnings.md:87` as "host leaf entry of
  2026-09-11, as extended 2026-09-12 and 2026-09-14". The citation names that home.
- **`verification-harness.md:66`** cited `host-win32.md` §Transports for "a PowerShell redirect writes
  UTF-16/BOM". That clause sat in the old leaf's body (`717bbf3^:.claude/rules/host-win32.md:23`), not in an
  entry setup moved, and the Linux leaf has no such clause. The same fact is stated in
  `verification-harness.md:78`, the 2026-09-16 surviving-grandchild entry that setup moved in from the old
  leaf. The citation names that entry, and the tag says the retired body's own wording is in git history only.

### 5 — the CARRY

Appended to the version-close entry (`working-route.md:100`, markerless), one `CARRY:` block of 951 chars.
`route.py pins` reads it as one row on `:100`, beside the five standing rows on `:98`. It quotes what each
comment says, names the commit that removed the leaf, and names where each cited lesson stands now
(`verification-harness.md`'s 2026-09-16 entry for the first; the Tier-3 drive-letter-anchor entry for the
second). Authority: word: "scripts/agent-run.ps1:230 and scripts/a11y-token-witness.ps1:160 are product files:
leave them and put a CARRY on the version-close entry" — this wrap's invocation arguments, 2026-10-07.

No entry was added, removed or reordered.

## What still names the old leaf, and why it stands

- The two correction tags and the CARRY quote the old name to say what was cited. That is the record of the
  correction, not a live citation.
- The two `.ps1` comments: product files, carried.
- Chunk folders, `route-archive.md` and the amendment sidecars (`architecture-amendments.md:750` among them):
  frozen or append-only history.

## Noticed, not touched

The masters and their leaves still describe the webview legs' home as "the Windows dev host": architecture 4
hits, test-plan 4, the registries 4, and the leaves `a11y.md:39`, `frontend.md:50`, `a11y-summary.md:11` and
`:39`, `commands.md:30`, `stack.md:39`, `tests-summary.md:46`. These are master claims about where a leg was
measured. This path runs no report and no fan-out and may not amend a master on drift it noticed; the directive
named none of them. They are named in the handoff for a chunk wrap's reconcile or the operator's word.

## Other step-6 duties

- **Gated re-check:** 0 `gated` records.
- **The `BLOCKED-ON` on `:98`:** premise re-read because the tail was touched. Pulse's master record
  `2026-10-07-l4-probe-reproduces-the-canary-history-miss` reads `pending`; Pulse HEAD is `48714f0`. Standing.
  The annotation was not edited.
- **Curation of this session's own conversation:** no new learning. One recurrence logged to the handoff's
  deferred learnings: a `cd` into the sibling Pulse repo, refused by the PreToolUse guard (`host-linux.md` §Paths).
- No consolidation, supersession, registry migration or master amendment ran.
