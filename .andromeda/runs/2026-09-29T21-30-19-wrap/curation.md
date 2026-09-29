CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A rule fixed before a drive is corrected after it by add-only dated brackets, never by rewording" (confidence 0.8)
    Proof: the contract `contracts/pulse-p025-measurement-contract.md` got its post-drive correction add-only.
      - `git diff --numstat 9da18e1 -- contracts/pulse-p025-measurement-contract.md` → `7 0`.
      - `git show 9da18e1:… | sha256sum` → `9da09cc1…`, equal to the pre-leg value recorded in the chunk's
        `evidence/hue-verdict.md`.
      - The overseer directed the dated correction ("the recorded pre-leg sha256 stays valid for the rule text as
        it stood").
      - Signals: operator direction +0.4, verified by measurement +0.4.
                                              + "pulse-app raises its own compact widget at boot: verify it, don't ask for it" (confidence 0.8)
    Proof: Pulse `pulse-app/src/main.rs:1120` calls `window::show_compact_widget(app)` in `setup`, read with
      `git show` at `4502d5d`. A window enumeration of PID 22800 at launch read
      `title "andromeda-pulse" visible True minimized False size 496x279` beside the hidden 1296x809 dashboard.
      The leg then recorded an in-window rise.
      - This changed the leg's design: the plan had assumed the operator opens the window.
      - Signals: verified by measurement +0.4, specific technical detail +0.2, no other durable home +0.2
        (no route annotation, no master amendment, no playbook rule, and no matrix note carries it).
  Filters: 3 dup · 1 task-specific · 1 conflict (→ handoff) · 0 deferred
    - dup:
      - (a) the dictated sidecar package name was wrong. Its correct form is already in the
        `verification-harness.md` 2026-08-10 entry, and CLAUDE.md 2026-08-09, extended 2026-08-15, covers
        dictated citations. The entry worked: the command was checked before it ran.
      - (b) bash-guard blocks a doubled backslash. `host-win32.md`'s 2026-09-23 corrected clause covers it.
      - (c) the `f0c38f5` grep claim written before the grep ran. This is a DEFECT record, so it is a
        recurrence, not a drop: `recurrence-despite-learning` against CLAUDE.md 2026-08-09 (→ handoff).
    - task-specific: `gate.py run --skip` comma-splits its reason list, so a reason carrying a comma is
      truncated. This is pipeline tooling, telemetry of the tool rather than a project learning (Filter 2 /
      curation-guide §Analysis scope).
    - conflict: the overseer's directive to give a live-leg data dir a letters-only suffix ("no digit run the
      scrubber could read as a card") contradicts `verification-harness.md` 2026-08-18
      "`%TEMP%/pulse-legs/<ts>`", where the timestamp is a digit run. → handoff "Curation conflicts", for the
      operator to decide.
  CLAUDE.md size: 137/200 · T1 47.3 KB, 9 of 16 over 600 B (health check 1, ⚠ — operator promotion; unchanged by this wrap)
