# Fan-out results — 2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09

Seven doc-agents, one batch. Detector bindings sent: architecture 5 · security-plan 4 · design-system 3 ·
layout-templates 3 · test-plan 5 · obs-plan 5 · a11y-plan 3 = 28, equal to the `doc:` names over `drift-base.md`
(21 single-doc bindings + `D-platform-claim` × 7). Every return passed the entity probe (no `&lt;` / `&gt;` / `&amp;`
in any YAML value). Dispositions are Validate's; none escalated.

## Verdict lines

- **design-system** — `proposals: []`. Stripped: a commented per-detector basis (no UI rendered; no moved count
  stated in the doc; no platform verdict stated). Raw twin: `.raw-fanout-design-system.md`.
- **layout-templates** — `proposals: []`. Stripped: a commented per-detector basis (no new surface; the doc's `14`
  tokens are a sample step index, not the capture count; no platform verdict stated). Raw twin:
  `.raw-fanout-layout-templates.md`.
- **a11y-plan** — `proposals: []`. Stripped: a commented per-detector basis (no interactive element; neither schema
  moved; the CI attempt-1 red was an artifact-upload reset with the arm passing, which changes no a11y-plan claim).
  Raw twin: `.raw-fanout-a11y-plan.md`.
- **obs-plan** — 1 proposal. Stripped: a commented header stating that no invariant is violated and listing four
  non-hits.
- **test-plan** — 1 proposal. Stripped: trailing comments (no other occurrence; four detectors with no hit; `:391`
  left alone as baseline prose).
- **security-plan** — 3 proposals. Stripped: a commented header (no unvalidated boundary; the three are
  record-completions, add-only).
- **architecture** — 6 proposals. Stripped: a commented header (per-detector outcomes, a byte forecast, and one
  observation outside the detectors — below).

## architecture (6)

- **A1** · D-arch-resources · warning · §Occupied Resources — On-disk artifacts, the posture-contract entry ·
  the series enumeration gains `2026-10-06` (+12 B).
  **apply** — check 5 (expected amendment 1 names the change); the report carries the count 3 → 4.
- **A2** · D-arch-resources · warning · same entry · the pin parenthetical `(re-pinned fcc31b2 2026-09-30, a2addb3
  2026-10-01)` becomes `(re-pinned per series, 5f77859 at 2026-10-06)` (−7 B); the earlier pins live in the sidecar.
  **apply** — check 5 (expected amendment 1) and `D-arch-registry-size`'s own remedy (this wrap's history to the
  sidecar). Verified first: the sidecar already holds both earlier pins (`fcc31b2` ×2, `a2addb3` ×4).
- **A3** · D-arch-decisions · warning · §Established Decisions [Read-Back Dependency Posture], the 2026-06-27 caveat ·
  the model literal leaves the L4 inference sentence. **apply** — check 5 (expected amendment 2) and check 6
  (disproved claim 2).
- **A4** · D-arch-decisions · warning · same caveat, `the 3B LLM` → `the L4 model` · dependent-of A3. **apply** with
  its primary.
- **A5** · D-arch-decisions · warning · §Standard Contracts — Readiness gate, the 2026-06-27 NOTE · `L4 Llama-3.2-3B`
  → `L4 inference` · dependent-of A3. **apply** with its primary.
- **A6** · D-arch-decisions · warning · §Occupied Resources — Environment variables, the
  `ANDROMEDA_PULSE_L4_DETERMINISTIC` entry · `the Llama-3.2-3B inference` → `the real-model inference` ·
  dependent-of A3. **apply** with its primary.
- Check 4 over the retired claim: every `Llama` / `3B` / `llama` occurrence in architecture was read by offset
  (`:70` @2105, @2115, @2125, @2208; `:93` @2346, @2356; `:200` @94, @104) — eight positions, all inside the four
  amended phrases; `llama.cpp` (the runtime's name) stays. After apply the sweep reads 0 master rows for both
  model patterns.
- `D-arch-registry-size`, run after apply: §Established Decisions 37980 B, §Occupied Resources 38086 B, threshold
  38115 B — `registries: within target`.
- **Outside the detectors (no proposal, not applied):** the agent notes that the posture-contract entry says the
  contract "NAMES only already-registered environment handles" while `ANDROMEDA_PULSE_MODEL_PATH` and
  `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` are absent by name from architecture. It predates this chunk (both handles
  stand in the contract since its 2026-09-30 section) and the overseer ruled at this chunk's plan review that no
  amendment is raised on that clause here. Surfaced to the operator in the wrap's console report and handoff.

## security-plan (3)

- **S1** · D-security-input · warning · §Input Validation → Real-model capture ingest · state `mask_host_paths`'
  match set with the two temp roots. **reject as a proposal** — its `basis` is a source file location and its change
  line carries match-set detail the report does not (the re-derivation tell). The report-carried fact (named roots
  3 → 5, same semantics, two arms red before green, no pin moved) is **raised by the orchestrator under check 5**
  (expected amendment 5) — routine, **applied** as SEC-1.
- **S2** · D-security-input · warning · same row · the fourth series joins the row's dated workspace-key
  measurements · dependent-of S1. **reject as a proposal** (its `basis` is a capture file; rejected with its
  primary). The report-carried facts (workspace-detector byte-identical to `5f77859`; leaf 0 across three captures;
  POSIX suffix `<redacted>`) are raised under check 5 — routine, **applied** as SEC-2.
- **S3** · D-security-input · warning · §Security Anti-Patterns → Data Protection · the exception's per-series
  inventory gains the 2026-10-06 series · dependent-of S1. **reject as a proposal** (capture-file basis; a clause
  about `## Previously Seen` titles the report does not carry). The inventory extension itself is raised under
  check 5 — routine, **applied** as SEC-3: three captures, one report body each, inside the exception's ratified
  terms. No boundary widens: the exception's scope is unchanged and the mask grew.

## test-plan (1)

- **T1** · D-tests-derived-count · warning · §6 Real-model interpretation leg · append the 2026-10-06 series'
  dated verdict after the per-run-identity sentence, and name the series set as the posture contract's dated
  sections. **apply** — check 5 (expected amendment 6 names the change). Re-derived from the report, not pasted.
  The amendment's second half (the temp-root arms) needs no edit: the bullet names the arm SET (`host-path-mask
  arms`, read at offset 1297), never the roots.

## obs-plan (1)

- **O1** · D-obs-instrumentation · warning · §4 Real-model posture · add a dated live observation at `5f77859`
  and the three-storm / two-`canary:`-line fact. **apply** — check 5 (expected amendment 7 names the change).
  Add-only; the mechanism clause carries `as measured at`.

## Checks 2, 3, 5, 6

- Check 2: no two proposals edit one section in opposing directions.
- Check 3: every deviation in the report carries the overseer's recorded word or a stated reason; the scope
  record's one line (`scenario_catalog.rs`, in-intent, serves step 14) holds — a comment mark of the class the
  step exists for.
- Check 5: expected amendments 1 → A1, A2 · 2 → A3-A6 · 3 → none, as planned · 4 → P3 curation · 5 → SEC-1..3 ·
  6 → T1 · 7 → O1.
- Check 6: disproved claim 1 → P7.3 (the matrix acceptance, a premise correction) · 2 → A3-A6 · 3 → P3 curation
  (a rule file) · 4 → P5 (the tenth walker, a CARRY); no master states the candidate count.
