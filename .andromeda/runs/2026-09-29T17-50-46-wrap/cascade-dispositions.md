# Cascade dispositions — 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin wrap

**The search:**
- Tool: `cascade.py sweep` over `cascade-patterns.toml`, 19 patterns, baseline `a76420ad` (the pre-CI parent). Every
  control fired; its listing is kept in this run dir as `.sweep-out.txt`.
- Patterns keyed on the retired claims' wording and verbs: the single drive (`never re-driven` · `fired once` ·
  `driven once` · `one drive` · `the one leg/drive`); membership (`UNBACKED_AUTO` · `diagnostic-quality`); the
  capture scrub and scope (`envelope fingerprints elided` · `no report was read` · `sweep to 64` · `workspace
  basename` · `rm-capture.txt` · `` evidence/` tree ``); the single-storm canary (`unique fingerprint-storm` ·
  `CANARY_STORM_COUNT`); the obs claim (`not observed live`); the posture-contract pin (`pulse-real-model-leg-posture`
  · `83d4060` · `NoAttributableIncident`).
- One uncontrolled hand probe: `cadence tick and the next`, the posture contract's retired canary-segment wording,
  run with `grep -rn` over `.andromeda/*.md`, `CLAUDE.md` and `.claude/`. It found 0 hits. That is a statement about
  that one phrasing, not an absence proof: the contract is not a master and corrected itself in place.
- Sections read by window: `architecture.md:53 · :62 · :70 · :93 · :113 · :171 · :182`; `test-plan.md:124 · :155 ·
  :336 · :467`; `security-plan.md:121 · :335`; `obs-plan.md:305`; the leaves named below.

## Rows
**Masters — amended this pass (`new` / `edited`):**
- `architecture.md:70` (one-drive new; unbacked, diag-quality, no-attrib, pin-83d edited) — **amended** (A1+A3).
  - The `83d4060` rows at @c7073 and @c8282 are true claims that share the token: the creation-time mechanism read at
    that HEAD, and the bootstrap-window default. No change.
- `architecture.md:62` (uniq-storm edited) — **amended.** It was a second restatement of the single-storm canary in
  [Read-Back Dependency Posture], not proposed by the detector. It now reads "(three, 90 s apart, under real-model
  L4)". To stay under the §Established Decisions threshold, "`blocked_precondition: null`" was dropped from its
  evidence parenthetical; the leg-verdict pointer keeps the full reading.
- `architecture.md:93` (uniq-storm, storm-count new; pin-83d edited) — **amended** (A4). The `83d4060` rows are the
  `fingerprint_refs` fact. No change.
- `architecture.md:113` (evid-tree standing) — **amended**, folded into this pass. It restated the corpus exception
  as the only persistence and now points at security-plan's recorded 2026-09-29 breach and its route owner (a
  §Standard Contracts line, no registry cap).
- `architecture.md:182` (posture-doc edited) — **amended** (A2).
- `test-plan.md:124 · :336 · :467` (posture-doc new; no-attrib edited) — **amended** (T3; T2+T5; T1).
- `test-plan.md:467` (one-leg, "before the one leg") — **amended** to "before each drive's leg". It was a true
  per-drive claim, re-worded so it cannot read as a single leg ever.
- `test-plan.md:155` (rm-capture edited ×2) — **no change** beyond T4. `runs/live-suite/rm-capture.txt` is still the
  harness's per-invocation capture path.
- `security-plan.md:121` (basename, rm-capture edited) — **amended** (OR-S1 + E2).
- `security-plan.md:335` (no-report, evid-tree edited) — **amended** (OR-S1 duplicate + E1 breach record). "no report
  was read" stays as dated 2026-09-23 history.
- `obs-plan.md:305` — **amended** (O1).

**Masters — standing, no change:**
- `architecture.md:53` (unbacked): generic pin description, no membership.
- `architecture.md:171` (rm-capture): the live-suite harness path, still true.
- `architecture.md:178` (storm-count): 12 per storm still clears the threshold.
- `architecture.md:64 · :66 · :68 · :181 · :199 · :201`, `test-plan.md:76 · :284 · :335`, `obs-plan.md:131 · :315 ·
  :316 · :350` (pin-83d): Pulse facts measured at that HEAD, a true claim sharing the token. None is the posture
  contract's pin.
- `test-plan.md:290 · :365`, `layout-templates.md:188`, `obs-plan.md:357` (unbacked): name the constant or the
  `(N unbacked)` placeholder, never its membership or count.
- `security-plan.md:362` (evid-tree): the NVDA evidence records, unrelated.

**Leaves (re-derived in step 3):**
- `.claude/docs/tests-summary.md:22` (one-drive, one-leg, no-attrib) — **re-derived**: the P-ID set, the series, the
  appended grades, the 2026-09-29 outcome.
- `.claude/docs/tests-summary.md:46` (one-drive: "one driver-free … gate") — a different claim. No change.
- `.claude/docs/tests-summary.md:26`, `CLAUDE.md:35` (unbacked): generic. No change.
- `.claude/docs/security-summary.md:11` (rm-capture) and `:24` (the corpus row) — **re-derived**: the
  `elide_fingerprints` scrub, per-drive captures, the route-owned workspace-key mask, the recorded breach.
- `.claude/rules/security.md:18` (env-fp) and `:10` (evid-tree) — **re-derived**: every fingerprint-shaped token is
  elided; the breach is recorded.
- `.claude/rules/verification-harness.md:19` (rm-capture ×2): the harness path, true. No change.
- `.claude/docs/gotchas.md:23` (pin-83d): a data-dir fact at that HEAD. No change.
- `.claude/docs/commands.md:12` — **re-derived** from the provenance enumeration (§Standard Contracts → commands.md),
  not from a sweep row: the real-model canary's three storms, and one counted drive per invocation.
- Provenance enumeration of the other leaves:
  - CLAUDE.md `GENERATED:setup:*`: the warnings, overview and architecture blocks state five preconditions, the
    `contracts/` roster and the headless-only real-model scenario, all unchanged.
  - `docs/stack.md` and `docs/conventions.md`: no amended fact.
  - `obs-summary.md` and `rules/observability.md`: no real-model statement.
  - `rules/testing.md`: no real-model statement.

**Curation homes (never edited by the cascade → P3):**
- `.claude/rules/verification-harness.md:47` (Session Additions, 2026-06-27 entry; diag-quality @c2330, unbacked
  @c2412): states the four diagnostic-quality capabilities sit in `UNBACKED_AUTO`, now false. It routes to P3 as an
  in-place extension.
  - Its uniq-storm @c848 ("CORRECT: emit a unique fingerprint-storm"), storm-count @c7784 ("`CANARY_STORM_COUNT = 6`
    sat…") and pin-83d @c12089-14176 rows are dated history. No change.
- `.claude/rules/verification-harness.md:55 · :60`, `CLAUDE.md:128`, `.claude/docs/session-learnings.md:258 · :262 ·
  :283`: dated history true at their dates. No change.

**Judgment bases:**
- `.andromeda/playbook.md:246` (posture-doc): a rule's recurrence record citing the contract as a `contracts/`
  member. True. No change.

**Binds:**
- test §3 ↔ obs §3: T4 changed test §3 wording only (no harness command); obs §3 carries no real-model text.
- a11y ↔ obs schema: untouched.
