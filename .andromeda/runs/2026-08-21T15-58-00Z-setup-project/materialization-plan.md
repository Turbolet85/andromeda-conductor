# Materialization Plan — Conductor (re-run)

**Run:** 2026-08-21T15-58-00Z-setup-project · **Style:** agent-driven · **Mode:** re-run (18 `GENERATED:setup` markers)
**Upstreams read:** input.md · architecture.md · security-plan · design-system · layout-templates · test-plan · obs-plan · a11y-plan · master-route.md
**Backup:** `.claude/backup/CLAUDE.md.pre-setup-2026-08-21T15-58-00Z`

Phases 1–6 read ONLY this file.

---

## Ground truth pinned this pass

| Fact | Value | Source |
|---|---|---|
| Workspace members | **9** | `Cargo.toml` + `architecture.md:156` |
| Active version | conductor-0.2.0 | master-route last `## ` heading + dir |
| Development Style | agent-driven | `architecture.md:224`, `:250` |
| Code-graph planes | rust + ts (both indexers on PATH) | root `Cargo.toml`; tracked `tsconfig.json` ×2 |
| Cache layout on disk | **per-plane** (`cache/rust/tree.db`, `cache/ts/tree.db`) | `ls .andromeda/cache/` |

---

## Tier 1 — CLAUDE.md

Regenerate all `GENERATED:setup:*`; preserve `USER:session-learnings` verbatim (lines 116–130 of the current
file, 13 curated entries).

- **overview** — stack one-liner + key dirs. Keep `crates/` = **9** (already correct).
- **modules** — the 9 crate-per-seam members, 1 line each. Unchanged (already correct at 9).
- **warnings** — top 10 universal invariants (below).
- **pointer-table** — 23 rows. **ONE ROW CHANGES:** the code-map row's DB path is a dead pointer
  (`.andromeda/cache/tree.db` does not exist; the DB is per-plane). Re-point to
  `.andromeda/cache/{plane}/tree.db` and name the plane argument. All other 22 rows validated by existence
  this pass — the `working-route.md` row is a derivation instruction (resolves to
  `conductor-0.2.0/working-route.md`), not a literal path, and is correct as written.
- **workflow** — arch key commands. Unchanged.
- **architecture** — 1–2 paragraphs. Unchanged.
- **imports** — `@.andromeda/architecture.md` · `@.andromeda/master-route.md` · `@.claude/session-handoff.md`. Unchanged.
- **deeper-topics** — docs + rules pointers. Unchanged.

Budget: currently 130/200. Expected delta ≈ 0.

### Universal warnings (top 10, severity security > a11y > obs > tests > design)

1. Scope law + trust boundary — every scenario carries a P-ID; no inbound listener of Conductor's own (`:4317` occupier is the sole deliberate bind, released on cleanup).
2. Accepted capability set is DATA (`contracts/pulse-capabilities.toml`); `check_sut_drift` + `check_scenario_backing` hold it; malformed/absent = `CoreError`, never `Blocked`, never a silent widen.
3. Verdict/error wall — outcomes are `Ok(Verdict/ReportState)`; `Result::Err` is harness faults only; never panic on child/transport input.
4. Preflight integrity — never silently downgrade; each of the FIVE named preconditions surfaces its own host-path-free string; a `declared-not-observable` term never blocks.
5. Supply chain — `Cargo.lock` committed + un-drifted; never `cargo build --release`/merge without cargo-audit (+ cargo-deny) green; toolchain ≥1.94.1, `tauri` ≥2.10.3.
6. Subprocess hardening — fixed program NAME via inherited `PATH`; `ANDROMEDA_PULSE_DATA_DIR` only via `.env(...)` after rejecting injection metacharacters.
7. Artifact hygiene — never leak absolute host paths or internal struct names into logs / report / `runs.db`; sanitize at the `anyhow` edge + tracing field-allowlist.
8. Self-obs never exports OTLP — the only OTLP is the PRODUCT stream to `:4317`; self-obs is `tracing` JSON, every line carrying `run_id`; zero unlogged panics.
9. Wall-clock from `std::time` — never tokio's virtual clock (journal-relative SLO math).
10. Status is never color-alone — text label + glyph (desktop) or `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` (cli).
11. Determinism is the bar — same scenario+seed ⇒ same stream shape; no nextest retries.

_(11 shipped: the existing block carries all 11 and each is stack-anchored + universal. Rejected candidates —
audit: per-surface CLI/webview bans → design-system §Per-Surface Bans (path-scoped, Tier 2 `frontend.md`);
axe/contrast tooling specifics → `a11y.md`; nextest fixture mechanics → `testing.md`; garde `dive`-not-`skip`
→ `security.md` (path-scoped enforcement, already there).)_

---

## Tier 2 — .claude/rules/

Six files, all present. Regenerate above `## Session Additions`; preserve that section onward verbatim.

| File | Scope | Session Additions preserved |
|---|---|---|
| `security.md` | unconditional (no `paths:`) | yes |
| `testing.md` | test paths | yes |
| `observability.md` | obs paths | yes |
| `a11y.md` | webview UI paths | yes |
| `frontend.md` | `crates/conductor-tauri/ui/**` | yes |
| `verification-harness.md` | agent-driven harness | yes |

No new rule files. `migrations` N/A (no migration framework); `api` N/A (no HTTP surface); `events` N/A
(no broker).

**Content note:** all six were re-derived by the 2026-08-21 wrap cascade (mtimes 13:15–13:19) and carry the
current amendments (five preconditions, garde sibling-boundary, `budget_ms`). Regeneration is a no-op
refresh; the value of this pass is the docs tier below.

---

## Tier 3 — .claude/docs/

**Core (5)** — regenerate. **Summaries (5)** — regenerate. `session-learnings.md` — wrap territory, untouched.

**Per-module `services/{module}.md`: N/A — deliberate, not a gap.** The 9 crates are compiler-enforced seams
inside ONE deployable, not a fleet: `architecture.md` §Project Intent states "Template patterns: N/A — single
harness, not a fleet of services", and §Growth model calls it a modular monolith. CLAUDE.md's `## Modules`
section already covers all 9 at the right altitude. No `services/` dir is created.

### Corrections to apply (cascade gaps — leaf diverged from a CORRECT master)

| # | File | Stale text | Canonical source |
|---|---|---|---|
| 1 | `conventions.md:9` | "`crates/` (8 seam crates)" | `architecture.md:156` — 9 members |
| 2 | `conventions.md:21` | "inbound MCP read-back (rmcp, …)" | `architecture.md:49`/`:73` — hand-rolled JSON-RPC, rmcp removed 2026-06-27 |
| 3 | `conventions.md:17` | timestamps "stored as … integer-millisecond journal offsets … (NOT ISO strings)" — **inverted** | `architecture.md` §Conventions — TEXT RFC-3339; the ms value is the separate `latency_ms` INTEGER column |
| 4 | `conventions.md` §Data model | no three-table structure, no per-table nullability | `architecture.md` §Conventions + §Occupied Resources — `runs` / `run_envelope` / `run_check`; `run_check` INTEGER NOT NULL, `budget_ms` NULL |
| 5 | `conventions.md` §Config | `[phases.fault]` + `[[expected]] budget_ms` absent | `architecture.md` §Conventions Config conventions |
| 6 | `gotchas.md:11-12` | "rmcp client negotiates down" | `architecture.md` §Established Decisions — negotiation reads the `initialize` result's `protocolVersion` |
| 7 | `gotchas.md:19` | "filters incidents by `workspace_root = data_dir`" | `architecture.md` §Standard Contracts — filtered by the **`workspace`** column, *not* `workspace_root` |
| 8 | `commands.md:14` | cleanup removes "`runs.db` row" (singular) | `test-plan.md` §3 cleanup — all THREE tables (`runs`, `run_check`, `run_envelope`) |
| 9 | `tests-summary.md` | no `run_check` / `CheckRecord` / `budget_ms` | `test-plan.md` §3 Status endpoint shape + Log format (amended 2026-08-21) |
| 10 | `workflow.md` | pins `conductor-0.1.0/working-route.md`; says `/new-session` | active version is 0.2.0; skill is `/andromeda-new-session` — make version-agnostic |

### Deliberately NOT corrected (leaf faithfully inherits an owned-stale master)

- `obs-summary.md:7` "8 crates + 2 surfaces" ← `obs-plan.md:25` — the KNOWN count, **CARRY-owned** by the
  Epoch-6 *Dependency polish* route entry. Masters are never setup's territory; the leaf re-derives what the
  master says.
- `tests-summary.md:45` "rmcp stub" ← `test-plan.md:113`/`:52` — **route-owned** by the Epoch-5
  *Cross-surface envelope parity* entry ("with the stale rmcp wording reconciled"). test-plan carries 25 rmcp
  mentions; obs-plan 4; security-plan 2. `architecture.md` is correct at all 3 of its sites.

---

## Agent harness (Phase 4) — PRESERVE

`scripts/agent-run.sh` (7087 B, exec bit set) + `agent-run.ps1` (7848 B) both present and project-evolved
(they carry `ensure_frontend` and derive the `boot` budget from `contracts/pulse-run-contract.toml`, neither
of which the template renders). **Only-if-missing rule → preserve both, no overwrite.** Drift vs a fresh
render is noted in the Phase 7 card, not merged.

`.claude/rules/verification-harness.md` → regenerate above `## Session Additions` (Phase 2 convention).

---

## Hooks / reviewer / gitignore (Phase 5)

- **Code reviewer:** `.claude/agents/code-reviewer.md` present (rust). Regenerate from
  `agent-templates/code-reviewer-rust.md`.
- **Hooks:** `.claude/settings.json` already carries the UTF-8 `env` block (`PYTHONUTF8`,
  `PYTHONIOENCODING`) applied 2026-08-21, plus PreToolUse generated-dir block and PostToolUse
  rustfmt + `clippy --fix`. **Merge, never replace** — the existing `env` and hook bodies are preserved as-is.
  No type-checker hook to add (clippy covers it for Rust).
- **.gitignore:** verified to carry `.claude/backup/`, `.claude/settings.local.json`, `target`. Confirm
  `.andromeda/cache/` + `scripts/__pycache__/` present; append the rust fragment idempotently.

---

## Code-graph pipeline (Phase 6) — THE HEADLINE

Per-file, only-if-missing, **with the drift rule applied to the three project-evolvable files.**

| File | Present | Template | Action |
|---|---|---|---|
| `scripts/code-graph.py` | 196 L, **Jun 18** | 348 L, **Aug 21 15:16** | **DRIFT → back up + update** |
| `scripts/code-graph-views.sql` | 46 L | 47 L | **DRIFT → back up + update** |
| `scripts/code-graph-cookbook.md` | 52 L | 77 L | **DRIFT → replace above the `Project-specific query learnings` marker, tail preserved verbatim** (tail = 1 line, the marker comment; no accumulated learnings) |
| `scripts/scip_pb2.py` | present | vendored | preserve (only-if-missing) |
| `scripts/requirements.txt` | present | vendored | preserve (only-if-missing) |

### Why this is a functional break, not cosmetic drift

The shipped `scripts/code-graph.py` is the ORIGINAL single-plane version: it resolves
`db = .andromeda/cache/tree.db` (lines 100, 140). That path **does not exist** — the on-disk cache is
per-plane (`cache/rust/tree.db` 2.4 MB, `cache/ts/tree.db` 1.3 MB, each with `built.fp`/`built.head`), and the
aggregate `.refresh-done` reads `rust ok 26s 2189/10185` / `ts ok 1s 247/377` — a two-line per-plane format
the shipped script cannot produce (it writes a single `done {secs}s {nodes}/{edges}`). The cache was
therefore built by the plane-aware version while the project's own script stayed behind.

Consequence if left: `python scripts/code-graph.py query …` — the exact command CLAUDE.md's pointer table
gives every agent — finds no DB, declares it absent/stale, rebuilds in the OLD flat layout, and overwrites
the per-plane `.refresh-done`. It also takes no `plane` argument, which the current callers pass.

The project's script carries **zero** project-specific strings (verified by grep for
`conductor|pulse|andromeda-pulse`) — it is pipeline-generic, so replacing it loses nothing.

**Class note (addendum expectation 1):** this IS a dead pointer row, and it is NOT Pulse's context/-era
class. Conductor was born on v3 and has no such history. Here the pointer row was *correct when written*
(Jun 18, flat layout) and died when the pipeline moved to per-plane while this project's copy did not — the
pointer and the script are stale from ONE cause, and the pointer row is the only surface that exposed it,
because Check 3 guards `@`-imports only, never pointer-table paths.

---

## Seeded artifacts (Phase 6 cont.) — all present, preserve

`state.yaml` (schema 3, lean) · `session-handoff.md` · `drift-base.md` · `playbook.md` ·
`session-learnings.md` · `.andromeda/cache/` (exists, gitignored).

---

## Out of scope for this run (recorded, not acted on)

- `.andromeda/runs/_wrap_tmp/` — committed residue from a 2026-08-15-era wrap. `runs/` territory, not
  setup's. Left in place.
- Master edits of any kind (the "8 workspace crates" counts, the rmcp wording) — masters are never setup's
  territory.
- **NEW find to relay:** `test-plan.md:25` also says "8 workspace crates" — a THIRD stale site the
  CARRY names neither of (it names `obs-plan.md` :25 and :645 only). Surfacing, not fixing.
