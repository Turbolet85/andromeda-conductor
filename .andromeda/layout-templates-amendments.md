# Layout Templates — Amendments

_Append-only changelog of amendments to `layout-templates.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-23-5-command-agent-run-harness — conductor preflight verb + agent-run stage flags registered
**Section:** §Surface: cli — Primary screens (commands)
**Change:** added the `conductor preflight [--json]` verb (the readiness gate / `agent-run boot` entrypoint; exits 0 iff ready:true, else non-zero) to the cli verb list, and named the `agent-run.sh` 5-command set + the `run` `--unit`/`--integration`/`--e2e` stage flags.
**Why:** the chunk landed the `conductor preflight` verb (cli verb surface 3→4) + the `run` stage flags (report §Changes); §cli Primary screens listed only run/suite/report + a generic `agent-run.sh` pointer. The preflight verb is a genuinely-new this-chunk surface (the `report`-verb precedent registered at ch1); D-layout-surface routine. No cascade — layout-templates has no specialist-summary doc and the cli-verb addition does not touch the frontend.md rule (webview-scoped). First amendment to layout-templates.

## 2026-06-23-line-oriented-output-rendering — conductor coverage verb registered; report output + indicatif version corrected
**Section:** §Surface: cli — Primary screens (commands) · §Surface: cli tooling context
**Change:** added the `conductor coverage [--write]` verb (render the static 60-P-ID matrix as a `comfy-table`; `--write` regenerates `coverage-matrix.md` at repo root) to the cli verb list; refined the `conductor report` line to "colored `comfy-table` results view (reads the JSONL journal; the `<run_id>.md` artifact stays Markdown)" per the P4-D2 output switch; corrected the tooling-context `indicatif 0.18`→`0.17` to the resolved lock.
**Why:** the chunk landed the `conductor coverage` verb (cli verb surface 4→5, the P4-D1 user decision) + switched `report` stdout from raw Markdown to the colored results table (P4-D2); §cli Primary screens listed run/suite/report/preflight + agent-run.sh but not coverage, and the report line + indicatif version were stale. The coverage verb is a genuinely-new this-chunk surface (the preflight-verb precedent at ch2); D-layout-surface routine. No cascade — layout-templates has no specialist-summary doc and the cli-verb addition does not touch the frontend.md rule (webview-scoped).

## 2026-06-24-frameless-window-shell — titlebar height space-lg → space-xl
**Section:** §Wireframe (Run console idle) · §Component — Header (frameless titlebar)
**Change:** reconciled the titlebar height `space-lg` (20px) → `space-xl` (32px) in both the idle-console wireframe annotation and the Component-Header spec; `space-lg` cannot contain the `Heading`-tier (18px) phase line + the window-control glyphs.
**Why:** D-layout-surface (warning). The frameless titlebar + window controls ARE already documented in §Component-Header (so the surface invariant holds) — only the height drifted: the implementation shipped `space-xl` because 20px is too short for the 18px heading + 20px controls (report Deviations). Routine per the spec-illustration→sound-impl reconcile rule (the doc tracks the shipped value). No cascade — layout-templates has no specialist-summary doc; the height is webview-scoped but does not change the frontend.md rule (which binds tokens by name, not component heights).

## 2026-08-08-sut-capability-manifest — De-hardcoded cli coverage range labels
**Section:** §Surface: desktop-webview wireframes (coverage header strip) · §Surface: cli (`conductor suite` header, Primary screens)
**Change:** `COVERAGE P-001..P-060` / `CONDUCTOR suite P-001..P-060` range labels and the "all 60 rows" virtual-scroll note now name the manifest's accepted set.
**Why:** Layout labels must not name a span wider or narrower than the rendered row set, which is now manifest-sourced.

## 2026-08-09-current-sut-coverage-classification — Mode cell documented; last three literal-60 sites de-hardcoded
**Section:** §Component — Primary content block 1 (coverage matrix) · §Surface: cli — Primary screens (`conductor coverage`) · §Surface: cli — Wireframe (suite-run summary caption)
**Change:** Two parts. (1) **De-hardcode:** the "full 60-row wall (P-001..P-060)" virtual-scroll note now names the manifest's accepted set; the `conductor coverage` verb line drops "static 60-P-ID"; and the suite-run wireframe caption — which baked *two coupled* facts, `step 60/60 done` and a per-state tally summing to 60 — is restored to internal coherence (`77 Pass · 1 Calib · 1 Fail · 1 Manual · 1 Residual · 1 Blocked · step 82/82 (manifest set) done`), matching the form the same sweep established one screen earlier at the coverage header strip. (2) **New:** the coverage-matrix row anatomy gains the **Mode** cell — the definition-of-done classification as text in one of four values (`auto` / `drive+observe` / `static-only` / `not-Conductor's`), never color-only, tallied by the header strip and mirrored as text in the cli table.
**Why:** (1) The classification widened 60 → 82, and `:121` directly contradicted the already-corrected `:37`. The caption was folded in by explicit operator decision at the plan review, overriding the distiller's "illustrative sample may stay": a baked count in a sample is the same stale-derived-fact class as one in prose, and its tally and denominator had to move together or the sample would be left internally inconsistent — worse than the stale total. (2) The Mode cell shipped at 2026-06-27-coverage-matrix-view but was never described here; D-layout-surface fired on it and the operator chose to apply now rather than carry, accepting that the follow-on *Out-of-scope classification treatment* chunk will revisit this block to add the not-Conductor's visual treatment. The de-hardcode edits landed at /implement (see this chunk's report §Deviations 1); this entry is the history half that flow owed. Cascade: `.claude/docs/design-summary.md`.

## 2026-08-09-out-of-scope-classification-treatment — Mode-cell out-of-scope treatment, the cli coverage roll-up, and the results-vs-coverage table split
**Section:** §Component — Primary content block 1 (Mode cell) · §Surface: cli / Primary screens (`conductor coverage`) · §Component — Primary content block 1 (cli)
**Change:** (1) the Mode-cell entry gains the out-of-scope **treatment** — `--status-residual` ↔ ANSI 246 ↔ Markdown emphasis, adapted per surface, explicitly not `--status-fail`/`--count-blocked` (the latter shares its hex with `--text-muted`, so a plain dim would read as `Blocked`), label always rendered, never a seventh lamp. (2) The `conductor coverage` entry now documents its **4-column** shape and the **roll-up caption** it gained (in-scope denominator beside the full row count, manifest-derived, out-of-scope token in the recessive tint). (3) The cli block is retitled **results / SLO table** and states that the coverage matrix is a separate narrower table.
**Why:** (1) is the standing commitment recorded at the previous chunk's wrap — the Mode-cell entry was authored there by operator decision explicitly accepting that this chunk would return to add the treatment. (2) the CLI **gained** a roll-up this chunk (it previously printed none — detector D-layout-surface). (3) resolves the same 6-vs-4 column conflation as the design-system entry. Operator-resolved escalations (1, 3) + detector D-layout-surface (2).

## 2026-08-09-interpretation-correctness-posture — coverage roll-up caption gains the unbacked qualifier
**Section:** §Surface: cli → Primary screens (`conductor coverage [--write]`)
**Change:** the roll-up caption literal now reads `… 66 in scope (43 auto (11 unbacked) · 16 drive+observe · 7 static-only) …`; the parenthetical qualifies the auto term only, is never a fifth summand, is plain text with no new token/ANSI/colour, is omitted at zero, and reaches the webview via the coverage matrix's `unbacked` prop (sourced from the `unbacked_auto` command, never mirrored in TypeScript).
**Why:** the chunk shipped the qualifier on all three surfaces; the documented caption still showed the pre-qualifier shape.

## 2026-08-09-in-lane-sut-scenarios — coverage roll-up caption: 11 → 10 unbacked
**Section:** Surface: cli → Primary screens → `conductor coverage [--write]`
**Change:** the quoted roll-up caption literal now reads `82 capabilities · 66 in scope (43 auto (10 unbacked) · 16 drive+observe · 7 static-only) · 16 not-conductors`. Only the qualifier moved; the surrounding rules (qualifies the auto term only, never a fifth summand, manifest-derived never a literal, plain text with no token/ANSI/colour, omitted at zero, identical on all three surfaces via the `unbacked` prop) are unchanged.
**Why:** the chunk named `P-079` in `constellation-severity-live-wiring.toml`, so `conductor_core::UNBACKED_AUTO` shrank 11 → 10 and every surface now renders `(10 unbacked)` — verified live via `conductor coverage`. The doc's sample caption still showed the pre-chunk value. Detector-raised (D-layout-surface) off the report's `Counts / qualifiers this chunk moved` bullet.

## 2026-08-09-sut-load-envelope — cli run-level load-envelope caption
**Section:** §Surface: cli — Output structure (`conductor run`) · Component — verdict / report-state lines
**Change:** The `conductor run` wireframe now shows the run-level `[ENVIRONMENT-SUSPECT]` caption above the verdict lines (ANSI 246, no new entry; ASCII label always printed; omitted when in-envelope or exempt), and the verdict-line list now states that its six entries are the closed per-P-ID set while stdout additionally carries that run-level non-lamp qualifier.
**Why:** The chunk landed a new cli output element that no wireframe showed, and the six-label enumeration read as the complete stdout label set. Named the set + flagged the qualifier; the lamp set stays six and `ReportState` stays five.

## 2026-08-10-pulse-run-contract — coverage roll-up caption: 10 → 9 unbacked
**Section:** §Surface: cli → Primary screens (`conductor coverage [--write]`)
**Change:** the quoted roll-up caption literal now reads `82 capabilities · 66 in scope (43 auto (9 unbacked) · 16 drive+observe · 7 static-only) · 16 not-conductors`. Only the qualifier moved; the surrounding rules (qualifies the auto term only, never a fifth summand, manifest-derived never a literal, plain text with no token/ANSI/colour, omitted at zero, identical on all three surfaces via the `unbacked` prop) are unchanged.
**Why:** the chunk named `P-073` in `scenarios/pulse-run-contract.toml`, so `conductor_core::UNBACKED_AUTO` shrank 10 → 9 and every surface now renders `(9 unbacked)` — verified live via `conductor coverage`. The doc's sample caption still showed the pre-chunk value. Detector-raised (D-layout-derived-count) off the report's `Counts / qualifiers moved` bullet. Note: the two sibling docs the report also named turned out NOT to bake the literal — obs-plan §4 names the SET (`read from conductor_core::UNBACKED_AUTO`, "neither is ever a literal") and design-system does not mention it at all — so this doc was the only real hit, and the report's Expected list was over-broad rather than the detectors under-running.

## 2026-08-13-dispatcher-determinism-goldens — `[ENVIRONMENT-SUSPECT]` sample caption re-based
**Section:** §Surface: cli — Output structure `conductor run <scenario>` (the caption sample)
**Change:** The sample caption no longer bakes the whole-scenario literal (`runs 900s, over the proven-good envelope ceiling of 600s`); it now names the breaching EMITTING PHASE and which sustained term it left (storm window / sustained rate — the asserted pair), noting whole-scenario duration is recorded rather than asserted. Placement, ANSI 246 reuse, always-printed ASCII bracket, run-level-qualifier status and the omitted-when-in-envelope rule are unchanged.
**Why:** `LoadEnvelope::classify` now judges per emitting phase against the two sustained terms, so the old sample described a caption the code can no longer produce. Named the term SET rather than substituting a fresh duration literal, per the detector's own guidance that a replacement literal simply re-stales.

## 2026-08-16-fingerprint-storm-live-proof — coverage roll-up caption DE-LITERALIZED
**Section:** §Surface: cli — Primary screens (`conductor coverage [--write]`)
**Change:** the sample roll-up caption's baked unbacked count becomes the placeholder form —
`43 auto (N unbacked)` instead of `43 auto (9 unbacked)`. The surrounding rules are unchanged (qualifies the
auto term only, never a fifth summand, plain text, omitted at zero, same shape on all three surfaces).
**Why:** the chunk named `P-074` in `scenarios/fingerprint-storm.toml`, shrinking
`conductor_core::UNBACKED_AUTO` 9 → 8 — the THIRD time this same literal went stale (11 → 10 at
`in-lane-sut-scenarios`, 10 → 9 at `pulse-run-contract`), each prior fix substituting a fresh literal that
re-staled. The line's own prose already declares "every number manifest-derived (never a literal)", so the
sample contradicted the rule stated beside it. De-literalizing ends the recurrence; founder-approved at this
wrap, with a playbook rule appended for the class.

## 2026-08-18-error-baseline-spike-live-proof — P-009 sample tier <5s → <90s (five sites)
**Section:** desktop-webview wireframes (Run console HOLD · Run report terminal) · cli Output structure (`conductor run` · `conductor suite`) · cli Primary content block 2
**Change:** Every P-009 error-baseline-spike sample row's slo_tier cell updated `<5s` → `<90s` (five sites, one sweep). Set-naming prose lines untouched (correct as written).
**Why:** The scenario TOML re-declared its tier this chunk (whole-run latency exceeds every tier by construction; `<90s` is the closest honest bucket, the v2-11 precedent).

## 2026-08-18-restart-suppression-live-proof — restart-suppression sample tiers de-literalized (five sites)
**Section:** §Wireframe — Run console (HOLD) · §Wireframe — Run report (terminal) · §cli Output structure `conductor run` · §cli Output structure `conductor suite` · §cli Component — Primary content block 2
**Change:** All five restart-suppression sample rows drop their pinned tier literals (`<20s` in the HOLD wireframe; `<5s` in the other four) for the set-named `<slo_tier>` placeholder; the HOLD row and block-2 example now state the cell renders the scenario's TOML-declared value from the closed `<5s`/`<20s`/`<90s` set.
**Why:** The chunk re-declared `restart-suppression`'s `slo_tier` `<20s` → `<90s`; the five sites disagreed with each other AND with the shipped value (report §Counts/qualifiers moved) — the de-literalization ends the re-staling class per the derived-count rule (the P-009 sweep precedent, extended to placeholder form).

## 2026-08-19-connection-lifecycle-live-proof — idle-console sample tier cells de-literalized
**Section:** §Wireframe — Run console (idle), the three P-ID sample rows
**Change:** The `P-001`/`P-002`/`P-003` sample rows' tier cells read `<slo_tier>` instead of baked `<5s`/`<20s` literals (the row-61 precedent form).
**Why:** This chunk re-declared two family tiers (`P-003`'s scenario `<5s`→`<90s`, `P-002`'s `<5s`→`<20s`); the P-003-labeled sample baked the old `<5s`. De-literalize, never re-pin a fresh literal (the 2026-08-18 restart-suppression precedent; raised by the orchestrator under the plan's expected-amendments floor).
## 2026-08-21-severity-lifecycle-live-proof — sample-row P-ID labels corrected against the coverage classification

**Section:** §Wireframe (coverage list · run-report card) · §cli Output structure · §cli Component — Primary
content block 2 · the per-P-ID prose lines
**Change:** every mislabeled sample row's P-ID now matches the scenario named beside it —
`span-status-error` P-001 → P-005, `baseline-error-rate` P-002 → P-009, `fingerprint-identity` P-003 → P-017,
`restart-suppression` P-014 → P-015 (5 sites), `port-occupier` P-022 → P-003 (4 sites). P-009 / P-032 / P-035
were already correct and are untouched.
**Why:** the pairings contradicted the authoritative coverage classification (`conductor-core::coverage_matrix`)
and the committed scenario catalog — P-022 is Auto-Resolution and Lifecycle (this chunk's own subject) while
the port-occupier is P-003's `receiver-failed-port-conflict`, and P-014 is Service Went Silent while
`restart-suppression.toml` names P-015/P-016/P-057. Raised at wrap Validate check 5 (the chunk plan queued the
P-022 site; verification found the mismatch systemic) and resolved with the operator, who chose the full sweep
over the plan's literal one-site scope. Labels, not layout: no wireframe, state, token or lamp changed.
