# U08 judgment-base supersession — seed rules beside their nearest project rules

Seed source: `andromeda-setup-project/references/seed-templates/playbook.md` (read 2026-09-29). Project rules: `.andromeda/playbook.md` at `67e8cb1`, cited by the line of their `- pattern:`. Every text below is copied by the script that wrote this file, not retyped. `Boundary widening` is already held at `:124` and is not listed.

## Seed: Sequencing deferral

```yaml
- pattern: Sequencing deferral — a drift-detector flags a gap (a tool not yet installed, a capability not yet hardened, a dependent not yet wired, a span not yet emitted, a test tier not yet built) whose resolution is a LATER, still-pending chunk's defined job. NOT Foundation-only: it equally covers a primitive shipping before its driver, a config landing before the runtime that traces it, and a gate deferred at /implement that the wrap light gate then runs green.
  verdict: routine
  note: builds land capabilities first and harden / depend on them in later chunks — expected ordering, NOT spec drift (the spec is right; the resolving chunk just hasn't run yet). Apply silently. Auditable caution — it is sequencing ONLY if a later chunk genuinely owns the resolution; a gap with NO resolving chunk anywhere in the route is real drift → escalate. And ownership is a ROUTE annotation (a CARRY/PREREQ pinned on the owning entry at route-resolve), never this note or prose — a note cannot verify delivery, and an owner-chunk can complete without delivering unless the obligation rides its line. A dismissal citing a route-sequenced owner whose entry is already `complete` is NOT routine — the owner came and went; escalate it.
```

### Project `:22`

```yaml
- pattern: a Foundation-epoch chunk defers a cross-cutting concern (redaction, CI gate, coverage tooling) that the obs/security/test plan already sequences as a SEPARATE downstream bootstrap phase, and the deferral introduces no actual leak/violation in the current chunk
  verdict: routine
  note: a plan that lists the concern as its own bootstrap phase (e.g. obs-plan §3 `pii-scrubbing-wire`) has already sequenced it later; deferring it is build-sequencing, not drift, when the current chunk leaks nothing. Confirmed with the user on 2026-06-15 (structured-logging-stack wrap; D-obs-redaction fired because logging landed before the redaction layer — which is the next chunk). Generalizes the test-runner sequencing rule above.
```

### Project `:16`

```yaml
- pattern: a Foundation-epoch chunk uses plain `cargo test` / `#[test]` before the "Test framework + fixtures" chunk installs cargo-nextest / rstest
  verdict: routine
  note: test-plan §2/§4 names cargo-nextest as the TARGET runner; interim `cargo test` is build-sequencing, not drift against the test strategy. Confirmed with the user on 2026-06-15 (config-validation-surface wrap); recurred from the prior chunk.
```

## Seed: Not this chunk's drift

```yaml
- pattern: Not this chunk's drift — the proposal names a symbol / dependency / section / surface that does NOT appear in the report's Changes as this chunk's work: a dependency the chunk never touched, a plan↔plan bind neither of whose sections it changed, an invariant already satisfied by shipped infrastructure it merely consumed, a pre-existing surface it only extended internally.
  verdict: routine
  note: reject — the report's Changes is the single source of what changed this wrap, so a detector firing on anything else has mis-fired (commonly by reading the doc's own history or rationale as a current gap). Verify against the report before dismissing. Caution: where the proposal is ACCURATE and the drift REAL but pre-existing across several artifacts the chunk did not touch, do NOT amend the one artifact the detector named — that leaves a worse spec-vs-impl mismatch; route the whole family to its OWNED channel instead: a `CARRY:` pin on the markerless entry that owns that surface (route-resolve, this wrap), or — when it belongs to a future version — an `.andromeda/residuals.md` append.
```

### Project `:46`

```yaml
- pattern: a drift detector whose TRIGGER keys on the report — a plan↔plan-bind detector (e.g. D-tests-obs-harness / D-a11y-obs-schema) whose precondition is unmet, OR a token-keyed cross-doc detector (D-platform-claim, and any check that greps a doc for a token the report names) — fires on a site the chunk's report shows it did NOT introduce (the chunk changed neither bound section), or on a site that merely MENTIONS the token without stating a claim of the detector's class (a descriptive mention, an entry-point pointer, an enumeration — not a verdict); the gap, where one exists at all, is pre-existing (often already a carried follow-up)
  verdict: routine
  note: dismiss — NOT this chunk's drift. The report is the single source of what changed; a plan↔plan bind detector whose precondition ("if the report changes the harness / status-read / log format / schema") is unmet has mis-fired on a pre-existing inconsistency that is out of scope this wrap and belongs to a dedicated doc-reconcile pass. Generalizes the not-added-dependency / library-symbol over-reach / deferred-span dismiss rules to plan↔plan bind detectors. Confirmed with the user on 2026-06-21 (markdown-run-report wrap; D-tests-obs-harness fired proposing a test-plan §3 two-record-shapes clarification — the chunk only renders the existing RunRecord envelope to Markdown, touching neither §3; the test-plan §3 ↔ obs-plan §3 reconcile stays a carried follow-up). **Extended 2026-09-05 (0-pending adaptation, operator-directed): widened beyond plan↔plan-bind detectors.** At the 2026-09-04T17-15-00 wrap D-platform-claim fired ×3 on design-system sites that NAME the two-shell harness without stating any platform verdict, and the escalation found NO rule matched because this pattern named plan↔plan-bind detectors only (`fanout-results.md:28-29`); the operator dismissed all three by hand. The detector's own invariant was tightened the same day (drift-base D-platform-claim: the trigger is a STATING sentence, quoted in the proposal), so the shape now has a narrower trigger AND this dismissal rule. The dismissal never covers a site that STATES the retired claim — that is the drift the detector exists for.
```

### Project `:31`

```yaml
- pattern: a drift detector proposes a dependency bump / version change for a dependency the chunk's report shows it did NOT add or modify, where the manifest already satisfies the spec's stated floor
  verdict: routine
  note: dismiss — NOT this chunk's drift. The report is the single source of what changed this chunk; a dependency it never touched (already at/above the spec floor) is out of scope this wrap. The detector likely read the spec's historical rationale (e.g. "arch originally pinned tauri 2.10.1 → bump to ≥2.10.3") as a current gap. Verify against the manifest before escalating. Confirmed with the user on 2026-06-16 (seeded-phase-scheduler wrap; D-security-deps escalate misfired on `tauri` — already pinned 2.10.3, untouched — while the chunk only added audit-green rand_chacha/rand_core).
```

## Seed: Registry over-reach

```yaml
- pattern: Registry over-reach — a detector proposes registering per-item REALIZATION into a spec registry that tracks its subject at CATEGORY grain: a public API symbol, an internal module, a CLI verb or flag, a command / handler name, an individual config file, a framework permission granted inside an already-registered capability file, a second transport instance for an already-registered surface.
  verdict: routine
  note: reject — the arch registries track occupied RESOURCES (ports / sockets / endpoints / IPC / events / env vars / crates/packages / artifacts) and contract SHAPES, not the realization inside an already-registered category. Check what the registry actually enumerates, and whether earlier chunks of the same kind registered theirs — they usually did not. A genuinely new resource of a kind the registry DOES enumerate still registers normally.
```

### Project `:37`

```yaml
- pattern: D-arch-resources proposes registering a public library API symbol (struct / enum / fn / const / method) in ANY arch registry section (§Occupied Resources OR §Standard Contracts), where the chunk's actual occupied resources (ports / sockets / endpoints / IPC methods / events / env vars / workspace crates) are already registered there
  verdict: routine
  note: dismiss — over-reach. arch §Occupied Resources tracks OCCUPIED RESOURCES (ports/sockets/routes/crates/artifacts/env vars) and §Standard Contracts tracks the readiness-gate + run-report-envelope SHAPES — neither tracks per-crate public API surface; the detector's own enumerated list does not include library symbols. When the chunk's real resources are already registered (the `:4317` socket + the workspace crates in §Occupied Resources; the run-report envelope `verdict ∈ {Pass,Fail,CalibrationRegion}` + `slo_tier`/`latency_ms` in §Standard Contracts), registering its API symbols is detail arch deliberately omits. Precedent: the 8 conductor-emit chunks added many public symbols (RateCurve / EmitError / TraceEmitter / LatencyProfile / PiiCorpus / ServiceTopology …) and registered none — D-arch-resources returned clean on those wraps. Confirmed with the user on 2026-06-19 (port-occupier-fault wrap, §Occupied Resources variant); broadened to §Standard Contracts on 2026-06-21 (expected-outcome-slo-timing-model wrap — the detector re-fired proposing ExpectedCheck/SloOutcome/CheckOutcome/evaluate_* into §Standard Contracts; same over-reach, dismissed with the user).
```

### Project `:49`

```yaml
- pattern: D-arch-resources proposes registering individual scenario config FILES under `scenarios/` (or other per-item content within an already-registered directory — INCLUDING read-back / detector-output TOKENS declared in a scenario's `[[expected]]`, e.g. `RetryStorm` / `ErrorRateSpike` / `ServiceWentSilent`) as occupied resources, where the directory + config-file category is already registered (arch §Cross-cutting [Config management] + §Infrastructure directory tree) and the chunk added no new port/socket/endpoint/IPC/event/env-var/crate
  verdict: routine
  note: dismiss — over-reach, the per-file/per-item flavor of the library-symbol rule above. arch registers `scenarios/` as a config-file CATEGORY (§Cross-cutting [Config management] "declarative scenario config files (serde + garde, no DSL)" + the §Infrastructure directory tree + CLAUDE.md §Key directories); the per-P-ID `.toml` files are CONTENT within that already-registered directory, not new occupied resources (the detector's own enumerated list is ports/sockets/endpoints/IPC/events/env-vars/crates — not config files, which arch tracks at directory grain). The category predates this chunk (Epoch-2 error-baseline-spike.toml registered none). Confirmed with the user on 2026-06-21 (connection-lifecycle-scenarios wrap; D-arch-resources fired proposing the 4 P-001..P-004 TOMLs into §Occupied Resources §On-disk artifacts). Prevents re-fire across the remaining Epoch-7 scenario chunks, each of which adds more scenario TOMLs. Broadened to read-back / detector-output TOKENS on 2026-06-22 (fingerprint-storm-scenarios wrap; D-arch-resources fired proposing the inferred `RetryStorm` token into §Occupied Resources — same over-reach: arch tracks the read-back contract at the 4 MCP tool names (§Occupied Resources) + the run-report envelope SHAPE (§Standard Contracts), NOT the open-ended detector-output token vocabulary; the 11 prior tokens — ErrorRateSpike/ServiceWentSilent/RestartEvent/LatencyRegression/ReceiverFailed/Idle/Stalled/Receiving/ERROR/WARN/exception — were never registered, and the chunk added no new port/socket/endpoint/IPC/event/env-var/crate). Pre-empts re-fire on severity-lifecycle (`Autonomous`/`Resolved`/`Curious`), constellation, and scrub/pipeline.
```

### Project `:61`

```yaml
- pattern: D-arch-resources proposes registering a new `conductor <verb>` CLI subcommand (or its `--flag`) into arch §Occupied Resources / §Standard Contracts, where the chunk added no new port/socket/endpoint/IPC/event/env-var/crate and the verb belongs in layout-templates §cli Primary screens
  verdict: routine
  note: dismiss — over-reach, the CLI-verb flavor of the library-symbol + config-file over-reach rules above. arch §Occupied Resources tracks ports/sockets/endpoints/IPC/events/env-vars/crates + §Standard Contracts the readiness-gate / run-report-envelope SHAPES; the CLI VERB SURFACE is layout-templates' concern (§cli Primary screens), not arch's — run/suite/report/preflight were never registered in arch §Occupied Resources, and preflight (ch2) + coverage (this chunk) landed as layout-templates amendments. Confirmed with the user on 2026-06-23 (line-oriented-output-rendering wrap; D-arch-resources proposed `conductor coverage` + the render module's public fns into §Occupied Resources — the verb went to layout-templates §cli, the render API is the library-symbol over-reach). Pre-empts re-fire on Epoch-8 ch4/ch5 (more verbs/flags) + Epoch-9 Tauri commands.
```

### Project `:67`

```yaml
- pattern: D-arch-resources proposes registering a Tauri framework capability PERMISSION (`core:window:allow-*` or any `core:<module>:allow-*` ACL grant in `capabilities/*.json`) into arch §Occupied Resources / §Standard Contracts, where the chunk added no new Conductor `#[tauri::command]` / `Channel` / port / socket / endpoint / env-var / crate
  verdict: routine
  note: dismiss — over-reach, the Tauri-capability-perm flavor of the CLI-verb/command + library-symbol + config-file over-reach rules above. The `core:window:*` perms are FRAMEWORK window ops the deny-by-default ACL allows (security-plan §Tauri GUI's domain — the security detector validates the ACL there), NOT Conductor's own IPC methods; arch §Occupied Resources tracks Conductor's command surface (start/stop, scenario/suite picker, run-report, operator-pause + the one `Channel`), which this chunk did not touch. The `capabilities/*.json` file is config CONTENT within the already-registered `conductor-tauri` crate (the per-file flavor of the scenario-TOML rule). Confirmed with the user on 2026-06-24 (frameless-window-shell wrap; D-arch-resources proposed `core:window:allow-start-dragging`/`-minimize`/`-close` into §Occupied Resources — the genuine new resource was the `logs/conductor-tauri.jsonl` artifact, registered; the perms dismissed). Pre-empts re-fire on the remaining Epoch-9 Tauri chunks (picker / Channel / operator-pause dialog), each adding more capability perms.
```

### Project `:73`

```yaml
- pattern: D-arch-resources / D-arch-decisions proposes registering a Conductor `#[tauri::command]` NAME (e.g. list_scenarios / start_run / stop_run), an internal MODULE within an already-registered crate (e.g. conductor-tauri::commands, conductor-core::scenario_catalog), or a frontend COMPONENT PACKAGE / component-styling decision (e.g. cmdk — shadcn Command's engine) into arch §Occupied Resources / §Stack / §Established Decisions, where the chunk added no new port / socket / endpoint / env-var / crate
  verdict: routine
  note: dismiss — over-reach, the Tauri-command-name + internal-module + frontend-component-package flavor of the established library-symbol / CLI-verb / Tauri-capability-perm / config-file over-reach family. arch §Occupied Resources registers the Tauri command surface at CATEGORY grain ("Tauri commands (internal IPC): start/stop, scenario/suite picker, run-report view, operator-pause prompt; one `Channel`") — the concrete handler fn names (list_scenarios/start_run/stop_run) are the realization arch omits, exactly as it omits per-crate public symbols + CLI verb names. Internal modules (commands / scenario_catalog) are content within already-registered crates, NOT workspace members (§Crate names lists members). The frontend component-library decision lives in design-system §Component Patterns §5 ("shadcn Command/Select (Radix-driven)") + frontend.md — arch §Stack summarizes the frontend stack (React/Vite/Tailwind) and never enumerated shadcn/Radix/Lucide; cmdk is the impl of the already-locked shadcn-Command decision (recorded in package.json + gated by npm audit), and frontend-component-styling is below the altitude of an arch §Established Decision (which locks language / runtime / DB / module-boundaries) — its realization (cmdk-direct + token `.css`) is a frontend.md curation learning, not an arch lock. Confirmed with the user on 2026-06-26 (scenario-suite-picker-start-stop wrap; D-arch-resources proposed the 3 command names + the conductor-tauri/conductor-core modules + cmdk-into-§Stack, D-arch-decisions proposed a cmdk Established Decision — all dismissed; the genuine new surfaces map to the pre-registered command category + the design-system component decision, validated boundaries [validate_selection/resolve_under], and audit-green deps). Pre-empts re-fire on Epoch-9 ch4 (Channel command), ch5 (component primitives library), ch8 (operator-pause dialog command).
```

### Project `:82`

```yaml
- pattern: D-arch-resources proposes registering an additional IPC `Channel` INSTANCE (a 2nd `tauri::ipc::Channel<T>` carrying a distinct backend→frontend stream, e.g. the operator-hold prompt) into arch §Occupied Resources / §Real-time Strategy / §Standard Contracts, where the stream serves an already-registered command/surface CATEGORY (e.g. "operator-pause prompt") and the chunk added no new port / socket / endpoint / env-var / crate
  verdict: routine
  note: dismiss — over-reach, the IPC-transport-INSTANCE flavor of the Tauri-command-name (2026-06-26) + library-symbol / CLI-verb / capability-perm / config-file over-reach family. arch §Occupied Resources registers the Tauri command/stream surface at CATEGORY grain ("...operator-pause prompt; one `Channel` for live counters") — the concrete TRANSPORT of an already-registered surface (a handler fn name, OR the `Channel` instance carrying its prompt) is the realization arch omits. The existing "one `Channel` for live counters" stays TRUE (the live-counter Channel still exists; the new Channel carries prompts, not counters — it does not contradict the statement). The operator-pause prompt was a registered surface from the start; ch8 only realized its transport as a 2nd `Channel` + the `resolve_operator_hold` command (command name dismissed per 2026-06-26; the Channel dismissed here). Confirmed with the user on 2026-06-27 (operator-pause-go-no-go-dialog wrap; D-arch-resources proposed both into §Occupied Resources). Pre-empts re-fire on any future chunk adding a `Channel` for an already-registered surface.
```

## Seed: Accurate this-chunk addition

```yaml
- pattern: Accurate this-chunk addition — the proposal registers something the CURRENT chunk genuinely introduced or changed (its named symbols / values DO appear in the report's Changes), landing inside an existing section; or it reconciles a spec's illustrative wording / mechanism to the sound implementation the chunk shipped, where the report shows the invariant still holds.
  verdict: routine
  note: apply — the apply-side dual of the two reject rules above; without it a playbook only learns to dismiss. Bringing a body to current truth IS reconcile's job. The test is that the named thing is THIS chunk's and the invariant survives — only form or mechanism may differ. A REVERSAL of a locked decision is not this rule: escalate that once to ratify it.
```

### Project `:28`

```yaml
- pattern: an amendment reconciles a spec's illustrative mechanism or wording to the sound implementation actually shipped, where the chunk report demonstrates the invariant still holds (names / values / contract preserved; only the form or mechanism differs)
  verdict: routine
  note: generalizes the redaction-reconciliation rule above to ANY spec-illustration → sound-impl alignment. When the report proves the invariant holds, correcting an over-literal or framework-naive spec illustration is documentation alignment, not drift. Confirmed with the user on 2026-06-15 (design-token-typography-bundle wrap; design-system §Tokens `@theme` → `:root` because Tailwind v4 tree-shakes non-namespace `@theme` tokens + forbids `@media` nesting). Recurred: redaction wording (log-error-boundary-redaction) → token-emission mechanism (this chunk).
```

### Project `:25`

```yaml
- pattern: a redaction/scrubbing amendment reconciles a spec's redaction WORDING to a sound host-file-path-anchored implementation (value-scrub on absolute host paths + field-name allowlist + Display-not-Debug edge), where the chunk report shows no actual host-path / struct-name leak
  verdict: routine
  note: when the redaction is implemented and the report demonstrates the invariant holds (host paths masked, no Debug struct dumps, allowlisted identity fields like `target` preserved), reconciling over-literal spec wording (e.g. "remove all `module::` tokens") to the sound model is documentation alignment, not a hygiene gap. Confirmed with the user on 2026-06-15 (log-error-boundary-redaction wrap; D-obs-redaction escalate → reconcile §6/§11). Distinct from the deferral rule above (that was redaction NOT YET built; this is redaction built + reconciling wording).
```

### Project `:97`

```yaml
- pattern: a chunk REVERSES a locked arch §Established Decision because the live SUT's actual behavior contradicts it (a tech choice that proved unworkable against the real system), user-confirmed at /andromeda-phase P4, where the report shows the decision's INVARIANTS still hold via the replacement (only the mechanism changed)
  verdict: escalate
  note: escalate once to ratify the reversal + apply the arch amendment (body → the new decision in current truth; the superseded rationale → the architecture-amendments sidecar), then the consequential same-reversal mentions across the doc (Stack/Conventions/Inherited/tree + any security/test/obs prose naming the old tech) reconcile in lockstep as routine wording→current-truth (the invariants preserved). DISTINCT from the 2026-06-15 spec-illustration→sound-impl rule (that aligns wording/mechanism WITHIN a decision; this REVERSES the decision's choice — hence escalate, not silent). The user's P4 approach-confirmation + the surfaced spec↔reality gap that drove the re-plan ARE the resolution; the wrap escalation is the conscious ratify-the-arch-record step, not a re-litigation. Confirmed with the user on 2026-06-27 (mcp-read-back-result-shape-adapter wrap; arch [MCP Read-Back Client] rmcp → hand-rolled JSON-RPC because Pulse's `tools/call` is non-MCP-compliant — rmcp's typed `call_tool` `UnexpectedResponse`'d every live call; the negotiate-down + hardened spawn + bounded decode invariants all held). Pre-empts a future live-SUT-forced locked-decision reversal from re-deriving the whole escalation from scratch.
```

## Seed: External decay

```yaml
- pattern: External decay — a gate turns red with NO in-diff cause: the lock / source / config it checks is un-drifted and the failure keys on the world moving while the project stood still (a freshly-fetched advisory DB, an expired tool or cert, a registry policy change) — typically surfacing after a pause.
  verdict: routine
  note: neither this chunk's drift (nothing in the diff caused it) nor sequencing (no future chunk owns it yet) — the world moved, the code didn't. It never blocks the chunk that DISCOVERED it, and it never resolves by silently widening an ignore list (actionable-with-fix items are not the non-actionable class an ignore legitimately absorbs). It ALWAYS produces an owner: a route entry, or a recorded bounded deferral. If the gate must pass meanwhile, an ID-scoped reasoned ignore NAMING its owning chunk is part of the pattern, not a shortcut — a permanently-red gate stops discriminating, so a NEW red becomes invisible. An operator-RATIFIED standing deferral may compact its re-pin and set a probe re-run interval — with the overlap probe NAMED and verified green every chunk, and interval skips recorded in the chunk report, never silent — or take the probe-auto-satisfy tier: the pin names its SIGNATURE (failing gate's exit + first diagnostic line + overlap result) and a byte-identical probe — read from the external artifact's current state, never only a local copy of it — satisfies it with a one-line record; any deviation restores the full form.
```

### Project `:91`

```yaml
- pattern: an external supply-chain input DECAYS under a STATIC dependency tree — a new advisory appears against an unchanged tree, OR the advisory database itself fails to parse/fetch — making a supply-chain gate RED while the chunk's own dependency delta is zero and the OVERLAPPING gate stays green
  verdict: routine
  note: record a BOUNDED DEFERRAL; do not distort the project. External decay is not the chunk's drift — the report's Dependencies bullet is the single source of what changed, and a zero-delta chunk with un-drifted lockfiles cannot have caused it. The response is bounded, and it FORKS on which of the two faults it is (established 2026-08-09, `2026-08-09-interpretation-correctness-posture`): a **TOOL fault** (the scanner is too old / carries a fixed bug) → re-check at the NEXT chunk and if it persists raise the tool's FLOOR to the fixed release (security-plan §Dependency Security states floors, not pins — the 2026-06-15 rule above already establishes that dev CLI tools can't be pinned in `Cargo.lock`); an **advisory-DATABASE fault** (the DB itself will not parse/fetch, so NO released version can read it) → the floor raise is UNEXECUTABLE and would look like compliance while changing nothing, so the remedy is the bounded WAIT alone, re-checked each chunk, with the audit↔deny overlap VERIFIED green rather than assumed. Prove which fault it is before prescribing — and prove FIRST that the data under test is the WORLD's: a measurement of an external artifact reads its CURRENT state (`git status --porcelain` empty in the local advisory-db checkout, or a fresh clone into an empty location / a CI run's clean checkout); a fetch INTO an existing copy does not clear residue, so a byte-identical local signature is evidence of a stable CACHE until that check has run — only then does the fork apply: install the latest published tool and re-run — identical failure means the data, not the tool (2026-08-09: `duplicate advisory ID: RUSTSEC-2026-0244` failed byte-identically on 0.22.1 and on the latest 0.22.2, read then as committed data in the advisory-db — **corrected 2026-09-05 (0-pending adaptation, operator-directed): it was NOT; upstream had moved that file from `crates/gettext-sys/` to `crates/gettext-rs/` on 2026-08-09, the deferral's first day, and the duplicate was an untracked leftover in the LOCAL `$CARGO_HOME/advisory-db` checkout whose HEAD matched upstream throughout — 52 consecutive identical readings were the cache's, not the world's; CI's clean checkout passed the same step and a fresh `--db` clone exits 0. The seeded playbook template carries the same current-state clause**). Explicitly NOT: a `deny.toml` ignore entry (that suppresses a real signal CLASS for a transient external fault), NOT a CI edit, NOT a silent accept. What preserves the signal meanwhile is the deliberate audit+deny OVERLAP (security.md 2026-06-23: the gate MUST run both) — one going red for an external reason while the other is green is the overlap working as designed, not a hole. DISTINCT from the 2026-06-15 tool-version-lag rule above, whose trigger is a GREEN gate on a slightly-behind tool; this rule's trigger is a RED gate from decay outside the project. Confirmed with the user on 2026-08-09 (out-of-scope-classification-treatment wrap; `cargo audit` exit 1 on "error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244" — the RustSec DB itself would not parse, reproduced identically with a fresh fetch and with `-n` against the cache; zero dependency delta, both lockfiles un-drifted, `cargo deny check` advisories/bans/licenses/sources all ok). Second occurrence of the class: 2026-08-08-dependency-advisory-remediation was the advisory-appears-against-a-static-tree flavor (lock-only bumps cleared RUSTSEC-2026-0194/-0195/-0204 with no Tauri bump). Note the CI consequence when this fires: CI's own `cargo audit` step hits the same wall until upstream heals — that is part of what the deferral tracks. The converse is the discriminator: CI GREEN while the local probe stays red means the fault is LOCAL (measured 2026-09-05 — run 33954347685 passed `cargo audit` on a clean checkout while the local cache reproduced the old signature).
```

### Project `:103`

```yaml
- pattern: a chunk lands a NEW dependency (a real `Cargo.lock` delta) while the supply-chain gate is red for an EXTERNAL advisory-DATABASE fault — the precondition of the bounded-deferral rule above ("the chunk's own dependency delta is zero") no longer holds
  verdict: routine
  note: apply — admit the dependency ONLY on a `cargo deny check advisories bans licenses sources` VERIFIED green over the NEW lockfile (verified, never assumed), since the deny overlap is then the SOLE coverage for the added packages; any new license/advisory it surfaces takes the established justified-exception mechanism (a `[licenses] allow` / `[advisories] ignore` entry naming the crate and its dependency path). When re-pinning the deferral, STATE the changed basis. The form the pin returns to keys on whether the delta ADMITS A PACKAGE, not on whether `Cargo.lock` moved: a delta that admits one changes the very set `cargo deny` scans, so "no dependency delta" stops being available as a reason and the pin returns to FULL form; a delta that is EDGE-ONLY (a dependency-list line for a package already resolved in the tree, zero `[[package]]` nodes added or removed — verified by diff, never assumed) leaves deny's coverage set identical, so the COMPACT ratified form still stands, provided the deny-green verification below was actually run over the new lock. Confirmed with the user on 2026-08-20 (`2026-08-20-read-back-seam-survivors-closed` wrap — first measured instance of the edge-only case: `assert_fs` added as a `conductor-verify` dev-dep, already a workspace dep consumed by `conductor-run`, one lock line, zero package nodes, deny true exit 0; this sharpening aligns the rule with the route PREREQ's own signature, which had already narrowed the trigger to "a dependency delta that ADMITS a package"). DISTINCT from the 2026-08-09 bounded-deferral rule above, which governs a ZERO-delta chunk (external decay is not that chunk's drift); this rule governs the case where the chunk genuinely changes the tree under a gate that cannot scan it. Explicitly NOT: blocking the chunk on an external fault (the deferral exists precisely because no floor can be raised), and NOT a `deny.toml` ignore standing in for the missing audit. Confirmed with the user on 2026-08-16 (canary-fingerprint-derivation-aligned wrap; `blake3` 1.8.6 + `arrayref`/`arrayvec`/`constant_time_eq`/`cpufeatures` landed against the 22nd consecutive `duplicate advisory ID: RUSTSEC-2026-0244`, with deny true exit 0 across all four classes after a justified BSD-2-Clause allow for `arrayref`). security-plan §Dependency Security now states the admission condition.
```

