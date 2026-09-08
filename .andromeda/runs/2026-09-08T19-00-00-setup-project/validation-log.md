# Validation Log — setup-project re-run (CE-2)

**Run:** `2026-09-08T19-00-00-setup-project`
**Subject:** CLAUDE.md after the CE-2 absorption; ecosystem unchanged otherwise.

## Pre-flight (FATAL)
`CLAUDE.md` resolves at `./CLAUDE.md`. **PASS** — no halt.

## The 14 health checks

| # | Check | Status | Measured |
|---|---|---|---|
| 1 | CLAUDE.md size ≤200 | ✓ | 135 lines |
| 2 | Section markers parseable | ✓ | 10 pairs, stack empty at EOF, no orphan/mismatch |
| 3 | `@`-imports valid | ✓ | 1 import (`.claude/session-handoff.md`), target exists |
| 4 | Rule YAML frontmatter | ✓ | 7 rules, 5 carry frontmatter (all parse); 2 unconditional-load rules carry none — a PASS per health-criteria |
| 5 | Docs 5 core present | ✓ | stack · conventions · commands · gotchas · workflow |
| 6 | Architecture staleness | ✓ | arch mtime −0.9h vs CLAUDE.md; 6 recent amendment sidecars |
| 7 | session-handoff.md exists | ✓ | present, non-empty |
| 8 | .gitignore Claude + stack | ✓ | `.claude/backup/` · `.claude/settings.local.json` · `/target` · `node_modules` |
| 9 | 6 plans + 5 summaries | ✓ | checked BY NAME (a `*-plan.md` glob counts four — the known false red) |
| 10 | master-route present | ✓ | 2 version sections, 128 lines |
| 11 | Operational artifacts + code-graph pipeline | ✓ | drift-base · playbook · code-graph.py · views.sql · scip_pb2.py · requirements.txt |
| 12 | state.yaml lean + parseable | ✓ | `schema_version: 3`, exactly the 4 lean fields |
| 13 | Agent harness (agent-driven) | ✓ | both scripts, all 5 verbs, `agent-run.sh` executable on-host |
| 14 | Pointer table ≥5 entries | ✓ | **22** rows (was 21; CE-2 added one) |

**Summary: 14 ✓ / 0 ⚠ / 0 – / 0 ✗**

## Hook smoke test (stdin input path — never the tool directly)

| Arm | Status | Measured |
|---|---|---|
| `jq` present | ✓ | jq-1.8.1 — guards actually evaluate (absent ⇒ every hook exits 0 open) |
| PostToolUse formatter | ✓ | fed `{"tool_input":{"file_path":".setup-validation-test.rs"}}` on stdin; exit 0 **and the file was reformatted** — the input path works, not merely the tool |
| Bash guard — deny arm | ✓ | heredoc-with-file-target ⇒ exit 2 |
| Bash guard — allow arm | ✓ | heredoc-to-program (`python - <<PY`) ⇒ exit 0 |
| Write guard — deny arm | ✓ | `target/debug/thing.rs` ⇒ exit 2 |
| Write guard — allow arm | ✓ | `crates/conductor-core/src/lib.rs` ⇒ exit 0 |

**Hook smoke: 6 ✓ / 0 ⚠.** Temp file removed; residue check clean.

Both guard directions were probed deliberately. A deny-only probe cannot tell a
working guard from one that blocks everything, and an over-blocking guard is the
failure mode that would silently stop ordinary work.

The smoke test ran from the Bash tool's own shell, not through a native-process
wrapper: a shell spawned by Windows-native python does not inherit the MSYS PATH,
`jq` resolves absent, and every hook's leading `command -v jq || exit 0` would
return a false PASS (host-win32.md Session Additions, 2026-09-08).

## Template drift (Phase 4 / Phase 6)

| File | Result | Disposition |
|---|---|---|
| `scripts/code-graph.py` | IDENTICAL (386 lines) | no backup, no proposal |
| `scripts/code-graph-views.sql` | IDENTICAL (64 lines) | no rebuild owed |
| `scripts/requirements.txt` | IDENTICAL (2 lines) | — |
| `scripts/scip_pb2.py` | IDENTICAL | vendored |
| `scripts/code-graph-cookbook.md` | IDENTICAL | tail marker intact |
| `scripts/agent-run.{sh,ps1}` | project-evolved, PRESERVED | expected divergence, not drift to remediate |

Byte-compared against the fenced template bodies this run. The pipeline being
*present* is not the check — the 2026-08-21 re-run found it two months stale
while every presence check passed.

## Commit decision
Pre-flight pass, zero `✗` → **commit**.
