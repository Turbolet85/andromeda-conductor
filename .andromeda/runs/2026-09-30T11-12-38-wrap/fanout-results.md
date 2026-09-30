# Fan-out results — 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed

Seven Explore doc-agents, one parallel batch. Deviation from the verbatim form: `{detectors_yaml}` was passed as a
pointer to `detectors-{doc}.yaml` in this run dir (the split of `drift-base.md` by `doc:`), not inlined. No contracts
line (every master `NOT MIGRATED`, registry.py exit 3). Returns parsed directly (no HTML entities present; no raw twin
warranted — no `proposals: []` return was altered by stripping; each return's `#` commentary lines were stripped).

## Verdicts
- architecture — 2 proposals (A1, A2)
- security-plan — `proposals: []` (all four detectors no hit; stripped commentary: no new external input, spawn,
  dependency or platform verdict; rule (b) count stays seven)
- design-system — `proposals: []` (stripped commentary named the floor sites :27 :34 :261 :269 :257 :124 :160 as real
  drift outside its three detectors → raised by the orchestrator, check 5)
- layout-templates — 5 proposals (L1–L5)
- test-plan — 3 proposals (T1–T3)
- obs-plan — `proposals: []` (no new op, no log writer, no CI step, no verdict retired)
- a11y-plan — 17 proposals (Y1–Y17); stripped commentary: `:424`'s "focus-ring fade-in" sits in a "Formerly this read …"
  provenance sentence — history, skipped.

## Proposals and dispositions
| id | doc · section | change (short) | disposition · check |
|---|---|---|---|
| A1 | arch §Occupied Resources → Env vars → `CONDUCTOR_SCENARIOS_DIR` | `wdio.conf.ts` also sets it for the `driven` suite (`runs/driven/scenarios`, harness-owned) | apply · routine (playbook :308 Accurate this-chunk addition); compact — 87 B headroom |
| A2 | arch §Occupied Resources (size) | keep net growth ≤ 87 B; history to the sidecar | apply as constraint · D-arch-registry-size measured after apply |
| L1 | layouts §Component — Primary navigation | k9s / Radix-bound shortcuts → the shipped modifier map + `run-controls__hint` | apply · routine (:308) on recorded P4 direction (overseer fork: key map + "register amended at wrap") |
| L2 | layouts §Wireframe — Run console (idle) | control row shows the hint line + Start/Stop shortcut declarations | apply · routine (:308) · dependent-of L1 |
| L3 | layouts §Component — Primary content block 1 | virtual-scroll → every row renders, focus-scroll, one roving stop, Arrow/Home/End | apply · routine (:308) |
| L4 | layouts §Wireframe — Run console (idle) | matrix "(virtual-scroll …)" annotation restated | apply · routine · dependent-of L3 |
| L5 | layouts §Component — Primary content block 1 → States | "selected row" → the `aria-current` current row edge, distinct from the ring | apply · routine · dependent-of L3 |
| T1 | test-plan §6 desktop-webview row | driven arm = one live run over its seeded two-scenario catalog; shortcuts + live row-nav; decisions from the backend log | apply · routine (:308) |
| T2 | test-plan §2 Deterministic invariant | "~54s" driven sample → one run, two holds, measured `1 passing (4m 2.1s)` 2026-09-30 | apply · routine (:171 measured scalar, basis in the report's Outcome) · dependent-of T1 |
| T3 | test-plan §6 SR firing-form clause | "ONLY sr-empty and sr-error carry a scenarios field" → the driven suite also sets it | apply · routine · dependent-of T1 |
| Y1 | a11y §5 run-console-live | retire "No suite asserts"; roving mechanism; owner driven; routine row-nav spec in place; virtual-scroll → focus-scroll | apply · routine (:308) |
| Y2 | a11y §5 idle-with-report | owner routine; `aria-current` + `--border-emphasis`, not `aria-selected` | apply · routine · dep Y1 |
| Y3 | a11y §5 Per-surface keyboard shortcuts | shipped modifier map; owner driven; routine key-map spec in place | apply · routine on recorded P4 direction · dep Y1 |
| Y4 | a11y §4 → Coverage matrix | table semantics, `aria-current`, roving contract, no virtualization | apply · routine · dep Y1 |
| Y5 | a11y §1 Critical paths → View run report | `aria-selected` → `aria-current`; virtual-scrolled → focus-scroll | apply · routine · dep Y1 |
| Y6 | a11y §1 surfaces Notes | virtual-scroll note → every row renders | apply · routine · dep Y1 |
| Y7 | a11y §3 Keyboard test harness | coverage sequence + shortcut map restated | apply · routine · dep Y1 |
| Y8 | a11y §11 Screen Reader ban | virtual-scroll premise restated (ban kept for any future virtualization) | apply · routine · dep Y1 |
| Y9 | a11y §11 Strategy | "what gates in its place" extended to live row-nav + shortcuts; `inPlace` checker-enforced; SC 2.4.7 routine | apply · routine (:308) |
| Y10 | a11y §9 E2E artifacts row | routine + driven arm contents | apply · routine · dep Y1 |
| Y11 | a11y §9 Pipeline integration | routine / driven keyboard split | apply · routine · dep Y1 |
| Y12 | a11y §3 CI integration → Command | arm contents | apply · routine · dep Y1 |
| Y13 | a11y §1 harness spec → CI integration | routine / driven split | apply · routine · dep Y1 |
| Y14 | a11y §6 Motion tokens | drop "focus-ring fade-in" — the ring is an untransitioned outline | apply · routine (:308 mechanism reconcile; scope.md premise-correction) |
| Y15 | a11y §1 triggers → motion-sensitive | drop focus-ring fade-in | apply · routine · dep Y14 |
| Y16 | a11y §10 failure conditions | drop focus-ring fade-in | apply · routine · dep Y14 |
| Y17 | a11y §11 Visual ban | drop focus-ring fade-in | apply · routine · dep Y14 |
| O1 | design §Navigation Pattern (k9s register, :27 :34) + §Component Patterns #5 (:261, :269) | → the modifier map | raised by the orchestrator · check 5 (plan floor) · routine on recorded P4 direction |
| O2 | design §Component Patterns #3 (:257) | "Virtual-scroll for the full wall" → shipped truth | raised · check 5 · routine (:308) |
| O3 | design §Depth Strategy (:124) + §Motion (:160 and any ring `--motion-micro` site) | ring = `outline`, no fade | raised · check 5 · routine (:308) |

Check 2 (cross-contradiction): none — every proposal on a shared section moves the same direction.
Check 3 (intent): the SR regrade criterion is UNMET with a two-sided not-this-chunk basis and a P5 owner (operator relay
§2); the plan-atom edit is operator-directed (relay §1) — justified, no escalation.
Check 4 (absence): design-system's floor sites re-read at apply; a11y `:424` read and left (provenance sentence).
Check 5 (expected amendments): all nine plan entries covered — a11y §5 (Y1–Y3), §11 (Y9), §4 (Y4); design §Navigation
+ #5 (O1), #3 (O2), §Depth + §Motion (O3); layouts (L1–L5); test-plan §6 (T1, T3); arch (A1).
Check 6 (disproved claims): 1 Spec Files atom → plan.md entries 8/10/11 corrected on the relay's direction (control:
scratch plan, TAB form green, single-space red) + `verification-matrix.json#v3-03` acceptance → P7.3 refine +
`verification-harness.md:58` (Session Additions) → P3 in-place correction; 2 → Y1–Y3; 3 → Y1/Y4–Y8, L3/L4, O2; 4 →
Y14–Y17, O3; 5 → L1, Y3, O1; 6 → already recorded in `scope.md` (the SC 2.4.7 row now exists) — disposed.

Escalations: 0.
