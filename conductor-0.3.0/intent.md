# Conductor 0.3.0 — intent

**Authored:** 2026-09-11, operator side. **Reader:** `/andromeda-route --version 0.3.0` (Phase A). This file is
human-authored input; no skill writes it.

**Purpose.** 0.2.0 closed at `b54e6ec` with done-test 32/32, and two of its claims closed on a narrowing rather
than on a measurement: `v2-24` is `deferred` with a candidate-list cause, and the P-025 half of Pulse's `P-075`
was routed forward after its own instrument was found unable to resolve the bound. Two more classes were
measured during 0.2.0's last chunks and recorded as residuals rather than fixed. 0.3.0 exists to retire those
compromises, so that switching attention to Pulse leaves nothing here that is true only by wording.

**How to read this.** Each finding is OBSERVED → EXPECT. Every coordinate below was measured at the stated
date against the repository at HEAD (or, for Pulse coordinates, against Pulse HEAD `83d4060`); nothing here is
recalled. The Out-of-scope section names work that must NOT become a chunk, so the intake does not mint it.

---

## F1 — The routine a11y arm is not gated in CI, and the keyboard half its requirement names lives in a different suite

**OBSERVED — two independent gaps behind one deferred capability (`v2-24`, "A11y CI gate and violation JSON").**

Its acceptance reads: the routine a11y specs — axe under `wcag2a`/`wcag2aa`/`wcag21aa`, the colorjs.io token-pair
contrast ratios, and the hold-free Operable pair (SC 2.1.1 keyboard reachability of the idle console's controls,
SC 2.4.3 focus order per a11y-plan §5 `run-console-idle`) — run in the `a11y` CI job on the measured Windows
WebView2 runner.

1. **The endpoint never appears on the hosted image, and the cause is unmeasured.** Locally the driven arm passes
   over the shipped bundle: 12 passing / 2 skipped (the expected skip set), banner `[webview2 152.0.4191.66
   windows]`, measured 2026-09-10 during the release chunk (implement, re-run verbatim at its light gate). On the runner the app boots — `EBWebView`
   present, three `msedgewebview2` children, the staged `conductor-tauri.jsonl` shows the UI calling
   `list_scenarios` / `coverage_matrix` / `run_report` / `run_envelope` within one second — yet
   `DevToolsActivePort` is never seen within 90 s. The reading is identical across CI runs `34162118841`,
   `34251573399`, `34256490781`, `34280136892` and `34365300658`, and identical under runtime 151 and 152, which
   falsifies the runtime-major hypothesis. `tauri-driver`'s bind on `:4444` is intermittent (four `ECONNREFUSED`
   in 1.56 s on one run, two on the next) and is therefore not the stable obstacle. What remains UNMEASURED: the
   WebView2/Edge policy state on the image (a registry read is a one-line probe), the module version the host
   processes actually loaded (image names only were captured), and any session / service-account property of the
   runner. The current record names these as candidates; none has been probed on the runner.
2. **The stated keyboard half is not in the routine arm at all.** Measured at the 2026-09-07 chunk's P3: the
   routine arm carries ZERO `browser.keys` calls — every keyboard, focus-trap and restoration assertion lives in
   `operator-hold.e2e.ts`, the driven suite. The capability's concretization failed on exactly this at that
   chunk's P5 and the cap was left pooled. So even with the endpoint problem solved, the requirement as written
   would still not be satisfiable by the routine arm.

**EXPECT.** The runner cause is measured rather than listed — one probing pass with named one-line probes, whose
reading is recorded whatever it says — and the capability then reaches one of two honest terminals: the routine
arm runs green in the `a11y` CI job, or the CI half is closed as a ratified permanent exclusion whose basis is a
MEASURED cause and whose owner is named. Independently, the requirement and the suites agree about where keyboard
and focus-order coverage lives: either the routine arm carries it, or the requirement names the driven suite as
its owner and says what CI gates instead.

---

## F2 — Committed scenarios carry checks that cannot pass by construction, and tiers below their own phase duration

**OBSERVED — one class, three shapes, all measured live during 0.2.0's last two chunks.**

1. **The `CountAtLeast` family is structurally unsatisfiable.** Conductor sums `evidence_count` from `span_refs`
   (`crates/conductor-verify/src/extract.rs:116`) and grades every `CountAtLeast` against that count whatever the
   scenario declares (`extract.rs:55`); Pulse writes `EvidenceRefs.span_ids: Vec::new()` at its only production
   incident-construction site (`andromeda-pulse pulse-app/src/inference_runtime.rs:871`) and
   `incidents_router.rs:67` derives `evidence_count` from that same empty vector. A `CountAtLeast >= 1` therefore
   evaluates `0 >= 1` forever. Three committed scenarios carry a live `kind = "CountAtLeast"` key:
   `scenarios/constellation-severity-live-wiring.toml:53`, `findings-counter-refresh.toml:49`,
   `pulse-run-contract.toml:58`. The row is non-blocked only because an unmet sample floor routes to
   `ClaimClass::CalibrationRegion` (`conductor-verify/src/slo.rs:97-100`) — the verdict is honest, the assertion
   is empty.
2. **A Hard `Contains` on an inferred token measured dead on its first live drive.**
   `scenarios/cross-incident-recurrence.toml:42-45` declares `Contains` / `Hard` / `"Previously seen"`; the
   scenario's own header concedes the token is inferred. Driven live on 2026-09-10 it returned `[FAIL]`, exit 1,
   state `Fail`, with read-back succeeding — the corpus was read and the token is not in it.
3. **Tier declarations below their own phase duration.** Of 36 committed scenarios, 17 declare an SLO tier
   shorter than the sum of their own phase `gap_ms`: 9 are the operator-ratified `<90s`-as-honest-bucket posture,
   2 exceed every tier in the closed set (`ack-cooldown` 370 s at `<20s`, `severity-tier-autonomous` 120 s at
   `<5s`), and 6 could hold a larger tier. `constellation-severity-live-wiring` carries this defect on top of
   shape 1 — measured 30 212 ms against its own `<20s` deadline.

Retiring any of these is not a TOML-only edit: the declarations are pinned by tests, and at least one
(`crates/conductor-core/src/scenario.rs:1188-1202`) is an inline `#[cfg(test)]` module inside `src/`, which the
project's prescribed companion sweep (`grep -rln "<scenario-name>" crates/**/tests`) structurally cannot see. The
precedent for the class is recorded at `.andromeda/architecture-amendments.md:403` (2026-08-21), the ground on
which five severity-lifecycle scenarios were retired to declare-only.

**EXPECT.** Every committed scenario's declared checks are satisfiable by construction against a live Pulse, or
are retired to declare-only **as a class in one pass** with their pinning tests updated in the same change; and
every scenario's tier either fits its own phase duration or carries a stated reason why it does not. The outcome
is auditable by a mechanical check that a fresh reader can re-run.

---

## F3 — The hue budget is graded through an instrument that cannot resolve it (cross-project; blocked on Pulse)

**OBSERVED.** Pulse delegated four timing budgets to Conductor. Three are graded hard at real measured values and
are verified under `v2-20`: P-027 constellation discovery 702.4 ms against ≤5000, P-037 report render 1 ms and
0 ms against ≤2000, P-045 counter refresh worst 7.0 ms of 269 in-window samples against ≤1000. The fourth, P-025
(hue shift ≤2 s), measured 35 581 ms and 36 705 ms on 2026-08-21, and `v2-20`'s acceptance was narrowed to three
budgets by operator ratification with P-025 routed forward. The 2026-09-06 re-drive concluded the bound is
UNMEASURABLE through `metric.constellation.hue_update_ms`. Measured at Pulse HEAD `83d4060`: that metric is a
Tauri command (`record_constellation_hue_latency`) whose `duration_ms` is computed in the FRONTEND canvas
(`pulse-app/ui/src/canvas/frame-metrics.ts`) and merely logged by the backend
(`crates/ui-bridge/src/telemetry.rs:279`); its start point is tied to a tick-refreshed `last_seen`, so its
resolution is the lifecycle tick, not the flip. The 35 s figure is therefore the reading of an instrument that
cannot see the quantity, not a proven pipeline latency — the true hue-shift latency is at present measured by
nothing.

**EXPECT.** Conductor states the measurement contract P-025 needs — which observable, at what resolution, over
what window, and what would constitute a hard grade — in a form Pulse can implement; and once Pulse emits it,
Conductor re-drives `halo-hue-encoding` and grades P-025 hard at its real value, restoring the fourth budget.

**SEQUENCING.** The instrument is Pulse's to ship. The re-drive entry is therefore `BLOCKED-ON` that Pulse-side
emission and must not be placed as if Conductor could satisfy it alone; the contract half (what Conductor needs)
is unblocked and can land first.

---

## F4 — Every live leg runs under canned L4, so Conductor proves plumbing and never interpretation

**OBSERVED.** Every live leg in 0.2.0 ran with `ANDROMEDA_PULSE_L4_DETERMINISTIC=true`, where a canned
`L4Output` replaces the inference. That is what makes the legs reproducible — the in-lane
scenario round's light gate re-measured its three legs within 34 ms of the implement round (18 169 → 18 145 ·
35 120 → 35 111 · 30 212 → 30 246 ms), and the release chunk's legs reproduced within milliseconds too — and it
is also what bounds what they prove:
"Conductor green" today means Pulse's plumbing carried a known payload, never that Pulse's interpretation is
trustworthy. The diagnostic-quality cluster P-031 / P-033 / P-034 / P-044 is pinned meanwhile in
`conductor_core::UNBACKED_AUTO` (`crates/conductor-core/src/drift.rs:61`, asserted by `check_scenario_backing`)
rather than exercised. Pulse's own memo calls the real-model path the single most important test in all of Pulse.
The prerequisite Conductor owed for it is already met: `v2-09` (real per-check read-back extraction) is
`verified` in 0.2.0. Recorded as a residual on 2026-08-09 with `target: 0.3.0`.

**EXPECT.** A live leg exists that runs with deterministic mode OFF, injects a known root cause, and asserts that
the top hypothesis identifies it — with the non-determinism handled as a property of the leg (operator-gated,
never a CI gate, its grading stated in advance) rather than by retrying until it passes. The diagnostic-quality
cluster is then backed by an exercised path instead of a drift pin.

---

## Out of scope for 0.3.0 — named so the intake does not mint work for them

- **`incident_events`-through-MCP.** Measured 2026-09-01 at Pulse HEAD `83d4060`: zero references in
  `crates/mcp-server`; the table is written by `crates/triage/src/incident/persistence.rs::save_incident_event`
  and read only corpus-side. No MCP tool surfaces it at any width, so Conductor cannot reach it. This is a
  PULSE-side capability gap already routed to Pulse's 0.4.0 incubator; it stays a residual here.
- **Pulse's `P-075` bookkeeping.** Whether Pulse's own capability closes on three budgets or waits for the fourth
  is a decision in Pulse's ledger, not Conductor work. F3 supplies the measurement Pulse would need either way.
- **Release and bundle work.** Done in 0.2.0: `cargo build --release --workspace`, `cargo tauri build` (host CLI
  `tauri-cli` 2.11.4), installers nsis 4.21 MB and msi 5.87 MB, supply chain green at 1243 advisories / 562
  crates / 7 allowed warnings. Nothing is owed here unless a 0.3.0 change moves it.

## Standing facts route should not re-derive

- 0.2.0 closed at `b54e6ec`; done-test 32/32 (31 verified + `v2-24` deferred, unclaimed 0); the working route's
  tail is empty and master carries no `pending` record.
- The intake's three sources are loaded: four `open` entries in `.andromeda/residuals.md` (interpretation
  correctness · `incident_events` · `v2-24` · the dead-assertion class), the prior matrix's one `deferred` entry
  (`v2-24`), and the prior requirements' carried-residuals section.
- `v2-09` and `v2-20` are `verified`; `v2-20` carries the ratified narrowing to three budgets, so F3 restores a
  budget rather than re-opening a closed claim.
- Capability ids continue the project scheme (`v2-NN` ran to 32 in 0.2.0); the `refs/` capability spec keeps its
  own `P-NNN` space and is never minted into.
