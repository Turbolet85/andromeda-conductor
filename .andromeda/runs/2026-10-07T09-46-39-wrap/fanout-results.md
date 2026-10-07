# Fan-out results — 2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09

Seven doc-agents, one batch, the prompt of `amendment-flow.md` sent verbatim. Detector ids over the seven prompts:
28 (architecture 5 · security-plan 4 · design-system 3 · layout-templates 3 · test-plan 5 · obs-plan 5 · a11y-plan
3), equal to the `doc:` names over the drift-base. Every return passed the entity probe (0 HTML entities). Each
return carried `#` comment lines beside its YAML naming why the other detectors had no hit; stripping removed those
and nothing else.

## Verdicts

- **design-system** — `proposals: []`. Stripped: its no-hit basis (no UI element; every moved count 0 hits; no
  platform verdict retired).
- **layout-templates** — `proposals: []`. Stripped: its no-hit basis (no surface; every moved count 0 hits; the
  `--live real-model` description at `:196` consistent with the drives).
- **a11y-plan** — `proposals: []`. Stripped: its no-hit basis, and one observation it marked as not a proposal: the
  reproduced envelope schema types `verdict` with no null arm while the drives' envelopes read `verdict` null — a
  baseline wording it attributes to obs-plan §6, unchanged by this chunk. Not acted on here: no report fact moved
  it. Named in the wrap's console report.
- **obs-plan** — 1 proposal.
- **test-plan** — 1 proposal.
- **security-plan** — 4 proposals.
- **architecture** — 5 proposals.

## Proposals and dispositions

### obs-plan
1. `D-obs-instrumentation` · warning · §4 Real-model posture (`:221`) — an add-only dated observation at Pulse
   `f70be92` (`model_identity` `gemma-4-E4B-it-Q4_K_M`, `prompt_version` `v2.6`; three envelopes `ManualCheck`,
   `verdict` null, eleven keys; no span, log line or field added), after the 2026-10-06 record.
   **Disposition: apply.** Check 1: no playbook rule matches a dated-record extension; settled by the plan's
   P5-approved Expected amendment 5, which names the change (the overseer, 2026-10-07, delegate for the founder,
   inputs#I17). Checks 2-4: no opposing proposal; inside the intent; nothing claimed absent. The applied text is
   re-derived from the report.

### test-plan
1. `D-tests-derived-count` · warning · §6 Real-model interpretation leg (`:260`) — the 2026-10-07 series' dated
   verdict after the 2026-10-06 record and before the closing "series set" sentence; no count introduced.
   **Disposition: apply.** Check 1: no rule; settled by Expected amendment 4. The applied text is re-derived.

### security-plan
All four cite capture line numbers or a Pulse `git diff` as basis, locations the report does not carry (the
re-derivation tell). **Each is REJECTED as proposed and RAISED by the orchestrator under check 5** (Expected
amendment 3, which names all four changes), routine, the report substantiating each fact:
1. §Input Validation, the capture ingest row (`:121`) — the fall-back-to-data-dir series list gains the 2026-10-07
   series. Report: Counts / qualifiers moved line 3; key rendering `verbatim` on all three drives. **Raised, applied.**
2. Same row — the leaf-rendering measurement gains the 2026-10-07 series (leaf 0 times; each `## Previously Seen`
   suffix prints `<redacted>`). Report: Coverage; Outcome. `dependent-of` 1. **Raised, applied.**
3. Same row — the workspace-key derivation's provenance gains "and at `f70be92`". Report: Expected amendment 3,
   basis inputs#I15 (`committed@f70be92c`, unchanged). **Raised, applied.**
4. §Security Anti-Patterns → Data Protection (`:338`) — the exception's per-series inventory gains the 2026-10-07
   series' three captures, one report body each. Report: Counts / qualifiers moved line 4; re-measured by the
   orchestrator (`grep -c -a '^## Symptom'` reads 1 in each capture). **Raised, applied.**
Severity: the agent returned `warning` under an `escalate` detector and said why — no new input surface, the
exception's scope and terms unchanged. Agreed: no boundary widens; the playbook's never-routine class does not
match (nothing new crosses the capture boundary — three more captures of the already-ratified class).

### architecture
1. `D-arch-resources` · warning · §Occupied Resources → Environment variables — register
   `ANDROMEDA_PULSE_MODEL_PATH` · `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`. Its basis cites contract line numbers the
   report does not carry. **REJECTED as proposed; RAISED by the orchestrator under check 5** (Expected amendment
   2). Check 1: `playbook.md:137` governs, `routine` — every clause holds: an external handle Conductor neither
   sets nor reads, registration into the env registry, and a SHIPPED artifact names it (a committed contract, one
   of the rule's own three examples). The entry states the rule's three facts. **Applied**, in a 360 B bullet
   re-derived from the report (the proposal's was 385 B).
2. `D-arch-resources` · `dependent-of` 1 · the `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` bullet — "The one
   registered handle …" becomes "A registered handle …". **Apply** (with the raised primary; the uniqueness claim
   is false once 1 lands).
3. `D-arch-resources` · the posture-contract entry (`:184`) — the series list gains `2026-10-07`. Its basis cites
   a contract line the report does not carry. **REJECTED as proposed; RAISED under check 5** (Expected amendment
   1). **Applied.**
4. `D-arch-resources` · `dependent-of` 3 · the same entry — the latest pin reads `f70be92` at 2026-10-07,
   replaced not appended. **Apply** (Expected amendment 1 names it).
5. `D-arch-registry-size` · warning · §Occupied Resources — free bytes for 1-4. The detector's check is the
   orchestrator's own tooling step, and the proposal's basis cites the check script. **REJECTED as proposed;
   decided by the orchestrator.** The proposal would move two evidence pointers (`:180`, `:161`) to the sidecar
   and drop three self-restating clauses. Taken: the three clauses (`:204`, `:206`, `:207`, 138 B), each restating
   its bullet's own opening "not in the reserved namespace". Not taken: the two evidence pointers — a mechanism
   statement keeps its `as measured at` pointer in the body. Taken instead: the bootstrap-window bullet's last
   sentence (203 B), the log TARGET that confirms the booted posture, which `test-plan.md:391` states in full and
   more exactly. **Applied.** Measured after: §Occupied Resources 38115 B of 38115 B, `registries: within target`,
   0 B of headroom; §Established Decisions untouched at 37980 B.

## Checks over the whole set

- **Check 2, cross-contradiction:** none. No two proposals edit one site in opposing directions.
- **Check 3, intent-consistency:** the report's deviations (the dir clause's reason; `command grep` on one entry;
  the ledger's restored heading) are justified and inside the intent. Scope record: no line; `gate.py scope` clean.
- **Check 4, absence needs evidence:** the "0 hits" claims carry their greps in the report; the cascade sweep's
  rows are dispositioned in `cascade-dispositions.md`. `splice.py summary` was not needed: every hit was read by
  offset through a bounded window.
- **Check 5, expected amendments:** 1 carried (architecture 3, 4) · 2 carried (architecture 1, 2, 5) · 3 carried
  (security-plan 1-4) · 4 carried (test-plan 1) · 5 carried (obs-plan 1) · 6 not carried — the series reads NOT
  MET, so architecture's one `v3-09` verdict statement stands.
- **Check 6, disproved claims:** `architecture.md:184` "NAMES only already-registered environment handles" —
  DISPOSED: true as written after architecture 1. The plan's step-6 dir clause — DISPOSED: a plan clause; no master
  states the retired reason (`security-plan.md:121` already names the temp roots); recorded in the report.
- **Escalations:** 0.
- **A rule to propose at the wrap card** (no match at check 1, fourth wrap in a row): a real-model series'
  dated-record extension — the capture row's series lists, the exception's inventory, test-plan's dated verdict,
  obs-plan's dated observation, architecture's series list and latest pin — as `routine`.
