# Working Route — conductor-0.3.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve edits only the markerless tail; once the chunk's master_
_record is complete, wrap P7 flip-compacts its line to `[{marker}] {title} — {scope hint}`, archiving the_
_verbatim line to route-archive.md); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation: the measurements the closures rest on
[2026-09-11-hosted-runner-endpoint-cause-probed] Hosted-runner endpoint cause probed — policy state, module versions, session identity; diagnose-only, the reading recorded whatever it says, host paths scrubbed
   ↓
[2026-09-11-hosted-runner-endpoint-cause-closed] Hosted-runner endpoint cause closed — the unread module-version probe placed where the app is alive, and the elevation difference varied
   ↓
[2026-09-12-ledger-gate-id-space-generalised] Ledger gate id-space generalised — `requirement_ids` filters `starts_with("v2-")` while the same file's directory resolution was deliberately generalised, so the gate goes vacuous-then-red at every version transition; it is red now on `conductor-0.3.0`'s `v3-` ids (906 run, 905 passed, 1 failed) and red in CI on `c93a379` and `eecc7f4`. Surface measured: seven `v2-` literals at `:219 :221 :228 :250 :256-259` plus the id-space assertion at `:208` in `crates/conductor-report/tests/matrix_ledger_gate.rs`
   ↓
[2026-09-13-audit-debt-retired-before-epoch-1-closes] Audit debt retired before Epoch 1 closes — seventeen surviving mutants killed or ratified by class, the emission-test fixture family shared, the envelope keys single-sourced
   ↓
[2026-09-13-p-025-measurement-contract-for-pulse] P-025 measurement contract for Pulse — which Pulse-emitted observable, at what resolution, over what window, and what constitutes a hard grade

### Epoch 2 — Scenario assertion hygiene
[2026-09-14-emit-scrubber-and-percentile-math-under-test] Emit scrubber and percentile math under test — conductor-emit's first-measured survivor population down to a stated floor, the timeouts classified
   ↓
[2026-09-15-structurally-dead-assertion-class-retired] Structurally-dead assertion class retired — three live CountAtLeast keys and one Hard Contains to declare-only, pinning tests paired in the same change
   ↓
[2026-09-15-remaining-structurally-dead-declarations-retired] Remaining structurally-dead declarations retired — the six blocks across five scenarios measured dead, each ground stated, pinning tests and guards moved in the same change
   ↓
[2026-09-15-scenario-tier-honesty] Scenario tier honesty — every declared tier fits its phase duration inside the closed tier set or states why, three situations kept distinct
   ↓
[2026-09-16-scenario-assertion-audit-gate] Scenario-assertion audit gate — one mechanical re-runnable check establishing both outcomes, named as a CI gate or an operator instrument

### Epoch 3 — The a11y capability's terminal
[2026-09-16-a11y-ci-gate-at-an-honest-terminal] A11y CI gate at an honest terminal — routine arm's asserted verdict green in CI, or a ratified exclusion naming its measured cause and owner
   ↓
[2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm] Medium-integrity launch for the a11y routine arm — the a11y job's leg runs at Medium integrity and the routine arm's asserted verdict is read from a CI run that reached its assertions, or the CI half is closed as a ratified exclusion resting on THIS measured cause
   ↓
[2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration] The a11y routine arm's terminal, on the measured configuration — `:384`'s counting basis corrected so SC 2.1.1 measures the reachability property rather than one environment's focus-cycle wrap, and the arm's CI verdict read green
   ↓
[2026-09-17-keyboard-and-focus-order-coverage-ownership] Keyboard and focus-order coverage ownership — the hold-dependent trap and restoration half's owner named, and its CI carve-out stated

### Epoch 4 — Live proof against a real Pulse
[2026-09-18-real-model-leg-posture-and-grading-rule] Real-model leg posture and grading rule — deterministic mode off, operator-gated, never a CI gate, grading and per-leg quiet window fixed beforehand
   ↓
[2026-09-22-interpretation-proven-live] Interpretation proven live — known root cause injected through the emission path, top hypothesis asserted to identify it, graded against the stated rule
   ↓
[2026-09-23-real-model-capture-path-handles-guarded-and-stale-read-back-texts-corrected] Real-model capture path handles guarded and stale read-back texts corrected — the three recorded path residuals and three stale texts closed, no live leg
   ↓
[2026-09-24-architecture-registries-compacted-under-the-read-cap] Architecture registries compacted under the read cap — Established Decisions and Occupied Resources each readable whole in one Read, no registered fact lost
   ↓
[2026-09-29-diagnostic-quality-cluster-off-the-drift-pin] Diagnostic-quality cluster off the drift pin — P-031, P-033, P-034 and P-044 on an exercised path, gate pin and committed matrix moving together
   ↓
[2026-09-29-dual-license-mit-or-apache-2-0] Dual license MIT OR Apache-2.0 — `LICENSE-MIT` and `LICENSE-APACHE` committed and every manifest's `license` field set, so the public repository carries a license
   ↓
[2026-09-29-hue-shift-budget-graded-hard] Hue-shift budget graded hard — `halo-hue-encoding` re-driven once Pulse emits the contracted observable, restoring the fourth delegated budget
   ↓
[2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir] Interpretation re-proven on a clean-named data dir — a new pre-registered real-model series for `v3-09` on a data dir whose name no scrubber pattern can match

### Epoch 5 — Polish & ship
[2026-09-24-secret-scanning-ci-gate] Secret-scanning CI gate — no secret-shaped string in the workspace, the key and cert ignores present, the build red on any hit
   ↓
[2026-09-30-mutation-gate-grades-every-tally-it-rests-on] Mutation gate grades every tally it rests on — `scripts/mutation-gate.py` reads `timeout.txt` beside `missed.txt` and `caught.txt`, and the timeout class is rostered rather than held only in a chunk's own disposition ledger
   ↓
[2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed] Unasserted keyboard and focus-visible claims closed — coverage-matrix row navigation, the first-class shortcuts and the SC 2.4.7 active-element ring given real assertions
   ↓
[2026-09-30-the-screen-reader-pass-grades-again-on-this-host] The screen-reader pass grades again on this host — the cause of NVDA hearing no webview focus event here removed, then S0-09, E0-05, E0-09 and every other silent focus row regraded
   ↓
The SR cause isolated on this host — why NVDA hears Conductor's webview focus only in the window's first burst on WebView2 154.0.4258.37 / Windows 26200.9457, isolated by controls run cheapest-first, each with its boundary named  PREREQ: close Rust gate deferral (deferred since 2026-09-30-the-screen-reader-pass-grades-again-on-this-host)  CARRY (2026-09-30-the-screen-reader-pass-grades-again-on-this-host; operator relay conductor-wrap-sr-2026-09-30 §2): Branch B measured — the object-model control (`[UIA] allowInChromium = 2`, witnessed applied) left E0-02..E0-06 silent as the default arm left R0-02..R0-04, so NVDA's object-model choice is ruled out; still standing: the runtime/driver pair (every focus row heard at 152.0.4191.66 on 2026-09-07 → silent at 154.0.4258.37), the cumulative updates KB5124008 / KB5129195, the desktop (`chunks/2026-09-30-the-screen-reader-pass-grades-again-on-this-host/evidence/cause-control.md`). Controls, cheapest-first: (1) the NO-BOUNDARY control first — hypothesis: the silence is host- or runtime-wide, not Conductor's; NVDA at defaults over a plain focus page in Edge 154 (the same Chromium) and/or another WebView2 app on the host — silent there ⇒ the cause sits below Conductor, heard there ⇒ it is Conductor's own webview posture; that reading decides whether (2) is needed at all. (2) the RUNTIME control — the leg pointed at the 153.0.4234.48 runtime still on disk (re-list it first; Evergreen prunes superseded dirs) through the loader's executable-folder variable, plus a matching 153 msedgedriver the operator supplies: a Boundary-widening crossing (playbook `:124`), awaiting the founder's word (asked by the overseer 2026-09-30; no answer at this wrap). A Windows-update rollback is NOT a control this entry may plan — irreversible host surgery, the founder's call alone. Every NVDA run is a quiet-desktop operator slot; the product lever (Tauri `additionalBrowserArgs` / the loader's browser-arguments variable) stays named and unbuilt.  CARRY (2026-09-30-the-screen-reader-pass-grades-again-on-this-host, the light gate's owner): this entry owns the three regrade reds that chunk's plan carried by construction under Branch B — the focus-row count over its committed `evidence/nvda-pass.json` (24, bar 0), S0-09 / E0-05 `announced-as-expected` (both `not-announced`; E0-09 stays `not-run-here`, a browse finding) and all three subjects recorded on one configuration (`false`) — to be re-graded once the cause is isolated and removed or routed, never by a rerun-until-announced.
   ↓
Full-gate regression over the moved surfaces — scenario corpus, the `a11y` job and the live-drive path, every gate green under both runners  CARRY: conductor-emit's public docs link FIVE private items — `exception.rs:70:45` (`MAX_FRAMES`), `:155:10` (`FINGERPRINT_BYTES`), `:158:13` (`NORMALIZED_FRAMES`), `:158:58` (`normalize_stacktrace`), `pii.rs:102:55` (`PiiCategory::index`) — so `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -p conductor-emit` baselines RED at exit 101, a state measured at `2026-09-13-audit-debt-retired-before-epoch-1-closes` and independent of that chunk. `conductor-run` already passes that gate form. Fix the five (make the items `pub`, or de-link to plain backticks) and promote conductor-emit's doc gate from the narrowed form that chunk shipped (`expect`: exit 0 · `lacks conductor_core` · `contains generated 5 warnings`) to the `-D warnings` form — at which point the 5-warning count stops being a pin (operator disposition, 2026-09-13 wrap)  CARRY: `npm run knip` in `crates/conductor-tauri/ui` EXITS 1 on twelve pre-existing unused exports, so any plan listing it with `expect = ['exit 0']` ships an unsatisfiable gate — measured at `2026-09-17-keyboard-and-focus-order-coverage-ownership`, whose own listing went red on it. Basis: a detached worktree at parent commit `3ddd405` with `node_modules` junctioned returns byte-identical findings (`conductor-0.3.0/chunks/2026-09-17-keyboard-and-focus-order-coverage-ownership/evidence/knip-at-HEAD-3ddd405.log`), so the red predates that chunk and is owned here. The twelve are exported-but-unimported symbols in `src/components/ScenarioPicker.tsx` and `CoverageMatrix.tsx` and in `test/a11y/screen-reader/parse-nvda-log.ts` and `rows.ts` — the class `rules/frontend.md`'s 2026-09-07 entry already records as residual, and which a11y-plan §11 forbids acting on against the harness, so DISPOSITION them (each exported symbol used, removed, or `ignoreExportsUsedInFile`-style excluded with its reason) rather than deleting blind; only then can the gate assert exit 0  CARRY: `conductor run <P-ID>` and the harness's `SCENARIO=<P-ID>` resolve to the FIRST scenario naming the P-ID in unsorted `read_dir` order (`conductor-cli` `find_by_pid`, read 2026-09-23), and seven P-IDs are named by several scenarios (P-017 · P-018 · P-019 · P-020 · P-021 · P-022 · P-060, census at the 2026-09-22-interpretation-proven-live wrap) — refuse an ambiguous P-ID as `preconditions --for` already does, or pick deterministically; test-plan §3 records the ambiguity meanwhile (operator-routed 2026-09-23).  CARRY: `.github/workflows/ci.yml`'s comment block above `A11y routine arm` (`:351-356` at `6008a68`) still gives `EDGEWEBDRIVER`'s shell read the reason "the ${{ env.* }} expression context holds only what a workflow, job or step declared" — measured false for a `GITHUB_ENV`-written key at CI run 36006370951 (`2026-09-24-secret-scanning-ci-gate`, where arch §Occupied Resources was corrected and playbook rule "a SHIPPED artifact READS" annotated). Reword the comment to the measured mechanism: the shell read is a choice, since a `GITHUB_ENV` key also resolves through `${{ env.* }}`; only an image-set runner variable does not. Pinned here by operator direction at that wrap (a doc-correctness fix of the class this entry already carries).
   ↓
Interpretation re-proven after Pulse's incident-surfacing fix — a third pre-registered real-model series for `v3-09` on a fresh letters-only data dir  BLOCKED-ON: Pulse's "Real-model incident surfacing" entry committed and pushed — clears when a Pulse commit carrying it is on Pulse's pushed branch (founder 2026-09-30, "fix in Pulse 0.3.0"; minted on relay `conductor-wrap-56-2026-09-30` §2, placement before Version close named there)  CONTEXT: the 2026-09-30 series graded 0 drives — d1 canary-blocked, and in d2 and d3 the real model dismissed the scenario's own storm digest (measured at `conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/evidence/attempt-ledger.md`); `v3-09` returns to the pool at that wrap and this entry owns it; same discipline as that series — a fixed drive count and rule in the posture contract before d1, digest-pinned captures, no retry-until-pass  CARRY (2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir): build BOTH `pulse-app` and the MCP sidecar from the fix HEAD and prove each by content (the sidecar renders the graded report through the same scrubber, measured at `fcc31b2` `crates/interpretation/src/markdown.rs`); confirm from Pulse's `app.boot.workspace_key` line that the key's leaf is the data dir's, since the capture's key mask derives the key from the data dir
   ↓
Version close on measured evidence — every capability verified or deferred on a measured basis with a named owner
