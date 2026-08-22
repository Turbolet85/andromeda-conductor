# Scope — 2026-08-22-operator-pause-and-checklist-live-firing

**Working entry (verbatim intent):** _Operator-pause and checklist live firing — frozen-count hold-point,
go/no-go hold and ManualCheck items exercised against a running Pulse._

**Epoch:** 4 — Lifecycle & delegated timing (this chunk's wrap CLOSES Epoch 4).
**Claim expectation:** `v2-29` only — "Operator pause and checklist exercised live" (`method: manual`, the
sanctioned method for this capability by construction).

---

## What this chunk proves

The pause machinery has existed end-to-end and been unit-proven since Epoch 8/9 of 0.1.0. What has **never
happened** is a *live attended activation*: every hold that has ever fired was answered by
`HeadlessResolver`. This chunk makes the three-part `v2-29` acceptance true by measurement against a running
Pulse, and closes the wiring gaps that measurement exposes.

`v2-29`'s acceptance (already concrete; not to be weakened) has three parts:

1. **A live hold fires and is resolved by a REAL Proceed or Abort activation** — an attended operator
   answering the prompt, not `HeadlessResolver`.
2. **The count freezes at its exact hold value** — neither blanking nor continuing — **and resumes on
   decision.**
3. **At least one ManualCheck checklist item renders with its induced state and expected observation.**

---

## Verified coordinates (directive-supplied, each re-verified against the artifact at HEAD `7fd1608`)

The phase directive supplied these as hypotheses; every one was read back before it shaped this scope.

| Coordinate | Verified state |
|---|---|
| `conductor-core/src/pause.rs` | `Decision` (:29) · `HoldPoint` (:55) · `HoldResolution` (:78) · `PauseResolver` (:95) · `HeadlessResolver` (:105) · `resolve_hold` (:137). **As stated.** |
| `conductor-core/tests/operator_pause.rs` | **6** tests, incl. `prompt_is_redacted_in_the_resolution` + `headless_never_blocks_under_paused_clock`. **As stated.** |
| `conductor-run/src/lib.rs:394-411` | The ONE live firing site. `if scenario.expected.is_empty()` → `HoldPoint { step: "operator-checklist", allow_no_go: true, prompt: "Observe the operator-checklist claim for this scenario" }` → `resolve_hold` → `manual_record`. **Line span exact.** |
| `lib.rs:403` tracing line | `"operator-checklist hold resolved headless: {}"` — labels the resolution **"resolved headless" UNCONDITIONALLY**. Goes false the first time an attended resolver answers. **Confirmed.** |
| `conductor-cli/src/pause.rs` | `PromptResolver { spinner: Option<ProgressBar> }` (:25) · `CliResolver::select(spinner, agent_mode)` (:73) · `resolve_kind` (:90), `agent_mode` OVERRIDES an attended TTY (pinned by a test at :136). **As stated.** |
| `conductor-cli/src/commands/run.rs:23` | `CliResolver::select(None, agent_mode)` — spinner `None`. **As stated.** |
| `conductor-cli/src/render.rs` | `hold_line` at :82-84 (doc :79-81), `hold_line_styled` at :160-163. Directive's `:79-83,:160-162` points at the right code; **spans corrected by ±1**. |
| `conductor-core/src/lamp.rs:44` | `(ReportState::ManualCheck, None) => Lamp::Manual`. **Exact.** CalibrationRegion→HOLD exception pinned by the test at :98-106. |
| `conductor-tauri/src/pause.rs` | `HoldPrompt` (:23) · `HoldGate` (:47) · `TauriResolver` (:68) · `#[tauri::command] resolve_operator_hold` (:92). **As stated.** |

### Two corrections to the directive's picture

- **The declare-only surface is far wider than the four scenarios named.** `24` catalog scenarios carry no
  `[[expected]]` block and therefore ALL route to the `lib.rs:394` checklist HoldPoint — not just the halo
  pair. The directive named `halo-hue-encoding` + `halo-breathing-encoding` plus "checklist halves" in
  `fingerprint-storm` / `error-baseline-spike` / `activity-floor` / `constellation-severity-live-wiring`;
  `activity-floor.toml` and `constellation-severity-live-wiring.toml` both exist but are NOT declare-only
  (they carry `[[expected]]`), so their checklist halves are prose, not a routed hold.
- **`halo-hue-encoding` declares zero `[phases.emission]`** (both phases are bare `gap_ms`) — confirmed
  against the file, and the reason its P-025 budget measured 18× over. `halo-breathing-encoding` DOES
  declare emission (two `ramp` phases, `from_rate`/`to_rate`/`windows`) and carries **P-026, not P-025**.

---

## The three acceptance parts against current reality

### (1) A real attended activation — machinery ready, never exercised
`PromptResolver::resolve` renders `render::hold_line(hold)` to stderr and asks via `inquire::Confirm`;
`allow_no_go: true` on the checklist hold means both Proceed **and** Abort are reachable outcomes
(`confirm()` maps `Ok(true)→Go`, `Ok(false)→NoGo`, `Err(_)→NoGo`). The gate is `resolve_kind`: an attended
TTY on **both** stdin and stdout, with `agent_mode` false. **The live leg cannot run through the agent
harness** — operator-driven invocation only.
`[premise-corrected: the harness forces Headless by passing the `--agent-mode` FLAG (`agent-run.sh:98,:100`),
not by exporting `CONDUCTOR_AGENT_MODE` as scope first stated — same effect, different mechanism. And the
conclusion is stronger than the premise: `agent-run.sh` has NO `suite` verb at all (only
`conductor run <scenario> --agent-mode`), so the harness could not host the part-(2) vehicle even with the
flag removed.]

### (2) The frozen count — the surface is the CLI **suite** spinner
`PromptResolver::resolve` freezes via `bar.suspend(render_and_prompt)` (indicatif) — the exact
"freezes at its hold value, neither blanking nor continuing, resumes on decision" behaviour. The bar is
`render::spinner(len)`, template `"{spinner} {pos}/{len} scenarios"` (`render.rs:115-122`), hidden when
stderr is not a terminal.

**Only `suite` supplies it:** `commands/suite.rs:28` passes `Some(progress.clone())`;
`commands/run.rs:23` passes `None`. So the count that can freeze is the **suite progress count**, and the
live vehicle for part (2) is `conductor suite` attended — `conductor run` has no count to freeze.
**VERIFIED.** `suite.rs:27-28` builds the bar and hands the same handle to the resolver; `progress.inc(1)`
runs *after* `execute_scenario` returns, so at the hold the count reads its exact pre-increment value.
`--filter` is a substring match on the file stem (`paths.rs:73`), so `--filter halo` yields a
two-scenario suite of exactly the declare-only halo pair — the natural attended vehicle *on the cli*.

**The GUI has the same property by the same construction, and is the chosen surface** (see part (3)):
`drive_run` emits its `RunEvent` only *after* `execute_scenario` returns (`conductor-run/src/lib.rs:774-777`),
so while a hold is pending no event flows and `App.tsx`'s titlebar count — set from `event.count` — holds its
exact pre-increment value, tinted by `setRunState('hold')`, resuming on the decision. Nothing needs building
for part (2) on either surface; it needs exercising and observing. One GUI-specific constraint:
`resolve_selection` is binary (whole suite, or exactly one scenario by name) with no `--filter` equivalent, so
the nonzero frozen count comes from the suite leg's successive holds across the 24 declare-only scenarios.

### (3) A checklist item with induced state + expected observation — the real gap
The `induced` / `observation` pair — verbatim the acceptance's wording — exists **only in the webview**
(`ui/src/components/OperatorChecklist.tsx`, `interface ChecklistItem { id, induced, observation, checked }`),
and `OperatorChecklistView` is rendered **only from `Gallery.tsx:199` with hardcoded placeholder items**
(`useState<ChecklistItem[]>([...])`). It has never rendered a real scenario's item.

There is **no Rust-side checklist-item type at all**: the only Rust checklist surface is the single generic
sentence at `lib.rs:399`, which carries neither an induced state nor an expected observation. The per-scenario
wording exists as **TOML comment prose** ("hue shifted toward burgundy under error pressure?", "halo breathing
rate tracks throughput?"), explicitly labelled *Epoch-8/10 calibration points* — prose, not data.

So part (3) requires real wiring: a scenario's induced state + expected observation must become **data** that
reaches a rendering surface.

**SURFACE RESOLVED at the P5 plan review (operator's call, which the phase directive reserved):
the desktop-webview.** The checklist item is declared as scenario TOML, carried through the core `HoldPoint`
and the `HoldPrompt` projection, and rendered by the existing `OperatorChecklistView` primitive inside
`OperatorPauseDialog`; the attended live legs are driven through the Tauri GUI. The CLI detail-line
alternative was P4's marked recommendation and was **not** taken — recorded in `plan.md`
§Constraints & rejected approaches so implement does not re-open it. Consequence carried knowingly: the
webview's machine a11y verification (axe/wdio) is display-gated to Linux+xvfb and cannot run on this host, so
it stays owed exactly as it already was — `v2-29`'s method is `manual`, so the claim rests on the operator's
confirmation, and **no WCAG conformance is claimed from this chunk**. **VERIFIED** — and independently mandated: a11y-plan §1/§4 already require that each checklist item
"exposes induced-state + expected-observation text", and design-system §Component Patterns 7 / layout-templates
§Component — Operator-checklist state the same pair as a contract rather than free prose.

---

## Boundaries

- **Claims `v2-29` and nothing else.** The likely live vehicles carry P-025 / P-026 halo claims; a checklist
  item **observing** a halo claim does **not** claim P-025 here. **P-025 stays deliberately in the UNVERIFIED
  pool** awaiting the Epoch-6 *"Halo hue budget re-driven"* entry — its budget was measured false
  (35581 ms / 36705 ms against 2000 ms) and its re-drive is a scenario redesign owned elsewhere.
- **No re-litigating the delegated-timing verdicts.** P-027 / P-037 / P-045 are verified at their measured
  values; this chunk does not touch them.
- **Plan stays WHAT-level** — if checklist-item wording wiring lands in scope, the plan names the surface and
  the contract, not the implementation.
- **No new external crates expected**; `indicatif` and `inquire` are already present and are the mechanisms
  parts (1)+(2) rest on.
- The `"resolved headless"` tracing line at `lib.rs:403` is a **known-false-under-attended** wording defect
  this chunk is positioned to correct; whether it lands here is a plan decision, not a scope commitment.
  **VERIFIED, and worse than scope first stated:** the line is `tracing::debug!`, and the default filter is
  `LevelFilter::INFO` (`obs.rs:79-80`), so under a normal attended invocation it does not appear **at all**.
  Separately, `manual_record` (`lib.rs:518-539`) never receives `resolution` — the operator's Decision is
  read only by that one debug line, so **a NoGo/Abort produces a byte-identical record to a Go**. Acceptance
  part (1) therefore has no artifact witness today by either route.

---

## Folded PREREQ (from the working entry — this chunk must absorb it)

**Re-check `cargo audit` — the 38th consecutive deferral.** Standing since
`2026-08-08-sut-capability-manifest`; operator-ratified at the `2026-08-10-workspace-key-divergence-probe`
wrap, re-pins silently thereafter. Basis re-verified at `2026-08-21-delegated-timing-budgets-proven`, which
admitted **zero** packages (`Cargo.toml` + `Cargo.lock` byte-untouched), so the advisory-**database** fault
remains the sole cause and the audit↔deny overlap stays *verified*, not assumed.

**PROBE-AUTO-SATISFY signature** (byte-identical reproduction ⇒ satisfied by the one-line record
`probe unchanged, 38th consecutive`, no basis re-authoring — this would be the **5th consecutive pure fire**):
- `cargo audit` **true exit 1**, first diagnostic line `duplicate advisory ID: RUSTSEC-2026-0244`
- `cargo deny check advisories bans licenses sources` **true exit 0**

**Trap, named in the pin:** capture the audit exit code **BEFORE any pipe** — `$?` after `| head` reports the
pipeline's last stage and reads as a false deviation.

**Any real deviation** (changed diagnostic, moved exit code, overlap shift, or a dependency delta that
ADMITS a package) restores the FULL form. Remedy stays the bounded wait — **no floor raise, no `deny.toml`
ignore, no CI edit**; close the moment it parses. This chunk should admit **zero** packages.

---

## Live-leg conditions (established runbook shape)

Pulse parked at `f0c38f5`; fresh data dir + `pulse-app` restart per leg, window open, deterministic L4
(`ANDROMEDA_PULSE_L4_DETERMINISTIC=true`). The delegated-timing precedent is the working pattern: the
**operator drives by runbook**, the session grades this-run artifacts. Under deterministic L4 every read-back
returns `degraded_mode`, and a declare-only scenario lands `verdict: null` / `KnownResidual` by either route —
which is the expected row shape for these legs, not a defect.
