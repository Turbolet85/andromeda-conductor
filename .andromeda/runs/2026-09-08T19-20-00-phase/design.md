# design extract

## Relevance
Partial — narrow: this is a CI-workflow + ledger chunk with no rendered Conductor surface; design's stake is guarding the token/caption contracts the newly-reachable `a11y` gate touches, not any new UI.

## Constraints
- The 34-name `:root` token block is a binding contract (names + values), per design-system §Surface: desktop-webview / Tokens. This chunk touches no UI source (scope: "No scenario, crate or contract change"), so no token value may move here — including as a way to make a newly-reachable axe run go green.
- §Color Palette / Text Hierarchy records `--text-tertiary` and `--text-muted` at values chosen to clear 4.5:1 on every surface they render on ("as measured 2026-09-01"). If the runtime-152+ install makes the axe pass reachable, that measurement is what re-runs; whether a runtime-major change alters axe's computed-color read-back is research's question.
- The cli status vocabulary is closed sets, per §Surface: cli / Tokens: six lamp prefixes (`[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`) plus the run-level non-lamp caption set (`[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`). A bracket-labelled line printed by a workflow shell step is CI-runner output and is NOT governed by that set; conversely this chunk must not push a new bracket caption or a seventh lamp into `conductor-cli` output.
- §Color Palette (Verdict-vs-ReportState note) requires `Blocked` (never measured) and `KnownResidual` (measured, pre-accepted) to stay visually and semantically distinct from `Fail`. If the probe falsifies the runtime-major hypothesis and `v2-24` is deferred, design requires that disposition never collapse into a `Fail` reading wherever it later renders.
- §Anti-Patterns / Per-Surface Bans (desktop-webview) forbids shipping visible Chromium/WebView2 artifacts (context menu, devtools). The probe's decisive observable is the WebView2 remote-debugging endpoint (`DevToolsActivePort`); design requires any enablement of it stay CI/harness-only. Whether the shipped release configuration already excludes devtools/remote-debug is research's question.
- §Surface: desktop-webview / Platform-Specific Notes states a Tauri floor (≥ 2.10.3) but states no WebView2 **runtime** floor. If this probe establishes a required runtime major, adding it is a wrap Expected-amendment, not a phase edit (scope: spec masters read-only here).

## Patterns to follow
- Reuse-the-existing-tier for any new recessive/qualifier label: the ANSI 246 ↔ `var(--status-residual)` pair already carries every non-lamp use (`hint:`, out-of-scope Mode cell, `[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`) with no new palette row (§Surface: cli / Tokens).
- Name-the-set, never a fresh literal, when a documented count or enumeration grows (§Surface: cli / Tokens; the same discipline §Component Patterns 3 applies to the coverage wall).
- Spec-illustration → sound-impl reconciliation: where shipped reality diverges from the plan's illustration and the invariant still holds, the correction rides wrap's amendment flow into the sidecar — the precedent used repeatedly in this plan's history.
- Distinct-state recording (`no result yet` ≠ `Blocked` ≠ `Fail`) as the model for writing the probe's either-way verdict (§Color Palette Verdict-vs-ReportState note).

## Anti-patterns to avoid
- Adding a new status color, a seventh lamp state, or a new bracket caption to conductor-cli output to describe the probe outcome (§Surface: cli / Tokens; §Anti-Patterns / Rejected Defaults).
- Recording a falsified hypothesis or a deferred `v2-24` as a `Fail` — "Conflating 'no result yet' with 'failed'" is a named Rejected Default (§Anti-Patterns / Rejected Defaults).
- Letting devtools / remote-debugging enablement added for the probe reach the shipped desktop-webview build (§Anti-Patterns / Per-Surface Bans — desktop-webview).

## Contract bindings
- design tokens ↔ a11y: token contrast pairs in §Color Palette / Text Hierarchy are the design-side assertion that the `a11y` job's axe run validates (a11y §Contrast / SC 1.4.3). This chunk unblocks the job; it does not own the pairs.
- design ↔ architecture §CI/CD + verification-matrix: the WebView2 runtime major is the engine that renders the desktop-webview surface during the gate; design-system §Surface: desktop-webview carries the Tauri version floor but no runtime floor, so a floor established by this probe is an amendment question for wrap.
- design ↔ tests/CI harness: `[diag]`-style workflow log lines belong to the CI harness, not to §Surface: cli's closed prefix set — the boundary must be kept explicit so neither vocabulary leaks into the other.

## Acceptance criteria contributions
- No design-token name or value is added, removed, or changed by this chunk; the `:root` 34-token contract is untouched (per design-system §Surface: desktop-webview / Tokens).
- No new status color, lamp state, or bracket caption enters conductor-cli output — lamp set stays closed at six, non-lamp caption set at `[ENVIRONMENT-SUSPECT]` / `[PRECONDITION]` (per design-system §Surface: cli / Tokens).
- A deferred or hypothesis-falsified `v2-24` is recorded as never-measured / pre-accepted, never as a `Fail` (per design-system §Anti-Patterns / Rejected Defaults).
- Any devtools / remote-debugging enablement introduced for the probe is confined to the CI job and does not reach the shipped build (per design-system §Anti-Patterns / Per-Surface Bans — desktop-webview).

## Relevant amendment history
- **2026-09-01-desktop-a11y-sweep** (§Color Palette Text Hierarchy · §Tokens both theme blocks): `--text-tertiary` and `--text-muted` were moved specifically to clear 4.5:1 after an axe `color-contrast` run (18 nodes, all one token; nine pairs recomputed in both themes). Relevant because the job this chunk is trying to make reachable is the automated re-check of exactly those values — and because it is the precedent that token moves happen as a deliberate amendment, never as a green-the-gate edit.
- **2026-09-02-screen-reader-manual-spec** and **2026-09-04-sr-findings-remediation** (§Component Patterns 3 / 5 / 6 / 7): prior a11y/SR passes whose findings entered design-system through wrap amendments (with the Pattern 3 correction escalated to the operator). This is the established route for anything the revived `a11y` job surfaces about design.
- **2026-08-09-out-of-scope-classification-treatment**, **2026-08-09-sut-load-envelope**, **2026-09-03-live-pulse-preconditions-probed** (§Surface: cli / Tokens): three successive growths of the Residual-mute non-lamp reuse set and of the run-level caption set, each requiring an amendment and each explicitly adding no new palette row. This is why a new bracket caption for the probe's outcome would be a drift event, not a free choice.
