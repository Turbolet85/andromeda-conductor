
## 2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09 — the capture's host-path mask gains two temp roots; the fourth series joins the row and the exception inventory
**Section:** §Input Validation → Real-model capture ingest · §Security Anti-Patterns → Data Protection (the `corpus.db` ban's ratified exception)
**Change:**
- The capture row now states `mask_host_paths`' named roots: `/home/`, `/Users/`, `%APPDATA%` and, since 2026-10-06, `/tmp/` and `/var/tmp/`, all with the same match semantics. Was: the stage named with no root set. No committed capture holds a temp-rooted path, so no digest pin moved.
- The row's workspace-key provenance was "at `fcc31b2`, unchanged at `a2addb3`"; it adds "and at `5f77859`". Its two series-dated measurements ("the 2026-09-30 and 2026-10-01 series") now include the 2026-10-06 series: leaf 0 times across its three captures, and on that series — the first on the Linux dev host, on a home-rooted data dir — each `## Previously Seen` suffix is a POSIX path printed `<redacted>`, taken whole by `redact_value` after the key mask.
- The exception's per-series inventory gains "the 2026-10-06 series' d1, d2 and d3 captures carry one report body each". The exception's scope and terms are unchanged.
**Why:** `redact_value` names neither temp root, so before the extension a temp-rooted workspace path kept its parent through the whole chain (two harvest arms, each red before the extension). The series' data dir was placed home-rooted for that reason and the mask was extended after the drives, on the overseer's answer at the plan's fork round (founder-delegated). Standing rule for a later leg: a temp-rooted data dir is now masked, and the mask's root set lives in this row.
**Kept:** `redact_value`'s own token set (the path-handle row and obs-plan's redaction rows) is untouched — a different function.
**Ref:** .andromeda/runs/2026-10-06T21-10-08-wrap/
