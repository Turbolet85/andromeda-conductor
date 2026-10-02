# Scope — 2026-10-02-captured-fingerprint-values-elided

**Working entry (working-route.md:81):** Captured fingerprint values elided — no committed capture carries a
`fingerprint_hex` value or prefix, the d3 residual fixed rather than ratified.

**Version:** conductor-0.3.0 · Epoch 5 — Polish & ship · the first markerless entry, not blocked.

## What this chunk builds
It closes the two stated residuals of the real-model capture's fingerprint elision by FIXING them, never by ratifying
them. That makes the security-plan clause "every fingerprint-shaped token elided" true of every committed real-model
capture with no stated exception. The founder's ruling is the mandate. Overseer rulings and the founder-ratification
PENDING wording around the residuals retire with the fix.

## CONTEXT block (folded verbatim as a hypothesis, re-verified below)
- Founder ruling 2026-10-02, live, relayed by the overseer: «Так все что планировали на 0.3.0 делаем сразу как
  положено, ничего не переносим» (everything planned for 0.3.0 is done properly now, nothing is deferred). The d3
  residual is FIXED, not ratified.
- The two stated residual sites are:
  - security-plan §Security Anti-Patterns → Data Protection's frozen 2026-09-22 file (one `fingerprint_hex` prefix);
  - the graded 2026-10-01 d3 capture (an all-digit 8-character prefix that the elider passes by its own definition).
- Whether more sites exist is the take-up's census. The CONTEXT points at arch §Established Decisions [Read-Back
  Dependency Posture], which records ten committed 2026-09-10 live envelopes carrying a 32-hex cue fingerprint.

## Re-verification of the named coordinates (at take-up, 2026-10-02)
- **security-plan.md:336** states both residuals as the CONTEXT names them (re-read 2026-10-02):
  - The frozen 2026-09-22 file keeps its one un-elided storm prefix "because frozen evidence is never edited".
  - The graded 2026-10-01 `rm-capture-d3.txt` is "RULED BY THE OVERSEER, FOUNDER RATIFICATION PENDING", and its tie
    is held exactly by the harvest arm `the_2026_10_01_captures_carry_no_fingerprint_and_no_workspace_key`.
  - `.claude/rules/security.md` and the handoff carry the same two residuals.
- **Site 1:** `conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/evidence/rm-capture.txt:256-257`.
  - Both lines carry `fingerprint_hex=` followed by one 8-hex value. The two lines hold the same value, so it is one
    prefix on two lines.
- **Site 2:** `conductor-0.3.0/chunks/2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix/evidence/rm-capture-d3.txt:514-515`.
  - Both lines carry `fingerprint_hex=` followed by one all-digit 8-character value. Every other `fingerprint_hex`
    line in that file reads `<fingerprint>`.

## Census (take-up, measured 2026-10-02 over `git ls-files`)
The probe takes `fingerprint_hex` followed by a value and classifies the value as elided / hex / all-digit. The
`.andromeda/runs/` trees and the friction ledger are excluded. Script: the session scratchpad `fp_census.py`.

- **The real-model capture class** (`rm-capture*.txt`, the class security-plan's scoped exception admits) has 14
  committed files. [premise-corrected at P4: the take-up census line read 15. The count is
  `git ls-files 'conductor-0.3.0/chunks/*/evidence/rm-capture*.txt' | wc -l` = 14.]
  - Exactly the TWO stated sites carry a value: 2026-09-22 has hex8 ×2, and 2026-10-01 d3 has digits8 ×2.
  - Every other `fingerprint_hex` value in that class reads `<fingerprint>`.
  - So within the class the census finds no third site.
- **Outside that class**, `fingerprint_hex` values (8-hex) also sit in:
  - conductor-0.2.0 live-leg evidence: Pulse's `pulse-events.jsonl` ×5 legs, the storm and distinct leg-window
    JSONL ×4, `trio-and-storm.jsonl`, `leg-witnesses.jsonl`, and three `leg-verdict.md`;
  - harvest TEST SOURCE in `crates/conductor-run/tests/`: `baseline_harvest.rs` 1, `pii_harvest.rs` 2,
    `severity_harvest.rs` 3, `storm_harvest.rs` 2;
  - prose quoting the residual: the 2026-09-22 `report.md` and the 2026-09-30 `plan.md` / `research.md`.
- Whether that outside-the-class population is within this chunk's subject is NOT settled by the entry. This was
  verified at P3 and it stays a P4 fork.
  - The same P3 sweep found the frozen site's value quoted verbatim in three committed PROSE files:
    - the 2026-09-22 `report.md` (×1);
    - the 2026-09-30 `plan.md` (×3) and `research.md` (×1).
    - The d3 value is quoted in none.
    - Two gate trails under `.andromeda/runs/` (the 2026-09-30 implement and wrap runs) quote it as well.
    - Derivation, which never prints a value: extract each residual value from the two sites at `31f9d92`
      (`git show 31f9d92:{path} | grep -o 'fingerprint_hex=[0-9a-f][0-9a-f]*'`), then
      `git grep --untracked -l -F {value} -- .` per value. The files are listed in research.md.
  - Several outside-class values are load-bearing for verified graders:
    - `severity_harvest.rs:351` asserts fingerprint IDENTITY across lines;
    - `storm_harvest.rs` reads and compares the field.
  - The title says "no committed capture", and in this project's vocabulary "capture" names the real-model
    capture.
  - The CONTEXT's own census pointer (the ten 2026-09-10 envelopes) aims outside that class.
  - Those values are Pulse's fingerprints of Conductor's OWN synthetic exceptions. Conductor derives the same values
    itself, aligned on 2026-08-16. They are not corpus-rendered model text.
  - **Decided at P4 (2026-10-02; the overseer answered for the founder): captures plus the residual quotes.**
    - Both capture sites are fixed, and every committed quote of either residual value is replaced. That covers the
      three prose files and the two gate trails, so the values leave the tree.
    - The outside-class values stay, as noted above.
    - Overseer's note: a verbatim prose quote of the residual value is the same Data Protection residual. History is
      not rewritten; git history keeping the original is accepted.
- The arch [Read-Back Dependency Posture] "ten committed 2026-09-10 live envelopes" claim was located and verified at
  P3. It is the "Freshness is the canary's carrier BY CHOICE" paragraph (`architecture.md:64`): "Ten committed live
  envelopes of 2026-09-10 carry such a 32-hex value".
  - Those envelopes are Conductor's own run journals under conductor-0.2.0 evidence, outside the real-model capture
    class.
  - The arch claim cites them as its basis, so the wide reading would remove the evidence an arch sentence rests on.

## Boundaries
- Real-model capture residuals:
  - IN: both stated residual sites closed by a fix;
  - IN: the harvest arm(s) that currently hold the residuals EXACTLY (they would turn red on a fix) re-expressed as
    "zero un-elided values";
  - IN: the sha256 digest pin of any re-written graded capture, updated in the same change;
  - IN: the security-plan / rules / handoff residual wording retired at wrap.
  - IN (P4 subject answer): the three prose quotes (2026-09-22 `report.md`; 2026-09-30 `plan.md` and `research.md`)
    and the two 2026-09-30 gate trails under `.andromeda/runs/`, each value token replaced by `<fingerprint>`.
  - IN: none of this chunk's own artifacts quotes either value. Every count is derived by extracting the values
    from `31f9d92` and is never printed.
- How the frozen 2026-09-22 file is fixed was a P4 fork, because "frozen evidence is never edited" is a standing
  rule. This was verified at P3.
  - **Decided at P4: elide it in place.** Every citation keeps resolving. The frozen-evidence exception is recorded
    against the founder ruling of 2026-10-02 in the chunk record (overseer's note).
  - Exactly one test reads the file: `the_elided_copy_is_the_frozen_capture_through_the_rule`
    (`real_model_harvest.rs:1713-1722`). Its `copy != frozen` assertion inverts under in-place elision.
  - Its chunk's `plan.md` cites the path at `:460`, `:481`, `:484` and `:498`.
  - The options are:
  - elide it in place, with the rule's exception recorded;
  - remove it from the tree, since the graded elided copy already exists under the 2026-09-30 chunk;
  - leave it frozen and record why the rule outranks the ruling. This option contradicts "fixed, not ratified".
- [premise-corrected: widening the shape is not viable. An all-digit run is a nanosecond stamp or a seed in every
  committed capture, so four `elide(committed) == committed` arms (`real_model_harvest.rs:1720`, `:1834`, `:2055`,
  `:2251`) would go red over captures that hold no fingerprint.] So d3 has ONE viable fix, and it is not a fork.
  - `elide_fingerprints` gains a CONTEXT-KEYED branch: a `fingerprint_hex=` value is elided whatever its character
    class, and an existing `<fingerprint>` stays untouched.
    - The `mask_workspace_key` precedent applies (`real_model_common/mod.rs:197`).
    - The unkeyed all-digit pass is kept.
  - Then the graded d3 file is re-elided and its pin moves (`real_model_series/mod.rs:95`).
  - Its grade cannot move. Verified at P3: the harvest reads `fingerprint_hex` only in the residual arm (`:2258`)
    and the synthetic elider arm (`:1433-1434`), and `real_model_common/mod.rs` reads it nowhere.
  - The capture's `scenario_storm=` bool is computed from the raw value before elision
    (`real_model_live.rs:666-670`).
  - The fix also governs every future capture, because the producer `emit_block` (`real_model_live.rs:161`) calls
    the same elider.
- OUT: git-history rewriting. Values in past commits stay there. Force-pushing is barred, and the subject is the
  committed TREE.
- OUT: any change to how Pulse derives or logs fingerprints.
- OUT: the two BLOCKED-ON entries after this one (the P-075 re-round and the fourth `v3-09` series).

## CI read at Setup (5a)
- `31f9d92` (the last wrap's flip, = HEAD): **green**, checks 3/3, wall 671 s, CI#37012156437 push completed/success.
  Nothing to fold.

## Annotations folded
- `CONTEXT:` (working-route.md:81, 640 chars). Folded above in full. No PREREQ, CARRY, BLOCKED-ON or WATCH on this
  entry.
