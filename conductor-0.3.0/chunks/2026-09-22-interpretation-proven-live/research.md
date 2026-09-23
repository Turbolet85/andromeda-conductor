# Codebase Research — 2026-09-22-interpretation-proven-live

## Scope
- **Depth:** deep · **Reads:** ~45 (Conductor 29 · Pulse 16, counted from this session's Read / `sed` / `cat`
  calls, approximate) · **Globs/Greps:** ~30 (same basis)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read IN FULL (66 lines, 72.7 KB, as two
  offset-bounded halves `1-46` + `47-66`; 27 Session Additions) · `.claude/rules/observability.md` (full, 2
  additions) · `.claude/rules/testing.md` (full, auto-loaded) · `.claude/rules/security.md` +
  `.claude/rules/host-win32.md` (always loaded). The additions this chunk rides are cited `{file}:{line}` below.
- **SUT read at:** `andromeda-pulse` HEAD `83d40601` — `git rev-list --count 83d4060..HEAD` = **0**, tracked tree
  clean but for Pulse's own bookkeeping. Every Pulse coordinate below is a reading at that HEAD.

## Files inspected
- `contracts/pulse-real-model-leg-posture.md` (full) — the binding document; four of its mechanism clauses are
  falsified or corrected below (§Premise findings). Its grading rule survives every correction.
- `crates/conductor-core/src/preconditions.rs` (full) — `OBSERVED_HANDLES` (`:28-32`) includes
  `ANDROMEDA_PULSE_L4_DETERMINISTIC`; every non-path handle is graded by `flag_declared` (`:48-53`, `:61-66`).
- `contracts/pulse-run-contract.toml` (full) — term `l4-deterministic` (`:32-37`) is a `shell-declaration` requiring
  the env var declared `=true`; `[incident_formation]` `warmup_ms = 45000`, `min_canary_poll_seconds = 90`.
- `crates/conductor-core/src/run_contract.rs` (`1-230`) — `evaluate` (`:156-169`): only a `ShellDeclaration` term can be
  unmet, unmet = env not in the declared set; `CheckKind` is closed at three (`:60-69`).
- `crates/conductor-run/src/preconditions.rs` (full) — env read at the caller (`declares` `:43-45`, `observe_preconditions`
  `:57-89`), the evaluator stays pure.
- `crates/conductor-run/src/canary.rs` (`1-260`) — `preflight` (`:37-63`) / `readiness` (`:68-83`) → `canary_gate`
  (`:173-202`) loads + observes the run contract, skips warm-up when unmet, emits the 12-exception canary storm on
  `conductor-canary` (`:111`, `:118`), freshness carrier = the storm's emission stamp (`:225-227`).
- `crates/conductor-verify/src/preflight.rs` (`95-294`) — `run_preflight` blocks on `!contract.is_satisfied()` with
  `"unmet run-contract terms: [id] statement — causes"` (`:141-151`, `:228-229`); canary poll skipped when unmet
  (`:208-212`).
- `scripts/agent-run.sh` (full) — `run --live` (`:91-151`): leading `conductor preconditions` refusal (`:95-98`), legs
  H → B1 → B2 → `sleep 150` → A → `a11y:driven` (`:122-139`), per-leg budget `preflight_budget + phases + 60`
  (`:73`), capture freeze `runs/live-suite/{leg}.jsonl` (`:78-89`), the `.ps1` twin at parity (346 lines).
- `crates/conductor-core/src/phase_spec.rs` (`1-405`) — `EmissionSpec { signal, occurrences, shape }` (`:94-111`), no
  service field; `EmissionShape` (`:208-259`); only `Topology` names services.
- `crates/conductor-run/src/dispatch.rs` (grep) — every non-topology shape emits under `DEFAULT_SERVICE_NAME`
  (`= "conductor"`, `conductor-emit/src/message.rs:25`); `Exception` derives from one `base_exception()` (`:260-269`).
- `crates/conductor-verify/src/extract.rs` (`1-150`) — `observe` (`:76-131`): `query_incident_list` (ACTIVE only) →
  `list_text` (status/severity/title per item) + every incident's `retrieve_report` markdown pushed into ONE
  `Observation.text` (`:90-103`); `degraded` is the OR across incidents (`:104-107`); only key NAMES are logged
  (`log_observed_keys`, `:99`).
- `crates/conductor-run/src/execute.rs` (grep) — `route_read_back` (`:234-240`) + `state_for` (`:248-254`): a
  declare-only read-back lands `ManualCheck` unless `degraded` (→ `KnownResidual`) or the corpus is empty (→
  `AutoResolved` → `KnownResidual`).
- `crates/conductor-run/tests/live_suite.rs` (full) + `live_suite_harvest.rs` (full) + `storm_harvest.rs` (`1-80`) +
  `lifecycle_live.rs` (`1-40`, and its first-committed header via `git show f1584b1:…`) — the capture/grade split.
- `crates/conductor-cli/src/cli.rs` + `commands/{run,preconditions,mod}.rs` (full) — no posture/L4 argument anywhere.
- `crates/conductor-core/src/drift.rs` (`30-64`) — `UNBACKED_AUTO` = 8 ids incl. the cluster P-031/P-033/P-034/P-044.
- `coverage-matrix.md` (`1-12`, `:36-50`) — header `(8 unbacked)`; `P-033` = *Ranked Hypothesis Generation*.
- `contracts/scenario-audit-ledger.toml` (`1-60`) — 2 `[[live_assertion]]` + 11 `[[over_tier]]` (`grep -c '^\[\['` = 13).
- `contracts/pulse-load-envelope.toml` — `max_sustained_storm_ms = 600000` per emitting phase.
- Pulse `pulse-app/src/inference_runtime.rs` (`395-425`, `480-560`, `665-679`, `740-915`) · `crates/triage/src/incident/
  registry.rs` (`126-160`) · `crates/mcp-server/src/tools.rs` (`360-387`) · `crates/interpretation/src/markdown.rs`
  (`1-330`) · `crates/interpretation/src/schema.rs` (`120-240`) · `crates/interpretation/src/prompt.rs` (`1-270`) ·
  `crates/triage/src/contract.rs` (`440-620`) · `crates/triage/src/digest/assembler.rs` (`240-300`, `555-690`) ·
  `pulse-app/src/model_router.rs` (grep) · `docs/v0_2_0/pulse-capability-spec.md` (`442-470`) ·
  `experiments/l4_output.json` + `l4_prompt_fixture.txt` · `AI-Model/RESUME-NOTE.md`.

## Graph impact
Query (rust plane, `db_state: fresh`, **32 rows** — trace `tree-query-2026-09-22-interpretation-proven-live.json`):
callers of `observe_preconditions` · `observe_run_contract` · `preflight` · `readiness` · `canary_gate` ·
`execute_scenario` · `load_run_contract`. Editor lines (graph line + 1):
- **`conductor_run::preflight`** — `conductor-cli commands/run.rs:21`, `commands/suite.rs:26`, `conductor-tauri
  commands.rs:323` (`run_thread`), tests `conductor-cli/tests/cross_surface_parity.rs:99`,
  `conductor-run/tests/composition_root.rs:50` (`public_api_paths_are_stable`). A posture parameter here threads
  through the GUI path too. (The name also collides with `conductor-cli commands::preflight` — the `main.rs:86` row.)
- **`observe_preconditions`** — `conductor-cli commands/preconditions.rs:20`, its own test `preconditions.rs:133`,
  `composition_root.rs:52`.
- **`observe_run_contract`** / **`load_run_contract`** — `canary.rs:179-180` (`canary_gate`) + two unit tests.
- **`canary_gate`** — `canary.rs:55` (`preflight`), `:75` (`readiness`).
- **`execute_scenario`** — `commands/run.rs:23`, `commands/suite.rs:32`, `drive.rs:80` (`drive_run`, the Tauri
  path), three `execute.rs` tests.

## Premise findings (the chunk's decisive research)

**P-A · The posture's launch is unreachable at HEAD, through every path.** The equality the design needs is "a
Conductor environment with `ANDROMEDA_PULSE_L4_DETERMINISTIC` absent/falsy reaches read-back". It does not:
(1) `conductor preconditions` grades the handle by `flag_declared` (core `preconditions.rs:28-31`, `:61-66`), so
`handles-declared` is unmet and `run --live` refuses at exit 1 before any leg (`agent-run.sh:95-98`); (2) independently,
`canary_gate` evaluates term `l4-deterministic` (`shell-declaration`, `pulse-run-contract.toml:32-37`) as unmet and
`run_preflight` sets `ready:false` with the named precondition (`preflight.rs:228-229`) — so even a bare
`conductor run` returns `[BLOCKED]` for every scenario. The posture's own term ("absent or falsy … a
shell-declaration") is an INVERTED declaration the core cannot express: `CheckKind` has no such kind and
`flag_declared` has no complement.

**P-B · A single-storm incident HAS a hypothesis observable — the posture's clause (1) is false at its pinned HEAD.**
`create_incident_from_l4_output` attaches the parsed interpretation AT CREATION: `resolution_summary_text:
scrubbed_l4_json(parsed)` (`pulse-app/src/inference_runtime.rs:884`, comment "The latest cleanly-parsed interpretation
attaches AT CREATION, so the report renders the model's content for a live incident instead of a false-degraded
notice"). `git blame -L 884,884` → commit `b2e4cb3` (2026-08-27, *"a clean parse reaches the report and citing becomes
copying"*) — the SAME commit that added the `:823` dedupe attach the posture did cite, before the pinned `83d4060`
(2026-08-31). The posture traced the two `IncidentRegistry` attach methods and missed the struct-literal write.
Conductor's own record already showed it: leg b1 (2026-09-06, HEAD `83d4060`) read back its single-storm canary
incident as `ManualCheck` — non-degraded, since `state_for` returns `KnownResidual` iff degraded
(`live_suite_harvest.rs:51`, `execute.rs:248-254`). Consequences: the second cue-bearing digest is NOT necessary (it
remains sufficient — the dedupe branch refreshes the attachment with the latest generation); a report is degraded
only when no L4 output was attached OR the attached JSON fails to parse — including Pulse's WHOLE-FIELD scrub, which
replaces the entire JSON with `[redacted: {category}]` if any part trips a scrubber pattern (`:665-671`).

**P-C · `## Hypotheses` always renders; the discriminator is its body.** `serialize_report` pushes the header
unconditionally (`markdown.rs:153`): degraded → `DEGRADED_NOTICE` (`:109`, "_Interpretation pending — …_"); parsed
but empty → "_No ranked hypotheses produced._" (`:158`); otherwise one `- **({confidence})** {statement}` entry per
hypothesis, in L4Output order, with an optional `\n  - {justification}` (`:160-171`). Rank is position (the prompt's
CONVENTIONS: "Hypotheses ranked highest-confidence-first", `prompt.rs:101`). "The report rendered hypotheses" must be
read as "≥1 ranked entry", never "section present".

**P-D · The success-path row lands `ManualCheck`, not `KnownResidual`.** A declare-only scenario whose read-back is
non-degraded takes `state_for(…, ManualCheck)` (`execute.rs:248-254`) — measured by b1. `KnownResidual` arises only
from a degraded report or an emptied active set (`route_read_back`, `:234-240`). The posture's "a null verdict and a
known-residual state" (§Disposition) holds for the failure paths only; the extracts that assumed `[RESIDUAL]` for the
success row (design, layouts) inherited it.

**P-E · The "~110 s real-model formation" figure is not a real-model measurement.** Its only witness is
`crates/conductor-run/tests/lifecycle_live.rs:20` ("INCIDENT FORMATION TOOK ~110s … L3's digest cadence with L4 behind
it"), as first committed at `f1584b1` — a leg whose own firing form sets `ANDROMEDA_PULSE_L4_DETERMINISTIC=true`
(`:14`), recorded "deterministic L4" in `conductor-0.2.0/chunks/2026-08-31-p-075-assert-round/report.md:161`. The
re-attribution is `conductor-0.2.0/chunks/2026-09-06-operator-gated-live-suite/plan.md:102` ("With the real model,
formation took ~110s (`lifecycle_live.rs` header)"), which propagated to `verification-harness.md:60` (c), the
posture, `architecture.md`'s registry row and this entry's CONTEXT. **No real-model formation figure exists in
Conductor's record.** The first drive MEASURES it; "confirm" is not available. The non-tier disposition still stands,
on the ground that survives: real-model timing is unmeasured and non-deterministic, so no tier can be declared
honestly before a drive.

**P-F · The grading as specified needs a capture channel that does not exist.** `extract.rs:102` does push the report
markdown into `Observation.text` (the claim's cited half holds), but `Observation` is in memory only: no envelope field
carries it, the journal holds the envelope, and the only read-back self-obs line logs key NAMES (`extract.rs:99`). A
harvest-tier grade "over the leg's verbatim captures" has nothing verbatim to read. Separately, `Contains` over the
composed text cannot express rank: the text unions list fields and EVERY active incident's report (`:90-103`), so the
comparison must be scoped to the first `- **(` entry of the attributed incident's `## Hypotheses` section.

**P-G · What the model can "identify" is bounded by the digest.** The L4 prompt (`prompt.rs:182-256`) embeds the L3
digest text `render_payload` builds (`assembler.rs:608-690`): `WINDOW`, `PROJECT`, `OVERALL`, `SERVICES` rows
(`{service} {rate}/s | {error}% | {p99}ms`), `ATTENTION CUES` (`[{tier}] {cue_kind} — {cue_kind} scope_id={service}`,
kinds `retry_storm` · `error_rate_spike` · `latency_regression` · `service_went_silent` · `restart_event`,
`:555-564`), corpus matches, plus the citable fingerprint ids. Exception type and message text never reach it (the
digest is contracted to carry no raw attribute payloads, `contract.rs:548-556`). So a "known root cause" is
expressible only as WHICH service and WHAT anomaly kind. Pulse's own P-033 verification line: "verify highest-ranked
hypothesis correctly identifies cause **within reasonable interpretation of model output**"
(`pulse-capability-spec.md:448`); its one committed real-model spike echoed exactly those two facets
("service-a is experiencing latency issues." for a `latency-regression: service-a` digest — `experiments/l4_output.json`).
Hypothesis count depends on the host's tier: `GpuFallback`/`CpuFallback` → the fallback prompt, EXACTLY ONE
hypothesis (`model_router.rs:36`, `prompt.rs:118-136`); primary → up to three.

**P-H · The canary is itself a real-model gate.** Under the posture the preflight canary's incident forms only if the
real model returns `surface` with severity ≠ `none` (`inference_runtime.rs` `create_incident_from_l4_output` returns
early on `Dismiss` / `None`), and within the poll budget. A dismissed canary is `Blocked: no incident opened after
the canary storm` — a model-dependent Blocked BEFORE the scenario emits. The poll floor (90 s) is raisable only
upward, through `CONDUCTOR_PREFLIGHT_TIMEOUT` (`agent-run.sh:38-49`); with formation unmeasured, the first drive's
budget is generous by construction.

## Patterns detected
- **Capture/grade split** (`live_suite.rs:1-44`, `live_suite_harvest.rs:1-23`): a `live-pulse`-gated target PRINTS what
  the leg produced; a default-suite harvest pins the frozen lines as LITERALS and grades them; no harvest reads a path.
- **Evaluator pure, observation at the caller** (core `preconditions.rs:1-16`, `run_contract.rs:9-18`; run
  `preconditions.rs:27-45`) — both branches testable without env mutation (testing.md 2026-08-10).
- **Freshness carrier** (`canary.rs:204-227`): attribute by an emission stamp taken just before the counted storm.
- **Reading Pulse's own log** (`storm_harvest.rs:9-15`): `{data_dir}/logs/agent-latest.jsonl.<date>`, sliced by pre-leg
  line count (`verification-harness.md:55` (a) — never assume 0).
- **Declare-only state routing** (`execute.rs:234-254`).

## Conventions to follow
- **One truthiness rule**: `flag_declared` is the workspace's single definition (`preconditions.rs:41-53`); any inverse must
  be defined from it, never re-implemented (so the two cannot drift).
- **Exact-set pins, both directions** (`drift.rs:40-63`, `preconditions.rs:435-447`, the audit ledger).
- **Live-leg doc header carries firing + stop form** (`live_suite.rs:13-28`, `lifecycle_live.rs:7-16`).
- **Harvest literals are host-path-free**; drive-letter checks word-anchored (testing.md 2026-09-03).
- **Committed artifacts read from tests via `CARGO_MANIFEST_DIR`** (testing.md 2026-09-06; `live_suite.rs:58-64`).
- **Expect atoms from printed output**: the CLI verdict line prints lamp prefix + SCENARIO NAME, never the P-ID
  (`verification-harness.md:64`) — success `[MANUAL] <scenario>`, degraded `[RESIDUAL] <scenario>`, gate
  `[BLOCKED] <scenario>`.

## Live-leg firing form (gathered here, per the research contract)
- Conductor's shell: `PATH=/d/dev/projects/andromeda-pulse/target/release:$PATH` (POSIX form, `verification-harness.md:54`,
  `:61`) · `ANDROMEDA_PULSE_DATA_DIR=<the live app's dir>` (`:50`) · `ANDROMEDA_PULSE_MCP_ENABLED=true` (`:47`) ·
  `ANDROMEDA_PULSE_L4_DETERMINISTIC` ABSENT or falsy (posture — inverted from every prior leg) ·
  `RUST_LOG=info,conductor_emit=debug` paired, never on a test-suite invocation (`:47` (a)) ·
  `CONDUCTOR_PREFLIGHT_TIMEOUT=<raised>` for an unmeasured formation (P-H).
- The operator's `pulse-app` launch: `ANDROMEDA_PULSE_MODEL_PATH` + `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` (Pulse
  `AI-Model/RESUME-NOTE.md`; the build needs `ANDROMEDA_LLAMA3_TOKENIZER_PATH`) — none registered in arch's body
  (arch extract); the suite-wide bootstrap disposition, confirmed by the log TARGET
  `triage.baseline.bootstrap_window.override` (`:60`); a cwd outside this repo (`:48` (2)); a fresh data dir under
  `%TEMP%/pulse-legs/<ts>` (`:52`).
- Pulse-log witnesses for the leg: `interpretation.prompt.assemble` (`prompt_version` → tier) · `interpretation.json.parse`
  (`parse_outcome`) · `interpretation.incident.created` (`created` / `deduped`) · `triage.pattern.storm.detected`
  (`severity_hint`).
- Stop form + census: taken twice, pattern derived from what the leg starts; `pulse-app` left running — operator stops it
  (`:58`).

## New files to create (provisional — P4 forks decide the shape)
- `scenarios/<real-model-scenario>.toml` — the known cause as declared emission data (declare-only, `<90s` + an
  `[[over_tier]]` ledger row if its phases sum past 90 000 ms).
- `crates/conductor-run/tests/<real_model>_live.rs` — `live-pulse`-gated capture tool (prints the attributed incident's
  `## Hypotheses` section and the witnesses).
- `crates/conductor-run/tests/<real_model>_harvest.rs` — default-suite harvest: pinned captures → the posture's outcome
  rows, with negative arms (below-rank-1 match, degraded/empty body, canary mis-attribution).

## Files to modify (provisional, by design axis)
- **Posture-aware gate (P-A — required by every option):** `crates/conductor-core/src/preconditions.rs` +
  `run_contract.rs` (+ `lib.rs` re-exports), `contracts/pulse-run-contract.toml`, `crates/conductor-run/src/
  preconditions.rs` + `canary.rs` (+ `lib.rs`), `crates/conductor-cli/src/cli.rs` + `commands/{preconditions,run,suite}.rs`,
  `crates/conductor-tauri/src/commands.rs` (`run_thread` → `preflight`, graph row above), `scripts/agent-run.{sh,ps1}`.
  If scenario-declared: `crates/conductor-core/src/scenario.rs` (a closed-enum field, garde-validated, serde default).
- **Caller/companion threading from the graph + the name sweep:** `conductor-run/tests/composition_root.rs:50,52`
  (`public_api_paths_are_stable`), `conductor-cli/tests/cross_surface_parity.rs:99`, `conductor-run/src/preconditions.rs`
  tests (`:100`, `:188`), `conductor-core/src/run_contract.rs:310` (pins `l4-deterministic`). Sweep
  `grep -rn "l4-deterministic" crates/ scripts/`: 6 hits · 2 changed-candidates (`run_contract.rs:310`,
  `conductor-run/src/preconditions.rs:100-107`) · 4 no-change (`run_contract.rs:74` doc example;
  `conductor-verify/tests/preflight.rs:26,29,369` — a constructed contract, not the committed one). Sweep
  `grep -rln "OBSERVED_HANDLES\|flag_declared\|handle_declared" crates/` (excluding `node_modules/.cache`): 4 files ·
  `wdio.conf.ts:67` no-change (a comment naming the truthiness rule). Sweep `grep -rln "pulse-run-contract" crates/ scripts/`:
  6 files · `drift.rs:60`, `scenario.rs:1417`, `operator-hold.e2e.ts:17` no-change (the P-073 SCENARIO of that name or
  a comment, not the contract).
- **If the scenario names P-033:** `crates/conductor-core/src/drift.rs` (`UNBACKED_AUTO` + its doc comment) +
  `coverage-matrix.md` (regenerated, header `(7 unbacked)`), in the same change (`verification-matrix.json#v3-10` notes).
- `contracts/scenario-audit-ledger.toml` (a row, if over-tier) · `contracts/pulse-real-model-leg-posture.md` (dated
  corrections to its mechanism clauses, grading rule untouched) · `conductor-0.3.0/verification-matrix.json#v3-09`.

## Open questions
- **How is the real-model posture selected?** (P-A) Scenario-declared (the scenario carries a closed posture; the gate
  inverts the L4 term only for it) vs a per-invocation CLI flag vs splitting the gate from the drive. The selector must
  be explicit — deriving it from the env would silently turn a forgotten `L4_DETERMINISTIC=true` into a real-model run
  and downgrade the deterministic suite's refusal. → blocks: plan-decision
- **How does the rank-1 hypothesis reach a gradeable, committable artifact, and may it be committed?** (P-F; security
  extract's open §Data Protection item) A capture tool that re-reads the attributed report over MCP vs a production
  self-obs line vs grading in-process. Any committed literal is corpus-rendered model text. → blocks: plan-decision
- **What is the known cause, and what text counts as identifying it?** (P-G) Must be committed before the first drive;
  the digest bounds it to service + anomaly kind; `conductor` is a prefix of `conductor-canary`. → blocks: plan-decision
