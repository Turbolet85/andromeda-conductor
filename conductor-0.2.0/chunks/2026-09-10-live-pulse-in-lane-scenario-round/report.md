# Report — 2026-09-10-live-pulse-in-lane-scenario-round

**Chunk:** Live-Pulse in-lane scenario round — P-067, P-072 and P-079 each driven against an operator-launched live Pulse to a non-blocked verdict, with journal and runs.db evidence per scenario
**Date:** 2026-09-10
**Commits:** none since `last_wrap` (2026-09-10T10:21:45Z, `ecf89a8`) — all of this chunk's work is uncommitted and rides this wrap's commit.

## Changes (structured — detectors read this)

- **Files:**
  - *New (4):* `conductor-0.2.0/chunks/2026-09-10-live-pulse-in-lane-scenario-round/evidence/{p-067.jsonl, p-072.jsonl, p-079.jsonl, live-round-verdict.md}` — three run-report envelope lines verbatim plus the verdict record.
  - *Chunk artifacts (new, untracked):* `chunks/2026-09-10-live-pulse-in-lane-scenario-round/{scope.md, research.md, plan.md, report.md}`.
  - *Modified:* `conductor-0.2.0/verification-matrix.json` (claim at phase P5, `implement` at /implement, `verified` at this P7) · `.andromeda/master-route.md` (promotion append only) · `conductor-0.2.0/working-route.md` (freeze stamp only) · `.andromeda/friction-log.ndjson` (evolve telemetry).
  - **ZERO source delta** — no `.rs`, no `.ts`/`.tsx`, no `Cargo.toml`, no `Cargo.lock`, no `scripts/`, no `scenarios/`, no `.github/`. Basis: `git status --short` + `git diff --name-only HEAD` at wrap Setup.
- **Symbols / APIs:** none added or changed. No public fn, IPC method, endpoint, export, port, socket or env var was introduced. The round READ existing surfaces only (`conductor run <scenario> --agent-mode`, the MCP read-back client, `runs.db`).
- **Crates / modules:** none added · none removed · none changed.
- **Dependencies:** none added · none bumped. `Cargo.lock` byte-unchanged; package count 562 (basis: `cargo audit` line "Scanning Cargo.lock for vulnerabilities (562 crate dependencies)", gate log `3.log`).
- **Schema / config:** none. No migration, no config key, no violation schema, no scrub/redaction shape.
- **Spec-master edits:** none — no `.andromeda/` master body was changed by this chunk.
- **Counts / qualifiers moved:** **none — verified.** No documented derived value changed. Explicitly checked: `UNBACKED_AUTO` unchanged (no capability classification moved), the scenario catalog unchanged (no file added/removed under `scenarios/`), the coverage matrix's verified count moves by the ledger flip alone (28→29 of 32), which is the matrix's own bookkeeping and not a value any spec bakes. The advisory figures DID move (below) but are recorded in a Tier-2 rules file, not in any of the seven masters.
- **Dev-tool versions:** none installed or upgraded.
- **Harness / gate surface:** **none changed.** `scripts/agent-run.{sh,ps1}` untouched; the five-command discipline, the `--live` stage flag and `live_suite()`'s leg order are byte-identical. The three legs were driven as `conductor run <scenario> --agent-mode` gate entries, which adds no verb, no stage flag, no port, no `CONDUCTOR_*` handle. The chunk's OWN `plan.md` gate fence changed (see Deviations) — that is a chunk artifact, not the project's harness surface.
- **Cross-project / external claims:** **yes — three, all read from the `andromeda-pulse` repo at HEAD `83d4060`** (basis: direct source read this session, plus a whole-repo enumeration):
  - `EvidenceRefs.span_ids` is `Vec::new()` at the only production incident-construction site — `pulse-app/src/inference_runtime.rs:871`; nothing mutates it after (enumeration of every `span_ids` write: the only non-empty ones are `crates/mcp-server/src/tools.rs:868`, a unit-test fixture, and `crates/interpretation/src/markdown.rs:507`, `vec![]`).
  - `dispatch_retrieve_telemetry_slice` maps `span_refs` 1:1 from that field — `crates/mcp-server/src/tools.rs:430-439`.
  - `DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS = 120` — `crates/triage/src/incident/persistence.rs:42`.
  - Live corroboration: `retrieve_telemetry_slice` returned `result_count: 0` on every leg (gate logs `7-9.log`).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** **one, and it is scenario-level, not spec-master-level.**
  - `scenarios/constellation-severity-live-wiring.toml` states as a calibration point that its `slo_tier = "<20s"` "tracks storm detection, not the L4 cadence" (`:33`). MEASURED FALSE as an attainable tier: the scenario's own phases sum to 30 s (`gap_ms` 3000 + 15000 + 12000 at `:29`/`:36`/`:44`) and `latency_ms` is `read_back_observed_at − journal_emitted_at`, which spans the whole emission window — so the measured 30 212 ms can never fall under the declared 20 000 ms deadline. Evidence: the `run_check` row of run `2026-09-10T15-11-26-937` (`latency_ms 30212`, `deadline_ms 20000`), committed at `evidence/live-round-verdict.md`. **No `.andromeda/` master states this tier**, so no master amendment follows; the disposition is the owned residual below.
  - **A SECOND claim, in a MASTER, surfaced during P2 and DISPOSED there (amendment applied).** `obs-plan.md:349` glossed `read_back_observed_at - journal_emitted_at` as "(Conductor's MCP round-trip)". MEASURED FALSE: the delta covers the scenario's whole emission window — three legs of 18s / 35s / 30s of declared phases recorded `latency_ms` 18169 / 35120 / 30212, while the sidecar's own tool calls logged `duration_ms` 0–22 in those same legs. No detector proposed it (all seven returned `proposals: []`; the obs detector surfaced it as an adjacent observation explicitly outside its invariants), so the orchestrator raised it under Validate check 5, playbook `:28` governing it as routine. The passage's conclusion is unchanged and strengthened. This bullet under-ran at authoring time: the report substantiated the fact but did not name it as a disproved claim, and P2 caught it.
  - NOT disproved, CONFIRMED: the always-false `CountAtLeast >= 1` over `span_refs` was already recorded at `.andromeda/architecture-amendments.md:403` ("`CountAtLeast` grades `span_refs` the incident producer writes empty", 2026-08-21). This round measured it live rather than falsifying anything.
  - A stale LEAF, master tier already correct: `.claude/rules/verification-harness.md:47` still says the deterministic-L4 fixture pins `evidence_refs` to `[]`. Pulse retired that (the fixture now populates a `det-*` triple, and this round's envelopes carry it). `architecture.md:63`, `obs-plan.md:314-315` and `test-plan.md:76/335` all already carry the corrected form (basis: `grep -rn 'evidence_refs' .andromeda/*.md`), so the master tier is right and only the rules file lags — a cascade gap for curation, never a master amendment.
- **Expected amendments (from plan):** the plan's `## Implementation notes` lists **"none to any of the seven masters"**, with the basis that both relevant arch sites were re-measured true at phase. Dispositioned: **not carried — correct as listed.** Re-verified at this wrap: `architecture-amendments.md:403` states the `span_refs`-written-empty mechanism (grep `'span_refs' .andromeda/*.md` → 1 hit, that line) and `architecture.md:69` records the accepted grading outcome for `findings-counter-refresh` (grep `'findings-counter-refresh' .andromeda/architecture.md` → line 69, 2 occurrences; line 63 has 0). Neither needs changing. No master is named as owner of any fact in this report.
- **Coverage of new surfaces:** **none — this chunk introduces no external surface, hot-path op, or UI element.** The three scenarios and the read-back path are pre-existing and previously covered; the round drives them, it does not add them.

## Deviations from intent

1. **Three gate `expect` atoms were unsatisfiable as authored, and were corrected in `plan.md` at this wrap — BEFORE the light gate — on the operator's directive.** Each live leg carried `contains P-067` / `P-072` / `P-079`, but the CLI's verdict line prints the SCENARIO NAME (`[MANUAL] live-only-service-truth`, `[MANUAL] investigate-actions-functional`, `[HOLD] constellation-severity-live-wiring`); `grep -c 'P-0'` over every leg log returns 0, while `grep -c 'BLOCKED'` also returns 0, so the substantive atoms held throughout. Corrected to `contains {scenario name}` (each now measured present exactly once in its leg's log). Justification: the plan's own provenance header sanctions edits between runs, the light gate re-runs entries verbatim and would have re-failed on the same unsatisfiable atom, and the P-ID linkage already lives where it belongs — `p_ids` in the journal envelope and the `runs.db` row.
   **Authoring cause, recorded because it is the useful half:** a `leg` entry is exempt from P5's baseline run by the `new`/`baseline` invariant ("P5 baseline-runs every `new` entry that is not a `leg`"), and the baseline run is exactly what catches an unsatisfiable expectation on every other entry. So an `expect` atom on a live leg has **no mechanical check at all** today — not at P5 authoring, not at P5 validation. The atom was written from the layouts extract's *described* row shape (`? P-0NN  <scenario>  Manual`) rather than from measured output.
2. **P1 wrote 0 files; the plan's New-files work executed after P2.** Every evidence file's content is produced by the live legs, so step 6 is dependency-ordered behind the gates. Justification: unavoidable for a zero-source-delta round whose artifacts the gates emit; no file was written speculatively.
3. **The source-delta gate deferral was NOT invoked.** With zero Rust delta the rule permits deferring `cargo nextest --workspace` and `clippy`; both were run instead (902/902, clean). Justification: release-adjacent chunk, warm build, and a green workspace beside the live evidence is worth more than the saved minutes. No deferral is owed forward.

## Decisions & corrections

- **Operator, P5 review (4 items):** the three-file `CountAtLeast` basis must be stated as live `kind =` keys with the seven-file token-grep trap named; `architecture.md:63` was a **mis-aimed citation** (0 occurrences of `findings-counter-refresh`; the clause is at `:69`, offset ~5712 of a 10 432-char line) and the corrected form is honestly weaker — acceptance-in-practice, not an explicit ruling; **evidence durability** was missing because `/runs/` is gitignored, so step 6 + a fourth probe were added; and the SUT must stay up through the wrap.
- **Operator, wrap directive (4 items):** correct the three atoms before P7.1 and disclose the edit; route P-079's two defects as ONE owned item into `.andromeda/residuals.md` as `open` (cross-version channel) rather than a new 0.2.0 entry; extend the existing advisory clause inside `.claude/rules/security.md:52` rather than adding a bullet; do not stop pulse-app.
- **Measured, not inferred, throughout:** the SUT's booted posture was read from Pulse's own log (`app.boot.workspace_key` 14:54:21.247Z, the two OTLP binds 14:54:21.915Z; `grep -c 'triage.baseline.bootstrap_window.override'` → **0**, so the DEFAULT window was in force).
- **A token count that needed reading, not trusting:** `grep -c 'service_went_silent'` over Pulse's log returns 1177, but every hit is `triage.baseline.service_went_silent.evaluate` — "service-went-silent evaluation cycle", the evaluator RUNNING. **Zero silence cues were emitted.**

## Outcome

**Acceptance criteria, re-asserted against the diff:**

- **(matrix) `verification-matrix.json#v2-04`** — MET. Three scenarios driven live to non-blocked verdicts with journal + `runs.db` evidence each; probes read `non-blocked rows: 3` and `journal envelopes non-blocked: 3`.
- **(arch) eleven-field envelope + `runs.db` row per leg, zero `run_check` rows for the two declare-only scenarios and per-check rows only for P-079** — MET, measured **0 / 0 / 1**.
- **(tests) evidence survives the commit** — MET: four non-empty files under `evidence/`, asserted by the `committed evidence files: 4` probe. Diff confirms they are new tracked files, not under gitignored `/runs/`.
- **(arch) no live-Pulse leg added to any CI job; sidecar resolved on `PATH`** — MET; `.github/` untouched in the diff, and the `which andromeda-pulse-mcp` probe ran green before any leg.
- **(obs) eleven envelope keys, closed-set `state`/`verdict`, `state` ≠ `Blocked`** — MET on all three.
- **(design) P-067/P-072 render `[MANUAL]` or `[RESIDUAL]`, never `[FAIL]`/`[BLOCKED]`; P-079 renders its own machine-verdict lamp** — MET: `[MANUAL]`, `[MANUAL]`, `[HOLD]` (the `CalibrationRegion` lamp). No new label minted.
- **(layouts) harness stage-flag set and five-command discipline unchanged** — MET; `scripts/` untouched in the diff.
- **(security) no artifact carries an absolute host path, seam-crate struct name, or `ANDROMEDA_PULSE_*` value** — MET (the committed envelope lines carry only run identity, verdict/state, timings, tier and fingerprint refs).
- **(security) readiness gate intact — no preflight arm relaxed, skipped or re-graded** — MET; every leg paid its own preflight and the `preconditions` probe ran non-priming ahead of them.
- **(tests) every leg exits 0 on a non-`Fail` row; no pass condition read off an exit code** — MET; the `lacks [BLOCKED]` atom plus the two artifact probes carry the verdict.
- **(a11y) no a11y coverage claimed; the `conductor` verb SET's not-assertable verdict left intact** — MET, nothing claimed.

**Gates** (the 14 `[[gate]]` entries by `run`, in order, as re-run by this wrap's light gate — see P7):
`cargo nextest run --workspace --profile ci` · `cargo clippy --workspace --all-targets -- -D warnings` · advisory-db porcelain · `cargo audit` · `cargo deny check …` · `which andromeda-pulse-mcp` · `conductor -- preconditions` · **three `leg = 'live'` entries** · runs.db probe · journal probe · evidence-durability probe · `agent-run.sh status`. No `defer` entry exists; none was voided. **Smoke:** the `role='smoke'` entry ran as a P2 gate and returned this round's own leg-3 `run_id` (mint-then-read, not residue).

**Outcome basis:** /implement's P4 report **as given**, plus an operator wrap directive issued after it (which added the pre-gate `plan.md` atom correction, the residuals routing for P-079, and the `security.md` clause extension), plus this wrap's own re-derivations (the `P-0`/`BLOCKED` leg-log counts, the `architecture.md:63` vs `:69` citation check, the advisory figures). No claim here rests on a superseded implement-report statement.

**Process hygiene** (from /implement's P4 census, re-measured at this wrap where the host list is readable):

| process | started by | final state |
|---|---|---|
| `pulse-app` (PID 33508) | the operator, for this run | **left running** — required through this wrap's light gate; the operator stops it afterwards via `bash /d/dev/tools/pulse-stop.sh` |
| `andromeda-pulse-mcp` (one per leg) | each leg | terminated — measured: no surviving process, so each entry's `stop` correctly never fired |
| `conductor`, `msedgedriver` | this run / — | none surviving |
