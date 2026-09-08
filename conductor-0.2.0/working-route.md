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
[2026-09-07-a11y-ci-gate] A11y CI gate — the a11y specs as a CI gate on whatever runner the project gets, without Pulse, failing on any violation; redacted service-tagged JSON aligned to the obs-owned format. Linux+xvfb was an ASSUMPTION, not a capability verdict — the Windows WebView2 path is the measured one (2026-09-01/02), so the gate is runner-agnostic and only the CI arrangement is open
   ↓
[2026-09-07-sr-findings-fixed] SR findings fixed — re-announce after first focus (1), sequential-focus start point measured then fixed (2), announcement variance recorded (3)
   ↓
[2026-09-08-hosted-runner-webview2-session] Hosted-runner WebView2 session — the a11y job's first green run (v2-24 claimed or deferred)
   ↓
[2026-09-08-webview2-runtime-152-installed-in-job] WebView2 runtime 152+ installed in-job — the one variable every passing case shares and the failing case lacks (v2-24 claimed or deferred)
   ↓
Workspace formatting pass and a fmt CI gate — the tree made `cargo fmt --check`-clean and the gate added to CI so it cannot re-rot   CONTEXT (minted at the `2026-09-08-webview2-runtime-152-installed-in-job` wrap on the operator's directive; placement operator-chosen at that wrap's P5): the red has NO in-diff cause — measured at that chunk, `cargo fmt --check` exits 1 with 282 flagged sites across ~40 files spanning nearly every crate, while `git status --porcelain` carries no `.rs`, so it reproduces unchanged at HEAD. `grep -n 'fmt' .github/workflows/ci.yml` returns 0: no CI job has ever run it, which is why the tree could rot unobserved. Root cause supplied by the operator, measured pipeline-side (backlog W90): `rustfmt.toml` (edition 2024) shipped 2026-09-05 with only `commands.rs` re-sorted, and CI never gained a fmt step. EVIDENCE: the invocation is NOT the cause — `rustfmt.toml` is `edition = "2024"` (stable, no unstable options), the toolchain is pinned 1.95.0 with rustfmt as a component, and the diffs are ordinary edition-2024 import ordering and line wrapping. test-plan §9's Lint row (`:455`, with the failure-condition mentions at `:477`/`:507`) is a TARGET-state descriptor deliberately left standing — this entry makes it TRUE rather than retiring it, per the `2026-09-06-coverage-completeness-gate` narrow ruling. `hypothesis:` a formatting-only diff can be partitioned from semantic change by byte-identity against `rustfmt --config-path rustfmt.toml`, keeping review cheap at ~40 files — the technique is established (`2026-09-06` crate-root recursion) but unmeasured at this scale.
   ↓
Release build and bundle — release binary plus Tauri 2 bundle with a final SLO verification pass   CARRY (from `2026-09-03-live-pulse-preconditions-probed`, operator WRAP directive item 1, verified first-hand at HEAD): the process-global panic-hook RACE — `conductor-core/src/obs.rs` takes and restores the global hook at TWO sites (`:491-496` and `:532-537`), so under `cargo test` (one process, parallel threads) one restoring the previous hook mid-flight leaves the other's deliberate panic handled by the DEFAULT hook and its buffer empty. Observed red exactly once, on the first run after that chunk added 14 core tests and shifted scheduling; passes since and did not reproduce at HEAD in 3 forced-parallel runs — a race that passes is still a race, and the zero-flakiness rule owns it (testing.md §Quality gates: a runner-dependent result is a shared-state defect to remove at the cause). Pre-existing and UNMASKED, not caused: both sites are in HEAD's source. Test-side remedy — serialize the two against each other, or give them one shared guard; note relocating them to their own test binary would widen `conductor-core`'s public API for tests alone, since both use crate-private helpers. Land it with this entry's final verification pass.   CARRY (from the 2026-09-05 0-pending adaptation, operator-reported — measured at the SR chunk per the directive: a wdio run printing `0 passed, 1 failed` while `agent-run.ps1 --e2e` exited 0, and the PRINTED verdict, not the exit code, is what caught two regressions there; the artifact carrying that line is not in the chunk folders, so re-measure at take-up before designing): **`agent-run.ps1 --e2e` can exit 0 over a RED wdio suite.** Source facts at HEAD: the ps1 arm does read `$LASTEXITCODE` after `& npm run a11y` (`scripts/agent-run.ps1:119-120`) and the sh arm runs `npm run a11y` under `set -euo pipefail` (`agent-run.sh:18`, `:105`), so the leak — if it reproduces — sits BELOW npm (hypothesis: wdio's launcher exiting 0 after its `onComplete` teardown, or the `npm.cmd` shim's status under PowerShell; `wdio.conf.ts`'s two `process.exit(0)` sites at `:269` / `:288` are the documented handle-unset SKIPs, not this path). Remedy ruled by the founder: the ps1 arm ASSERTS the runner's PRINTED verdict (the `Spec Files: N passed, M failed` summary) and fails on any `failed` > 0, so a red suite can never ride a green `--e2e`; mirror the assertion in the sh arm for parity. Belongs with the final verification pass because a release gate that trusts this exit code is hollow.   **DISCHARGED by `2026-09-07-a11y-ci-gate` (operator wrap-directive item 3b):** both shells now assert the runner's PRINTED verdict at identical semantics — the `Spec Files:` failed count, the per-spec skip tally against the expected-skip SET (a third skip is a failure), and the `[webview2 <version> windows]` driven-session banner — with the exit read from the BARE command, never through a pipe. Two format details the calibration corrected on the way: the `Spec Files:` line OMITS the `failed` term when nothing failed, and the skip tally is `N skipped` plus the per-spec `-` markers, never the token `pending`. Nothing owed here.   CARRY (from `2026-09-07-sr-findings-fixed`, wrap light gate): **the SR leg's run-to-run announcement variance is REAL and is NOT closed.** That chunk inherited it as a finding, measured two runs per subject byte-identical, and concluded it did not reproduce — then the wrap's own light gate ran the live subject a THIRD time on the unchanged tree and it moved: 33 announced-as-expected / 15 not-run-here / 2 subject-absent / **1 not-announced**, the mover being `S3-04` ("Escape resolves NoGo; focus restored", `heard: []`) on a session whose `nvda_named_window` was `false` (`chunks/2026-09-07-sr-findings-fixed/evidence/nvda-pass.live.run3-lightgate.json`, live subject recorded 2026-09-07T22:09:30.914Z; compare `nvda-pass.live.run{1,2}.json`). Rate: 1 row of 51, on the third of three runs — far rarer than the original finding described, which is why two runs missed it. The lesson the CARRY itself stated ("ONE observation cannot separate variance from regression") applies to TWO observations too. It sits against test-plan §10's zero-flake bar, so the remedy is at the cause — no retry, no runner pin, no in-test `sleep(N)` (that carve-out was proposed and REJECTED at `2026-09-06-operator-gated-live-suite`). `hypothesis:` the `nvda_named_window: false` correlation is the thread to pull first — it recurred across sessions and is already recorded as the attach signal. Pinned HERE because this entry's final SLO verification pass is the plausible owner; if it wants its own entry, that is the operator's call at promotion.
