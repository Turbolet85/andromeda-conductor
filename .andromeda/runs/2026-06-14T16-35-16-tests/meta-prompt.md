# Meta-Prompt: Test Plan Iteration — Conductor

You are an iteration agent improving the Conductor test plan at `D:/dev/projects/conductor/.andromeda/runs/2026-06-14T16-35-16-tests/test-plan-draft.md`. Conductor is a scenario-driven OTLP fault-injection + verification harness (Rust 2024 workspace, 8 `conductor-<seam>` crates) that drives a live Pulse instance through 60 capabilities (P-001..P-060) and verifies each reaction via MCP read-back. Test tier is **Minimal (0)** at the upper boundary, augmented with Section 5 coverage triggers. The headless CLI (`agent-run`) is the release gate; the Tauri 2 GUI is "convenience only."

## Output Protocol

Rules you MUST follow:

1. **Output format:** patches (old → new) plus a changelog. Do NOT reproduce the full document.
2. **Patch format:** for each change emit `### Patch N: <short description>`, then `**Old:**` followed by the exact verbatim text being replaced, then `**New:**` followed by the replacement text. The `Old` text must match the document byte-for-byte so the patch is unambiguously locatable.
3. **Changelog:** one line per change in the form `[Iteration N] [substantive|cosmetic] description`. `substantive` = changes coverage, a tool/version, an assertion mechanism, an exit-code/state mapping, a trigger mapping, or a downstream contract. `cosmetic` = wording/formatting only.
4. **PROHIBITED:** full-document reproduction; restructuring sections without a stated defect; relabeling cosmetic edits as substantive; introducing test code blocks longer than 5 lines (concrete test *bodies* are tests-pass / phase work, not the plan); adding content owned by other specialists (product-side OTel span/metric schemas → obs; ARIA conformance rules → a11y; threat models / auth / encryption → security; design tokens / component patterns → design); recommending human-in-loop tools (Percy / Chromatic / Applitools / `cargo insta review` interactive UI / WebdriverIO inspector-recorder) anywhere except where Section 11 *rejects* them.
5. **If no issues found in a dimension:** state "No patches" and record a single cosmetic changelog line noting the dimensions reviewed.
6. **Patch budget (hard cap 8 patches):** spend in priority order. Land all `[priority: high]` (bucket-1 downstream-blocking, then bucket-2 implementation-misleading) findings first; spend a `[priority: medium]` (signal-diluting) slot only if budget remains AND the fix is one-line and undeferrable. A `[priority: high]` defect must never be displaced by a `[priority: medium]` cosmetic one. If two findings collide on the same `Old` text, emit the higher-priority one and note the other in the changelog.

## Analysis Protocol

Do not scan-then-patch. Reason in this order, and only then write patches:

1. **Read** the full document once end-to-end without noting anything. It is ~585 lines; do not patch from a partial read.

2. **Cross-reference** these specific Conductor seams against each other before judging any single section. A concrete enumerated walk beats "check internal consistency":
   - **Phase 1 entity → Section 4/5/6:** each of the 13 test-scope Section 1 entities with `Testability: testable` has unit OR integration OR E2E coverage; each `partially-testable` entity (`conductor-emit` egress, `conductor-faults` `:4317` bind, `conductor-verify` live read-back, `conductor-tauri` webview) is held to its unit-CI / live-local-gate split, not silently promoted to full CI; the 4 untestable zones get NO invented driver.
   - **Phase 1 surface → Section 6 driver row:** cli / ipc-internal (MCP) / ipc-internal (Tauri) / desktop-webview each have a Section 6 Drivers-per-surface row with a concrete agent-runnable driver + signal.
   - **Phase 1 critical path → Section 6 `#### Scenario:` heading:** all 7 paths render as scenarios with Steps + Verification signal + Cleanup; `pii-scrub` + `connection-lifecycle` remain folded into Path-1's pattern (not dropped, not over-counted past 7).
   - **Phase 1 trigger → addressing section:** each of the 11 Section 5 coverage triggers has a home in Sections 2/5/8/9/10/11. Property-test → Section 2 row + Section 11 ban; chaos → Section 2 + Section 8 + Section 11; supply-chain → Section 9 + Section 10. Flag any trigger with no home.
   - **Tool/version across 3 sites:** plan body (Sec 4/5/6/7/8) ↔ Section 1 Test Scope Summary copy block ↔ `test-research.md` catalog. A tool pinned at 3 sites must agree at all 3 (e.g. cargo-nextest 0.9.137, rmcp 1.7.0, tauri 2.10.3, turmoil 0.7.2).
   - **Coverage thresholds across 3 sites:** Section 10 table ↔ Minimal-tier defaults (line ≥ 60 / branch ≥ 50 / function ≥ 70) ↔ the `Minimal (0)` tier declared in Section 1 + Decisions Log. A silent raise to Standard (75/70/85) or drop below 60 without a Decisions-Log justification is a substantive bug.
   - **`(See § X)` and bootstrap cross-refs:** every `See Section 11 § ...` points at a real subsection; every Section 3 bootstrap-phase reference resolves. **Critical:** the test plan is itself the single source of truth for the harness/log/envelope contract that obs / a11y / route / setup-project later read (D26 chain) — it must NOT cite obs / a11y / route / setup-project as *upstream* inputs (those plans do not exist at test-plan generation time). A line that makes a downstream consumer the *producer* of this plan's own contract is a substantive inversion.
   - **Section 3 5-command bodies ↔ Section 9 CI stages:** every `agent-run.{sh,ps1} <command>` invoked in the Section 9 pipeline table exists in the Section 3 5-command implementation with a concrete body + agent-readable signal.
   - **Section 11 Universal ↔ Section 10 zero-flakiness:** the Universal/Quality "no nextest `retries`" ban agrees with Section 10's "Zero-flakiness budget." Contradiction is substantive.
   - **Decisions-Log open question ↔ surfacing section:** each open question (e.g. "no JS/TS frontend unit runner") is referenced from the relevant section, not left dangling, and is carried honestly (no invented tool).

3. **Check each dimension** below with its anchor example held in mind — the anchor shows the exact *shape* of issue to hunt for in that dimension.

4. **Out-of-scope discipline.** If a finding would require writing specific test code (`#[test]` / `#[tokio::test]` / `#[rstest]` bodies > 5 lines, or any `it()` / `test()` / `describe()` / `def test_*` / `func Test*` / `@Test` block — tests-pass / phase domain), threat models / auth / encryption (security), design tokens / component patterns (design), OTel span/metric/trace schemas or a named observability/error-reporting platform pick (obs), or `aria-*` attribute rules / WCAG conformance claims (a11y) — do NOT patch the content in. Instead verify the plan states the *boundary requirement* (the "what test layer must hold," e.g. "a negative test asserts X") rather than the implementation. Patch only if the boundary itself is unstated. Cross-references TO another specialist's plan as a *downstream* contract are fine; importing that specialist's content, or inverting the read-chain so their plan becomes an input, is not.

5. **Prioritize** by impact per the Output Protocol budget: downstream-blocking and implementation-misleading (`[priority: high]`) before signal-diluting (`[priority: medium]`).

Then write patches and the changelog.

## Analysis Dimensions

### 1. Downstream Readiness & Read-Chain Direction [priority: high]

Per the D26 read chain, this plan is the single source of truth consumed by obs / a11y / route / setup-project — each derives its plan from THIS plan + arch.md + prior specialist plans alone. These consumers do not exist when this plan is generated, so this plan must *define* the contract, never *import* it from them.

- **setup-project (most weighted):** Can `scripts/agent-run.{sh,ps1}` be materialized from Section 3 alone — all five command bodies copy-paste concrete (`boot` = `cargo run -p conductor-cli ... preflight --json` with stub-leg-in-CI vs. live-leg gated on `ANDROMEDA_PULSE_MCP_ENABLED`; `run` = `cargo nextest run --workspace --profile ci` + `cargo test --workspace --doc` + clippy; `status` = `jq -e` / rusqlite bound-param `SELECT`; `cleanup` = `rm -f` + bound-param `DELETE` + `:4317` release; `logs` = read `runs/<run_id>.jsonl`) — with the both-shell-variants-identical-semantics contract stated?
- **Read-chain direction (adversarial-by-default):** Does any Section 3 bootstrap phase or derivation note make a *downstream* consumer the *upstream producer* of a contract this plan already owns? The plan's header declares it "the single source of truth for all test strategy + harness contracts," and Section 3 fully specifies the log format itself — so a phase that says the harness will "consume obs-plan §3 ... verbatim" inverts the read chain and points setup-project at a file that does not yet exist.
- **obs / a11y / route derivation:** Can obs derive the product-side log envelope from the Section 3 Log-format subsection (it owns only the field-allowlist/redaction layer, asserted here via a negative test — no platform pick)? Can a11y read Section 6's selector contract (text-paired `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`, `role="alertdialog"`, `aria-live`, "HOLD — operator pause") as a *selector* contract, not an a11y ruleset? Can route order the 8 bootstrap phases, with the two N/A phases (status endpoint, PID file) explained as N/A-with-substitute rather than dangling?

**Anchor example:** Section 3 Test Harness Contract, "Bootstrap phases (derive for route / setup-project)"

> "- **log-format-bind-with-obs:** consume obs-plan §3 log format JSON schema verbatim for the `tracing-subscriber` JSON journal per Log format above (binding contract — harness greps the journal for assertions; format break = harness break)."

…reinforced by the closing derivation note on the same list:

> "setup-project materializes each phase's bootstrap script + dependency list + verification command (binding contract with obs' harness contract — log format + envelope shape come from obs-plan §3)."

**Issue:** This inverts the D26 read chain. This plan's header (lines 3-6) declares it "the single source of truth for all test strategy + harness contracts + quality gates," and Section 3's own "Log format" subsection already fully specifies the journal — "structured JSONL — one JSON object per line ... emitted by `tracing-subscriber` 0.3.x `format::Json`" with the complete required-fields list. obs is a *downstream consumer* of that format, not its producer. Searched Section 3 three ways — Grep `obs-plan|obs plan|obs's harness` (hits only at lines 207, 213), Grep `log format|Log format|tracing-subscriber|format::Json` (the format is defined at lines 67, 184-186, with no upstream obs source), and an end-to-end re-read of lines 182-213 — the format is owned and defined here, so "come from obs-plan §3" is a genuine direction inversion, not a legitimate cross-reference. The correct framing is the inverse: obs consumes this plan's Section 3 log format; obs owns only the redaction/allowlist layer, which this plan asserts via a negative test.

**Why this matters:** setup-project is the most-weighted consumer and reads Section 3 verbatim to materialize `scripts/agent-run.{sh,ps1}`. Pointed at "obs-plan §3," it blocks on a file that does not exist at test-plan time, or worse, emits a TODO that stalls the harness boot. The fix is a contract reversal (state that obs derives FROM this section; this section defines the format), which is substantive and downstream-blocking — exactly the bucket-1 spend.

**Adversarial:** If the iteration agent "fixes" this by *deleting* the obs reference entirely rather than reversing its direction, does the field-allowlist/redaction ownership boundary (which legitimately belongs to obs) silently vanish — leaving setup-project to either implement redaction itself (scope creep into obs) or ship a journal that leaks host paths / struct names that the Section 11 negative test was supposed to guard? The patch must reverse direction AND preserve "obs owns the redaction layer; this plan asserts it via a negative test."

### 2. Test Scope Faithfulness [priority: high]

- Does every testable entity from test-scope Section 1 receive coverage in Section 4 (Unit) / 5 (Integration) / 6 (E2E)? Cross-check all 13 — the 8 `conductor-*` crates, the scenario-config validation surface, the error-handling surface, the scenario catalog + `coverage-matrix.md`, and the `runs.db` + JSONL store — against Section 4's "What unit tests cover" list. Is any entity silently dropped, or any *partially-testable* entity (`conductor-emit` gRPC egress, the live-Pulse leg of `conductor-verify`, the `:4317` bind of `conductor-faults`, the webview leg of `conductor-tauri`) promoted to full CI coverage instead of held to its unit-CI / live-local-gate split?
- Are the four untestable zones (live Pulse + encrypted `corpus.db` / P-049; `OsKeychainBackend`; `drive+observe` human visual judgment; `static-only` capabilities) excluded from Section 6 with no invented driver, and does the Section 6 skip note still enumerate all four? Are all 7 critical paths rendered as `#### Scenario:` headings with Steps + Verification + Cleanup, and `pii-scrub` + `connection-lifecycle` still folded into Path-1's pattern (not dropped, not pushing the count past 7)?
- Is each of the 11 coverage triggers addressed somewhere (Vector-1 path-traversal negative; Vector-2 garde property test; Vector-4 rmcp STDIO injection / empty-canary-`blocked` / bounded-prost; Vector-3 Tauri deny-by-default; `runs.db` bound-parameter assertion; contract test negotiating down to `2024-11-05`; determinism-replay property test; cross-surface parity; bounded chaos; `cargo-audit`/`cargo-deny` gate), and does the Section 1 Test Scope Summary copy the Phase 1 tier label / justification / surfaces / harness reqs / paths / triggers verbatim rather than paraphrased?

**Anchor example:** Section 1 Test Scope Summary, Critical paths

> "(7 paths — the maximum. `pii-scrub` and `connection-lifecycle` Must-Work scenarios are covered by E2E suite expansion as additional auto-scenario instances of Path 1's pattern.)"

**Affirmation:** This is faithful and correctly bounded. Cross-checked against Section 6: the seven `#### Scenario:` headings (lines 304, 319, 326, 333, 340, 347, 354) match the seven paths one-for-one, the count cap "(7 scenarios — the test-scope Section 4 maximum.)" is restated at line 363, and `pii-scrub` / `connection-lifecycle` are explicitly re-homed under Path 1 at line 317 ("Also covers `pii-scrub` and `connection-lifecycle` as additional auto-scenario instances of this pattern"). No path is dropped and none over-counts. No patch needed in the current state.

**Why this matters:** Scope faithfulness is the spine of iteration completeness — a dropped path or an over-counted scenario silently changes what "done" means for the whole phase loop. This dim stays in the loop because later iterations may add or rename scenarios, and the 7-path cap + Path-1 folding is exactly the invariant that regresses first when scenarios get edited.

**Adversarial:** If a later iteration promotes `pii-scrub` to its own 8th `#### Scenario:` heading (it reads like a first-class flow), does the plan now over-count past the test-scope maximum of 7 and silently re-weight the GUI-secondary surfaces — and would the iteration agent catch that the cap note at line 363 now contradicts the heading count?

### 3. Tool Anchoring (Catalog ↔ Plan) [priority: high]

- Does every named tool in the plan appear in `test-research.md` with the **matching version**? Spot-check: cargo-nextest 0.9.137, cargo-llvm-cov 0.8.7, rstest 0.26.1, assert_cmd 2.1.2 + predicates 3 + assert_fs 1, rmcp 1.7.0, tauri 2.10.3 (`test` feature), @crabnebula/tauri-driver 2.0.9, proptest 1.9.0, insta 1.46.1, turmoil 0.7.2, wiremock-rs 0.6.5, cargo-audit 0.22.2, cargo-deny 0.19.6, tracing-subscriber 0.3.x. Flag any version drift or any tool named in the plan but absent from the catalog (a fabricated pick).
- For catalog alternatives, does the plan pick one and state the trade-off — (a) cargo-nextest vs. plain `cargo test` with the `cargo test --doc` doctest-fallback rationale; (b) wiremock-rs NOT force-fit onto the gRPC egress (in-process tonic `TraceServiceServer` instead); (c) turmoil framed as *optional / not load-bearing*? Is each tool's catalog agent-runnable mechanism preserved as a concrete machine signal (`NextestExitCode` 100/101/4 + JUnit; `--fail-under-lines` exit; `jq -e` exit 1/4; `serde_json::from_str::<RunReportEnvelope>`; assert_cmd `.code()`/`.success()`; insta CI fail-don't-write)?
- Is the Decisions-Log "no JS/TS unit-test runner for the React 19 SPA" gap honestly carried (webview asserted only at the E2E/parity layer; no vitest-class tool invented), not quietly resolved by slipping in a frontend unit runner the catalog lacks?

**Anchor example:** Section 5 Integration Test Strategy, Driver(s)

> "**Driver(s):** rmcp 1.7.0 in-process duplex / `TokioChildProcess` stub server (MCP read-back); `tauri::test::mock_builder()` + `mock_context(noop_assets())` + `get_ipc_response()` (Tauri commands, behind the `test` feature, `tauri` ≥ 2.10.3); rusqlite 0.38.0 `Connection::open_in_memory()` / `assert_fs::TempDir`-backed file DB; assert_cmd 2.1.2 `Command::cargo_bin` for the CLI subprocess leg of cross-surface parity."

**Affirmation:** Every tool + version here traces to `test-research.md` (verified by Grep over the catalog: cargo-nextest 0.9.137 line 5, rmcp 1.7.0 line 61, tauri 2.10.3 `test` feature line 422, turmoil 0.7.2 line 223, wiremock-rs 0.6 line 154, proptest 1.9 line 192, insta 1.46.1 line 199, cargo-audit/cargo-deny lines 235/245). The three alternative-picks are correctly resolved: doctest fallback (line 148), wiremock NOT-on-gRPC (line 423 + Section 11 line 540), turmoil optional (line 407 + Decisions-Log line 584). No fabricated pick; no version drift. No patch needed in the current state.

**Why this matters:** A fabricated framework or drifted version propagates straight to `/implement` and breaks test execution at install time — bucket-1 cost. This dim is mandatory and stays even when clean, because version bumps and "just use X" additions during iteration are the most common way a clean catalog mapping later rots.

**Adversarial:** If a future iteration "tightens" the gRPC egress story by replacing the in-process tonic `TraceServiceServer` line with a wiremock-rs mock (it reads simpler), does that silently violate both the catalog key-detail (line 156, "do not force-fit wiremock onto the gRPC leg") and the Section 11 Mocking ban (line 540) — and would the iteration agent notice the contradiction across catalog + Section 11, or only see a locally-plausible mock?

### 4. Tier Calibration [priority: high]

- The plan declares **Minimal (0)** but augments with Section 5 triggers, so Integration (Sec 5), E2E (Sec 6), Mocking (Sec 8) are present *by trigger*. Does each justify inclusion against a specific trigger (integration: MCP read-back / `runs.db` / Tauri IPC; E2E: the 7 paths; mocking: security-vector negatives + MCP stub) rather than reading like an un-triggered Comprehensive escalation?
- Do the Section 10 thresholds match Minimal defaults (line ≥ 60 / branch ≥ 50 / function ≥ 70), with the `--fail-under-lines 60` gate and the "branch informational on stable, line gate binding" note internally consistent and not silently raised to Standard or dropped below 60? Are Performance Budgets N/A and the Compliance subsection omitted, both justified in-line and consistent with test-scope Section 5 — with no perf-budget table or compliance row sneaking in?
- Does the Section 2 pyramid keep Performance/Load **Excluded** while keeping Property-based and Chaos/Fault **Included via trigger** (not blanket Comprehensive), with the "Included via trigger" labels pointing at real Section 5 triggers and the bounded P-060 chaos not conflated with load?

**Anchor example:** Section 2 Test Strategy, Test pyramid table

> "| Performance / Load | Budget assertions on perf-critical paths | **Excluded** — no perf-budget trigger; Creator Brief anti-pattern "NOT a load-tester ... 50k+ spans/sec saturation regimes are explicitly out" |
> | Chaos / Fault | Bounded failure injection (conductor-faults generators + loopback port probe; optional turmoil transport faults) | **Included via trigger** — bounded "typical/high" profiles only (P-060); NOT saturation — see Section 8 |"

**Affirmation:** Tier calibration is correct and self-consistent for Minimal-at-upper-boundary. Performance/Load is Excluded with the load-tester disqualifier; Chaos is Included-via-trigger and explicitly bounded (not saturation), matching Section 10's "Performance budgets: N/A" (line 482) and the Section 11 ban on load escalation (line 507). The Section 10 thresholds (line 476: ≥ 60 / ≥ 50 / ≥ 70) match Minimal defaults exactly and the `--fail-under-lines 60` gate (line 480) is consistent. No patch needed in the current state.

**Why this matters:** Over-engineering (Comprehensive sections in a Minimal project) or a silent threshold drift mis-sizes the entire test build — implementation-misleading, bucket-2. This dim must stay because the boundary between "bounded chaos correctness" and "load test" is exactly what a later iteration blurs when it adds fault scenarios.

**Adversarial:** If a later iteration adds a fingerprint-storm variant that ramps span volume "to see where Pulse degrades," does that quietly cross the Excluded Performance/Load line while still wearing the "Chaos / Fault — Included via trigger" label — and would the iteration agent catch that the new scenario is saturation testing (Pulse's `perf_load_profiles.rs` domain, line 482) wearing a chaos costume?

### 5. Anti-Pattern Relevance [priority: medium]

- Are the Section 11 stack-specific bans grounded in *this* Rust/Tauri/MCP stack, not generic? Verify the load-bearing ones map to real pitfalls: never `format!`-build `runs.db` SQL (rusqlite bound params); never stamp the journal from tokio's virtual clock (`std::time::SystemTime`); never force-fit wiremock-rs onto gRPC; never interpolate `ANDROMEDA_PULSE_DATA_DIR` into the sidecar argv (rmcp STDIO injection, CVE-2026-30623); never treat `blocked`/`ManualCheck`/`KnownResidual`/`CalibrationRegion` as a non-zero exit; never add a scenario without a P-ID; never promote the `:4317` port-occupier into a general listener.
- Does each domain subsection (Test Strategy / Unit / Integration / E2E / Test Data / Mocking / CI / Quality / Universal) carry at least one stack-specific ban, not a placeholder? Do the Universal bans stay genuinely cross-stack and agent-driven (zero-flakiness / no nextest `retries` / no Percy-Chromatic-Applitools / no manual smoke / no Selenium-IDE or WebdriverIO recorder / no real network beyond loopback / no real time without tokio injection), with `cargo insta review` interactive UI banned *as a test author* while insta CI/assert mode stays approved?
- Are bans centralized in Section 11 and merely cross-referenced from Sections 4-10 (`See Section 11 § ...`), not duplicated inline in section bodies?

**Anchor example:** Section 11 Test Anti-Patterns, Integration subsection

> "- **(stack-specific)** NEVER build `runs.db` SQL via `format!`/string concatenation — use rusqlite bound parameters even for synthetic data (security anti-pattern); a negative/static test asserts this."

**Affirmation:** The bans are genuinely stack-anchored, not generic. Each of the nine domain subsections carries a `**(stack-specific)**` or `**(project-specific)**` entry tied to a real Conductor/Rust/Tauri/MCP pitfall (Test Strategy: fake-Pulse-reaction + load-escalation, lines 506-507; Unit: real-wall-clock in timeline, line 513; Integration: the `format!`-SQL ban quoted above, line 519; E2E: state-as-exit-code, line 526; Test Data: virtual-clock journal stamp, line 533; Mocking: wiremock-on-gRPC + argv injection, lines 540-541; CI: audit-green + no-live-Pulse-CI-gate, line 548; Quality: retry-once, line 552; Universal `**(project-specific)**`: no-P-ID-scenario + sole `:4317` bind, line 564). The negative-test boundary ("a negative/static test asserts this") is stated, not the test body — correct out-of-scope discipline. No patch needed in the current state.

**Why this matters:** Generic bans dilute the signal so developers stop reading Section 11 for the load-bearing stack rules (the argv-injection CVE, the virtual-clock journal corruption). This dim is signal-diluting (bucket-3, medium) — patch only if a domain's ban actually goes generic or a ban gets duplicated inline. It stays because added sections during iteration tend to grow generic placeholder bans.

**Adversarial:** If a later iteration adds a new domain subsection (or a new ban) that restates a stack rule *inline* in a section body instead of cross-referencing Section 11 — e.g. re-typing the bound-parameter SQL ban in Section 5's body — does Section 11 stop being the single source of truth for bans, so a future edit fixes one copy and leaves the other stale? Would the iteration agent catch the duplication, or only see two individually-correct statements?

### 6. Cross-Surface Coordination Coverage [trigger: multiple surfaces in test-scope Sec 2 OR cross-surface-coordination trigger in Sec 5] [priority: medium]

- The plan spans cli + ipc-internal (MCP) + ipc-internal (Tauri) + desktop-webview + persistent-storage. Is the cross-surface-coordination trigger covered in Section 5 — the parity test where the `tauri::test` mock-runtime run and the `assert_cmd` CLI subprocess run, both pointed at the *same* `assert_fs::TempDir` file `runs.db` via `CONDUCTOR_RUNS_DIR`, assert an identical envelope (`verdict`/`state`/`seed`) for the same scenario+seed?
- Does the Section 6 "Both-surface parity" scenario enumerate its surfaces and specify *how* the agent confirms state transitioned across surfaces (Tauri-launched run's `runs.db` envelope == headless `conductor run` envelope, journal written per run from both paths)? Does the plan honestly acknowledge the documented coordination *gap* — `tauri::test` mock runtime does NOT bridge `Channel` frames, so the live-counter sequence assertion falls to a direct unit test of the channel-producing fn or the tauri-driver E2E leg — rather than over-promising full Channel coverage? Is the CLI-as-gate / GUI-as-"convenience only" hierarchy preserved so parity is not mis-weighted as a primary gate?
- Is there a Section 11 anti-pattern guarding against testing a surface in isolation when the parity path spans both, consistent with the no-P-ID / single-`:4317`-bind scope law?

**Anchor example:** Section 6 E2E Test Strategy, "Both-surface parity (control panel AND headless)" scenario

> "  2. GUI leg (CI): tauri::test mock runtime `get_ipc_response()` start command for the same scenario+seed, same `CONDUCTOR_RUNS_DIR`; (webview render leg: tauri-driver `wdio run` headless on Linux/`xvfb` asserting the Run console live/HOLD layout + envelope display)."

**Affirmation:** The cross-surface coordination is correctly specified and honestly bounded. The parity scenario points both legs at the same `CONDUCTOR_RUNS_DIR` file `runs.db` and asserts envelope identity (`verdict`/`state`/`seed`) at line 360; the Section 5 integration row states the same shared-TempDir mechanism (line 279); and the Channel-frame gap is acknowledged in three places (Section 5 boundary table line 266, Section 6 Tauri driver row line 295, and the catalog key-detail line 76) — "mock runtime does not bridge Channel frames," routing the live-counter assertion to a direct unit test or the tauri-driver leg. The GUI-secondary hierarchy is preserved ("GUI is 'convenience only,' CLI is the gate," line 296). No patch needed in the current state.

**Why this matters:** Cross-surface parity is Critical Path 7; if the iteration agent lets the parity assertion drift to a single surface, or lets the Channel-frame gap get papered over as "covered by the mock runtime," the plan over-promises coverage the `tauri::test` runtime structurally cannot deliver. This is a trigger-driven conditional dim kept (not dropped) because the trigger is present and the Channel-gap honesty is fragile under edits.

**Adversarial:** If a later iteration "simplifies" the parity scenario by asserting the live-counter `Channel` sequence directly through `get_ipc_response()` (assuming the mock runtime streams frames), does the plan now claim coverage that the documented gap at lines 76/266/295 says is impossible — producing a scenario that passes in the plan but cannot be implemented against the `tauri::test` runtime? Would the iteration agent cross-check the three gap statements, or accept the locally-plausible "just assert the channel" edit?

### 7. E2E Selector Strategy Robustness [trigger: E2E Section 6 present AND UI surfaces (desktop-webview) in test-scope Sec 2] [priority: medium]

- The desktop-webview surface (Tauri 2 bundled webview, React 19 + Vite + Tailwind v4.1 + shadcn/ui) is driven by @crabnebula/tauri-driver 2.0.9 headless via `wdio run`. Does the Section 6 Selector strategy use role / text / data-testid / brand anchors and explicitly ban xpath and hashed Tailwind/CSS class selectors, leaning on the Color-Only a11y rule (every lamp text-paired) so text/role selectors are reliable?
- Are selectors anchored in the upstream layout-templates Signature Placements — the "HOLD — operator pause" heading line, the `role="alertdialog"` operator-pause dialog, `aria-live` HOLD/verdict announcements, and the mono P-ID tokens (`P-001`..`P-060`) for coverage-matrix/report rows? Is the strategy consistent with the Section 11 E2E bans (no xpath, no `sleep(N)` — wait on envelope `state` / JSONL journal line / element role / `aria-live` / exit code), and is the macOS constraint honored (no WKWebView WebDriver → webview E2E on Linux CI under `xvfb` only, role/text/`aria-live` assertions only, no visual-pixel/Percy)?

**Anchor example:** Section 6 E2E Test Strategy, Selector strategy

> "**Selector strategy:** role/text/data-testid + brand anchors (upstream-context Layout Templates Signature Placements + Design System brand anchors) — text-paired status labels `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` (color never sole signal, so text/role selectors are reliable per the Color-Only a11y rule), `role="alertdialog"` on the operator-pause dialog, `aria-live` HOLD/verdict announcements, signature heading text "HOLD — operator pause", and mono P-ID tokens (`P-001`..`P-060`) for coverage-matrix/report rows. NO xpath, NO hashed-CSS-class selectors."

**Affirmation:** The selector strategy is correctly anchored to durable Signature Placements (text-paired labels, `role="alertdialog"`, `aria-live`, "HOLD — operator pause" heading, mono P-ID tokens) and bans the drift-prone selectors (xpath, hashed-CSS) — consistent with the Section 11 E2E bans (lines 523-525: no xpath, no `sleep(N)`, no hashed-class) and the macOS/`xvfb`-only constraint (Section 6 driver row line 296). It stays a *selector* contract and does not stray into a11y conformance rules (a11y's domain). No patch needed in the current state.

**Why this matters:** A selector anchored to a hashed Tailwind class or an xpath path drifts every shadcn/Vite rebuild, making the webview E2E leg flaky — and flakiness is explicitly zero-tolerance here (Section 10). This trigger-driven conditional dim is kept because the desktop-webview surface is present and selector robustness regresses the moment a new E2E step is added against a non-signature element.

**Adversarial:** If a later iteration adds a webview assertion that waits on a CSS class or a fixed `sleep(500)` to "let the live counter settle" (the path of least resistance for a render-timing race), does it silently violate both the no-hashed-class and the no-`sleep(N)` Section 11 E2E bans (lines 524-525) — shipping a flaky selector into a zero-flakiness gate? Would the iteration agent catch that the new step waits on a timer/class instead of the envelope `state` field or `aria-live`?

---

Read the document, walk the Analysis Protocol cross-reference checklist, analyze along all dimensions with the anchors in mind, then output patches and changelog within the 8-patch priority budget.
