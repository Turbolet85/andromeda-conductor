# Fan-out results — 2026-09-30-the-screen-reader-content-findings-fixed

Seven Explore doc-agents, one per spec source, prompts `prompt-{doc}.txt` (sent verbatim; `{key}`/`{slug}` are the
template's own literals). Returns carried no HTML entities (probe: `&lt;|&gt;|&amp;` 0 in every return). No raw twin:
every `proposals: []` return arrived as plain YAML with only `#` comment notes, which stripping removed.

## Verdicts
- architecture — `proposals: []` (notes: no new port/env/crate; 4317/4318 Pulse-owned, 4444/4445 harness-owned; no platform verdict retired)
- security-plan — `proposals: []` (notes: parser ingest stays inside the speech-log ingest row; no spawn change; no deps)
- obs-plan — `proposals: []` (notes: no Rust, no OTel, `clock` field carries numbers/key names only; no CI gate change)
- design-system — 1 proposal (DS1)
- layout-templates — 9 proposals (L1–L9)
- test-plan — 4 proposals (T1–T4)
- a11y-plan — 6 proposals (A1–A6); note: the §3 Screen-reader-pattern expected amendment left to the orchestrator's floor

## Proposals and dispositions

| id | detector | section | disposition (check) |
|---|---|---|---|
| DS1 | D-design-derived-count | CP7 Operator-checklist: "(the window footer strip is unbuilt as of 2026-09-02)" → shipped, without the unticked count | apply — routine (playbook `a chunk SHIPS the fix … route-owned-not-shipped`; directive: footer fix spec-owned) |
| L1 | D-layout-surface | §Component — Footer: "Designed, NOT shipped" lead → SHIPPED 2026-09-30 except the unticked count | apply — routine (same rule) |
| L2 | D-layout-derived-count | §Footer: landmark set "`banner` + `main` + two `region`s" → the named set incl. `contentinfo` | apply — routine (derived-count names the SET) |
| L3 | D-layout-surface | §Footer: "height `space-sm`" → `--space-xs` block padding | apply — routine (report Spec claim 6, measured; directive: footer fix spec-owned) |
| L4 | D-layout-surface | §Footer: phase sample `idle / HOLD step 14 / run complete` → the RunState set + per-non-zero-lamp roll-up; HOLD accent a glyph | apply — routine (report Symbols + Spec claim 7) |
| L5 | D-layout-surface (dep of L1) | Wireframe idle `:48` footer row | apply — routine, with L1 |
| L6 | D-layout-surface (dep of L1) | Wireframe HOLD `:75` footer row | apply — routine, with L1 (no step index shipped; glyph accent) |
| L7 | D-layout-surface (dep of L1) | Wireframe idle-with-report `:99` footer row | apply — routine, with L1 |
| L8 | D-layout-surface (dep of L1) | §Operator-checklist `:154` "the window footer strip is unbuilt" | apply — routine, with L1 |
| L9 | D-layout-surface | §Component — Header: the phase line is the single `h1` | apply — routine (`Accurate this-chunk addition`; directive: row-name/h1 spec-owned) |
| T1 | D-tests-coverage | §2 Deterministic bullet: the `sr*` real-wall-clock list gains T-01's quiet window + the clock calibration | apply — routine (`Accurate this-chunk addition`) |
| T2 | D-tests-coverage (dep of T1) | §6 desktop-webview row (SR leg): OS browse keys on every browse row, T-01's second run, calibration + the 357 ms single-key limitation (wrap directive), leg length as a dated sample | apply — routine (`Accurate this-chunk addition`; expected amendment 5) |
| T3 | D-tests-coverage (dep of T1) | §11 E2E no-sleep carve-out extended to the T-01 170 s quiet window | ESCALATE (E1) — no rule governs widening a ban's carve-out; uneasy → RESOLVED by the operator: "Extend the carve-out" (same precondition class; name it with its measured reason; the ban on sleeping to synchronise unchanged) → applied; the reason cited to the 2026-08-16/08-19 legs that measured the dedupe, not to this chunk |

## Applied beyond the proposals
- a11y-plan `:113` — the same-master restatement of O1's retired browse-zone claim (cascade sweep, first listing) → amended.
- test-plan `:307` "one preflight canary" → TWO (T-01's second run), folded into T2's apply.
- Gate treatment asked at the same HALT (P7): the delta guard's red on exactly the three scope-recorded files → "Accept as recorded" (operator).
| T4 | D-tests-derived-count | §1 Untestable zone: the browse-mode zone empty of rows (not-run-here 8 → 0), definition kept, evidence re-pointed | apply — routine (playbook `a spec master's OWN explicitly-provisional claim … retired`; expected amendment 5) |
| A1 | D-a11y-surface | §4 landmark table `:320` contentinfo SHIPPED; measured set | apply — routine (route-owned-not-shipped rule; directive a11y-plan `:548`) |
| A2 | D-a11y-surface (dep of A1) | §4 landmark note `:323` h1 + contentinfo SHIPPED, the asserting specs | apply — routine (directive a11y-plan `:323`) |
| A3 | D-a11y-surface (dep of A1) | §3 Pass spec format: "the `contentinfo` footer are unbuilt" | apply — routine, with A1 |
| A4 | D-a11y-surface (dep of A1) | §5 idle-with-report "(no footer strip ships)" | apply — routine, with A1 |
| A5 | D-a11y-surface | §4 catalog Coverage matrix: row name via `aria-labelledby`, SC 4.1.2, the routine spec | apply — routine (`Accurate this-chunk addition`) |
| A6 | D-a11y-surface (dep of A5) | §3 Per-surface test spec: the S0-09/E0-05 finding recorded closed | apply — routine, with A5 |
| O1 | (orchestrator, check 5) | a11y-plan §3 Screen reader test pattern: T-01 by a second un-stopped run, no browse row without an OS key, the regrade configuration | apply — routine (expected amendment 2; report substantiates; `Accurate this-chunk addition`) |

## Validate checks
1. **Playbook** — as the table; T3 escalates (no rule; a ban's scope).
2. **Cross-contradiction** — none: DS1 / L8 / A3 / A4 all retire the same "footer unbuilt" claim toward the same shipped state (the unticked count stays unbuilt in all four).
3. **Intent-consistency** — the report's deviations are justified (measured); the scope record: `widening` parse-nvda-log.ts carries the operator's word (justified); `companion` operator-hold.e2e.ts serves Titlebar.tsx (holds: its `count()` reads the new class); `in-intent` srOnly.ts serves step 9 (holds: plan step 9 names the `SR_ONLY` idiom).
4. **Absence needs evidence** — no proposal claims an absence; long-line sites (`a11y-plan` `:268` 4 679 chars, `test-plan` `:307`) are applied by offset.
5. **Expected amendments** — 1 a11y §4 → A1/A2 · 2 a11y §3 SR pattern → O1 (raised) · 3 layouts Footer + wireframes + Header → L1–L9 · 4 design CP7 → DS1 · 5 test-plan §1 + §6 → T4, T2.
6. **Disproved claims** — 1 (browse caret / NVDA bind) → curation (host/harness rule) + O1 · 2 (T-01 quiet window) → T1/T2/T3 · 3 (E0-10) → P5 CARRY on "Full-gate regression" (directive) · 4 (count aria-label in browse mode) → curation (a11y rule; no master states it: grep `Scenario count|Scenarios completed` 0 hits in the seven) · 5 (parser clock premise) → T2 + curation (the harness rule `:59` is a curation home) · 6 (footer height) → L3 · 7 (HOLD accent contrast) → L4/L6.
