# Codebase Research — 2026-09-18-real-model-leg-posture-and-grading-rule

## Scope
- **Depth:** deep · **Reads:** 18 (9 Conductor, 9 SUT) · **Globs/Greps:** 12
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — **read in full** (71 485 B / 67 lines,
  max line 13 254 chars, so the in-full read was performed as a structural extraction per the over-cap rule:
  `grep -nE '^#{1,4} '` + `grep -nE '^- '` for the index, then five offset-bounded spans covering lines 1-39,
  40-47, 48-53, 54-58, 59-67 — every indexed entry covered). Also `.claude/rules/observability.md`
  (`paths:` includes `scripts/agent-run.*`); `security.md` + `host-win32.md` load unconditionally.
  27 Session-Additions entries; 14 bear directly on a live leg.

## Files inspected

**Conductor**
- `scripts/agent-run.sh` (60-150, the `--live` suite) — leg order H → B1 → B2 → 150s quiet → A → driven-a11y;
  `live_leg_budget_sec` = preflight budget + phases + 60s; per-leg freeze to `runs/live-suite/{leg}.jsonl`
  because the self-obs sink truncates per invocation; the leading non-priming `preconditions` probe refuses
  at exit 1. **Its own leg-order comment states the finding this chunk turns on** (see Patterns 1).
- `scripts/agent-run.ps1` (7-210) — `--live` at parity: same leg order, same 150s window, same refusal line,
  same capture dir. Any arm landing in one shell binds the other.
- `crates/conductor-core/src/scenario.rs:52-72` — `SloTier` is **closed at three**: `<5s` / `<20s` / `<90s`,
  `deadline_ms()` maxing at 90 000. Nothing admits a bound above 90s.
- `crates/conductor-core/src/expected.rs:29-58` — `ComparisonKind` closed at four: `Exact`, `Contains`
  (substring/membership), `Absent`, `CountAtLeast`. `MAX_BUDGET_MS` derives from `Tier90s.deadline_ms()`.
- `crates/conductor-verify/src/extract.rs:33-57, 91-102, 135-155` — `Observation { text, evidence_count,
  degraded, fingerprints }`; `observed_for(kind)` hands `CountAtLeast` the evidence count and **every other
  kind the composed `text`**; `:102` pushes the `retrieve_report` markdown into that text.
- `crates/conductor-run/src/execute.rs:44, 97` — `execute_scenario` (the `pub async fn`), and the single
  production `route_read_back` call site.
- `crates/conductor-run/src/lib.rs:39`, `crates/conductor-cli/src/commands/preconditions.rs:19` — the two
  production `observe_preconditions` callers.

**SUT (`andromeda-pulse`, HEAD `83d40601`, tree clean on tracked source — only bookkeeping modified)**
- `pulse-app/src/deterministic_inference.rs` (1-75) — the canned `L4Output`: `schema_version` 2.0,
  `decision: surface`, `severity: autonomous`, a **`hypotheses` array** of `{statement, confidence,
  justification}`, `investigation_steps`, and `evidence_refs` **populated** with a `det-*` triple (68-72).
- `crates/mcp-server/src/jsonrpc.rs:188-189` — the `retrieve_report` tool declaration and its description.
- `crates/mcp-server/src/tools.rs:360-387` — `dispatch_retrieve_report`: the decisive code.
- `crates/interpretation/src/markdown.rs:15-32, 41, 97, 153` — the hybrid render contract, `HypothesisView`,
  and the literal `## Hypotheses` section header.
- `crates/triage/src/incident/registry.rs:128-160` — `attach_resolution_summary` and its sibling
  `attach_interpretation_summary`, both writing `resolution_summary_text`.
- `pulse-app/src/inference_runtime.rs:236-258, 800-835` — the two call sites that write it.

## Graph impact (from the code-graph query; `rows: 11`, plane `rust`, `db_state: fresh`, `probe_hits: None`)
- **`route_read_back`** — 7 rows, of which exactly **one production caller**: `conductor-run/src/execute.rs:97`
  (the other 6 are that file's own `#[cfg(test)]` module). The read-back router this chunk must compose with
  rather than duplicate.
- **`observe_preconditions`** — 4 rows, **2 production callers**: `conductor-cli/src/commands/preconditions.rs:19`
  and `conductor-run/src/lib.rs:39`, plus `conductor-run/tests/composition_root.rs:51`.
- **`execute_scenario` — 0 rows, and that zero is an INDEX GAP, not a finding.** The prescribed `symbol` probe
  by name (`SELECT name, kind, crate, file, def_line FROM symbol WHERE name = 'execute_scenario'`) also returned
  `[]`, so the name is not on the rust plane at all; `grep -rn 'execute_scenario' --include=*.rs crates/`
  returns **15** hits including the definition (`conductor-run/src/execute.rs:44`, a generic `pub async fn`)
  and production callers at `conductor-cli/src/commands/run.rs:23`, `commands/suite.rs:32`,
  `conductor-run/src/drive.rs:14` (use) and `:80` (call). **Any impact statement for this symbol rests on grep,
  not the graph.** Recorded because CLAUDE.md's 2026-09-02 learning measured this same symbol at 7 sites / 5
  callers *from the graph* — so the index lost it since, and that learning's figure must not be re-quoted.

## Patterns detected

1. **The deterministic flag is load-bearing for the dedupe WINDOW, not only for reproducibility**
   (`scripts/agent-run.sh:118-121`, and `.claude/rules/verification-harness.md` entry 2026-09-06 (c)):
   canned-L4 incident formation is ~2s so leg B2's storm lands inside Pulse's 120s idle window, while
   **real-model formation was measured at ~110s and would fall outside it**. An env missing the flag "does not
   make the leg slow, it makes it measure the wrong thing".
2. **The full firing form is part of the leg** (harness rules 2026-08-10 / -13 / -14 / -19 / 2026-09-07):
   sidecar BUILT and resolvable on `PATH` in POSIX form (a bash `PATH` prepend of a Windows-form path splits on
   the drive-letter colon), `ANDROMEDA_PULSE_MCP_ENABLED` in Conductor's own env, `ANDROMEDA_PULSE_DATA_DIR`
   equal to the LIVE app's dir, the paired `RUST_LOG=info,conductor_emit=debug` (never the bare per-target
   form), and never `boot` immediately before `conductor run` on one data dir.
3. **A `[BLOCKED]` row in ~0s is sidecar resolution until proven otherwise** (rule 2026-08-20) — the same row
   shape a genuine SUT gate failure produces; elapsed time is the discriminator.
4. **Declare-only + harvest-tier is the established landing for a claim the envelope cannot carry**
   (tests + obs extracts; obs-plan §4): the scenario row lands `verdict` null / `state=KnownResidual` and the
   hard assertion moves to a `crates/conductor-run/tests/*_harvest.rs` target over verbatim leg captures.
5. **Every leg that boots an external process ends with a real process census, taken twice**, with the stop
   form named beside the firing form (rule 2026-09-02); `pulse-app` is always `left running — operator stops it`.

## The load-bearing mechanism, stated as the equality the design needs — and verified at HEAD

**Needed:** a real-model leg's incident, read back through the MCP surface, yields a report whose
`## Hypotheses` section carries the model's top-ranked statement, in a form a Conductor check can grade.

**Verified chain (SUT source at `83d4060`, then Conductor source at HEAD):**
1. `dispatch_retrieve_report` (`tools.rs:360-387`) parses `incident.resolution_summary_text` as `L4Output`;
   `let degraded_mode = parsed_l4.is_none();` and `assemble_report(&incident, parsed_l4.as_ref(), …)`.
   It returns `{ "markdown": …, "degraded_mode": … }`.
2. `serialize_report` emits the literal section header `## Hypotheses` (`markdown.rs:153`) only when the
   parsed L4 is present; `HypothesisView` (`:41`) is described at `incidents_router.rs:84` as a **"Single
   ranked hypothesis"**, so position in the array is the rank.
3. Conductor's `extract.rs:102` pushes that `markdown` into `Observation.text`, and `observed_for()` hands
   the composed text to every substring kind — so **`ComparisonKind::Contains` can grade a hypothesis
   substring**. The assertion is expressible as a scenario `[[expected]]` check; it does not need a new kind.

**The half that decides the design — WHO WRITES `resolution_summary_text`:** two writers, and neither fires
on first incident creation.
- `attach_resolution_summary` — rejects anything not already **Resolved** (`registry.rs:128-143`); fired from
  `inference_runtime.rs:250` only when `digest.kind == DigestKind::ResolutionSummary || parsed.is_resolution_summary`.
- `attach_interpretation_summary` — the **live** sibling (`registry.rs:145-160`), added expressly "so a report
  renders the model's actual content instead of a false-degraded notice"; rejects a Resolved incident. Fired at
  `inference_runtime.rs:823`, and only inside the **dedupe re-generation** branch: an existing active incident
  matching `(kind, scope, scope_id)` whose `observe_reemission` succeeded.

**Therefore:** the FIRST incident a storm forms carries `resolution_summary_text: None`, so `retrieve_report`
returns `degraded_mode: true` and a report with **no `## Hypotheses` section at all** — nothing to grade. The
hypotheses become readable only after a SECOND cue-bearing digest on the same `(kind, scope, scope_id)` identity
while the incident is still active, or after a resolution summary post-resolution. **A single-storm real-model
leg has no hypothesis observable by construction.**

## Two claims that did NOT survive re-derivation at HEAD
- **`.claude/rules/verification-harness.md` entry 2026-08-16 is stale on `evidence_refs`.** It states the
  deterministic fixture "pins [`evidence_refs`] to `[]` (`deterministic_inference.rs:35`)". At Pulse HEAD the
  fixture populates it with a `det-*` triple (`deterministic_inference.rs:68-72`) and its doc comment
  (`:26-35`) explains at length why an empty vector would be wrong ("makes every payload-identity assertion …
  compare nothing and pass"); line 35 is now a scrubber-pattern sentence. The obs extract carried the CORRECT
  version ("a constant `det-*` triple"), which is what surfaced the divergence. Owner: a rule-file curation
  item at wrap, not a phase edit.
- **`retrieve_report`'s own tool description (`jsonrpc.rs:189`) is stale**, and so is `markdown.rs`'s header
  contract (`:20-27`). Both say active incidents render a degraded notice and only resolved ones render full
  hypotheses; `attach_interpretation_summary` exists precisely to defeat that, and is the live path. The
  behaviour is decided by `parsed_l4.is_some()`, never by status — which is why the chain above is traced to
  the writers rather than read off either description.

## Conventions to follow
- **5-command surface, sibling stage flag** — no sixth command; `--live` is a `case` branch (`agent-run.sh:270`).
- **Two-shell parity** — every `--live` change lands in both `.sh` and `.ps1` (measured at parity today).
- **Reader-less `contracts/` member** — `contracts/pulse-p025-measurement-contract.md` is the precedent: no
  `default_path()`, no `resolve_under`, no bounds check, no `CONDUCTOR_*` override; carries `provenance`.
- **Host-path freedom** — the committed artifact is grep-verifiable clean of drive-letter/`%APPDATA%`/`/home`
  tokens; note the standing false-positive class (a `letter:slash` run also matches a URL scheme).

## New files to create
- `contracts/pulse-real-model-leg-posture.md` — the committed posture + grading rule + quiet-window statement,
  reader-less regime. (Name/shape is P4's to fix; the regime is settled by the precedent above.)

## Files to modify
- `scripts/agent-run.sh` · `scripts/agent-run.ps1` — only if P4 decides the posture needs a registered arm;
  a documentation-only posture may touch neither. Both bind together if either does.
- No `.rs` modify-set is implied by the posture itself. **The PREREQ's `cargo clippy` therefore runs over a
  zero-`.rs`-delta tree by design — which is the same condition that justified deferring it twice. It runs
  anyway: the pin's third consecutive re-pin trips the age trigger, and the discharge is the point.**

## Open questions
- **Does the posture register a harness arm, or is it documentation-only?** → blocks: plan-decision. P4 must
  settle it; layouts/arch both constrain the answer (sibling flag, never a sixth command) but neither requires
  an arm to exist now, and the drive chunk is the natural owner of one.
- **What disposition does a real-model leg carry, given `SloTier` caps at `<90s` and formation alone measured
  ~110s?** → blocks: plan-decision. The declare-only + harvest-tier precedent (Patterns 4) is the only shape
  in the system that fits; P4 states it or states why not.
- **Is the ~110s figure still current at Pulse HEAD?** → blocks: implementation-scope. It is a 2026-09-06
  measurement of SUT timing that cannot be re-measured without a drive, and this chunk does not drive. It is
  recorded with its date and witness; the drive chunk confirms it.
