# Materialization Plan — Conductor (re-run, 2026-06-24)

**Mode:** re-run (18 `GENERATED:setup` markers) · **Development Style:** agent-driven · **User directive:** full re-run, **preserve the chunk-evolved `agent-run.{sh,ps1}`** (ensure_frontend + 5-command hardening) — do NOT regress.

## Assessment — the ecosystem is wrap-maintained + current
The CLAUDE.md `GENERATED:setup:*` sections + rules + docs are kept current by every wrap's cascade; the just-committed wrap (`69010f4`) already cascaded this Epoch-9-opening chunk's amendments (→ `stack.md` gained `@tauri-apps/api` + `tauri-build`; CLAUDE.md was a verified no-op). So this re-run is **confirmatory** — one genuine refresh (below); everything else confirmed-current.

## Tier 1 — CLAUDE.md
- **GENERATED:setup:*** (overview · modules · warnings · pointer-table · workflow · architecture · imports · deeper-topics): **confirmed current** against the post-amendment specs. Overview Stack line is high-level (no per-lib → `@tauri-apps/api` not surfaced here); warnings carry `tauri ≥2.10.3`; pointer-table covers all topics; `@imports` = arch.md · master-route.md · session-handoff.md. **No change.**
- **USER:session-learnings** (6 entries incl. the 2026-06-14 version-branch rule): **preserve verbatim.**
- Size 123/200.

## Tier 2 — .claude/rules/ (6: security · testing · observability · a11y · frontend · verification-harness)
- Bodies are template-derived + stable; **confirmed current.** `## Session Additions` **preserved verbatim** — incl. THIS session's curation (frontend.md build-order/gen/icons; security.md Tauri-tree deny). **No body change.**

## Tier 3 — .claude/docs/ (5 core + 5 summaries)
- **gotchas.md — ONE GENUINE REFRESH:** add the documented architectural trap *"Tauri `generate_context!` resolves `ui/dist` at compile time"* (now in arch §Infrastructure Patterns). Surgical add (preserve the 7 existing traps). The disk + supply-chain learnings correctly live in `session-learnings.md` (runtime) + `security.md` (Tier 2) — gotchas.md is documented-traps-only.
- **stack.md:** already refreshed by the wrap cascade (`@tauri-apps/api` + `tauri-build` build-order). Confirmed current.
- **conventions.md / commands.md / workflow.md + 5 summaries:** distill mostly-unchanged plans; **confirmed current** (the wrap cascade would have updated any that drifted). No change.
- **session-learnings.md:** wrap territory — untouched.

## Agent harness — PRESERVE (user directive)
- `scripts/agent-run.{sh,ps1}`: **NOT regenerated.** Keep the committed `ensure_frontend` build-order (Epoch-9) + the 5-command-harness hardening — these evolved beyond the template/`test-plan §3` contract; a template regen would regress them and break the `conductor-tauri` build. `verification-harness.md` rule: Session Additions preserved; body current.

## Code reviewer · Hooks · Gitignore
- `code-reviewer.md` (rust): present, current — no change.
- Hooks (`settings.json`): present — confirm, preserve user-managed keys.
- `.gitignore`: current — already carries the base ignores + stack fragment + `crates/conductor-tauri/gen/` (added this chunk). No change.

## Code-graph pipeline + seeds (Phase 6) — present, NO-OP
- `scripts/{code-graph.py, code-graph-views.sql, scip_pb2.py, code-graph-cookbook.md}` present (rust-analyzer SCIP indexer). `.andromeda/cache/` present. `state.yaml` (schema 3, session 49) · `session-handoff.md` · `drift-base.md` · `playbook.md` · `session-learnings.md` all present → no re-seed.

## Net change set
**1 file:** `.claude/docs/gotchas.md` (+1 architectural trap) + this materialization-plan (run-dir). Everything else confirmed-current or preserved.
