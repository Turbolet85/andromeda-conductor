# Layout Templates — Amendments

_Append-only changelog of amendments to `layout-templates.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-23-5-command-agent-run-harness — conductor preflight verb + agent-run stage flags registered
**Section:** §Surface: cli — Primary screens (commands)
**Change:** added the `conductor preflight [--json]` verb (the readiness gate / `agent-run boot` entrypoint; exits 0 iff ready:true, else non-zero) to the cli verb list, and named the `agent-run.sh` 5-command set + the `run` `--unit`/`--integration`/`--e2e` stage flags.
**Why:** the chunk landed the `conductor preflight` verb (cli verb surface 3→4) and the `run` stage flags, while §cli Primary screens listed only run/suite/report + a generic `agent-run.sh` pointer. The verb is a genuinely-new surface (the `report`-verb precedent). No cascade: layout-templates has no specialist-summary doc, and the cli-verb addition does not touch the webview-scoped frontend.md rule. First amendment to layout-templates.
**Ref:** .andromeda/runs/2026-06-23T19-20-31-wrap/

## 2026-06-23-line-oriented-output-rendering — conductor coverage verb registered; report output + indicatif version corrected
**Section:** §Surface: cli — Primary screens (commands) · §Surface: cli tooling context
**Change:** added the `conductor coverage [--write]` verb (render the static 60-P-ID matrix as a `comfy-table`; `--write` regenerates `coverage-matrix.md` at repo root) to the cli verb list; refined the `conductor report` line to "colored `comfy-table` results view (reads the JSONL journal; the `<run_id>.md` artifact stays Markdown)"; corrected the tooling-context `indicatif 0.18` → `0.17` to the resolved lock.
**Why:** the chunk landed the `conductor coverage` verb (cli verb surface 4→5, a user decision) and switched `report` stdout from raw Markdown to the colored results table; the verb list lacked coverage, and the report line + indicatif version were stale. The coverage verb is a genuinely-new surface (the preflight-verb precedent). No cascade: layout-templates has no specialist-summary doc, and the cli-verb addition does not touch the webview-scoped frontend.md rule.
**Ref:** .andromeda/runs/2026-06-23T20-40-17Z-wrap/

## 2026-06-24-frameless-window-shell — titlebar height space-lg → space-xl
**Section:** §Wireframe (Run console idle) · §Component — Header (frameless titlebar)
**Change:** the titlebar height was `space-lg` (20px); now `space-xl` (32px), in both the idle-console wireframe annotation and the Component-Header spec — `space-lg` cannot contain the `Heading`-tier (18px) phase line + the window-control glyphs.
**Why:** the frameless titlebar + window controls were already documented in §Component-Header; only the height drifted, because the implementation shipped `space-xl` (20px is too short for the 18px heading + 20px controls). The doc tracks the shipped value (spec-illustration → sound-impl reconcile rule). No cascade: layout-templates has no specialist-summary doc, and frontend.md binds tokens by name, not component heights.
**Ref:** NOT DERIVED

## 2026-08-08-sut-capability-manifest — De-hardcoded cli coverage range labels
**Section:** §Surface: desktop-webview wireframes (coverage header strip) · §Surface: cli (`conductor suite` header, Primary screens)
**Change:** the `COVERAGE P-001..P-060` / `CONDUCTOR suite P-001..P-060` range labels and the "all 60 rows" virtual-scroll note now name the manifest's accepted set.
**Why:** layout labels must not name a span wider or narrower than the rendered row set, which is now manifest-sourced.
**Ref:** .andromeda/runs/2026-08-08T16-05-00-wrap/

## 2026-08-09-current-sut-coverage-classification — Mode cell documented; last three literal-60 sites de-hardcoded
**Section:** §Component — Primary content block 1 (coverage matrix) · §Surface: cli — Primary screens (`conductor coverage`) · §Surface: cli — Wireframe (suite-run summary caption)
**Change:**
- De-hardcode: the "full 60-row wall (P-001..P-060)" virtual-scroll note now names the manifest's accepted set; the `conductor coverage` verb line drops "static 60-P-ID"; the suite-run wireframe caption, which baked two coupled facts (`step 60/60 done` and a per-state tally summing to 60), now reads `77 Pass · 1 Calib · 1 Fail · 1 Manual · 1 Residual · 1 Blocked · step 82/82 (manifest set) done`, the form already used at the coverage header strip.
- New: the coverage-matrix row anatomy gains the Mode cell — the definition-of-done classification as text in one of four values (`auto` / `drive+observe` / `static-only` / `not-Conductor's`), never color-only, tallied by the header strip and mirrored as text in the cli table.
**Why:** the classification widened 60 → 82, and one site directly contradicted an already-corrected one. The operator ruled that a baked count in an illustrative sample is the same stale-derived-fact class as one in prose, and that a sample's tally and denominator must move together — leaving it internally inconsistent is worse than the stale total. The Mode cell shipped at 2026-06-27-coverage-matrix-view but was never described here; the operator chose to apply now, accepting that the follow-on *Out-of-scope classification treatment* chunk revisits this block to add the not-Conductor's visual treatment. Cascaded to `.claude/docs/design-summary.md`.
**Ref:** .andromeda/runs/2026-08-09T14-17-38-wrap/

## 2026-08-09-out-of-scope-classification-treatment — Mode-cell out-of-scope treatment, the cli coverage roll-up, and the results-vs-coverage table split
**Section:** §Component — Primary content block 1 (Mode cell) · §Surface: cli / Primary screens (`conductor coverage`) · §Component — Primary content block 1 (cli)
**Change:**
- The Mode-cell entry gains the out-of-scope treatment — `--status-residual` ↔ ANSI 246 ↔ Markdown emphasis, adapted per surface, explicitly not `--status-fail`/`--count-blocked` (the latter shares its hex with `--text-muted`, so a plain dim would read as `Blocked`), label always rendered, never a seventh lamp.
- The `conductor coverage` entry documents its 4-column shape and its new roll-up caption (in-scope denominator beside the full row count, manifest-derived, out-of-scope token in the recessive tint).
- The cli block is retitled results / SLO table and states that the coverage matrix is a separate narrower table.
**Why:** the treatment discharges the commitment recorded when the Mode-cell entry was authored at the previous chunk's wrap. The CLI gained a roll-up this chunk (it printed none before). The retitle resolves the same 6-vs-4 column conflation as the design-system entry. The treatment and the retitle were operator-resolved.
**Ref:** .andromeda/runs/2026-08-09T15-56-31-wrap/

## 2026-08-09-interpretation-correctness-posture — coverage roll-up caption gains the unbacked qualifier
**Section:** §Surface: cli → Primary screens (`conductor coverage [--write]`)
**Change:** the roll-up caption literal now reads `… 66 in scope (43 auto (11 unbacked) · 16 drive+observe · 7 static-only) …`; the parenthetical qualifies the auto term only, is never a fifth summand, is plain text with no new token/ANSI/colour, is omitted at zero, and reaches the webview via the coverage matrix's `unbacked` prop (sourced from the `unbacked_auto` command, never mirrored in TypeScript).
**Why:** the chunk shipped the qualifier on all three surfaces; the documented caption still showed the pre-qualifier shape.
**Ref:** .andromeda/runs/2026-08-09T19-30-00-wrap/

## 2026-08-09-in-lane-sut-scenarios — coverage roll-up caption: 11 → 10 unbacked
**Section:** Surface: cli → Primary screens → `conductor coverage [--write]`
**Change:** the quoted roll-up caption literal was `(11 unbacked)`; now `82 capabilities · 66 in scope (43 auto (10 unbacked) · 16 drive+observe · 7 static-only) · 16 not-conductors`. Only the qualifier moved; the surrounding rules (qualifies the auto term only, never a fifth summand, manifest-derived never a literal, plain text with no token/ANSI/colour, omitted at zero, identical on all three surfaces via the `unbacked` prop) are unchanged.
**Why:** the chunk named `P-079` in `constellation-severity-live-wiring.toml`, so `conductor_core::UNBACKED_AUTO` shrank 11 → 10 and every surface now renders `(10 unbacked)`; the doc's sample caption still showed the pre-chunk value.
**Ref:** .andromeda/runs/2026-08-09T20-30-00-wrap/

## 2026-08-09-sut-load-envelope — cli run-level load-envelope caption
**Section:** §Surface: cli — Output structure (`conductor run`) · Component — verdict / report-state lines
**Change:** the `conductor run` wireframe now shows the run-level `[ENVIRONMENT-SUSPECT]` caption above the verdict lines (ANSI 246, no new entry; ASCII label always printed; omitted when in-envelope or exempt), and the verdict-line list states that its six entries are the closed per-P-ID set while stdout additionally carries that run-level non-lamp qualifier.
**Why:** the chunk landed a new cli output element no wireframe showed, and the six-label enumeration read as the complete stdout label set. The lamp set stays six and `ReportState` stays five.
**Ref:** .andromeda/runs/2026-08-10T15-43-07-wrap/

## 2026-08-10-pulse-run-contract — coverage roll-up caption: 10 → 9 unbacked
**Section:** §Surface: cli → Primary screens (`conductor coverage [--write]`)
**Change:** the quoted roll-up caption literal was `(10 unbacked)`; now `82 capabilities · 66 in scope (43 auto (9 unbacked) · 16 drive+observe · 7 static-only) · 16 not-conductors`. Only the qualifier moved; the surrounding rules (qualifies the auto term only, never a fifth summand, manifest-derived never a literal, plain text with no token/ANSI/colour, omitted at zero, identical on all three surfaces via the `unbacked` prop) are unchanged.
**Why:** the chunk named `P-073` in `scenarios/pulse-run-contract.toml`, so `conductor_core::UNBACKED_AUTO` shrank 10 → 9 and every surface now renders `(9 unbacked)`; the doc's sample caption still showed the pre-chunk value.
**Kept:** obs-plan §4 and design-system do not bake the literal — obs-plan names the SET (read from `conductor_core::UNBACKED_AUTO`, "neither is ever a literal") and design-system does not mention it — so this doc was the only site.
**Ref:** .andromeda/runs/2026-08-10T21-24-17-wrap/

## 2026-08-13-dispatcher-determinism-goldens — `[ENVIRONMENT-SUSPECT]` sample caption re-based
**Section:** §Surface: cli — Output structure `conductor run <scenario>` (the caption sample)
**Change:** the sample caption was the whole-scenario literal (`runs 900s, over the proven-good envelope ceiling of 600s`); it now names the breaching EMITTING PHASE and which sustained term it left (storm window / sustained rate — the asserted pair), noting whole-scenario duration is recorded rather than asserted. Placement, ANSI 246 reuse, the always-printed ASCII bracket, run-level-qualifier status and the omitted-when-in-envelope rule are unchanged.
**Why:** `LoadEnvelope::classify` now judges per emitting phase against the two sustained terms, so the old sample described a caption the code can no longer produce. The term SET is named rather than a fresh duration literal, because a replacement literal simply re-stales.
**Ref:** .andromeda/runs/2026-08-13T18-25-00-wrap/

## 2026-08-16-fingerprint-storm-live-proof — coverage roll-up caption DE-LITERALIZED
**Section:** §Surface: cli — Primary screens (`conductor coverage [--write]`)
**Change:** the sample roll-up caption's baked unbacked count was `43 auto (9 unbacked)`; now the placeholder form `43 auto (N unbacked)`. The surrounding rules are unchanged (qualifies the auto term only, never a fifth summand, plain text, omitted at zero, same shape on all three surfaces).
**Why:** the chunk named `P-074` in `scenarios/fingerprint-storm.toml`, shrinking `conductor_core::UNBACKED_AUTO` 9 → 8 — the third time this literal went stale (11 → 10 at `in-lane-sut-scenarios`, 10 → 9 at `pulse-run-contract`), each prior fix substituting a fresh literal that re-staled. The line's own prose declares "every number manifest-derived (never a literal)", so the sample contradicted the rule beside it. De-literalizing ends the recurrence; founder-approved, with a playbook rule appended for the class.
**Ref:** .andromeda/runs/2026-08-16T14-06-03-wrap/

## 2026-08-18-error-baseline-spike-live-proof — P-009 sample tier <5s → <90s (five sites)
**Section:** desktop-webview wireframes (Run console HOLD · Run report terminal) · cli Output structure (`conductor run` · `conductor suite`) · cli Primary content block 2
**Change:** every P-009 error-baseline-spike sample row's slo_tier cell was `<5s`; now `<90s` (five sites). Set-naming prose lines untouched (correct as written).
**Why:** the scenario TOML re-declared its tier this chunk — whole-run latency exceeds every tier by construction, and `<90s` is the closest honest bucket (the v2-11 precedent).
**Ref:** .andromeda/runs/2026-08-18T19-10-05-wrap/

## 2026-08-18-restart-suppression-live-proof — restart-suppression sample tiers de-literalized (five sites)
**Section:** §Wireframe — Run console (HOLD) · §Wireframe — Run report (terminal) · §cli Output structure `conductor run` · §cli Output structure `conductor suite` · §cli Component — Primary content block 2
**Change:** all five restart-suppression sample rows drop their pinned tier literals (`<20s` in the HOLD wireframe; `<5s` in the other four) for the set-named `<slo_tier>` placeholder; the HOLD row and block-2 example state the cell renders the scenario's TOML-declared value from the closed `<5s`/`<20s`/`<90s` set.
**Why:** the chunk re-declared `restart-suppression`'s `slo_tier` `<20s` → `<90s`; the five sites disagreed with each other and with the shipped value. De-literalization ends the re-staling class per the derived-count rule (the P-009 sweep precedent, extended to placeholder form).
**Ref:** .andromeda/runs/2026-08-18T21-55-43-wrap/

## 2026-08-19-connection-lifecycle-live-proof — idle-console sample tier cells de-literalized
**Section:** §Wireframe — Run console (idle), the three P-ID sample rows
**Change:** the `P-001`/`P-002`/`P-003` sample rows' tier cells were baked `<5s`/`<20s` literals; now `<slo_tier>` (the established placeholder form).
**Why:** this chunk re-declared two family tiers (`P-003`'s scenario `<5s` → `<90s`, `P-002`'s `<5s` → `<20s`), and the P-003-labeled sample baked the old `<5s`. De-literalize, never re-pin a fresh literal (the 2026-08-18 restart-suppression precedent).
**Ref:** .andromeda/runs/2026-08-19T23-10-30-wrap/
## 2026-08-21-severity-lifecycle-live-proof — sample-row P-ID labels corrected against the coverage classification
**Section:** §Wireframe (coverage list · run-report card) · §cli Output structure · §cli Component — Primary
content block 2 · the per-P-ID prose lines
**Change:** every mislabeled sample row's P-ID now matches the scenario named beside it — `span-status-error` P-001 → P-005, `baseline-error-rate` P-002 → P-009, `fingerprint-identity` P-003 → P-017, `restart-suppression` P-014 → P-015 (5 sites), `port-occupier` P-022 → P-003 (4 sites). Labels, not layout: no wireframe, state, token or lamp changed.
**Why:** the pairings contradicted the authoritative coverage classification (`conductor-core::coverage_matrix`) and the committed scenario catalog — P-022 is Auto-Resolution and Lifecycle while the port-occupier is P-003's `receiver-failed-port-conflict`, and P-014 is Service Went Silent while `restart-suppression.toml` names P-015/P-016/P-057. The mismatch was systemic; the operator chose the full sweep over the plan's one-site scope.
**Kept:** P-009 / P-032 / P-035 were already correct and are untouched.
**Ref:** .andromeda/runs/2026-08-21T09-50-00-wrap/

## 2026-08-21-per-check-latency-measurement — Per-check detail region in the Markdown run report
**Section:** Surface: cli -> Component — Primary content block 2
**Change:** documented the `runs/<run_id>.md` per-check detail region: one indented line per expected check under its P-ID verdict line, carrying that check's kind, verdict, latency and the deadline it was judged against — explicitly no new column, no new bracket label (the lamp set stays closed at six), no new ANSI/token entry, with stdout and the 6-column results table unchanged.
**Why:** the chunk added a rendered region layout-templates owns; the doc templated the run report as one line per P-ID and named `<run_id>.md` only as a destination.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-22-operator-pause-and-checklist-live-firing — operator-checklist rows inside the HOLD dialog
**Section:** Surface: desktop-webview → Wireframe Run console (HOLD) · Component — Hero / signature section (operator-pause dialog) · Component — Operator-checklist
**Change:** the HOLD wireframe and the dialog's part list now carry a checklist-rows region between Body and Actions, present only when the hold point declares `[[checklist]]` items and rendered as a SIBLING of the dialog's `Description` (never nested inside it). The Operator-checklist component was framed as "distinct from the operator-pause go/no-go dialog"; now the same primitive renders in two contexts, and the distinction is context (items shown at the hold vs a past observation in the report), not component.
**Why:** the chunk projects declared items into `OperatorPauseDialog`; the doc enumerated the dialog as exactly Header/Body/Actions and asserted the checklist primitive was distinct from it, both of which the shipped surface retires.
**Ref:** .andromeda/runs/2026-08-22T12-15-00Z-wrap/

## 2026-09-01-webview-self-verify-windows-host — `--e2e` is the cross-surface stage, and `.ps1` ships
**Section:** Surface: cli — Primary screens (the `agent-run` harness bullet)
**Change:** the bullet named `.sh` alone; it now names `scripts/agent-run.{sh,ps1}` (both shells at identical semantics per test-plan §3) and qualifies the stage set: `--unit`/`--integration`/the bundled default remain same-engine cli legs, while `--e2e` is the CROSS-SURFACE stage that builds the release Tauri bundle and drives the desktop-webview console through the tauri-driver webview leg, gated on a Conductor-owned driver handle whose absence skips it at exit 0 with a host-path-free `error:`/`hint:` precondition.
**Why:** the doc described every stage as an in-engine cli leg, which the re-pointed `--e2e` retires, and named no `.ps1` though it ships and test-plan §3 requires the two at parity. The amendment names the stage's TARGET and GUARD SHAPE rather than a command literal, so a future harness edit cannot re-stale it — the derived-count lesson applied at authoring time.
**Ref:** .andromeda/runs/2026-09-01T18-49-38Z-wrap/

## 2026-09-01-desktop-a11y-sweep — Mode-cell reason re-grounded; operator-pause dialog fade + restoration corrected
**Section:** §Component — Primary content block 1 (Mode cell) · §Wireframe — Run console (HOLD) · §Component — Hero / signature section · §Decisions Log (motion)
**Change:**
- The out-of-scope Mode cell keeps its recessive `--status-residual` ruling, but its justification no longer rests on `--count-blocked` sharing a hex with `--text-muted`; it rests on the lamp-token SET (reusing any lamp token would read as a verdict).
- The operator-pause dialog's `200ms fade entrance` → `motion-micro` at three sites, matching design-system §Component Patterns 2.
- §Component — Hero records that Radix supplies the trap + Escape but NOT focus restoration for this Channel-opened dialog.
**Why:** the chunk moved dark `--text-muted` off `#565F89` for contrast, ending the hex identity the reason cited; the ruling never depended on it. design-system was amended 200ms → 150ms (`--motion-micro`) by `2026-06-26-component-primitives-library` and layout-templates never received that cascade (operator-ratified). Focus restoration was measured 2026-09-01: with no Radix `Trigger`, focus landed on `<body>` on close.
**Ref:** .andromeda/runs/2026-09-01T22-22-12Z-wrap/

## 2026-09-01-live-per-p-id-verdict-lamps — desktop load-envelope banner + coverage row anatomy corrected to shipped
**Section:** desktop-webview Component Primary content block 1 + block 2 + Wireframe Run report (terminal) + Primary screens + cli Component Primary content block 2
**Change:**
- Documented the run-level load-envelope banner as a region of the run-report card — above the verdict lines, outside the lamp column, recessive `status-residual` with an always-rendered label, omitted entirely when in-envelope, neither a seventh lamp nor a sixth `ReportState` — and carried it into the wireframe and the Primary-screens content enumeration.
- The cli block-2 sentence names the run-level qualifier's three-surface SET (cli bracket label / Markdown banner / webview banner), adapted per surface, never forked.
- Coverage-matrix row anatomy was a six-cell lamp-first row; now what ships: four cells — P-ID, Capability, Mode, Status — with the lamp LAST, the not-yet-run absence state described, the worst-lamp-wins collision rule stated, and `slo_tier`/`latency_ms` recorded as deliberately not on this surface.
**Why:** the chunk added the banner as a genuinely new user-facing region — the webview leg of a qualifier the doc framed as cli-only. The row anatomy was a pre-existing divergence surfaced by the chunk being the first to populate the Status cell; the operator chose to correct the doc to shipped rather than carry the gap.
**Ref:** .andromeda/runs/2026-09-02T00-58-00Z-wrap/

## 2026-09-02-screen-reader-manual-spec — run states, titlebar labels, dialog copy and the unbuilt footer / report-site checklist corrected to shipped
**Section:** desktop-webview Primary screens + Wireframes (idle / HOLD / the run-report render, retitled "Idle with a report") + Component Header (phase line) + Hero (Header / Body) + Primary content block 1 (Empty) + block 2 (non-result states) + Operator-checklist + Footer + Decisions Log key layout choices
**Change:**
- The run-state set is named as shipped — `idle` · `live` · `hold` · `aborted`, idle-with-report a sub-state; an `aborted` screen recorded (no wireframe yet); the "Run report (terminal)" screen and wireframe re-labelled idle-with-report.
- The phase line renders one of four fixed `Conductor · {state}` labels, never the segment name — the Header component and the three wireframe titlebar rows corrected.
- The hold dialog's title / description are the backend hold prompt (`{p_id} — operator-checklist` / `Observe the operator-checklist claim for this scenario`) — HOLD wireframe + Hero corrected.
- Empty samples → shipped `No scenarios found.` / `No run yet`; the never-shipped "Run in progress" state removed (block 2 + Primary screens + idle wireframe).
- The footer status strip and the report-site checklist render are recorded DESIGNED, NOT SHIPPED (no `contentinfo`; the dialog site is the only mount) — the Footer section, the three wireframe footer rows, the run-report Manual row, block 2's ManualCheck clause and the Operator-checklist section; the roll-up re-attributed to the checklist view's own status line.
**Why:** every item was measured by the chunk's screen-reader pass against the release bundle (the landmark set is banner + main + two regions) and read off the shipped components. The design intents are kept as target state; the doc now also carries the shipped state.
**Ref:** .andromeda/runs/2026-09-02T11-47-51Z-wrap/

## 2026-09-03-conductor-tauri-survivors-dispositioned — `@theme` retired at both tooling-context sites (cascade)
**Section:** §Tooling context (`:11`) · §Surfaces covered (`:305`)
**Change:** both sites cited tokens declared via `@theme`; §Tooling context now reads "CSS Tailwind v4.1 (Oxide static stylesheet; design tokens declared on `:root`, never `@theme`)" and §Surfaces covered reads "React 19 / Tailwind v4.1 design tokens on `:root` / shadcn".
**Why:** cascade of the design-system amendment in the same pass — both sites cited design-system as saying tokens are declared via `@theme`, which design-system §Tokens retired on 2026-06-15 and the shipped `tokens.css` contradicts. Neither site fell inside any layout-templates detector's scope; the cross-master sweep the cascade mandates found them, and they were folded into this pass rather than deferred. These were the only two stale sites in this document.
**Ref:** .andromeda/runs/2026-09-03T09-10-06-wrap/

## 2026-09-03-live-pulse-preconditions-probed — the `preconditions` verb registered; verb literal de-literalized
**Section:** §Surface: cli — Primary screens · Component Primary navigation · Component Primary content block 2 · Component Header / banner · Tooling context · Decisions Log (cli)
**Change:**
- Registered `conductor preconditions [--json]` in §cli Primary screens with its three probe subjects, its exit rule (0 iff every subject satisfied — the documented `preflight` go/no-go exception), the `[PRECONDITION]` caption omitted when satisfied, and the fact that it mints no `Verdict`/`ReportState`/per-P-ID row and fires no canary.
- The `preflight` bullet no longer calls itself "the `agent-run boot` entrypoint" — boot's leading arm can short-circuit past it.
- The three-verb literal is de-literalized to the set `Commands` declares, at both sites (Primary navigation and the Decisions Log).
- Primary content block 2 states the non-lamp qualifier SET rather than "one".
- `anstream` retired from Tooling context and the Header/banner mechanism, replaced by the shipped per-stream `IsTerminal` gate.
**Why:** the verb lands in layout-templates rather than architecture, per the playbook. The de-literalizations apply as SET-NAMING under the playbook rule minted this wrap, since the earlier rule's two qualifiers both failed (the prose never declared the value derived, and no prior wrap had substituted a literal that re-staled).
**Ref:** .andromeda/runs/2026-09-03T19-20-00-wrap/
## 2026-09-04-sr-findings-remediation — empty-state split; the `boot`-reaches-preflight claim retired
**Section:** §Component — Primary content block 1 (coverage matrix) States · §Surface: cli Primary screens — `conductor preconditions` (primary) · `conductor preflight`
**Change:**
- The matrix Empty state attributed `No scenarios found.` to this component; now to `App.tsx` (the console's empty CATALOG), and the picker's distinct FILTER-MISS state with its `No scenarios match.` prose is recorded — two states, two strings.
- The `conductor preconditions` screen retires "handles read presence-only" + "exits 0 iff every subject is satisfied": the three handles share one boolean `declares()` gate, so a path-valued member makes exit 0 unreachable, and the `[PRECONDITION]` caption's "omitted when satisfied" arm is stated as intent, not a reachable state.
- The `conductor preflight` screen no longer says `agent-run boot` reaches it once every precondition is satisfied.
**Why:** the string split was measured by the screen-reader pass; the probe claim was disproved by measurement. Both routine (spec wording reconciled to the shipped implementation).
**Kept:** the doc's coverage-count literals (`82 loaded`, the mode tallies) were checked against the chunk's moved counts and are a different set, unmoved.
**Ref:** .andromeda/runs/2026-09-04T07-33-12Z-wrap/

## 2026-09-04-preconditions-probe-reads-path-handles-by-presence — both caption arms reachable
**Section:** §Surface: cli — Primary screens (`conductor preconditions [--json]` and `conductor preflight [--json]`) · §Surface: cli — Component — Primary content block 2 (the run-level non-lamp qualifier SET)
**Change:** retires the sr-findings-remediation entry's probe half. `conductor preconditions` exit 0 IS reachable — each handle is graded by its own KIND (the PATH-valued member by presence-after-trim, the flag-valued members by an affirmative `"true"`/`"1"`) — so both arms of the `[PRECONDITION]` caption are shipped, measured behaviour rather than intent: one ANSI-246 line per unmet subject naming its statement and candidate causes, and the single satisfied line when none is unmet. The `conductor preflight` screen no longer says `boot` cannot reach it; `boot` reaches it in both shipped shells, and a direct invocation is a valid entry point, not a forced workaround. The content-block-2 qualifier SET entry gains the same both-arms wording, so `[PRECONDITION]` is not read as failure-only. The four-key `--json` shape, the no-`Verdict`/no-`ReportState`/no-per-P-ID-row rule, the fires-no-canary rule and the closed six-member lamp set are unchanged — no new bracket label, ANSI entry or token.
**Why:** the satisfied line `[PRECONDITION] every live-Pulse precondition is satisfied` executed for the first time (it was dead code before this chunk), and both shells' `boot` emitted `ReadyState` JSON with no preflight skip, measured 2026-09-04. Routine: spec wording reconciled to the shipped implementation, and the chunk that operationalizes a described behaviour owns its current truth.
**Kept:** the `[ENVIRONMENT-SUSPECT]` half of the qualifier SET and the ANSI-246 reuse list were checked and are unmoved.
**Ref:** .andromeda/runs/2026-09-04T17-15-00-wrap/

## 2026-09-06-operator-gated-live-suite — the `--live` stage flag on the cli surface
**Section:** §Surface: cli — Primary screens (the `scripts/agent-run.{sh,ps1}` entry)
**Change:** the harness stage-flag set was three flags; now `--unit`/`--integration`/`--e2e`/`--live` (default = the full bundled gate). `--live` is registered as the operator-gated live-Pulse suite — a sibling stage flag, never a sixth command, reachable from no default or CI path — with its exit rule: the leading non-priming `conductor preconditions` probe REFUSES the suite at exit 1 on an unmet subject, printing one host-path-free `[PRECONDITION]` line per unsatisfied subject with its candidate causes and firing no leg. The two classes are fixed apart: an incomplete live-Pulse environment REFUSES, while an absent HOST-TOOL handle (`CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA`) still SKIPS its own leg at exit 0 with a fetch recipe.
**Why:** the chunk shipped the flag in both shells at identical semantics, and the doc's surface map enumerated only three stage flags, leaving the new stage undocumented. The refuse-vs-skip distinction is exactly the kind of exit rule this surface map exists to state, and was measured on both arms. Routine (spec wording reconciled to the shipped implementation).
**Kept:** the Tooling context carries no flag set, and the Decisions-Log bullet states the `conductor-cli` VERB set rather than harness stage flags, so neither restates the retired closed set.
**Ref:** .andromeda/runs/2026-09-06T09-37-04-wrap/

## 2026-09-06-run-report-envelope-conformance-gate — the cli verb enumeration gains `conductor cleanup`
**Section:** Surface: cli - Primary screens (commands)
**Change:** added a `conductor cleanup <run_id>` bullet after `conductor coverage`: the teardown verb removing one run's rows from all three `runs.db` tables in a single transaction, printing a plain confirmation line with the run id in the ID-cyan mapping. Idempotent; mints no bracket label, lamp, `ReportState`, token or ANSI entry; never prompts; the run_id binds as a SQL `?1` parameter and is never joined into a path.
**Why:** the enumeration listed six verbs and the shipped `Commands` set is now seven. This doc is the enumerating home by its own declaration and by architecture's explicit refusal of the role — the plan had aimed the amendment at architecture and the wrap re-aimed it. The CLI verb surface is layout-templates' concern (`preflight` and `coverage` set the precedent); under SET-NAMING the enumeration is not a re-staling literal, because the doc already names the source of truth.
**Kept:** the two lines that state the set by mechanism stay true unamended.
**Ref:** .andromeda/runs/2026-09-06T13-07-09-wrap/

## 2026-09-07-dependency-polish — cli Tooling-context library versions reconciled
**Section:** §Surface: cli — Tooling context
**Change:** `indicatif` 0.17 → 0.18 and `inquire` 0.7 → 0.9.
**Why:** the indicatif value is this chunk's own bump. The inquire value was a pre-existing stale literal on the same line, resolved at 0.9.4; architecture's roster row already said 0.9, so the three masters were split three ways on it.
**Kept:** every behavioural claim on this surface (the spinner STOPS in place, never hides, never animates to 100%) is untouched and re-asserted met.
**Ref:** .andromeda/runs/2026-09-07T12-43-31-wrap/

## 2026-09-07-a11y-ci-gate — the `--e2e` handle-absence outcome stated as an ARM SET
**Section:** §Surface: cli → Primary screens (the `scripts/agent-run.{sh,ps1}` bullet)
**Change:** both sentences on the bullet's line carried an unconditional skip verdict; both amended in one edit. The `--live` contrast now qualifies the host-tool half ("still SKIPS its own leg at exit 0") as the LAX arm, with an affirmative `CONDUCTOR_A11Y_STRICT` exiting non-zero instead — so the refuse-vs-skip contrast holds for the default arm and CI adds a third outcome rather than replacing either. The `--e2e` clause drops its unconditional "never a hard failure" the same way, and gains the printed-verdict assertion (the `Spec Files:` failed count, the per-spec skip markers against the expected set, the `[webview2 … windows]` banner), because the exit code alone cannot tell a full pass from a total skip.
**Why:** the two `process.exit(0)` sites were replaced by `exitUnresolvedHandle()` and both arms were measured. The two sentences sit ~400 chars apart on the SAME line, so a single-sentence apply would have left the retired verdict standing intra-line — the case the cascade rule names explicitly.
**Ref:** .andromeda/runs/2026-09-07T16-19-12Z-wrap/

## 2026-09-07-sr-findings-fixed — a third App.tsx-owned console state; shell parity conditioned
**Section:** §Surface: desktop-webview → Primary content block 1 (coverage matrix) → States · §Surface: cli — `scripts/agent-run.{sh,ps1}` bullet
**Change:**
- The two-states-two-strings enumeration gains a THIRD App.tsx-owned state — the catalog LOAD-ERROR render (`status-fail`, paired text, never tint alone). Its region was already mounted-empty-at-first-paint; what changed is the SITING — the visible copy sits OUTSIDE the announced `role="alert"` region, which carries only a visually-hidden re-assertion inserted once on the first `focusin`, so the message is announced exactly once (with both inside, NVDA read the region whole and spoke it twice). The three sibling error regions keep the empty-mount shape, because they fire at arbitrary times rather than before the screen reader binds.
- Cascade: the `agent-run.{sh,ps1}` bullet's verbatim citation of test-plan §3's "both shells ship at identical semantics" is conditioned to match the amended source — parity is a property of the scripts TOGETHER with the invoking environment.
**Why:** the region is pre-existing (2026-09-04); only the echo and the outside-placement are new, so the amendment states the change, not a creation. The parity edit is the cascade's verbatim-cross-citation edge: the sweep for the retired MECHANISM (an unconditional parity claim, not the token `identical semantics` alone) found this master citing test-plan for something it no longer says.
**Ref:** .andromeda/runs/2026-09-07T21-30-50-wrap/


## 2026-09-22-interpretation-proven-live — `preconditions --for` and the `--live real-model` selector
**Section:** Surface: cli › Primary screens (commands) — the `conductor preconditions` bullet (`:187`) and the `scripts/agent-run.{sh,ps1}` bullet (`:190`)
**Change:**
- The `preconditions` synopsis becomes `conductor preconditions [--for <SCENARIO>] [--json]`. The three handles are read for an affirmative declaration under the deterministic posture (the flagless default); `--for` takes a scenario NAME (a `P-NNN` value is refused at parse, exit 2, because a P-ID can name several scenarios) and grades the handles under that scenario's declared `l4_posture` — under `real-model` the L4 handle is read for ABSENCE, its unmet line reading `declared in this environment, but the real-model posture requires it absent or falsy`.
- The per-KIND grading sentence keeps its deterministic arm and gains the real-model one: the L4 flag is unmet when EITHER side's truthiness rule declares it (Conductor's `true`/`1` ∪ Pulse's `1`/`true`/`yes`), every other handle graded as before; exit 0 was measured reachable under that posture too (2026-09-23).
- The agent-run bullet gains a third-token selector: `--live real-model` runs the ONE operator-gated real-model leg instead of the suite, led by `preconditions --for real-model-interpretation` and refusing the same way; an unknown selector prints usage and exits 2 before any probe, in both shells — a selector of `--live`, neither a sixth command nor a new stage flag.
**Why:** the chunk shipped `--for`, its statement text and the selector, with the `--for` probe green under the live env and both shells refusing a bogus selector. Every applied sentence was re-derived from the chunk's own facts, not pasted from the detector's basis.
**Kept:** the doc's other per-P-ID mentions are P-ID-keyed rows, lamps and verdict lines, not a scenario cardinality, and were left; `docs/design-summary.md` and `.claude/rules/frontend.md` carry no cli command list.
**Ref:** .andromeda/runs/2026-09-23T08-03-55-wrap/

## 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed — the shipped key map and hint line; the coverage matrix's roving stop
**Section:** §Wireframe — Run console (idle) · §Component — Primary navigation · §Component — Primary content block 1 (anatomy · States)
**Change:**
- Primary navigation: keybindings are a MODIFIER map (was "exposed through the Radix primitives … mirroring the k9s / lazygit register" with a k9s-style hint frame): Ctrl+Enter start (and proceed in the hold dialog), Ctrl+. stop, Escape abort; one in-webview `window` listener that stands down during a hold; `aria-keyshortcuts` on each control; single letters, Ctrl+W, Alt+F4 unbound; ONE shipped hint line `Ctrl+Enter start · proceed   Ctrl+. stop   Esc abort` (Data, `text-tertiary`, not focusable, no live region).
- Idle wireframe: the hint line is drawn under the control row; Start/Stop annotated with their `aria-keyshortcuts`; the matrix tail reads "every manifest row rendered; one roving tab stop" (was "virtual-scroll").
- Primary content block 1: every row renders (was "Virtual-scroll for the full wall") inside a scroll container that is not a tab stop; ONE tab stop — the current row (ArrowUp/ArrowDown ±1 clamped, Home/End, focus by any means makes it current); States: the keyboard-current row = `aria-current` + a 2px `border-emphasis` edge on its P-ID cell (transparent reserve on every cell), distinct from the `color-focus` outline ring (was "selected row").
**Why:** the chunk shipped the map and the roving focus (P4 fork, overseer: modifier map, one tab stop per region; the k9s register amended at wrap).
**Kept:** the picker's `border-emphasis` active-item edge; the dialog's `motion-micro` fade entrance; the 58/59-column width mismatch between the titlebar and the body rows (pre-existing).
**Ref:** .andromeda/runs/2026-09-30T11-12-38-wrap/

## 2026-09-30-the-screen-reader-content-findings-fixed — the footer status strip shipped; the phase line is the h1
**Section:** §Component — Footer (status strip) · the footer rows of the Run console (idle), Run console (HOLD) and Idle-with-a-report wireframes · §Component — Operator-checklist · §Component — Header (frameless titlebar + Paused-count heartbeat)
**Change:**
- Footer: was "Designed, NOT shipped (as of 2026-09-02)", landmark set `banner` + `main` + two `region`s, "height `space-sm`", phase `idle` / `HOLD step 14` / `run complete`, each count token tinted; now SHIPPED 2026-09-30 (`footer.footer`, the `contentinfo` landmark; set `banner` + `main` + the `region`s + `contentinfo`), `--space-xs` block padding, ONE text line with plain-text ` · ` separators, not focusable, not live; `conductor` · `seed <n>` (`—` with no report) · the `RunState` word (`idle` · `live` · `HOLD` · `aborted`, no step index) · one `{n}` + `StatusLamp` per non-zero lamp in `LAMP_ORDER`, the glyph tinted and the label text; the HOLD accent an `aria-hidden` `count-hold` glyph, never a text tint. The unticked-ManualCheck count stays DESIGNED, NOT SHIPPED.
- Wireframes: the three footer rows marked shipped (HOLD row `● HOLD`, roll-up row with seed and run state first).
- Operator-checklist: "the window footer strip is unbuilt" → shipped without the unticked count.
- Header: the phase line renders as the window's single `h1` (was a `span`), drag region and `aria-live` switch kept.
**Why:** this chunk shipped the strip and the heading the masters named route-owned; an 8px `space-sm` box cannot hold a 12px Label line, and `count-hold` as 12px text on the light `color-base` computes ≈4.47:1 (under SC 1.4.3's 4.5:1); separate items with no whitespace read as "conductorseed —idle" to NVDA.
**Ref:** .andromeda/runs/2026-09-30T20-18-56-wrap/

## 2026-10-03-p-075-re-round-on-incident-events — the cli readiness sample's tool count names the pinned set
**Section:** §Surface: cli → Output structure (the `conductor run` and `conductor suite` readiness-line samples)
**Change:** Both samples were `tools 4/4`; they now read `tools <present>/<required>`, with `<required>` the pinned manifest's `required_tools` set (`contracts/mcp-contract.toml`), never a baked count — and never a substituted `5/5`.
**Why:** The pinned set grew from four to five tools this chunk, and a literal sample re-stales on every addition; the set is owned by the manifest.
**Ref:** .andromeda/runs/2026-10-03T23-44-32-wrap/
