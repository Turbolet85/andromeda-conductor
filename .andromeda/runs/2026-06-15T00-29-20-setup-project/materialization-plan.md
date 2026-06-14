# Materialization Plan — Conductor (setup-project checkpoint)

_Phase 0 output. The SINGLE synthesis pass — Phases 1–6 read ONLY this file, never the upstreams._
_Source-of-truth: input.md + architecture.md + 6 specialist plans + master-route.md (all read in Phase 0)._

- **Project:** Conductor — scenario-driven OTLP fault-injection + verification harness driving a live Pulse through P-001..P-060.
- **Development Style:** **agent-driven** (arch §Cross-cutting Patterns) → agent harness + `verification-harness.md` rule INCLUDED.
- **Stack:** Rust 2024 (cargo 1.85, toolchain ≥1.94.1) workspace · tokio current_thread · opentelemetry-proto/tonic/prost (OTLP) · rmcp (MCP) · rusqlite/bundled SQLite · Tauri 2 + React 19 + Tailwind v4.1 + shadcn/ui · serde+garde · thiserror+anyhow.
- **Mode:** fresh (no prior CLAUDE.md) — no backup needed.
- **@imports (own-line):** `@.andromeda/architecture.md` · `@.andromeda/master-route.md` · `@.claude/session-handoff.md`

---

## Module map (arch §Directory structure / Inherited Defaults — 8 crates)

- **`conductor-core`** — runtime-agnostic engine library every other crate depends on (shared `Verdict`/`ReportState` types, scenario model).
- **`conductor-timeline`** — deterministic seeded phase scheduler on `tokio::time` (current_thread).
- **`conductor-emit`** — OTLP raw-type emission primitives (opentelemetry-proto + tonic/prost), gRPC egress to `127.0.0.1:4317`.
- **`conductor-faults`** — fault helpers: ramps, silence, port-occupier (`:4317`), fingerprint generation.
- **`conductor-verify`** — MCP read-back client (rmcp over TokioChildProcess stdio), preflight gate, verdict logic.
- **`conductor-report`** — JSONL emission journal + Markdown run report + `runs.db` (rusqlite) storage seam.
- **`conductor-cli`** — `agent-run` binary, headless source of truth + release gate (`#[tokio::main(current_thread)]`).
- **`conductor-tauri`** — Tauri 2 GUI bin (commands + live-counter `Channel`; React 19 webview).

---

## Universal warnings (Top 10 — CLAUDE.md §Critical Warnings)

_Apply to EVERY file (universal/cross-cutting). Severity order security > a11y > obs > tests > design. Path-specific bans routed to Tier 2 (see Rejected, below)._

1. **[security] Scope law + trust boundary** — every scenario carries a Pulse P-ID ("no scenario without a P-ID"); Conductor opens NO inbound listener of its own (the `:4317` port-occupier is the sole deliberate bind, released on cleanup) — never widen beyond the loopback gRPC/MCP-client model.
2. **[security] Verdict/error wall** — verification outcomes are typed VALUES (`Verdict`/`ReportState` returned as `Ok`); `Result::Err` is reserved for harness faults only; malformed child/transport input (`tonic::Status`, MCP errors) becomes a typed `Blocked`/`Fail`, never a panic.
3. **[security] Preflight integrity** — never silently downgrade a failed MCP preflight (protocol≠`2024-11-05` / missing tool / empty canary) to pass/fail/manual-check; surface the distinct `Blocked` state with its named precondition.
4. **[security] Supply chain** — keep `Cargo.lock` committed + un-drifted; never `cargo build --release` or merge without `cargo-audit` (+ `cargo-deny`) green; hold toolchain ≥1.94.1 and `tauri` ≥2.10.3.
5. **[security] Subprocess hardening** — spawn `andromeda-pulse-mcp` from a fixed hard-coded path; pass `ANDROMEDA_PULSE_DATA_DIR` only via `.env(...)` after rejecting injection metacharacters — never interpolate operator input into argv/shell.
6. **[security/obs] Artifact hygiene** — never leak absolute host paths or internal struct names into logs / run-report / `runs.db`; sanitize at the `anyhow` edge + the tracing-subscriber field-allowlist.
7. **[obs] Self-observation never exports OTLP** — the only OTLP is the PRODUCT fault stream to Pulse `:4317`; self-obs is `tracing` JSON to stdout/file/console (no OTel SDK), every line carrying `run_id`; zero unlogged panics (`std::panic::set_hook` → `tracing::error`).
8. **[determinism: security/obs/tests] Wall-clock from `std::time`** — journal/report stamps use `std::time::SystemTime`/`Instant`, never tokio's virtual clock (scheduling-only); journal-relative SLO math depends on it.
9. **[a11y/design] Status is never color-alone** — pair every Verdict/ReportState with its text label + glyph (desktop) or ASCII prefix `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` (cli); color encodes run state, never decoration.
10. **[tests] Determinism is the quality bar** — same scenario+seed ⇒ same stream shape; loopback gRPC/MCP stubs only (no real network/time in tests; `start_paused` for scheduling); zero-flakiness (no nextest retries).

**Rejected → Tier 2 (audit):** garde-validate scenario config at load (→ security rule, config-scoped) · bound-parameter `runs.db` SQL (→ security/observability rule, report-scoped) · deny-by-default Tauri capabilities / no `shell-open` / no remote-origin iframe (→ frontend rule) · no Percy/Chromatic / no xpath / no `sleep()` sync (→ testing rule) · banned generic fonts / no KPI-card-grid (→ frontend rule) · axe/contrast/keyboard on the four accessible paths (→ a11y rule). All path-scoped, not universal.

---

## Pointer table (CLAUDE.md §Where to Look — topic → source)

| Topic | Source |
|---|---|
| Architecture decisions | `.andromeda/architecture.md` |
| Build plan / chunk route | `.andromeda/master-route.md` · `conductor-0.1.0/working-route.md` |
| Module dependencies | `.andromeda/context/dependency-tree.md` |
| Interface / API surface | `.andromeda/context/api-surface.md` |
| MCP read-back contract + preflight gate | `.andromeda/architecture.md` §Standard Contracts |
| Run-report envelope | `.andromeda/architecture.md` §Standard Contracts · `.andromeda/test-plan.md` §3 |
| Scenario config validation | `.andromeda/security-plan.md` §Input Validation |
| Dependency security / cargo-audit | `.andromeda/security-plan.md` §Dependency Security |
| Subprocess-spawn hardening | `.andromeda/security-plan.md` §Security Anti-Patterns |
| Design tokens / palette / typography | `.andromeda/design-system.md` §Color Palette / §Typography |
| Layout / surfaces (desktop + cli) | `.andromeda/layout-templates.md` |
| Test harness (5-command) | `.andromeda/test-plan.md` §3 |
| CI integration / quality gates | `.andromeda/test-plan.md` §9 / §10 |
| Observability / log JSON schema | `.andromeda/obs-plan.md` §3 / §6 |
| WCAG / a11y harness | `.andromeda/a11y-plan.md` §3 |
| Determinism discipline | `.andromeda/architecture.md` §Cross-cutting Patterns |
| Coverage matrix (60 P-IDs) | `.andromeda/input.md` §Coverage classification |
| Agent harness commands | `scripts/agent-run.sh` · `.claude/rules/verification-harness.md` |
| Stack / versions | `.claude/docs/stack.md` |
| Conventions | `.claude/docs/conventions.md` |
| Commands | `.claude/docs/commands.md` |
| Gotchas | `.claude/docs/gotchas.md` |

---

## Tier 1 — CLAUDE.md

- Template: `references/claude-md-template.md`. ≤200 lines.
- Overview: from arch §Project Intent + input.md (stack one-liner + key dirs: `crates/`, `scenarios/`, `contracts/`, `runs/`, `scripts/`).
- Modules: the 8-crate map above.
- Critical Warnings: the Top-10 above.
- Where to Look: the pointer table above (22 rows).
- Workflow key commands: `cargo build --release` · `scripts/agent-run.sh run` · `cargo nextest run --workspace` · `cargo clippy --workspace -- -D warnings` · `cargo audit`.
- Architecture: 1–2 paras from arch §Design Philosophy (determinism under seed · headless-drivable core / thin shells · compiler-enforced crate seams · outcomes-are-values/verdict-error-wall · journal-relative ground truth).
- @imports: arch.md / master-route.md / session-handoff.md.
- Deeper Topics: docs (5 core + 5 summaries) + rules list (6 files); no services docs.
- USER:session-learnings seed (3–5 foundational invariants): scope-law/no-listener · verdict-error-wall · std::time journal stamps · cargo-audit-green before release · status-never-color-alone.

---

## Tier 2 — .claude/rules/

| Rule file | Include? | Path globs (best-effort from arch tree) | Specialist anchor |
|---|---|---|---|
| `security.md` | ALWAYS | `crates/**`, `scripts/**`, `contracts/**`, `scenarios/**` | security-plan §Input Validation / §Anti-Patterns |
| `testing.md` | YES (test-plan present) | `crates/**/tests/**`, `crates/**/src/**/*.rs`, `scenarios/**` | test-plan §3 / §11 |
| `observability.md` | YES (obs-plan present) | `crates/**/src/**/*.rs` | obs-plan §3 / §6 / §11 |
| `a11y.md` | YES (a11y plan + desktop-webview UI) | `crates/conductor-tauri/**` | a11y-plan §3 / §4 / §11 |
| `frontend.md` | YES (React 19 + Tailwind + shadcn webview) | `crates/conductor-tauri/**` (ui/src) | design-system §Component Patterns + layout-templates; security Tauri guardrails |
| `verification-harness.md` | YES (agent-driven) | `scripts/agent-run.*`, `crates/conductor-cli/**`, `crates/conductor-verify/**` | test-plan §3 + arch §Standard Contracts |
| `migrations.md` | NO | — | arch: raw SQL, no migration framework |
| `api.md` | NO | — | arch: no HTTP/served API (no inbound listener) |
| `events.md` | NO | — | arch: no message broker / event topics |

Preserve `## Session Additions` on any existing rule file (fresh run → none exist; append empty `## Session Additions`).

---

## Tier 3 — .claude/docs/

- **Core (5):** `stack.md` (mirror arch §Stack verbatim) · `conventions.md` (arch §Conventions — snake_case, `conductor-<seam>`, P-ID keying, kebab-case artifacts) · `commands.md` (cargo + agent-run verbs) · `gotchas.md` (documented architectural traps: tonic 0.14 prost-build split, bundled-SQLite cold builds, rmcp negotiate-DOWN to 2024-11-05, data-dir-mismatch ⇒ silent-empty read-back, virtual-clock-corrupts-journal) · `workflow.md` (agent-driven loop).
- **Specialist summaries (5):** `security-summary.md` · `design-summary.md` · `tests-summary.md` · `obs-summary.md` · `a11y-summary.md` (~100 lines each, end with "Full plan: `.andromeda/{plan}.md`").
- **Per-module `services/{crate}.md`:** OMIT — greenfield, no implementation notes yet; Module map (CLAUDE.md) + arch §Workspace crates cover crate purposes. (phase-loop/wrap-session seeds per-crate docs once a crate gains implementation.)
- `session-learnings.md`: create only if missing (wrap territory).

---

## Agent harness (Development Style = agent-driven)

`scripts/agent-run.sh` + `.ps1` + `.claude/rules/verification-harness.md`. 5-command params from test-plan §3 + obs-plan §3:

- **boot** = MCP preflight readiness gate — `cargo run -p conductor-cli --bin conductor -- preflight --json` (assert negotiated `2024-11-05` + required-tool presence vs `contracts/` manifest + data-dir canary round-trip; CI uses the rmcp stub returning `ready:true`; timeout 30s). `ready:false` ⇒ dependent scenarios `Blocked`.
- **run** = `cargo nextest run --workspace --profile ci` + `cargo test --workspace --doc` + `cargo clippy --workspace --all-targets -- -D warnings`; scenario leg `cargo run -p conductor-cli --bin conductor -- run <scenario|P-ID> --seed <s>` / `conductor suite`. Exit 0 = all Pass; non-zero = a hard `Fail` (nextest 100); Blocked/Manual/KnownResidual/CalibrationRegion are reported states, not exits. Stage flags `--unit`/`--integration`/`--e2e`.
- **status** = read `runs/<run_id>.jsonl` (`jq -e`) or `runs.db` row (rusqlite bound-param SELECT); no HTTP endpoint (no listener).
- **cleanup** = `rm -f runs/<run_id>.{jsonl,md}` + `DELETE FROM runs WHERE run_id=?1` + release `:4317` bind; idempotent.
- **logs** = read `runs/<run_id>.jsonl` emission journal (wall-clock `std::time` stamps, JSONL one-object-per-line) + sanitized stderr.

---

## Hooks (.claude/settings.json — Rust-primary, resolved obs-plan §3 / manifest / hooks-matrix)

- **formatter:** `cargo fmt` · **linter:** `cargo clippy --fix` · **type-checker:** `cargo check`.
- **PreToolUse:** block writes to `dist/ build/ .next/ node_modules/ coverage/ target/ vendor/`.
- **PostToolUse:** on `.rs` edits → `cargo fmt` + `cargo clippy --fix` + `cargo check`. (Webview `ui/` TS files optional prettier/eslint — Rust is the release gate; keep hooks Rust-primary.)
- Preserve user-managed keys.

---

## Living artifacts (seed only; wrap reconciles)

- `.andromeda/context/dependency-tree.md` — Tooling: `cargo modules generate tree` (fallback `cargo tree`). LIVING slug `dep-tree`.
- `.andromeda/context/api-surface.md` — Tooling: `cargo public-api --simplified --workspace`. LIVING slug `api-surface`.
- METADATA + LIVING markers per `integrity-protocol.md`; seeded `(stale — run {tool})` until first wrap.

---

## Code reviewer

`references/agent-templates/code-reviewer-rust.md` → `.claude/agents/code-reviewer.md` (substitute project name = Conductor).

---

## Gitignore

Append `references/gitignore-fragments/rust.md` fragment (idempotent) + Claude entries (`.claude/backup/`, `.claude/settings.local.json`). A `.gitignore` already exists (untracked) — append, don't clobber.

---

## Session / operational seed (Phase 6 — only if missing)

- `.andromeda/state.yaml` — lean schema_version 3 (last_wrap null, living_doc_freshness nulls, session_count 0).
- `.claude/session-handoff.md` — skeleton: Last Updated=now, Branch=current, Status=clean, Next="/andromeda-phase to plan the first chunk".
- `.andromeda/drift-base.md` — seed starter detectors (7 greenfield artifacts) from `seed-templates/drift-base.md`.
- `.andromeda/playbook.md` — near-empty starter from `seed-templates/playbook.md`.
- `.claude/docs/session-learnings.md` — create only if missing.
