# Scope — `2026-09-22-interpretation-proven-live`

**Epoch 4 — Live proof against a real Pulse** · head of the markerless tail, `conductor-0.3.0/working-route.md:44`

**Working entry (verbatim intent):** Interpretation proven live — known root cause injected through the emission
path, top hypothesis asserted to identify it, graded against the stated rule

---

## What this chunk is

The **drive half** of Epoch 4's live-proof pair. The posture half (`2026-09-18-real-model-leg-posture-and-grading-rule`)
committed `contracts/pulse-real-model-leg-posture.md`, which fixed — before any drive — the launch posture, the
grading rule and the per-leg quiet window, and stated an emission profile it believed the observable requires. P3
found that belief false; see (1) below. This chunk performs the drive under that document: a live leg with Pulse's deterministic L4 mode OFF, a root cause known in advance introduced
through Conductor's normal OTLP emission path, and the SUT's top-ranked hypothesis graded against the stated rule.

It is the chunk that moves "Conductor green" from *Pulse's plumbing carried a canned answer* to *Pulse's
interpretation named the cause Conductor planted* — intent §F4 EXPECT (`conductor-0.3.0/intent.md:116`), requirement
`v3-09` (`requirements.md:27`), and the residual `2026-08-09-interpretation-correctness-posture`
(`.andromeda/residuals.md:9`, `absorbed:v3-09`).

## What it builds (stated by the entry)

1. **A known root cause, injected through the emission path** — a cause fixed in advance and introduced by
   Conductor's ordinary OTLP emission (the harness's normal path — never a staged corpus row, never a side channel).
2. **The top-hypothesis assertion** — the SUT's top-ranked hypothesis asserted to identify that cause.
3. **Grading against the stated rule** — the verdict decided by `contracts/pulse-real-model-leg-posture.md` §The
   grading rule, with no part of that rule re-decided here.

## Folded annotation (from the working entry, re-verified at promotion)

**`CONTEXT (from 2026-09-18-real-model-leg-posture-and-grading-rule)`** — the entry's only freight block
(`route.py pins`: one block at line 44, 1 831 chars; no abstention names line 44). Folded whole. Its binding clause,
verbatim: *"the stated rule" is now a committed artifact — `contracts/pulse-real-model-leg-posture.md` fixes the
launch posture, the grading rule and the per-leg quiet window this entry is bound by, and the drive must not
re-decide any of them.* It also states that **`v3-09` stays pooled and is this entry's to claim.**

Every coordinate the block names, re-verified against the artifact at promotion (the fold's duty covers named
coordinates only; the mechanism claims below are P3's to close):

| Named coordinate | Verified against | Result |
|---|---|---|
| `contracts/pulse-real-model-leg-posture.md` | the file, read whole | ✓ present, 181 lines, 7 `## ` sections (regime · launch posture · emission profile · grading rule · quiet window · process census · non-goals) |
| Pulse HEAD `83d4060` | `git -C ../andromeda-pulse log -1` / `rev-list --count 83d4060..HEAD` | ✓ HEAD is still `83d40601` — **0 commits since**; tracked tree clean but for Pulse's own bookkeeping (`.andromeda/friction-log.ndjson`, `.claude/session-handoff.md`) |
| `pulse-app/src/inference_runtime.rs:823` | the SUT file at HEAD | ✓ `registry.attach_interpretation_summary(existing.id, json, now_unix_nano)`, inside the branch commented "Refresh the latest interpretation on the deduped incident" |
| `crates/triage/src/incident/registry.rs:128-143` | the SUT file at HEAD | ✓ `attach_resolution_summary` — "`InvalidTransition` when target incident is NOT already Resolved"; the live sibling `attach_interpretation_summary` is declared immediately after it |
| `crates/conductor-core/src/scenario.rs:52-72` | this repo | ✓ `pub enum SloTier` (`<5s` / `<20s` / `<90s`) and `deadline_ms()`; the `Tier90s => 90_000` arm itself sits at `:73`, one line past the cited range |
| `crates/conductor-verify/src/extract.rs:102` | this repo | ✓ `push_segment(&mut observation.text, markdown)` — the `retrieve_report` markdown pushed into the composed observation text |
| `v3-09` pooled | `matrix.py show --id v3-09` | ✓ `chunk: None` · `status: planned`; its 2026-09-18 note records the posture chunk's deliberate non-claim and four measured findings |

### Mechanism claims the block carries — CLOSED at P3 (`research.md` §Premise findings holds the derivations)

Each original claim is quoted verbatim so the correction can be read against it.

- `[premise-corrected: FALSE at the pinned HEAD — Pulse attaches the parsed L4 output AT CREATION
  (pulse-app/src/inference_runtime.rs:884, commit b2e4cb3 of 2026-08-27, the same commit as the cited :823 dedupe
  attach, before HEAD 83d4060), and Conductor's own 2026-09-06 capture read a single-storm canary incident
  non-degraded (live_suite_harvest.rs:51, ManualCheck)]` **(1) A single-storm incident HAS a hypothesis observable.**
  Original: "A single-storm leg has NO hypothesis observable at all … measured at Pulse HEAD `83d4060` … — so the
  emission profile must produce a SECOND cue-bearing digest on one identity or the leg reads back a degraded report and
  grades nothing." A second digest is therefore SUFFICIENT but not NECESSARY. A report is degraded only when no L4 output
  attached or the attached JSON fails to parse — including Pulse's whole-field scrub, which replaces the entire JSON
  when any part trips a scrubber pattern (`inference_runtime.rs:665-671`). `## Hypotheses` always renders
  (`markdown.rs:153`); "the report rendered hypotheses" means at least one ranked `- **(` entry, never "section present".
- **(2) Never an SLO tier — VERIFIED on a corrected ground.** Original: "the ladder caps at 90 000 ms … while real-model
  incident formation measured ~110 s, so any tier assignment would red the leg on timing whatever the model said." The
  cap holds (`scenario.rs:73`, `Tier90s => 90_000`). `[premise-corrected: the ~110 s figure is NOT a real-model
  measurement — its only witness (crates/conductor-run/tests/lifecycle_live.rs:20, committed f1584b1) ran under its own
  firing form's ANDROMEDA_PULSE_L4_DETERMINISTIC=true, recorded "deterministic L4" at
  conductor-0.2.0/chunks/2026-08-31-p-075-assert-round/report.md:161, and was re-attributed to the real model at
  conductor-0.2.0/chunks/2026-09-06-operator-gated-live-suite/plan.md:102]` The disposition stands because real-model
  timing is UNMEASURED and non-deterministic, so no tier can be declared honestly before a drive — not because 110 s
  exceeds 90 s.
- **(3) No new machinery — PARTLY verified.** Original: "The report markdown reaches `Observation.text` …, so
  `ComparisonKind::Contains` expresses it, and rank is position in the hypotheses array." `extract.rs:102` does push the
  markdown into `Observation.text`, and rank IS position (`markdown.rs:160-171`; the prompt's "ranked
  highest-confidence-first", `prompt.rs:101`; a fallback-tier host emits exactly one, `model_router.rs:36`).
  `[premise-corrected: the declare-only + harvest-tier grade needs a CAPTURE channel that does not exist — Observation is
  in memory only and the read-back self-obs line logs key names only (extract.rs:99); and Contains over the composed text
  (every active incident's report, extract.rs:90-103) cannot express rank]` — the grade scopes to the first entry of the
  attributed incident's `## Hypotheses` section.
- `[premise-corrected: no real-model formation figure exists to confirm — see (2)]` **The formation figure is MEASURED
  at the first drive**, not confirmed. Original: "A carried 2026-09-06 measurement, not re-measured by the posture chunk:
  confirm it at the first drive."

## Scope the entry did not state — CLOSED at P3

- **The known cause is authored as a scenario naming a Pulse P-ID — VERIFIED.** P-033 is *Ranked Hypothesis
  Generation* (`coverage-matrix.md:39`), one of the eight ids pinned in `conductor_core::UNBACKED_AUTO`
  (`drift.rs:61-63`), which `check_scenario_backing` holds to exact-set equality; naming it moves that pin AND the
  committed matrix's `(8 unbacked)` header in the same change (`verification-matrix.json#v3-10` notes). The next entry
  (`working-route.md:46`, claim `v3-10`) owns the cluster's pin move. Where this chunk's boundary falls is P4's.
- **The cause and its identifying text are fixed before the first drive — VERIFIED, and bounded.** The model reads the
  L3 digest only (`assembler.rs:608-690`): service rows, the cue kind, `scope_id={service}` and the citable fingerprint
  ids. Exception type and message text never reach it. A known cause is therefore expressible only as WHICH service and
  WHAT anomaly kind. Pulse's own P-033 verification line: "verify highest-ranked hypothesis correctly identifies cause
  within reasonable interpretation of model output" (`pulse-capability-spec.md:448`). Every non-topology emission shape
  runs under the one service `conductor` (`conductor-emit/src/message.rs:25`), a prefix of the canary's `conductor-canary`.
- **"Top-ranked" is position, not presence — VERIFIED** (see (3) above).
- `[premise-corrected: the firing form is not merely open — the posture's launch is UNREACHABLE at HEAD through every
  path: conductor preconditions grades ANDROMEDA_PULSE_L4_DETERMINISTIC by flag_declared (conductor-core
  preconditions.rs:28-31, :61-66), so run --live refuses at exit 1 (agent-run.sh:95-98); independently preflight's
  l4-deterministic term (pulse-run-contract.toml:32-37) blocks every scenario (conductor-verify preflight.rs:228-229)]`
  **The chunk must add a posture-aware gate**: an explicit, operator-chosen selector under which the L4 term reads
  "absent or falsy", while the deterministic legs' gating stays byte-unchanged and every unmet term is still named. The
  selector can never be derived from the env itself: a forgotten `L4_DETERMINISTIC=true` would then silently become a
  real-model run. How it is selected is P4's.
- `[premise-corrected: with the creation-time attach (1), reaching the dedupe branch is no longer the thing to
  confirm]` **The leg confirms it graded the SCENARIO's incident, non-degraded.** Attribution is by the report
  header's `Opened (unix-nano)` against the scenario's emission instant (the freshness carrier), so the canary's
  incident can never be graded in its place. Pulse's own `interpretation.incident.created` line (`created` /
  `deduped`) records which branch fired.
- **The attended seam — VERIFIED.** The operator launches `pulse-app` with the model configured (Pulse
  `AI-Model/RESUME-NOTE.md`: `ANDROMEDA_PULSE_MODEL_PATH` + `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`, neither registered in
  arch's body) and deterministic L4 absent or falsy. The drive itself is agent-driven, and the chunk holds `pending`
  until the drive is recorded — never wrapped with it owed.
- **Budgets, window and census follow the posture's arithmetic — VERIFIED, sharpened.** The poll floor is raisable only
  upward through `CONDUCTOR_PREFLIGHT_TIMEOUT` (`agent-run.sh:38-49`). The preflight canary is ITSELF a real-model gate:
  a canary the model dismisses is `Blocked` before the scenario emits (`research.md` P-H).
- **Where the measured formation figure lands — reframed:** a measurement record (report, `v3-09` notes, the posture's
  provenance line), never an edit to the grading rule.

### Added at P3 (the entry did not state these)
- **The success-path row lands `ManualCheck`, not `KnownResidual`** (`execute.rs:248-254`, measured by leg b1).
  `KnownResidual` is the degraded path, or the emptied active set.
- **A capture channel for the rank-1 hypothesis is new work** (`research.md` P-F). Committing any capture commits
  corpus-rendered model text, which is the security extract's open §Data Protection item. P4 settles it explicitly.

## CI verdict (Setup 5a)

`74a087225b9b032b2630515a3e6fbb9930e8e6e4` — the last wrap's commit, the tree this chunk starts from — read by full
sha through `gh api …/commits/{sha}/check-runs`: all three completed **success** (Rust gate · A11y gate, routine arm
· Frontend gate). No red to disposition; nothing folded from CI.

## Boundaries — what this chunk does NOT do

- **It does not re-decide the posture.** Launch posture, grading rule, quiet window and census obligation are the
  committed document's; this chunk applies them. The document's emission-profile requirement rests on the falsified
  premise (1). Whether the leg keeps that profile is P4's decision, recorded and never silent.
- **It does not re-posture, re-grade or re-run the deterministic-L4 `--live` legs** (H · B1 · B2 · quiet · A ·
  driven-a11y); their posture and results stand.
- **It never re-drives a failed leg until it passes** — a failing leg grades as a failure (posture §The grading rule).
- **It is never a CI gate** — the leg is reachable only from the operator-gated arm.
- **It does not touch `v3-08`** (BLOCKED on a Pulse release) or the Hue-shift entry (`working-route.md:48`).
- **It opens no inbound listener and adds no network reach** (scope law, unchanged).
- **It writes nothing in `andromeda-pulse`** — the SUT is read, never amended.

## Surfaces and contracts it touches

- `scenarios/` — the leg's scenario.
- **The posture-aware gate** (added at P3, `research.md` P-A): `crates/conductor-core/src/{preconditions,run_contract}.rs`,
  `contracts/pulse-run-contract.toml`, `crates/conductor-run/src/{preconditions,canary}.rs`,
  `crates/conductor-cli/src/{cli.rs,commands/}` and `scripts/agent-run.{sh,ps1}` — the operator-gated `--live` arm
  and its `preconditions` short-circuit. `[val-1 amended at P5: the gate adds posture-taking entry points BESIDE the
  deterministic ones (preflight_for / observe_preconditions_for / evaluate_for), so the Tauri run_thread
  (commands.rs:323), the suite and the parity test keep calling the unchanged deterministic preflight() — a real-model
  scenario reached from them lands Blocked by the posture-mismatch check]`
- A capture tool plus a harvest-tier test in `crates/conductor-run/tests/` (precedent: `live_suite.rs` /
  `live_suite_harvest.rs`).
- `contracts/pulse-real-model-leg-posture.md` — binding for its launch posture, grading rule, window and census; its
  mechanism clauses carry the corrections above. `contracts/pulse-run-contract.toml` — the budget terms, plus the
  posture's L4 term. `contracts/pulse-capabilities.toml` — the P-ID's membership.
- `conductor_core::UNBACKED_AUTO` / `check_scenario_backing` + `coverage-matrix.md` — only if the scenario names a
  cluster P-ID.
- `contracts/scenario-audit-ledger.toml` — an `[[over_tier]]` row if the scenario's phases sum past 90 000 ms.
- `conductor-0.3.0/verification-matrix.json#v3-09` — this entry's to claim (stated by the block).
- `.andromeda/residuals.md:9` — `absorbed:v3-09`; must stay consistent with what ships.

## Why it is here (provenance)

- Route entry `conductor-0.3.0/working-route.md:44`, Epoch 4, directly after the frozen posture entry (`:42`).
- `verification-matrix.json#v3-09` (*Real-model interpretation leg*) — the claim target.
- `conductor-0.3.0/intent.md:116` §F4 · `conductor-0.3.0/requirements.md:27`.
- `.andromeda/residuals.md:9` — the absorbed `2026-08-09-interpretation-correctness-posture` residual.
- `contracts/pulse-real-model-leg-posture.md`, committed by `2026-09-18-real-model-leg-posture-and-grading-rule`.
