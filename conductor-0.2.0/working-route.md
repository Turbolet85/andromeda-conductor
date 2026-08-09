# Working Route — conductor-0.2.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve then skips frozen lines); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation: re-aim at the SUT
[2026-08-08-sut-capability-manifest] SUT capability manifest — validated versioned refs/ Pulse artifact as the accepted P-ID source, replacing the compile-time 001..=060 bound, malformed reporting blocked
   ↓
[2026-08-09-sut-drift-check] SUT-drift check — loud failure when Pulse's ledger advances past the capability set Conductor knows   CARRY: the accepted set now ships as `contracts/pulse-capabilities.toml` (`sut_version`/`captured_at`/`capabilities[]`) with `CapabilityManifest::load` + `accepts` — build the drift check over THAT, don't re-source it
   ↓
[2026-08-08-dependency-advisory-remediation] Dependency advisory remediation — crossbeam-epoch ≥0.9.20 and quick-xml ≥0.41 through the Tauri tree, restoring cargo-audit and cargo-deny green
   ↓
[2026-08-09-current-sut-coverage-classification] Current-SUT coverage classification — capabilities in the manifest as auto, drive+observe, static-only or not-Conductor's, the boundary a recorded decision   CARRY: `coverage.rs` is a `&'static str` `[CapabilityRow; 60]` documented "no runtime IO" — manifest-sourcing it is a representation change, and its two exactly-sixty tests move with it   CARRY (from 2026-08-09-sut-drift-check): this chunk OWNS retiring `conductor_core::KNOWN_UNCLASSIFIED` — the drift check asserts the observed gap equals that pin by exact set, so classifying P-061..P-082 without shrinking the pin to `[]` fails `drift::tests::committed_artifacts_match_the_known_gap` with "already classified — shrink the known gap". The empty-pin shape is already proven by `in_sync_with_an_empty_known_gap_passes`, so this is a data edit, not a redesign   CARRY (doc wording, verified first-hand at that wrap, NOT that chunk's drift — dismissed under playbook:46): three specs still name the superseded 60 while the manifest carries 82 — `architecture.md:33` "60-P-ID coverage tables"; `design-system.md:7, :257` "all 60 capabilities" / "the full 60-row wall"; and `layout-templates.md` contradicts itself, `:37` reading "82 loaded (manifest set)" while `:121` still says "the full 60-row wall (P-001..P-060)" — residue of the 2026-08-08 de-hardcoding sweep, which updated the header strip but not the component prose. Reconcile all three here, where the classification becomes manifest-sourced
   ↓
[2026-08-09-out-of-scope-classification-treatment] Out-of-scope classification treatment — not-Conductor's rows distinct in coverage matrix, footer roll-up and CLI table, never Blocked or Fail   CARRY (from 2026-08-09-current-sut-coverage-classification): the mode itself SHIPPED — `CoverageMode::NotConductors` (wire `not-conductors`), 16 rows classified, rendering as text in the Markdown / CLI / webview surfaces and counted by `tally()`; this chunk owns only the VISUAL treatment. `layout-templates.md` §Component — Primary content block 1 now documents the Mode cell and its four values (added at that chunk's wrap by operator decision, accepting this chunk will revisit the same block) — EXTEND that entry with the out-of-scope treatment, don't re-author it. Nit to sweep while there: `crates/conductor-tauri/ui/src/components/CoverageMatrix.css:1` still comments "all 60 capabilities"
   ↓
[2026-08-09-interpretation-correctness-posture] Interpretation-correctness posture — one real-model leg with a known root cause and asserted top hypothesis, or a recorded deferral naming its owner   PREREQ (from 2026-08-09-out-of-scope-classification-treatment): **re-check `cargo audit`** at this chunk's gates. It exited 1 there on `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244` — the RustSec DB itself would not parse (reproduced with a fresh fetch AND with `-n`), against a zero-dependency-delta chunk with both lockfiles un-drifted, so it was taken as a BOUNDED DEFERRAL with `cargo deny check` (all four classes ok) as the overlapping signal. If it is green again, note the heal and close the deferral. **If it still fails, raise the cargo-audit FLOOR to the fixed release** — not a `deny.toml` entry, not a CI edit (`.claude/rules/security.md` 2026-08-09 + `playbook.md` external-decay rule). CI's own `cargo audit` step hits the same wall until upstream heals.
   ↓
In-lane SUT scenarios — live service truth, Investigate result and single-sourced workspace key (P-067, P-072, P-079)   PREREQ (from 2026-08-09-interpretation-correctness-posture): **re-check `cargo audit`** — still red at a THIRD consecutive check, and now proven to be an advisory-DATABASE fault, not a tool fault: installing the latest published 0.22.2 reproduced `duplicate advisory ID: RUSTSEC-2026-0244` byte-identically, so the duplicate id is committed data in RustSec's advisory-db and there is nothing to raise a floor to. Per the amended rule (`playbook.md` external-decay · `.claude/rules/security.md` 2026-08-09 · security-plan §Dependency Security), the remedy is the **bounded wait alone** — re-run it, and verify `cargo deny check` actually ran green as the overlapping signal. Do NOT raise the floor, do NOT add a `deny.toml` ignore, do NOT edit CI. Close the deferral the moment it parses.   CARRY (from 2026-08-09-interpretation-correctness-posture): **P-073/P-074/P-079 sit in `conductor_core::UNBACKED_AUTO`** — the pin of `Auto`-classified capabilities no scenario names, asserted by `check_scenario_backing` at EXACT-SET equality. Authoring a scenario that names any of them WITHOUT removing it from the pin fails the gate with "in the unbacked ledger but now named by a scenario — shrink the ledger". Shrinking the pin is part of this chunk's work, not a follow-up.
   ↓
SUT load envelope — proven-good storm bounds for scenario authors plus environment-suspect flagging of over-envelope runs

### Epoch 2 — Live-path enablement
Workspace-key divergence probe — two-launch verdict plus a host-path-free named precondition replacing the opaque corpus Blocked
   ↓
Pulse run contract — deterministic-L4 mode and shared data-dir asserted at preflight, unmet terms named (P-073)
   ↓
scenario.run root span tree — the must-trace self-observation spans beneath each scenario run
   ↓
Faithful emission dispatcher — per-phase telemetry shaping over the emit primitives, replacing one-signal-per-phase
   ↓
Dispatcher determinism goldens — same scenario and seed yielding an identical emission stream shape over the new per-phase dispatcher
   ↓
Per-check read-back extraction — observed values from the corpus tools feeding the unchanged evaluate/classify path, degraded_mode mapping to KnownResidual
   ↓
First live green preflight — ready:true against a real Pulse with journal and runs.db evidence

### Epoch 3 — Live proof: the five families
Fault-application spans — silence, ramp and port-occupier phases observable beneath the timeline span with duration and journal offset
   ↓
fingerprint-storm live proof — identity triple, storm cue thresholds and exactly-one-incident coalescing (P-017, P-018, P-074)
   ↓
error-baseline-spike live proof — baseline convergence, ramp and candidate persistence over the sample floors (P-009..P-012)
   ↓
restart-suppression live proof — gap/resume restart event, surgical suppression window and the bypass triple (P-015, P-016, P-057)
   ↓
pii-scrub live proof — seven PII categories scrubbed across corpus and report excerpts, verdicts recorded without persisting excerpt content (P-035, P-047, P-048)
   ↓
connection-lifecycle live proof — Listening/Receiving/Idle/Stalled walk plus the :4317 port-occupier leg (P-001..P-004)

### Epoch 4 — Lifecycle & delegated timing
severity-lifecycle live proof — auto-resolve and resolution summary observed through read-back (P-019..P-023, P-059, P-060)
   ↓
Per-check latency measurement — sub-5s budgets beneath the closed slo_tier set, per-check latency_ms carried in the run-report envelope
   ↓
Delegated timing budgets proven — halo hue, constellation discovery, report render and counter refresh at real values (P-025, P-027, P-037, P-045)
   ↓
Operator-pause and checklist live firing — frozen-count hold-point, go/no-go hold and ManualCheck items exercised against a running Pulse

### Epoch 5 — Verification surfaces
Live per-P-ID verdict lamps — coverage rows carrying each capability's actual verdict state, not-yet-run rows visibly distinct from failed
   ↓
Webview E2E harness leg — agent-run's --e2e stage covering the headless tauri-driver run on Linux+xvfb, both shell variants
   ↓
Desktop a11y sweep — operator-gated live-Pulse pass over the four accessible paths: axe, contrast, not-color-alone, keyboard-trap escape, focus restoration, reduced-motion
   ↓
Screen-reader manual spec — per-state NVDA/VoiceOver/Orca must-announce pass spec for the four run states
   ↓
A11y CI gate — same specs on Linux+xvfb without Pulse, failing on any violation; redacted service-tagged JSON aligned to the obs-owned format
   ↓
Cross-surface envelope parity — CLI and Tauri identical envelope for one seed, with the stale rmcp wording reconciled

### Epoch 6 — Polish & ship
Operator-gated live suite — re-runnable live-Pulse proof invocation carrying its evidence, never a CI gate
   ↓
Run-report envelope conformance gate — every run journal row schema-complete and host-path-free, build failing on violation
   ↓
Coverage completeness gate — zero-gap classification over the current SUT set with every CI gate green   CARRY (from 2026-08-09-interpretation-correctness-posture): **the gate now has TWO axes, and this entry owns only the first.** `check_sut_drift` (classification↔manifest set-equality) is what "zero-gap classification" means; `check_scenario_backing` (`Auto`-classification↔scenario-catalog backing, over the `UNBACKED_AUTO` pin) already SHIPPED and is live in nextest — do not rebuild it, and do not fold it into this gate's acceptance. Note the roll-up it feeds: the auto term carries a derived `(N unbacked)` qualifier that is NOT a fifth summand (obs-plan §4 denominator semantics), so a gate reading those fields must keep the four per-mode counts summing to the row total.   CARRY (from 2026-08-09-current-sut-coverage-classification): **decide whether `coverage-matrix.md` becomes a real committed artifact.** `architecture.md` §Occupied Resources registers it as the definition-of-done artifact and `test-plan.md` §1 Critical Path 6 names it as the gate's surface, but the file has NEVER existed in this repo — `conductor coverage --write` generates it on demand and that chunk deleted the one it produced while verifying, since a plan declaring zero new files would otherwise have had `git add -A` commit it. This entry owns the call (commit it + gate on it · keep it on-demand and re-word the two specs · or gitignore it); the count/mode staleness on that same test-plan line was already amended, only the artifact question is open   CARRY (from 2026-08-09-out-of-scope-classification-treatment): **second consecutive chunk to do the create-inspect-delete dance** — that chunk's smoke also had to generate `coverage-matrix.md` to verify the Markdown surface, then delete it for the same reason. The cost is now recurring, not incidental: any chunk touching the Markdown coverage render pays it. Also note the gate's denominator is now pinned — `obs-plan.md` §4 records that `coverage_percent` is computed over the **in-scope** count (manifest set minus out-of-scope rows) while `p_id_count_expected` stays the full manifest count, so build the gate to that split, not to a single total
   ↓
Dependency polish — indicatif bump preserving the stop-in-place hold spinner, plus opentelemetry-proto default-features trim   CARRY (from 2026-08-08-dependency-advisory-remediation): three PRE-EXISTING doc-vs-artifact gaps, none introduced by that chunk, all dismissed there under playbook:46 — arch §Stack/§Established Decisions/§Inherited Defaults name `tokio 1.48.x` (architecture.md:14, :41, :229) while the lock resolves **1.52.3**; arch §Stack/§Inherited Defaults name Tauri `v2.10.x / latest 2.10.1` (:26, :236) while it resolves **2.11.3**; and `deny.toml` carries 17 `[advisories] ignore` + 8 `[licenses] allow` entries while security-plan §Dependency Security's Accepted-exceptions paragraph names exactly one of each (`number_prefix` / RUSTSEC-2025-0119 and the `Zlib` allow). Reconcile the docs to the resolved artifacts here, where the dependency surface is already open   CARRY (from 2026-08-09-sut-drift-check): a fourth doc-vs-artifact gap of the same class, verified first-hand and likewise not that chunk's drift — `obs-plan.md:25` and `:645` both justify the Minimal tier with "8 workspace crates" while the workspace has **9** (stale since `conductor-run` was extracted on 2026-06-26). The tier conclusion is unaffected; only the count is wrong
   ↓
Release build and bundle — release binary plus Tauri 2 bundle with a final SLO verification pass
