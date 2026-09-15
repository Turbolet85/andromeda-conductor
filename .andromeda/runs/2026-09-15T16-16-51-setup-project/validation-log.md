# Validation Log — setup-project re-run `2026-09-15T16-16-51`

## Phase 8 — the 14 health checks

Pre-flight (FATAL): `./CLAUDE.md` exists — **pass**.

| # | Check | Status | Reading |
|---|---|---|---|
| 1 | CLAUDE.md size · Tier-1 entry cap | ⚠ | 136/200 lines **✓**; `USER:session-learnings` **45.6 KiB across 15 bullets, 8 over the 600 B cap — those 8 hold 43.7 KiB, of which 39.0 KiB is above the cap** (45.6 KiB = the marked block incl. heading + prologue, 46 672 B; bullets alone 45.3 KiB. Removable debt is 39 940 B — the 8 × 600 B entitlement is not debt) |
| 2 | Section markers parseable | ✓ | 9 `GENERATED` + 1 `USER`, stack empty at EOF, no orphan/mismatch |
| 3 | `@`-imports valid | ✓ | 1 import, `.claude/session-handoff.md`, resolves |
| 4 | Rule files — frontmatter · size | ⚠ | YAML parses on all 5 that carry it; 2 unconditional files (no `paths:`) correctly pass. **`testing.md` 79.4 KB and `verification-harness.md` 69.8 KB exceed the 25 000-token read cap**; always-loaded total 40.2 KB |
| 5 | Docs 5 core present | ✓ | stack · conventions · commands · gotchas · workflow |
| 6 | Architecture staleness | ✓ | arch newer than CLAUDE.md by 2.3 h, but `architecture-amendments.md` modified within 0.0 h of it — the v3 amend-workflow arm, not a defect |
| 7 | session-handoff.md | ✓ | present, 5 641 B |
| 8 | .gitignore Claude + stack | ✓ | all base entries; `scripts/__pycache__/` covered by bare `__pycache__/` (`git check-ignore -v` → `.gitignore:8`) |
| 9 | 6 plans + 5 summaries | ✓ | all 11 present by name |
| 10 | master-route present | ✓ | 129 records, 0 `pending` |
| 11 | Operational artifacts · code-graph | ✓ | seeded; both planes' host tools resolve (rust-analyzer · scip-typescript · duckdb 1.5.3 + protobuf). **Currency arm: all 5 files now byte-identical to their templates** (was 2 behind at session start) |
| 12 | state.yaml lean + parseable | ✓ | schema 3; exactly `last_wrap` · `tree_db_refreshed_at` · `session_count` |
| 13 | Agent harness (agent-driven) | ✓ | `agent-run.sh` 327 lines (`test -x` pass) + `.ps1` 346; all 5 commands in both |
| 14 | Pointer table ≥5 entries | ✓ | 22 data rows, every `§` anchor grepped and resolving |

**Summary: 12 ✓ / 2 ⚠ / 0 –.** Both warnings are curation debt the operator owns (wrap never auto-promotes);
neither was introduced by this run and neither blocks.

---

## Phase 6 change set — verification performed

| Assertion | Method | Result |
|---|---|---|
| `code-graph.py` matches its template | byte-diff of the fenced body | **identical, 387 lines** |
| `code-graph.py` is valid python | `python -m py_compile` | **OK** |
| `code-graph-cookbook.md` matches its template | full-content comparison | **identical, 107 lines** |
| cookbook learnings marker + tail intact | string probe | marker present; tail empty in template and live alike, so nothing was at risk |
| cookbook gained the three template shapes | string probes | `EXTERNAL SURFACE` ✓ · the never-`\| head`/`\| tail` reading rule ✓ · `index MISSED its definition` ✓ |
| both files LF in an LF-pinned repo | byte count | `code-graph.py` CRLF=0 LF=387 · cookbook CRLF=0 LF=107 |
| the pipeline still runs | live query on the rust plane | returned `persist` / `fn` — 1 row |
| **the `newline=""` fix works on the live path** | line-ending count of the trace that query wrote | **CRLF=0, LF=13** — the first trace this host has written as LF |

The last row is the one that matters: the fix is proven by a before/after on the real writer, not by the
template diff alone. Every prior trace this host wrote was CRLF in the worktree (5 measured, most recent 5 of 128).

---

## The relay's CRLF premise — corrected

The directive asked for a re-stamp of "already-committed CRLF traces". Measured across all 128 tracked
`tree-query-*.json`: **121 `i/lf w/lf` · 5 `i/lf w/crlf` · 2 `i/none w/none`** — and `git show ":$f"` yields
**0 CR on every one of the five**. The committed content was already LF throughout; `.gitattributes`'
`* text=auto eol=lf` normalized each at `git add` time, which is what the warning announced.

`git add --renormalize` on those paths would stage nothing, so **no re-stamp is owed and the operator fork
dissolves**. The 196-CR figure reproduces exactly — on the worktree copy of this wrap's trace, which is what
was measured. Diagnosis and fix stand; only the claimed harm to history does not.

---

## Not changed, deliberately

- **`PostToolUse` stays rustfmt-only.** `grep -c clippy .claude/settings.json` → 0, before and after.
  Restoring clippy from the hooks-matrix default would regress a measured project invariant
  (CLAUDE.md Tier 1, 2026-09-02); against this **77 GB** `target/` a cold `clippy --fix` exceeds the 30 s
  hook timeout under `|| true`, yielding no signal and no error.
- **CLAUDE.md** — every `GENERATED:setup:*` block verified current against its upstream; no template delta
  exists this run, so Phase 1 wrote nothing. The backup was taken and is unused.
- **7 rule files · 11 docs · 2 harness scripts · the code reviewer** — preserved whole.
