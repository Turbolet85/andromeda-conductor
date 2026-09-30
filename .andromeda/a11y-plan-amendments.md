# A11y Plan — Amendments

_Append-only changelog of amendments to `a11y-plan.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-16-emission-journal-writer — violation-schema reproduction gains `read_back_observed_at` (obs bind)
**Section:** §3 Structured violation JSON schema (both verbatim reproductions of the obs Run-report envelope)
**Change:** the reproduced Run-report envelope gains `read_back_observed_at` after `journal_emitted_at`, matching the now-11-field obs §6 envelope. No a11y behavior change.
**Why:** a11y reproduces the obs envelope verbatim (violations fold into `fingerprints[]`), so its reproduction must track every obs envelope field — the a11y↔obs schema bind.
**Ref:** .andromeda/runs/2026-06-16T21-43-46-wrap/

## 2026-06-21-run-report-envelope-serializer — CalibrationRegion→ManualCheck co-occurrence + verdict-first lamp
**Section:** §6 State color tokens — State-naming crosswalk (HOLD + Manual rows)
**Change:**
- The HOLD row's `state` is now `ManualCheck` (the default `CalibrationRegion → ManualCheck` mapping); was "verdict-only/n/a".
- Added the verdict-first lamp-precedence rule: `verdict == CalibrationRegion` ⇒ HOLD lamp regardless of state.
- The Manual row clarified: a `ManualCheck` state WITH a `CalibrationRegion` verdict is the HOLD lamp, not Manual; Manual = `ManualCheck` with no verdict.
**Why:** `Verdict::default_report_state` maps CalibrationRegion→ManualCheck, so `verdict==CalibrationRegion` + `state==ManualCheck` now co-occur and the prior "do not join HOLD on state / NOT the ManualCheck state" design no longer holds. Verdict-first precedence keeps the six lamps distinct. Escalated and user-confirmed.
**Ref:** .andromeda/runs/2026-06-21T18-23-37-wrap/

## 2026-08-09-out-of-scope-classification-treatment — --status-residual's non-lamp reuse scoped out of the six-label assertion
**Section:** §6 Visual Design Verification → State color tokens (not-color-alone)
**Change:** `--status-residual` also tints the coverage-matrix out-of-scope Mode cell (webview `.cov__mode--out-of-scope` · cli ANSI 246 · Markdown emphasis). This is a coverage-mode classification, NOT a seventh lamp state: the six-label not-color-alone assertion keys on the LAMP display labels, and the Mode cell's own `not-conductors` text is its signal (no glyph supplement). Contrast still applies to the pair.
**Why:** §6 bound the token solely to the `Residual` lamp and §10 gates the build on the six-label assertion, so a token-tinted `not-conductors` element would sit uncovered — or be mistaken for a missing seventh state.
**Ref:** .andromeda/runs/2026-08-09T15-56-31-wrap/

## 2026-08-21-delegated-timing-budgets-proven — Residual crosswalk row disambiguated
**Section:** §6 Visual Design Verification -> State color tokens -> state-naming crosswalk, `Residual` row
**Change:** the `Residual` row's obs `verdict` cell now records that a `KnownResidual` state carrying `verdict == CalibrationRegion` is the HOLD lamp, not Residual (verdict-first) — mirroring the `Manual` row's disambiguation; was a bare "(n/a — no verdict)".
**Why:** a live leg produced exactly that governed combination (`CalibrationRegion` + `KnownResidual`), which the row denied; a join from a Residual lamp back to its journal row would have missed a real state.
**Ref:** NOT DERIVED

## 2026-08-22-operator-pause-and-checklist-live-firing — the HOLD focus trap now contains the checklist rows
**Section:** §3 Focus-management harness pattern · §3 Keyboard harness dialog sequence · §4 ARIA pattern catalog (operator-pause dialog keyboard contract; operator-checklist row) · §5 Keyboard Navigation (run-console-HOLD focus order; focus-trap bullet)
**Change:** six sites move from a Proceed/Abort-only trap to one that also contains the operator-checklist `checkbox` rows when the hold declares items: Tab/Shift+Tab cycles across the rows then the actions, Space toggles a focused row, `aria-checked` per row, the unticked roll-up is announced. The §4 checklist row records the second render site and the sibling-of-`Description` placement rule.
**Why:** the HOLD `alertdialog` now renders interactive rows, and every site restated the retired trap composition. The axe/WebdriverIO assertion for the new shape stays deferred, and NO WCAG conformance is claimed from this chunk.
**Ref:** .andromeda/runs/2026-08-22T12-15-00Z-wrap/

## 2026-09-01-webview-self-verify-windows-host — the platform verdict, and the emulate caveat's Chromium half
**Section:** §3 Primary tool per surface (:217) · §3 Bootstrap `a11y-ci-gate-wire` (:299) · §6 Motion tokens Verification (:419) · §9 Platform (:452) · §11 CI stack ban (:560) · §12 Decisions Log (:589)
**Change:**
- The Linux+`xvfb`-only framing is retired as a CAPABILITY claim at every site: the one webview-automation stack runs on Linux+`xvfb` AND, measured 2026-09-01, headfully on the Windows WebView2 host under `CONDUCTOR_MSEDGEDRIVER` (unset ⇒ skip at exit 0); macOS alone stays manual-pass-only. The CI arrangement is unchanged.
- §6's reduced-motion caveat gains its Chromium half: the session connects BiDi, but script evaluation over it answers `Page/Frame is not ready` indefinitely against wry/WebView2 while classic WebDriver succeeds, so the session enforces classic. Whether `browser.emulate` works under classic on WebView2 is recorded, NOT established, so the OS-level / `CONDUCTOR_TEST_REDUCED_MOTION` fallback may be the Windows path too.
**Why:** a11y's detectors cover new-UI-element coverage and violation-JSON-vs-obs-§6 only, so six stale platform claims went unproposed; a platform-claim detector now covers that class going forward. The §6 split honours the epistemic-status rule — claim only what the leg measured and label the untested half. Operator-ratified.
**Ref:** .andromeda/runs/2026-09-01T18-49-38Z-wrap/

## 2026-09-01-desktop-a11y-sweep — reduced-motion emulation retired API-wide; focus restoration is an explicit contract
**Section:** §1 (CI integration, harness spec, operator-pause critical path) · §3 (Configuration, Command, focus-management harness, bootstrap focus-library phase) · §4 (Button row) · §5 (focus trap, focus restoration) · §6 (Motion tokens) · §9 (E2E pipeline row) · §10 (perf budget) · §12 (Decisions Log, open questions)
**Change:**
- `browser.emulate('prefers-reduced-motion', 'reduce')` is retired on EVERY driver: webdriverio 9.x declares exactly six emulate scopes (`clock` / `geolocation` / `userAgent` / `device` / `colorScheme` / `onLine`), reduced-motion not among them. SC 2.3.3 is asserted from the compiled `@media` rule per-declaration, with the limit stated: it proves the rule ships correctly shaped, NOT that the OS preference was exercised. §12's `verify at implement` closed as resolved.
- Focus restoration is NOT the Radix default for this dialog: the hold arrives over a Tauri `Channel`, so there is no `Trigger` and Radix restored to `<body>`. Restoration is an explicit `onCloseAutoFocus` + `restoreFocusTo` contract, and the invoker must stay focusable (`aria-disabled`, never native `disabled`).
- §9/§3/§10 record the routine/driven arm split; §1's headless-Linux+xvfb-only framing is retired to the measured platform SET.
**Why:** both claims measured false. The emulate finding is broader than the earlier recorded-not-established-on-WebView2 caveat, which framed it as a platform question — it never was one. The `aria-disabled` change is kept on independent a11y grounds and is NOT the SC 2.4.3 remedy (the natively-`disabled`-invoker hypothesis was disproved).
**Ref:** .andromeda/runs/2026-09-01T22-22-12Z-wrap/

## 2026-09-01-live-per-p-id-verdict-lamps — `--status-residual` contrast pairs + the load-envelope banner pattern row
**Section:** 1 A11y Scope Summary (Contrast verification harness) + 3 A11y Assertion Harness Contract (Source-of-truth tokens) + 4 ARIA Patterns and Roles (per-component catalog) + 6 Visual Design Verification (Color contrast pairs)
**Change:**
- §6's pair table gains `--status-residual`/`--color-raised-1` and `--status-residual`/`--color-base` at SC 1.4.3 4.5:1 — not the 3:1 a non-text indicator owes, because the token colours TEXT on both its non-lamp surfaces. The same two pairs enter §3's foreground-token enumeration and §1's token-pairs-to-assert list (11 → 13 asserted pairs).
- §4 gains a catalog row for the run-level load-envelope banner: non-interactive text qualifier, no keyboard contract, SCs 1.4.3 + 1.4.1, its rendered-DOM axe recorded UNRUNNABLE on the routine arm.
**Why:** §1 already directed the residual pair to be asserted like any other, but §6 and §3 omitted it — a dangling pointer. The banner is a new element on a must-be-accessible path, and §10's zero-violations invariant scopes the gate to §4's catalog, so the catalog must name it and record its reach limit.
**Ref:** .andromeda/runs/2026-09-02T00-58-00Z-wrap/

## 2026-09-02-screen-reader-manual-spec — the SR pass is agent-driven; the shipped run states, strings and unbuilt surfaces recorded
**Section:** 1 A11y Scope Summary (Run-report view entity · assistive-tech reach · Notes · harness-spec SR bullet · CI integration · View-run-report path) + 3 A11y Assertion Harness Contract (Focus management harness Driver · Screen reader test pattern, both bullets · Bootstrap `screen-reader-test-spec-setup`) + 4 ARIA Patterns and Roles (landmark table) + 5 Focus order (`idle-with-report`) + 9 CI Integration (E2E row) + 11 Anti-Patterns (the skeleton ban's prose samples)
**Change:**
- The SR test pattern reads "agent-driven via NVDA's speech log; the operator reviews; browse-mode rows pending OS-level injection"; was "No automated SR tool exists for the stack" / "Manual" (§1, §3 both bullets, the bootstrap phase). NVDA (Windows) is agent-driven; VoiceOver / Orca are not-runnable-on-this-host. §9's E2E row names three suite families (`sr` / `sr-empty` / `sr-error`, `CONDUCTOR_NVDA`).
- The run-state set is the shipped `idle` / `live` / `hold` / `aborted`, with idle-with-report a sub-state.
- The must-announce strings are the shipped ones (`No scenarios found.`, `No run yet`); no "Run in progress" prose ships.
- `contentinfo` (§4) and the report-site checklist render (§3, §5) are recorded designed-but-unshipped and route-owned; the unticked roll-up is attributed to the checklist view's own `role=status` line.
- §3's focus-harness Driver: the residual "Linux+xvfb target" literal is retired to the measured platform SET.
**Why:** measured by the chunk's `sr*` leg (51 rows, operator-reviewed): 33 rows announced as expected on the agent arm, the 14 browse-class rows unreachable by WebDriver-injected keys, the shipped run states and strings heard live, the landmark set measured banner + main + two regions. The SR-pattern wording is the operator's.
**Ref:** .andromeda/runs/2026-09-02T11-47-51Z-wrap/

## 2026-09-02-cross-surface-envelope-parity — the load-envelope banner's rendered-DOM axe retired from UNRUNNABLE
**Section:** §4 per-component catalog (load-envelope banner row) · §6 Color contrast pairs (the note under the pairs table) · §9 CI Integration (E2E row, routine-arm clause)
**Change:** the banner's "Rendered-DOM axe is UNRUNNABLE on the routine arm" verdict is retired at all three sites.
- §4: it RUNS there — the arm seeds an over-envelope `run_envelope` row through the production writer, the label spec asserts (its `this.skip()` guard removed), and axe under `wcag2a`/`wcag2aa`/`wcag21aa` is clean. Runnability is NOT platform-bound (it holds across the arm's measured platform set; this pass ran headfully on WebView2 152.0.4191.53).
- §6: the `--status-residual` pair is demoted from "its only asserting a11y coverage" to the render-INDEPENDENT half of a two-part coverage.
- §9: the routine-arm carve-out ("only a spec whose DOM state that fixture cannot produce — today the load-envelope banner — context-skips") is removed; the RULE it qualified is kept.
**Why:** measurement disproved the verdict. The §9 site was not in the report's list; a single-site apply would have left the routine arm's own CI row asserting the retired verdict.
**Ref:** .andromeda/runs/2026-09-02T14-34-37Z-wrap/

## 2026-09-03-live-pulse-preconditions-probed — the cli entity named as a verb SET
**Section:** §1 A11y Scope Summary — A11y scope (entities needing assertions), the `conductor-cli` entity
**Change:** the three-verb literal (`conductor-run`, `conductor-suite`, `conductor-report`) is replaced by a set-scoped naming of the whole `conductor` verb surface, so the not-assertable verdict covers verbs added later without restaling.
**Why:** the CLI verb surface moved 5 → 6, and the chunk claimed `a11y n/a` for the new verb under this exclusion — but the enumeration named neither `preconditions` nor `preflight` nor `coverage`, so its scope no longer visibly covered the element added.
**Kept:** other §1 references to `conductor run` paths name a class, not an enumeration, and stay unchanged.
**Ref:** .andromeda/runs/2026-09-03T19-20-00-wrap/
## 2026-09-04-sr-findings-remediation — the picker's filter-miss prose enters the must-announce inventory
**Section:** §3 Screen reader test pattern → Pass spec format, must-announce items (primary) · §11 Anti-Patterns → Visual (the real-prose ban) · §1 A11y assertion harness specification → Screen reader test pattern
**Change:** all three must-announce restatements carry three strings with their owning states; was a conflated pair:
- `No scenarios found.` — the empty CATALOG (`App.tsx`, SR row E0-01);
- `No scenarios match.` — the PICKER's filter miss, announced from a persistently-mounted `aria-live="polite"` region sited outside `Command.List` (SR row S0-16, SC 4.1.3);
- `No run yet`.
**Why:** the picker's announcement (the `FilterMiss` region, S0-16 announced as expected) was absent from the inventory, while the doc's wording made `No scenarios found.` readable as the picker's string.
**Ref:** .andromeda/runs/2026-09-04T07-33-12Z-wrap/

## 2026-09-04-sidecar-spawn-without-a-console-window — the pass spec's path points at the file
**Section:** §3 Screen reader test pattern — Pass spec format
**Change:** the NVDA pass spec is cited at `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md`; was the repo-root-relative `test/a11y/screen-reader/nvda-pass-spec.md`, which resolves to nothing.
**Why:** a distiller quoted the path and the location did not exist; the extract inherited the master faithfully, so the master owned the correction. A path correction is invisible to a11y-plan's detectors (WCAG coverage, obs schema, platform verdict), so this class surfaces only through the orchestrator's expected-amendments floor.
**Kept:**
- The bare `nvda-pass-spec.md` filename in the §3 `screen-reader-test-spec-setup` bootstrap phase carries no path and needs no edit.
- a11y-plan registers no window re-activation step (`reactivateWindow()` / `bringToForeground()`); test-plan §6 owns that registration alone, and retiring `reactivateWindow()` owes no amendment in either master. The report's expected "a11y-plan §3 re-activation registration" amendment was wrong and is not carried.
**Ref:** .andromeda/runs/2026-09-04T20-15-00-wrap/

## 2026-09-06-coverage-completeness-gate — two false present-tense CI-existence claims retired
**Section:** §9 CI Integration → Platform · §3 A11y Assertion Harness Contract → tool pick / Configuration
**Change:** both sites asserted a CI job that did not exist. §9 Platform said the webview a11y path "runs on the `ubuntu-latest` + `xvfb` runner where tests' webview E2E already runs"; it now states that runner as the TARGET, records that neither it nor a webview-E2E job exists in CI today (CI has two `windows-latest` jobs, measured 2026-09-06), names the *A11y CI gate* route entry as owner, and keeps the a11y path operator/local-gated. §3 carried the identical "where tests' webview E2E already runs" phrase and is corrected the same way. Every capability sentence is UNCHANGED — the measured platform SET (Linux under `xvfb` AND headfully on the Windows WebView2 host, macOS manual-pass-only) stands.
**Why:** escalated rather than routine: the gap was pre-existing and the *A11y CI gate* entry was BLOCKED-ON building that job, so a full sweep risked rewriting target state. Operator ruling: NARROW — retire only claims that a CI job exists TODAY.
**Kept:** eight further proposed sites (§1, §3 Command / Driver / Pipeline integration, §9 Pipeline integration ×2, §12 Decisions Log ×2) left standing as target-arrangement descriptors.
**Ref:** .andromeda/runs/2026-09-06T15-59-32-wrap/

## 2026-09-07-a11y-ci-gate — the routine arm is CI-gated; the violation schema's real shape; the Operable carve-out; CARRY 4 closed
**Section:** §1 CI integration · §3 Structured violation JSON schema · §3 tool pick / Configuration · §3 Pipeline integration · §3 Bootstrap `a11y-ci-gate-wire` · §9 Platform · §9 artifacts E2E row · §9 Pipeline integration · §11 Anti-Patterns → CI (×2: the skipping ban, the second-stack ban's platform clause) · §11 Anti-Patterns → Strategy (POUR)
**Change:**
- SEVEN sites said "operator/local gate, not a CI gate" / "None of the three is CI-wired" / "no such CI job exists today"; all now record that the ROUTINE arm is CI-wired since 2026-09-07 as job `a11y` on `windows-2025`, the `driven` and `sr*` arms staying operator-local, and the job's first push-triggered run PENDING — wired, never claimed green. CI coordinates are cited by job NAME, not line.
- §3's violation schema retires the unqualified "no new top-level fields": the shipped artifact is the eleven envelope keys PLUS the obs resource tags (13 local / 15 CI), admitted by obs §3's extension point and by `journal_conformance` asserting key presence, not exclusivity. The scenario-shaped keys are MAPPED, not measured; `fingerprints[]` is one tuple PER NODE.
- §11's skipping ban gains its enforcing MECHANISM (`CONDUCTOR_A11Y_STRICT` + the printed-verdict assertion), since the exit code alone cannot enforce it.
- §11's second-stack ban drops the "reduced-motion emulation is the one platform-dependent assertion" clause (CARRY 4 closed).
- §11 Strategy gains the Operable CARVE-OUT.
**Why:** the schema change is operator-directed: the artifact is correct at 13 keys and the eleven-and-no-more sentence was not, so that criterion is recorded UNMET AS WORDED rather than silently met. The carve-out is operator-selected: the routine arm carries ZERO `browser.keys` calls, all Operable coverage lives in the CI-barred `driven` suite, and closing it with routine keyboard specs would gate on the open sequential-focus defect.
**Ref:** .andromeda/runs/2026-09-07T16-19-12Z-wrap/

## 2026-09-07-sr-findings-fixed — the Operable carve-out narrowed to its hold-dependent half
**Section:** §11 Anti-Patterns → Strategy (the CARVE-OUT) · §5 Focus order per layout (run-console-idle) · §1 Scope Summary → CI integration · §3 Harness Contract → CI integration → Command · §3 → Focus management test harness (Driver) · §9 CI Integration → E2E row · §9 → Pipeline integration · §10 SLO Invariants → per-arm budget
**Change:** eight sites, one claim — "keyboard/focus ⇒ the operator-only driven arm".
- The routine arm now carries the HOLD-FREE Operable pair: SC 2.1.1 keyboard reachability of the idle console's controls and SC 2.4.3 focus order per §5 run-console-idle (first Tab reaches "Minimize window", then "Close window"; no `tabindex > 0`), green locally at 12 passing / 2 skipped, expected-skip SET unchanged.
- The §11 carve-out is narrowed to the HOLD-DEPENDENT half (trap entry/containment, restoration), which needs a live Pulse and stays barred from CI.
- Retired, each quoted in the body as retired: the routine arm "carries ZERO `browser.keys` calls"; the Operable half "lives entirely in `operator-hold.e2e.ts`"; the condition "until a hold-free keyboard path exists on the routine arm", now met.
- The selection rationale retires: the sequential-focus start-point defect it called "open and its holder unmeasured" measured ABSENT (`initialFocus` BODY, first Tab → "Minimize window", a full cycle over 5 focusables).
- The routine pair's CI PROOF is still OWED — the `a11y` job is red at WebView2 session creation; owner the *Hosted-runner WebView2 session* route entry.
**Why:** the master itself scoped the work ("until a hold-free keyboard path exists") and named the blocking defect; this chunk discharged both with measurement, so the narrowing applied whole. The chunk report's claim that "no master states" the sequential-focus premise was wrong — the carve-out sentence carried it — and was corrected.
**Ref:** .andromeda/runs/2026-09-07T21-30-50-wrap/



## 2026-09-08-hosted-runner-webview2-session — the a11y job's first push-triggered run is no longer pending
**Section:** §1 A11y Scope Summary (CI integration) · §11 Anti-Patterns (CI) · §3/§9 harness roster line
**Change:** three sites said the `a11y` job's first push-triggered run was pending; all now record that it ran — run 34209940695, `event: push` on `build/conductor-0.2.0`, 2026-09-08, RED at WebView2 session creation — so the job is exercised but has never been green.
**Why:** a fact measured in-session that the chunk report did not carry, so the fan-out could not see it; raised by the orchestrator.
**Kept:** §1's driver-151 cross-major pairing attribution stays as written — its runs predate the driver refresh, so it is true; the proposal to change it was rejected on measurement.
**Ref:** .andromeda/runs/2026-09-08T14-20-00-wrap/

## 2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate — the three verbatim citations of arch §Stack's CI rationale
**Section:** §1 A11y Scope Summary (CI integration) · §9 CI Integration (Pipeline integration) · §9 CI Integration (the ROUTINE-checks CI-gate bullet)
**Change:** a11y-plan's three verbatim quotes of arch §Stack's CI/CD rationale cell move with arch's amendment to "Formatting, build + test gating, …"; the two enumerating sites also gain `cargo fmt --all --check` in their command list.
**Why:** a verbatim cross-master citation must track its source, and a11y's detectors do not cover a CI gate-set enumeration. The third site surfaced only through a known-positive control count — a two-site apply would have left it stale.
**Kept:**
- a11y-plan's runtime and driver literals — WebView2 151.0.4129.107 (§1), 152.0.4191.53 under a 151 msedgedriver (§1, a measured unsupported-but-working cross-major pair), msedgedriver at §3, the 152.0.4191.53 pass record at §6 — are DATED DEV-HOST measurements, not claims about the hosted image, so the hosted `windows-2025` falsification retires none of them.
- a11y-plan states no runtime × driver pair SET; test-plan alone carries the pair-set literals.
- §3's "CI is TARGETED at a Linux+`xvfb` runner" is a11y-plan's own statement, not an arch citation (arch carries no `xvfb` / `ubuntu-latest`), so it does not ride this pass; it reads as target-state the same sentence self-corrects with the shipped `windows-2025` job.
**Ref:** .andromeda/runs/2026-09-09T13-20-08-wrap/

## 2026-09-17 — 2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm
**Section:** §1 CI integration · §9 Platform · §9 E2E artifacts row · §11 Strategy carve-out
**Change:**
- The hosted-runner endpoint verdict is retired as UNCONDITIONAL and re-stated as configuration-bound. At CI run 35192876641 — hosted `windows-2022`, a coherent 131.0.2903.86 msedgedriver + WebView2-runtime pair, High integrity — `DevToolsActivePort` appeared, a WebDriver session was created, and the routine arm ran 11 passing / 1 failing / 2 skipped, SC 2.4.3 passing and the single red a counting-basis defect in the assertion. The endpoint still does not open on `windows-2025` at runtime 152/153. Corroborated by actions/runner-images#14738 for a plain Tauri/wry app with no token work.
- Retired wordings: "opens no remote-debugging endpoint", "RED at WebView2 session creation", "never been green", "Still never a green run", "endpoint remains CLOSED", "runnability is measured-unproven".
- Integrity's SIGN is configuration-bound: Medium helped at runtime 153 on the dev host; High is REQUIRED at 131 on `windows-2022`.
- The medium-integrity launcher is MEASURED-INSUFFICIENT: it lowers the label as designed (`S-1-16-12288` → `S-1-16-8192`) and did not open the endpoint.
**Why:** the unconditional verdict was falsified by a green configuration. The 2026-09-16 legs A/B/C are BOUNDED by this, never retired — they were correctly measured on what they measured.
**Kept:** the shipped arrangement is unchanged and no arrangement row moved: four escalations reached no operator ruling, so the probe-scoped `ci.yml` surfaces (the `windows-2022` label, the `≥152` floor bypass, the `msedgedriver.microsoft.com` egress, the launcher's removal from the asserting step) were removed rather than ratified.
**Ref:** NOT DERIVED

## 2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration — the routine pair's CI proof DISCHARGED, stated with its configuration
**Section:** §1 A11y Scope Summary (CI integration bullet) · §3 A11y Assertion Harness Contract (Configuration · Pipeline integration · Bootstrap phases, a11y-ci-gate-wire) · §9 CI Integration (Platform · E2E artifacts row · Pipeline integration) · §11 A11y Anti-Patterns → Strategy (the 2026-09-07 CARVE-OUT entry)
**Change:**
- The carve-out's debt "The CI PROOF of the routine pair is still owed" is DISCHARGED: the hold-free Operable pair (SC 2.1.1 reachability, SC 2.4.3 focus order) is green in CI on hosted `windows-2022` at a coherent 131.0.2903.86 driver/runtime pair, High integrity, driver pinned from the image's own runtime — 12 passing / 0 failing / 2 skipped, run 35208593666.
- Retired: "CI runs 152 and is unaffected; the pair's CI proof remains owed" and "The job is exercised but has never been green".
- The 2026-09-16 dev-host regressions of this pair are re-recorded as ASSERTION defects (a walk waiting on a `BODY` sentinel this webview's focus cycle need not contain), not a platform property; the pair is environment-independent across two configurations differing in runtime major, driver major and coherence.
- Six shipped-arrangement literals move `windows-2025` → `windows-2022`; §9's Pipeline-integration launch path moves from `scripts/a11y-limited-token-launch.ps1` to the direct `scripts/a11y-token-witness.ps1` entry at native High integrity.
- §3's "The step is not Linux-bound" gains the measured platform SET: Linux+`xvfb`, the Windows WebView2 dev host, and the hosted Windows image at a coherent pair; only macOS lacks a WebDriver.
**Why:** every discharged claim is written WITH its configuration, never as a bare verdict (operator-directed): an over-general claim cost six days, and the next image bump can move the label, runtime, driver or their coherence — the record must show that rather than say "the a11y arm is green in CI".
**Kept:** the three premise-correction brackets binding the RED reading to `windows-2025` at runtime 152/153 stand as that configuration's fact.
**Ref:** .andromeda/runs/2026-09-17T10-34-20-wrap/

## 2026-09-17-keyboard-and-focus-order-coverage-ownership — §5 keyboard/focus claims given owners; §11 carve-out gains its substitute gate; §1 dittography repaired
**Section:** §5 Keyboard Navigation (focus order per layout · focus trap · focus restoration · per-surface shortcuts) · §11 Strategy (the CARVE-OUT) · §1 CI integration
**Change:**
- §5's unattributed bullets gain an owner or a recorded gap. Run-console-HOLD focus order, the focus trap and focus restoration name the operator-only DRIVEN arm (`npm run a11y:driven` → `operator-hold.e2e.ts`) as sole owner and point at §11 *Strategy* for what gates in its place. Run-console-live row navigation, idle-with-report row navigation and the first-class shortcuts state that **no suite asserts them today** — gaps owned by the route entry *Unasserted keyboard and focus-visible claims closed*.
- §5 focus restoration no longer says "(Radix AlertDialog default)": it states the explicit `onCloseAutoFocus` + `restoreFocusTo` contract, the invoker kept focusable via `aria-disabled`.
- §5 HOLD focus order and the focus trap include the operator-checklist `checkbox` rows in the Tab/Shift+Tab cycle; was a Proceed/Abort-only cycle.
- §11 *Strategy* gains the substitute-gate sentence `v3-03` requires: the hold-dependent half's owner named, CI's structural inability to run it stated with its reason, and the three things CI gates in its place — the expected-skip SET held at two under `CONDUCTOR_A11Y_STRICT`, the committed claim→owner enumeration, and its checker — each stated as NOT a substitute for the assertions.
- §1: the clause "the one webview-automation stack running on the measured platform SET — " stood twice in one line; it now occurs once.
**Why:** measured against the shipped suites, three §5 claims had no asserting suite, and §5 disagreed with §9 and §11, which already stated per-arm ownership. The restoration and checklist sites were survivors of readings the 2026-09-01 and 2026-08-22 amendments retired everywhere else. The dittography rode a route CARRY for want of a routine channel; that class now has one.
**Kept:** §5's one already-attributed focus-order bullet was not touched.
**Ref:** .andromeda/runs/2026-09-17T17-09-36-wrap/

## 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir — readiness before every axe analysis
**Section:** §9 CI Integration (Pipeline integration; the routine arm's CI line) · §11 A11y Anti-Patterns → CI
**Change:**
- §9 gains a bullet, "Readiness before every axe analysis":
  - axe opens every `analyze()` with a `document.readyState` probe raced against a hard 1 000 ms budget, and any miss prints `Page/Frame is not ready` (its catch-all).
  - `axeFindings()` waits until that probe answers in under 250 ms, then analyzes once.
  - The standing stall arm binds a 1.5 s stall to the probe's own read and was measured red with the wait bypassed and green restored.
  - A timer-scheduled stall cannot witness it.
- §9's CI line: the routine arm is green "0 failing, only the expected-skip SET skipped" at run 35208593666 and since (CI#36681853843) (was "12 passing / 0 failing / 2 skipped").
- §11 → CI: NEVER answer `Page/Frame is not ready` with a retry of `analyze()`, a sleep, a skip, `retries`, `continue-on-error` or a patched `FRAME_LOAD_TIMEOUT`.
**Why:**
- CI#36635281444 went red when the probe answered `true` 1 173 ms after it was sent, during the busy window after `#root` mounts; that was the classic-execute race, not BiDi.
- Measured on the dev host at a coherent 154.0.4258.37 pair, and in CI#36681853843 on `windows-2022` 20260920.314.1 at a coherent 131.0.2903.86 pair (13 passing / 2 skipped).
- The plan's timer form passed with the wait bypassed (the scheduling execute absorbed the stall; the next probe's round trip was 17.7 ms), so the witness moved onto the probe.
**Kept:** the 2026-09-01 BiDi narrative under §6 Motion tokens (true of that session), and the run-anchored "12 passing … as measured at run 35208593666" records.
**Ref:** .andromeda/runs/2026-09-30T07-22-03-wrap/
