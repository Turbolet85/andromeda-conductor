# A11y Plan — Amendments

_Append-only changelog of amendments to `a11y-plan.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-16-emission-journal-writer — violation-schema reproduction gains `read_back_observed_at` (obs bind)
**Section:** §3 Structured violation JSON schema (both verbatim reproductions of the obs Run-report envelope)
**Change:** added `read_back_observed_at` after `journal_emitted_at` in the reproduced Run-report envelope, keeping a11y's violation-schema reproduction consistent with the now-11-field obs §6 envelope.
**Why:** honor the a11y↔obs schema bind (D-a11y-obs-schema). The obs envelope gained `read_back_observed_at` this chunk (see `obs-plan-amendments.md`, same marker); a11y reproduces that envelope verbatim (violations fold into `fingerprints[]`), so its reproduction tracks the field. No a11y behavior change; first amendment to a11y-plan (sidecar lazy-created here).

## 2026-06-21-run-report-envelope-serializer — CalibrationRegion→ManualCheck co-occurrence + verdict-first lamp
**Section:** §6 State color tokens — State-naming crosswalk (HOLD + Manual rows)
**Change:** the HOLD row's `state` is now `ManualCheck` (the default `CalibrationRegion → ManualCheck` mapping), not "verdict-only/n/a"; added the verdict-first lamp-precedence rule (`verdict == CalibrationRegion` ⇒ HOLD lamp regardless of state); clarified the Manual row (a `ManualCheck` state WITH a `CalibrationRegion` verdict is the HOLD lamp, not Manual).
**Why:** the run-report-envelope-serializer chunk's `Verdict::default_report_state` maps CalibrationRegion→ManualCheck, so `verdict==CalibrationRegion` + `state==ManualCheck` now co-occur — the prior "do not join HOLD on state / NOT the ManualCheck state" design no longer holds. Verdict-first precedence keeps the six lamps distinct (HOLD identified by verdict; Manual = ManualCheck with no verdict). Escalated + user-confirmed 2026-06-21. Cascade no-op (a11y-summary.md / rules/a11y.md carry no crosswalk detail — grep-confirmed).

## 2026-08-09-out-of-scope-classification-treatment — --status-residual's non-lamp reuse scoped out of the six-label assertion
**Section:** §6 Visual Design Verification → State color tokens (not-color-alone)
**Change:** recorded that `--status-residual` also tints the coverage-matrix out-of-scope Mode cell (webview `.cov__mode--out-of-scope` · cli ANSI 246 · Markdown emphasis), that this is a **coverage-mode classification and not a seventh lamp state**, and that the six-label not-color-alone assertion therefore keys on the LAMP display labels — the Mode cell's own `not-conductors` text is its signal (no glyph supplement). Contrast still applies to the pair.
**Why:** §6 bound the token solely to the `Residual` lamp and §10 gates the build on the six-label assertion, so a token-tinted element whose label is `not-conductors` would sit uncovered — or worse, be mistaken for a missing seventh state. Detector D-a11y-surface.

## 2026-08-21-delegated-timing-budgets-proven — Residual crosswalk row disambiguated
**Section:** §6 Visual Design Verification -> State color tokens -> state-naming crosswalk, `Residual` row
**Change:** The `Residual` row's obs `verdict` cell now records that a `KnownResidual` state carrying `verdict == CalibrationRegion` is the HOLD lamp, not Residual (verdict-first) — mirroring the `Manual` row's existing disambiguation.
**Why:** Leg D produced exactly that governed combination (`CalibrationRegion` + `KnownResidual`), which the row's bare "(n/a — no verdict)" denied; a downstream join from a Residual lamp back to its journal row would have missed a real state.
