# P2 cascade step-2 sweep — 2026-09-22-interpretation-proven-live

Run AFTER every body amendment of the pass was applied (step 1), over the seven masters, CLAUDE.md,
`.claude/docs/*`, `.claude/rules/*`, and the two judgment bases (`playbook.md` · `drift-base.md`), with
`sweep.py` (per-hit offset windows, so hits on multi-KB lines were read by offset, never from a clipped view).
The pattern set was derived from EVERY amendment of the pass before the first grep; each group keys on the
retired claim's wording AND its mechanism's phrasings. Every hit is dispositioned: **amended** (this pass) ·
**re-derived** (a leaf, cascade step 3) · **curation** (a preserve-verbatim home, routed to P3) · **no change**
(with why). Line numbers are post-edit.

## G1 — "one scenario per P-ID" + the P-ID run selector
Pattern `one[- ]per[- ](Pulse )?P-ID|one scenario per|scenario\|P-ID|name\|P-ID|by name or P-ID|run <P-ID>|run P-0[0-9]+|per-P-ID scenarios` → **15 hits**.
- test-plan `:151` ×3 — **amended** (T10, the P-ID determinacy sentence covers `run` and `SCENARIO=`).
- test-plan `:64` "per-P-ID scenarios" — **no change**: names the scenarios by their P-ID keying, not a cardinality.
- test-plan `:356` `conductor run P-032` — **no change**: P-032 is named by ONE scenario (census below), so the selector is determinate.
- CLAUDE.md `:13` · commands.md `:12`/`:18` · testing.md `:32`/`:38` · verification-harness.md `:19` ×2 — **re-derived**.
- testing.md `:59` (Session Additions) "author one scenario per value" — **no change**: it argues FOR several scenarios.
- verification-harness.md `:49` ×2 (Session Additions) — **no change**: the `SCENARIO=` firing-form prefix, not determinacy.
- architecture `:239` (dir tree) — **amended** before the sweep (no longer matches).
Companion `per P-ID|per-P-ID` → **49 hits**: every one names a P-ID-keyed ROW, lamp, verdict line or emit cohort
(design-system, layout-templates, a11y-plan, test-plan, obs-plan lamp/row/cohort prose; session-learnings;
frontend.md; testing.md `:25`), never a scenario cardinality — **no change**; except conventions.md `:9`
"one config per P-ID" — **re-derived**; architecture `:52` "per-P-ID scenario files" — **no change** (descriptive);
playbook `:51` — **no change** (judgment base, descriptive).
Census basis (this wrap, `scenarios/*.toml` `p_ids`): P-017 ×2 · P-018 ×2 · P-019 ×3 · P-020 ×3 · P-021 ×2 ·
P-022 ×2 · P-060 ×3; P-018 is the only one this chunk moved (1 → 2).

## G2 — `corpus.db` "encrypted (P-049)"
Pattern `encrypted|decrypt` → **10 hits**, every one stating plaintext / "NOT encrypted-at-rest" / an unencrypted
`runs.db` (architecture `:113` ×2, security-plan `:38`, `:46`, `:153`, `:261`, `:336`, `:381`, session-learnings
`:229` ×2) — **no change**. test-plan `:44` / `:309` were **amended** before the sweep (operator: "Fix now").

## G3 — the preconditions probe's grading (posture)
Pattern `affirmatively declares|affirmative declaration|only kind that can block|ONE OF TWO|SIX terms|six terms|shell-declaration (set|proxy|proxies|term)|declares\(\)|keep(s|ing)? the truthy` → **5 hits**:
architecture `:115` "truthy-only `declares()`" — **no change** (the dated pre-fix state the passage supersedes);
architecture `:178` (dated "carried SIX terms as of 2026-09-03"), `:197`, security-plan `:380`, layout-templates
`:187` — **amended**.
Mechanism pattern `handles-declared|flag_declared|handle_declared|by an affirmative|"true"`/`"1"|true/1` → **33 hits**:
architecture `:115` ×6, `:197` ×4; security-plan `:114` ×7, `:122`, `:380` ×3; layout-templates `:187` ×3 — **amended**;
architecture `:196`, security-plan `:119`, test-plan `:87` (`CONDUCTOR_A11Y_STRICT`) — **no change**, another handle;
test-plan `:63` ×2 and `:145` — **no change**: `boot`, which keeps the flagless deterministic probe;
test-plan `:611` ×3 — **no change**: the mutation roster's dated history.

## G4 — both-surface parity
Pattern `call identically|identical envelope|same scenario\+seed|every catalog scenario|both-surface parity|Both-surface parity|control panel AND headless` → **23 hits**:
- architecture `:4`, test-plan `:81` ×4, `:94` ×2 — **amended** (the Creator Brief quotes inside stay as the SOURCE of the Must-Work).
- test-plan `:375` (§6 signal) — **amended**; test-plan `:369` ×2 / obs-plan `:136`, `:361` — path names, **no change**.
- test-plan `:71`, `:200`, `:397` "same scenario+seed ⇒ same stream shape" — **no change**: EMISSION determinism, which the real-model scenario keeps.
- test-plan `:250` "every catalog scenario carries a P-ID" — **no change** (true).
- test-plan `:289` ×2 — **no change**: §5's shipped `cross_surface_parity.rs` leg, describing what that test asserts over its own scenario, no universal.
- test-plan `:374` (§6 steps) — **no change**: the amended signal line governs the pair.
- obs-plan `:49`, `:146`, `:590` — **no change**: parity's MECHANISM (envelope comparison, not trace correlation); the SCOPE lives at CP7 `:369`, **amended**; obs-plan `:370` sits inside that scoped block.
- architecture `:177` "every catalog scenario passes the asserted terms" — **no change**: the load envelope (green this chunk).

## G5 — the payload-fidelity universal (A17–A19, operator: "Amend now, as measured")
Pre-edit `payload-invariant|payload-INVARIANT|no read-back field varies|NO read-back field varies|reaches no read-back surface|no read-back surface carr|never an identity carrier|Payload fidelity is unattainable|out of reach in this mode` → **22 hits**:
architecture `:64` ×3, `:66`, `:93` ×3; test-plan `:76` ×2, `:335` ×2; obs-plan `:131`, `:314` ×2, `:315` — **amended**;
tests-summary.md `:22` ×2 — **re-derived**; CLAUDE.md `:128` (`USER:session-learnings`), session-learnings `:228` ×2,
verification-harness.md `:47` ×2 (Session Additions) — **curation** (P3).
Post-edit, widened to `…|fixture constants|evidence_refs|fingerprint_refs|payload fidelity|payload identity` → **24 hits**, all
in amended text except test-plan `:284` "never payload fidelity" (**no change**: names what the lifecycle leg PROVED)
and `:323` "renders fixture constants" (**no change**: the degraded `retrieve_report` markdown, another axis).

## G6 — ~110 s / formation / single-storm / "2 of 36"
Pattern `~ ?110|110 ?s\b|110s|incident-formation|formation figure|real-model formation|single-storm|single storm|hypothesis observable|multi-digest|2 of 36|36 scenarios|36 committed|of 36\b` → **11 hits**:
architecture `:70` ×2, `:182` ×5; test-plan `:336`, `:466` — all **amended** text; verification-harness.md `:60` ×2
(Session Additions, "real-model formation (~110s measured)") — **curation** (P3, the plan's named Tier-2 correction).
"2 of 36" → 0 (architecture `:135` amended; control: the report measured 1 hit there pre-edit).

## G7 — the corpus-content ban (S9, operator: "Ratify, scoped")
Pattern `never persist|persists/exfiltrat|persists or exfiltrat|exfiltrat|never persisted` → **8 hits**:
security-plan `:46` ×2, `:157` ×2, `:335` ×2 — **amended**; architecture `:113` ×2 — **amended** (cascade fold, the exception pointer).

## G8 — `CONDUCTOR_RUNS_DIR` test-binary readers (S8, operator: "Record all + route")
Pattern `journal_conformance\.rs\` reads|TEST-binary reader|test-binary reader|test-only reader|Rust test-binary reader|no longer the only` → **10 hits**:
security-plan `:115` ×2, `:121` — **amended**; `:221` ×3 and `:325` ×4 — **amended** (cascade folds).

## G9 — `--live` shapes and the probabilistic policy
Pattern `future leg|live-leg set|in one shape|one shape|sanctioned live|model-interpretive|never hard-failed|never Pass|ONE invocation|--live\` (stage|suite)|run --live\`` → **22 hits**:
architecture `:71` ×3, `:171`, `:242`; security-plan `:380`; layout-templates `:190`; test-plan `:124`, `:466` ×3 — **amended**;
architecture `:61` (timings route to calibration — true), `:70` (the ManualCheck route — true), `:201` (the `run --live` banner — unaffected);
security-plan `:327` (refuses before any leg spawns a sidecar — true of both shapes); design-system `:63`
(amber = `CalibrationRegion` — lamps unchanged, the Pass arm is test-tier); test-plan `:64`, `:152` ×2 (the Creator
Brief's "never hard-failed" — still true); test-plan `:545` (the bare suite's 150 s window), `:569` (never a CI gate —
true of both shapes) — **no change**.

## G10 — `ANDROMEDA_PULSE_DATA_DIR`'s value
Pattern `never becomes a path|carries no value|no value (ever )?becomes|value never becomes|never reads (its|the) value|only via \`\.env|strictly via the \`\.env` → **5 hits**:
security-plan `:114` — **amended** (cascade: scoped to its read); `:122` — **amended**; `:119` (`CONDUCTOR_A11Y_STRICT`),
`:326` (the argv/shell ban — the capture uses neither), test-plan `:89` (negative test (a)) — **no change**.

## Other
- design-system: `--live` 0 hits; `posture|real-model|preconditions` 1 hit, `:312` (the `[PRECONDITION]` caption, reused unchanged) — **no change**. a11y-plan: 0 hits for both.
- obs-plan `§3` (the test-plan §3 ↔ obs-plan §3 bind): `--live|live-suite|preconditions --` 0 hits; the envelope, `status` read and JSONL shapes are unchanged, so the bind holds.
- obs-plan `§5`/`§10` (the plan's "pickup figure" entry): `pickup|formation` 0 hits and no figure was measured — nothing to record.
- Judgment bases: G1–G10 → 0 hits in `playbook.md` / `drift-base.md` bar playbook `:51` (G1 companion, descriptive).
- Intra-line duplicate re-read of every amended line: no amended line states a retired mechanism twice; architecture `:64` quotes the retired universal once, marked RETIRED.
