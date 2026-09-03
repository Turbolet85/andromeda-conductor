# Security Plan — Amendments

_Append-only changelog of amendments to `security-plan.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-15-config-validation-surface — garde pinned 0.23.0 → 0.22.1
**Section:** §Input Validation (scenario-config boundary row) · §Bootstrap phases (input-validation-library-install)
**Change:** garde version 0.23.0 → 0.22.1 in both spots. The validation contract is otherwise unchanged (garde `#[derive(Validate)]` + `#[garde(custom)]` at load; the `CONDUCTOR_*` path handles canonicalize-and-bounds-check at the edge, OUTSIDE garde).
**Why:** mirrors the arch §Stack downgrade — `garde_derive 0.23.0` is absent from the registry (latest 0.22.1), so garde 0.23.0 + the `derive` feature is unbuildable; user authorized 0.22.1 this session. §Dependency Security carried no garde version pin, so it was untouched.

## 2026-06-15-dependency-audit-gate — audit-tool versions are minimum floors; toolchain bump confirmed done
**Section:** §Dependency Security (Audit tool + Pinning) · §Bootstrap phases (dep-audit-tooling-install)
**Change:** the cargo-audit/cargo-deny version pins (0.22.2 / 0.19.8) reframed as minimum **floors** (installed 0.22.1 / 0.19.4), noting they are external CLI tools (not `Cargo.lock`-pinnable) and the RustSec advisory DB is fetched fresh each run — so any tool ≥ floor running green satisfies the gate. The toolchain "currently MSRV 1.88.0 — bump required" updated to "**done** (channel 1.95.0, `rust-version = 1.94.1`)".
**Why:** the audit gate landed **green** with cargo-audit 0.22.1 / cargo-deny 0.19.4 — a hair behind the researched 0.22.2 / 0.19.8. Dev CLI tools can't be locked in-repo and the advisory DB is runtime-fetched, so the body now states floors + records actuals rather than over-precise pins (user-approved at the wrap escalation; a `playbook.md` rule was added so this no longer escalates). Toolchain floor confirmed satisfied by this chunk's green audit. NOTE: `tauri` ≥2.10.3 is left as a forward "required bump" in §Update policy / §Anti-Patterns — tauri is declared in `[workspace.dependencies]` but not yet referenced/locked (dormant until the Tauri GUI, Epoch 9), so its audit remit + any further body reconciliation belong to that chunk.

## 2026-06-15-structured-logging-stack — obs identity env-handles noted as non-path (no validation)
**Section:** §Input Validation
**Change:** added a note that `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` (obs-plan §3) are non-path string labels stamped into self-obs JSON log *values* (JSON-escaped; no path/SQL/argv exposure) requiring **no** validation — distinct from the `CONDUCTOR_*` *path* handles that canonicalize + bounds-check.
**Why:** D-security-input (escalate) fired on the two new env-var reads; resolved WITH the user (2026-06-15 wrap) — they are benign labels, not validation boundaries, so a clarifying note (not table rows) closes the gap. D-security-subprocess + D-security-deps cleared (sidecar untouched; `tracing`/`tracing-subscriber` audit + deny green).

## 2026-06-15-design-token-typography-bundle — npm (frontend) supply-chain gate added
**Section:** §Dependency Security (new Frontend (npm) supply chain paragraph)
**Change:** added the npm/frontend supply-chain control — `npm audit` clean (0 vulns) + committed `package-lock.json` + vendored fonts (no runtime CDN) for the `crates/conductor-tauri/ui/` tree, which cargo-audit/cargo-deny do not cover; npm advisories drive the same floor discipline as cargo (no rigid dependency allowlist at Minimal tier).
**Why:** D-security-deps (escalate) fired on the chunk's new npm dependency ecosystem (React 19 / Vite 8 / Tailwind 4.1 / Fontsource / TS); §Dependency Security was cargo-only. Resolved WITH the user (2026-06-15 wrap, all-4 recordings): the npm-audit-clean gate + committed lockfile is the control. Cascaded to `.claude/rules/security.md` + `.claude/docs/security-summary.md`.

## 2026-06-21-runs-db-index — bundled SQLite version corrected to 3.50.4
**Section:** §Infrastructure (Database)
**Change:** `bundled` SQLite `3.51.1 → 3.50.4` (noting `via libsqlite3-sys 0.36.0`); rusqlite 0.38.0 unchanged.
**Why:** mirrors the arch §Stack correction — the runs-db-index chunk's first real `rusqlite 0.38.0 bundled` compile resolves `libsqlite3-sys 0.36.0` → SQLite 3.50.4 (verified from the bundled `sqlite3.h` + `Cargo.lock`); the stated 3.51.1 was assumed. D-security-deps (escalate) cleared: the dep is arch-locked (§Established Decisions [Database]) and audit-green (cargo-audit exit 0 / cargo-deny advisories+bans ok); Cargo.lock committed this chunk. Cascade no-op for the distillations (security-summary.md / rules/security.md carry no bundled-SQLite version — grep-confirmed).

## 2026-06-23-line-oriented-output-rendering — accepted deny.toml exceptions recorded (number_prefix advisory + Zlib license)
**Section:** §Dependency Security
**Change:** added an "Accepted exceptions (deny.toml)" note recording the two justified deny.toml entries — ignore RUSTSEC-2025-0119 (the unmaintained-but-non-vulnerable `number_prefix`, via `indicatif`; no upstream fix) and allow the `Zlib` license (`foldhash` via `rusqlite`→`hashbrown`; OSI+FSF permissive) — the documented mechanism, audit/deny green, no unresolved CVE.
**Why:** D-security-deps (escalate) fired on the chunk's three new presentation deps (owo-colors/indicatif/comfy-table) + the supply-chain policy change. Resolved WITH the user (2026-06-23 wrap, "bless + document in security-plan"): the deps are audit-green + deny-green; the deny.toml gained the advisory-ignore (this chunk's indicatif transitive) + the license-allow. NOTE: the `foldhash` Zlib failure was PRE-EXISTING on HEAD's committed lock (confirmed `git show HEAD:Cargo.lock`) — `cargo deny check licenses` was already red before this chunk (the prior session's recorded gates listed `cargo audit` but not `cargo deny`); fixed here because the gate cannot be green otherwise. Follow-up: a future `indicatif` 0.18 bump may drop `number_prefix` (retiring the RUSTSEC-2025-0119 ignore). Cascade no-op — security-summary.md / rules/security.md carry the high-level "cargo-audit + cargo-deny green" policy (unchanged; the accepted items live in deny.toml).

## 2026-06-24-sanitized-stderr-agent-mode-logging — CONDUCTOR_AGENT_MODE added to the non-path no-validation env-handle note
**Section:** §Input Validation (non-path env-handle paragraph)
**Change:** added `CONDUCTOR_AGENT_MODE` to the non-path-no-validation env-handle list alongside `CONDUCTOR_SERVICE_NAME`/`CONDUCTOR_ENV` — a boolean trigger whose presence (not value) is read; noted that the agent-mode log path `logs/agent-latest.jsonl` derives from `CONDUCTOR_RUNS_DIR` through the same `resolve_under` traversal guard, not from `CONDUCTOR_AGENT_MODE`.
**Why:** D-security-input (the detector itself downgraded escalate→warning) fired on the new env handle. Documentation alignment — the invariant already holds (report Coverage validation n/a; the value is unused, never path/SQL/argv-exposed; the derived log path reuses the shipped `resolve_under` guard). No new external-input boundary (the consuming-shipped-hardened-infra + reconcile-wording-where-the-invariant-holds playbook rules). D-security-deps clean (no new dep — std-only file sink; `Cargo.lock` un-drifted); D-security-subprocess clean (the sidecar spawn is untouched). Cascade no-op — security-summary.md / rules/security.md carry no per-env-handle inventory.

## 2026-06-27-desktop-a11y-harness-setup — npm-audit gate made dev-aware (`--omit=dev`) for the non-shipping a11y test tooling
**Section:** §Dependency Security (Frontend (npm) supply chain)
**Change:** the npm gate `npm audit` clean (0) → **`npm audit --omit=dev` clean (0 PRODUCTION vulns)**; dev-only transitive advisories in the non-shipping test tooling are accepted at dev-tree grain (the npm analogue of the deny.toml accepted-exceptions mechanism). Registered the a11y/GUI test devDeps (axe-core / @axe-core/webdriverio / lighthouse / colorjs.io / @crabnebula/tauri-driver / webdriverio / tsx) as the new dev tree.
**Why:** D-security-deps (escalate) fired on the new a11y harness devDeps → `npm audit` = 20 (19 mod + 1 high), ALL dev-only + transitive (`@opentelemetry/core <2.8.0` via lighthouse→@sentry/node; `serialize-javascript <=7.0.4` via @wdio/mocha-framework→mocha), none fixable non-breaking; `npm audit --omit=dev` = 0. Resolved WITH the user (2026-06-27 — implement AskUserQuestion "Dev-aware gate (wrap amends)" + wrap mechanism AskUserQuestion "`npm audit --omit=dev`"): the dev tree is non-shipping test tooling, so gate production deps strictly + accept dev-only advisories at dev-tree grain — the npm analogue of the established deny.toml justified-exceptions discipline (a category-grain accept, vs cargo's per-advisory ignore, chosen for a Minimal-tier local tool to avoid adding an audit-suppression tool). A new playbook rule was added so this no longer escalates. Cascaded to `.github/workflows/ci.yml` (Frontend gate `npm audit` → `npm audit --omit=dev`) + `.claude/rules/security.md` + `.claude/docs/security-summary.md`; the arch §Inherited Defaults / §Stack one-liners updated in lockstep (architecture-amendments). The arch devDep-registration proposal was dismissed as over-reach (a11y-plan §3.5 + stack.md already name the harness tooling).

## 2026-06-27-mcp-read-back-result-shape-adapter — read-back boundary reconciled to the hand-rolled JSON-RPC client (rmcp removed)
**Section:** §Threat Model (attack-surface entry point) · §Input Validation (MCP contract-manifest + MCP-read-back-child-stdout rows) · §Anti-Patterns (STDIO injection wording + negotiate-down) · §Dependency Security (audit tree + Update policy) · §Key decision
**Change:** every "rmcp" mention reconciled to the hand-rolled line-delimited JSON-RPC client: the read-back-child-stdout boundary is now **bounded JSON-RPC decoding** (serde_json recursion-limited against decode-bomb depth + a soft per-line size bound) parsing each response line to `serde_json::Value`, faults → `VerifyError::{JsonRpc,Decode,Transport}` via the verdict/error wall; the manifest assert reads the `initialize` result's `protocolVersion` (was `peer_info()`); the STDIO command/argument-injection class re-anchored to "MCP-sidecar STDIO" (no longer rmcp-specific); the rmcp ≥1.4.0 / GHSA-89vp-x53w-74fx stdio-client note retired; the audit tree "tonic/prost/rmcp" → "tonic/prost".
**Why:** mirrors the arch [MCP Read-Back Client] reversal (rmcp removed — Pulse's `tools/call` is non-MCP-compliant). **Every security INVARIANT is preserved** (the report demonstrates it): the hardened spawn (fixed path + `.env(...)` + metachar-reject) is unchanged (D-security-subprocess clean), the read-back decode is bounded + non-panicking through the verdict/error wall (D-security-input — the boundary is still validated, JSON not protobuf), no new dependency was added (D-security-deps clean — rmcp removed, tokio/serde_json already deps, `cargo audit` exit 0 / `cargo deny check` ok, Cargo.lock un-drifted). Routine wording→sound-impl reconcile (the 2026-06-15 rule) where the invariant holds; the decision reversal itself user-confirmed at the wrap escalation (2026-06-27). Cascaded to `.claude/rules/security.md` + `.claude/docs/security-summary.md`.

## 2026-06-27-live-pulse-e2e-proof — corpus.db is plaintext (P-049 encryption assumption corrected)
**Section:** §Threat Model (credential type + corpus.db note) · §Data Classification (compliance note) · §Encryption at rest · §Anti-Patterns (corpus.db rule)
**Change:** every "Pulse's encrypted `corpus.db` (P-049) / OS keychain / decrypt" assertion reconciled — `corpus.db` is **plaintext SQLite** (verified live 2026-06-27; the P-049 encrypted-at-rest / `OsKeychainBackend` assumption is WRONG, so the keychain-failure canary mode does not apply). The security POSTURE is preserved + sharpened: Conductor's production code accesses the corpus via MCP read-back ONLY (never directly opens / copies / exfiltrates it) and never persists its content into Conductor artifacts; `corpus.db` remains out-of-scope SUT-owned data.
**Why:** verified live (operator findings — `corpus.db` header "SQLite format 3"; read directly via Python `sqlite3` for one-off operator-sanctioned diagnosis only). Mirrors the arch §Standard Contracts corpus-access correction. No security INVARIANT weakened — the "no secrets owned / don't touch Pulse's corpus" posture holds; only the false "encrypted" framing + the inapplicable keychain-failure mode are corrected. Cascaded to `.claude/rules/security.md` (decrypt→exfiltrate + plaintext) + `.claude/docs/security-summary.md` (encrypted→plaintext).

## 2026-08-08-sut-capability-manifest — Capability-manifest input boundary
**Section:** §Input Validation · §Data Classification (scenario-config volume)
**Change:** Registered `contracts/pulse-capabilities.toml` as an external-input boundary validated at load (non-empty version/date/set, `P-NNN` shape per id, no duplicates; read faults carry `e.kind()` only, never the path; fixed path through `resolve_under`, no `CONDUCTOR_*` override). Scenario-config volume no longer names P-001..P-060.
**Why:** The chunk added a new runtime-parsed config artifact; §Input Validation enumerates every such surface and omitted it.

## 2026-08-09-interpretation-correctness-posture — advisory-DATABASE fault distinguished from tool fault
**Section:** §Dependency Security (Audit tool; Bootstrap phases dep-audit-tooling-install)
**Change:** installed `cargo-audit` recorded as 0.22.2; added the two-fault split — a TOOL fault is remedied by raising the floor, an advisory-DATABASE fault (the RustSec DB itself unparseable) is remedied by a bounded wait with the audit↔deny overlap VERIFIED green, never a floor raise, never a `deny.toml` ignore, never a CI edit.
**Why:** empirically established this chunk — `duplicate advisory ID: RUSTSEC-2026-0244` failed byte-identically on 0.22.1 and on the latest published 0.22.2, so the prescribed floor-raise was unexecutable and would have looked like compliance while changing nothing.

## 2026-08-09-sut-load-envelope — committed SUT-facing manifests registered as an input boundary
**Section:** §Input Validation (boundary table)
**Change:** Added one boundary row covering both committed SUT-facing manifests read at a fixed path — `contracts/pulse-capabilities.toml` and `contracts/pulse-load-envelope.toml` — with their per-artifact validation, the fixed `default_path()` → `resolve_under` resolution with deliberately no `CONDUCTOR_*` override, `e.kind()`-only read faults, and absent/malformed as a hard fault never defaulted.
**Why:** The chunk added the load envelope, a new runtime-parsed committed artifact, and §Input Validation enumerates every external-input surface. Registering it surfaced that the capability manifest's own row — recorded as landed in this sidecar on 2026-08-08 — was absent from the body, so the row covers both rather than leaving a sibling boundary undocumented.

## 2026-08-10-workspace-key-divergence-probe — preflight never-downgrade ban: four named preconditions
**Section:** §Security Anti-Patterns → Universal (the "NEVER silently downgrade a failed preflight" bullet)
**Change:** The bullet's cause enumeration now names the gate's FOUR named preconditions (version mismatch /
missing tool / canary fingerprint absent / app-sidecar workspace-key agreement), each required to surface as
its own distinct `blocked` precondition string — explicitly never the generic corpus-empty string. The stale
**keychain read-while-write** cause was removed from the enumeration and replaced by an explicit statement
that it does NOT apply (the P-049 encrypted-at-rest assumption was disproved live; `corpus.db` is plaintext).
**Why:** the bullet cites `(Standard Contracts: Readiness gate)` verbatim, so it is a cross-master citation of
the passage amended this chunk — cascade step 2 folds such fixes into the same pass. The keychain half had
been stale since the 2026-06-27 arch corpus-access correction, which `architecture.md` §Standard Contracts
already records; the security master had never been reconciled to it.

## 2026-08-10-pulse-run-contract — the run contract as a third committed input boundary
**Section:** §Input Validation (boundary table)
**Change:** extended the committed-SUT-facing-manifest row to `contracts/pulse-run-contract.toml`, naming its bounds (non-empty identity/provenance, non-empty and unique-id `[[term]]` list, every term carrying a statement AND its causes, non-zero `min_canary_poll_seconds`, no warm-up window with zero emissions, every `shell-declaration` term naming the env var it observes) and its controls (explicit `validate()` in `conductor-core`, fixed `default_path()` through `resolve_under`, no `CONDUCTOR_*` override, `e.kind()`-only read faults, absent/malformed a hard harness fault). Added a row for `ANDROMEDA_PULSE_L4_DETERMINISTIC` as a declaration-only read with nothing to validate — the value never becomes a path, an argv element, or a log value.
**Why:** §Input Validation is an exhaustive enumeration of external-input surfaces and the chunk added one (the playbook rule of 2026-08-10 governs this class as routine-apply). Per that rule's own trap note, the body was grepped rather than the sidecar trusted: both sibling manifests were already present in the body, so no widening beyond the new artifact was owed. Detector-raised (D-security-input).

## 2026-08-10-pulse-run-contract — never-downgrade ban restated over five preconditions
**Section:** §Security Anti-Patterns → Universal
**Change:** the bullet's enumeration names FIVE preconditions, adding unmet run-contract terms, and records that the run-contract arm composes one string naming each unmet term individually with its causes — and that a term whose truth lives on the SUT's side is recorded `declared-not-observable` and never blocks, because blocking on it would assert a measurement Conductor cannot make.
**Why:** the chunk moved the count; the bullet verbatim-enumerates the set, so it goes stale on every addition. The never-claim-a-measurement clause carries over the v2-17 precedent the preceding chunk established. Detector-raised (D-security-subprocess).
## 2026-08-11-faithful-emission-dispatcher — scenario-config boundary widened; `dive` never `skip`
**Section:** §Input Validation (scenario-config row) · §Input Validation Vector 2 trust boundary · §Security Anti-Patterns (Input)
**Change:** The scenario-config row now covers the `[phases.emission]` surface — `kind` REQUIRED whenever the table exists (no inferred default), `occurrences` bounded by `MAX_OCCURRENCES` (`0` = a declared silence window), and the per-shape cross-field rules (percentile ordering, severity ∈ 1..=24, the breathing amplitude < center sign guard, ≥2 distinct topology services, non-empty variant/category sets) — and states the error fraction in its shipped integer-percent encoding. All three sections now mandate that **every nested spec field must `dive`, never `skip`**: a skipped nested struct deserializes entirely unvalidated because garde never descends into it, so its rules never run.
**Why:** The chunk added the `[phases.emission]` external-input surface across 23 committed scenario TOMLs and found `PhaseSpec.emission` marked `#[garde(skip)]` — the documented trust boundary was not the boundary that existed. §Input Validation is an exhaustive enumeration of external-input surfaces, so the new surface earns its description (the 2026-08-10 committed-artifact precedent).

## 2026-08-13-first-live-green-preflight — MCP_ENABLED locus + shell-side contract readers
**Section:** Threat Model Summary §Auth model (Reason) · §Input Validation (committed SUT-facing manifests row)
**Change:** (1) `ANDROMEDA_PULSE_MCP_ENABLED` must be present in Conductor's OWN process environment, where the operator exports it and the sidecar receives it by inheritance — `spawn.rs` passes only the data dir via `.env(...)`, so inheritance is the sole channel; absent it the sidecar starts and immediately exits and every read-back measures the unreachable path. Conductor still never sets it itself. (2) The manifests row records that `pulse-run-contract.toml` now has readers outside `conductor-core` — the two harness scripts parse `warmup_ms` + `min_canary_poll_seconds` under the same never-defaulted rule (missing term ⇒ hard exit 2; a lowered env value clamps up to the contract floor).
**Why:** Verified live 2026-08-13 — the first arm ran without the flag, measured the read-back-unreachable path and was discarded. The four-item operator recipe omitted it. The shell-side reader is a genuinely new external-input boundary this chunk added.

## 2026-08-16-canary-fingerprint-derivation-aligned — third preflight precondition renamed
**Section:** §Security Anti-Patterns → Universal (never-silently-downgrade ban)
**Change:** In the FIVE-precondition enumeration, "canary fingerprint absent" becomes "no incident opened
after the canary storm was emitted"; count and never-downgrade rule unchanged.
**Why:** the fingerprint precondition could never pass at any width (`fingerprint_refs` carries no
Pulse-computed fingerprint), so the ban named a precondition that no longer exists; the distinct-blocked-state
requirement it enforces is unchanged and now attaches to the freshness arm.

## 2026-08-16-canary-fingerprint-derivation-aligned — child-stdout row states the freshness assertion
**Section:** §Input Validation (MCP read-back child stdout row)
**Change:** The canary clause becomes: asserts an incident opened after the emission stamp
(`extract::opened_at_unix_nanos` vs `CanaryMarker.emitted_at_unix_nano`); an absent or unparseable stamp
contributes nothing and reads as NOT-fresh ⇒ `blocked`, never a false pass.
**Why:** the row asserted an identity claim the gate no longer makes. The degrade DIRECTION is the security-
relevant half and is now explicit: the other readers degrade to empty on shape, but a missing stamp must never
read as satisfied.

## 2026-08-16-canary-fingerprint-derivation-aligned — third accepted deny.toml exception recorded
**Section:** §Dependency Security (Accepted exceptions)
**Change:** Adds the `BSD-2-Clause` allow for `arrayref` (via `conductor-emit`→`blake3`) beside the
`number_prefix` ignore and the `Zlib` allow, and states that `deny.toml` itself is the authority on the full
set while reconciling the two is carried by the `Dependency polish` route entry.
**Why:** the paragraph read as an exhaustive list and did not authorize the license this chunk's dependency
needs. The authority note prevents a future reader treating the (known-incomplete) prose as the allowlist.

## 2026-08-16-canary-fingerprint-derivation-aligned — admitting a dependency under a red audit
**Section:** §Dependency Security (advisory-DATABASE fault bullet)
**Change:** States the admission condition explicitly — a dependency delta MAY land while the advisory DB
cannot be parsed, but only on a `cargo deny check advisories bans licenses sources` VERIFIED green over the
NEW `Cargo.lock`, with the deferral's basis no longer resting on "no dependency delta".
**Why:** the bounded wait was justified partly by a static tree, a precondition this chunk broke. Left
unstated, the doc would assert an audit-green admission condition its own shipped dependency does not meet,
and the next chunk would re-derive the case from scratch.

## 2026-08-16-canary-fingerprint-derivation-aligned — deprecated-crypto ban is now active
**Section:** §Security Anti-Patterns → Data Protection
**Change:** Retires the hypothetical "if any fingerprint/hash is ever added" framing — Conductor ships a
blake3 derivation (first 16 bytes / 32 hex), the FNV-1a derivation is removed, and the run-report
`fingerprints[]` column is noted as read-back-fed, never fed from that derivation.
**Why:** the ban described a future possibility that this chunk made present; anchoring it to the shipped
derivation makes it enforceable rather than aspirational.

## 2026-08-19-connection-lifecycle-live-proof — `[phases.fault]` registered at the scenario-config boundary
**Section:** Input Validation (scenario-config row) · Threat Model Summary (attack-surface config-files vector) · Security Anti-Patterns (Input ban)
**Change:** All three enumerations of the scenario-config trust boundary now carry the `[phases.fault]` block: closed `FaultKindSpec` kind enum (unknown kind = deserialize error), `dive` never `skip` on `PhaseSpec.fault`, the scenario-level `fault_phases_are_silent` (fault ⇒ occurrences 0), and the deliberate NO-port-in-config rule (private builder parameter only — no operator-steerable bind target).
**Why:** The chunk added a new operator-authored config surface that shipped fully validated (report §Coverage: garde✓); the plan's emission-only enumerations would have left a later `#[garde(skip)]` on `fault` violating no written mandate.

## 2026-08-20-latency-regression-re-proof — the sidecar spawn resolves a NAME through PATH, not a path
**Section:** §Security Anti-Patterns → Input (+ its restatement in → Code Patterns)
**Change:** Re-worded the sidecar-spawn ban from "a fixed, hard-coded program path only" to "a fixed, hard-coded program NAME resolved through the inherited `PATH` (never a config-derived or operator-supplied command string)", and named `PATH` as a spawn-resolution input — the environment selects WHICH binary the fixed name resolves to. Added the measured diagnostic: a `PATH` entry split on a drive-letter colon resolved the sidecar nowhere and the run reported `[BLOCKED]` in ~0s, indistinguishable at row level from a genuine SUT-side gate failure. Both occurrences (→ Input, → Code Patterns) updated together; the immutable Security Decisions Log entry is left as historical record.
**Why:** `crates/conductor-verify/src/spawn.rs` pins `PULSE_MCP_PROGRAM = "andromeda-pulse-mcp"` — a program NAME whose own doc comment says "resolved from `PATH`" — so the ban's wording overstated the control while its INTENT (never operator-chosen; a compile-time constant) held and is preserved. Pre-existing gap, surfaced by this chunk's live leg failure; operator-ratified at the wrap escalation (2026-08-20).

## 2026-08-21-per-check-latency-measurement — budget_ms + the load-path check arm
**Section:** Input Validation -> scenario-config row
**Change:** The what-to-validate cell gains `[[expected]].budget_ms` (garde `range(min = 1, max = MAX_BUDGET_MS)`), and the How cell records the load-path `Scenario::check_*()` arm as the sanctioned co-equal mechanism for sibling-reading cross-field rules, faulting as `CoreError::Config`.
**Why:** The chunk added a new external-input key whose cross-field rule garde 0.22.1 structurally cannot express; the row mandated `#[garde(custom)]`, so an auditor would read a validated boundary as unvalidated, or 'fix' it into an unbuildable validator.

## 2026-08-21-per-check-latency-measurement — Input ban names both arms
**Section:** Security Anti-Patterns -> Input
**Change:** The scenario-config ban now names both arms (garde `range`/`dive` AND the load-path `check_*()` checks) and adds the carve-out that a SIBLING-reading invariant must NOT be written as `#[garde(custom)]` — it would look enforced while never testing the invariant.
**Why:** Duplicate occurrence of the retired claim; left unamended, the shipped `check_budgets` boundary would read as a ban violation at the next audit.

## 2026-08-21-per-check-latency-measurement — Bootstrap phase requires the load-path arm
**Section:** Bootstrap phases -> input-validation-library-install
**Change:** Confirmation extended to require every sibling-reading cross-field rule ship as a `Scenario::check_*()` from `from_toml_str`, with a test pinning that the load path invokes it.
**Why:** Third restatement; as written the phase mandated a check that cannot be satisfied for sibling-spanning rules.

## 2026-08-21-per-check-latency-measurement — Threat-model trust boundary records the load-path arm
**Section:** Threat Model Summary -> Attack surface (config files)
**Change:** Trust boundary records the load-path `check_*()` arm and the budget rule beside the `#[garde(custom)]` examples.
**Why:** Fourth occurrence of the same retired claim. NOTE: the detector proposed applying this in lockstep with a verbatim mirror `threat-assessment.md`; that file does NOT exist in this repo, so the mirror clause was dropped as unsubstantiated (validate check 4 — absence needs evidence).

## 2026-08-22-operator-pause-and-checklist-live-firing — `[[checklist]]` registered at the scenario-config boundary
**Section:** Input Validation → scenario-config row (both the what-to-validate and the how columns) · Threat Model Summary → Attack surface, config-files trust boundary
**Change:** Added the `[[checklist]]` block to the enumerated scenario-config boundary — `Scenario.checklist: Vec<ChecklistItem>`, `#[serde(default)]`, `dive` never `skip`, both text halves bounded by `MAX_CHECKLIST_TEXT` (200) — and extended the sibling-spanning load-path enumeration from the pair (`check_capabilities`; `check_budgets`) to three by naming `check_checklist`, at both the §Input Validation site and its verbatim Threat-Model restatement.
**Why:** §Input Validation is an exhaustive enumeration of external-input surfaces; the chunk added one with its controls already present (garde dive + bounds + load-path rule), so only the registry lagged. The two-rule `check_*()` list was restated identically in §Threat Model, so a single-site apply would have left it stale.

## 2026-08-31-p-075-assert-round — Corpus access is read AND write through the MCP tool surface
**Section:** Threat Model Summary (corpus.db note) · Input Validation (MCP child stdout row) · Data Protection (at-rest paragraph) · Security Anti-Patterns (corpus.db NEVER-ban)
**Change:** Retired "Conductor only reads the corpus via MCP read-back" at all four sites, replaced by "through the MCP TOOL SURFACE only — read-back plus the `mark_incident_resolved` lifecycle write — never the file". The Input-Validation row is renamed to `MCP child stdout` and now names the write direction explicitly (the applied `{resolved, incident_id}` response and the DECLINED `ToolDispatchFailed` arm both arrive through the same `call_tool` path under the same bounded-decode / typed-error controls). The anti-pattern ban is UNCHANGED in force and gains a clarifying clause: the ban is on FILE access, and staging a corpus row directly to induce `DeclinedStale` is the banned route — which is why that arm is stub-proven.
**Why:** The chunk landed the first production caller of a corpus-MUTATING MCP tool (`conductor_run::probe_resolve_lifecycle`) and exercised it live. Every control the plan mandates is present (bounded decode, `VerifyError::JsonRpc` whose Display renders the code only, no direct file access, nothing persisted or exfiltrated), so this is boundary-INVENTORY drift, not an unvalidated boundary. Operator-ratified at this wrap after escalation (D-security-input, severity escalate, no matching playbook pattern).

## 2026-09-01-webview-self-verify-windows-host — a new harness boundary, and two bans scoped to shipped binaries
**Section:** Input Validation (boundary table) · Security Anti-Patterns (Input canonicalize ban · Code Patterns spawn ban · Universal inbound-listener ban) · Threat Model Summary (Attack surface: CLI input vector · trust boundary) · Secret Management (Development) · Bootstrap phases (input-validation-library-install)
**Change:** Registered `CONDUCTOR_MSEDGEDRIVER` as a dev-harness-only path boundary validated at the wdio edge (existence + `isFile` + shell-metacharacter rejection, then a separate argv element of an array-form spawn; unset-or-not-a-file ⇒ skip at exit 0), explicitly NOT under the Rust-side `canonicalize` rule since no Rust code reads it — named as such in the boundary table, the canonicalize ban, the bootstrap phase, the CLI-input vector and the non-secret handle inventory. The **spawn ban** was split into its MCP-sidecar rule (unchanged) plus a harness-spawn rule (program must be a repo-derived resolved constant; an operator value may appear only as an array-form argv element after metacharacter rejection, never as the program and never in a shell). The **inbound-listener ban** and the trust-boundary sole-port claim were scoped to Conductor's SHIPPED binaries, with the dev-only `4444`/`4445` driver ports named as outside them.
**Why:** Unlike the 2026-06-23 dismissals under playbook rule @58, this chunk genuinely ADDS an external-input boundary and a spawn, so that rule's precondition was unmet and the escalate-severity detectors fired correctly. Both ban qualifications were escalated and operator-ratified at this wrap: left unqualified, the plan would forbid a leg the harness now ships, so a reader checking a green chunk would read a violation that is not one. The controls themselves are unchanged in force.

## 2026-09-01-desktop-a11y-sweep — dev driver-leg surfaces enumerate both a11y arms
**Section:** §Threat Model Summary (attack surface: port bind, CLI-input entry point) · §Input Validation (`CONDUCTOR_MSEDGEDRIVER` row) · §Security Anti-Patterns (harness-spawn rule (b), inbound-listener ban carve-out)
**Change:** five sites re-scoped from the `--e2e` leg alone to the dev-only driver stack fired by EITHER arm (`agent-run … --e2e` routine, `npm run a11y:driven` driven): the harness-lifetime `4444`/`4445` listeners, the ban carve-out covering them, the `CONDUCTOR_MSEDGEDRIVER` reader path, its unset/not-a-file skip-at-exit-0 consequence, and the harness-spawn rule's scope label.
**Why:** the chunk put a second suite on the same `wdio.conf.ts`, so the same guard, the same handle and the same listeners are now reached by a second path. No control changed — the `UNSAFE_PATH` guard, the repo-derived resolved program and the array-form spawn are untouched; only the enumerations were stale, and a ban whose scope label names one leg reads as not covering the other.

## 2026-09-01-live-per-p-id-verdict-lamps — Tauri IPC arguments get a boundary row
**Section:** Input Validation (boundary table)
**Change:** Added a row for `#[tauri::command]` IPC arguments -- the `run_id`-class string carried by `run_report` / `run_envelope` -- mandating a `resolve_under` traversal guard against the resolved runs dir before any read (absolute or `..` REJECTED, never clamped) plus `sanitize_error` on every returned `Err`; garde is n/a (not scenario config). The row records that app-defined commands are NOT capability-ACL-gated, so this row -- not the capabilities file -- is what governs them.
**Why:** The shipped `run_envelope` doc comment cites this section for its guard, but the table had no Tauri-IPC row at all -- a dangling citation, and no rule for the next command author to follow. The boundary class is pre-existing (`run_report` shipped the same shape in Epoch 9), so this documents already-correct practice rather than adding a mandate. Escalated at the wrap and resolved with the operator.

## 2026-09-02-screen-reader-manual-spec — `CONDUCTOR_NVDA`, the speech-log ingest boundary, the harness-spawn forms, the sidecar window-suppression duty
**Section:** Threat Model Summary (CLI-input vector · port-bind vector) + Input Validation (boundary table: the `CONDUCTOR_MSEDGEDRIVER` row, new `CONDUCTOR_NVDA` row, new speech-log ingest row) + Bootstrap phases (`input-validation-library-install`) + Secret Management (Development) + Security Anti-Patterns (Input canonicalize ban · Logging host-path ban · Code Patterns spawn rule (a)+(b) · Universal listener carve-out)
**Change:** (1) `CONDUCTOR_NVDA` registered as the second dev-harness host-tool path handle (existence + `isFile` + metacharacter guard at the wdio edge, array-form detached spawn with a fixed argv, skip at exit 0 when unset, value never committed) and every "one handle" singular widened to the SET. (2) A new boundary row for the screen-reader speech-log ingest -- untrusted third-party text, `heard` bounded at 400, closed enums, host-path scrub -> `<host-path>` + `security_finding`. (3) Rule (b) now governs three spawn forms (the repo-resolved driver constant; a fixed OS program + fixed argv for the PowerShell activation script; the guarded host-tool path as the program for NVDA). (4) Rule (a) gains the console-window-suppression duty -- the flagless sidecar spawn publishes its absolute exe path as a foreground pane title -- recorded as a measured, route-owned defect, not a shipped control. (5) The driver-leg enumerations (CLI-input vector, port-bind vector, Universal carve-out) name the three-family set; the Logging ban covers committed evidence records.
**Why:** (1)(2)(3) are the chunk's new external-input surfaces and spawn forms, named on the plan's P5-approved expected-amendments list and the operator's relay (the NVDA CLI form); a real host path arrived through the speech log (row S1-01). (4) measured 2026-09-02 -- NVDA spoke the pane's title; escalate-class, resolved on the operator's WRAP directive item 1. (5) the report's count move restated at three sites.

## 2026-09-02-cross-surface-envelope-parity — the third `CONDUCTOR_*` handle class, the fourth harness spawn form, and the protocol-version mechanism
**Section:** Input Validation · Threat Model Summary (CLI-input vector) · Secret Management (Development) · Bootstrap phases (input-validation-library-install) · Security Anti-Patterns (Input; Code Patterns)
**Change:** (1) New §Input Validation row for `CONDUCTOR_E2E_SEED_DIR` stating the THIRD control model —
a harness-owned repo-relative CONSTANT carried to the seeding child through the spawn's env map (never argv,
never a shell), no `canonicalize` rule and no `isFile`/metacharacter guard because the value is not
operator-supplied, no-op when unset — with the residual recorded explicitly: the test applies no
`resolve_under` guard of its own, so the repo-relative discipline lives at the wdio caller and holds only
while no shipped binary reads the handle. (2)-(5) Every singular "the two handles" clause widened to the SET
of three, with the reason SPLIT where it differs (`MSEDGEDRIVER`/`NVDA` never reach Rust; `E2E_SEED_DIR`
reaches Rust only in a test binary): the canonicalize ban's exemption, the CLI-input vector enumeration, the
non-secret handle list, and the bootstrap phase's out-of-scope set. (6) The harness-spawn enumeration went
from THREE governed forms to FOUR, adding the fixture-seed spawn (fixed `cargo` program, fixed argv, handle
in the env map, throws on non-zero exit). (7) The protocol-version ban keeps its subject and retires its
mechanism clause: the hand-rolled client READS `protocolVersion` from the `initialize` result and HOLDS it
against the pinned `2024-11-05` — it does not negotiate down from a client default.
**Why:** (1)-(6) the chunk's new external-input boundary, escalated by D-security-input and
operator-ratified 2026-09-02 as a third class; the widenings are the flow's no-single-site-apply rule, since
each clause asserted an exhaustive set the new handle falsifies. (7) D-security-subprocess, matching the
report's MIXED classification of the sibling `test-plan.md:91` site and the same correction shipped this
chunk in `conductor-verify/src/lib.rs:7`.

## 2026-09-03-live-pulse-preconditions-probed — the declaration-only read SET, and a non-spawning resolution of the sidecar name

**Section:** §Input Validation (2 rows) · §Threat Model Summary (Auth model Reason) · §Security Anti-Patterns (Input · Code Patterns (a) · Universal)
**Change:** The `ANDROMEDA_PULSE_L4_DETERMINISTIC` row is generalised to the three-handle declaration-only READ SET (`conductor_core::OBSERVED_HANDLES`) — presence-only, no value ever becoming a path, argv element or log value. The `ANDROMEDA_PULSE_DATA_DIR` row is scoped to the SPAWN-propagation use, now that the handle has a second, non-spawn presence-only reader that neither relaxes nor satisfies that duty. Auth model records that an absent `ANDROMEDA_PULSE_MCP_ENABLED` is now observed UPSTREAM (a `[PRECONDITION]` subject and the `mcp-enabled` term) rather than only downstream as the sidecar exiting. §Anti-Patterns Input and Code Patterns (a) record the second, NON-SPAWNING resolution of `PULSE_MCP_PROGRAM` — a `split_paths` + `PATHEXT` walk constructing no `Command`, returning a boolean-grade fact, never the resolved path, therefore opening no console window. §Universal scopes the never-silently-downgrade-a-preflight ban to preflights that are INVOKED, recording `boot`'s short-circuit as the sanctioned skip.
**Why:** 2026-09-03-live-pulse-preconditions-probed. **Escalated and operator-ratified**: playbook `:58`'s dismiss precondition failed (this chunk DOES add external-input surfaces — a `PATH` directory walk and two handles with no row), and `:124` (boundary widening) is never-routine by design. Ratified as apply-all-six on the basis that the probe adds no write, no spawn and no new subprocess crossing — it records a new READ-only surface. Per `:124` no playbook rule was minted; the class keeps escalating.
