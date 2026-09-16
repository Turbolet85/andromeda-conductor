# Fan-out results — 2026-09-16-a11y-ci-gate-at-an-honest-terminal

7 doc-agents, one per spec source. **7 proposals** from 3 docs; 4 docs returned `proposals: []`.

| doc | verdict |
|---|---|
| design-system | `proposals: []` — no new UI, no moved count; platform mentions are naming, not verdicts |
| layout-templates | `proposals: []` — no new surface; `:297` headless-invariant is an `isatty` rule, a different mechanism from the falsified `runas`-prompts claim |
| obs-plan | `proposals: []` — `obs-plan.md:42` already classes the CI/CD pipeline Not-instrumentable, so `instrumentation n/a` matches the spec |
| a11y-plan | `proposals: []` — `:299`'s platform SET unretired; independently verified the 3 `build + test gating` citation sites intact |
| **arch** | **5 proposals** (all `warning`) |
| **security-plan** | **1 proposal** (`escalate`) |
| **test-plan** | **1 proposal** (`warning`) |

## Claim sites re-derived by the orchestrator (not taken on the proposals' word)

| site | verbatim finding |
|---|---|
| `architecture.md:60` | "The leg IS demonstrably runnable on the Windows dev host (12 passing / 2 skipped through tauri-driver); hosted-runner runnability is measured-unproven." — **confirmed stating sentence** |
| `architecture.md:60` | "the remedy (a limited-token launch, then a re-measurement) is owned by the `v3-02` route entry" — **confirmed** |
| `architecture.md:60` | a11y job stated as "`scripts/agent-run.ps1 run --e2e` under `CONDUCTOR_A11Y_STRICT`, then the reused `journal_conformance` gate…" — **confirmed superseded invocation path** |
| `architecture.md:224` | `webview2-cause-probe.ps1 # CI-only read-only diagnostic; invoked solely by ci.yml's a11y job, wired into neither harness shell, adds no 6th command` — **confirmed precedent**, and `grep -c webview2-cause-probe` → 1, so it is the only CI-only script registered |
| `architecture.md:244` | "plus the `a11y` job's Pulse-free routine webview…" — **confirmed second occurrence** |
| `security-plan.md:363` | "The SIXTH is the CI-WORKFLOW spawn (added 2026-09-08)…" — **confirmed the registry closes at six** |
| `test-plan.md:461` | "whose step set is `agent-run.ps1 run --e2e` → … plus the `continue-on-error` DIAGNOSTIC step SET — {WebView2 driver+runtime versions (2026-09-07) …" — **confirmed three-member enumeration against four shipped** |

## Validation disposition

**ROUTINE — apply (playbook `:127` SET-naming / registration precedent):**
1. `arch:224` — register `a11y-limited-token-launch.ps1` + `a11y-token-witness.ps1` on the `webview2-cause-probe.ps1` precedent.
2. `arch:60` + `arch:244` — correct the superseded direct-`agent-run.ps1` invocation path (dependent pair, one claim, two sites).
3. `test-plan.md:461` — name the diagnostic step SET with its fourth member and the launcher path.

**ESCALATE — reached the operator before apply:**
- **E1 `security-plan.md:363`** — seventh governed harness-spawn form. Playbook `:124` Boundary widening → `escalate`, "always a human's call — never mint a routine rule for this class". Complication: the operator's ratification named the **scheduled task**; the shipped mechanism is **`runas /trustlevel`**.
- **E2 `arch:60`** — the dev-host capability verdict, whose only positive datum this chunk falsified. No routine rule covers a D-platform-claim hit on a real stating sentence → uneasy → escalate. Also corrects the chunk report's own item 4.
- **E3 `arch:60`** — the remedy clause naming a limited-token launch as owned-and-open by `v3-02`; this chunk shipped it in two mechanisms and measured both insufficient, and `v3-02` is being un-claimed in the same wrap.
