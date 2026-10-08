# Cascade dispositions — 2026-10-08-capture-canary-pairing-window-corrective

Written after `cascade.py sweep` ran (cascade v1.1; baseline `39e197b1`, the parent of the pre-CI commit) and before
any sidecar entry of this pass.

## The pass

Two amendments, both applied before the sweep:
- `.andromeda/obs-plan.md` §4, Real-model posture: a bracketed dated note after the sixth series' `pipeline-fault`
  clause, before the read-from-Pulse's-log sentence.
- `.andromeda/test-plan.md` §6, Real-model interpretation leg: a dated record after the sixth series' sentence,
  before the closing series-set sentence.

## The search

`cascade-patterns.toml`, six patterns, each derived from the claim both amendments re-scope (the capture pairs only
the lines stamped before the emission instant, so a canary ticked just before it prints `pipeline-fault`): the
claim's name (`pairing window`), its token (`pipeline-fault`), its mechanism's phrasings (`canary:. line per`,
`emission instant`, `prompt assembly`, `third .canary:. line`). Swept: the seven masters, every
`.andromeda/registries/**` file, the three curation homes, the two judgment bases and the leaf bodies. Every
pattern's control fired on the pre-pass masters. Not looked for: the words "selects" and "selected", which are this
pass's own new wording and carry no retired claim.

Counts, as printed: `pairing-window` new 0 · standing 2 · `pipeline-fault` new 0 · standing 2 · `line-per` standing 1
· `emission-inst` new 1 · standing 4 · `prompt-assembly` standing 1 · `third-line` standing 1; leaf 0 · curation 0 ·
base 0 for every pattern.

## The rows (12), each read at its offset with `cascade.py window`

- `.andromeda/test-plan.md:260` `pairing-window` standing edited @c7641 → no change. The sixth series' sentence: the
  d1 and d2 captures "print a third `canary:` line reading `pipeline-fault`, a reading of the capture's own pairing
  window that Pulse's log does not bear out". It is a statement about committed captures, which keep those tokens, so
  it stays true; the plan's approved entry keeps it as written, and this pass's record follows it.
- `.andromeda/test-plan.md:260` `pipeline-fault` standing edited @c7593 → no change; the same sentence.
- `.andromeda/test-plan.md:260` `third-line` standing edited @c7563 → no change; the same sentence.
- `.andromeda/test-plan.md:260` `emission-inst` new @c8060 → this pass's own text (the dated record).
- `.andromeda/obs-plan.md:221` `pairing-window` standing edited @c2298 → no change, re-scoped by the note. The clause
  reads "three where the tick falls just before it, the third then reading `pipeline-fault` because its prompt
  assembly lies outside the capture's pairing window … (d1 and d2 of the 2026-10-07 sixth series …)". The note
  placed directly after its measurement pointer opens "the `pipeline-fault` reading above is the capture as it stood
  through the 2026-10-07 sixth series and stays as that series' measured record", then states what holds from this
  chunk. The clause is kept on the plan's approved entry (the reading stays as the sixth series' record), in the
  form the posture contract itself uses for a dated correction.
- `.andromeda/obs-plan.md:221` `pipeline-fault` standing edited mixed ×2 @c2227,2683 → 2227 is that same clause, no
  change; 2683 is this pass's note.
- `.andromeda/obs-plan.md:221` `prompt-assembly` standing edited @c2255 → no change; the same clause.
- `.andromeda/obs-plan.md:221` `line-per` standing edited @c1897 → no change. "the capture prints one `canary:` line
  per retry-storm digest ticked before the scenario's emission instant" is the SELECTION, which this chunk did not
  move; it stays true.
- `.andromeda/obs-plan.md:221` `emission-inst` standing edited mixed ×2 @c1963,2948 → 1963 is that selection
  sentence, no change; 2948 is this pass's note.
- `.andromeda/architecture.md:62` `emission-inst` standing → no change. The preflight gate's canary round-trip: an
  incident opened "AFTER the storm's emission instant". A different instant (the preflight canary storm's) and a
  different claim (freshness), true and untouched.
- `.andromeda/architecture.md:64` `emission-inst` standing → no change. "Conductor stamps the emission instant from
  `std::time`" — the same freshness gate.
- `.andromeda/architecture.md:93` `emission-inst` standing @c392 → no change. The same canary round-trip, restated
  in the preflight contract's row.

No row in a registry file, a curation home or a judgment base. No `leaf` row.

## The leaves, re-computed from the amended masters

- `.claude/docs/tests-summary.md` (distilled from test-plan): its Fingerprint-storm bullet carries the real-model
  leg's dated history, ending at the 2026-10-01 `skip_reason` clause. Re-derived: one clause added for 2026-10-08
  (the pairing reads across the instant, pinned by the default-suite arms, the sixth series' captures keep their
  tokens, a tick at or after the instant gets no line, unmeasured on a drive).
- `.claude/docs/obs-summary.md` (distilled from obs-plan): read whole at its grain (4623 B); it carries no line on the
  real-model posture or on the capture's `canary:` lines, before or after this pass. No change.
- `.claude/rules/testing.md`, `.claude/rules/observability.md`, `.claude/rules/verification-harness.md`: their
  generated bodies state nothing about the capture's pairing (0 leaf rows for all six patterns). No change. Their
  `## Session Additions` are curation's.
- `CLAUDE.md` `GENERATED:setup:warnings`: no line on the capture's pairing. No change.
- The lateral binds (test-plan §3 ↔ obs-plan §3; the a11y ↔ obs schema): neither §3 nor a schema moved.

## Not this pass's

The "NO Rust reader" claim (architecture's row and key file, `CLAUDE.md`, `conventions.md`) is not amended at this
wrap: it goes to the version-close entry as a `CARRY`, on the operator's direction (`fanout-results.md`, check 6).
`D-arch-registry-size` after Apply: no architecture byte moved in this pass.
