# Materialization Plan — Conductor (setup-project RE-RUN)

**Run:** `.andromeda/runs/2026-09-02T21-45-00-setup-project/`
**Mode:** RE-RUN (18 `GENERATED:setup` markers found; CLAUDE.md backed up to `.claude/backup/CLAUDE.md.pre-setup-2026-09-02T21-45-00Z`)
**Development Style:** `agent-driven` (architecture.md §Cross-cutting Patterns:232)
**Generating host:** Windows (win32) — activates the `host-win32` rule branch
**Trigger:** pipeline-template absorption (operator directive) — hooks I/O contract + the code-graph triple. **No master changed structurally.**

Phases 1–6 read ONLY this file.

---

## Re-run posture (what this run is and is not)

This is **template absorption**, not a re-derivation of project content. Two upstream template
improvements landed that the seeded files predate:

1. **Hooks I/O contract.** `$CLAUDE_FILE_PATH` / `$CLAUDE_TOOL_INPUT` do not exist in the runtime
   (0 occurrences in 2.1.257). The seeded `.claude/settings.json` reads both, so its PreToolUse
   write-block **never blocked** (it also exits 1 — advisory — where blocking needs exit 2) and every
   PostToolUse row ran `rustfmt`/`clippy` on an **empty path**, swallowed by `|| true`. Phase 5
   re-renders in the stdin-JSON + exit-2 form and adds the two PreToolUse Bash guards.
2. **Code-graph triple.** `code-graph.py` / `code-graph-views.sql` / `code-graph-cookbook.md` all
   drift from their templates. The graph now derives `name`/`kind` at build and queries key on
   `callee_name`; the old cookbook idiom `LIKE '%/name().%'` misses Rust names after `]`, `#`, or at
   crate root — the mechanism behind 7 false "leaf" answers in Epochs 4–5 (`read_envelope` really has
   5 call sites, `for_record` 14). Phase 6's drift rule proposes all three; **accept**.

**Standing preservation contract for this run** — the project-adapted bodies of `.claude/rules/*` and
`.claude/docs/*` are the correct output of prior substitution against these same plans, and the plans
have been kept current by wrap's cascade (5 masters amended 2026-09-02). Regeneration therefore
**refreshes against the plans in context and preserves every `## Session Additions` tail verbatim** —
it never reverts a project-adapted rule body to the generic web-centric template text (that would
replace measured invariants with irrelevant CSRF/XSS bullets — a regression, not an absorption).

---

## Tier 1 — CLAUDE.md

**Budget:** ≤200 lines. Current 131. Target ≈131 (+2 for the new rule pointer). Preserve
`USER:session-learnings` (lines 116–131) **verbatim** — the operator has explicitly deferred its
2026-08-08 code-graph rule correction to the next wrap's curation.

| Block | Action | Source |
|---|---|---|
| `setup:overview` | regenerate | arch §Project Intent + §Stack + §Infrastructure Patterns tree |
| `setup:modules` | regenerate | arch §Occupied Resources — Crate names (9 members) |
| `setup:warnings` | regenerate | top-10 below |
| `setup:pointer-table` | regenerate (21 rows) | arch §Standard Contracts + specialist anchors + code-graph |
| `setup:workflow` | regenerate | arch §Workflow key commands |
| `setup:architecture` | regenerate | arch §Design Philosophy |
| `setup:imports` | unchanged | `architecture.md` · `master-route.md` · `session-handoff.md` |
| `setup:deeper-topics` | regenerate — **add `host-win32.md`** to the rules list | Phase 2 output |
| `USER:session-learnings` | **PRESERVE VERBATIM** | wrap territory |

### Modules (9 workspace members — arch §Occupied Resources)

- `conductor-core` — runtime-agnostic engine library every other crate depends on (shared `Verdict`/`ReportState`, scenario model)
- `conductor-timeline` — deterministic seeded phase scheduler on `tokio::time`
- `conductor-emit` — OTLP raw-type emission primitives + exception events + the blake3 fingerprint primitive
- `conductor-faults` — fault helpers: ramps, silence, port-occupier, fingerprint-storm
- `conductor-verify` — MCP read-back client (hand-rolled JSON-RPC over sidecar stdio), preflight gate, verdict logic
- `conductor-report` — JSONL journal + Markdown run report + `runs.db` (rusqlite) storage seam
- `conductor-run` — run composition root library (preflight + scenario execution + `persist` + `drive_run` + the fault-phase occupier guard)
- `conductor-cli` — `agent-run` binary, headless source of truth + release gate
- `conductor-tauri` — Tauri 2 GUI bin (commands + live-counter `Channel`; React 19 webview)

### Universal warnings (top 10 — selected)

Severity priority security > a11y > obs > tests > design. Every entry holds for **every file**;
path-specific rules go to Tier 2.

1. **Scope law + trust boundary** — every scenario carries a Pulse P-ID; SHIPPED binaries open no inbound listener (the `:4317` port-occupier is the sole deliberate bind); the dev-only webview a11y driver ports `4444`/`4445` live and die with the harness across all three suite families. *(security §Universal · arch §Scope law)*
2. **Accepted capability set is DATA** — `contracts/pulse-capabilities.toml`, never a compile-time constant; malformed/absent is a named `CoreError`, never `Blocked`, never a silent widen; `check_sut_drift` + `check_scenario_backing` hold both axes. *(arch §Accepted Capability Set)*
3. **Verdict/error wall** — outcomes are typed VALUES (`Ok(Verdict/ReportState)`); `Result::Err` is harness-faults only; malformed child/transport input becomes typed `Blocked`/`Fail`, never a panic. *(security §Universal · obs §Error Reporting)*
4. **Preflight integrity** — never silently downgrade a failed preflight; each of the FIVE named preconditions surfaces its own host-path-free string; a `declared-not-observable` term never blocks. *(security §Universal)*
5. **Supply chain** — `Cargo.lock` committed + un-drifted; never `cargo build --release` or merge without `cargo-audit` (+ `cargo-deny`) green; toolchain ≥1.94.1, `tauri` ≥2.10.3. *(security §Dependency Security)*
6. **Subprocess hardening** — spawn `andromeda-pulse-mcp` as a fixed hard-coded program NAME through the inherited `PATH`; pass `ANDROMEDA_PULSE_DATA_DIR` only via `.env(...)` after rejecting injection metacharacters. *(security §Code Patterns)*
7. **Artifact hygiene** — never leak absolute host paths or internal struct names into logs / run-report / `runs.db`; sanitize at the `anyhow` edge + the tracing field-allowlist. *(security §Logging · obs §PII Scrubbing)*
8. **Self-obs never exports OTLP** — the only OTLP is the PRODUCT fault stream to Pulse `:4317`; self-observation is `tracing` JSON to stderr/file, every line carrying `run_id`; zero unlogged panics. *(obs §Universal)*
9. **Wall-clock from `std::time`** — journal/report stamps never from tokio's virtual clock. *(security §Logging · tests §Test Data)*
10. **Status is never color-alone** — every Verdict/ReportState carries text label + glyph (desktop) or ASCII prefix (cli). *(a11y §Visual — SC 1.4.1)*
11. **Determinism is the bar** — same scenario+seed ⇒ same stream shape; stubs only in tests; zero flakiness, no nextest retries. *(tests §Quality)*

*(Rejected for Tier 1 — audit: garde `dive`-not-`skip` (path-scoped → `rules/security.md`); axe `withTags` scope (→ `rules/a11y.md`); nextest `-p` selection (→ `rules/testing.md`); the `RUST_LOG` target-replaces-default rule (→ `rules/observability.md`); Tailwind `:root`-not-`@theme` (→ `rules/frontend.md`); the cargo-mutants `-f` workspace-root rule (→ `docs/gotchas.md`) — each is real but does not hold for every file.)*

### Pointer table (21 rows — unchanged in shape, refreshed)

Retains the existing rows. The code-graph row keeps its **project-accurate** form —
`plane REQUIRED` (Conductor has **two** detected planes, rust + ts), which is stricter and more correct
than the template's "plane needed only when several are detected". No version-workspace path is baked:
the working-route is cited as *the active version's* (master-route's last `## {project}-{version}`
heading).

---

## Tier 2 — .claude/rules/

**7 files** (6 existing refreshed + 1 new). Every file preserves content from `## Session Additions`
onward **verbatim** (measured tails: a11y 1 · frontend 9 · observability 3 · security 7 · testing 32 ·
verification-harness 21 lines).

| File | Status | `paths:` frontmatter | Authority |
|---|---|---|---|
| `security.md` | refresh | **none** (unconditional) | security-plan §Input Validation / §Dependency Security / §Error Handling / §Security Anti-Patterns |
| `testing.md` | refresh | `**/*.rs`, `**/tests/**`, `.config/nextest.toml` | test-plan §3/§4/§5/§10/§11 |
| `observability.md` | refresh | `**/*.rs`, `**/obs*.rs` | obs-plan §3/§4/§6/§11 |
| `a11y.md` | refresh | `crates/conductor-tauri/ui/**` | a11y-plan §3/§4/§6/§11 |
| `frontend.md` | refresh | `crates/conductor-tauri/ui/**` | design-system §Tokens · layout-templates §desktop-webview |
| `verification-harness.md` | refresh | `scripts/agent-run.*`, `crates/conductor-cli/**`, `crates/conductor-run/**` | test-plan §3 (5-command) · arch §Standard Contracts |
| **`host-win32.md`** | **NEW — template addition** | none (host-serving, inert off-Windows) | `rules-templates/host-win32.md` (verbatim + `## Session Additions`) |

`host-win32.md` is rendered because the **generating host** is Windows. It serves the host, not the
stack: MSYS arg-conversion, `grep -P` locale death, the zero-is-healthy count-probe `|| true` rule, the
JSON/document transport ban, `$?`-before-pipe, heredoc column-0 terminators, and the **6500-byte Bash
transport cut** — the same limit Phase 5's second PreToolUse guard now enforces mechanically. A project
later moved off Windows keeps it inert.

Conditional rules NOT rendered: `api.md` (no HTTP surface), `events.md` (no broker),
`migrations.md` (raw SQL, no migration framework) — each absent by an arch-stated N/A.

---

## Tier 3 — .claude/docs/

**10 files, no `services/`** (the crate map lives in CLAUDE.md §Modules; no per-service split).

- **Core 5:** `stack.md` (mirrors arch §Stack) · `conventions.md` · `commands.md` · `gotchas.md` · `workflow.md`
- **Summaries 5:** `security-summary.md` · `design-summary.md` · `tests-summary.md` · `obs-summary.md` · `a11y-summary.md` (~100 lines each, each ending "Full plan: …")
- `session-learnings.md` — **EXISTS (72 KB), wrap territory — create-only-if-missing ⇒ SKIP**

All ten were refreshed by wrap's cascade on 2026-09-02 (six leaves re-derived). They are already
faithful to the plans in context; this run preserves them and touches only what the plans moved.

---

## Agent harness (Phase 4) — PRESERVE

Development Style is `agent-driven`, so the harness is in scope. Both scripts EXIST and are
**project-evolved** beyond a fresh render:

- `scripts/agent-run.sh` (7798 B, exec bit set on-host) · `scripts/agent-run.ps1` (8874 B)
- Evolved surface a fresh render would not carry: the `ensure_frontend` pre-build step, the `--e2e`
  stage (release build with `--features tauri/custom-protocol` → fixture seeding under
  `CONDUCTOR_E2E_SEED_DIR` → tauri-driver `wdio run`), and the `boot` wall-clock budget **derived from
  `contracts/pulse-run-contract.toml`** (`warmup_ms/1000 + poll + margin`, missing term ⇒ hard exit 2,
  `CONDUCTOR_PREFLIGHT_TIMEOUT` clamps UP to the contract floor).

**Action: preserve (skip), back up for merge traceability, surface the drift in the Phase 7 card.**
Per the operator's directive the project's evolved versions stand. `verification-harness.md` is a rule
file and follows the Phase 2 preserve-Session-Additions convention.

---

## Hooks (Phase 5) — RE-RENDER

`.claude/settings.json` — the substantive change of this run.

**Preserve:** the existing `env` block (`PYTHONUTF8: "1"`, `PYTHONIOENCODING: "utf-8"`) — already
exactly the mandated block; merged, never replaced. No other user-managed keys present.

**Replace:** every hook command, in the stdin-JSON prologue + exit-2 form.

| Hook | Matcher | Change |
|---|---|---|
| PreToolUse — generated-dir write block | `Edit\|MultiEdit\|Write\|NotebookEdit` | reads `.tool_input.file_path` from **stdin**; **exit 2** + stderr (was exit 1 on an empty `$CLAUDE_TOOL_INPUT`, so it never blocked); matcher gains `NotebookEdit`; `timeout: 5` |
| PreToolUse — Bash transport guard | `Bash` | **NEW** — denies a >6500-byte command on MINGW/MSYS/CYGWIN, and a `cat`/`tee` heredoc with a file target; `timeout: 5` |
| PostToolUse — formatter | `Edit\|MultiEdit\|Write` | `rustfmt "$f"` with `$f` from stdin JSON (was an empty path); `timeout: 30` |
| PostToolUse — linter | `Edit\|MultiEdit\|Write` | `cargo clippy --fix --allow-dirty --allow-staged -- -D warnings`, gated on `$f` matching `*.rs` from stdin JSON; `timeout: 30` |

Type-checker: **skipped** — Rust has no separate type-checker hook (hooks-matrix). Tool picks follow
precedence rule 2 (arch §Stack: rustfmt + clippy); obs-plan names no formatter/linter, so rule 1 is silent.
Every command opens with `command -v jq >/dev/null 2>&1 || exit 0` and messages stay ASCII-only.

### Operator ruling at Phase 7 (2026-09-02) — the clippy PostToolUse row is DROPPED

Rendered, reviewed, and removed on the operator's decision. **rustfmt stays; `cargo clippy --fix` does
not run at write time.** The reasons are measured properties of this project, not preference, and they
are recorded here so a future re-run does not silently restore the row from the matrix default:

1. **Redundant** — clippy already runs as a plan gate and again at wrap's light gate, so a write-time
   invocation adds no coverage.
2. **Scope violation** — `--fix` is workspace-wide, so it can rewrite files OUTSIDE the chunk's declared
   modify-set mid-implement.
3. **Races Read-before-Edit** — a hook that rewrites a file after the tool call invalidates the harness's
   file-state tracking for any file the agent has read but not yet edited.
4. **Silently useless on a cold cache** — this workspace's `target/` is ~23 GB; a cold `clippy --fix`
   exceeds the 30 s hook timeout and aborts under `|| true`, producing no signal and no error.

The matrix's Rust linter row (`cargo clippy --fix --allow-dirty -- -D warnings`) therefore does **not**
apply to this project. PostToolUse ships exactly ONE row (rustfmt, `$f`-gated on `*.rs`).
Post-ruling smoke: **12/12** (both PreToolUse guards blocking at exit 2; the rustfmt row no-op on a
non-Rust path and on an empty path).

## Code reviewer (Phase 5)

`.claude/agents/code-reviewer.md` from `agent-templates/code-reviewer-rust.md`, project name substituted.
Exists with correct frontmatter (`name` / `description` / `tools: Read, Glob, Grep` / `model: sonnet`) — regenerate in place.

## .gitignore (Phase 5)

Idempotent append only. Base ignores all present (`.claude/backup/`, `.claude/settings.local.json`,
`/.andromeda/cache/`, `scripts/__pycache__/`) plus the rust fragment (`/target/`, `**/*.rs.bk`) and the
project's own `/runs/`, `/logs/`, `.env*`, `crates/conductor-tauri/ui/logs/`. **Expected: no change.**

---

## Code-graph pipeline (Phase 6) — DRIFT: 3 of 5 files

Plane detection (integrity-protocol two-source rule): **planning truth** — arch §Stack primary language
Rust + design-system §Surface naming a desktop-webview; **UNION manifest scan** — root `Cargo.toml`
(rust) + tracked `crates/conductor-tauri/ui/tsconfig.json` and `ui/test/tsconfig.json` (ts). **Two planes.**
Host tools all present: `rust-analyzer` ✓ · `scip-typescript` ✓ · python `duckdb`+`protobuf` ✓ — no WARN.

| File | Current md5 | Template md5 | Action |
|---|---|---|---|
| `scripts/code-graph.py` | `2a43d840` | **`6a7a59aa`** | **DRIFT → back up + replace** |
| `scripts/code-graph-views.sql` | `18dcfacd` | **`26bb460c`** | **DRIFT → back up + replace** |
| `scripts/code-graph-cookbook.md` | `af7c07ae` | **`f41c1f86`** | **DRIFT → back up + replace above marker** |
| `scripts/scip_pb2.py` | `b13a00ca` | (vendored) | present ⇒ preserve |
| `scripts/requirements.txt` | `48fae1bb` | `48fae1bb` | present + identical ⇒ preserve |

All three expected md5s match the operator's stated values. **Cookbook tail check: the marker
`<!-- Project-specific query learnings accumulate below via wrap curation. -->` sits at line 77 with
ZERO lines after it** — confirmed by measurement, matching the operator's statement — so the
above-marker replacement is a full replacement with an empty preserved tail. The template's own marker
(line 91) likewise terminates the file.

**Expected consequence, not a defect:** a replaced `code-graph-views.sql` makes **each plane rebuild
once at its next query** (a `built.views` sidecar appears). Two planes ⇒ two one-time rebuilds. The DB
is NOT built here (`.andromeda/cache/` is gitignored and stays so).

## Session / operational artifacts (Phase 6) — all present ⇒ SKIP

`state.yaml` (schema 3, lean, parses) · `session-handoff.md` (4781 B) · `drift-base.md` ·
`playbook.md` · `session-learnings.md`. Create-only-if-missing ⇒ every one skipped.

---

## Phase 9 commit scope (expected)

`CLAUDE.md` · `.claude/` (rules incl. the new `host-win32.md`, docs, agents, settings.json, handoff) ·
`scripts/` (the code-graph triple; agent-run untouched) · `.gitignore` · the whole `.andromeda/`
(masters + state + this run dir + **the two untracked boundary run dirs** —
`2026-09-02T15-24-02-evolve-diagnose`, `2026-09-02T15-49-17-code-audit` — and the transient
`friction-log.ndjson` / `code-metrics.ndjson` appends) · `conductor-0.1.0/` · `conductor-0.2.0/`.
The boundary-dir sweep is expected per the operator's directive. `.andromeda/cache/` stays gitignored.

## Internal consistency

- CLAUDE.md ≤200 (131 + ~2) ✓ · markers balanced ✓ · 3 `@`-imports resolve ✓
- Every Tier-2 rule named in `setup:deeper-topics` is rendered by Phase 2 ✓
- Every Tier-3 doc named is rendered or preserved by Phase 3 ✓
- Warnings are universal-only; the six rejected candidates each land in a named Tier-2/3 home ✓
- The 6500-byte Bash guard (Phase 5) and the `host-win32.md` transport rule (Phase 2) state the same
  measured limit — mechanism and doc agree ✓
