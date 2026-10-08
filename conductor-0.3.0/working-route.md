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
[2026-09-30-the-sr-cause-isolated-on-this-host] The SR cause isolated on this host — why NVDA hears Conductor's webview focus only in the window's first burst on WebView2 154.0.4258.37 / Windows 26200.9457, isolated by controls run cheapest-first, each with its boundary named
   ↓
[2026-09-30-the-sr-pass-regrades-on-the-os-input-path] The SR pass regrades on the OS input path — the confound (injected keys vs launch under tauri-driver + msedgedriver) separated first, then S0-09, E0-05, E0-09 and every silent focus row regraded on the path that step validates, the configuration named in every verdict
   ↓
[2026-09-30-the-screen-reader-content-findings-fixed] The screen-reader content findings fixed — every SR row conveys its required content, and the next regrade grades all 51 rows
   ↓
[2026-09-30-full-gate-regression-over-the-moved-surfaces] Full-gate regression over the moved surfaces — scenario corpus, the `a11y` job and the live-drive path, every gate green under both runners
   ↓
[2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix] Interpretation re-proven after Pulse's incident-surfacing fix — a third pre-registered real-model series for `v3-09` on a fresh letters-only data dir
   ↓
[2026-10-01-per-run-span-identity-in-the-real-model-harness] Per-run span identity in the real-model harness — two same-seed drives inside one Pulse buffer window both land, held by a test
   ↓
[2026-10-02-p-075-assert-round-against-pulse] The P-075 assert round against Pulse — one deterministic round against Pulse's returned build, every delegated budget graded hard
   ↓
[2026-10-02-captured-fingerprint-values-elided] Captured fingerprint values elided — no committed capture carries a `fingerprint_hex` value or prefix, the d3 residual fixed rather than ratified
   ↓
[2026-10-03-p-075-re-round-on-incident-events] The P-075 re-round on incident events — the round re-driven against Pulse with its incident lifecycle events read back through MCP and graded
   ↓
[2026-10-04-host-portable-tauri-ipc-tests] Host-portable Tauri IPC tests — the six `conductor-tauri` mock-runtime tests pass on the Linux dev host as they do on Windows

### Epoch 5b — Version close
[2026-10-04-real-model-test-surface-corrective] Real-model test-surface corrective — `real_model_harvest.rs` split by series, its repeated grading in `tests/real_model_common`, and `secret_scan_gate` skipping cleanly where no `.git` exists
   ↓
[2026-10-04-second-test-surface-corrective] Second test-surface corrective — `delegated_timing_harvest.rs` split under the size line, and the Epoch 5 audit's new emit/run mutation survivors killed
   ↓
[2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09] A fourth pre-registered real-model series for `v3-09` — interpretation re-proven once Pulse's retry-storm interpretation names its retry cause
   ↓
[2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09] A fifth pre-registered real-model series for `v3-09` — the same three-drive design and rule, against the Pulse sha that ships the fix
   ↓
[2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive] A capture run records the prompt the model received in each drive — the fifth series' three-drive design, read by Pulse's builder; not a `v3-09` verdict
   ↓
[2026-10-07-a-sixth-pre-registered-real-model-series-for-v3-09] A sixth pre-registered real-model series for `v3-09` — the same three-drive design and rule, against the Pulse sha that ships the measured remedy
   ↓
Version close on measured evidence — every capability verified or deferred on a measured basis with a named owner  CARRY: two product-script comments still cite the retired Windows host leaf by file name — `scripts/agent-run.ps1:230` ("waits on the child alone (host-win32.md)") and `scripts/a11y-token-witness.ps1:160` ("`HKLM\` does not (host-win32.md)"). That leaf was removed at setup commit `717bbf3` (2026-10-07; the host leaf is now `.claude/rules/host-linux.md`). The lesson each comment cites still stands elsewhere: the first in `.claude/rules/verification-harness.md` (its 2026-09-16 surviving-grandchild entry), the second in `.claude/docs/session-learnings.md` ("host leaf entry of 2026-09-11", the drive-letter-anchor entry). Both files were left unedited at the 2026-10-07 0-pending correction wrap as product files (word: "scripts/agent-run.ps1:230 and scripts/a11y-token-witness.ps1:160 are product files: leave them and put a CARRY on the version-close entry" — this wrap's invocation arguments, 2026-10-07); re-point or drop the two citations here  CARRY: the real-model capture's `canary:` classifier pairs only Pulse lines stamped before the scenario's emission instant (`crates/conductor-run/tests/real_model_live.rs`, the canary block), so a third canary storm whose digest tick lands a few ms before that instant prints `pipeline-fault` with `parse=none` whatever Pulse did with it — measured at the sixth series' d1 and d2 (ticks 4 ms and 2 ms before the instant, prompt assemblies 1-2 ms after it; Pulse's own log shows each storm parsed `ok` and deduped; `conductor-0.3.0/chunks/2026-10-07-a-sixth-pre-registered-real-model-series-for-v3-09/evidence/attempt-ledger.md`). No grade read the token and no re-fire was owed. Whether the pairing window changes is the founder's call at this entry's take-up; no entry was minted for it and it is not a residual for 0.4.0 (word: "Mint no entry; the pairing-window finding is a CARRY on the version-close entry" — the operator, 2026-10-08, the sixth series' wrap arguments)  CARRY: one rank-1 model statement stands outside `evidence/` in a frozen chunk's report — `conductor-0.3.0/chunks/2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive/report.md` (found by a fixed-string search at the sixth series' wrap, 2026-10-08; the same statement on this route's frozen sixth-series line was replaced by a pointer at that wrap, on the operator's ruling of 2026-10-07). The report was left untouched, its chunk being frozen; the founder rules on it (word: "Do NOT touch the frozen capture-run report.md here: pin a CARRY on the version-close entry saying one rank-1 model statement stands in that report outside evidence/ and the founder rules on it; no quote in the CARRY." — the operator, 2026-10-08)  CARRY: architecture §Occupied Resources stands at its byte threshold — 38114 of 38115 B, and §Established Decisions at 38082 B, `scripts/arch-registry-check.py measure` at the sixth series' wrap (2026-10-08) — so an amendment that grows either first frees bytes there (origin `2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09`)
