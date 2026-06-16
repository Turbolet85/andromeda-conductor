# A11y Plan — Amendments

_Append-only changelog of amendments to `a11y-plan.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-16-emission-journal-writer — violation-schema reproduction gains `read_back_observed_at` (obs bind)
**Section:** §3 Structured violation JSON schema (both verbatim reproductions of the obs Run-report envelope)
**Change:** added `read_back_observed_at` after `journal_emitted_at` in the reproduced Run-report envelope, keeping a11y's violation-schema reproduction consistent with the now-11-field obs §6 envelope.
**Why:** honor the a11y↔obs schema bind (D-a11y-obs-schema). The obs envelope gained `read_back_observed_at` this chunk (see `obs-plan-amendments.md`, same marker); a11y reproduces that envelope verbatim (violations fold into `fingerprints[]`), so its reproduction tracks the field. No a11y behavior change; first amendment to a11y-plan (sidecar lazy-created here).
