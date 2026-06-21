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
