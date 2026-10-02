# Fan-out results — 2026-10-02-captured-fingerprint-values-elided

Seven Explore doc-agents in one parallel batch. Each prompt was the amendment-flow template substituted. Two
substitution notes:

- `{contracts_line}` was dropped: `registry.py contracts` printed `NOT MIGRATED` for all four keyed-contract docs.
- `{detectors_yaml}` was substituted with a pointer to the per-doc split file (`detectors-{doc}.yaml` in this run
  dir), which the agent read whole, rather than a hand transcription. `D-platform-claim` (doc: all seven) is
  appended to every split file.

No return carried HTML entities: every proposal text was read back with no `&lt;`, `&gt;` or `&amp;`, and the
`<fingerprint>` spans arrived intact. No raw twin was warranted, since no `proposals: []` return was changed by
stripping and none failed the parse.

## Verdicts

| Doc | Proposals | Stripped commentary (substance) |
|---|---|---|
| architecture | 0 | Every detector clear. A note outside the detectors' scope says arch `:113`'s residual parenthetical is carried by the expected-amendments floor. Its sweep found no other arch site: `:171`/`:248` are live-suite paths, `:263` is the send-keys ratification, `:70` is a b2 measurement. |
| security-plan | 3 | Subprocess / deps / platform clear. Its sweep found the retired claims only at `:121` and `:336`. `:122` "frozen" is the self-obs journals; `:46` and `:160` name the ratified exception, which is unchanged. |
| design-system | 0 | The `100` hits are "animate-to-100%" and the `residual` hits are the KnownResidual token, both unrelated. |
| layout-templates | 0 | The `100` hits are motion phrases, "step 14" is a sample index, and the `a3f9c1b…` sample is a cli shape. |
| test-plan | 1 | Other detectors clear. `:122` is nextest exit code 100, a false positive. `:155` is a recipe mention. `:307`, `:469`, `:619` and `:621` are other rulings. §7 `:403` is unaffected. |
| obs-plan | 0 | `:305` says "over the scrubbed capture", which stays true. |
| a11y-plan | 0 | — |

## Proposals and dispositions

### T1 — test-plan
- detector: D-tests-derived-count · severity: warning
- section: §6 E2E Test Strategy → Fingerprint-storm → Real-model interpretation leg
- change: retire the frozen-file and d3 "counts exactly … founder ratification pending" clauses. State:
  - both residuals were fixed 2026-10-02 under the founder's ruling;
  - the frozen file was elided in place (a recorded exception) and now equals the graded copy;
  - d3 was re-elided and its pin moved;
  - the keyed elision rule;
  - the committed-capture population arm (the set named, pinned by `COMMITTED_CAPTURES`) with its inverse control;
  - the 2026-10-01 arm asserts zero for every drive.
- basis: `test-plan.md:336`
- **disposition: APPLY.**
  - Check 1: no playbook match (`:25` fails its host-path anchor; `:149` fails "retired by the measurement the
    sentence names"). The recorded direction settles it: the founder's ruling of 2026-10-02 plus the P5-approved
    plan's Expected-amendments entry for test-plan §6.
  - Checks 2-4 clear. Check 4: the sweep matches `master_sites.py` and the cascade sweep below.
  - Check 5: the entry is matched.

### S1 — security-plan (primary)
- detector: D-security-input · severity: escalate
- section: §Input Validation → the real-model capture ingest row
- change: `elide_fingerprints`' definition becomes two rules: keyed (every `fingerprint_hex=` value, any class,
  placeholder and empty kept) and unkeyed (unchanged, an all-digit run kept). Drop the d3-residual pointer. Record
  the harvest's population read (every committed capture under the chunks' `evidence/`, `CARGO_MANIFEST_DIR`-anchored,
  no handle; the population arm plus its inverse control).
- basis: `security-plan.md:121`
- **disposition: APPLY.**
  - Check 1: no rule match (as for T1). The severity is escalate by detector class, but the substance is a
    NARROWING (more is elided). No boundary widens, no input class is admitted and no write is gained. The
    recorded direction settles it: the founder's ruling plus the plan's Expected-amendments entry for
    §Input Validation.
  - Check 4: the agent's sweep is `:121`/`:336` only, consistent with `master_sites.py`.
  - The applied text is RE-DERIVED: the population is named as the set, with its count pinned in code, never a fresh
    literal in prose.

### S2 — security-plan (dependent-of D-security-input)
- section: §Security Anti-Patterns → Data Protection (the exception clause's "an all-digit run … passes")
- basis: `security-plan.md:336`
- **disposition: APPLY**, atomically with S1.

### S3 — security-plan (dependent-of D-security-input)
- section: §Security Anti-Patterns → Data Protection (both residual statements)
- change: both residuals FIXED 2026-10-02 under the founder's ruling, never ratified. State:
  - the frozen file was elided in place (a recorded exception to "frozen evidence is never edited") and is
    byte-identical to the graded copy;
  - d3 was re-elided and its pin moved;
  - the 2026-10-01 arm asserts zero;
  - the population arm holds zero across the committed captures;
  - no committed quote of either value remains.
- basis: `security-plan.md:336`
- **disposition: APPLY**, atomically with S1.

### A1 — architecture (orchestrator-raised, Validate check 5)
- section: §Standard Contracts (Readiness gate, "Corpus access"), the tail of the `corpus.db` parenthetical
- change: retire "the frozen 2026-09-22 file keeps its one `fingerprint_hex` prefix as a stated residual; a second
  — … — is overseer-ruled, founder ratification pending". Replace it with both fixed 2026-10-02 under the founder's
  ruling (the frozen file elided in place), in lockstep with security-plan, byte-neutral or negative.
- basis: `architecture.md:113` (offset ~4280)
- **disposition: APPLY**, as routine. The report substantiates it (Counts, Expected amendments), and the plan lists
  it. No arch detector covers the class, which the arch agent's note confirms.

### Expected-amendments reconciliation (check 5)

| Plan entry | Matched by |
|---|---|
| security Data Protection | S2 + S3 |
| security Input Validation | S1 |
| security Threat Model / other sites | swept — none beyond `:121` / `:336` (both agents and `master_sites.py`); no proposal |
| arch §Standard Contracts | A1 |
| test-plan §6 | T1 |
| test-plan §7 | superseded (no §7 site states the prefix; `:336` carries it) |
| leaves | cascade step 3 |

### Disproved claims (check 6)
None in the report.

## Escalations
None open. The escalate-severity S group is applied on the recorded direction (validate check 1's no-match branch),
and its rule is proposed at the wrap card.
