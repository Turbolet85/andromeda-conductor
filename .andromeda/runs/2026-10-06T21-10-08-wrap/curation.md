# Curation log — 2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "a content proof by a short flag literal can read 0 on a correct release build — take a two-sided control on both binaries before pre-registering one"
                                              + host-win32.md: "inputs.py snap refuses a source under the OS temp dir — snap a relayed message from the run dir with --message-file and --origin"
                                              ~ verification-harness.md 2026-10-03 (the WebKit launch lever): corrected in place
                                              ~ verification-harness.md 2026-06-27 (the L4 model's name): corrected in place
  Tier 3 (.claude/docs/session-learnings.md): ~ the 2026-06-27 live-findings entry (the L4 model's name): corrected in place
  Filters: 0 dup · 1 task-specific · 1 conflict (→ handoff) · 0 deferred (→ handoff)
  CLAUDE.md size: 138/200 · T1 47.8 KB, 9 over 600 B
```

Writes: T2 4 (2 new, 2 corrections) · T3 1 (a correction). Corrections are exempt from the three-entry cap; two
new entries sit under it.

## Applied

- **verification-harness.md, new 2026-10-06 entry** (the binary-proof lesson; the overseer's curation note left
  the wording to this wrap). Signals: verified by measurement (+0.4) · an explicit curation request (+0.4) · a
  specific technical detail with context (+0.2) = 1.0.
  Proof: plan gate entry 8 read `--grammar-file` 0 at exit 1 on the rebuilt `pulse-app`; the two-sided control read
  stale 0 / new 0 for the literal, 0 / 1 for each 8-byte half, and 0 / 2 for `hypotheses-item-statement-kv` — the
  chunk's `evidence/attempt-ledger.md` §The binaries; the pre-registered section was amended before any drive.
- **host-win32.md, new 2026-10-06 entry** (the inputs tool's temp-dir refusal). Signals: verified by measurement
  (+0.4) · a specific technical detail (+0.2) = 0.6, plus the no-other-home signal (+0.2; the fact reached no route
  annotation, master, rule or contract this wrap) = 0.8. One sentence, under the always-loaded file's 600 B bar.
  Proof: two consecutive `inputs.py snap` calls exited 3 with "under the temp dir /tmp — a scratch file is no
  input" (`--source`, then `--message-file`); the third, from the implement run dir, snapped I20.
- **verification-harness.md 2026-10-03, corrected** (the overseer's curation note, relay I23 §3).
  Proof: the launch env block carried neither rendering lever; Pulse logged `app.boot.render.posture` `lever`
  `__NV_DISABLE_EXPLICIT_SYNC` `posture` `applied`; the app was alive at 10 s and through three drives — the
  ledger's §The model-run slot and the launch.
- **verification-harness.md 2026-06-27 and session-learnings.md 2026-06-27, corrected** (routed here by the
  cascade sweep: a curation home is never edited by the cascade).
  Proof: Pulse's `interpretation.model.load` line read `model_identity` `gemma-4-E4B-it-Q4_K_M` before d1; the model
  file's sha256 equalled the pin — the ledger's booted-posture and pre-leg rows. architecture dropped the same
  literal in this wrap's P2.

## Not applied

- **Task-specific (1):** "the capture prints two `canary:` lines for a three-storm canary" — amended into obs-plan
  §4 this wrap; the master owns it.
- **Conflict → handoff (1):** CLAUDE.md's 2026-08-22 Tier-1 entry opens "This host is Windows-only — no Linux
  runner exists or is planned", while this series ran on the Linux dev host. No candidate of this session corrects
  it and the entry is the operator's to re-word; logged under the handoff's Notes.
- **Recurrence-despite-learning → handoff:** host-win32.md 2026-09-08 (the Bash working directory persists) — a
  `cd` at the head of one probe moved the session cwd again at this wrap (a sixth consecutive session).
