# Security Plan — Amendments

_Append-only changelog of amendments to `security-plan.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-15-config-validation-surface — garde pinned 0.23.0 → 0.22.1
**Section:** §Input Validation (scenario-config boundary row) · §Bootstrap phases (input-validation-library-install)
**Change:** garde version was 0.23.0; now 0.22.1 in both spots. The validation contract is otherwise unchanged: garde `#[derive(Validate)]` + `#[garde(custom)]` at load; the `CONDUCTOR_*` path handles canonicalize-and-bounds-check at the edge, OUTSIDE garde.
**Why:** mirrors the arch §Stack downgrade — `garde_derive 0.23.0` is absent from the registry (latest 0.22.1), so garde 0.23.0 + the `derive` feature is unbuildable; user-authorized.
**Kept:** §Dependency Security carries no garde version pin, so it was left untouched.
**Ref:** .andromeda/runs/2026-06-15T15-45-15-wrap/

## 2026-06-15-dependency-audit-gate — audit-tool versions are minimum floors; toolchain bump confirmed done
**Section:** §Dependency Security (Audit tool + Pinning) · §Bootstrap phases (dep-audit-tooling-install)
**Change:** the cargo-audit / cargo-deny version pins (0.22.2 / 0.19.8) were exact pins; now minimum **floors** (installed actuals 0.22.1 / 0.19.4 recorded). They are external CLI tools, not `Cargo.lock`-pinnable, and the RustSec advisory DB is fetched fresh each run — so any tool ≥ floor running green satisfies the gate. Toolchain was "currently MSRV 1.88.0 — bump required"; now "**done** (channel 1.95.0, `rust-version = 1.94.1`)".
**Why:** the audit gate landed green with tools a hair behind the researched versions; dev CLI tools cannot be locked in-repo and the advisory DB is runtime-fetched, so the body states floors + records actuals rather than over-precise pins (user-approved; a playbook rule was added so this no longer escalates). The toolchain floor is confirmed satisfied by the green audit.
**Kept:** `tauri` ≥2.10.3 left as a forward "required bump" in §Update policy / §Anti-Patterns — declared in `[workspace.dependencies]` but not yet referenced/locked (dormant until the Tauri GUI, Epoch 9); its audit remit and any further body reconciliation belong to that chunk.
**Ref:** .andromeda/runs/2026-06-15T16-42-50-wrap/

## 2026-06-15-structured-logging-stack — obs identity env-handles noted as non-path (no validation)
**Section:** §Input Validation
**Change:** added a note that `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` (obs-plan §3) are non-path string labels stamped into self-obs JSON log *values* (JSON-escaped; no path/SQL/argv exposure) requiring **no** validation — distinct from the `CONDUCTOR_*` *path* handles that canonicalize + bounds-check.
**Why:** the two new env-var reads are benign labels, not validation boundaries, so a clarifying note (not table rows) closes the gap; resolved with the user.
**Ref:** .andromeda/runs/2026-06-15T17-46-44-wrap/

## 2026-06-15-design-token-typography-bundle — npm (frontend) supply-chain gate added
**Section:** §Dependency Security (new Frontend (npm) supply chain paragraph)
**Change:** added the npm/frontend supply-chain control for the `crates/conductor-tauri/ui/` tree, which cargo-audit/cargo-deny do not cover — `npm audit` clean (0 vulns) + committed `package-lock.json` + vendored fonts (no runtime CDN); npm advisories drive the same floor discipline as cargo (no rigid dependency allowlist at Minimal tier).
**Why:** the chunk introduced a new npm dependency ecosystem (React 19 / Vite 8 / Tailwind 4.1 / Fontsource / TS) while §Dependency Security was cargo-only; resolved with the user — the npm-audit-clean gate + committed lockfile is the control.
**Ref:** .andromeda/runs/2026-06-15T22-05-00-wrap/

## 2026-06-21-runs-db-index — bundled SQLite version corrected to 3.50.4
**Section:** §Infrastructure (Database)
**Change:** `bundled` SQLite was 3.51.1; now 3.50.4 (via `libsqlite3-sys 0.36.0`); rusqlite 0.38.0 unchanged.
**Why:** mirrors the arch §Stack correction — the first real `rusqlite 0.38.0 bundled` compile resolves `libsqlite3-sys 0.36.0` → SQLite 3.50.4; the stated 3.51.1 was assumed. The dep is arch-locked (§Established Decisions [Database]) and audit-green, with `Cargo.lock` committed.
**Ref:** .andromeda/runs/2026-06-21T19-09-44-wrap/

## 2026-06-23-line-oriented-output-rendering — accepted deny.toml exceptions recorded (number_prefix advisory + Zlib license)
**Section:** §Dependency Security
**Change:** added an "Accepted exceptions (deny.toml)" note recording the two justified deny.toml entries — ignore RUSTSEC-2025-0119 (the unmaintained-but-non-vulnerable `number_prefix`, via `indicatif`; no upstream fix) and allow the `Zlib` license (`foldhash` via `rusqlite`→`hashbrown`; OSI+FSF permissive) — the documented mechanism; audit/deny green, no unresolved CVE.
**Why:** the chunk's three new presentation deps (owo-colors / indicatif / comfy-table) plus the supply-chain policy change; resolved with the user ("bless + document in security-plan"). The `foldhash` Zlib failure was PRE-EXISTING on the committed lock — `cargo deny check licenses` was already red because the prior session ran `cargo audit` but not `cargo deny` — and was fixed here because the gate cannot be green otherwise. Follow-up: a future `indicatif` 0.18 bump may drop `number_prefix`, retiring the RUSTSEC-2025-0119 ignore.
**Ref:** .andromeda/runs/2026-06-23T20-40-17Z-wrap/

## 2026-06-24-sanitized-stderr-agent-mode-logging — CONDUCTOR_AGENT_MODE added to the non-path no-validation env-handle note
**Section:** §Input Validation (non-path env-handle paragraph)
**Change:** added `CONDUCTOR_AGENT_MODE` to the non-path-no-validation env-handle list alongside `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` — a boolean trigger whose presence (not value) is read; the agent-mode log path `logs/agent-latest.jsonl` derives from `CONDUCTOR_RUNS_DIR` through the same `resolve_under` traversal guard, not from `CONDUCTOR_AGENT_MODE`.
**Why:** documentation alignment — the invariant already holds: the value is unused and never path/SQL/argv-exposed, and the derived log path reuses the shipped `resolve_under` guard, so there is no new external-input boundary (the consuming-shipped-hardened-infra + reconcile-wording-where-the-invariant-holds playbook rules).
**Ref:** .andromeda/runs/2026-06-24T20-38-37-wrap/

## 2026-06-27-desktop-a11y-harness-setup — npm-audit gate made dev-aware (`--omit=dev`) for the non-shipping a11y test tooling
**Section:** §Dependency Security (Frontend (npm) supply chain)
**Change:** the npm gate was `npm audit` clean (0); now **`npm audit --omit=dev` clean (0 PRODUCTION vulns)**. Dev-only transitive advisories in the non-shipping test tooling are accepted at dev-tree grain (the npm analogue of the deny.toml accepted-exceptions mechanism). Registered the a11y/GUI test devDeps (axe-core / @axe-core/webdriverio / lighthouse / colorjs.io / @crabnebula/tauri-driver / webdriverio / tsx) as the new dev tree.
**Why:** the new a11y harness devDeps brought 20 advisories (19 moderate + 1 high), all dev-only and transitive (`@opentelemetry/core <2.8.0` via lighthouse→@sentry/node; `serialize-javascript <=7.0.4` via @wdio/mocha-framework→mocha), none fixable non-breaking, while production stayed at 0. Resolved with the user: gate production deps strictly and accept dev-only advisories at dev-tree grain — a category-grain accept, vs cargo's per-advisory ignore, chosen for a Minimal-tier local tool to avoid adding an audit-suppression tool. A playbook rule was added so this no longer escalates; the CI Frontend gate and the arch §Inherited Defaults / §Stack one-liners moved in lockstep.
**Kept:** an arch devDep-registration proposal was dismissed as over-reach — a11y-plan §3.5 and stack.md already name the harness tooling.
**Ref:** .andromeda/runs/2026-06-27T12-09-32-wrap/

## 2026-06-27-mcp-read-back-result-shape-adapter — read-back boundary reconciled to the hand-rolled JSON-RPC client (rmcp removed)
**Section:** §Threat Model (attack-surface entry point) · §Input Validation (MCP contract-manifest + MCP-read-back-child-stdout rows) · §Anti-Patterns (STDIO injection wording + negotiate-down) · §Dependency Security (audit tree + Update policy) · §Key decision
**Change:** every "rmcp" mention reconciled to the hand-rolled line-delimited JSON-RPC client:
- the read-back-child-stdout boundary is now **bounded JSON-RPC decoding** (serde_json recursion-limited against decode-bomb depth + a soft per-line size bound), parsing each response line to `serde_json::Value`; faults → `VerifyError::{JsonRpc,Decode,Transport}` via the verdict/error wall;
- the manifest assert reads the `initialize` result's `protocolVersion` (was `peer_info()`);
- the STDIO command/argument-injection class re-anchored to "MCP-sidecar STDIO" (no longer rmcp-specific);
- the rmcp ≥1.4.0 / GHSA-89vp-x53w-74fx stdio-client note retired;
- the audit tree was "tonic/prost/rmcp"; now "tonic/prost".
**Why:** mirrors the arch [MCP Read-Back Client] reversal — rmcp removed because Pulse's `tools/call` is non-MCP-compliant; the reversal was user-confirmed. Every security invariant is preserved: the hardened spawn (fixed path + `.env(...)` + metachar-reject) is unchanged, the read-back decode is bounded and non-panicking through the verdict/error wall (still a validated boundary — JSON, not protobuf), and no new dependency was added (rmcp removed; tokio/serde_json already deps; `Cargo.lock` un-drifted). A routine wording→sound-impl reconcile where the invariant holds.
**Ref:** .andromeda/runs/2026-06-27T20-59-40-wrap/

## 2026-06-27-live-pulse-e2e-proof — corpus.db is plaintext (P-049 encryption assumption corrected)
**Section:** §Threat Model (credential type + corpus.db note) · §Data Classification (compliance note) · §Encryption at rest · §Anti-Patterns (corpus.db rule)
**Change:** every "Pulse's encrypted `corpus.db` (P-049) / OS keychain / decrypt" assertion was reconciled — `corpus.db` is **plaintext SQLite**; the P-049 encrypted-at-rest / `OsKeychainBackend` assumption was wrong, so the keychain-failure canary mode does not apply. The posture is preserved and sharpened: Conductor's production code accesses the corpus via MCP read-back ONLY (never directly opens / copies / exfiltrates it) and never persists its content into Conductor artifacts; `corpus.db` remains out-of-scope SUT-owned data.
**Why:** verified live (2026-06-27, operator findings; the file was read directly once, for operator-sanctioned diagnosis only). Mirrors the arch §Standard Contracts corpus-access correction. No invariant weakened — the "no secrets owned / don't touch Pulse's corpus" posture holds; only the false "encrypted" framing and the inapplicable keychain-failure mode are corrected.
**Ref:** .andromeda/runs/2026-06-27T23-24-58-wrap/

## 2026-08-08-sut-capability-manifest — Capability-manifest input boundary
**Section:** §Input Validation · §Data Classification (scenario-config volume)
**Change:** registered `contracts/pulse-capabilities.toml` as an external-input boundary validated at load (non-empty version/date/set, `P-NNN` shape per id, no duplicates; read faults carry `e.kind()` only, never the path; fixed path through `resolve_under`, no `CONDUCTOR_*` override). Scenario-config volume no longer names P-001..P-060.
**Why:** the chunk added a new runtime-parsed config artifact; §Input Validation enumerates every such surface and omitted it.
**Ref:** .andromeda/runs/2026-08-08T16-05-00-wrap/

## 2026-08-09-interpretation-correctness-posture — advisory-DATABASE fault distinguished from tool fault
**Section:** §Dependency Security (Audit tool; Bootstrap phases dep-audit-tooling-install)
**Change:** installed `cargo-audit` recorded as 0.22.2; added the two-fault split — a TOOL fault is remedied by raising the floor; an advisory-DATABASE fault (the RustSec DB itself unparseable) is remedied by a bounded wait with the audit↔deny overlap VERIFIED green — never a floor raise, never a `deny.toml` ignore, never a CI edit.
**Why:** `duplicate advisory ID: RUSTSEC-2026-0244` failed byte-identically on 0.22.1 and on the latest published 0.22.2, so the prescribed floor-raise was unexecutable and would have looked like compliance while changing nothing.
**Ref:** .andromeda/runs/2026-08-09T19-30-00-wrap/

## 2026-08-09-sut-load-envelope — committed SUT-facing manifests registered as an input boundary
**Section:** §Input Validation (boundary table)
**Change:** added one boundary row covering both committed SUT-facing manifests read at a fixed path — `contracts/pulse-capabilities.toml` and `contracts/pulse-load-envelope.toml` — with their per-artifact validation, the fixed `default_path()` → `resolve_under` resolution with deliberately no `CONDUCTOR_*` override, `e.kind()`-only read faults, and absent/malformed as a hard fault, never defaulted.
**Why:** the chunk added the load envelope, a new runtime-parsed committed artifact, and §Input Validation enumerates every external-input surface. Registering it surfaced that the capability manifest's own row, recorded as landed on 2026-08-08, was absent from the body — so the row covers both rather than leaving a sibling boundary undocumented.
**Ref:** .andromeda/runs/2026-08-10T15-43-07-wrap/

## 2026-08-10-workspace-key-divergence-probe — preflight never-downgrade ban: four named preconditions
**Section:** §Security Anti-Patterns → Universal (the "NEVER silently downgrade a failed preflight" bullet)
**Change:** the bullet's cause enumeration now names the gate's FOUR named preconditions (version mismatch / missing tool / canary fingerprint absent / app-sidecar workspace-key agreement), each surfacing as its own distinct `blocked` precondition string — never the generic corpus-empty string. The stale **keychain read-while-write** cause was removed and replaced by an explicit statement that it does NOT apply (the P-049 encrypted-at-rest assumption was disproved live; `corpus.db` is plaintext).
**Why:** the bullet cites `(Standard Contracts: Readiness gate)` verbatim, so it is a cross-master citation of the passage amended this chunk and its fix folds into the same pass. The keychain half had been stale since the 2026-06-27 arch corpus-access correction, to which the security master had never been reconciled.
**Ref:** .andromeda/runs/2026-08-10T19-49-03-wrap/

## 2026-08-10-pulse-run-contract — the run contract as a third committed input boundary
**Section:** §Input Validation (boundary table)
**Change:** extended the committed-SUT-facing-manifest row to `contracts/pulse-run-contract.toml`, naming its bounds (non-empty identity/provenance; non-empty, unique-id `[[term]]` list; every term carrying a statement AND its causes; non-zero `min_canary_poll_seconds`; no warm-up window with zero emissions; every `shell-declaration` term naming the env var it observes) and its controls (explicit `validate()` in `conductor-core`, fixed `default_path()` through `resolve_under`, no `CONDUCTOR_*` override, `e.kind()`-only read faults, absent/malformed a hard harness fault). Added a row for `ANDROMEDA_PULSE_L4_DETERMINISTIC` as a declaration-only read with nothing to validate — the value never becomes a path, an argv element, or a log value.
**Why:** §Input Validation is an exhaustive enumeration of external-input surfaces and the chunk added one; the 2026-08-10 playbook rule governs this class as routine-apply. Per that rule's trap note, check the body rather than trusting the sidecar — both sibling manifests were already present, so no widening beyond the new artifact was owed.
**Ref:** .andromeda/runs/2026-08-10T21-24-17-wrap/

## 2026-08-10-pulse-run-contract — never-downgrade ban restated over five preconditions
**Section:** §Security Anti-Patterns → Universal
**Change:** the bullet's enumeration was four preconditions; now FIVE, adding unmet run-contract terms. The run-contract arm composes one string naming each unmet term individually with its causes, and a term whose truth lives on the SUT's side is recorded `declared-not-observable` and never blocks, because blocking on it would assert a measurement Conductor cannot make.
**Why:** the bullet verbatim-enumerates the set, so it goes stale on every addition. The never-claim-a-measurement clause carries over the v2-17 precedent.
**Ref:** .andromeda/runs/2026-08-10T21-24-17-wrap/
## 2026-08-11-faithful-emission-dispatcher — scenario-config boundary widened; `dive` never `skip`
**Section:** §Input Validation (scenario-config row) · §Input Validation Vector 2 trust boundary · §Security Anti-Patterns (Input)
**Change:** the scenario-config row now covers the `[phases.emission]` surface — `kind` REQUIRED whenever the table exists (no inferred default), `occurrences` bounded by `MAX_OCCURRENCES` (`0` = a declared silence window), and the per-shape cross-field rules (percentile ordering, severity ∈ 1..=24, the breathing amplitude < center sign guard, ≥2 distinct topology services, non-empty variant/category sets) — and states the error fraction in its shipped integer-percent encoding. All three sections now mandate that **every nested spec field must `dive`, never `skip`**: a skipped nested struct deserializes entirely unvalidated because garde never descends into it, so its rules never run.
**Why:** the chunk added the `[phases.emission]` external-input surface across 23 committed scenario TOMLs and found `PhaseSpec.emission` marked `#[garde(skip)]` — the documented trust boundary was not the boundary that existed. §Input Validation enumerates every external-input surface, so the new surface earns its description (the 2026-08-10 committed-artifact precedent).
**Ref:** .andromeda/runs/2026-08-13T16-43-31-wrap/

## 2026-08-13-first-live-green-preflight — MCP_ENABLED locus + shell-side contract readers
**Section:** Threat Model Summary §Auth model (Reason) · §Input Validation (committed SUT-facing manifests row)
**Change:**
- `ANDROMEDA_PULSE_MCP_ENABLED` must be present in Conductor's OWN process environment, where the operator exports it and the sidecar receives it by inheritance — `spawn.rs` passes only the data dir via `.env(...)`, so inheritance is the sole channel; absent it the sidecar starts and immediately exits and every read-back measures the unreachable path. Conductor still never sets it itself.
- The manifests row records that `pulse-run-contract.toml` now has readers outside `conductor-core` — the two harness scripts parse `warmup_ms` + `min_canary_poll_seconds` under the same never-defaulted rule (missing term ⇒ hard exit 2; a lowered env value clamps up to the contract floor).
**Why:** verified live 2026-08-13 — an arm run without the flag measured only the read-back-unreachable path, and the operator recipe had omitted it. The shell-side reader is a genuinely new external-input boundary this chunk added.
**Ref:** .andromeda/runs/2026-08-13T22-48-53-wrap/

## 2026-08-16-canary-fingerprint-derivation-aligned — third preflight precondition renamed
**Section:** §Security Anti-Patterns → Universal (never-silently-downgrade ban)
**Change:** in the FIVE-precondition enumeration, "canary fingerprint absent" was retired; now "no incident opened after the canary storm was emitted". Count and never-downgrade rule unchanged.
**Why:** the fingerprint precondition could never pass at any width (`fingerprint_refs` carries no Pulse-computed fingerprint), so the ban named a precondition that no longer exists; the distinct-blocked-state requirement is unchanged and now attaches to the freshness arm.
**Ref:** .andromeda/runs/2026-08-16T09-59-45-wrap/

## 2026-08-16-canary-fingerprint-derivation-aligned — child-stdout row states the freshness assertion
**Section:** §Input Validation (MCP read-back child stdout row)
**Change:** the canary clause was an identity claim; now it asserts an incident opened after the emission stamp (`extract::opened_at_unix_nanos` vs `CanaryMarker.emitted_at_unix_nano`); an absent or unparseable stamp contributes nothing and reads as NOT-fresh ⇒ `blocked`, never a false pass.
**Why:** the row asserted an identity claim the gate no longer makes. The degrade DIRECTION is the security-relevant half and is now explicit: the other readers degrade to empty on shape, but a missing stamp must never read as satisfied.
**Ref:** .andromeda/runs/2026-08-16T09-59-45-wrap/

## 2026-08-16-canary-fingerprint-derivation-aligned — third accepted deny.toml exception recorded
**Section:** §Dependency Security (Accepted exceptions)
**Change:** adds the `BSD-2-Clause` allow for `arrayref` (via `conductor-emit`→`blake3`) beside the `number_prefix` ignore and the `Zlib` allow, and states that `deny.toml` itself is the authority on the full set, while reconciling the two is carried by the `Dependency polish` route entry.
**Why:** the paragraph read as an exhaustive list and did not authorize the license this chunk's dependency needs. The authority note prevents a future reader treating the known-incomplete prose as the allowlist.
**Ref:** .andromeda/runs/2026-08-16T09-59-45-wrap/

## 2026-08-16-canary-fingerprint-derivation-aligned — admitting a dependency under a red audit
**Section:** §Dependency Security (advisory-DATABASE fault bullet)
**Change:** states the admission condition explicitly — a dependency delta MAY land while the advisory DB cannot be parsed, but only on a `cargo deny check advisories bans licenses sources` VERIFIED green over the NEW `Cargo.lock`; the deferral's basis no longer rests on "no dependency delta".
**Why:** the bounded wait was justified partly by a static tree, a precondition this chunk broke. Left unstated, the doc would assert an audit-green admission condition its own shipped dependency does not meet, and the next chunk would re-derive the case from scratch.
**Ref:** .andromeda/runs/2026-08-16T09-59-45-wrap/

## 2026-08-16-canary-fingerprint-derivation-aligned — deprecated-crypto ban is now active
**Section:** §Security Anti-Patterns → Data Protection
**Change:** the hypothetical "if any fingerprint/hash is ever added" framing is retired — Conductor ships a blake3 derivation (first 16 bytes / 32 hex), the FNV-1a derivation is removed, and the run-report `fingerprints[]` column is noted as read-back-fed, never fed from that derivation.
**Why:** the ban described a future possibility this chunk made present; anchoring it to the shipped derivation makes it enforceable rather than aspirational.
**Ref:** .andromeda/runs/2026-08-16T09-59-45-wrap/

## 2026-08-19-connection-lifecycle-live-proof — `[phases.fault]` registered at the scenario-config boundary
**Section:** Input Validation (scenario-config row) · Threat Model Summary (attack-surface config-files vector) · Security Anti-Patterns (Input ban)
**Change:** all three enumerations of the scenario-config trust boundary now carry the `[phases.fault]` block: closed `FaultKindSpec` kind enum (unknown kind = deserialize error), `dive` never `skip` on `PhaseSpec.fault`, the scenario-level `fault_phases_are_silent` (fault ⇒ occurrences 0), and the deliberate NO-port-in-config rule (private builder parameter only — no operator-steerable bind target).
**Why:** the chunk added a new operator-authored config surface that shipped fully validated; emission-only enumerations would have left a later `#[garde(skip)]` on `fault` violating no written mandate.
**Ref:** .andromeda/runs/2026-08-19T23-10-30-wrap/

## 2026-08-20-latency-regression-re-proof — the sidecar spawn resolves a NAME through PATH, not a path
**Section:** §Security Anti-Patterns → Input (+ its restatement in → Code Patterns)
**Change:** the sidecar-spawn ban was "a fixed, hard-coded program path only"; now "a fixed, hard-coded program NAME resolved through the inherited `PATH` (never a config-derived or operator-supplied command string)", with `PATH` named as a spawn-resolution input — the environment selects WHICH binary the fixed name resolves to. Added the diagnostic: a `PATH` entry split on a drive-letter colon resolved the sidecar nowhere and the run reported `[BLOCKED]` in ~0s, indistinguishable at row level from a genuine SUT-side gate failure. Both occurrences (→ Input, → Code Patterns) updated together.
**Why:** `PULSE_MCP_PROGRAM = "andromeda-pulse-mcp"` is a program NAME resolved from `PATH`, so the ban's wording overstated the control while its INTENT (never operator-chosen; a compile-time constant) held and is preserved. A pre-existing gap surfaced by a live leg failure; operator-ratified.
**Kept:** the immutable Security Decisions Log entry, left as historical record.
**Ref:** .andromeda/runs/2026-08-20T17-10-10-wrap/

## 2026-08-21-per-check-latency-measurement — budget_ms + the load-path check arm
**Section:** Input Validation -> scenario-config row
**Change:** the what-to-validate cell gains `[[expected]].budget_ms` (garde `range(min = 1, max = MAX_BUDGET_MS)`), and the How cell records the load-path `Scenario::check_*()` arm as the sanctioned co-equal mechanism for sibling-reading cross-field rules, faulting as `CoreError::Config`. The row previously mandated `#[garde(custom)]` for such rules.
**Why:** the new key's cross-field rule is one garde 0.22.1 structurally cannot express; under the old mandate an auditor would read a validated boundary as unvalidated, or "fix" it into an unbuildable validator.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — Input ban names both arms
**Section:** Security Anti-Patterns -> Input
**Change:** the scenario-config ban now names both arms (garde `range`/`dive` AND the load-path `check_*()` checks) and adds the carve-out that a SIBLING-reading invariant must NOT be written as `#[garde(custom)]` — it would look enforced while never testing the invariant.
**Why:** a duplicate occurrence of the retired garde-custom-only claim; left unamended, the shipped `check_budgets` boundary would read as a ban violation at the next audit.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — Bootstrap phase requires the load-path arm
**Section:** Bootstrap phases -> input-validation-library-install
**Change:** confirmation extended to require every sibling-reading cross-field rule to ship as a `Scenario::check_*()` from `from_toml_str`, with a test pinning that the load path invokes it.
**Why:** a third restatement; as written the phase mandated a check that cannot be satisfied for sibling-spanning rules.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — Threat-model trust boundary records the load-path arm
**Section:** Threat Model Summary -> Attack surface (config files)
**Change:** the trust boundary records the load-path `check_*()` arm and the budget rule beside the `#[garde(custom)]` examples.
**Why:** a fourth occurrence of the same retired claim.
**Kept:** no lockstep mirror clause for a verbatim `threat-assessment.md` — that file does not exist in this repo, so the proposed mirror was dropped as unsubstantiated (absence needs evidence).
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-22-operator-pause-and-checklist-live-firing — `[[checklist]]` registered at the scenario-config boundary
**Section:** Input Validation → scenario-config row (both the what-to-validate and the how columns) · Threat Model Summary → Attack surface, config-files trust boundary
**Change:** added the `[[checklist]]` block to the enumerated scenario-config boundary — `Scenario.checklist: Vec<ChecklistItem>`, `#[serde(default)]`, `dive` never `skip`, both text halves bounded by `MAX_CHECKLIST_TEXT` (200) — and extended the sibling-spanning load-path enumeration from two (`check_capabilities`; `check_budgets`) to three by naming `check_checklist`, at both the §Input Validation site and its verbatim Threat-Model restatement.
**Why:** §Input Validation is an exhaustive enumeration of external-input surfaces; the chunk added one with its controls already present, so only the registry lagged. The `check_*()` list is restated identically in §Threat Model, so a single-site apply would leave it stale.
**Ref:** .andromeda/runs/2026-08-22T12-15-00Z-wrap/

## 2026-08-31-p-075-assert-round — Corpus access is read AND write through the MCP tool surface
**Section:** Threat Model Summary (corpus.db note) · Input Validation (MCP child stdout row) · Data Protection (at-rest paragraph) · Security Anti-Patterns (corpus.db NEVER-ban)
**Change:** was "Conductor only reads the corpus via MCP read-back" at all four sites; now "through the MCP TOOL SURFACE only — read-back plus the `mark_incident_resolved` lifecycle write — never the file". The Input-Validation row is renamed `MCP child stdout` and names the write direction explicitly: the applied `{resolved, incident_id}` response and the DECLINED `ToolDispatchFailed` arm both arrive through the same `call_tool` path under the same bounded-decode / typed-error controls. The anti-pattern ban is UNCHANGED in force and gains a clarifying clause: the ban is on FILE access, and staging a corpus row directly to induce `DeclinedStale` is the banned route — which is why that arm is stub-proven.
**Why:** the chunk landed the first production caller of a corpus-MUTATING MCP tool (`conductor_run::probe_resolve_lifecycle`) and exercised it live. Every mandated control is present (bounded decode, `VerifyError::JsonRpc` whose Display renders the code only, no direct file access, nothing persisted or exfiltrated), so this is boundary-INVENTORY drift, not an unvalidated boundary. Operator-ratified after escalation; no matching playbook pattern existed.
**Ref:** .andromeda/runs/2026-09-01T16-50-00Z-wrap/

## 2026-09-01-webview-self-verify-windows-host — a new harness boundary, and two bans scoped to shipped binaries
**Section:** Input Validation (boundary table) · Security Anti-Patterns (Input canonicalize ban · Code Patterns spawn ban · Universal inbound-listener ban) · Threat Model Summary (Attack surface: CLI input vector · trust boundary) · Secret Management (Development) · Bootstrap phases (input-validation-library-install)
**Change:**
- Registered `CONDUCTOR_MSEDGEDRIVER` as a dev-harness-only path boundary validated at the wdio edge (existence + `isFile` + shell-metacharacter rejection, then a separate argv element of an array-form spawn; unset-or-not-a-file ⇒ skip at exit 0), explicitly NOT under the Rust-side `canonicalize` rule since no Rust code reads it — named as such in the boundary table, the canonicalize ban, the bootstrap phase, the CLI-input vector and the non-secret handle inventory.
- The **spawn ban** was split into its MCP-sidecar rule (unchanged) plus a harness-spawn rule: the program must be a repo-derived resolved constant; an operator value may appear only as an array-form argv element after metacharacter rejection, never as the program and never in a shell.
- The **inbound-listener ban** and the trust-boundary sole-port claim were scoped to Conductor's SHIPPED binaries, with the dev-only `4444`/`4445` driver ports named as outside them.
**Why:** unlike the 2026-06-23 dismissals under the playbook's no-new-boundary rule, this chunk genuinely ADDS an external-input boundary and a spawn, so that rule's precondition was unmet and escalation was correct. Both ban qualifications were operator-ratified: left unqualified, the plan would forbid a leg the harness now ships, so a reader checking a green chunk would read a violation that is not one. The controls themselves are unchanged in force.
**Ref:** .andromeda/runs/2026-09-01T18-49-38Z-wrap/

## 2026-09-01-desktop-a11y-sweep — dev driver-leg surfaces enumerate both a11y arms
**Section:** §Threat Model Summary (attack surface: port bind, CLI-input entry point) · §Input Validation (`CONDUCTOR_MSEDGEDRIVER` row) · §Security Anti-Patterns (harness-spawn rule (b), inbound-listener ban carve-out)
**Change:** five sites were scoped to the `--e2e` leg alone; now to the dev-only driver stack fired by EITHER arm (`agent-run … --e2e` routine, `npm run a11y:driven` driven): the harness-lifetime `4444`/`4445` listeners, the ban carve-out covering them, the `CONDUCTOR_MSEDGEDRIVER` reader path, its unset/not-a-file skip-at-exit-0 consequence, and the harness-spawn rule's scope label.
**Why:** a second suite now runs on the same `wdio.conf.ts`, so the same guard, handle and listeners are reached by a second path. No control changed — the `UNSAFE_PATH` guard, the repo-derived resolved program and the array-form spawn are untouched; only the enumerations were stale, and a ban whose scope label names one leg reads as not covering the other.
**Ref:** .andromeda/runs/2026-09-01T22-22-12Z-wrap/

## 2026-09-01-live-per-p-id-verdict-lamps — Tauri IPC arguments get a boundary row
**Section:** Input Validation (boundary table)
**Change:** added a row for `#[tauri::command]` IPC arguments — the `run_id`-class string carried by `run_report` / `run_envelope` — mandating a `resolve_under` traversal guard against the resolved runs dir before any read (absolute or `..` REJECTED, never clamped) plus `sanitize_error` on every returned `Err`; garde is n/a (not scenario config). The row records that app-defined commands are NOT capability-ACL-gated, so this row — not the capabilities file — is what governs them.
**Why:** the shipped `run_envelope` doc comment cites this section for its guard, but the table had no Tauri-IPC row — a dangling citation, and no rule for the next command author to follow. The boundary class is pre-existing (`run_report` shipped the same shape in Epoch 9), so this documents already-correct practice rather than adding a mandate. Resolved with the operator.
**Ref:** .andromeda/runs/2026-09-02T00-58-00Z-wrap/

## 2026-09-02-screen-reader-manual-spec — `CONDUCTOR_NVDA`, the speech-log ingest boundary, the harness-spawn forms, the sidecar window-suppression duty
**Section:** Threat Model Summary (CLI-input vector · port-bind vector) + Input Validation (boundary table: the `CONDUCTOR_MSEDGEDRIVER` row, new `CONDUCTOR_NVDA` row, new speech-log ingest row) + Bootstrap phases (`input-validation-library-install`) + Secret Management (Development) + Security Anti-Patterns (Input canonicalize ban · Logging host-path ban · Code Patterns spawn rule (a)+(b) · Universal listener carve-out)
**Change:**
- `CONDUCTOR_NVDA` registered as the second dev-harness host-tool path handle (existence + `isFile` + metacharacter guard at the wdio edge, array-form detached spawn with a fixed argv, skip at exit 0 when unset, value never committed); every "one handle" singular widened to the SET.
- A new boundary row for the screen-reader speech-log ingest — untrusted third-party text, `heard` bounded at 400, closed enums, host-path scrub → `<host-path>` + `security_finding`.
- Rule (b) now governs three spawn forms: the repo-resolved driver constant; a fixed OS program + fixed argv for the PowerShell activation script; the guarded host-tool path as the program for NVDA.
- Rule (a) gains the console-window-suppression duty — the flagless sidecar spawn publishes its absolute exe path as a foreground pane title — recorded as a measured, route-owned defect, not a shipped control.
- The driver-leg enumerations (CLI-input vector, port-bind vector, Universal carve-out) name the three-family set; the Logging ban covers committed evidence records.
**Why:** the handle, the ingest row and the spawn forms are the chunk's new external-input surfaces and spawn forms; a real host path arrived through the speech log. The window-suppression duty was measured 2026-09-02 (NVDA spoke the pane's title), escalate-class and resolved on operator direction. The driver-leg enumerations restate a moved count at three sites.
**Ref:** .andromeda/runs/2026-09-02T11-47-51Z-wrap/

## 2026-09-02-cross-surface-envelope-parity — the third `CONDUCTOR_*` handle class, the fourth harness spawn form, and the protocol-version mechanism
**Section:** Input Validation · Threat Model Summary (CLI-input vector) · Secret Management (Development) · Bootstrap phases (input-validation-library-install) · Security Anti-Patterns (Input; Code Patterns)
**Change:**
- New §Input Validation row for `CONDUCTOR_E2E_SEED_DIR` stating the THIRD control model — a harness-owned repo-relative CONSTANT carried to the seeding child through the spawn's env map (never argv, never a shell), no `canonicalize` rule and no `isFile`/metacharacter guard because the value is not operator-supplied, no-op when unset. Residual recorded: the test applies no `resolve_under` guard of its own, so the repo-relative discipline lives at the wdio caller and holds only while no shipped binary reads the handle.
- Every singular "the two handles" clause widened to the SET of three, with the reason SPLIT where it differs (`MSEDGEDRIVER`/`NVDA` never reach Rust; `E2E_SEED_DIR` reaches Rust only in a test binary): the canonicalize ban's exemption, the CLI-input vector enumeration, the non-secret handle list, and the bootstrap phase's out-of-scope set.
- The harness-spawn enumeration was THREE governed forms; now FOUR, adding the fixture-seed spawn (fixed `cargo` program, fixed argv, handle in the env map, throws on non-zero exit).
- The protocol-version ban keeps its subject and retires its negotiate-down mechanism clause: the hand-rolled client READS `protocolVersion` from the `initialize` result and HOLDS it against the pinned `2024-11-05` — it does not negotiate down from a client default.
**Why:** the new external-input boundary was operator-ratified 2026-09-02 as a third class; the widenings follow the no-single-site-apply rule, since each clause asserted an exhaustive set the new handle falsifies. The mechanism correction matches the same correction shipped in `conductor-verify` and the sibling test-plan site.
**Ref:** .andromeda/runs/2026-09-02T14-34-37Z-wrap/

## 2026-09-03-live-pulse-preconditions-probed — the declaration-only read SET, and a non-spawning resolution of the sidecar name
**Section:** §Input Validation (2 rows) · §Threat Model Summary (Auth model Reason) · §Security Anti-Patterns (Input · Code Patterns (a) · Universal)
**Change:**
- The `ANDROMEDA_PULSE_L4_DETERMINISTIC` row is generalised to the three-handle declaration-only READ SET (`conductor_core::OBSERVED_HANDLES`) — presence-only, no value ever becoming a path, argv element or log value.
- The `ANDROMEDA_PULSE_DATA_DIR` row is scoped to the SPAWN-propagation use, now that the handle has a second, non-spawn presence-only reader that neither relaxes nor satisfies that duty.
- Auth model records that an absent `ANDROMEDA_PULSE_MCP_ENABLED` is now observed UPSTREAM (a `[PRECONDITION]` subject and the `mcp-enabled` term), not only downstream as the sidecar exiting.
- Anti-Patterns Input and Code Patterns (a) record the second, NON-SPAWNING resolution of `PULSE_MCP_PROGRAM` — a `split_paths` + `PATHEXT` walk constructing no `Command`, returning a boolean-grade fact, never the resolved path, therefore opening no console window.
- Universal scopes the never-silently-downgrade-a-preflight ban to preflights that are INVOKED, recording `boot`'s short-circuit as the sanctioned skip.
**Why:** the playbook's dismiss precondition failed (the chunk DOES add external-input surfaces — a `PATH` directory walk and two handles with no row), and boundary widening is never-routine by design. Operator-ratified on the basis that the probe adds no write, no spawn and no new subprocess crossing — only a new READ-only surface. No playbook rule was minted; the class keeps escalating.
**Ref:** .andromeda/runs/2026-09-03T19-20-00-wrap/
## 2026-09-04-sr-findings-remediation — "not a downgrade" withdrawn; the path handle cannot declare
**Section:** §Security Anti-Patterns → Universal (primary) · §Input Validation — the `ANDROMEDA_PULSE_*` declaration-only READ SET row
**Change:** the Universal bullet keeps the probe's non-mutating / no-spawn / no-env-write properties but WITHDRAWS the "that is not a downgrade" reading: because `declares()` value-gates all three handles, the PATH-valued `ANDROMEDA_PULSE_DATA_DIR` never satisfies `handles-declared`, the probe's exit is unconditionally non-zero, and `boot` has skipped the MCP readiness gate on every run since `480bc66` — which IS a downgrade of that path until the handle is presence-graded. The §Input Validation row now separates the two declarable BOOLEAN handles from the PATH handle, keeps "nothing to validate" (no value becomes a path, argv element or log value), and records that a present-and-correct data-dir is presently indistinguishable from an absent one.
**Why:** a spec claim disproved by measurement. Routine under the playbook's measured-product-defect rule (zero delta at its site, dispositioned by an operator directive naming the defect itself); applied as a record, with the fix route-owned.
**Ref:** .andromeda/runs/2026-09-04T07-33-12Z-wrap/

## 2026-09-04-preconditions-probe-reads-path-handles-by-presence — the withdrawal retired
**Section:** §Security Anti-Patterns → Universal (primary) · §Input Validation — the `ANDROMEDA_PULSE_*` declaration-only READ SET row · §Input Validation — the `ANDROMEDA_PULSE_DATA_DIR` SPAWN-propagation row (parenthetical)
**Change:** The Universal bullet's "not a downgrade" reading HOLDS again (retiring the withdrawal): the probe grades the PATH handle by presence via `conductor_core::handle_declared`, the two flag handles keeping the truthy gate through the value-only `flag_declared`, so `handles-declared` is satisfiable, the short-circuit fires only on a genuinely unmet subject, and `boot` reaches the MCP readiness gate. The ban's scoping (it binds a preflight that IS invoked) and the probe's non-mutating / no-spawn / no-env-write properties are unchanged.
- The READ SET row is re-pointed from `declares()` value-grading to `conductor_core::{flag_declared, handle_declared}` and records the non-widening witness; "nothing to validate" still holds.
- The spawn-propagation row's parenthetical now says the declaration reader grades this handle on PRESENCE; the spawn-side duty is untouched (`.env(...)`-only, metacharacter rejection, never in argv).
**Why:** `handles-declared` was measured satisfiable. Non-widening witness met: `ANDROMEDA_PULSE_DATA_DIR=false` declares while `ANDROMEDA_PULSE_MCP_ENABLED=false` does not, because the run-contract path routes only to the value-only `flag_declared` and structurally cannot reach the presence arm. Handle-NAME-only met (the gate's `data_dir` renders `<redacted>`). Operator-ratified (apply all three): the master's own text named this chunk as the fix's owner and the chunk shipped it; the operator approved minting a playbook rule for that class. The boundary-widening (never-routine) rule does NOT match — the probe grants nothing and passes nothing onward, the blocking gate is behaviourally unchanged, and the withdrawn bullet had itself identified the PRE-chunk state as the downgrade.
**Ref:** .andromeda/runs/2026-09-04T17-15-00-wrap/

## 2026-09-04-sidecar-spawn-without-a-console-window — the console-suppression duty stops being owed and starts being held
**Section:** §Security Anti-Patterns → Code Patterns rule (a) — console-window clause (primary) · rule (a) — `sidecar_resolves_on_path` sibling clause · §Input Validation — `ANDROMEDA_PULSE_DATA_DIR` SPAWN-propagation row · §Threat Model Summary — MCP read-back child-process stdout vector
**Change:**
- Rule (a)'s console clause: was "recorded as a routed product defect, not yet shipped"; now SHIPPED and live-verified — `build_command` applies `.creation_flags(console_suppressing_flags())` under `#[cfg(windows)]`, held by a `#[cfg(windows)]` unit test. The 2026-09-02 pane measurement stays as dated history marked SUPERSEDED. The applied flag's unit-tier unassertability (`std::process::Command` has no creation-flags getter) is recorded in the body.
- The `sidecar_resolves_on_path` sibling clause drops the "constructs no `Command`, therefore opens no console window" contrast and states its remaining distinct property: a boolean-grade directory walk whose resolved path is never returned, logged, rendered, or used as a spawn program.
- Both restatements of the retired `TokioChildProcess` spawn mechanism now read `spawn::build_command` / `tokio::process::Command`; every control (fixed program NAME through the inherited `PATH`, `.env(...)`-only data dir, injection-metacharacter reject, nothing operator-supplied in argv, no shell in the vector) is unchanged.
- §Input Validation's screen-reader speech-log ingest row: the parser path was the non-resolving `test/a11y/screen-reader/parse-nvda-log.ts`; now `crates/conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts`.
**Why:** The body had named the fix route-owned and not shipped; the chunk discharged it, and the live SR pass measured the pane gone (row S1-01 free of the `<host-path>` and `pane` utterances, 0 `security_finding` rows). Routine under the playbook rule for a chunk that ships a fix a master's own body names route-owned-not-shipped, even at escalate severity; the boundary-widening (never-routine) rule does NOT match — the flag suppresses a window and admits nothing new across the boundary, narrowing a disclosure channel. The unassertability is recorded so a later reader does not mistake the missing unit assertion for missing coverage. The parser path is the same defect class as the a11y-plan pass-spec path corrected the same wrap, fixed so a duplicate does not stand.
**Ref:** .andromeda/runs/2026-09-04T20-15-00-wrap/

## 2026-09-05-audit-corrective — the RUSTSEC-2026-0244 red re-adjudicated from external to local
**Section:** Dependency Security — the two-external-decay-faults block (the *Established empirically 2026-08-09* line) + the NEW-dependency admission bullet
**Change:** The 2026-08-09 failure is re-adjudicated: was an advisory-DATABASE fault; now a LOCAL host-checkout artifact — upstream moved `RUSTSEC-2026-0244` to `crates/gettext-rs/` on 2026-08-09, the deferral's first day, while an untracked copy of the old path stayed in this host's `${CARGO_HOME:-$HOME/.cargo}/advisory-db`, so all 52 byte-identical readings measured the cache rather than the world. Post-clean the probe is green (porcelain empty, `cargo audit` exit 0, `cargo deny` exit 0), closing the standing deferral, and the porcelain check now PRECEDES any external classification. The "dependency delta MAY land under a red audit, deny-overlap as sole coverage" regime is CLOSED as of 2026-09-05, re-arming only on a porcelain-verified external fault — at both sites, the admission bullet included.
**Why:** The chunk measured the prior claim false and the gate green. The admission bullet restated the retired premise in its own words ("while the fault persists", "under a red audit", "the deny overlap is then the sole coverage"), so both sites were amended and no single-site apply leaves the premise standing.
**Kept:** The fault-CLASS definition one bullet up — it defines a class for future use and asserts nothing about the current gate.
**Ref:** .andromeda/runs/2026-09-05T21-09-11Z-wrap/

## 2026-09-06-operator-gated-live-suite — the leading preconditions probe now covers two harness paths
**Section:** §Security Anti-Patterns — Universal (the "scopes to a preflight that is INVOKED" bullet) · §Security Anti-Patterns — Input (the sidecar-spawn ban's `PATH`-miss upstream-naming sentence)
**Change:** Both sites state the path SET `{boot, run --live}` (was `boot` alone): the leading `conductor preconditions` arm short-circuits on either, minting no `ReadyState`, no `Verdict`/`ReportState` and no `blocked` row; on `run --live` the short-circuit is a REFUSAL at exit 1 naming each unsatisfied subject with no leg fired, so the sidecar is never spawned rather than the run degrading to a ~0s `[BLOCKED]` row. The `PATH`-miss sentence likewise names the SET and the `sidecar-resolvable` subject. The probe's own properties (non-mutating, client-only `:4317` connect, no spawn, no env write, fires no canary) are unchanged.
**Why:** The chunk added a second harness path running the same probe as its leading arm; the refusal arm was measured (exit 1, `sidecar-resolvable` and `handles-declared` named, 0 host paths, no capture dir created). Routine as spec wording reconciled to the shipped implementation: the invariant is unchanged, only the set of paths realising it grew.
**Kept:** Three other security proposals were withdrawn at the source. The harness's first recursive delete (`rm -rf "$RUNS_DIR/live-suite"`, from an operator-supplied `CONDUCTOR_RUNS_DIR` with no canonicalize, no `--`, no non-empty assertion, executing BEFORE any Rust-side `resolve_under` rejection) was proposed as a documented residual plus two ban widenings; escalated as a never-routine security boundary, the operator chose to FIX THE CODE. Both shells clear the capture dir with the non-recursive `cleanup` idiom (`rm -f …/*.jsonl` · `Remove-Item '…/*.jsonl'`) and the harness ships NO recursive delete. §Input Validation, §Anti-Patterns → Input's canonicalize-duty scope and Code Patterns clause (b)'s "In every form" universal are UNCHANGED — no security ban was weakened.
**Ref:** .andromeda/runs/2026-09-06T09-37-04-wrap/

## 2026-09-06-run-report-envelope-conformance-gate — the canonicalize duty binds per READER; the CLI run_id argv enumerated; harness-spawn rule (b) gains a fifth form
**Section:** Input Validation (env-var path handles row, CLI arguments row) + Threat Model Summary (Vector: CLI input) + Bootstrap phases (input-validation-library-install) + Security Anti-Patterns - Input (canonicalize taxonomy) + Security Anti-Patterns - Code Patterns (child-spawn rule (b))
**Change:** SIX sites in three claim-families.
- (a) The canonicalize duty binds by READER, not by handle (three sites): `CONDUCTOR_E2E_SEED_DIR` is no longer the only Rust test-binary reader — `conductor-run/tests/journal_conformance.rs` reads `CONDUCTOR_RUNS_DIR` in a test binary, applying no canonicalize of its own and carrying the same explicit residual, bounded by a host-path-free failure origin.
- (b) The `run_id`-class POSITIONAL argument is enumerated (Input Validation CLI row, Threat Model CLI-input vector) with its control: on `cleanup` its only sink is a rusqlite `?1` bound parameter, never a path join, so neither the canonicalize duty nor the Tauri row's `resolve_under` duty attaches.
- (c) Child-spawn rule (b): was four governed forms scoped to the dev-only driver stack; now FIVE, the fifth on the ROUTINE `agent-run.{sh,ps1}` path — the repo-built `conductor` binary as a fixed program with a fixed verb and the operator-supplied `<run_id>` as a separate argv word, admitted only after `valid_run_id` / `Test-RunId`.
**Why:** All six were operator-ratified. Family (c) is boundary widening — a subprocess boundary gaining a new crossing — which is never routine, so no rule was minted and a sixth form escalates again. Net posture improves: the fifth form REPLACES a `sqlite3`-CLI spawn, removing an undeclared host-tool dependency (`sqlite3` is absent from this host and installed by no CI runner, so the delete it guarded had never executed observably). Family (a) proved live in-chunk: the gate's own failure origin leaked an absolute host path via `path.display()` before being fixed to handle + filename.
**Ref:** .andromeda/runs/2026-09-06T13-07-09-wrap/

## 2026-09-06-coverage-completeness-gate — committed-manifest row records a test-binary reader
**Section:** §Input Validation → the "Committed SUT-facing manifests read at a fixed path" row
**Change:** The row's resolution sentence is scoped to SHIPPED readers ("resolved by every SHIPPED reader from a fixed `default_path()` through `resolve_under`"), and a per-reader clause is added: `pulse-capabilities.toml` also has a TEST-BINARY reader outside `conductor-core` (`conductor-report --test coverage_gate`), which loads it through the same bounds-checking `CapabilityManifest::load` but resolves the conventional repo path from `CARGO_MANIFEST_DIR` rather than `default_path()`, because a cargo test binary's CWD is its own package root where no `contracts/` exists. It introduces no `CONDUCTOR_*` handle and reads nothing operator-supplied, so the not-operator-steerable property holds by a different resolution.
**Why:** Escalated (the matching playbook rule's qualifier — re-confirming a hardening/validation invariant — was false; this records a new READER) and operator-resolved 2026-09-06: record the reader. The row already carried the analogous per-reader clause for `pulse-run-contract.toml`'s outside readers, and under the per-READER restatement an unrecorded reader is the drift. Single site.
**Ref:** .andromeda/runs/2026-09-06T15-59-32-wrap/

## 2026-09-07-dependency-polish — Accepted-exceptions reconciled to deny.toml's post-bump set
**Section:** §Dependency Security → Accepted exceptions
**Change:** The paragraph states the reconciled set — 16 `[advisories] ignore` + 9 `[licenses] allow` — with `deny.toml` still the authority and the counts marked a reconciliation stamp, not a second source. A separate Retired-2026-09-07 note records that the `number_prefix` / RUSTSEC-2025-0119 ignore is gone.
**Why:** The retirement's authority is `deny.toml`'s own justifying-comment requirement — explicitly NOT a rule of this plan: no security-plan rule compelled it, and the 2026-06-23 sidecar Follow-up anticipated it without obliging it. Wording operator-constrained after a review retracted an assertion that this master forbade leaving such an ignore standing.
**Ref:** .andromeda/runs/2026-09-07T12-43-31-wrap/

## 2026-09-07-dependency-polish — Tauri exposure premises retired; the floors kept verbatim
**Section:** §Threat Model Summary → Infrastructure/Hosting · §Dependency Security → Update policy · §Security Anti-Patterns → Universal
**Change:** The Hosting bundle version reads 2.11.3. The Update-policy and Anti-Patterns parentheticals no longer say the architecture pins 2.10.1 and is therefore exposed / owes the bump (retired premise: the current pin is 2.10.1).
**Why:** Tauri resolves to 2.11.3; only the stale premise moved, while the ban and the floors are unchanged.
**Kept:** Both `≥ 2.10.3` floors, the toolchain `≥ 1.94.1` floor and the MSRV clause, verbatim.
**Ref:** .andromeda/runs/2026-09-07T12-43-31-wrap/

## 2026-09-07-a11y-ci-gate — `CONDUCTOR_A11Y_STRICT` registered as a FOURTH handle class
**Section:** §Input Validation (boundary table) · §Threat Model Summary → Attack surface → Vector: CLI input · §Secret Management → Storage → Development
**Change:** Added the `CONDUCTOR_A11Y_STRICT` boundary row and widened the two namespace-wide enumerations from three dev-harness shapes to four. Its control model is NEITHER shipped shape: it is FLAG-valued, not path-valued, so the `std::fs::canonicalize` rule has nothing to canonicalize and the wdio-edge `isFile` + shell-metacharacter guard has nothing to guard; the only test is affirmative truthiness mirroring `conductor_core::flag_declared`; the value never becomes a path, argv element, spawn program or log value. Its sole effect is to INVERT what an already-unresolved host-tool handle costs — lax keeps the skip at exit 0, strict exits non-zero — with the existing guards byte-unchanged: an ADDED arm, not a relaxation. Mandated coverage is the e2e tier's two measured arms.
**Why:** No playbook rule governs the class: the host-tool-handle rule requires a PATH-valued handle read SOLELY by `wdio.conf.ts`, behind the `isFile`/metacharacter guard, in an array-form spawn (all four false); the harness-owned-constant rule requires the handle SET by `wdio.conf.ts` and READ by a `conductor-run` test binary (both false), and warns against stretching the host-tool rule. Escalated; the operator ratified a fourth class, mirroring `CONDUCTOR_E2E_SEED_DIR` as the third (2026-09-02), and a playbook rule was minted.
**Kept:** The two enumerations of PATH handles were deliberately NOT widened — a flag handle is outside their subject.
**Ref:** .andromeda/runs/2026-09-07T16-19-12Z-wrap/

## 2026-09-07-sr-findings-fixed — the CI arm's PRODUCER of `CONDUCTOR_MSEDGEDRIVER` now validates
**Section:** §Input Validation — the `CONDUCTOR_MSEDGEDRIVER` boundary row
**Change:** ONE clause added: the CI arm's producer of the value validates upstream of the wdio edge — `ci.yml`'s `a11y` job derives it in the step SHELL from the runner image's `$env:EDGEWEBDRIVER` behind a printed handle-named `Test-Path` precondition (handle NAME + boolean, never the path) and exits NON-ZERO when it does not resolve, unlike the dev host's skip at exit 0. An ADDED arm, never a relaxation: `wdio.conf.ts` stays the sole READER and the sole site applying the `isFile` + shell-metacharacter guard, byte-unchanged.
**Why:** Applied in re-derived, narrowed form. The proposal to record "a SECOND, CI-only reader" at four sites rested on a false premise — `ci.yml` WRITES the handle from `EDGEWEBDRIVER` and never reads it, so `wdio.conf.ts` remains the only reader. Only the primary was applied, restated as a producer-side validation fact rather than a reader-set change.
**Kept:** The three dependent sites, rejected on the operator's ruling: they stay literally true, and amending them would have written a second reader where the code has none, planting a premise a later chunk would have to disprove. Dependent-group atomicity did not bind because the primary survived in corrected form.
**Ref:** .andromeda/runs/2026-09-07T21-30-50-wrap/

## 2026-09-08-webview2-runtime-152-installed-in-job — spawn rule (b) 5 → 6 forms; a THIRD dependency class no lockfile gate can see
**Section:** Security Anti-Patterns → Code Patterns rule (b) · Dependency Security (new third-class paragraph + the supply-chain-integrity SKIP note) · Threat Model Summary → Infrastructure → CI/CD · Threat Model Summary → Networking (four sections; all four applied)
**Change:**
- Rule (b): was FIVE governed harness-spawn forms on two loci; now SIX on three — `.github/workflows/ci.yml` joins the dev-only driver stack and the routine `agent-run.{sh,ps1}` path. The sixth is the `a11y` job's `Install WebView2 Evergreen runtime 152+ (gate)` step running the fetched bootstrapper via `Start-Process -FilePath <bootstrapper> -ArgumentList '/silent','/install' -Wait -PassThru`: array-form argv, no shell string, no `Invoke-Expression`, no operator-supplied value. It is the FIRST governed form whose PROGRAM is neither repo-derived, repo-vendored nor a fixed OS/toolchain binary (it arrives from the network at job time), so its admitting control is the pre-execution Authenticode check, both arms dev-host validated (a byte-corrupted copy returned `UnknownError` and was rejected).
- §Dependency Security gains a THIRD dependency class beside the Rust crate graph and the npm tree: a CI-time-fetched third-party binary in NO lockfile, to which `cargo audit` / `cargo deny` / `npm audit` are structurally blind. Version disposition: always-latest Evergreen while the chunk is a probe, pinned once the job gates; in-step floor major ≥ 152.
- The supply-chain-integrity SKIP is scoped to artifacts Conductor PRODUCES; consumption-side signature verification is an ACTIVE control.
- §CI/CD's "the supply-chain steps are unchanged by it": the STEPS are unchanged, the audited SURFACE is not.
- §Networking's egress enumeration and its "no public/VPN networking" clause are scoped to shipped binaries and dev-host harness legs, naming the CI-only exception.
**Why:** All escalated under the boundary-widening (never-routine) rule and operator-ratified; precedent: the 4 → 5 widening at `2026-09-06-run-report-envelope-conformance-gate` was ratified with no routine rule minted, so each further crossing escalates again. The operator added the Dependency Security half: a signed installer fetched and executed at CI time is a supply-chain dependency whose control is the Authenticode check. The Networking site's "no public/VPN networking" clause was false as written.
**Kept:** The "loopback-only egress" claim — product-scoped to Conductor-the-program, still true; and `supply-chain-signing-init — Standard + Hardened only` — it concerns Conductor SIGNING its own artifacts, the opposite direction from verifying a consumed one.
**Ref:** .andromeda/runs/2026-09-08T20-30-00-wrap/

## 2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate — the CI/CD enumeration gains the fmt gate
**Section:** Threat Model — CI/CD
**Change:** The CI/CD bullet's enumeration of what GitHub Actions runs (was "`cargo build` / cargo-nextest (incl. golden tests) / `cargo clippy`, plus the `a11y` job's Pulse-free routine webview leg") now leads with `cargo fmt --all --check`, the `rust` job's early formatting gate (2026-09-09). No other security clause moves.
**Why:** Raised by the cross-master sweep, not a detector — "what CI runs" belongs to no security detector's invariant.
**Kept:** Rule (b)'s SIX governed harness-spawn forms enumeration does NOT move: a plain `run:` step invoking a fixed toolchain binary with fixed argv is the class of the existing `cargo build` / `cargo audit` steps, none counted among the six (those are constructed-argv spawns with a program the step supplies). No seventh crossing, so no boundary-widening ratification is owed.
**Ref:** .andromeda/runs/2026-09-09T13-20-08-wrap/
## 2026-09-10-release-build-and-bundle — bundler split from the crate in the CVE floor; hosting figure retired
**Section:** Threat Model Summary — Infrastructure (Hosting) · Dependency Security (Update policy)
**Change:**
- §Hosting: was "optional Tauri 2 (2.11.3) ~3 MB GUI bundle"; now the measured facts — the bundle is produced by the host `tauri-cli` dev-tool (absent from `Cargo.lock`, first install 2.11.4) as the Windows installer set, measured 2026-09-10 at nsis 4.21 MB · msi 5.87 MB, both under gitignored `target/`, neither committed.
- Update policy: was "`tauri` (+ bundler) ≥ 2.10.3 … the floor is met — the tree resolves 2.11.3". The CRATE half is byte-intact; "(+ bundler)" is split out, since nothing in the tree resolves a bundler version. The bundler keeps the same ≥ 2.10.3 floor, now a HOST-TOOL floor verified by probe (`cargo tauri --version`), not read off the lock; measured 2.11.4.
**Why:** The floor edit touches a security FLOOR sentence with no governing playbook rule, so it was escalated and operator-resolved: a floor whose stated verification mechanism does not exist for half its subject reads as lock-verified when nothing verifies it. The CVE substance (origin-confusion CVE-2026-42184) is unaffected, only the attribution. The Hosting edit is the cross-master co-citation of the architecture claim.
**Kept:** The site naming `tauri` the crate with no bundler extension — not a hit.
**Ref:** .andromeda/runs/2026-09-10T20-36-29-wrap/

## 2026-09-11-hosted-runner-endpoint-cause-closed — the Evergreen fetch made floor-conditional at all three of its sites
**Section:** Dependency Security (third dependency class — version disposition) · Threat Model Summary — Infrastructure (Networking) · Threat Model Summary — Infrastructure (CI/CD)
**Change:** One claim at three sites.
- §Dependency Security: was "**Version disposition — always-latest Evergreen, deliberately and temporarily:** the chunk that added it is a PROBE whose subject is the runtime MAJOR, so fetching the newest is the measurement rather than a build choice"; now **floor-conditional Evergreen** — the fetch+install runs only when the step's measured pre-install runtime major is below 152, so an image at or above the floor is never fetched for. The rationale is recorded EXHAUSTED, not contradicted: the runtime major was falsified as the discriminator; the subject is now the driver/runtime SKEW, which fetching the newest CREATES. The `≥ 152` floor assertion sits OUTSIDE the conditional and runs every job; the Authenticode gate guards the only fetching path; neither step carries `continue-on-error`.
- §Networking: the one non-loopback egress is CI-only AND CONDITIONAL, not reached at all at or above the floor.
- §CI/CD: the audited-surface widening is entered conditionally; admitting control unchanged.
**Why:** Escalated with no governing playbook rule (the posture-change rule fails: the sentence's own retire-condition, "pinned once it actually GATES", is measurably unmet while the posture changed for a different reason) and resolved by operator directive, which ruled the framing EXHAUSTED-not-contradicted. No playbook rule proposed: routinising a posture change whose own retire-condition is unmet is the silent precedent-widening the escalate branch exists to prevent. The two Threat Model sites carry no `always-latest` token; a single-site apply would have left the unconditional acquisition standing. The boundary-widening rule does NOT match — this narrows the CI-time fetch.
**Ref:** .andromeda/runs/2026-09-12T10-08-17-wrap/

## 2026-09-16-scenario-assertion-audit-gate — the scenario-audit ledger, and the second test-binary reader
**Section:** §Input Validation — the committed-SUT-facing-manifests row
**Change:** The row's subject enumeration gains `contracts/scenario-audit-ledger.toml` as a fourth member, and its per-READER clause names TWO test-binary readers (was one): `conductor-report --test coverage_gate` (2026-09-06) and `conductor-core --test scenario_audit_gate` (2026-09-16), both resolving the workspace root from `CARGO_MANIFEST_DIR` because a cargo test binary's CWD is its own package root. The ledger's shape is stated: serde typed parse with a hard `CoreError` on absent or malformed (never defaulted, never a silent widen); `default_path()` a hard-coded relative constant and `load()` taking an ALREADY-RESOLVED path, so `conductor-core` never calls `resolve_under` for it; no `CONDUCTOR_*` override handle; garde n/a (not a scenario-config struct); read faults carry `e.kind()` only, never the path. It is the FIRST member of this row with no shipped reader at all.
**Why:** A committed artifact parsed at runtime is a new external-input surface, §Input Validation enumerates those exhaustively, and the duty binds per READER — an unrecorded reader is the drift. The committed-manifest playbook rule did NOT govern (its clause requires a fixed `default_path()` through `resolve_under`), so the proposal escalated; the operator applied both halves (the boundary row AND the per-reader record): the artifact earns its row whether or not a shipped binary parses it. The operator also ratified minting a narrower playbook rule for the test-binary-reader class, its second occurrence with an identical resolution.
**Kept:** The row's `resolve_under` mechanism sentences describe the three SHIPPED-reader manifests and remain true of them; this member's divergence is stated explicitly rather than by widening theirs.
**Ref:** .andromeda/runs/2026-09-16T08-43-09-wrap/

## 2026-09-16 — a11y-ci-gate-at-an-honest-terminal
**Section:** §Security Anti-Patterns → Code Patterns rule (b)
**Change:** The governed harness-spawn registry moves SIX → SEVEN, the seventh registered as it SHIPS: the `a11y` job's asserting step launching the routine leg through `scripts/a11y-limited-token-launch.ps1`, a `runas /trustlevel:0x20000` launch whose leg entry point is `scripts/a11y-token-witness.ps1`. The rule's closing sentence gains a qualifier: the seventh is the one exception to the array SHAPE and to no other clause.
**Why:** Rule (b) forbids a shell / `eval`-equivalent with operator-supplied input; this form is neither — `-File` only, never `-Command`, no operator-supplied value in argv. Operator ruling 2026-09-16: the master is NOT violated and the chunk plan's "every supplied value is a separate array-form argv element" was over-strict, so this is an enumeration amendment, never a relaxation. The registered mechanism is the one that SHIPS: a scheduled-task form was replaced by `runas` within the chunk and registering it would have recorded a mechanism that does not exist. The composition is forced by the API (`New-ScheduledTaskAction -Argument` is typed `System.String`; `runas` takes one command string), so a guard stands in for the array form: whitespace and shell metacharacters rejected on every element before composing, measured at exit 90 with a named `A11Y_LIMITED_TOKEN_ARGV: REJECTED` precondition on space- and `;`-bearing controls. Escalated under the boundary-widening rule (always a human's call, never a routine rule however often it recurs) and operator-ratified; an eighth crossing escalates again.
**Kept:** Rule (a) — the MCP sidecar spawn, `.env(...)` handling, the `2024-11-05` pin — unchanged; the `Get-AuthenticodeSignature` sites (the sixth form's admitting control) unchanged.
**Ref:** NOT DERIVED

## 2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration — third dependency class swapped; spawn-rule (b) forms six and seven dispositioned
**Section:** §Threat Model Summary (Infrastructure → Networking · CI/CD) · §Dependency Security (third dependency class · the supply-chain SKIP note) · §Input Validation (`CONDUCTOR_MSEDGEDRIVER` row) · §Security Anti-Patterns → Code Patterns (spawn rule (b), sixth and seventh governed forms)
**Change:**
- The THIRD dependency class keeps exactly one member and SWAPS it: the WebView2 Evergreen bootstrapper retires with its install step; Microsoft's version-pinned msedgedriver, fetched at the version the image's own WebView2 runtime names, replaces it under the same sole `Get-AuthenticodeSignature` control. The `≥ 152` floor is retired as falsified: not necessary (a coherent 131/131 pair runs green), not sufficient (a coherent 152/152 pair fails), destructive on the working configuration. The float's EXIT CONDITION is rewritten — the Standalone Installer installs Evergreen and takes no version, and Fixed Version cannot supply 131 — so the float retires by pinning the DRIVER to the runtime. A clause distinguishes this coherence from the separately-retired skew cause.
- §Networking: the sole non-loopback egress is the unconditional, version-derived `msedgedriver.microsoft.com` fetch (count one before, one after). §CI/CD: the fetching step verifies and stops; the LEG executes the fetched binary.
- `CONDUCTOR_MSEDGEDRIVER` row: `EDGEWEBDRIVER` is written by the pin gate through `GITHUB_ENV` (was read from the runner image); the `Test-Path` precondition and `wdio.conf.ts`'s sole-reader `isFile` + metacharacter guard are byte-unchanged.
- Spawn rule (b): the SIXTH form is RETIRED with its step, ordinal not reused, but its property — a PROGRAM arriving from the network at job time — TRANSFERS to the dev-only driver-stack spawn, since `CONDUCTOR_MSEDGEDRIVER` now holds the fetched driver. The SEVENTH is RETAINED, scope corrected to the job's driver-alone diagnostic steps (four `ci.yml` call sites, was five), never the asserting step, which invokes the leg's entry point directly at native High integrity.
**Why:** Rule (b)'s class always escalates; both forms were operator-ratified 2026-09-17: a registry silent about a spawn a committed workflow still performs fails like one describing a spawn it no longer performs (seventh), and retiring the network-program property would orphan the Authenticode control's justification while the job still executes a fetched binary (sixth). The egress change is a NET NARROWING: the fwlink had no version selector, this URL's version derives from the image; `ci.yml` makes exactly two network calls, the driver fetch and loopback `127.0.0.1:9515`.
**Ref:** .andromeda/runs/2026-09-17T10-34-20-wrap/

## 2026-09-22-interpretation-proven-live — the posture's absence arm, the capture's crossings, and the one ratified corpus exception
**Section:** Threat Model Summary → Data classification (corpus note) · §Input Validation rows: Scenario config · Committed SUT-facing manifests · the `ANDROMEDA_PULSE_*` READ SET · Env-var path handles · NEW Real-model capture ingest · `ANDROMEDA_PULSE_DATA_DIR` spawn · CLI arguments · §Data Protection (encryption at rest) · §Bootstrap phases (`input-validation-library-install`) · §Security Anti-Patterns → Input (the per-READER clause) · → Data Protection (the corpus bullet) · → Universal (the probe short-circuit)
**Change:**
- `l4_posture`: a CLOSED unit enum, no catch-all; unknown ⇒ load-time `CoreError::Config`; its `garde(skip)` is the closed-unit-enum form, not the banned nested-spec skip.
- Manifests: every SHELL term (`shell-declaration`, `shell-absence`) names its env var; a term's optional `posture` is from the closed `L4Posture` set.
- READ SET + probe: deterministic posture keeps declaration grading; under real-model, `handle_declared_for` requires the L4 handle ABSENT or falsy under `flag_declared_on_either_side` (Conductor `true`/`1` ∪ Pulse `1`/`true`/`yes`), a union that only widens what BLOCKS; the run-contract path reaches only value-only tests; `run --live real-model` leads with `preconditions --for real-model-interpretation`. "No value ever becomes a path" is scoped to the READ SET's own read.
- CLI: `conductor preconditions --for <SCENARIO>` (validated load; `P-NNN` refused at parse, exit 2); `run --live [real-model]` (closed allowlist; unknown ⇒ usage + exit 2 before any probe).
- NEW capture ingest: untrusted SUT output (an MCP re-read, Pulse's own log) via `redact_value` + `mask_host_paths`, fields-only, fingerprints elided, written once. Data-dir row: the capture is a TEST-binary VALUE reader with no canonicalize (residual).
- Two more `CONDUCTOR_RUNS_DIR` test-binary readers join with no `resolve_under`: `real_model_live.rs` (failures name display name + error kind) and the unrecorded `live_suite.rs` (its panic prints the resolved path to test output, never an artifact). Hardening both is route-owned.
- Corpus ban gains ONE ratified, scoped exception: corpus-rendered model text the real-model capture re-reads over MCP may enter a chunk's committed `evidence/` tree, only via the same two scrubbers, fingerprints elided; the Data-classification note and encryption-at-rest text carry it too. The 2026-09-23 capture carries none.
- Bootstrap + Input per-READER lists gain the two capture readers.
**Why:** Posture, manifest-term and CLI rows had no governing rule; the operator minted one per class. The rest is Boundary-widening (SUT data into the repo; a handle value becoming a path), so it escalated: operator-recorded, hardening routed (a working-route CARRY), exception ratified scoped.
**Ref:** .andromeda/runs/2026-09-23T08-03-55-wrap/

## 2026-09-23-real-model-capture-path-handles-guarded-and-stale-read-back-texts-corrected — the test-binary path-handle residuals closed
**Section:** §Input Validation (env-var path-handles row `:115`; real-model capture ingest row `:121`; `ANDROMEDA_PULSE_DATA_DIR` spawn row `:122`) · §Bootstrap phases → input-validation-library-install (`:221`) · §Security Anti-Patterns → Input (`:325`)
**Change:** The three `CONDUCTOR_RUNS_DIR` test-binary readers (`journal_conformance.rs`, `real_model_live.rs`, `live_suite.rs`) resolve the handle through the shared test module `conductor-run/tests/capture_paths` (`runs_dir_from` → `conductor_core::resolve_under`), so an absolute or `..` value is REJECTED with a path-free reason instead of replacing the workspace root; `live_suite.rs`'s failure texts name the file and error kind, never a path. The `ANDROMEDA_PULSE_DATA_DIR` VALUE read in `real_model_live.rs::pulse_log` is canonicalized and required to be a directory before `logs/` is joined. Held by the test target `capture_paths_guard`. Was: an unguarded join, a path-printing panic and a no-canonicalize value read (residuals recorded 2026-09-06 / 2026-09-22 / 2026-09-23); now retired at all five sites.
**Why:** The chunk shipped the hardening the master had recorded as route-owned; `journal_conformance.rs`, recorded as a residual but never routed, was brought in by operator ruling. The playbook rule "a chunk SHIPS the fix that a spec master's own body already names as route-owned-not-shipped" covers the routed readers; its route-entry clause does not match an unrouted residual, which needs an operator direction. Nothing new crosses a boundary: the guard narrows what the readers accept.
**Kept:** the `CONDUCTOR_E2E_SEED_DIR` residual clause, still true, unchanged.
**Ref:** .andromeda/runs/2026-09-23T20-49-35-wrap/

## 2026-09-24-secret-scanning-ci-gate — the secret-scanning CI gate realized; tool selection recorded
**Section:** §Bootstrap phases `secret-scanning-ci-gate` · §Secret Management "Secret scanning in CI" · §Security Anti-Patterns → Secrets (the `.env` NEVER bullet) · §Threat Model Summary → CI/CD · §Security Decisions Log (new dated entry)
**Change:** The gate was "optional defense-in-depth, tool selection deferred"; now realized. The `rust` job's `Secret-scan gate` runs `crates/conductor-core/tests/secret_scan_gate.rs` (content rules + a secret-file-name class over git's cached + untracked-not-ignored listing, an exact-set allowlist, a hit never echoing its match). `.gitignore`'s secret classes widen to `.env*` plus `*.pem *.p12 *.pfx *.key *.cer *.crt *.jks *.keystore id_*`. The CI/CD threat-model line lists the secret-scan and workflow env-context gates (in-repo test targets, no fetched tool). A new 2026-09-24 Decisions Log entry records the operator's selection of an in-repo Rust static gate over a job-time-fetched scanner, and closes Initial-entry open question (3).
**Why:** The operator selected the in-repo gate because a job-time-fetched scanner would have been a second third-class dependency member and a second CI egress. No egress, spawn-form or dependency-class count moves.
**Kept:** the historical Initial entry's open question (3) is left untouched, as the log requires — historical entries are never modified; the new entry closes them.
**Ref:** .andromeda/runs/2026-09-24T14-02-12-wrap/

## 2026-09-24-secret-scanning-ci-gate — rule (b)'s scope: a cargo test binary's fixed spawn is outside the governed forms
**Section:** §Security Anti-Patterns → Code Patterns, rule (b) (the harness-spawn rule)
**Change:** A closing clause records that a spawn from a cargo TEST binary sits outside rule (b)'s three loci (the driver stack · `scripts/agent-run.{sh,ps1}` · `.github/workflows/ci.yml`) and adds no governed form. The instance is `crates/conductor-core/tests/secret_scan_gate.rs`'s `git -C <repo root> ls-files -z --cached --others --exclude-standard`: a fixed PATH-resolved program, fixed argv, no operator-supplied value, never a shell string. Like the test binaries' `CARGO_BIN_EXE_*` spawns it composes nothing, so the count stays seven.
**Why:** Escalated and operator-directed: record it as outside rule (b). It is the workspace's first test-binary spawn of a PATH-resolved external program; recorded so the next reader need not re-derive the scope question against the rule's "an EIGHTH crossing escalates" clause. It is NOT an eighth governed form and NOT a boundary widening, since nothing crosses.
**Ref:** .andromeda/runs/2026-09-24T14-02-12-wrap/

## 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin — the capture ingest row's new line classes and scrub, and the exception's recorded breach
**Section:** §Input Validation → Real-model capture ingest row · §Security Anti-Patterns → Data Protection (the `corpus.db` ban's exception)
**Change:**
- Ingest row, what the capture writes and prints:
  - One `evidence/rm-capture-{drive}.txt` per series drive.
  - The MCP re-read prints the report body verbatim from its first `## ` line.
  - The id sweep reads at most 64 ids, from the highest id seen minus 63, floor 1 (was "an id sweep to 64").
  - It also prints Pulse's `creating digest corpus retrieval rows:` line, the `canary:` lines and `scenario_storm=` on storm lines.
- Ingest row, the scrub:
  - Every printed line passes `redact_value`, then `mask_host_paths`, then `elide_fingerprints` (was: only the envelope fingerprints elided, to counts).
  - The capture itself never prints the workspace basename, but the verbatim report body shows Pulse's rendering of the workspace key (measured `[redacted: credit_card]`). Masking it before the next series is owned by the new `v3-09` series route entry.
- Data Protection:
  - The exception's scrub is now `redact_value` + `mask_host_paths` + `elide_fingerprints`, over every fingerprint-shaped token (was "envelope fingerprints elided").
  - The 2026-09-29 b2 capture carries a report body.
- A BREACH is recorded, never ratified:
  - The series pins in `crates/conductor-run/tests/real_model_series/mod.rs` put report text in test source, outside the `evidence/` tree.
  - The 2026-09-22 capture and `PINNED_CAPTURE` still carry one un-elided `fingerprint_hex` prefix.
  - Both remedies (a sha256 digest pin per evidence file; that prefix elided) are owned by the new `v3-09` series entry, and land before the next series.
**Why:**
- The chunk widened what the capture prints (the report body, for P-031/P-034/P-044) inside the ratified exception, and fixed a scrub gap.
- Escalated at this wrap. Ratifying the test-source copies would widen the exception, and a widening waits for the founder's live word (his 2026-09-27 ruling), so the overseer recorded a breach with a route-owned remedy.
- The workspace-key mask keeps "never printed" true without widening.
**Ref:** .andromeda/runs/2026-09-29T17-50-46-wrap/

## 2026-09-29-dual-license-mit-or-apache-2-0 — own crates license-checked
**Section:** §Dependency Security (Accepted exceptions)
**Change:** The paragraph now states that the workspace's own `conductor-*` crates are NOT exempt from the license policy. `deny.toml` carries no `private` exemption (was `private = { ignore = true }`, "unpublished — skip license checks"), so `cargo deny check licenses` checks them like any dependency. They pass as `MIT OR Apache-2.0`, inherited from `[workspace.package]`; a member without an allowed license fails the gate, and `publish = false` exempts nothing. The allow set needed no entry (`MIT` / `Apache-2.0` were already allowed), and the 16 `[advisories] ignore` + 9 `[licenses] allow` counts hold unchanged at 2026-09-29.
**Why:** The public repository was dual-licensed `MIT OR Apache-2.0` on the founder's direction. The deny fork was decided "check them" by the overseer (founder-delegated): a gate that skips our own crates proves nothing about them. The change NARROWS an exemption, so it is no boundary widening. The gate was measured red (nine `error[unlicensed]`) before the manifests carried the license, and green after.
**Ref:** .andromeda/runs/2026-09-29T19-16-23-wrap/

## 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir — breach remedied, workspace key masked
**Section:** §Input Validation (the real-model capture ingest row) · §Security Anti-Patterns → Data Protection
**Change:**
- Ingest row, the scrub:
  - Every printed line now passes `mask_workspace_key` FIRST, then `redact_value` → `mask_host_paths` → `elide_fingerprints` (was three stages).
  - Every `workspace=` line's value is masked whatever it holds, and the key is masked wherever bounded outside `[A-Za-z0-9_.-]`.
  - The key is the data dir's basename, taken from the guarded `capture_paths::pulse_logs_dir_from` path, never a raw handle read.
  - Pulse stamps the key as a PATH (`fcc31b2` `workspace_key`), so the leaf matches only when its detection falls back to the data dir.
  - A class-only witness line `pulse-report workspace rendering:` replaces the value (was "masking it … owned by the new `v3-09` series route entry").
- Ingest row, the reader: `real_model_harvest` checks each committed capture against a sha256 digest pin and grades the file after the digest matches (was "asserts the pinned literal EQUAL to its section").
- Data Protection:
  - The exception's scrub list leads with `mask_workspace_key`.
  - The 2026-09-29 BREACH now reads REMEDIED, never ratified: digest pins, no capture text in test source, the graded 2026-09-23 copy elided.
  - Stated residual: the frozen 2026-09-22 file keeps its one `fingerprint_hex` prefix.
**Why:**
- This chunk shipped both remedies the body named route-owned. It narrowed what crosses the exception and widened nothing.
- The 2026-09-30 series measured the leaf 0 times across its three captures. The source arm's inverse control read red with a report line planted and green without.
**Ref:** .andromeda/runs/2026-09-30T07-22-03-wrap/

## 2026-09-30-the-sr-cause-isolated-on-this-host — two ratified session-level controls outside rule (b)'s loci
**Section:** §Security Anti-Patterns → Code Patterns rule (b), the outside-the-loci record
**Change:** Rule (b) now records two founder-ratified session-level crossings on the dev host, 2026-09-30, that sit outside the three loci and add no governed form:
- arm W — a gitignored session script that validated `CONDUCTOR_MSEDGEDRIVER` the wdio way, started that driver with the fixed argv `--port=4445` on the registered dev-only port, drove ONE WebDriver session to the Edge browser and tore both down;
- arm 153 — the existing `sr-empty` leg with an operator-supplied msedgedriver 153.0.4234.48, admitted by a pre-execution `Get-AuthenticodeSignature` check (`Valid`, an `O=Microsoft Corporation` signer) plus a SHA-256 and version match BEFORE the binary executed at all.
It states the standing rule "a binary is never executed before its signature verdict is read". Both are CONTROLS only; no committed form moved; the count stays seven. The founder's ratification is quoted in the body.
**Why:** Boundary widening, always a human's call: ratified live by the founder at P4 as controls, relayed by the overseer, and its record's placement ruled at this wrap by the overseer (founder-delegated). The pre-execution rule answers a plan defect — the plan's admission entry ran the driver's `--version` in the same command as its signature check, whatever the signature said.
**Kept:** The `4444`/`4445` binder lists (the committed suite families) are unchanged: W was a one-off session crossing, recorded here.
**Ref:** .andromeda/runs/2026-09-30T13-50-58-wrap/

## 2026-09-30-the-sr-pass-regrades-on-the-os-input-path — rule (b) registers its eighth governed form
**Section:** §Security Anti-Patterns → Code Patterns, rule (b); §Input Validation → the screen-reader speech-log ingest row
**Change:**
- Rule (b) was "seven governed forms"; now **eight**. The EIGHTH is `crates/conductor-tauri/ui/test/a11y/screen-reader/send-keys.ps1`, the `sr*` legs' per-key OS input spawn:
  - `spawnSync('powershell.exe', [...,'-File', <send-keys.ps1>, '-Key', <constant>])`, `windowsHide`, 20 s, `-File` only, never `-Command`;
  - `-Key` is a closed `ValidateSet` (`Tab` · `ShiftTab` · `h` · `d` · `ArrowDown`), so no operator value is in the vector;
  - its new capability, `SendInput` keystroke synthesis, is bounded by a foreground guard: exit 4 = nothing sent unless the foreground process is `conductor-tauri`; exit 5 = a short insert or Shift still down; a non-zero exit throws with no injected fallback;
  - no listener; array-form, so the seventh stays the sole array-SHAPE exception.
- "The four driver-stack forms" is now five, adding the OS-key form beside the window-activation form.
- The `secret_scan_gate` carve-out was "the count stays seven"; it now "moves no count".
- The W/153 carve-out was "no committed form moved, and the count stays seven"; now no committed form moved by them, their ratification stays scoped to those controls, and the eighth is a separate ratification.
- The speech-log ingest's closed sets gain `input` (`os` · `webdriver` · `mixed` · `none`).
**Why:** a boundary widening (a subprocess boundary gaining a new crossing), escalated under playbook `:124` and ratified live by the founder, relayed by the overseer. The class keeps escalating: a ninth crossing escalates again.
**Kept:** the ordinals "The SEVENTH is …" and "the seventh form the one exception to the array SHAPE" — still true of the seventh form.
**Ref:** .andromeda/runs/2026-09-30T15-22-00-wrap/

## 2026-09-30-full-gate-regression-over-the-moved-surfaces — the `run` target refuses an ambiguous P-ID
**Section:** §Input Validation (CLI arguments / stdin row)
**Change:** The row names the `run` target (a scenario name or a P-ID) beside the seed/scenario flags, the `run_id` positional and the `preconditions --for` scenario name. A P-ID that several scenarios name is refused BEFORE any scenario load — an `anyhow` harness fault naming the P-ID, the count and the matching scenario file stems (never a path), `error:` + `hint:` at exit 1; the harness's `SCENARIO=` inherits it; a single-owner P-ID still resolves. Was: the first `read_dir` match, taken silently.
**Why:** The refusal narrows what the boundary admits — no new crossing, no write, no new value reaching an argv, shell or path — so it records validation rather than widening it; the row already justified `--for`'s parse refusal by "a P-ID can name several scenarios".
**Ref:** .andromeda/runs/2026-10-01T00-02-40-wrap/

## 2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix — the capture's skip witness; an all-digit residual pending the founder
**Section:** §Input Validation (the real-model capture ingest row) · §Security Anti-Patterns → Data Protection (the corpus.db ban's capture exception)
**Change:**
- The ingest row: since 2026-10-01 the capture also prints Pulse's `interpretation.incident.skipped` lines fields-only (`skip_reason`, `decision`, `severity`, `digest_kind` — closed enums Pulse's code writes, allowlisted on its side) and ends each `canary:` line with its inference's `skip_reason`, through the unchanged four-stage chain.
- The elision guarantee now states its definition: a fingerprint-shaped token is eight or more lowercase hex digits holding at least one letter, so an all-digit run passes. Was: "a fingerprint-shaped token prints as `<fingerprint>`" / "every fingerprint-shaped token elided", read as covering every fingerprint value.
- The `workspace_key` coordinate: at `fcc31b2`, unchanged at `a2addb3`. The key-leaf fallback and the 0-occurrence rendering measurement now cite the 2026-09-30 and 2026-10-01 series.
- The report-body enumeration: the 2026-10-01 series' d1 and d3 captures carry one report body each, d2 none.
- A second residual: the GRADED 2026-10-01 `rm-capture-d3.txt` carries one all-digit 8-character `fingerprint_hex` prefix on two lines (the preflight canary's synthetic storm), counted exactly by the harvest; no elision code changed. Recorded as overseer-ruled, founder ratification PENDING.
**Why:** escalated under playbook `:124` (Boundary widening). The overseer ruled the residual and held that a delegate does not decide whether it is a widening, so it is recorded neither as ratified nor as a breach; the founder's word is asked beside `v3-09`'s next step, and a later wrap records it.
**Kept:** the 2026-09-22 residual sentence and its frozen-evidence basis, unchanged; the 2026-09-30 clauses as true history.
**Ref:** .andromeda/runs/2026-10-01T20-39-22-wrap/

## 2026-10-01-per-run-span-identity-in-the-real-model-harness — the span-landing witness as a guarded test-binary reader
**Section:** §Input Validation (path-handle row · new Span-landing witness ingest row · `ANDROMEDA_PULSE_DATA_DIR` spawn row · declaration-only read-set row) · §Security Anti-Patterns → Input · the phase-scoping paragraph (`:222`)
**Change:**
- `CONDUCTOR_RUNS_DIR`'s test-binary readers go from THREE to FOUR, adding the operator-gated `conductor-run/tests/span_landing_live.rs` (2026-10-01). It resolves the handle through `capture_paths::runs_dir_from` → `resolve_under` from birth. The same enumeration is restated at §Security Anti-Patterns → Input ("two … captures" / "all three" → three / "all four") and at the phase-scoping paragraph ("Those three" → "Those four").
- A new §Input Validation row, **Span-landing witness ingest**, records untrusted SUT output: every `agent-latest.jsonl*` under `ANDROMEDA_PULSE_DATA_DIR`'s `logs/`, plus the frozen `span-{a,b}.jsonl` journals.
  - It reads both handles through the shared `capture_paths` guards: canonicalize + is-dir, and `resolve_under`. Failures are path-free (`e.kind()` only).
  - It does one bounded `serde_json` decode per line.
  - It prints exactly ONE integers-only `span-landing: PASS|FAIL …` line and commits nothing from the ingest, so no scrub chain is owed.
- The data-dir handle's test-binary VALUE reader becomes plural (the real-model capture and the witness) in the spawn row and the read-set row. The witness spawns no sidecar, so neither reader relaxes or satisfies the spawn duty.
**Why:** the duty binds per READER and each reader is recorded, never assumed. The validation was present from birth, so this is a record of a validated boundary, not a widening. Playbook `:124` was considered and its precondition failed: no new input class (the same handle, the same guard module, the same Pulse-log class `real_model_live` already ingests), no new crossing, no write. It applied as routine under `:308`.
**Ref:** .andromeda/runs/2026-10-01T23-55-00-wrap/

## 2026-10-02-p-075-assert-round-against-pulse — span-landing ingest path and stale-pair refusal
**Section:** §Input Validation → the span-landing witness ingest row
**Change:**
- The two frozen Conductor self-obs journals now read `runs/span-landing/span-{a,b}.jsonl` under `CONDUCTOR_RUNS_DIR`. The firing form clears those two named files, non-recursively, before drive A. Was `runs/live-suite/span-{a,b}.jsonl`.
- The How column adds that a STALE pair is refused before any grading. `pair_is_current` requires drive A at or after the live Pulse log's first line, drive B at or before its last, and A before B. A pair left from an earlier pass is therefore never graded against the current log.
- The guards are unchanged: `capture_paths::runs_dir_from` → `resolve_under` and `pulse_logs_dir_from`. The `CONDUCTOR_RUNS_DIR` test-reader count stays at four.
**Why:** the chunk moved the pair to its own dir, out of reach of the `--live` suite's `rm -f`, and added the stale-pair refusal. The refusal narrows what is graded and widens nothing, so the row records it as validation.
**Kept:** the `ANDROMEDA_PULSE_DATA_DIR` reader rosters (the declaration-only READ SET row and the spawn-propagation row) are unchanged. The new `p075_round_live` passes the handle only to the shipped `ReadbackClient::connect` and joins no path, like `lifecycle_live.rs`, which no row lists, so it is not a VALUE reader.
**Ref:** .andromeda/runs/2026-10-02T12-53-46-wrap/

## 2026-10-02-captured-fingerprint-values-elided — both capture fingerprint residuals fixed; the keyed elision rule
**Section:** §Input Validation → the real-model capture ingest row · §Security Anti-Patterns → Data Protection (the `corpus.db` exception: its elision clause and its residual statements)
**Change:**
- Ingest row: `elide_fingerprints` is now two rules. KEYED: the alphanumeric value after every `fingerprint_hex=` prints `<fingerprint>` whatever its class, all-digit included; an existing `<fingerprint>` or an empty keyed value is left as it is. This is the shape of the `workspace=` rule. UNKEYED: unchanged — a fingerprint-shaped token is elided, and an unkeyed all-digit run (a stamp, a seed) passes.
- The ingest row now records the harvest's population read: every committed `rm-capture*.txt` under the chunks' `evidence/`, workspace-root anchored with no handle, held at zero un-elided keyed values and a fixed point of the elision, the population pinned by count beside an inverse control.
- The row's "the 2026-10-01 d3 residual" pointer is retired.
- Data Protection: the exception clause was "every fingerprint-shaped token elided — an all-digit run … passes"; it is now every `fingerprint_hex=` value of any class plus every unkeyed fingerprint-shaped token.
- Data Protection: both stated residuals are retired as FIXED (was: the frozen 2026-09-22 file keeps its prefix; the d3 all-digit prefix overseer-ruled, founder ratification pending, per the 2026-10-01 entry "the capture's skip witness; an all-digit residual pending the founder"; now both are fixed).
- The frozen file was elided in place, a recorded exception to "frozen evidence is never edited", and now equals the graded copy.
- d3 was re-elided with its pin moved, and the 2026-10-01 harvest arm asserts zero for every drive.
- No committed quote of either value remains in the tree; git history was not rewritten.
**Why:** The founder ruled the residuals FIXED, never ratified (2026-10-02, relayed by the overseer), and the overseer answered the frozen-file fork as elide-in-place. The change is a narrowing: more is elided, and no input class or write is admitted. A keyed value is a fingerprint whatever its digits, while an unkeyed all-digit run stays a stamp or a seed, so widening the shape rule instead would break every capture's fixed point.
**Ref:** .andromeda/runs/2026-10-02T16-24-28-wrap/

## 2026-10-03-p-075-re-round-on-incident-events — the pinned manifest's fifth tool and the new MCP-child-stdout read
**Section:** §Input Validation → Pinned MCP contract manifest (`contracts/`) · §Input Validation → MCP child stdout (trusted-child boundary)
**Change:**
- The pinned manifest's required-tool name set was four; it is five, adding `retrieve_incident_events`. A sidecar lacking any one blocks on the EXISTING `required tool(s) absent` precondition, so the gate's named preconditions stay five.
- The MCP-child-stdout boundary now also names the `retrieve_incident_events` by-id READ on the same `call_tool` path and controls: its response `{incident_id, events[{event_kind, occurred_unix_nano}], total, truncated}`, and its unknown-id arm, a JSON-RPC error carrying `incident not found` that lands as a typed `VerifyError::JsonRpc`, never `Ok` and never a panic.
**Why:** A Boundary widening, the MCP child stdout admitting a new input class: ratified on the founder's live word, relayed by the overseer 2026-10-03. The validation it needs is the boundary's existing bounded decode plus the typed error wall, both stub-proven.
**Kept:** the corpus-access sentences ("read-back plus the `mark_incident_resolved` lifecycle write") stay as written — the new tool is a read, which "read-back" already covers. The dated 2026-09-04 "4-of-4 tools" `ReadyState` reading stays as measured.
**Ref:** .andromeda/runs/2026-10-03T23-44-32-wrap/

## 2026-10-04T01-45-46-wrap — registry migration (U35): the security-plan Decisions Log leaves the body
**Section:** §Security Decisions Log · §Secret Management
**Change:** The Security Decisions Log moved verbatim to security-plan-amendments-archive.md (2 entries: the 2026-06-14 Initial entry and the 2026-09-24 secret-scanning tool selection). One lift: §Secret Management gains "Secret-scan gate shape" after "Secret scanning in CI" — the gate as a `conductor-core` test target on the already-locked `regex` dev-dependency (no new package), a presence-guarded CI step plus the local `agent-run run` nextest, in-suite negative arms proving it can fail, and the accepted trade-off (a self-maintained rule set, no entropy scoring) over a job-time-fetched scanner. Every other in-force item already stands in the body: the Minimal (0) tier and its justification (§Threat Model Summary), sidecar spawn hardening by a fixed program NAME resolved through `PATH`, which supersedes the Initial entry's "program path" (§Security Anti-Patterns → Code Patterns), the `tauri` ≥ 2.10.3 and toolchain ≥ 1.94.1 floors (§Dependency Security · §Universal), the scoped `shell-open` ban (§Code Patterns), the cargo-audit / cargo-deny posture (§Dependency Security), and the realized secret-scanning gate (§Bootstrap phases · §Secret Management · §Threat Model Summary CI/CD · §Security Anti-Patterns → Secrets). Open question (3) is closed by the 2026-09-24 entry, and the research tool versions (cargo-deny 0.19.8) are not floors.
**Why:** a Decisions Log is keyed by time — history, not current truth; its in-force items now stand in the body
**Ref:** .andromeda/runs/2026-10-04T01-45-46-wrap/

## 2026-10-04T01-45-46-wrap — citations of the retired Security Decisions Log re-pointed to the body
**Section:** §Bootstrap phases `secret-scanning-ci-gate` · §Secret Management
**Change:** both body citations "(Security Decisions Log, 2026-09-24)" now name §Secret Management → Secret-scan gate shape, the paragraph U35 lifted from that log entry: the in-repo Rust static gate, its presence-guarded CI invocation, its negative arms, its accepted trade-off and the rejected job-time-fetched scanner. A blank line now separates that paragraph from "Secret scanning in CI".
**Why:** U35 moved the log verbatim to the archive, which no loop skill reads, so a citation into it points at cold history rather than at the current rule (the founder-delegated overseer directed the re-point).
**Ref:** .andromeda/runs/2026-10-04T01-45-46-wrap/

## 2026-10-04-real-model-test-surface-corrective — the secret-scan gate skips only where no `.git` exists
**Section:** §Secret Management → Secret-scan gate shape
**Change:** the gate now skips with one path-free stderr line and passes where the workspace root holds no `.git` entry at all (a VCS-less copy, such as cargo-mutants' default copy). The check is `Path::exists` before any spawn, so a `.git` FILE counts as a repository and no spawn, argv element or env var is added. A `git ls-files` failure inside a repository still fails the gate; CI always runs over a checkout, behind the unchanged presence guard.
**Why:** the gate panicked in a VCS-less copy and left `conductor-core` unmeasurable by mutation (the Epoch 5 code audit). The overseer ruled that the skip fires only with no VCS at all and that a test pins both arms.
**Ref:** .andromeda/runs/2026-10-04T12-32-24-wrap/

## 2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09 — the capture's host-path mask gains two temp roots; the fourth series joins the row and the exception inventory
**Section:** §Input Validation → Real-model capture ingest · §Security Anti-Patterns → Data Protection (the `corpus.db` ban's ratified exception)
**Change:**
- The capture row now states `mask_host_paths`' named roots: `/home/`, `/Users/`, `%APPDATA%` and, since 2026-10-06, `/tmp/` and `/var/tmp/`, all with the same match semantics. Was: the stage named with no root set. No committed capture holds a temp-rooted path, so no digest pin moved.
- The row's workspace-key provenance was "at `fcc31b2`, unchanged at `a2addb3`"; it adds "and at `5f77859`". Its two series-dated measurements ("the 2026-09-30 and 2026-10-01 series") now include the 2026-10-06 series: leaf 0 times across its three captures, and on that series — the first on the Linux dev host, on a home-rooted data dir — each `## Previously Seen` suffix is a POSIX path printed `<redacted>`, taken whole by `redact_value` after the key mask.
- The exception's per-series inventory gains "the 2026-10-06 series' d1, d2 and d3 captures carry one report body each". The exception's scope and terms are unchanged.
**Why:** `redact_value` names neither temp root, so before the extension a temp-rooted workspace path kept its parent through the whole chain (two harvest arms, each red before the extension). The series' data dir was placed home-rooted for that reason and the mask was extended after the drives, on the overseer's answer at the plan's fork round (founder-delegated). Standing rule for a later leg: a temp-rooted data dir is now masked, and the mask's root set lives in this row.
**Kept:** `redact_value`'s own token set (the path-handle row and obs-plan's redaction rows) is untouched — a different function.
**Ref:** .andromeda/runs/2026-10-06T21-10-08-wrap/

## 2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09 — the capture row and the exception's inventory gain the 2026-10-07 series
**Section:** §Input Validation, the real-model capture ingest row · §Security Anti-Patterns → Data Protection, the capture exception's per-series inventory
**Change:**
- The workspace-key derivation's provenance was "at `fcc31b2`, unchanged at `a2addb3` and at `5f77859`"; it now ends "at `5f77859` and at `f70be92`".
- The series whose launch fell back to the data dir were "the 2026-09-30, 2026-10-01 and 2026-10-06 series"; the 2026-10-07 series joins them.
- The leaf-rendering measurement names the same four series, whose leaves occur 0 times across their three captures each; the home-rooted clause (each `## Previously Seen` suffix a POSIX path printing `<redacted>`) now names the 2026-10-06 series, the first on the Linux dev host, and the 2026-10-07 series.
- The exception's inventory ended at the 2026-10-06 series; it now adds that the 2026-10-07 series' d1, d2 and d3 captures carry one report body each.
**Why:** the 2026-10-07 series committed three more captures of the ratified class, through the unchanged chain: the workspace key (`rm-fifth-series`) occurs 0 times in them, the host-path probe over the chunk's evidence reads 0, and the key rendering reads `verbatim` on all three drives. Pulse's derivation file is unchanged at `f70be92`. The exception's scope and terms did not move, and no boundary widened.
**Kept:** no handle inventory moved. Architecture registered two Pulse model handles at this wrap, and Conductor reads neither, so §Input Validation gains no row for them.
**Ref:** .andromeda/runs/2026-10-07T09-46-39-wrap/

## 2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive — the capture run joins the ingest row's dated records and the exception's inventory
**Section:** §Input Validation, the real-model capture ingest row · §Security Anti-Patterns → Data Protection, the capture exception's inventory
**Change:**
- The Boundary cell said "one `evidence/rm-capture-{drive}.txt` per series drive since 2026-09-29"; now "per pre-registered drive since 2026-09-29 (a series' drive, or the 2026-10-07 capture run's)".
- The launches whose detection fell back to the data dir were the 2026-09-30, 2026-10-01, 2026-10-06 and 2026-10-07 series'; the 2026-10-07 capture run's joins them, on a stated basis: the 2026-10-07 series' `pulse-app` by digest and its launch shape, and a boot line that read the data dir's leaf as the workspace basename.
- The leaf-rendering measurement names the capture run beside the four series: its leaf occurs 0 times across its three captures, and each `## Previously Seen` suffix prints `<redacted>`.
- The exception's inventory adds that the capture run's d1, d2 and d3 captures carry one report body each.
- One dated clause follows the inventory: on that run an operator-owned recording of the model's argv ran beside the leg, outside this repository; no Conductor artifact holds or reads it, the attempt ledger names the operator's three files by sha256 and byte count and copies none, and no fact derived from a recorded prompt entered a committed file.
**Why:** the run committed three more captures of the ratified class through the unchanged chain (the workspace key `rm-recorded-run` 0 times, the host-path probe 0, rendering `verbatim` on all three), and it is the first leg driven while a recording of the model's input existed on the host. The founder chose the operator's vehicle so that neither the capture exception nor the two-handle entry would move; the clause records that nothing crossed.
**Kept:** the exception's scope and terms, the scrub chain, the `capture_paths` guards and every handle row are unchanged. §Dependency Security is unchanged: that `cargo audit` reads green over an advisory copy holding fetch residue is recorded in the chunk's report, not in a body.
**Ref:** .andromeda/runs/2026-10-07T13-18-48-wrap/
