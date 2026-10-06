# Validation log — setup-project re-run `2026-10-06T21-31-03`

Host: Linux dev host. Tool stamps: `upgrade v1.5 · eeed2076` · `health v1.0 · 3c0f4685` · `route v1.6 · 699f7ef0`.

## Phase 7 — the operator's word
`yes`, with three rulings relayed in the same message (overseer, founder-delegated):
1. Keep the five U35 pointer-table rows (the four registries they name exist on disk).
2. Include the exec bit on `scripts/agent-run.sh` in the manifest — performed after the card, on that word:
   `chmod +x`, a mode-only change (`100644 => 100755`, numstat `0 0`, content md5 unchanged). The checkpoint's
   §4 recorded it as a proposal; this line and the checkpoint's closing section record that it was performed.
3. U14 stays as it is: the detector fires on a backticked literal inside the working-route :94 CONTEXT prose,
   not on a real introducer; the next wrap's P5 re-spells it.

## Phase 7.5 — upgrade re-detect
U11 and U12 read `ok` at Phase 0, so no `apply` ran. Re-detect, the row this run wrote:

```
U02 · ok-uncommitted · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin pr…
```

`ok-uncommitted` = written, P9 commits it. Summary line unchanged otherwise: U14 `behind` (hand), U04 · U09 ·
U10 · U36 `noted`, `INDETERMINATE 0`. **Upgrade re-detect: ✓**

## Phase 8 — health checks

| Check | Status | Diagnostic |
|---|---|---|
| pre-flight | ✓ | CLAUDE.md at `./CLAUDE.md` |
| 1 CLAUDE.md size | ⚠ | 138/200 lines · Tier-1 region 47.8 KB · 9 of 17 bullets over 600 B (+40.3 KB) — the operator promotes; setup never does |
| 2 section markers | ✓ | 20 markers · 10 starts · 0 orphan / mismatch / unclosed |
| 3 `@`-imports (hand) | ✓ | one line, `@.claude/session-handoff.md`; the target exists |
| 4 rule files | ⚠ | 7 files · frontmatter 5/5 parsed · always-loaded 2 (55.3 KB: `host-win32.md` 22.3 KB, `security.md` 33.0 KB) · `testing.md` 85.2 KB and `verification-harness.md` 80.6 KB past the read cap |
| 5 core docs | ✓ | 5/5 |
| 6 architecture staleness (hand) | ✓ | `architecture.md` mtime is 905 s OLDER than `CLAUDE.md`'s |
| 7 session-handoff (hand) | ✓ | present, non-empty |
| 8 .gitignore | ✓ | 6/6 entries satisfied by the root `.gitignore` (fragment `rust`) |
| 9 plans + summaries | ✓ | plans 6/6 by name · summaries 5/5 by name |
| 10 master-route | ✓ | present |
| 11 operational artifacts | ✓ | seeded 2/2 · planes rust, ts · pipeline 4/4 · behind py 0 · sql 0 · cookbook 0 |
| 12 state.yaml (hand) | ✓ | `schema_version: 3`; keys exactly `last_wrap` · `tree_db_refreshed_at` · `session_count` |
| 13 agent harness | ✓ | agent-driven · harness 2/2 · `test -x` ok · verbs sh 5/5 · ps1 5/5 (was ⚠ at the Phase 0 baseline; the exec bit moved it) |
| 14 pointer table | ✓ | 22 entries |

## Hook smoke (each hook driven through its own stdin, the command taken as stored)

| Arm | Expected | Got |
|---|---|---|
| `jq` | present | present |
| formatter — a mis-formatted `.setup-validation-test.rs` | exit 0, file changed | exit 0, file reformatted; temp file removed |
| Bash guard — `cat` heredoc to a file | 2 | 2 |
| Bash guard — python heredoc | 0 | 0 |
| Bash guard — `cd .andromeda && ls` | 2 | 2 |
| Bash guard — `cd . && ls` | 0 | 0 |
| Bash guard — `ls && cd .andromeda` | 2 | 2 |
| Bash guard — `ls; ( cd .andromeda && ls )` | 0 | 0 |
| write guard — `src/x.rs` | 0 | 0 |
| write guard — a backslash `target` path | 2 | 2 |
| write guard — a drive-form backslash `target` path | 2 | 2 |

**Hook smoke: ✓** (formatter · Bash guard · write guard · jq).

## Summary
`12 ✓ / 2 ⚠ / 0 – / 0 ✗` over the 14 checks · hook smoke ✓ · upgrade re-detect ✓.
Both warnings (1, 4) stood before this run and are curation weight, not setup's to move.

**Decision: commit.**
