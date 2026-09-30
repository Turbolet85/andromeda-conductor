# Scope — 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir

**Working entry (`working-route.md:56`):** Interpretation re-proven on a clean-named data dir — a new pre-registered
real-model series for `v3-09` on a data dir whose name no scrubber pattern can match.

**Matrix target:** `v3-09` (*Real-model interpretation leg*, `dynamic-external`, status `deferred`, `chunk: null` at
take-up — un-claimed to the pool by the 2026-09-29 wrap, owned by this entry). Its acceptance as it stands: a pre-stated
series of at most five drives through `bash scripts/agent-run.sh run --live real-model` against a pulse-app launched per
`contracts/pulse-real-model-leg-posture.md`; design, decision rule, grading rules and pass condition fixed in that
contract before the first drive; every capture committed under `evidence/`, pinned by
`crates/conductor-run/tests/real_model_harvest.rs`, one row each in `evidence/attempt-ledger.md`; met iff ≥ 1 drive is
graded AND every graded drive is `Identified` (rank-1 statement names `conductor` as a whole word + a retry token), with
the real-model witnesses; a graded `NotIdentified` is NOT met and never replaced.

**Chunk base (W182):** `9785405` (`9785405bf9d55405d7c99cb081566db421685598`), HEAD at take-up. Every diff-shaped gate
probe names it explicitly (`git diff --numstat 9785405 -- <f>`), because the operator pre-CI commit moves HEAD before
the wrap (overseer directive, W182 — as applied at `2026-09-29-hue-shift-budget-graded-hard/scope.md:128-130`).

## What this chunk builds — ORDER IS PART OF THE SCOPE (operator directive)
The hermetic CARRYs are planned and landed FIRST; the drive series is planned LAST and is gated.

1. **Digest pins replace the literal series captures in test source** (CARRY 2). The harvest's series pins in
   `crates/conductor-run/tests/real_model_series/mod.rs` (78 879 B at base; 28 `fingerprint_hex=` hits) become one
   sha256 digest pin per committed evidence file, so no corpus text sits in test source. This discharges the recorded
   BREACH in security-plan §Data Protection (`security-plan.md:335` — "Recorded BREACH (2026-09-29), never ratified")
   WITHOUT a widening (a widening waits for the founder's live word; security.md 2026-09-29). Must land before the next
   series runs.
   - The grading/routing assertions the literals feed today (`grade`, `row`, `route`, witness predicates) keep their
     coverage by reading the committed evidence FILE at test time after its digest matches — not by dropping the
     assertion. VERIFIED at P3: the harvest already carries both readers (`committed_capture` `:1493`,
     `committed_series_capture` `:1616`), and the literals are duplicates held equal to the files (`:1598`, `:1640`).
     Only the operand the grading reads moves. `sha2 0.10.9` is already in `Cargo.lock` (via `tauri-codegen`,
     `wry`), so a `conductor-run` dev-dependency adds no package (562 held).
2. **The capture masks the data dir's workspace key in the verbatim report body** (CARRY 3). Keeps the ingest row's
   never-printed guarantee (`security-plan.md:121`: "the capture itself never prints the workspace basename — but the
   verbatim report body carries Pulse's Project Context, which shows the data dir's workspace key however Pulse's
   scrubber renders it") without a widening. Lands before the next series.
   - The mask applies to the report body the real-model capture writes (`real_model_live.rs` `emit_block` `:155`,
     beside `redact_value` + `mask_host_paths` + `elide_fingerprints`). The key is the data dir's BASENAME, derived
     from the guarded `capture_paths::pulse_logs_dir_from` path.
     [premise-corrected: "mask the key both RAW and in any scrubber-rendered form" — a scrubber rendering carries no
     key text (`[redacted: credit_card]` replaces it whole, `rm-capture-b2.txt:455`). The mask is: the RAW basename
     wherever it appears (it reaches the body at the Project Context `workspace=` value AND each `## Previously Seen`
     entry's `({workspace})` suffix, `:455`, `:459`), plus the `workspace=` value of `## Project Context` WHATEVER it
     holds, so a rendering the capture cannot predict still prints as the placeholder.]
3. **The 2026-09-22 capture's storm prefix elided** (CARRY 1). `fingerprint_hex=…` in the pinned `PINNED_CAPTURE`
   literal (`crates/conductor-run/tests/real_model_harvest.rs:1455`; the source evidence carries it at
   `conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/evidence/rm-capture.txt:256-257`) is elided IN THIS
   CHUNK — never an edit to that frozen chunk's evidence (relay `conductor-wrap-50-2026-09-29` §2.3 — the relay is not
   in this repo; the coordinate is taken from the entry's own text and the two in-repo files above).
   - Resolved at P3 to (b): an elided copy committed under THIS chunk's `evidence/`
     (`rm-capture-2026-09-22-elided.txt`, the frozen file passed once through `elide_fingerprints`, otherwise
     byte-identical), which the harvest digest-pins and grades. Route (a) (a digest over the frozen file) leaves the
     only graded copy un-elided, and security-plan `:335` names the remedy "that prefix elided". Stated residual: the
     FROZEN 2026-09-22 file keeps its prefix, because the entry forbids editing it. That is recorded for the wrap's
     BREACH-line amendment, never silently counted as remedied.
4. **CI red folded (operator word, overseer founder-delegated, 2026-09-30).** CI#36635281444 on `9785405` —
   `verdict: red · checks 3/3 · first-fail +592 s A11y gate (routine arm · axe · contrast · violation JSON)`; failed
   job 109634300221; the failing test `desktop a11y — routine arm (no live Pulse) zero axe violations on the
   Minimal-tier baseline` threw `Error: Page/Frame is not ready` at `assertFrameReady`
   (`@axe-core/webdriverio/dist/index.mjs:109`) via `axeSourceInject`; 11 passing / 1 failing / 2 skipped; WebView2
   131.0.2903.86. (The listing's wall-clock clause was clipped by the tool; first-fail +592 s is what it printed.)
   - Nondeterministic, not a regression — VERIFIED against THAT RUN (CI#36635281444 · `9785405` · the A11y gate's
     routine arm): the wrap commit's diff over `9da18e1` touches no code, UI or workflow path, and CI#36632527433 on
     `9da18e1` was green 3/3. Mechanism measured in the run's log: `@axe-core/webdriverio` 4.12.1's
     `assertFrameReady` races `execute(() => document.readyState === "complete")` against a hard
     `FRAME_LOAD_TIMEOUT = 1000 ms`, and its catch-all turns any miss into this string. The probe POSTed at
     `21:49:59.654Z` and returned `true` at `21:50:00.827Z`, 1 173 ms later. The `before` hook's second poll had
     already taken 600 ms, and every later execute answered in 5-25 ms. So the webview's main thread is busy for ~1-2 s
     right after `#root` first mounts, and a 1 s wall-clock readiness budget races that window.
   - [premise-corrected: the BiDi mechanism on record (a11y-plan amendments `:42`) is NOT the cause — the session is
     enforced-classic (`wdio.conf.ts:340`), the probe took `assertFrameReady`'s non-BiDi arm (POST `/execute/sync`),
     and it RETURNED `true`; the shared string is the library's catch-all for a missed 1 000 ms budget.]
   - Operator's witness requirement, verbatim: "a witness that cannot pass vacuously (reproduce the race, then show it
     cannot recur, not one lucky green)". The fix carries its own acceptance criterion.
5. **The new pre-registered series** (the entry's substance; LAST). A new real-model series for `v3-09` on a
   clean-named data dir, pre-registered like the `:50` chunk's (`2026-09-29-diagnostic-quality-cluster-off-the-drift-pin`):
   a FIXED drive count, the decision rule stated before the first drive, no retry-until-pass.
   - Gated, both conditions required before any drive: (a) the Pulse scrubber fix is COMMITTED AND PUSHED (the
     BLOCKED-ON's own clearing event); (b) the operator's slot. Implement stops and asks for the slot.
   - The data dir's leaf name carries no digit at all, so it matches no scrubber pattern even without the Luhn fix.
     VERIFIED at Pulse `a08ae29` (`crates/security/src/scrubber.rs:116-127`): `credit_card` and `ssn` both need a
     digit run, and the provider-key and email patterns need a vendor prefix or an `@`, which a plain lowercase
     hyphenated leaf avoids.
     This touches the handoff's open curation conflict (letters-only suffix vs `verification-harness.md` 2026-08-18
     `%TEMP%/pulse-legs/<ts>`); the rule-file entry is the operator's call at wrap, and this chunk's name choice is a
     plan decision, not a curation write.
   - The contract `contracts/pulse-real-model-leg-posture.md` gains the NEW series' design and rule (fixed before the
     first drive), add-only beside the 2026-09-29 series record (`## The drive series`, `:213-321`) — never
     rewriting that record. VERIFIED: the add-only dated idiom is already in use at `:307-313`.
   - `v3-09` claimed if P5 concretizes an acceptance this chunk's lifecycle can reach.

## Folded freight (`working-route.md:56`, five blocks, all per `route.py pins`)
- **BLOCKED-ON** (478 chars, folded whole): "Pulse's scrubber fix — its `credit_card` pattern matched the data dir's
  workspace key, so the 2026-09-29 series' one graded drive read `NotIdentified` on `[redacted: credit_card]` (as
  measured at `conductor-0.3.0/chunks/2026-09-29-diagnostic-quality-cluster-off-the-drift-pin/evidence/rm-capture-b2.txt`);
  routed on Pulse's own route as the entry after its CI chunk (relay `conductor-wrap-50-2026-09-29` §2.3); clears when a
  Pulse commit carrying that fix is pushed."
  - Taken up anyway by founder ruling ("nothing skipped or deferred"); the drive series (item 5) carries the block as
    its gate.
  - Re-verified at take-up: the evidence file exists; the prior report states the mechanism
    (`2026-09-29-diagnostic-quality-cluster-off-the-drift-pin/report.md:31`, workspace key `rm-20260923-093840` rendered
    `[redacted: credit_card]`).
  - Pulse at take-up: local HEAD `a08ae29` on `chore/migrate-pulse-to-v3`, 0/0 against its local
    `origin/chore/migrate-pulse-to-v3` (no fetch run). The committed credit_card pattern is `\b(?:\d[ \-]?){13,19}\b`
    with the comment "Doesn't check Luhn" (`scrubber.rs:116-121` at `a08ae29`). **The fix is NOT committed at take-up.**
    Pulse's worktree is dirty, including an uncommitted chunk dir `andromeda-pulse-0.3.0/chunks/2026-09-29-scrubber-path-false-positive/`
    — NOT read (operator: "read Pulse at its committed HEAD only"). The fix's design per the operator: a Luhn check on
    13-19 digit windows, founder-ratified 2026-09-30 — recorded as relayed, not measured.
- **CONTEXT** (207 chars, folded whole): "founder 2026-09-29 — fix the cause and run a NEW series; `v3-09` is recorded
  NOT MET by the 2026-09-29 series, never closed as deferred, and this entry owns it (un-claimed to the pool at that
  wrap)." Re-verified against the matrix `notes` (2026-09-29 wrap note).
- **CARRY** (200 chars) → item 3 above, verbatim: "elide the 2026-09-22 capture's storm prefix (`fingerprint_hex=…`) and
  its pinned `PINNED_CAPTURE` literal in this entry's own chunk — never an edit to that frozen chunk's evidence (relay
  §2.3)."
- **CARRY** (355 chars) → item 1 above, verbatim: "replace the harvest's literal series pins in
  `crates/conductor-run/tests/real_model_series/mod.rs` with a sha256 digest pin per evidence file, so no corpus text
  sits in test source — the recorded BREACH in security-plan §Data Protection (E1 at the 2026-09-29 wrap; a widening
  waits for the founder's live word) — landed before the next series runs."
- **CARRY** (202 chars) → item 2 above, verbatim: "the capture masks the data dir's workspace key in the verbatim report
  body before the next series, keeping the ingest row's never-printed guarantee without a widening (E2 at the
  2026-09-29 wrap)."

## Causal claims carried in (closed at P3, marker kept)
- "its `credit_card` pattern matched the data dir's workspace key" (marker: "as measured at `…/rm-capture-b2.txt`") —
  VERIFIED: `rm-20260923-093840` holds 14 digits with one separator, inside `\b(?:\d[ \-]?){13,19}\b`, and
  `rm-capture-b2.txt:455` shows `workspace=[redacted: credit_card]`.
- "the model diagnosed exactly that" (`attempt-ledger.md:23-24`, marker: measured) — kept as recorded: b2's incident
  title is "Credit Card Issue" (`rm-capture-b2.txt:459`). Whether the redaction CAUSED `NotIdentified` is what the
  new series tests, and nothing here settles it.

## Boundaries (out of scope)
- No edit to Pulse's repository; Pulse is read at its committed HEAD (`git show <sha>:<path>`), never its worktree. The
  scrubber fix is Pulse's.
- No edit to any frozen chunk's `evidence/` (2026-09-22, 2026-09-29 series).
- No widening of the security-plan scoped exception (the corpus-text exception stays as ratified 2026-09-23); the
  BREACH is REMEDIED, not ratified.
- No `v3-09` acceptance weakening: the at-most-five, every-graded-must-be-`Identified`, never-replaced shape stands
  unless P5 concretizes it stricter.
- No drive before both gates (Pulse fix pushed; operator slot). No retry-until-pass; a pipeline-fault re-fire only
  under the rule the contract states before the first drive.
- The seven spec masters stay read-only; security-plan's BREACH line and the posture contract's arch row are wrap
  amendments.

## Operator directives (take-up, 2026-09-30)
- Take up `:56` in order despite its BLOCKED-ON (founder ruling: nothing skipped or deferred).
- Read Pulse at its committed HEAD only.
- Plan the hermetic CARRYs first, the drive series last, gated on the Pulse fix being committed and pushed AND on the
  operator's slot.
- The series is pre-registered like `:50`: fixed count, rule before the first drive, no retry-until-pass.
- Diff-shaped probes name the chunk base (W182) — `9785405`.
- CI red folded (above), with a non-vacuous witness: reproduce the race, then show it cannot recur.

## CI (Setup 5a)
- `9785405` (the 2026-09-29 hue-shift wrap commit): **`verdict: red` · checks 3/3 · first-fail +592 s** — CI#36635281444
  push completed/failure; failed 1: A11y gate (routine arm · axe · contrast · violation JSON). Folded as item 4.
