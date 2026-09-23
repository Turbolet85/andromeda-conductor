
## 2026-09-22-interpretation-proven-live — the posture's absence arm, the capture's crossings, and the one ratified corpus exception

**Section:** Threat Model Summary → Data classification (corpus note) · §Input Validation rows: Scenario config · Committed SUT-facing manifests · the `ANDROMEDA_PULSE_*` READ SET · Env-var path handles · NEW Real-model capture ingest · `ANDROMEDA_PULSE_DATA_DIR` spawn · CLI arguments · §Data Protection (encryption at rest) · §Bootstrap phases (`input-validation-library-install`) · §Security Anti-Patterns → Input (the per-READER clause) · → Data Protection (the corpus bullet) · → Universal (the probe short-circuit)

**Change:**
**(S4)** scenario-config row — the top-level `l4_posture` key: a CLOSED unit enum with no catch-all, unknown ⇒ load-time `CoreError::Config`; its `garde(skip)` is the closed-unit-enum form, not the banned nested-spec skip.
**(S3)** committed-manifests row — every SHELL term (`shell-declaration`, `shell-absence`) names its env var; a term's optional `posture` comes from the closed `L4Posture` set.
**(S1 + S2)** READ SET row and the Universal probe bullet — the deterministic posture keeps the declaration grading; under the real-model posture `handle_declared_for` requires the L4 handle ABSENT or falsy under `flag_declared_on_either_side` (Conductor's `true`/`1` ∪ Pulse's `1`/`true`/`yes`), a union that can only widen what BLOCKS; the run-contract path reaches value-only tests alone; the `run --live real-model` arm leads with `preconditions --for real-model-interpretation`.
**(S5)** CLI-arguments row — `conductor preconditions --for <SCENARIO>` (a scenario name through the validated load; a `P-NNN` value refused at parse, exit 2) and the `run --live [real-model]` selector (a closed allowlist; unknown ⇒ usage + exit 2 before any probe).
**(S6 + S7) ESCALATED, operator "Record all + route"** — a NEW Real-model capture ingest row (untrusted SUT output: an MCP re-read and Pulse's own log, through `redact_value` + `mask_host_paths`, fields-only, fingerprints elided, written once); the data-dir row records the capture as a TEST-binary VALUE reader with no canonicalize, a residual.
**(S8) ESCALATED, same decision** — the env path-handle row records two more `CONDUCTOR_RUNS_DIR` test-binary readers joining with no `resolve_under`: `real_model_live.rs` (failures name the display name + error kind) and the pre-existing, previously unrecorded `live_suite.rs` (its capture-read panic prints the resolved path into test output, never a committed artifact). Hardening both joins is route-owned (a working-route CARRY).
**(S9 + S10 + S11) ESCALATED, operator "Ratify, scoped"** — the corpus ban gains ONE ratified, scoped exception: corpus-rendered model text the real-model capture re-reads over MCP may enter a chunk's committed `evidence/` tree, only through `redact_value` + `mask_host_paths` with fingerprints elided; the two dependent passages (Data-classification note, encryption-at-rest paragraph) carry the same exception. The 2026-09-23 capture carries none.
**Cascade folds** — the READ SET row's "no value ever becomes a path" scoped to its own read; the Bootstrap phase and the Input anti-pattern's per-READER enumerations gain the two capture readers.

**Why:** report §Changes (Symbols/APIs, Schema/config, Harness, Coverage of new surfaces) and the plan's Expected amendments (all five §Input Validation rows + the Data Protection ratification). S1/S4/S5 applied under the plan's P5-approved direction with no governing rule — the operator minted rules for all three classes this wrap. S6–S11 are the Boundary-widening class (a new crossing of SUT data into the repo, and a handle's value becoming a path), so they escalated and are ratified here.

**Sweep (full ledger `.andromeda/runs/2026-09-23T08-03-55-wrap/reconcile-sweep.md`):**
- **G7 corpus ban** — 8 hits: `:46` ×2, `:157` ×2, `:335` ×2 amended; architecture `:113` ×2 amended there.
- **G8 test-binary readers** — 10 hits: `:115` ×2, `:121`, `:221` ×3, `:325` ×4, all amended.
- **G10 data-dir value** — 5 hits: `:114`, `:122` amended; `:119` (another handle), `:326` (the argv/shell ban — the capture uses neither) and test-plan `:89` left.
- **G3 grading** — `:114` ×7, `:122`, `:380` ×3 amended; `:119` another handle.
- **G2 encrypted** — `:38`, `:46`, `:153`, `:261`, `:336`, `:381` all state plaintext or an unencrypted `runs.db` — left.
- **G9** — `:327` (refuses before any leg spawns) left, true of both shapes.
- Leaves re-derived: `docs/security-summary.md` (per-reader, capture ingest, `l4_posture`, shell terms, corpus exception) and `.claude/rules/security.md` body (corpus exception, `l4_posture`, the READ SET absence arm, capture ingest + residuals). Session Additions untouched.
