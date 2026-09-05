# Fan-out results — 2026-09-04-sidecar-spawn-without-a-console-window

7 doc-agents, one parallel batch. 19 proposals returned across 4 docs; 3 docs clean.

| doc | verdict | proposals |
|---|---|---|
| arch | drift | 7 (2 primary + 4 dependent-of + 1 standalone) |
| security-plan | drift | 4 (2 primary + 2 dependent-of) |
| obs-plan | drift | 2 (1 primary + 1 dependent-of) |
| test-plan | drift | 6 (3 primary + 3 dependent-of) |
| design-system | clean | `proposals: []` |
| layout-templates | clean | `proposals: []` |
| a11y-plan | clean | `proposals: []` + 2 orchestrator notes |

Entity-decode applied to every return (the transport HTML-escaped `<`/`>` in obs-plan's and
test-plan's YAML values); `entities=0` on the decoded text. Raw twins kept only for the four docs
that returned proposals; the three clean returns are recorded here, which is their sanctioned
audit artifact.

## Dispositions

### Applied (routine)

| # | doc · section | change | rule |
|---|---|---|---|
| A1 | arch §Occupied Resources — Service/process names | `TokioChildProcess` → the real `spawn::build_command` / `tokio::process::Command` stdio spawn; register the `CREATE_NO_WINDOW` flag | playbook `:28` wording→sound-impl |
| A2 | arch §Cross-cutting Patterns — Trust boundary | same retired name (duplicate occurrence) | dependent-of A1 |
| A3 | arch §Infrastructure Patterns — Directory structure | register the new workspace-root `rustfmt.toml` | on the plan's Expected list |
| S1 | security-plan §Anti-Patterns → Code Patterns (a) | console-suppression retired from "route-owned, not yet shipped" → SHIPPED + live-verified | playbook `:134` |
| S2 | security-plan §Anti-Patterns (a), `sidecar_resolves_on_path` clause | re-base the rationale — "constructs no `Command`, therefore no console window" no longer distinguishes it | dependent-of S1 |
| S3 | security-plan §Input Validation — `ANDROMEDA_PULSE_DATA_DIR` SPAWN row | `TokioChildProcess` → `spawn::build_command` | playbook `:28` |
| S4 | security-plan §Threat Model Summary — attack surface | same retired name (duplicate occurrence) | dependent-of S3 |
| O1 | obs-plan §11 PII Scrubbing | host-path channel recorded CLOSED; locator refreshed off the stale `spawn.rs:80` | playbook `:134` |
| O2 | obs-plan §11 Project-specific bans | same "not yet shipped" claim, second site | dependent-of O1 |
| T1 | test-plan §1 Untestable zones | add the applied-creation-flag zone (no getter on `std::process::Command`) | the doc's own agent-driven invariant |
| T2 | test-plan §12 Test Decisions Log | roster gains its FOURTH member (`conductor-verify`'s `sidecar_resolves_on_path` pair) | on the report's Counts bullet |
| T3 | test-plan §10 Mutation-survivor disposition | extend the inline roster sample to four | dependent-of T2 |
| T6 | test-plan §6 Drivers per surface | widen the measured msedgedriver/WebView2 set to include runtime 152.0.4191.62 | playbook `:127` set-naming |
| ORCH-1 | a11y-plan §Pass spec format (`:269`) | correct the NVDA pass-spec path to `crates/conductor-tauri/ui/test/a11y/…` | orchestrator-raised; on the plan's Expected list |

### Dismissed (not this chunk's drift)

| # | doc · section | reason |
|---|---|---|
| A4 | arch §Stack — Async runtime row | tokio `1.48.x` → resolved `1.52.3`. The report's Dependencies bullet states "none added, none bumped" with both manifests byte-untouched (independently re-verified: `git diff HEAD` on `Cargo.toml`/`Cargo.lock` is empty). The detector reached past the report into `Cargo.lock`, which its own prompt forbids. **Playbook `:31` — dismiss, NOT this chunk's drift.** The staleness is real and pre-existing; recorded here so it is not lost. |
| A5 | arch §Established Decisions [Language / Runtime] | dependent-of A4 — dismissed with its primary |
| A6 | arch §Inherited Defaults — Language / runtime | dependent-of A4 — dismissed with its primary |

### Disposed without amendment

| item | disposition |
|---|---|
| `TokioChildProcess` @ `test-plan:597` | **LEAVE.** A fifth site, found by the orchestrator's own cross-master grep and proposed by no agent — but it sits inside §12 Test Decisions Log and already carries an inline `[**2026-09-02 correction:** rmcp was removed 2026-06-27 …]`. A decisions-log record is annotated, never rewritten; this is one of the prior wrap's deliberate "leave" calls. All 5 sites of the claim are now disposed (arch ×2 amend · security ×2 amend · test-plan ×1 historical-with-correction). |
| Report Expected-amendment "test-plan §6 / a11y-plan §3 re-activation registration" | **NOT CARRIED — the entry was wrong on both halves.** Independently verified: a11y-plan has **0** hits for `re-?activat`/`bringToForeground`/`activate-window`/`.ps1`/`powershell` (all 6 `foreground` hits are contrast-token context); test-plan's single hit (`:306`) describes `activate-window.ps1` bringing the app to the foreground — i.e. `bringToForeground()`, which this chunk leaves UNCHANGED with all three call sites. Neither master registers `reactivateWindow()`, so retiring it owes no amendment. The two agents each asserted the *other* doc owned it; both were right about their own. |
| `.claude/rules/verification-harness.md` entry 59 item (4) | Routed to P3 curation — a rule file, not a spec master (report *Spec claims disproved* #4). |
| `cargo check -p conductor-verify --lib` red (report #6) | Pre-existing at HEAD, out of the chunk's code scope. Its route owner is settled at P5; the arch-body question is escalation **E1**. |

### Escalated

**E1 — arch §Established Decisions [Module Boundaries] (proposal A7).**
**E2 — test-plan §9 Matrix builds + Pipeline E2E row (proposals T4/T5).**
Both resolved with the operator before apply; see `escalations.md`.
