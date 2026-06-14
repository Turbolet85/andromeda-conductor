## Output Protocol

Rules you MUST follow:

1. **Output = patches + changelog only.** Do NOT reproduce the full document. Emit only the exact spans you change plus a changelog.
2. **Patch format** — one block per change:
   - `### Patch N: <short description>`
   - `**Old:**` followed by the *exact* verbatim text from the draft (enough surrounding text to locate it unambiguously, byte-for-byte).
   - `**New:**` followed by the replacement text.
3. **Changelog** — one line per patch: `[Iteration N] [substantive|cosmetic] <description>`. `substantive` = changes meaning, coverage, a control, a version, or a cross-reference. `cosmetic` = wording/formatting only.
4. **PROHIBITED:**
   - Full-document reproduction.
   - Restructuring (reordering/renaming sections, re-templating) without a stated correctness reason.
   - Labeling a cosmetic change as `substantive`.
   - Introducing content owned by other specialists: no test cases (tests' domain), no OTel span/metric/trace schemas or observability platform picks like Datadog/Sentry/Splunk (obs' domain — a cross-reference to the obs plan is fine, a pick is not), no design tokens/component patterns (design's domain), no ARIA/a11y conformance rules (a11y's domain). This plan defines WHAT to protect/log/verify; others define HOW.
   - Inventing tools, versions, or CVEs not traceable to `security-research.md` / `threat-assessment.md` / arch.md.
5. **If no issues found:** output exactly `No patches` followed by a single cosmetic changelog line stating the plan was reviewed and is sound along the dimensions checked.

## Analysis Protocol

Do NOT scan-then-patch. Work the document in this order; reasoning quality, not patch count, is the goal.

1. **Read once, silently.** Read the entire document end-to-end without noting anything. Build a mental map of which section owns which claim.

2. **Cross-reference sections against each other.** Walk this concrete checklist — a dangling cross-reference is a `substantive` bug, not cosmetic:
   - **TMS Vector N → mitigation:** each of the six live Attack-surface vectors (CLI input + `CONDUCTOR_*` env; config-file parsing; Tauri IPC; MCP child-stdout; OTLP/gRPC loopback egress; the `:4317` port-occupier bind) must map to at least one Input Validation row OR Security Anti-Patterns ban OR Dependency Security control. A vector with no concrete named control is dangling.
   - **Data classification → protection:** each TMS classification entry (config; synthetic test-telemetry/run-metadata; the four "none" rows; the out-of-scope `corpus.db` / `OsKeychainBackend` reference) must be covered by Data Protection + the Logging/Error-Handling NEVER-leak list, and the Pulse-owned items must stay scoped OUT, not pulled into Conductor's controls.
   - **Tool/version agreement across 3 sites:** every pinned tool or version must agree across (a) the section body, (b) the TMS verbatim copy block, and (c) `security-research.md`. A version pinned at 3 sites must read identically at all 3.
   - **Architecture-pinned vs upgrade-recommended:** where Dependency Security or an Anti-Pattern declares a required bump (toolchain ≥ 1.94.1, `tauri` ≥ 2.10.3), that same bump must appear consistently in Dependency Security, the Universal anti-patterns, and the Security Decisions Log. `/implement` reads section bodies; the three must reconcile.
   - **`(See ## … § …)` pointers → real targets:** every cross-reference pointer must resolve to an existing subsection. Dangling pointers are substantive.
   - **Decisions-Log open-question N → surfacing section:** each open question (Tauri plugin scope; `cargo-deny` recommended-not-required; deferred secret scanner) must be referenced from the section that raises it, not left dangling.
   - **NEVER-leak list ↔ run-report fields:** the identity field set the Error-Handling artifact-sanitization paragraph enumerates (`run_id`/`seed`/`scenario`/`p_ids`/`slo_tier`/`latency_ms`/`fingerprints`) must match the set the Logging anti-pattern protects, and both must forbid the same absolute-path / struct-name leakage.
   - **Contract tool-name set consistency:** the required-tool name set named in any one location (Input Validation manifest row, read-back canary row, Error Handling) must not contradict the set named elsewhere.

3. **Check each dimension** below with its anchor example held in mind — the anchor shows the EXACT shape of issue to hunt for in that dimension.

4. **Out-of-scope discipline.** If a candidate finding would require writing test cases (tests' domain), OTel span/metric/trace schemas (obs' domain), `aria-` attributes or WCAG conformance claims (a11y's domain), design tokens/component patterns/typography (design's domain), or naming a concrete observability / error-reporting platform pick (Datadog/Sentry/Splunk) — do NOT patch the content in. Instead verify the security plan exposes the *boundary requirement* (the "what must hold," e.g. "WHAT must be scrubbed," "WHAT scope tests must cover"). Patch only if that boundary itself is unstated. This plan defines WHAT to protect/log/verify; other specialists define HOW.

5. **Prioritize by impact, then budget.** Patches come from **bucket 1 (downstream-blocking, `[priority: high]`)** first; **bucket 2 (implementation-misleading, `[priority: high]`)** only if budget remains; **bucket 3 (signal-diluting, `[priority: medium]`)** only if the fix is one line and undeferrable. Spend the 8-patch budget on findings that would cascade into a downstream skill or `/implement` before any wording polish.

## Analysis Dimensions

### 1. Downstream Readiness (route / setup-project / obs / tests) [priority: high]

Per the D26 read chain, each consumer must derive its own plan from this security plan + arch.md alone — no out-of-band knowledge.

- Can `route` reconstruct the bootstrap ordering purely from the "Bootstrap phases" list (input-validation-library-install → dep-audit-tooling-install → secret-management-init → secret-scanning-ci-gate → error-sanitization-wire → dep-security-ci-gate), with the three Minimal-dropped phases (auth-scaffolding-baseline, logging-redaction-wire, supply-chain-signing-init) named as *dropped* so route does not re-add them?
- Can `setup-project` materialize secure defaults from plan content alone (the garde `range` + `#[garde(custom)]` rule, the `std::fs::canonicalize` `CONDUCTOR_*` rule, the fixed-sidecar-path + `.env(...)` subprocess rule) without inventing controls the plan never states? Are the env-var names (`CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SCENARIOS_DIR`, `CONDUCTOR_CONTRACT_MANIFEST`, `CONDUCTOR_SEED`, `ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_MCP_ENABLED`) spelled identically at every occurrence so setup-project can key off a single canonical name?
- Can `obs` derive the scrub list (the identity-only field set; never leak canonicalized `CONDUCTOR_*` / `ANDROMEDA_PULSE_DATA_DIR` absolute paths or seam-crate struct names; `blocked` ⇒ identity fields only, measurement fields JSON `null`) from Error Handling + the Logging anti-patterns alone — leaving the actual span/log emission schema to the obs plan and naming no platform pick?
- **Adversarial:** route plans phases strictly in the listed order. If `secret-scanning-ci-gate` is sequenced as a normal phase but the plan never names a scanner and only says selection is "deferred," does route emit a phase with no installable tool — stalling the bootstrap on a phase that cannot complete — or does the plan mark it clearly enough as optional/deferred that route can skip-with-note instead of blocking?

**Anchor example:** Bootstrap phases, secret-scanning-ci-gate bullet

> "**secret-scanning-ci-gate:** (optional defense-in-depth) wire a secret-scanning CI gate so no secret is ever introduced into the `conductor-*` workspace, and add `.gitignore` entries for `*.p12` / `*.pem` / `*.cer` / `.env*` even though none are expected (Secret Management § Anti-Patterns). No specific scanner was researched in Phase 2 (secret scanning was not a researched category for this no-secrets tool) — tool selection is deferred to setup-project / operator (see Security Decisions Log)."

**Issue:** The cross-reference "(Secret Management § Anti-Patterns)" points to a subsection that may not exist under that exact name — Secret Management's bans live in the centralized `## Security Anti-Patterns § Secrets`, and Secret Management itself ends with "(See `## Security Anti-Patterns` § Secrets for secret-handling bans.)". If "§ Anti-Patterns" under Secret Management is a dangling target, route/setup-project chase a pointer that resolves nowhere. Verify the referenced subsection exists with that name before treating the pointer as sound; if it does not, the pointer is dangling (substantive).

**Why this matters:** setup-project materializes each phase's CI workflow snippet by following the plan's internal pointers. A pointer to a non-existent subsection makes setup-project either fail to locate the ban set or silently invent `.gitignore` content — defeating the plan's role as the single source of truth for the bootstrap.

---

### 2. Threat-Model Faithfulness & Vector→Control Closure [priority: high]

The Threat Model Summary is a declared VERBATIM copy from `threat-assessment.md`; divergence here cascades into every downstream read.

- Does every live Attack-surface vector have at least one concrete *named* control in Input Validation or Security Anti-Patterns — including the `:4317` port-occupier bind and the MCP child-stdout trust boundary — with none left listed-but-unmitigated?
- Are the TMS fields (tier label `Minimal (0)`, the three residual-risk classes, data classifications, auth model "none", infrastructure, compliance "None") preserved verbatim and not paraphrased, softened, or dropped versus `threat-assessment.md`?
- Does the data-classification handling keep the Pulse-owned `corpus.db` / `OsKeychainBackend("com.andromeda.pulse")` reference scoped strictly OUT of Conductor's controls (round-trip-only), rather than silently pulling Pulse's encryption into Conductor's responsibilities?
- **Adversarial:** the MCP child-stdout vector explicitly declares the child is "a locally-spawned, co-located process under the same OS user (not a remote/untrusted peer)." If a future reader takes that trust assertion at face value and the bounded-decode / empty-canary-⇒-`blocked` control is weakened or dropped, what stops a malformed or empty child stdout from being silently read as a valid-but-empty pass? Is the bounded-decode control independently load-bearing, or does it lean on the "trusted child" framing to excuse itself?

**Anchor example:** Threat Model Summary, Attack surface, MCP read-back vector

> "**Vector:** MCP read-back child-process stdout — Conductor spawns the Pulse MCP server (`andromeda-pulse-mcp`) via `TokioChildProcess` (stdio) and **trusts that child's stdout as the read-back source**"

**Issue:** This vector is declared a trust boundary whose read-back source is explicitly *trusted*. Its mitigation lives in the Input Validation "MCP read-back child stdout" row and the `## Security Anti-Patterns § Input` bounded-decode ban. Confirm BOTH are present and that neither has been softened to rely on the "trusted child" language as the sole control — the trust assertion describes provenance, not input safety. If the bounded-decode + empty-canary-⇒-`blocked` control is absent or downgraded, this vector is listed-but-unmitigated.

**Why this matters:** `tests` derives its fuzz/validation SCOPE per attack vector from the TMS + Anti-Patterns. A vector whose only "control" is a prose trust assertion gives tests no concrete boundary to target (bounded protobuf decode, empty-canary rejection), and the empty-canary-as-false-pass failure mode reaches `/implement` unguarded.

---

### 3. Tool & Version Anchoring (Catalog ↔ Plan ↔ Required Bumps) [priority: high]

Every named tool/version and every required bump must trace to `security-research.md` with its specific advisory id; a fabricated pick or an unjustified bump propagates straight to `/implement`.

- Does every tool/version in the plan (`cargo-audit` 0.22.2, `cargo-deny` 0.19.8, `cargo-auditable` 0.7.4, `cargo-geiger` 0.13.0, garde 0.23.0, serde 1.0.x, rusqlite 0.38.0, thiserror 2.0.18, rmcp 1.7.0, tonic/prost, Tauri 2.10.x) match the research catalog exactly, with no version that disagrees and no tool that appears without a catalog source?
- Is each required bump tied to its specific id and rationale — toolchain ≥ 1.94.1 (CVE-2026-33056 / RUSTSEC-2026-0033, from MSRV 1.88.0); `tauri` ≥ 2.10.3 (CVE-2026-42184, from pinned 2.10.1) — and is the rmcp ≥ 1.4.0 GHSA-89vp-x53w-74fx note preserved with its NOT-APPLICABLE (stdio-client) qualifier so it does not trigger a downstream false alarm?
- For the same-category `cargo-audit` vs `cargo-deny` choice, is the trade-off stated (cargo-audit = firm minimum bar / report-only first; cargo-deny = recommended superset gating bans/sources/licenses), and is the deferred secret scanner recorded as "no maintained option researched — selection deferred" rather than naming a fabricated scanner?
- **Adversarial:** the rmcp note flags GHSA-89vp-x53w-74fx as NOT-APPLICABLE because Conductor is a stdio client. If a downstream `cargo-audit` run surfaces that exact advisory and the plan's NOT-APPLICABLE rationale has been dropped or detached from the GHSA id, does the operator have any in-plan evidence to triage it as a false alarm — or do they burn the ~1-week SLA chasing a non-applicable advisory?

**Anchor example:** Dependency Security, Update policy line

> "rmcp stays **≥ 1.4.0** (on 1.7.0 — past the Streamable-HTTP DNS-rebinding fix GHSA-89vp-x53w-74fx, which is NOT-APPLICABLE since Conductor is a stdio client, documented here to prevent a downstream false alarm)."

**Issue:** This NOT-APPLICABLE qualifier is the only thing that pre-empts a future false alarm on GHSA-89vp-x53w-74fx. Verify the same GHSA id + NOT-APPLICABLE + stdio-client rationale survives intact wherever rmcp is discussed (here, and not contradicted in the Code Patterns protocol-negotiation ban or the Decisions Log). If any site mentions the rmcp version bump without the NOT-APPLICABLE tie, the safeguard is half-present and a downstream scan can re-raise it as actionable.

**Why this matters:** `/implement` and the dep-security CI gate act on advisory hits per the stated SLA. A detached or dropped NOT-APPLICABLE note converts a documented non-issue into an apparent critical advisory, wasting the patch SLA and possibly forcing an unnecessary rmcp bump that destabilizes the pinned stdio client.

---

### 4. Tier Calibration (no over- or under-engineering for Minimal) [priority: high]

At Minimal (0) the plan must document absences without inflating into Standard/Hardened machinery — and must not under-state the residual-risk controls Minimal does require.

- Is Logging & Monitoring correctly absent as a full section (folded into Logging anti-patterns + Error Handling) and Supply-chain integrity explicitly SKIP-marked under Dependency Security, with no section crept up to Standard/Hardened depth?
- Does Data Protection state and justify the omission of At-rest / Key-management / Data-lifecycle against the all-"none" classification, rather than adding encryption/key controls Conductor owns no material for (it holds no keys; `corpus.db` encryption is Pulse-side)?
- Does Secret Management stay at "document the absence" depth, rather than inflating into a storage/rotation/Vault-KMS regime that contradicts the credential-"none" classification (rotation, access-auditing should read N/A)?
- **Adversarial:** Data Protection asserts "Encryption at rest is out of scope for Conductor" while keeping the `OsKeychainBackend("com.andromeda.pulse")` reference in view. If a Standard-tier reader (or a future re-run that re-tiers the project) sees a keychain reference inside a section titled Data Protection, do they conclude Conductor must manage that key material — re-introducing the exact key-management treatment this section deliberately omits? Is the Pulse-ownership boundary stated firmly enough to foreclose that misread?

**Anchor example:** Data Protection, Encryption-at-rest paragraph + omission note

> "_Per template: for Minimal tier the At-rest / Key-management / Data-lifecycle subsections are omitted — no sensitive data is stored and there is no key material under Conductor's control._"

**Issue:** This omission is correct ONLY if it stays an omission. Verify no later section (Secret Management "Storage" / "Rotation cadence" / "Access auditing," or the Anti-Patterns Data Protection subsection) re-introduces at-rest/key-management controls that would contradict this explicit omission. The crypto-algorithm ban ("NEVER use deprecated crypto algorithms (MD5, SHA-1, …)") is acceptable as a forward-guardrail, but a full key-lifecycle treatment anywhere would be Standard-tier creep that this note forbids.

**Why this matters:** Tier is the SINGLE SOURCE OF TRUTH for how much rigor downstream skills apply. If the plan simultaneously omits and re-introduces key management, security-pass and `/implement` get contradictory signals about whether Conductor must hold key material — producing either dead encryption scaffolding or an audit finding for a "missing" control that should not exist.

---

### 5. Non-HTTP Surface Mapping (Tauri IPC / gRPC / MCP-stdio adapter) [priority: high] [trigger: non-REST RPC/IPC surface present per Threat Model Summary § Attack surface]

This plan has no REST/web API but three RPC-shaped surfaces; each needs a surface-specific control, not a generic web-API control.

- Do all three surfaces — **Tauri IPC** (`#[tauri::command]` start/stop/picker/run-report/operator-pause + one live-counter `Channel`), the **MCP stdio session** (rmcp over `TokioChildProcess`), and **OTLP/gRPC loopback egress** (tonic → `127.0.0.1:4317`) — have a surface-specific mapping (deny-by-default Tauri capabilities scoped to exactly those commands + the one `Channel`; rmcp preflight gate with protocol pinned `2024-11-05` + required-tool set + canary round-trip ⇒ `blocked` on mismatch; loopback-only egress whose refusal surfaces as `Result::Err`, not a verdict), with CORS/CSRF/HSTS correctly marked N/A?
- Is the MCP-stdio bounded-decode + DoS control on the child's stdout present and tied to the trust boundary (locally-spawned, same-OS-user child, not a remote peer), with empty canary ⇒ `blocked` and never a false pass-as-empty?
- For Tauri IPC, are the origin-confusion / iframe re-exposure risks mapped to enforced controls (no remote-origin iframes per GHSA-57fm-592m-34r7; `tauri` ≥ 2.10.3 for CVE-2026-42184; `shell-open` avoided or explicitly allow-scoped per CVE-2025-31477) so the "IPC is in-process-only" assumption is enforced rather than merely asserted?
- **Adversarial:** the Tauri IPC vector's trust boundary is "in-process app boundary … not network-exposed." The capabilities-file ban ("NEVER ship Tauri commands without a minimal capabilities file") is what actually enforces that. If the capabilities ban were dropped or weakened to a permissive default, what is the residual exposure given CVE-2026-42184 custom-protocol origin confusion + GHSA-57fm-592m-34r7 iframe IPC bypass — i.e., does "in-process-only" silently become "any bundled-webview content can reach every command"?

**Anchor example:** Threat Model Summary, Attack surface, Tauri IPC vector

> "**Trust boundary:** in-process app boundary (backend↔bundled webview); not network-exposed. REST/GraphQL/tRPC are N/A."

**Issue:** This boundary is asserted as in-process-only. Confirm it is *enforced* by concrete controls in `## Security Anti-Patterns § Code Patterns` — specifically the deny-by-default capabilities file, the no-remote-origin-iframe ban (GHSA-57fm-592m-34r7), and the `tauri` ≥ 2.10.3 origin-confusion bump (CVE-2026-42184). If any of those three is absent, the "not network-exposed" claim is an assertion without an enforcing control, and the vector is effectively unmitigated against origin confusion.

**Why this matters:** `setup-project` derives the Tauri capabilities file and plugin scoping from this plan. If the in-process boundary is asserted but the capabilities/iframe/origin-confusion controls are not all present, setup-project ships a webview whose IPC surface is wider than the threat model assumes — and `/implement` has no plan basis to scope it down.

---

### 6. Dependency / Supply-Chain Coverage (Minimal-tier primary residual risk) [priority: high] [trigger: dependency/supply-chain audit named as the primary residual-risk class in the Minimal-tier justification]

Dependency/supply-chain audit is the explicitly-named primary residual-risk class, so Dependency Security carries Minimal's heaviest weight even though full Supply-chain integrity is correctly SKIPPED.

- Does Dependency Security name the audit regime concretely: `cargo-audit` 0.22.2 against a committed `Cargo.lock` (covering the `bundled` SQLite C version via the `libsqlite3-sys` advisory + the tonic/prost/rmcp tree), the recommended `cargo-deny` 0.19.8 superset via `deny.toml`, and a CI step wired into the existing GitHub Actions pipeline that fails the build on an advisory hit?
- Is the Supply-chain integrity subsection (signed artifacts / SBOM / base-image scanning) SKIP-marked as Standard+Hardened-only with its reason, while the `cargo-auditable` 0.7.4 note is kept clearly low-priority / not-a-gate so it is not mistaken for a required control?
- Is the Critical-CVE-response SLA stated and proportionate (~1-week patch for actionable advisories; immediate bump for the `cargo build` extraction path / tar-rs class or the Tauri IPC surface), and are the two required bumps reflected consistently across Dependency Security, the Universal anti-patterns, and the Decisions Log?
- **Adversarial:** the `cargo-auditable` note sits inside the same SKIP-marked Supply-chain block as signed artifacts / SBOM. If a reader collapses the whole italic block to "supply-chain stuff = SKIP," do they also skip the additive `cargo-auditable` binary-embedding (harmless) — or, worse, read the SKIP as also skipping `cargo-audit`/`cargo-deny`, which are NOT in the SKIP block and ARE the Minimal-tier residual-risk gate? Is the boundary between "SKIPPED supply-chain integrity" and "REQUIRED dependency audit" sharp enough to prevent the gate itself being skipped?

**Anchor example:** Dependency Security, Supply-chain integrity SKIP block

> "_Supply chain integrity subsection (signed artifacts / SBOM / base-image scanning): **SKIP** — Standard + Hardened only; Conductor is Minimal tier with no container/cloud deployment. Optional additive note only: `cargo-auditable` 0.7.4 (security-research.md) can wrap the release build as `cargo auditable build --release` to embed the dependency list into the shipped `conductor-cli` binary for later `cargo audit bin <binary>` when the binary is run beside Pulse on another dev host — low priority, purely additive, not a gate._"

**Issue:** The SKIP marker and the additive-`cargo-auditable` note share one italic block. `cargo-auditable` is correctly "not a gate," but the required `cargo-audit`/`cargo-deny` gate lives in the *non-italic* lines above (Audit tool / CI integration). Verify the SKIP scope is unambiguously bounded to signed-artifacts/SBOM/base-image + the additive note, and does not visually or grammatically extend over the required audit gate. If the boundary is fuzzy, the one control Minimal actually mandates can be read as optional.

**Why this matters:** This audit gate IS the Minimal-tier residual-risk control for the `bundled`-SQLite-from-C + OTLP/gRPC/MCP tree. If a downstream reader skips it under a too-broad SKIP umbrella, the single highest-weight control for this tier silently disappears, and unmaintained/advisory-flagged crates float into the release build untracked.

---

### 7. Anti-Pattern Relevance & Forward-Guardrail Framing [priority: medium]

Every ban must be load-bearing for THIS Rust/Tauri/stdio-MCP stack; generic or mis-filed bans dilute the signal so developers stop reading the section for the stack-relevant ones.

- Are the bans stack-concrete (canonicalize + bounds-check on `CONDUCTOR_*` handles; no operator input into `andromeda-pulse-mcp` argv/shell per CVE-2026-30623; fixed sidecar path; rusqlite bound parameters; bounded prost decode per RUSTSEC-2020-0002 / RUSTSEC-2024-0437; `shell-open` scoping CVE-2025-31477; no remote-origin iframes GHSA-57fm-592m-34r7; deny-by-default capabilities; no symlink-follow on pre-1.94.1 extraction) — with no `eval()`/web-XSS/SQL-string-injection ban framed for a language or surface Conductor lacks?
- Are the Authentication and API domains explicitly framed as forward-guardrails ("IF an auth/network surface is ever added") so they are not mistaken for live requirements, while each still carries at least one genuinely-applicable line (e.g. "NEVER add interactive login to the headless `scripts/agent-run.sh` path"; "NEVER promote the `:4317` port-occupier fault bind into a general-purpose listener")?
- Are bans centralized in `## Security Anti-Patterns` and merely cross-referenced from section bodies, with no full restatement inline, and is the Universal list genuinely cross-cutting (`cargo-audit`-green / `Cargo.lock`-no-drift / no-panic-on-read-back / no-silent-`blocked`-downgrade / `unsafe`-FFI-review / required-bump) rather than holding mis-filed stack-specific bans?
- **Adversarial:** the Authentication and API subsections lead with generic web bans (account enumeration, weak recovery questions, `X-Forwarded-For` trust) under a forward-guardrail header. If a developer skims those two subsections, sees four bans that obviously do not apply to a no-auth no-network tool, and concludes the whole `## Security Anti-Patterns` section is boilerplate — do they then skip the Input / Code Patterns / Universal subsections where the *actually load-bearing* bans (argument-injection, fixed sidecar path, bounded decode, capabilities file, required bumps) live? Is the forward-guardrail framing prominent enough, and the genuinely-applicable line in each domain early enough, to keep the reader engaged?

**Anchor example:** Security Anti-Patterns § Authentication

> "_Conductor has no live auth surface today (auth approach = none — single-user local tool). These are forward-guardrails: IF any auth/recovery surface is ever added (e.g. a remote-controllable mode), these bans apply._
>
> - NEVER reveal whether an account exists in a recovery/login flow (account enumeration) — applies the instant any identity surface is introduced."

**Issue:** The first three Authentication bans (account enumeration, weak recovery questions, rate limiting on auth endpoints) are pure forward-guardrails with zero current applicability; only the fourth (`scripts/agent-run.sh` interactive-login ban) is live for this stack. The framing header is present and correct — verify it stays, and that the one live, stack-specific ban is not buried beneath the three inert ones such that a skimming reader writes off the section. This is a signal-dilution check, not a correctness defect; patch only if the live ban is genuinely obscured or the framing header is missing.

**Why this matters:** `## Security Anti-Patterns` is the single source of truth for all bans, cross-referenced by every section. If the reader pattern-matches the section as web-boilerplate from its opening bans and disengages, the stack-critical Input/Code-Patterns/Universal bans (the ones `/implement` must honor) lose their audience — a section that is technically complete but practically unread.
