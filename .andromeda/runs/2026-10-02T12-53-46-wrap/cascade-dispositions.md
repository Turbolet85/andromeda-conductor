# Cascade dispositions — 2026-10-02-p-075-assert-round-against-pulse

**Search:**
- Patterns: `cascade-patterns.toml`, eight of them, run by `cascade.py sweep` (`sweep-out.txt`, the second run).
- They cover what each retired claim SAYS:
  - the span pair under `live-suite/` (`ls-span`, `span-pair`);
  - a second operator-local writer there (`second-writer`, `move-route`);
  - `retrieve_report` degraded mode-wide (`perm-degraded`: "permanent(ly) … degraded", "always-degraded",
    "degraded … in this mode", "degraded … every read-back under deterministic");
  - no check computing the emitted-vs-read-back match (`no-check`);
  - the replaced 2026-09-10 posture citation and its hex value (`leg1-cite`, `hex-0bdd`).
- Every control fired on the pre-pass masters at baseline `e1092ce5`.

**The first run under-ran.** Its `perm-degraded` regex keyed on `permanently` and missed
`.claude/docs/tests-summary.md:22`, which reads "permanent `degraded_mode`". A leaf probe by hand found it. The
pattern was widened to `permanent(ly)?` and the sweep re-run. The rows below are the second run's.

**Read by hand beside the sweep** (masters, every hit windowed): `degraded|always-degraded|is_none()` over all seven
masters is dispositioned in `fanout-results.md` TR3. The leaves were also grepped for
`span_landing|span-landing|live-suite|p075|degraded_mode|permanently|fingerprint_refs|no shipped check|live-pulse|delegated`
(tests / security / obs summaries, gotchas, commands, rules testing / observability / security / verification-harness,
and CLAUDE.md).

## Rows
- `.andromeda/architecture.md:172` span-pair `new` → AMENDED. This is A1's own new entry, `runs/span-landing/`.
- `.andromeda/architecture.md:249` span-pair `standing edited` → AMENDED. This is A3's new tree line, `span-landing/`.
- `.andromeda/security-plan.md:122` span-pair `standing edited` → AMENDED. SR1 now names `runs/span-landing/`.
- `.claude/docs/tests-summary.md:22` perm-degraded `leaf` → RE-DERIVED. "(permanent `degraded_mode`)" is replaced by
  the per-read-back statement plus the 2026-10-02 membership check, following test-plan :335.
- `CLAUDE.md:128` perm-degraded `curation` @c17558 → NO CHANGE. It is the 2026-08-09 learning's dated 2026-09-06
  extension, which quotes a directive's claim "(0 hits over the seven)" as a past measurement of a pattern's reach.
  It asserts nothing about Pulse's current mode. It is preserve-verbatim and not stale.
- Every other pattern: 0 rows, with its control fired. No master, leaf, curation home or judgment base still states
  the span pair under `live-suite/`, a second writer there, "no shipped check computes", or the replaced citation and
  hex value.

## Leaves re-derived (step 3)
- **Set:** the table's rows for architecture / security-plan / test-plan / obs-plan, plus provenance headers
  (`conventions.md` ← arch §Conventions, which was not amended; `stack.md` ← arch §Stack, which was not amended).
- `.claude/docs/tests-summary.md` → re-derived at :22 (above) and at :46. The gated SET gains `p075_round_live.rs`,
  and `span_landing_live.rs` gains its `runs/span-landing/` pair and stale-pair refusal, from test-plan :124 / :467.
  The `mark_incident_resolved` re-grade comes from test-plan :284.
- `.claude/docs/security-summary.md:11` → re-derived. The span-landing ingest gains its `runs/span-landing/` journals
  and the stale-pair refusal, from security-plan :122.
- `.claude/rules/security.md:18`, in the body and not `## Session Additions` → re-derived. Same fact, same source.
- `.claude/docs/obs-summary.md` and `.claude/rules/observability.md` → NO CHANGE. Neither states the fingerprint-storm
  check-coverage claim (O1) nor the delegated-timing grades (OR1); grep on `no shipped check|delegated|P-025` → 0.
- `.claude/rules/testing.md` and `.claude/rules/verification-harness.md` → NO CHANGE. Neither enumerates the
  `live-pulse` gated set, the span-landing path or the degraded claim; the hits are dated Session-Addition
  learnings.
- CLAUDE.md `GENERATED:setup:*` → RECOMPUTED, NO CHANGE.
  - The overview's `runs/` line ("per-run JSONL journal + Markdown report + `runs.db` SQLite index") never listed
    `live-suite/`.
  - The warnings, modules, pointer table and architecture blocks state none of the amended facts.
- `.claude/docs/gotchas.md` / `commands.md` / `stack.md` / `conventions.md` → NO CHANGE. Their sources' sections
  (§Cross-cutting, §Standard Contracts, §Stack, §Conventions) were not amended, and no amended fact appears in them.
- Lateral binds: test-plan §3 and obs-plan §3 were not amended, and neither were the a11y and obs schemas, so both
  binds hold.
- Judgment bases (`playbook.md`, `drift-base.md`): 0 rows.
