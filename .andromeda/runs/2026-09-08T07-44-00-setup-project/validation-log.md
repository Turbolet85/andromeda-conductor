# Validation Log — setup-project re-run

_Phase 8. Run 2026-09-08T07-44-00Z. Project: Conductor · branch `build/conductor-0.2.0` ·
Development Style `agent-driven` · mode RE-RUN._

## Pre-flight (FATAL)

`test -f CLAUDE.md` → **PASS** (present, 135 lines). No halt.

## Checks 1–14

| # | Check | Status | Result |
|---|---|---|---|
| 1 | CLAUDE.md size ≤200 | ✓ | 135 / 200 (was 134; +1 net from the maintainer-note comment) |
| 2 | Section markers parseable | ✓ | 10 start/end pairs, 0 orphans, 0 mismatches (authoritative regex from `section-markers.md`) |
| 3 | `@`-imports valid | ✓ | 2/2 resolve — `.andromeda/architecture.md`, `.claude/session-handoff.md` |
| 4 | Rule YAML frontmatter | ✓ | 5 files with frontmatter all parse; `security.md` + `host-win32.md` carry none by design (unconditional load) — a PASS per the criterion |
| 5 | Docs 5 core present | ✓ | stack · conventions · commands · gotchas · workflow |
| 6 | Architecture staleness | ✓ | `architecture.md` mtime older than `CLAUDE.md`; 7 amendment sidecars modified within 24h of it |
| 7 | session-handoff.md exists | ✓ | present, non-empty |
| 8 | .gitignore Claude + stack | ✓ | `.claude/backup/`, `.claude/settings.local.json`, `/.andromeda/cache/`, `scripts/__pycache__/`, `/target/`, `**/*.rs.bk`, `*.iml` |
| 9 | 6 plans + 5 summaries | ✓ | all six `.andromeda/` plans by name; 5 `*-summary.md` |
| 10 | master-route present | ✓ | present (2 version headings, 115 records, 0 `pending`) |
| 11 | Operational artifacts seeded | ✓ | `drift-base.md` · `playbook.md` · the 4 code-graph files + cookbook. Host tools per plane: `rust-analyzer` ✓, `scip-typescript` ✓, python `duckdb`+`protobuf` ✓. Both planes built (`cache/rust/tree.db`, `cache/ts/tree.db`) |
| 12 | state.yaml lean + parseable | ✓ | `schema_version: 3`; exactly the lean keys (`last_wrap`, `tree_db_refreshed_at`, `session_count`) |
| 13 | Agent harness (agent-driven) | ✓ | `agent-run.sh` (exec bit set on-host) + `agent-run.ps1`, both exposing all 5 commands |
| 14 | Pointer table ≥5 entries | ✓ | 23 table lines (21 data rows) |

## Hook smoke test

Run in the **Git-Bash shell the runtime uses**, each hook fed its input on stdin (never the tool
invoked directly), payloads built with `jq -n` so no banned literal ever reached a command line.

| Arm | Expected | Measured |
|---|---|---|
| Formatter (`rustfmt` on `.setup-validation-test.rs`) | exit 0 **and the file actually reformatted** | exit 0, reformatted ✓ (`fn main( ) {let x=1;}` → `fn main() {\n    let x = 1;\n}`) |
| Bash transport guard — deny arm (`cat`-heredoc with a file target) | exit 2 | 2 ✓ |
| Bash transport guard — allow arm (`python - <<'PY'`) | exit 0 | 0 ✓ |
| Generated-directory guard — deny (`crates/x/target/foo.rs`) | exit 2 | 2 ✓ |
| Generated-directory guard — allow (`crates/conductor-core/src/lib.rs`) | exit 0 | 0 ✓ |
| `jq` present | required, else hooks exit 0 open | `jq-1.8.1` at `/c/Users/turbo/scoop/shims/jq` ✓ |

Temp file removed; no residue in the repo.

**Recorded probe artifact (not a defect).** The first smoke attempt ran the hooks through
Windows-native Python's `subprocess.run(["bash", …])` and reported all five arms failing with
`jq present: False`. That measured the PROBE: the `bash` Python resolves has a different PATH and
no `jq`, so every hook took its `command -v jq || exit 0` guard. Re-run in the runtime's own shell,
all five arms pass. The earlier reading is retracted — a diagnostic form must be validated where the
real path passes before its failure is believed.

## settings.json equivalence (Phase 5, measured)

Compared element-by-element against `references/hooks-matrix.md` (2026-09-05) rather than inspected:

- `env.PYTHONUTF8 = "1"`, `env.PYTHONIOENCODING = "utf-8"` — present (matrix renders these unconditionally)
- `PreToolUse[Edit|MultiEdit|Write|NotebookEdit]` — **IDENTICAL** to the matrix's literal JSON
- `PreToolUse[Bash]` — **IDENTICAL** to the matrix's literal JSON
- `PostToolUse` — 1 row, **IDENTICAL** to the matrix's Rust formatter prologue
- clippy at write time — **absent**, as the matrix's file-scoped Rust linter row requires (clippy has
  no file scope; it stays a gate)

Net: the re-render is byte-identical to the file on disk. No write performed.

## Summary

**14 ✓ / 0 ⚠ / 0 – / 0 ✗**  ·  Hook smoke **6/6** ✓

**Commit decision: COMMIT** (pre-flight pass, zero `✗`).
