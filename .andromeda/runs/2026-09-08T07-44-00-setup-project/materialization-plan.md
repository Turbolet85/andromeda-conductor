# Materialization Plan — Conductor

_Phase 0 checkpoint. Emitted 2026-09-08T07:44:00Z. Phases 1–6 read ONLY this file._

**Run mode:** RE-RUN (18 `GENERATED:setup` markers found in `CLAUDE.md`; backed up to
`.claude/backup/CLAUDE.md.pre-setup-2026-09-08T07-44-00`).
**Development Style:** `agent-driven` (architecture.md §Cross-cutting Patterns:240, §Inherited Defaults:266).
**Trigger:** absorb a pipeline-template change — the CLAUDE.md template no longer imports
`.andromeda/master-route.md`.

**Upstreams read (9/9):** `input.md` · `architecture.md` (11 headings, tail verified) ·
`security-plan.md` · `design-system.md` · `layout-templates.md` · `test-plan.md` · `obs-plan.md` ·
`a11y-plan.md` · `master-route.md` (read structurally: 2 version headings, 115 records, 0 `pending`).

---

## Standing finding that governs Phases 2, 3 and 5

The `references/` templates are **generic scaffolds** carrying placeholder blocks
(`{Extract from architecture.md …}`) and stack-agnostic web/API content. The project's materialized
leaves are **fully project-specific renders** produced by prior runs of this pipeline from the six
plans, and kept current since by wrap's cascade (four leaves re-derived last session).

A verbatim template re-render would therefore be a **regression, not a refresh**. Measured instances:

- `rules-templates/verification-harness.md` prescribes a daemon `boot`, a PID file, a heartbeat and
  `TIDELINE_DATA_DIR`. Conductor has **no daemon and no PID file** (test-plan §3: "PID file — N/A").
  The materialized `.claude/rules/verification-harness.md` carries the explicit adaptation
  *"Conductor has NO daemon and NO inbound listener … Do not reintroduce daemon/PID/endpoint
  machinery"* — the exact clause a verbatim re-render would delete.
- `rules-templates/security.md` is generic (constant-time compare, rate limiting, XSS, `npm audit`).
  The materialized rule is Conductor's boundary set (garde + the load-path `check_*()` arm, rusqlite
  bound parameters, the fixed-NAME sidecar spawn, the four `CONDUCTOR_*` handle classes, the FIVE
  preflight preconditions).
- `docs-templates/stack.md` is a pure `{Extract …}` skeleton.
- `agent-templates/code-reviewer-rust.md` is generic Rust; the materialized agent keeps every
  template checklist item (condensed) and **adds** a `## Conductor-specific checks` section
  (verdict/error wall · `std::time` journal stamps · scope law · MCP hand-rolled JSON-RPC ·
  capability-set-as-data · self-obs never exports OTLP · artifact hygiene).

**Rule applied in Phases 2/3/5:** the substitution step is authoritative and already applied. Where a
leaf is present, project-correct and consistent with its master, it is the materialized form and is
preserved. This run changes only what the pipeline-template change and the plans actually require.

---

## Tier 1 — CLAUDE.md

Budget ≤200 lines. Nine `GENERATED:setup*` regions + one preserved `USER:session-learnings`.

| Block | Action | Basis |
|---|---|---|
| `setup:overview` | UNCHANGED | arch §Project Intent + §Stack + §Infrastructure Patterns directory tree; 9 members, 5 key dirs |
| `setup:modules` | UNCHANGED | arch §Occupied Resources — Crate names (9 workspace members) |
| `setup:warnings` | UNCHANGED | re-derived below; 11 invariants, template allows 5–12 |
| `setup:pointer-table` | UNCHANGED | 22 rows ≥ 10; `Build plan / chunk route` row RETAINED — it is the navigation that replaces the dropped import, and it already complies with the template's no-baked-version-path rule (cites `master-route.md` literally, describes `working-route.md` as derived) |
| `setup:workflow` | UNCHANGED | arch §Workflow Key commands (5 commands) |
| `setup:architecture` | UNCHANGED | arch §Design Philosophy (2 paragraphs) |
| **`setup:imports`** | **CHANGE — 3 lines → 2** | template: `@.andromeda/architecture.md` + `@.claude/session-handoff.md`. `@.andromeda/master-route.md` is DROPPED |
| **maintainer-note comment** | **ADD** | present in the template INSIDE `GENERATED:setup` (between the imports block and `## Deeper Topics`); absent from Conductor's current file. Block-level HTML comments are stripped before context injection, so it costs zero runtime tokens, and it documents the very change this run absorbs |
| `setup:deeper-topics` | UNCHANGED | 7 rule files + 5 core docs + 5 summaries + session-learnings — matches what exists on disk |
| `USER:session-learnings` | **PRESERVE VERBATIM** | spliced byte-for-byte from the backup, never retyped |

### Universal warnings — re-derivation (audit)

The 11 shipped invariants each re-derive from a plan's anti-patterns or arch §Cross-cutting, in the
mandated severity order security > a11y > obs > tests > design:

1. Scope law + trust boundary — security §Universal; test-plan §11 Universal; arch §Cross-cutting
2. Accepted capability set is DATA — security §Input; arch §Established Decisions
3. Verdict/error wall — security §Universal; arch §Cross-cutting
4. Preflight integrity — security §Universal (the FIVE named preconditions)
5. Supply chain — security §Universal + §Dependency Security
6. Subprocess hardening — security §Input + §Code Patterns
7. Artifact hygiene — security §Logging; obs §Logs
8. Self-obs never exports OTLP — obs §Universal + §Telemetry Strategy
9. Wall-clock from `std::time` — security §Logging; obs §Project-specific; test-plan §11 Test Data
10. Status is never color-alone — a11y §Visual (SC 1.4.1); design §Universal Bans
11. Determinism is the bar — test-plan §10 zero-flakiness; arch §Cross-cutting

Rejected candidates (audit trail — each is path-scoped, so it belongs to Tier 2, not Tier 1):
garde `dive`-never-`skip` (→ `rules/security.md`) · rusqlite bound parameters (→ `rules/security.md`) ·
no `tabindex > 0` / focus-ring rules (→ `rules/a11y.md`) · nextest zero-retry + mutation-survivor
disposition (→ `rules/testing.md`) · `RUST_LOG` per-target-replaces-default (→ `rules/observability.md`) ·
tokens on `:root` not `@theme` (→ `rules/frontend.md`) · no `framer-motion` / expression-level 0.3
(→ design, path-scoped) · Windows/MSYS shell recipes (→ `rules/host-win32.md`).

---

## Tier 2 — `.claude/rules/` (7 files, all present)

Conditional set resolves to exactly the 7 on disk. `security.md` (always) · `testing.md` +
`observability.md` (plans present) · `a11y.md` (a11y plan + a UI surface) · `verification-harness.md`
(agent-driven) · `frontend.md` (arch marks a React/Tailwind webview subtree) · `host-win32.md`
(the GENERATING host is Windows — serves the host, not the stack).
NOT rendered: `migrations.md` (arch: "ORM / migrations: None — raw SQL"), `api.md` (no HTTP surface),
`events.md` (arch: "Message broker: N/A").

**Action: preserve all 7** per the standing finding. Each was verified to carry (a) parseable
frontmatter or a deliberate no-frontmatter unconditional load, (b) its authoritative-source
citation, (c) a trailing `## Session Additions`. `## Session Additions` content is wrap territory and
is untouched.

---

## Tier 3 — `.claude/docs/` (11 files, all present)

5 core (`stack` · `conventions` · `commands` · `gotchas` · `workflow`) + 5 summaries
(`security` · `design` · `tests` · `obs` · `a11y`) + `session-learnings.md`.
**No `services/{module}.md`** — the 9 crates are compiler-enforced seams of ONE deployable, and arch
§Project Intent states "Template patterns: N/A — single harness, not a fleet of services."

**Action: preserve all 11.** `session-learnings.md` is wrap territory (create-only-if-missing; it
exists). The remaining 10 are project-specific renders consistent with their masters.

---

## Agent harness (Phase 4)

`scripts/agent-run.sh` (18 343 B / 327 lines) and `scripts/agent-run.ps1` (20 894 B / 346 lines) both
exist and both **DRIFT** from their templates (fenced bodies 3 748 B / 4 271 B). The drift is the
project's own evolution — `--unit`/`--integration`/`--e2e`/`--live` stage flags, `ensure_frontend`,
the run-contract-derived boot budget, the printed-verdict a11y assertions, the non-recursive
live-suite capture wipe.

**Action: PRESERVE (only-if-missing rule). Back up both, surface the drift in P7, regenerate neither.**
The evolved scripts are the truth. `.claude/rules/verification-harness.md` is a rule file and follows
the Phase 2 rule (preserve, `## Session Additions` untouched).

---

## Hooks + code-reviewer + gitignore (Phase 5)

**Hooks (`.claude/settings.json`):** resolves to the same render the file already carries. Precedence
walk: obs-plan §3 names no formatter/linter/type-checker → arch §Stack names clippy + rustfmt →
hooks-matrix Rust rows. Formatter `rustfmt "$f"`; **linter: none at write time** (the matrix's Rust row
— clippy has no file scope; it stays a GATE); type-checker skipped (Rust has none). Both PreToolUse
guards (generated-directory block, Bash transport guard) and the unconditional
`env: {PYTHONUTF8, PYTHONIOENCODING}` block are already present verbatim.
**Action: no change** — a prior run already absorbed the 2026-09-05 matrix. `rustfmt.toml`
(`edition = "2024"`) exists → only-if-missing, preserved.

**Code-reviewer:** `.claude/agents/code-reviewer.md` is the Rust template correctly tailored to
Conductor (no template item dropped; a `## Conductor-specific checks` section added).
**Action: preserve**, back up.

**`.gitignore`:** all required entries present (`.claude/backup/`, `.claude/settings.local.json`,
`/.andromeda/cache/`, `scripts/__pycache__/`, plus the Rust fragment `/target/`, `**/*.rs.bk`,
`*.iml`). **Action: no change** (idempotent append is a no-op).

---

## Phase 6 — operational artifacts (all only-if-missing; all present)

| Artifact | State |
|---|---|
| `scripts/code-graph.py` · `code-graph-views.sql` · `scip_pb2.py` · `requirements.txt` · `code-graph-cookbook.md` | present, **byte-identical to templates** — no drift, no backup, no rebuild note |
| `.andromeda/cache/` | present (gitignored); both planes built (`rust/tree.db`, `ts/tree.db`) |
| `.andromeda/state.yaml` | present, schema 3, lean fields only |
| `.claude/session-handoff.md` | present |
| `.andromeda/drift-base.md` · `.andromeda/playbook.md` | present |
| `.claude/docs/session-learnings.md` | present |

**Plane detection (two-source rule):** planning truth — arch primary language Rust + design-system
§Surface `desktop-webview` → `rust` + `ts`; manifest scan — root `Cargo.toml` present, tracked
`tsconfig.json` ×2 under `crates/conductor-tauri/ui/`. Both planes real; both indexers on PATH
(`rust-analyzer`, `scip-typescript`); `duckdb` + `protobuf` importable. No forward notes.

---

## Net change set for this run

1. `CLAUDE.md` — `setup:imports` 3 lines → 2 (drop `@.andromeda/master-route.md`); add the template's
   maintainer-note comment inside `GENERATED:setup`. Every other GENERATED block byte-unchanged;
   `USER:session-learnings` spliced verbatim.
2. `.claude/backup/` — CLAUDE.md + the two agent-run scripts + code-reviewer.md.
3. Everything else: verified, preserved, unchanged.
