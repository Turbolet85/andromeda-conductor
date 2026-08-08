# v0.2.0 Intent — conductor

**Purpose & how to use this file.** This is the human-authored INTENT for version 0.2.0 — the single
source of truth `/andromeda-route --version 0.2.0` derives `vision.md`, `requirements.md`,
`working-route.md` and `verification-matrix.json` from. Every finding states what we OBSERVED and what
we EXPECT, so route produces precise capabilities and chunks with no guesswork.
**Working pattern: we author the intent → Andromeda derives everything else from it.**

_Authored 2026-08-08, after a 41-day pause, from the 0.1.0 route tail + a cross-project audit against
Pulse's actual v0.3.0 state._

---

## 1. Context — why a new version instead of finishing 0.1.0's tail

0.1.0 delivered the harness: **61 chunks, all complete, 0 pending**, CI-green (workspace nextest
420/420, clippy, doctest). The scenario catalog, the seeded deterministic timeline, the emission
journal, the CLI verbs, the 5-command agent-run harness, the Tauri control panel, the coverage-matrix
view, the operator-pause dialog, the CI quality/obs gates — all shipped.

What it did **not** do is the thing the harness exists for. 0.1.0's own definition of done ends with:

> *"End-to-end proof against a real Pulse: at minimum `error-baseline-spike`, `fingerprint-storm`,
> `restart-suppression` (incl. one bypass case), `pii-scrub`, and `connection-lifecycle` produce
> verified expected outcomes (MCP read-back where applicable), and one full `severity-lifecycle` pass
> observes auto-resolve + resolution summary."*

None of that is proven. The last chunk (`2026-06-27-live-pulse-e2e-proof`) shipped **Part A** only — a
canary fingerprint-fidelity bridge — and deferred Part B because Pulse's incident creation was
LLM-in-the-loop non-deterministic. The live `conductor preflight` ran end-to-end correctly
(connect / negotiate `2024-11-05` / 4 tools / emit / poll all ✓) and still returned
`Blocked: incident not found in corpus`.

**That blocker is gone.** Pulse shipped exactly the leading option during its own v0.3.0: a
deterministic env-gated L4 mode (**P-073**, canned `L4Output`, no GPU/3B) plus Tier1 storm coalescing
so a sustained storm yields exactly ONE incident (**P-074**). Both are `verified` in Pulse's matrix. The
0.1.0 route entry *"Pulse LLM-in-the-loop verification posture — OPEN DECISION"* is therefore
**resolved and retired** — it is not a chunk and does not migrate.

**But Pulse moved while Conductor stood still, and that is the first thing 0.2.0 must fix.** Conductor
is a tool that exists to test Pulse; over the 41-day pause Pulse shipped 22 new capabilities
(P-061..P-082) and Conductor cannot so much as *name* them — its P-ID type rejects anything above
P-060 (§4 Theme 1). A 0.2.0 that proved every 0.1.0 scenario and shipped a release would still be a
harness aimed at a version of Pulse that no longer exists. Synchronization with the SUT is therefore
the first epoch, not a housekeeping afterthought.

Eight markerless entries remained in 0.1.0's route, including its own ship-and-close items. They are a
version's worth of coherent work, and they re-decompose better under fresh epoch calibration than they
sit in a tail where one entry (Part B) had swallowed five scenario families, the emit/extract framework
and the live `ready:true` proof. Two structural gains come free with a new version: a
**`verification-matrix.json`** (0.1.0 predates the feature, so Conductor — a verification harness — has
had no capability ledger of its own), and a sizing gate that will not let a Part-B-sized monolith
through again.

**Version story:** 0.1.0 = the harness is built. 0.2.0 = the harness is aimed at the Pulse that
actually exists, proves it, and ships.

## 2. The core problem (one sentence)

> Conductor can drive and read back a real Pulse, but **has never actually proven one** — and it is
> now aimed at a SUT 22 capabilities out of date, its verification legs unbuilt, its four delegated
> timing budgets structurally unassertable, and its own delivery untracked by any ledger.

## 3. What WORKS today — preserve, build on, do NOT rebuild

0.2.0 is re-aiming, completion and proof on top of a working harness, not a rewrite:

- **Deterministic engine** — `current_thread` tokio runtime + seeded RNG; the JSONL emission journal is
  the agent-parseable ground truth and the left side of every SLO check.
- **Scenario catalog + config** — 31 declarative TOML scenarios keyed to P-IDs, garde-validated; the
  "no scenario without a P-ID" scope law holds.
- **Live-Pulse plumbing** — sidecar spawn (`conductor-verify/src/spawn.rs`), the hand-rolled JSON-RPC
  read-back client (`jsonrpc.rs` — rmcp was removed deliberately), the 4 corpus tools (`client.rs`),
  the pinned `contracts/mcp-contract.toml` (`2024-11-05` + required tools).
- **The canary bridge (Part A)** — `conductor-run::emit_canary` → poll `query_incident_list` → assert
  the fingerprint via `retrieve_telemetry_slice.fingerprint_refs`. CI-green. **Part B reuses it; do not
  re-author it.**
- **CLI + GUI surfaces** — run/suite/report verbs, the 5-command agent-run harness, line-oriented
  output, the Tauri control panel (picker, live Channel counters, coverage-matrix view, run-report and
  operator-checklist views, the go/no-go pause dialog).
- **CI** — quality gates, obs conformance gate, artifact uploads, the a11y harness (authored and
  shipped, display-gated).

## 4. Findings — OBSERVED vs EXPECT

### Theme 1 — Synchronization with the SUT (KEYSTONE — everything else is worthless without it)

- **F1 · The P-ID type hard-rejects Pulse's current capabilities.** OBSERVED:
  `crates/conductor-core/src/scenario.rs:23-33` validates `P-NNN` with `(1..=60).contains(&n)` and
  returns `"expected P-NNN with NNN in 001..=060"`; the unit test at `:195` asserts `"P-061"` MUST be
  rejected. Pulse's ledger now reaches **P-082**. A scenario naming P-074 fails garde validation at
  load — Conductor cannot express, classify or verify anything Pulse built in v0.3.0.
  EXPECT: the accepted capability set tracks the SUT instead of a compile-time constant. Preferred
  shape: source it from a versioned SUT capability artifact under `refs/` (the precedent exists —
  `.andromeda/refs/pulse-v0_2_0-capability-audit-2026-06-12.md`), so a Pulse release is a **data**
  update, not a code change, with a drift check that fails loudly when the SUT advances past what
  Conductor knows. Widening the constant to `001..=082` is the minimum and re-freezes the same defect
  one version later — take it only as an explicit interim, never as the answer.
- **F2 · The coverage universe is frozen at 60.** OBSERVED: `crates/conductor-core/src/coverage.rs`
  enumerates exactly P-001..P-060; the scenario catalog's maximum is P-060; 0.1.0's definition of done
  reads "all **60** P-IDs classified, zero gaps". Nothing in the repo knows P-061..P-082 exist.
  EXPECT: every current Pulse capability is classified — **auto / drive+observe / static-only /
  not-Conductor's** — and the boundary is a recorded decision, not an accident. Conductor's standing
  non-goal ("NO UI automation of Pulse — visual claims are operator-checklist items") means most of
  Pulse's v0.3.0 work is legitimately **Pulse's own** to verify: the UI/visual set
  (P-061..P-066, P-068..P-071, P-080..P-082) via its webview suite and its P-076 tauri-driver e2e, plus
  P-078 (its agent-headful self-verify harness) and P-077 (the demo injector), which are Pulse's own
  tooling. Say so explicitly, so the gap is deliberate. The ones that fall squarely in Conductor's lane
  and MUST be covered:
  - **P-074** — a sustained identical-fingerprint storm yields exactly ONE incident. This is a
    quintessentially Conductor-shaped claim and the natural extension of the `fingerprint-storm`
    family; Pulse verified it with its own integration test only.
  - **P-079** — the incident workspace key is single-sourced. Externally observable via read-back, and
    it is the very defect that currently blocks Conductor's live legs (F10) — verifying it closes the
    loop.
  - **P-073** — the deterministic L4 mode. Conductor depends on it; a run contract that asserts it is
    active (F11) is that verification.
  - **P-072** — Investigate actions produce a real L4-backed result; candidate drive+observe via report
    read-back.
  - **P-067** — live-only service truth; partially observable through the service-registry surface.
- **F3 · Deterministic L4 proves the pipeline and bypasses the product's core value.** OBSERVED: with
  `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` the canned `L4Output` replaces the model, so every live leg
  0.2.0 plans exercises the plumbing and never the interpretation. Pulse's own positioning memo names
  interpretation-correctness — "inject a known root cause, verify the top hypothesis identifies it"
  (the P-033 Conductor path) — as *"the single most important test in all of Pulse"*, because for a
  non-specialist a confident-but-wrong explanation is more dangerous than raw lines.
  EXPECT: a conscious decision, recorded either way. Either 0.2.0 carries one **real-model**
  correctness leg (deterministic mode OFF, known root cause injected, top hypothesis asserted), or the
  gap is explicitly deferred with a named owner and a note that "Conductor green" does not mean
  "interpretation is trustworthy". Do not let determinism silently redefine what the harness proves.
- **F4 · The SUT has a known failure mode that bounds scenario design.** OBSERVED: Pulse records that
  the DuckDB append path stalls after ~10 minutes of sustained 10k/s storm + deterministic L4 — ingest
  keeps receiving while `duckdb.append` stops, so `viz.query.traces` returns 0 rows and no new
  incidents form until restart. Conductor's whole method is sustained shaped load.
  EXPECT: scenario durations and storm profiles stay inside the SUT's proven-good envelope, the
  constraint is recorded where scenario authors will see it, and a run that exceeds it is flagged as
  environment-suspect rather than reported as a Pulse failure.

### Theme 2 — The live proof (the reason Conductor exists)

- **F5 · Coarse emission.** OBSERVED: `conductor-run::coarse_emit` sends one signal per phase; it cannot
  shape the per-family telemetry the scenarios describe. EXPECT: a per-phase dispatcher over the
  `conductor-emit` primitives, faithful to each scenario's declared shape and extensible to the deferred
  families. (0.1.0 chunk plan, Stage 1 step 6.)
- **F6 · Placeholder read-back.** OBSERVED: `execute_scenario` writes the literal token
  `"incidents-listed"` as the observed value, so `evaluate_check` grades a placeholder rather than the
  SUT's actual response. EXPECT: real per-check extraction from `query_incident_list` /
  `retrieve_telemetry_slice`, feeding the existing `evaluate_check` → `classify` path unchanged, with
  `execute_scenario`'s signature preserved. (Stage 1 step 7.)
- **F7 · Never green.** OBSERVED: no run has ever produced live `ready: true`; the headline acceptance
  of the 0.1.0 e2e chunk was never achieved. EXPECT: a live `ready: true` against a real Pulse in
  deterministic-L4 mode, recorded with its journal + `runs.db` evidence.
- **F8 · The five families are unproven.** OBSERVED: only the canary path has touched a live Pulse.
  `fingerprint-storm` (P-017/P-018), `error-baseline-spike` (P-009..P-012), `restart-suppression`
  (P-015/P-016/P-057, incl. one bypass case), `pii-scrub` (P-035/P-047/P-048) and
  `connection-lifecycle` (P-001..P-004 + the `PortOccupier` fault) have never been driven end-to-end.
  EXPECT: each family emits faithfully, reads back through MCP, and yields a verified expected outcome;
  authored tokens and SLO tiers are re-calibrated against live behavior where they prove wrong. The
  `fingerprint-storm` family additionally carries **P-074**'s exactly-one-incident assertion (F2).
  _Sequencing note: fingerprint-storm + error-baseline-spike were the 0.1.0 Stage-2 target; the other
  three were its fast-follow set. Route may split them across chunks — they need not land together._
- **F9 · Severity lifecycle unobserved.** OBSERVED: auto-resolve and the resolution summary have never
  been observed via read-back. EXPECT: one full `severity-lifecycle` pass observes both.
- **Constraint carried from 0.1.0:** the live legs are **operator-gated** (`workflow_dispatch` / local
  invocation), **never a CI gate** — CI has no Pulse. Determinism goldens and stub-backed tests remain
  the CI-side coverage.

### Theme 3 — The cross-project blocker (a Pulse defect, discovered here) — PREREQ

- **F10 · The workspace filter key diverges between Pulse's app and Pulse's own MCP sidecar.**
  OBSERVED (code read, 2026-08-08 — **verify live before designing around it**): incidents are filtered
  `WHERE workspace = ?1` (`crates/corpus/src/contract.rs:628`). `pulse-app` derives that key via
  `resolve_workspace_for_incidents` (`pulse-app/src/main.rs:688-693` →
  `pulse-app/src/digest_runtime.rs:109-127`) = the **detected project root** whenever
  `workspace_detector::detect(cwd)` succeeds, falling back to `data_dir` only when detection fails. The
  **sidecar** always uses `data_dir` (`crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:74`, whose
  comment still claims parity with `main.rs` — stale since Pulse's P-079 moved the app to the detected
  root). Consequence: if `pulse-app` is launched anywhere the detector recognises,
  `query_incident_list` returns ZERO rows and `conductor preflight` reproduces
  `Blocked: incident not found in corpus` **even with deterministic L4 on**.
  EXPECT: the live legs run against a Pulse whose app and sidecar agree on the workspace key. The real
  fix is **Pulse-side** (align the sidecar with `resolve_workspace_for_incidents`) and is a blocking
  external prerequisite, not Conductor work — but Conductor MUST detect the condition and fail with a
  named precondition rather than an opaque `Blocked`. An interim unblock exists (launch `pulse-app`
  from a directory where detection fails, so both sides fall back to `data_dir`), but it empties the
  digest project context and degrades context-grounding assertions (P-036); use it for a first live
  signal only, never as the standing configuration.
  _First action of this version: run the two-launch check (repo root vs. marker-less temp dir) and
  record the verdict. It decides whether a Pulse chunk gates the live legs._
- **F11 · The run contract is nowhere recorded.** OBSERVED: nothing in this repo states how a
  verifiable Pulse must be launched; `conductor-verify/src/spawn.rs:18` passes only
  `ANDROMEDA_PULSE_DATA_DIR`, and there are zero references to Pulse's deterministic mode anywhere in
  the tree. EXPECT: a recorded, reproducible, machine-checked run contract —
  `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` belongs on the **launched `pulse-app`** (the OTLP receiver
  that CREATES incidents), **not** on the MCP sidecar Conductor spawns; both sides must resolve the
  same data dir; in deterministic mode the model prerequisites (`ANDROMEDA_PULSE_MODEL_PATH`,
  `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`) do not apply. Preflight asserts the contract and names what is
  unmet. This doubles as **P-073**'s verification (F2).

### Theme 4 — The four delegated timing capabilities

- **F12 · The budgets are structurally unassertable.** OBSERVED: `SloTier` is a closed three-value set —
  5s / 20s / 90s (`crates/conductor-core/src/scenario.rs:36-61`) — while Pulse delegates **P-025** halo
  hue ≤2s, **P-027** constellation discovery ≤5s, **P-037** report render ≤2s and **P-045** counter
  refresh ≤1s. Only P-027 fits. All four scenarios carry empty `expected` (DriveObserve / ManualCheck),
  and the only latency recorded is the coarse whole-loop `observed_ms - emitted_ms`
  (`conductor-run/src/lib.rs:208`), not per-check.
  EXPECT: the four budgets are assertable at their real values — a sub-5s budget representation (extend
  the tier set, or a per-scenario budget field) plus **per-check** latency measurement, so a run yields
  a real Pass/Fail per capability rather than a ManualCheck.
  _This is what closes Pulse's **P-075**, whose acceptance names all four budgets explicitly. If route
  splits F12 from Theme 2, the plan must say out loud that P-075 stays open until both land._

### Theme 5 — Conductor's own verification ledger

- **F13 · The harness has no ledger.** OBSERVED: `conductor-0.1.0/` predates the verification-matrix
  feature, so every matrix step in every skill is a noted no-op here; Conductor's own delivery has never
  been coverage-tracked. The 60 Pulse P-IDs live in `coverage.rs` + `scenarios/*.toml` — a **different**
  mechanism, tracking the SUT's capabilities, not Conductor's.
  EXPECT: 0.2.0 carries a `verification-matrix.json` (route emits it), and the two id spaces stay
  visibly separate — see §6.

### Theme 6 — Ship and close (0.1.0's migrated definition of done)

- **F14 · Desktop a11y is authored but never run.** OBSERVED: the harness and the real-webview specs
  shipped at `2026-06-27-desktop-a11y-harness-setup` (`crates/conductor-tauri/ui/wdio.conf.ts` +
  `ui/test/a11y/*.e2e.ts` — axe-core / colorjs.io / keyboard over `@crabnebula/tauri-driver`,
  DISPLAY-gated); they have never been executed. EXPECT: run them on Linux+xvfb against a live Pulse and
  ASSERT — zero axe violations, token-pair contrast, keyboard-trap and focus-order across the four
  accessible paths — plus an NVDA/VoiceOver manual spec. **Do NOT re-author the harness**, and do not
  redo the deterministic `tauri::test` mock-runtime command tests or the test-plan Path-7 cross-surface
  parity checks — both already landed (10/10).
- **F15 · No a11y CI gate.** OBSERVED: a11y results do not reach the obs envelope. EXPECT: axe /
  contrast / keyboard PASS/FAIL emitted into the obs envelope, service-tagged, gated in CI. Reuse the
  `ci.yml` obs-gate scaffold from `2026-06-27-obs-ci-conformance-gate` — the `shell: bash` gate-step
  idiom (jq/grep assertions + `::error::` annotation + non-zero exit) plus the
  `actions/upload-artifact@v4` `if: always()` upload. The violation JSON is **Conductor's own** artifact,
  so the redaction boundary applies. Unlike the obs gate this is **not** Windows-doable — the harness
  that produces the violations is display-gated (Linux+xvfb).
- **F16 · Cross-surface parity unproven.** OBSERVED: CLI and Tauri share a core but their envelopes have
  never been compared for the same seed. EXPECT: an identical-envelope proof, CLI vs Tauri, same seed.
- **F17 · The coverage matrix has never been gated.** OBSERVED: the P-IDs are classified in code but
  nothing fails when a gap appears. EXPECT: a completeness gate — zero-gap over the **current** SUT
  capability set (per F1/F2, no longer a hardcoded 60), all CI gates green. This was 0.1.0's stated
  definition of done and is inherited here.
- **F18 · Never released.** OBSERVED: no release build, no bundle. EXPECT: `cargo build --release` + a
  Tauri 2 bundle, with a final SLO verification pass.

### Theme 7 — Housekeeping carried forward (would otherwise be lost with the handoff)

- **F19 · Stale `rmcp` wording** survives in `test-plan.md` body, `tests-summary.md:44` and
  `verification-harness.md` (~lines 18/33) — rmcp was removed in favour of the hand-rolled client.
  EXPECT: reconciled at the next test-plan-touching chunk.
- **F20 · Operator-pause has never fired live**, and the operator-checklist live items were gated behind
  the (now resolved) posture decision. EXPECT: both exercised during the live legs.
- **F21 · Coverage view lacks live per-P-ID verdict lamps**, and there is no `scenario.run` root obs
  span. EXPECT: both landed.
- **F22 · Dormant maintenance:** `indicatif` 0.17→0.18; `opentelemetry-proto default-features=false`
  trim. EXPECT: folded into a polish chunk, not a chunk of their own.

## 5. Scope, priorities, non-goals

- **Epoch 1 — re-aim at the SUT (keystone):** F1, F2, F4. Cheap, and until it lands every downstream
  proof is aimed at a stale target. F3's decision is taken here even if its execution is deferred.
- **Epoch 2 — make the live path real:** F10/F11 (run contract + blocker verdict), then F5–F7 (the
  emit/extract framework and the first live `ready: true`).
- **Epoch 3 — the proof:** F8 families (incl. P-074), F9 severity lifecycle, F12 timing budgets.
- **Epoch 4 — verify and polish:** F14–F16, F20, F21.
- **Epoch 5 — ship:** F17 completeness gate, then F18 release.
- **Non-goals carried from 0.1.0, unchanged:** not a load-tester; no Pulse process management (launching
  Pulse stays an operator-pause step); no UI automation of Pulse; no scenario DSL; no multi-target /
  distributed / cloud; **"no scenario without a P-ID."**
- **New non-goal:** Conductor does **not** fix Pulse. F10's real remedy is a Pulse chunk; Conductor
  records the requirement, detects the misconfiguration and fails loudly.
- **Explicit boundary:** Pulse's v0.3.0 UI capabilities (P-061..P-066, P-068..P-071, P-080..P-082) plus
  its own tooling (P-077 demo injector, P-078 self-verify harness) are **not** Conductor's to verify —
  they belong to Pulse's webview suite and its P-076 tauri-driver e2e. F2 records that as a decision so
  the coverage gate does not read it as a gap. Every P-ID from P-061 to P-082 is therefore accounted
  for: Conductor's lane = P-067, P-072, P-073, P-074, P-079; Pulse's own = the rest; P-075/P-076 are the
  delegation itself.

## 6. Capability id scheme (read this before minting ids)

0.1.0's `requirements.md` carries **no** numbered ids, so there is no project scheme to continue; use
the version-scoped form (`v2-NN`).

**Two id spaces coexist in this repo and must not blur:**

- `v2-NN` — **Conductor's own** capabilities. These are what `verification-matrix.json` tracks.
- `P-NNN` — **Pulse's** capabilities, the things Conductor proves. They appear **only inside acceptance
  criteria** ("…proving Pulse P-017/P-018"), in `coverage.rs` and in scenario configs — never as
  Conductor's own ids, and never as matrix entries.

Pulse's own v0.3.0 ledger reaches P-082; nothing here may mint into that space.

## 7. Definition of done (what "v0.2.0 works" means)

Conductor knows the Pulse that actually exists: every current Pulse capability is classified, the ones
in Conductor's lane are covered by scenarios, and the SUT-drift check fails loudly when Pulse advances
again. A real Pulse is launched under a machine-checked run contract; Conductor drives it and —
programmatically, via MCP read-back — proves the five families (including P-074's exactly-one-incident
claim) and one full severity-lifecycle pass, reaching live `ready: true` with journal and `runs.db`
evidence. The four delegated timing budgets (P-025 ≤2s, P-027 ≤5s, P-037 ≤2s, P-045 ≤1s) return real
Pass/Fail at their actual values, closing Pulse's P-075. The interpretation-correctness question (F3)
has a recorded answer. Desktop a11y runs and asserts on Linux+xvfb with its violations gated in CI; CLI
and Tauri produce an identical envelope for the same seed; the coverage matrix is gated zero-gap with
all CI gates green; and a release build plus Tauri bundle ships with a final SLO pass.

## 8. What we expect Andromeda to derive from this intent

- `vision.md` — the §1–2 framing (the harness is built; 0.2.0 re-aims it at the real Pulse, proves it,
  and ships).
- `requirements.md` — the §4 findings as numbered `v2-NN` capabilities, each carrying its
  OBSERVED→EXPECT acceptance shape.
- `working-route.md` — capabilities sequenced under the §5 epoch headers. Route may resequence by
  dependency, but Epoch 1 must precede every live-proof chunk: a proof aimed at a stale capability set
  is not a proof.
- `verification-matrix.json` — one entry per capability, `v2-NN` ids verbatim. Note for method
  classification: the live-Pulse legs are operator-gated `dynamic-external`, not CI gates.
