# Fan-out results — 2026-10-01-per-run-span-identity-in-the-real-model-harness

There were seven Explore doc-agents in one parallel batch. Every prompt was substituted and self-checked (0
unsubstituted placeholders) and sent verbatim. No doc was migrated to keyed contracts (`registry.py contracts` → `NOT
MIGRATED` for arch / tests / obs / a11y; `n/a` for the other three), so `{contracts_line}` was dropped. The cross-doc
`D-platform-claim` was given to all seven.

For every return, stripping removed only the trailing `#` comment lines, which were the agent's own per-detector
reasoning. The substance of each is summarised in its verdict line. The returns carried no `<` `>` `&` entities, so the
entity probe read 0 for all seven. No raw twin was warranted: the four `[]` returns were unchanged by stripping beyond
those comments, and every list parsed.

## Verdicts

| Doc | Proposals | Stripped substance |
|---|---|---|
| architecture | 4 | — |
| security-plan | 5 | — |
| design-system | 0 | no UI; counts are not baked in any palette or token row; no platform verdict retired |
| layout-templates | 0 | no surface; no caption bakes 15 / 1153; "headless" sentences are prompt policy, not a verdict |
| test-plan | 7 | framework / obs-harness / derived-count / platform: no hit. A note, not proposed: §12 Decisions Log `:616` cites the old two-arg `Dispatcher::connect` |
| obs-plan | 0 | no new tracing; the re-export is the PRODUCT proto type, not a self-obs SDK; no CI step; `:32`/`:39` ordering claims still hold |
| a11y-plan | 0 | no interactive element; no schema change. The agent said "quiet window" never appears in a11y-plan; that is FALSE (`grep -noF` → `:268`). The site is read at the cascade sweep |

## Proposals and dispositions

### architecture
- **A1** D-arch-decisions (warning) · §Cross-cutting Patterns → Determinism discipline. Seed-purity is scoped to stream
  shape + content; production span identity = f(seed, per-execution `std::time` salt) via `rekey_trace_identity`; the
  unsalted tier is pinned by the goldens.
  - **APPLY:** routine, playbook `:308` (Accurate this-chunk addition).
  - It is Expected amendment 1. Check 5 ✓.
  - Boundary widening (`:124`) was answered NOT at P4 by the overseer.
- **A2** D-arch-decisions (warning, dependent-of A1) · §Occupied Resources → `contracts/pulse-run-contract.toml` identity
  passage. Re-cite `schema.rs:36` at `a2addb3`; a collision rejects the WHOLE batch at flush (not log-and-skip one row);
  scenario drives carry per-execution identity.
  - **APPLY:** routine, `:308` plus the report's Cross-project bullet (Pulse source re-read at `a2addb3`).
  - It is Expected amendment 1, second site.
  - It applies as a group with A1.
- **A3** D-arch-collision (escalate) · §Occupied Resources → `runs/live-suite/{leg}.jsonl`. The operator pass's manual
  `cp` writes `span-a.jsonl` / `span-b.jsonl` into a harness-owned directory, which is a second writer class.
  - **ESCALATE:** the detector severity is escalate, and no playbook rule matches a second writer of a registered
    artifact path.
- **A4** D-arch-collision (escalate, dependent-of A3) · §Infrastructure Patterns → Directory structure `runs/live-suite/`
  line.
  - **ESCALATE:** it applies as a group with A3.

### security-plan
- **S1** D-security-input (escalate) · §Input Validation → `CONDUCTOR_RUNS_DIR` test-binary reader set: THREE becomes
  FOUR (+`span_landing_live`, through `capture_paths::runs_dir_from` → `resolve_under`).
  - **APPLY:** routine, `:308`. The named reader is this chunk's, its validation is present (report Coverage line), and
    the per-reader record invariant survives.
  - `:124` Boundary widening was CONSIDERED and its precondition FAILED. No new input class: the same handle, the same
    shared guard module, the same Pulse-log class `real_model_live` already ingests. No new crossing. No write: the
    ingest commits nothing beyond integers.
  - Re-derivation tell: S1's `basis` cites source lines the report does not carry. Its FACTS are in the report's Coverage
    line, so the applied text is re-derived from the report alone and the cited line numbers are not used.
- **S2** (dependent-of S1) · §Security Anti-Patterns → Input: the live-pulse capture enumeration +1, "all three" becomes
  "all four".
  - **APPLY:** as a group with S1.
- **S3** D-security-input (escalate) · §Input Validation → Pulse-log ingest: `span_landing_live` recorded as a second
  guarded test-binary reader of Pulse's `agent-latest` logs (canonicalize + is-dir, bounded per-line decode, `e.kind()`
  only, integers-only output).
  - **APPLY:** routine, `:308`, on the same basis as S1.
- **S4** (dependent-of S3) · §Input Validation → `ANDROMEDA_PULSE_DATA_DIR` spawn row: test-binary VALUE readers
  pluralised; the witness spawns no sidecar.
  - **APPLY.**
- **S5** (dependent-of S3) · §Input Validation → read-set row pointer pluralised.
  - **APPLY.**

### test-plan
- **T1** D-tests-coverage · §2 operator-local real-wall-clock list + `span_landing_live`.
  - **APPLY:** `:308`; Expected amendment 2. Check 5 ✓.
- **T2** (dependent-of T1) · §9 the `live-pulse` gated-target SET + `span_landing_live.rs`.
  - **APPLY.**
- **T3** (dependent-of T1) · §11 E2E the SUT-side quiet-window class + the 180 s inter-drive window.
  - **APPLY.**
- **T4** D-tests-coverage · §6 real-model leg: the d2 replay is repaid by per-execution identity, held by the two-drive
  test + the witness.
  - **APPLY:** `:308`; Expected amendment 2 (`:336`).
- **T5** D-tests-coverage · §4 conductor-emit unit coverage names `rekey_trace_identity` (5 tests).
  - **APPLY:** `:308`; Expected amendment 3.
- **T6** D-tests-coverage · §7 Golden artifacts row: the `dispatch_wire__*` goldens pin the UNSALTED tier.
  - **APPLY:** `:308`; the `dispatch_wire` site of Expected amendment 3.
- **T7** D-tests-coverage · §8 Random sources: the identity salt is a DI-injected seeded source (tests None / fixed;
  production `emitted_ms`); the tier placement is stated.
  - **APPLY:** `:308`; Expected amendment 3.
- The note on §12 `:616` (the old two-arg `connect`) is NOT applied. A Decisions Log is history and takes no edit; it
  records the signature as it stood then.

## Validate summary
- **Check 1 (playbook):** 11 apply (`:308`), 2 escalate (A3 + A4 as one group).
- **Check 2 (cross-contradiction):** none. T1 / T2 name `runs/live-suite/span-{a,b}.jsonl`, the same files A3 asks
  about. Their wording follows A3's resolution.
- **Check 3 (intent):** consistent. The scope record is empty (`gate.py scope` clean).
- **Check 4 (absence):** no absence claim among the applied proposals. The a11y "quiet window" absence claim is
  falsified above and does not drive any edit.
- **Check 5 (expected amendments):** all three entries are matched (A1+A2 · T1+T4 · T5+T6+T7).
- **Check 6 (disproved claims):** both report entries are plan-only (the step 5 (ii) wording; the Δ prediction). They
  are DISPOSED as no master edit, recorded in the report. No master states either.

## Post-escalation record
- A3 + A4: RESOLVED by the overseer at this wrap. Register the second writer truthfully now (applied), and carry the move
  as a CARRY on the P-075 assert round entry (P5). That overrides the directive's route-unchanged for that one CARRY only.
- S6 (orchestrator-raised at the cascade sweep, dependent-of S1): `security-plan.md:222` restated the reader enumeration
  ("two … captures" / "Those three"). Folded as routine.
- D-arch-registry-size (tooling, after Apply): §Occupied Resources first read 39137 B, OVER. The amended entries were
  reduced to bare current truth (A2's mechanism detail moved to the sidecar) and the `runs/live-suite` entry's wording
  was compressed with no fact dropped. Final reading: 38112 B within the 38115 B target; §Established Decisions 38111 B.
- Totals: 18 amendments applied (arch 4 · security-plan 6 · test-plan 7, plus the A2 detail moved to the sidecar,
  counted with A2), 1 escalation resolved, 0 open.
