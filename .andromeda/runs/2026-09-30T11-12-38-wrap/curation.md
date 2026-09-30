# Curation — 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    ~ verification-harness.md 2026-09-02 (read the SPEC LIST): "(`Spec Files: 1 passed, 1 total`)" corrected — the runner prints a TAB after `Spec Files:` (correction, cap-exempt)
    + verification-harness.md 2026-09-02 (screen reader driven by the OS FOREGROUND): extended — a parallel agent session takes the foreground; a row it silences is a finding naming the window, never a pass (confidence 0.9)
    ~ verification-harness.md 2026-08-18 (one parent for live-leg data dirs): refined on the operator's wrap directive — parent `pulse-legs` kept, leaf letters-only (curation conflict resolved)
    + host-win32.md: "A BOM-less .ps1 holding any non-ASCII character fails to PARSE under Windows PowerShell 5" (confidence 0.8)
    + host-win32.md: "mklink /J through the Bash tool dies with Invalid switch; junction via New-Item; remove it non-recursively first" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup ("pulse-app's webview tree lingers after a forced stop" ↔ host-win32.md 2026-09-16 repeated census passes — the rule was applied, not a recurrence) · 0 task-specific · 0 conflict · 1 below threshold ("run the base-bundle SR control before attributing silence", exactly 0.6 — carried instead as the new route entry's CARRY) · 0 deferred
  Extended: T2/verification-harness.md: "2026-09-02 … A screen reader is driven by the OS FOREGROUND" + "a parallel agent session takes the foreground; silenced row = finding naming the window"
  No-other-home: "BOM-less .ps1 with a non-ASCII character fails to parse" · "mklink /J via MSYS; junction removal order"
```

## Proofs
- Correction (`Spec Files`): implement entry 8's log read with `od -c` shows `S p e c   F i l e s : \t   1   p a s s e d`;
  the overseer read `runs/a11y-e2e.log` with `cat -A`: `Spec Files:^I 1 passed`; a scratch-plan control in the session
  scratchpad — the TAB atom green on the real bytes, red on the single-space form, the old atom red on the real bytes.
- Extension (foreground): `sr-empty` runs at 10:09Z and 10:12Z — NVDA spoke "Windows PowerShell terminal ◐ pulse-builder"
  and (second run) a WebView2 console window's title, then nothing for every focus row (chunk
  `evidence/driven-leg.md` Findings 1). The operator's rule was stated in the slot directive. Confidence: operator
  rule (+0.4) · "never" (+0.3) · technical detail (+0.2) = 0.9.
- Refinement (data-dir leaf): the operator's wrap directive carried in the chunk's `plan.md` Implementation notes
  ("the curation conflict resolves REFINED") and `scope.md`; the leaf this session used was `drivenkeys`.
- `.ps1` parse: the first `launch-pulse.ps1` run failed `Unexpected token 'pick'` at the em dash and launched nothing
  (exit 1); the ASCII rewrite launched pid 20656. Confidence: measured failure (+0.4) · detail (+0.2) · no other
  home (+0.2) = 0.8.
- Junction: `cmd //c mklink /J …` printed `Invalid switch - "…node_modules"` and the vite build then found no
  `node_modules`; `New-Item -ItemType Junction` worked; the non-recursive `Directory.Delete(path, $false)` removed the
  link and the real `node_modules` kept its 441 entries. Confidence: measured (+0.4) · detail (+0.2) · no other home
  (+0.2) = 0.8.
```
