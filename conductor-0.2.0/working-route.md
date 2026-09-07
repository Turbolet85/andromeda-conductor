# Working Route — conductor-0.2.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve then skips frozen lines); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation: re-aim at the SUT
[2026-08-08-sut-capability-manifest] SUT capability manifest — validated versioned refs/ Pulse artifact as the accepted P-ID source, replacing the compile-time 001..=060 bound, malformed reporting blocked
   ↓
[2026-08-09-sut-drift-check] SUT-drift check — loud failure when Pulse's ledger advances past the capability set Conductor knows
   ↓
[2026-08-08-dependency-advisory-remediation] Dependency advisory remediation — crossbeam-epoch ≥0.9.20 and quick-xml ≥0.41 through the Tauri tree, restoring cargo-audit and cargo-deny green
   ↓
[2026-08-09-current-sut-coverage-classification] Current-SUT coverage classification — capabilities in the manifest as auto, drive+observe, static-only or not-Conductor's, the boundary a recorded decision
   ↓
[2026-08-09-out-of-scope-classification-treatment] Out-of-scope classification treatment — not-Conductor's rows distinct in coverage matrix, footer roll-up and CLI table, never Blocked or Fail
   ↓
[2026-08-09-interpretation-correctness-posture] Interpretation-correctness posture — one real-model leg with a known root cause and asserted top hypothesis, or a recorded deferral naming its owner
   ↓
[2026-08-09-in-lane-sut-scenarios] In-lane SUT scenarios — live service truth, Investigate result and single-sourced workspace key (P-067, P-072, P-079)
   ↓
[2026-08-09-sut-load-envelope] SUT load envelope — proven-good storm bounds for scenario authors plus environment-suspect flagging of over-envelope runs

### Epoch 2 — Live-path enablement
[2026-08-10-workspace-key-divergence-probe] Workspace-key divergence probe — two-launch verdict plus a host-path-free named precondition replacing the opaque corpus Blocked
   ↓
[2026-08-10-pulse-run-contract] Pulse run contract — deterministic-L4 mode and shared data-dir asserted at preflight, unmet terms named (P-073)
   ↓
[2026-08-10-scenario-run-root-span-tree] scenario.run root span tree — the must-trace self-observation spans beneath each scenario run
   ↓
[2026-08-11-faithful-emission-dispatcher] Faithful emission dispatcher — per-phase telemetry shaping over the emit primitives, replacing one-signal-per-phase
   ↓
[2026-08-13-dispatcher-determinism-goldens] Dispatcher determinism goldens — same scenario and seed yielding an identical emission stream shape over the new per-phase dispatcher
   ↓
[2026-08-13-per-check-read-back-extraction] Per-check read-back extraction — observed values from the corpus tools feeding the unchanged evaluate/classify path, degraded_mode mapping to KnownResidual
   ↓
[2026-08-13-first-live-green-preflight] First live green preflight — ready:true against a real Pulse with journal and runs.db evidence

[2026-08-14-canary-fingerprint-feed-capture] Canary fingerprint-feed capture — why Pulse's storm detector tracks zero fingerprints while nine spans arrive and both sides agree on the attribute keys, settled by a capture taken while Pulse is running
   ↓

### Epoch 3 — Live proof: the five families
[2026-08-15-canary-storm-autonomous-band] Canary storm inside Pulse's Autonomous band — the preflight canary forms a Tier-1 incident and preflight reaches ready:true against a live Pulse
   ↓
[2026-08-15-canary-spans-pulse-fingerprints] Canary spans that Pulse fingerprints — the buffer enumerates the canary's span events and its fingerprint observer runs, as it already does for inject_demo
   ↓
[2026-08-16-canary-fingerprint-derivation-aligned] Canary fingerprint derivation aligned — Conductor's expected fingerprint is what Pulse actually derives, so the round-trip's last precondition can pass
   ↓
[2026-08-16-fault-application-spans] Fault-application spans — silence, ramp and port-occupier phases observable beneath the timeline span with duration and journal offset
   ↓
[2026-08-16-fingerprint-storm-live-proof] fingerprint-storm live proof — identity triple, storm cue thresholds and exactly-one-incident coalescing (P-017, P-018, P-074)
   ↓
[2026-08-17-fingerprint-semantics-token-leading] Fingerprint semantics re-aligned to token-leading — identity-triple premise, variant tests, the P-017 clause and storm grading against the SUT's current derivation
   ↓
[2026-08-18-error-baseline-spike-live-proof] error-baseline-spike live proof — baseline convergence, ramp and candidate persistence over the sample floors (P-009..P-012)
   ↓
[2026-08-18-restart-suppression-live-proof] restart-suppression live proof — gap/resume restart event, surgical suppression window and the bypass triple (P-015, P-016, P-057)
   ↓
[2026-08-19-pii-scrub-live-proof] pii-scrub live proof — seven PII categories scrubbed across corpus and report excerpts, verdicts recorded without persisting excerpt content (P-035, P-047, P-048)
   ↓
[2026-08-19-connection-lifecycle-live-proof] connection-lifecycle live proof — Listening/Receiving/Idle/Stalled walk plus the :4317 port-occupier leg (P-001..P-004)
   ↓
[2026-08-20-latency-regression-re-proof] latency-regression re-proof — a regression short relative to accumulated history, read back on a surface that survives the window (P-011, P-012)

### Epoch 4 — Lifecycle & delegated timing
[2026-08-20-verifier-self-hardening] Verifier self-hardening — the mutation-audit survivor families gain killing assertions and the suite passes under either test runner (code-audit baseline 2026-08-20, §B1–§B3)
   ↓
[2026-08-20-read-back-seam-survivors-closed] Read-back seam survivors closed — the five out-of-family audit survivors gain killing assertions (code-audit baseline 2026-08-20, §B2 remainder)
   ↓
[2026-08-21-severity-lifecycle-live-proof] severity-lifecycle live proof — auto-resolve and resolution summary observed through read-back (P-019..P-023, P-059, P-060)
   ↓
[2026-08-21-per-check-latency-measurement] Per-check latency measurement — sub-5s budgets beneath the closed slo_tier set, per-check latency_ms carried in the run-report envelope
   ↓
[2026-08-21-delegated-timing-budgets-proven] Delegated timing budgets proven — halo hue, constellation discovery, report render and counter refresh at real values (P-025, P-027, P-037, P-045)
   ↓
[2026-08-22-operator-pause-and-checklist-live-firing] Operator-pause and checklist live firing — frozen-count hold-point, go/no-go hold and ManualCheck items exercised against a running Pulse

### Epoch 5 — Verification surfaces
[2026-08-31-p-075-assert-round] P-075 assert round — the read-back fidelity Pulse's last 0.3.0 capability can honestly claim, and the payload-level premise measured false
[2026-09-01-webview-self-verify-windows-host] Webview self-verify on the Windows host — the driver path measured, not inherited: a real WebView2 session drives the release bundle here, and four defects a never-executed config had hidden are fixed
   ↓
[2026-09-01-desktop-a11y-sweep] Desktop a11y sweep — operator judgment pass over the accessible paths, with axe and routine affordance proof agent-driven
   ↓
[2026-09-01-live-per-p-id-verdict-lamps] Live per-P-ID verdict lamps — coverage rows carrying each capability's actual verdict state, not-yet-run rows visibly distinct from failed
   ↓
[2026-09-02-screen-reader-manual-spec] Screen-reader manual spec — per-state NVDA/VoiceOver/Orca must-announce pass spec for the four run states
   ↓
[2026-09-02-cross-surface-envelope-parity] Cross-surface envelope parity — CLI and Tauri identical envelope for one seed, with the stale rmcp wording reconciled

### Epoch 6a — Verification follow-ups

[2026-09-02-mutation-tier-restored-for-conductor-tauri] Mutation tier restored for conductor-tauri — the parity test's CLI binary declared in the build graph rather than found by chance
   ↓
[2026-09-03-conductor-tauri-survivors-dispositioned] conductor-tauri survivors dispositioned — the crate's first mutation score turns into a killing test or a cited accepted-deliberate entry per survivor
   ↓
[2026-09-03-conductor-run-composition-root-survivors-dispositioned] conductor-run composition-root survivors dispositioned — every standing mutation survivor gains a killing test or a cited accepted-deliberate entry
   ↓
[2026-09-03-live-pulse-preconditions-probed] Live-Pulse preconditions probed before a leg is scheduled, so an absent SUT is named once rather than absorbed per chunk
   ↓
[2026-09-04-sr-findings-remediation] SR findings remediation — the eight remaining screen-reader findings from the NVDA pass fixed at their defects, then the sr* leg re-run against the spec
   ↓
[2026-09-04-preconditions-probe-reads-path-handles-by-presence] Preconditions probe reads path handles by presence — `handles-declared` is satisfiable and `boot` reaches the preflight
   ↓
[2026-09-04-sidecar-spawn-without-a-console-window] Sidecar spawn without a console window — the GUI-launched MCP sidecar stops raising a foreground terminal pane that steals focus and shows its path

### Epoch 6b — Polish & ship

[2026-09-05-audit-corrective] Audit corrective — render.rs survivors dispositioned, civil_from_unix boundary-date kill, jsonrpc.rs timeouts owned, M2's three dedups, and conductor-run/src/lib.rs split along named seams with its inline tests moved to tests/
   ↓
[2026-09-06-operator-gated-live-suite] Operator-gated live suite — re-runnable live-Pulse proof invocation carrying its evidence, never a CI gate
   ↓
[2026-09-06-run-report-envelope-conformance-gate] Run-report envelope conformance gate — every run journal row schema-complete and host-path-free, build failing on violation
   ↓
[2026-09-06-coverage-completeness-gate] Coverage completeness gate — zero-gap classification over the current SUT set with every CI gate green
   ↓
[2026-09-06-halo-hue-budget-re-driven] Halo hue budget re-driven — an error stream sustained through incident formation so the severity tier flips while the service is still emitting (P-025)
   ↓
[2026-09-07-dependency-polish] Dependency polish — indicatif bump preserving the stop-in-place hold spinner, plus opentelemetry-proto default-features trim
   ↓
A11y CI gate — the a11y specs as a CI gate on whatever runner the project gets, without Pulse, failing on any violation; redacted service-tagged JSON aligned to the obs-owned format. Linux+xvfb was an ASSUMPTION, not a capability verdict — the Windows WebView2 path is the measured one (2026-09-01/02), so the gate is runner-agnostic and only the CI arrangement is open   BLOCKED-ON: one green CI run of the a11y job after a push — the push is the operator's act. Two facts, both measured at HEAD `480bc66` (2026-09-04): both `ci.yml` jobs already run on `windows-latest` (`.github/workflows/ci.yml:18` rust gate, `:164` frontend gate), so a CI runner EXISTS; and the a11y job itself is absent from that workflow, while `origin/build/conductor-0.2.0` stands at `dd15dc3` (2026-08-09) with the local branch 49 commits ahead and unpushed — CI has not run in a month. This SUPERSEDES the 2026-08-22 wording ("a CI runner (none exists; Windows-only host)"), which was inherited from the retired Linux-runner block (`.andromeda/runs/2026-08-22T21-17-21Z-wrap/adaptation-record.md`) and is measurably false; corrected by operator directive at the 2026-09-04 wrap (`.andromeda/runs/2026-09-04T01-35-00Z-wrap/adaptation-record.md`). The trajectory question stays dispositioned; /andromeda-phase Setup refuses promotion until that run exists, which is intended.   CARRY (from `2026-09-02-screen-reader-manual-spec`, plan §Implementation notes): the ui test README (`crates/conductor-tauri/ui/test/README.md`) still opens "This leg runs only on Linux + xvfb against a live Pulse" — a non-master project doc the 2026-09-01 sweeps measured false (the leg runs headfully on the Windows host; only macOS is driver-less) and that chunk deliberately left standing, editing only its pointer sentence. Reword it at THIS entry, which touches the ui test tree, to the measured platform set.   CARRY (from `2026-09-04-sr-findings-remediation`, operator WRAP directive 2026-09-04 item 3 — this entry owns the a11y suite): tighten the operator-checklist spec's subject-absent guard from a bare `[role="status"]` to `[role="alertdialog"]`. Measured: the bare token stands in for "a hold is raised" and was sound only while the checklist roll-up was the app's only `role="status"`; two a11y-plan-mandated live regions added at that chunk satisfied it, the skip stopped firing, and the spec asserted checkboxes it found none of. Worked around there by taking `aria-live` instead, which leaves the guard still loose for the next `role="status"` anyone adds.   CARRY (from `2026-09-04-sidecar-spawn-without-a-console-window`, operator WRAP directive items 1+2, placement ruled at that wrap's P5 halt): **three SR-leg findings, all measured, none fixed.** (1) **The load-time alert is NOT announced** — R0-01 graded `not-announced` with `heard: []` once the reload was dropped, so the prior chunk's fix (four `role="alert"` regions mounting EMPTY at first paint) is necessary but NOT sufficient: NVDA binds a window on its FIRST FOCUS EVENT, which necessarily follows a load-time error, so the text arrives in a region nobody is listening to yet. The remedy is a RE-ANNOUNCE mechanism (re-assert the message after first focus), not another mount tweak — that shape is disproved. Row R0-01 and its `expected` already say silence here is a FINDING, never a pass. (2) **The sequential-focus start point is still wrong** — `tabs_to_start` measured live 3 · empty 6 · error 5 with `initial_focus: "BODY"`, unchanged by that chunk. NEW fact from it: `browser.refresh()` had been RESETTING the start point, which is why dropping the reload exposed the defect at R0-02 (`the first Tab landed on "Close window", not "Minimize window"`). The holder is STILL UNMEASURED — cmdk 1.1.1 has no `autofocus`, both its `.focus()` calls are guarded, no `src/` file focuses at mount, yet the leg's own comment attributes it to "the picker input at mount". Measure WHICH element holds Chromium's sequential-focus-navigation starting point before designing any fix; the leg's `tabsToStart` cycling loop would mask one. (3) **The leg has real run-to-run announcement variance** — a harness-reliability finding against test-plan §10's zero-flake bar, recorded here rather than passed: two identical runs graded S0-01/S0-02 `not-announced` then `announced-as-expected`, and three `empty`-subject rows (E0-06/E0-07/E0-09) moved the same way, correlating with that subject's `nvda_named_window` flipping. Consequence for anyone reading this leg: ONE observation cannot separate variance from regression — take a second before concluding, as that chunk did to exonerate its own change.   CARRY (from `2026-09-06-operator-gated-live-suite`, wrap P2 fan-out): a PRE-EXISTING internal inconsistency in `a11y-plan.md`, surfaced by that wrap's a11y doc-agent as explicitly out of its detectors' scope and therefore proposed by nobody — `:565` still reads "reduced-motion emulation is the one platform-dependent assertion", a verdict `:218`, `:424` and `:597` already retired in an earlier chunk. Not that chunk's drift (its report changed no a11y fact), so it had no amendment channel; recorded here because this entry owns the a11y surface. Re-verify the four line numbers before acting — they are the agent's, unre-derived at that wrap.   CARRY (from `2026-09-07-dependency-polish`, operator wrap-directive item 2 — this entry named as the owner, the do-it call staying the founder's at promotion): **`knip` is installed on the web plane but its series is unusable — a `knip.json` entry-point config is OWED.** That chunk added `knip` as a `crates/conductor-tauri/ui` devDependency plus a `knip` npm script (report-only, no gate, `npm audit --omit=dev` still 0 production vulns and knip added ZERO advisories — 38 dev-tree before and after). Its first run returned **20 findings, all 20 false positives of ONE class**: WebdriverIO discovers specs through `wdio.conf.ts`'s `specs`/`suites` config and loads devDependencies through its own plugin resolution, and neither is an import edge, so knip's reachability graph cannot see the package's largest consumer — it flagged the three `test/a11y/*.e2e.ts` specs as unused files and five devDeps as unused, four of which (`@axe-core/webdriverio`, `axe-core@4.12.0`, `colorjs.io@0.6.1`, `lighthouse@13.0.3`) are a11y-plan §3-mandated BY NAME AND PIN. Nothing was deleted; every finding is dispositioned in that chunk's `evidence/knip-first-report.md`, and a11y-plan §11 forbids acting on such a report against the harness. This entry is the named owner because it owns the ui test tree and those specs ARE the false positives; knip ships a `wdio` plugin for exactly this. **Until the config lands, the next code-audit record's A5 `dead-code-web` column must read "tool present, series unusable (100 % FP)" — never a usable series, and never `tool-missing`, which is now false.**
   ↓
Release build and bundle — release binary plus Tauri 2 bundle with a final SLO verification pass   CARRY (from `2026-09-03-live-pulse-preconditions-probed`, operator WRAP directive item 1, verified first-hand at HEAD): the process-global panic-hook RACE — `conductor-core/src/obs.rs` takes and restores the global hook at TWO sites (`:491-496` and `:532-537`), so under `cargo test` (one process, parallel threads) one restoring the previous hook mid-flight leaves the other's deliberate panic handled by the DEFAULT hook and its buffer empty. Observed red exactly once, on the first run after that chunk added 14 core tests and shifted scheduling; passes since and did not reproduce at HEAD in 3 forced-parallel runs — a race that passes is still a race, and the zero-flakiness rule owns it (testing.md §Quality gates: a runner-dependent result is a shared-state defect to remove at the cause). Pre-existing and UNMASKED, not caused: both sites are in HEAD's source. Test-side remedy — serialize the two against each other, or give them one shared guard; note relocating them to their own test binary would widen `conductor-core`'s public API for tests alone, since both use crate-private helpers. Land it with this entry's final verification pass.   CARRY (from the 2026-09-05 0-pending adaptation, operator-reported — measured at the SR chunk per the directive: a wdio run printing `0 passed, 1 failed` while `agent-run.ps1 --e2e` exited 0, and the PRINTED verdict, not the exit code, is what caught two regressions there; the artifact carrying that line is not in the chunk folders, so re-measure at take-up before designing): **`agent-run.ps1 --e2e` can exit 0 over a RED wdio suite.** Source facts at HEAD: the ps1 arm does read `$LASTEXITCODE` after `& npm run a11y` (`scripts/agent-run.ps1:119-120`) and the sh arm runs `npm run a11y` under `set -euo pipefail` (`agent-run.sh:18`, `:105`), so the leak — if it reproduces — sits BELOW npm (hypothesis: wdio's launcher exiting 0 after its `onComplete` teardown, or the `npm.cmd` shim's status under PowerShell; `wdio.conf.ts`'s two `process.exit(0)` sites at `:269` / `:288` are the documented handle-unset SKIPs, not this path). Remedy ruled by the founder: the ps1 arm ASSERTS the runner's PRINTED verdict (the `Spec Files: N passed, M failed` summary) and fails on any `failed` > 0, so a red suite can never ride a green `--e2e`; mirror the assertion in the sh arm for parity. Belongs with the final verification pass because a release gate that trusts this exit code is hollow.
