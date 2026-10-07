# Host leaf re-seed — the proposed sort (nothing is moved until the card's "yes")

`.claude/rules/host-win32.md` was rendered for win32; this host is linux (`upgrade.py host`, `upgrade v1.6 · 814083ff`).
Leaf md5 `b32a9ece05a83470dfe32e4fdf37d9af` · 22851 B · 145 lines · 22 items below `## Session Additions`.
The machine form of this list is `host-reseed.json` beside this file; the tool reads that, never this.

**What the re-seed would do** (the tool's dry-run, verbatim):

```
upgrade v1.6 · 814083ff
U04: would re-seed .claude/rules/host-win32.md (rendered for win32) → .claude/rules/host-linux.md · 22851 B → 3490 B every turn
U04: 22 items · 18438 B — keep 3 · learnings 16 · verification-harness.md 1 · drop 2
  .claude/rules/verification-harness.md: `paths:` 4 glob(s) · 83612 B · +1 line(s)
  .claude/docs/session-learnings.md: Tier 3 — read on demand · 146861 B · +138 line(s)
still naming host-win32.md: 3 line(s) — CLAUDE.md:111 · .claude/rules/testing.md:91 · .claude/rules/verification-harnes…
```

Every item moves byte for byte or not at all. `keep` = stays in the new `host-linux.md` (loaded every turn).
`learnings` = `.claude/docs/session-learnings.md` (Tier 3, read on demand), each under a `## 2026-10-07 — {title}`
heading. `drop` = written verbatim to `host-reseed-dropped.md` in this run dir, which the commit carries.

## The sort — one line per item

Rule numbers are Phase 0 step 8a's, the first yes winning: 1 a path-scoped rule file · 2 old-host-only → drop ·
3 live with an old-host clause → learnings (`mixed`) · 4 live and within ~600 B → keep · 5 the rest → learnings.

| n | lines | bytes | entry (its own date) | to | why |
|---|---|---|---|---|---|
| 1 | 65 | 142 | the section's owner line | **keep** | rule 4 — names no host |
| 2 | 66-77 | 1224 | 2026-09-10 · a line-granular grep cannot date a clause inside a multi-KB single line | learnings | rule 5 — live, over ~600 B; cites the old leaf's `Long single-line files` section |
| 3 | 78-89 | 1196 | 2026-09-08 · the bash-to-native boundary changes paths and environment | learnings · mixed | the MSYS half is the old host's; "a tool-missing result is a claim about the spawning path" is host-neutral |
| 4 | 90-101 | 1204 | 2026-09-08 · the Bash tool's working directory persists across calls | learnings | rule 5 — live, over ~600 B; its directive is the new leaf's own template line |
| 5 | 102-114 | 1270 | 2026-09-08 · `grep -E` accepts a PCRE lookahead silently | learnings · mixed | the positive-probe rule is live; it leans on the old host's `grep -P` clause |
| 6 | 115-128 | 1470 | 2026-09-10 · a clipped search over several paths answers for the first paths only | learnings | rule 5 — live, over ~600 B |
| 7 | 130 | 3568 | 2026-09-11 (extended 09-12, 09-14) · the drive-letter anchor's false positives | learnings · mixed | over the ~1.5 KB a rule file takes; URL-scheme and gate-text classes live, the registry class the old host's |
| 8 | 131 | 926 | 2026-09-15 · an unset variable in a redirect path | learnings · mixed | `${VAR:?}` is live; the `$TEMP` remedy is the old host's |
| 9 | 132 | 1035 | 2026-09-15 · a sub-agent extract goes through the Write tool | learnings · mixed | the rule is live; the CRLF mechanism is the old host's |
| 10 | 133 | 582 | 2026-09-16 · a phrase wrapped across two comment lines is invisible to grep | learnings | re-sorted at the card (`host 10 learnings`) — proposed `keep` by rule 4 (live, 582 B); moved so it stays beside item 2, the "2026-09-10 clause above" it refers to |
| 11 | 134 | 984 | 2026-09-16 · a Windows capability check returning False | learnings · mixed | Windows-only tools the committed a11y token scripts still carry |
| 12 | 135 | 755 | 2026-09-16 · a surviving grandchild pins the parent; `Start-Process` without `-Wait` | **`verification-harness.md`** | rule 1 — a harness launch recipe; that file's `paths:` names `scripts/agent-run.*`, and `agent-run.ps1:230` cites it |
| 13 | 136 | 1081 | 2026-09-17 (corrected 09-24) · a GitHub Actions `if:` cannot read a runner-process variable | learnings · mixed | a CI-workflow lesson, live; no rule file's `paths:` names the workflow |
| 14 | 137 | 551 | 2026-09-17 · a Windows process is attributed by parentage | learnings · mixed | Windows-only tooling the committed `webview2-cause-probe.ps1` still carries |
| 15 | 138 | 277 | 2026-09-23 · a multi-file count probe reads green when a file is missing | **keep** | rule 4 — live, host-neutral |
| 16 | 139 | 369 | 2026-09-29 · a stopped Monitor can leave its `tail` child running | learnings · mixed | the lesson is live; the census it prescribes is the old host's |
| 17 | 140 | 211 | 2026-09-30 · a BOM-less scratch `.ps1` with a non-ASCII character | **drop** | rule 2 — scratch scripts under Windows PowerShell 5 on the dev host |
| 18 | 141 | 349 | 2026-09-30 · `mklink` through the Bash tool; junctions | **drop** | rule 2 — no committed file uses a junction |
| 19 | 142 | 286 | 2026-09-30 · Windows PowerShell 5 has no `[ushort]` / `[uint]` | learnings · mixed | PowerShell-5-only, but the committed `.ps1` scripts carry `[uint16]` |
| 20 | 143 | 509 | 2026-10-02 (extended 10-04) · `grep -i` aborted; ugrep refuses bounded repeats | learnings · mixed | the lead clause is the old host's (`grep -i` exits 0 here, measured); the extension is this host's |
| 21 | 144 | 238 | 2026-10-02 · stop this repo's rust-analyzer flycheck before a heavy cargo step | learnings · mixed | the lesson is live; the pid read it names is the old host's |
| 22 | 145 | 211 | 2026-10-06 · `inputs.py snap` refuses a source under the OS temp dir | **keep** | rule 4 — live, host-neutral |

**Totals (after the card's `host 10 learnings`; the first proposal read keep 4 · learnings 15):** keep 3 (630 B,
every turn) · learnings 16, of them 12 mixed (16493 B, on demand) ·
`verification-harness.md` 1 (755 B, path-scoped) · drop 2 (560 B, to the run dir). 18438 B in all.

## Three things the sort rests on that are the founder's to see

1. **"Drop" was read narrowly.** Windows tooling has not left the repository: `.github/workflows/ci.yml` runs its
   three jobs on Windows runners (`windows-latest` x2, `windows-2022` x1) with nine `shell: pwsh` steps, and six `.ps1`
   scripts are tracked outside the run dirs. So an entry whose tools a committed script or the workflow still uses (items 11, 12, 13, 14,
   19) was sorted as a live lesson, not dropped; only the two entries about the dev host's own scratch work
   (17, 18) are dropped. If the ruling retires those lessons too, `host {n} drop` moves each.
2. **Item 12 goes into an 83.6 KB rule file** that is already past the read cap. Rule 1 names it; `host 12 learnings`
   sends it to Tier 3 instead.
3. **One host-neutral body section has no place in the new leaf.** The old leaf's `## Long single-line files`
   (lines 58-62, above the cut) is not in the template the linux leaf is rendered from, and a body line is not an
   item, so the re-seed moves it nowhere. It stays in git history and in the gitignored backup. Items 2 and 10
   refer to it. Restoring it is a template change — the pipeline overseer's, not setup's.

## Still naming the old leaf after the re-seed (setup edits none of these but the first)

- `CLAUDE.md:111` — the `GENERATED:setup:deeper-topics` rule list; Phase 7.5 re-renders it after "yes".
- `.claude/rules/testing.md:91` · `.claude/rules/verification-harness.md:66` — preserved leaves; the cascade's.
- Outside the tool's scan: `scripts/agent-run.ps1:230` and `scripts/a11y-token-witness.ps1:160` (source comments),
  `.claude/session-handoff.md`, the architecture amendment sidecars and the friction ledger (records).

## `USER:session-learnings` bullets that name the old host — listed, never edited

- **2026-08-22** (CLAUDE.md, the twelfth bullet of the region) — opens "This host is Windows-only — no Linux runner
  exists or is planned". One bullet of the 17. Door: a 0-pending `/andromeda-wrap-session`, asked for the
  correction. (The handoff already carries this as a curation conflict.)
