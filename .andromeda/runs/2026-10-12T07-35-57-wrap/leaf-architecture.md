group: architecture
read: .claude/docs/commands.md · 48 · «- `cargo modules generate tree` / `cargo tree` — module/depe»
read: .claude/docs/conventions.md · 56 · «- Warnings & gotchas → `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 72 · «- Path-specific rules → `.claude/rules/*.md`. Decision ratio»
read: .claude/docs/stack.md · 56 · «1. Update `architecture.md` §Stack first → 2. update `Cargo.»
read: .claude/docs/workflow.md · 51 · «Reference-based CLAUDE.md: a thin index (≤200 lines) with de»
read: .claude/rules/frontend.md · 27 · «- 2026-09-17: **In a webview spec, an expectation that inter»
read: CLAUDE.md · 137 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:27   class: stale
  leaf-text: «the live-counter `drive_run`»
  source: .andromeda/registries/contracts/architecture/directory-structure-crate-per-seam-cargo-workspace.md:23 «# run composition root lib (preflight + execute_scenario + persist + the suite driver drive_run (progress callback + Ctrl-C stop flag) + the fault-phase occupier guard; → conductor-faults)»
  replace-with: «the suite driver `drive_run` (progress callback + Ctrl-C stop flag)»

- leaf: .claude/docs/conventions.md:43   class: stale
  leaf-text: «a dialog row's display bound) carrying the operator-checklist rows a hold renders; `HoldPoint` carries the same vector so the hold projects them to its shell.»
  source: .andromeda/architecture.md:81 «a length bound on untrusted scenario-authored text the CLI's interactive hold prints, each half on its own indented `induced:` / `observe:` line)»
  replace-with: «a length bound on untrusted scenario-authored text the CLI's interactive hold prints, each half on its own indented `induced:` / `observe:` line) carrying the operator-checklist rows a hold renders; `HoldPoint` carries the same vector so the hold projects them to the CLI's interactive hold (the headless path prints none).»

- leaf: .claude/docs/conventions.md:36   class: omission
  leaf-text: «(`check_budgets`, `check_capabilities`)»
  source: .andromeda/architecture.md:49 «`Scenario::from_toml_str` → `CoreError::Config` on a parse failure OR on a non-garde load-path check failure (`check_budgets`, `check_capabilities`, `check_checklist`)»
  replace-with: «(`check_budgets`, `check_capabilities`, `check_checklist`)»

- leaf: .claude/docs/stack.md:13   class: stale
  leaf-text: «opentelemetry-proto 0.32.0 (`gen-tonic` + trace/metrics/logs)»
  source: .andromeda/architecture.md:17 «opentelemetry-proto 0.32.0 (`default-features = false` at the workspace entry; `gen-tonic` + `trace`/`logs`)»
  replace-with: «opentelemetry-proto 0.32.0 (`default-features = false` at the workspace entry; `gen-tonic` + `trace`/`logs`)»

- leaf: .claude/docs/stack.md:42   class: retired
  leaf-text: «- **A11y:** axe-core 4.12.0 + @axe-core/webdriverio + Lighthouse 13.0.3 + colorjs.io 0.6.1 (operator/local-gated).»
  source: .andromeda/architecture.md:57 «The `rust` job is the only job: the `frontend` and `a11y` jobs, the msedgedriver pin gate and the routine webview a11y leg left with the window on 2026-10-11 (intent R1, ratified at that chunk's wrap)»
  replace-with: «- **A11y:** none since 2026-10-11 — the webview a11y leg and the `a11y` CI job left with the window; no a11y tooling remains in the stack.»

- leaf: .claude/docs/commands.md:36   class: stale
  leaf-text: «(17 as of 2026-09-07: 16 `unmaintained` + 1 `unsound`)»
  source: .andromeda/registries/contracts/architecture/build-system.md:1 «`deny.toml` ignores no advisory and allows 8 licenses since then, when the Tauri tree's 16 `unmaintained` ignores and the `MPL-2.0` allow left with their subject»
  replace-with: «(empty as measured at the `2026-10-11-conductors-window-retired` gate: `deny.toml` ignores no advisory and `cargo audit` reports no allowed warning)»

- leaf: .claude/docs/gotchas.md:67   class: stale
  leaf-text: «Post-clean this workspace measures exit 0 (1239 advisories, 562 packages, 17 `deny.toml`-adjudicated allowed warnings — re-measured 2026-09-07 after the indicatif 0.18 bump dropped `number_prefix` and its RUSTSEC-2025-0119 ignore).»
  source: .andromeda/registries/contracts/architecture/build-system.md:1 «`cargo audit` exit 0 (1296 advisories loaded, 268 crates scanned, no allowed warning, as measured at the `2026-10-11-conductors-window-retired` gate over a porcelain-clean advisory-db copy;»
  replace-with: «Post-clean this workspace measures exit 0 (1296 advisories loaded, 268 crates scanned, no allowed warning — as measured at the `2026-10-11-conductors-window-retired` gate over a porcelain-clean advisory-db copy; the 2026-09-05 reading — 1239 advisories, 562 packages, 17 allowed warnings — stands as that date's record).»
