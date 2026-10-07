# Validation log — setup-project re-run `2026-10-07T15-13-34`

Host: linux. Tool stamps: `upgrade v1.6 · 814083ff` · `health v1.0 · b1b10944` · `route v1.6 · 1b5a5c62`.

## Phase 7 — the word at the card
1. `host 10 learnings` — relayed by the overseer in the session; applied to `host-reseed.json`, fresh dry-run
   exit 0 (keep 3 · learnings 16 · `verification-harness.md` 1 · drop 2).
2. `yes` — the founder's own word, relayed by the overseer: he answered the overseer's dialog at 17:22 local with
   "yes, with the change" (the change being item 1 above). The checkpoint's closing section records the relay as
   given. The dialog is in no committed file.

## Phase 7.5 — the re-seed and the upgrade re-detect
U11 and U12 read `ok` at Phase 0, so neither was applied. The host leaf's re-seed, the tool's lines:

```
wrote b32a9ece05a83470dfe32e4fdf37d9af  .claude/backup/host-win32.md.pre-setup-2026-10-07T15-13-34-setup-project
wrote 9e3bb665ae24aed4c72633ac37a8cf90  .andromeda/runs/2026-10-07T15-13-34-setup-project/host-reseed-dropped.md
wrote ecd0eafe4fbb554dcb7ce263fa76c66e  .claude/rules/verification-harness.md
wrote 02860591efcc6044ff687ee53b1dcf88  .claude/docs/session-learnings.md
wrote 8bf9a4d5c75ca4df6d6e25edeeb9b80b  .claude/rules/host-linux.md
removed  .claude/rules/host-win32.md
U04: re-seeded .claude/rules/host-win32.md (rendered for win32) → .claude/rules/host-linux.md · 22851 B → 3490 B every turn
U04: 22 items · 18438 B found on disk where the sort says — keep 3 · learnings 16 · verification-harness.md 1 · drop 2
```

Read back, not taken from the exit code: `git diff --numstat` reads `138 0` on `session-learnings.md`, `1 0` on
`verification-harness.md`, `0 145` on the removed leaf; the new leaf is 53 lines, its body the template's `any` and
`linux` sections and its `## Session Additions` holding the owner line and items 15 and 22; the 16 inserted
headings are the sort's titles in item order; both destinations read `i/lf w/lf`. CLAUDE.md :111 re-rendered —
`1 1` numstat, 138 lines.

Re-detect, the row this run wrote:

```
U04 · ok-uncommitted · setup · .claude/rules/host-{os}.md · every template line present above `## Session Additions`
```

Summary otherwise unchanged: nothing handed, U09 · U10 · U36 `noted`, `INDETERMINATE 0`. **Upgrade re-detect: ✓**

## Phase 8 — health checks

| Check | Status | Diagnostic |
|---|---|---|
| pre-flight | ✓ | CLAUDE.md at `./CLAUDE.md` |
| 1 CLAUDE.md size | ⚠ | 138/200 lines · Tier-1 region 10.6 KB · 1 of 17 bullets over 600 B (+4.4 KB; line 132, 5078 B — the 2026-08-22 entry, which is also the one naming the old host) — the operator promotes; setup never does |
| 2 section markers | ✓ | 20 markers · 10 starts · 0 orphan / mismatch / unclosed |
| 3 `@`-imports (hand) | ✓ | one line, `@.claude/session-handoff.md`; the target exists |
| 4 rule files | ⚠ | 7 files · frontmatter 5/5 parsed · always-loaded 2 (36.7 KB: `host-linux.md` 3.4 KB, `security.md` 33.3 KB — the 2026-10-06 run read 55.3 KB with the old leaf at 22.3 KB) · `testing.md` 85.2 KB and `verification-harness.md` 82.4 KB past the read cap |
| 5 core docs | ✓ | 5/5 |
| 6 architecture staleness (hand) | ✓ | `architecture.md` mtime is 6896 s OLDER than `CLAUDE.md`'s |
| 7 session-handoff (hand) | ✓ | present, non-empty |
| 8 .gitignore | ✓ | 6/6 entries satisfied by the root `.gitignore` (fragment `rust`) |
| 9 plans + summaries | ✓ | plans 6/6 by name · summaries 5/5 by name |
| 10 master-route | ✓ | present |
| 11 operational artifacts | ✓ | seeded 2/2 · planes rust, ts · pipeline 4/4 · behind py 0 · sql 0 · cookbook 0 |
| 12 state.yaml (hand) | ✓ | `schema_version: 3`; keys exactly `last_wrap` · `tree_db_refreshed_at` · `session_count` |
| 13 agent harness | ✓ | agent-driven · harness 2/2 · `test -x` ok · verbs sh 5/5 · ps1 5/5 |
| 14 pointer table | ✓ | 22 entries |

## Hook smoke (each hook driven through its own stdin, the command taken as stored)

`.claude/settings.json` was not written by this run; the smoke is the letter's and was run all the same.

| Arm | Expected | Got |
|---|---|---|
| `jq` | present | present |
| formatter — a mis-formatted `.setup-validation-test.rs` | exit 0, file changed | exit 0, file changed; temp file removed |
| Bash guard — `cat` heredoc to a file | 2 | 2 |
| Bash guard — python heredoc | 0 | 0 |
| Bash guard — `cd .andromeda && ls` | 2 | 2 |
| Bash guard — `cd . && ls` | 0 | 0 |
| Bash guard — `ls && cd .andromeda` | 2 | 2 |
| Bash guard — `ls; ( cd .andromeda && ls )` | 0 | 0 |
| write guard — a `src` path | 0 | 0 |
| write guard — a backslash `target` path | 2 | 2 |
| write guard — a drive-form backslash `target` path | 2 | 2 |

**Hook smoke: ✓** (formatter · Bash guard · write guard · jq).

## Summary
`12 ✓ / 2 ⚠ / 0 – / 0 ✗` over the 14 checks · hook smoke ✓ · upgrade re-detect ✓.
Both warnings (1, 4) are curation weight and stood before this run; check 4's always-loaded figure fell by the
re-seed.

**Decision: commit.**
