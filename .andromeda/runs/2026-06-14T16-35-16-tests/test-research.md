## Test Runner / Framework

### cargo-nextest

- **Version:** 0.9.137 (also distributed as the `cargo nextest` subcommand; `nextest-metadata` 0.12.x for the machine-readable contract)
- **Last release:** 2026-05-26
- **Status:** actively maintained (recent changes include the `rand` 0.10.1 bump for RUSTSEC-2026-0097 and per-command env support — no unresolved critical advisories)
- **Agent-runnable:** yes — documented machine-readable exit codes (`NextestExitCode`: e.g. 100/101 test or build failure, 102 cargo-metadata failure, 4 `NO_TESTS_RUN`) plus the `ci` profile emitting JUnit XML (via `quick-junit`) and `--message-format json`/libtest-json for per-test structured output; configuration: `cargo nextest run --profile ci` with `[profile.ci.junit] path = "junit.xml"` in `.config/nextest.toml`.
- **Fits because:** the arch Test-Relevant Conventions name cargo-nextest / `cargo test` as the runner and require golden tests in CI; it is the runner behind every Section 2 surface's per-seam `cargo nextest run -p conductor-<seam>` (the per-seam isolation convention) and is the engine the `run` harness command (Section 3) wraps for the CI build+test gate.
- **Key detail:** the `ci` profile's JUnit XML is parsed by the `dorny/test-reporter` GitHub Action into per-test PR annotations — agent-readable without scraping logs; exit codes are stable per the published `NextestExitCode` enum so a harness can distinguish "tests failed" (100) from "build failed" (101) from "no tests" (4).
- **Source:** https://nexte.st/docs/running/ , https://docs.rs/nextest-metadata/latest/nextest_metadata/enum.NextestExitCode.html , https://nexte.st/book/junit.html

### cargo test (libtest) — baseline fallback

- **Version:** ships with Rust toolchain (pin ≥ 1.94.1 — see Supply-Chain Audit)
- **Last release:** 2026-03-26 (Rust 1.94.1)
- **Status:** actively maintained (stdlib)
- **Agent-runnable:** yes — exit code 0 = all pass, non-zero = failure; `--format json -Z unstable-options` (nightly) or stable libtest human format; configuration: `cargo test -p <crate>`.
- **Fits because:** arch lists `cargo test` alongside nextest as acceptable; it is the zero-dependency fallback for the same per-seam isolation and for `#[tokio::test]` async unit tests on `conductor-timeline`/`conductor-verify`. nextest is preferred for the JUnit/CI signal; plain `cargo test` covers doctests (which nextest does not run).
- **Key detail:** nextest does not execute doctests — keep a `cargo test --doc` step for any documented examples on `conductor-core` public API.
- **Source:** https://nexte.st/docs/running/ , https://blog.rust-lang.org/2026/03/26/1.94.1-release

## Coverage Tool

### cargo-llvm-cov

- **Version:** 0.8.7
- **Last release:** 2026-05-13
- **Status:** actively maintained (taiki-e)
- **Agent-runnable:** yes — emits LCOV, Cobertura XML, JSON, and Codecov JSON, plus a `--fail-under-lines <pct>` exit-code gate for regression enforcement; configuration: `cargo llvm-cov nextest --lcov --output-path lcov.info` (composes directly with cargo-nextest), or `--cobertura --output-path coverage.xml`.
- **Fits because:** it is the canonical LLVM source-based coverage tool for the Rust stack and runs over the workspace's per-seam crates; it integrates with `cargo nextest` so the same Section 3 `run` invocation produces coverage. Machine-parseable LCOV/Cobertura satisfies the agent-driven release-gate need.
- **Key detail:** `cargo llvm-cov nextest` is a first-class integration — run coverage and the nextest JUnit report in one pass; branch coverage requires nightly but line/function coverage works on the stable pinned toolchain.
- **Source:** https://github.com/taiki-e/cargo-llvm-cov , https://crates.io/crates/cargo-llvm-cov , https://docs.rs/crate/cargo-llvm-cov/latest

## Fixture & Factory Library

### rstest

- **Version:** 0.26.1
- **Last release:** 2025-07-27
- **Status:** actively maintained (la10736)
- **Agent-runnable:** yes — it is a `#[dev-dependencies]` proc-macro layer over `cargo test`/nextest; results surface through the runner's exit code + JUnit; configuration: `rstest = "0.26"` under `[dev-dependencies]`.
- **Fits because:** the harness test-data strategy (Section 3) is self-bootstrapping seeded synthetic generation with **no developer-seeded DB** — rstest fixtures express this cleanly: a `#[fixture]` producing a seeded `conductor-timeline` generator, table-driven `#[rstest]` cases over the P-001..P-060 catalog, and `#[case]` rows for the garde valid/invalid config matrix (Section 5 config-parsing trigger). `#[once]` fixtures suit the in-memory `runs.db` schema setup.
- **Key detail:** rstest supports `#[future]` + `#[awt]` for async fixtures, matching the `#[tokio::test(flavor = "current_thread")]` async tests on `conductor-timeline`/`conductor-verify` without hand-rolled runtime boilerplate.
- **Source:** https://github.com/la10736/rstest , https://crates.io/crates/rstest , https://docs.rs/rstest/latest/rstest/attr.fixture.html

## Per-Surface Drivers

### assert_cmd (+ predicates) — surface: cli (`conductor-cli` / `agent-run`)

- **Version:** 2.1.2 (release tag v1.74.0 cut ~Jan 2026)
- **Last release:** 2026-01-09
- **Status:** actively maintained (assert-rs; the older `assert_cli` is deprecated — use `assert_cmd`)
- **Agent-runnable:** yes — runs the built binary as a child `process::Command` and asserts exit code (`.code(n)` / `.success()` / `.failure()`), stdout, and stderr; configuration: `assert_cmd = "2"` + `predicates = "3"` under `[dev-dependencies]`, `Command::cargo_bin("conductor")`.
- **Fits because:** this is exactly the Section 2 cli driver ("subprocess + `Command::cargo_bin` … assert_cmd-style"). It asserts the release-gate exit-code semantics from Section 3 `run` (0 = all Pass; non-zero = a hard `Fail`; `blocked`/`ManualCheck`/`KnownResidual`/`CalibrationRegion` are reported states, **not** failure exits) and the `NO_COLOR`-stable status labels `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]`. It drives Critical Paths 1-6 (`conductor run <scenario>`, `conductor suite`, `conductor report <run_id>`) and verifies the headless path is "never blocked on an interactive prompt" by asserting non-interactive completion with stdin closed.
- **Key detail:** pair with `assert_fs` (same maintainer) for a `TempDir` `runs/` sandbox so the file-absence + `runs.db` row-count `cleanup` idempotency check (Section 3) runs hermetically; set `CONDUCTOR_*` env via `.env()` to drive the Section 5 Vector-1 path-traversal negative tests.
- **Source:** https://github.com/assert-rs/assert_cmd , https://crates.io/crates/assert_cmd , https://alexwlchan.net/2025/testing-rust-cli-apps-with-assert-cmd/

### rmcp (stdio) + custom stub server — surface: ipc-internal (MCP read-back, `conductor-verify`)

- **Version:** rmcp 1.7.0 (the project's pinned SDK; `client` feature)
- **Last release:** 2026-05-13
- **Status:** actively maintained (official MCP org; stable + dev channels, 1.x migration guides published)
- **Agent-runnable:** yes — JSON-RPC over stdio; typed tool calls return deserializable structs you assert against, and the transport carries `tonic::Status`-equivalent typed error responses; configuration: build a minimal in-test server with `rmcp` + `#[tool]` macros bound to `rmcp::transport::io::stdio()` (or an in-process duplex transport), spawned/connected via `serve_client()` over `TokioChildProcess`.
- **Fits because:** Section 2 ipc-internal driver calls for "a stub/mock MCP server (rmcp) over stdio + JSON schema assertion on the typed tool responses" (`query_incident_list`, `retrieve_report`, `retrieve_telemetry_slice`, `mark_incident_resolved`). A scripted stub lets CI run the preflight contract test (Section 5 contract-test trigger) and the Vector-4 negative tests fully headless: assert protocol negotiates **down** to `2024-11-05`, required-tool presence vs. the pinned `contracts/` manifest, `ready:false ⇒ blocked`, `degraded_mode ⇒ KnownResidual` (Critical Path 5, P-032), and an empty canary round-trip ⇒ `blocked` (never false-pass-as-empty). The real `andromeda-pulse-mcp` sidecar leg stays on the local/operator gate.
- **Key detail:** the same rmcp test transport is the only way to exercise the Vector-4 security triggers in CI — a stub that returns malformed/empty payloads and one that advertises a strict-newer protocol drive the "no silent downgrade" and "bounded prost recursion does not panic" assertions without a live Pulse. Use an **in-process duplex** transport for fast unit tests and `TokioChildProcess` against a tiny test-binary stub for the spawn/`.env()`-injection-rejection path (Vector-4 argv-injection negative test).
- **Source:** https://github.com/modelcontextprotocol/rust-sdk , https://docs.rs/rmcp , https://www.shuttle.dev/blog/2025/07/18/how-to-build-a-stdio-mcp-server-in-rust

### tauri::test (mock runtime) — surface: ipc-internal (Tauri 2 `#[tauri::command]` + `Channel`, `conductor-tauri`)

- **Version:** bundled with `tauri` 2 (**pin ≥ 2.10.3** — see Supply-Chain Audit; behind the `test` feature)
- **Last release:** 2.10.3 patch announced 2026-06-03
- **Status:** actively maintained (tauri-apps)
- **Agent-runnable:** yes — `tauri::test::mock_builder()` + `mock_context(noop_assets())` build an app with a mock runtime (no native webview launched); `get_ipc_response()` invokes a command and returns its serialized result for assertion; configuration: `tauri = { version = "2.10.3", features = ["test"] }` under `[dev-dependencies]`.
- **Fits because:** Section 2's ipc-internal Tauri driver wants "JSON schema assertion on command request/response (start/stop/picker/run-report/operator-pause) and Channel live-counter stream payloads." The mock runtime exercises the in-process command boundary headlessly (no webview), asserting command return shape and the `runs.db` row after a start→stop cycle — the in-process, "not network-exposed" boundary (security Vector 3). It is the agent-runnable half of Critical Path 7 (both-surface parity).
- **Key detail:** the mock runtime does **not** stream `Channel` frames through a real IPC bridge, so live-counter message-sequence assertions need either a direct unit test of the channel-producing function or the tauri-driver E2E leg below; treat command-shape + DB-row assertions as the CI-runnable core and the Channel-frame sequence as the secondary (tauri-driver) signal.
- **Source:** https://docs.rs/tauri/latest/tauri/test/index.html , https://v2.tauri.app/develop/tests/ , https://deepwiki.com/tauri-apps/tauri/4.4-testing-and-mocking

### @crabnebula/tauri-driver (WebDriver, headless) — surface: desktop-webview

- **Version:** 2.0.9
- **Last release:** 2026-02-12
- **Status:** actively maintained (CrabNebula; the maintained successor to the in-tree tauri-driver for Tauri 2)
- **Agent-runnable:** yes — W3C WebDriver protocol; a WebdriverIO/Selenium client asserts on DOM role/text selectors and returns structured pass/fail with a process exit code; runs headless on CI under a fake display (`xvfb`) on Linux; configuration: `tauri-driver` bridging to `WebKitWebDriver` (Linux) / `msedgedriver` (Windows), driven by WebdriverIO's spec runner.
- **Fits because:** Section 2's desktop-webview driver is "tauri-driver (headless)" over Run console idle/live/HOLD + Run report layouts. Selectors are guaranteed reliable by the Design-System Color-Only a11y rule (every lamp paired with text `[PASS]`/`[FAIL]`/…; `role="alertdialog"` on operator-pause; `aria-live` HOLD/verdict; signature heading text "HOLD — operator pause"). This is the GUI half of Critical Path 7 parity. **Constraint applied:** WebdriverIO ships an interactive recorder/inspector — recommend ONLY the headless spec-runner mode (`wdio run`), which exits non-zero on failure and emits JUnit/JSON; no human-in-loop inspector.
- **Key detail:** macOS has **no** WKWebView WebDriver in the official toolchain (Windows + Linux only) — for an agent-driven release gate this is fine because the GUI is "convenience only" and the CLI is the gate (arch Surfaces); run the webview E2E on Linux CI under `xvfb`. Do not put visual-pixel assertions here (no Percy/Chromatic) — assert role/text/`aria-live` only, consistent with the Color-Only rule and the Creator anti-pattern "NO UI automation of Pulse."
- **Source:** https://www.npmjs.com/package/@crabnebula/tauri-driver , https://v2.tauri.app/develop/tests/webdriver/ , https://v2.tauri.app/develop/tests/webdriver/ci/

_(Surfaces skipped: live Pulse + encrypted `corpus.db`, `OsKeychainBackend`, `drive+observe` human visual judgment, and `static-only` Pulse capabilities — all flagged untestable/out-of-scope in test-scope Section 1; no driver researched per instructions.)_

## CI Integration Pattern

### GitHub Actions — nextest `ci` profile + dorny/test-reporter + matrix

- **Version:** GitHub Actions (platform); `taiki-e/install-action` for nextest install; `dorny/test-reporter` for JUnit parsing
- **Last release:** 2026-03-21
- **Status:** actively maintained
- **Agent-runnable:** yes — the `ci` profile writes JUnit XML; `dorny/test-reporter` turns it into per-test PR check annotations; `cargo-llvm-cov --cobertura` uploads coverage; job exit code gates the merge; configuration: matrix over `os: [ubuntu-latest, windows-latest, macos-latest]`, steps `cargo build` → `cargo nextest run --profile ci` → `cargo test --doc` → `cargo clippy -- -D warnings`.
- **Fits because:** arch CI/CD names GitHub Actions running `cargo build` + `cargo nextest`/`cargo test` (incl. golden tests) + `cargo clippy` as **build+test gating only** — the dynamic end-to-end scenario proof (live Pulse, `mcp-server` feature + `ANDROMEDA_PULSE_MCP_ENABLED`) is an explicit **operator/local gate, not a CI gate**. The matrix covers the dev-OS target (Tauri is Windows/macOS/Linux); the tauri-driver webview E2E runs as a separate Linux-`xvfb` job (not on the macOS leg — no WKWebView driver).
- **Key detail:** set the nextest `job_name` per matrix variant so the report tool distinguishes per-OS results; keep the live-Pulse scenarios behind a manual/`workflow_dispatch` or local `scripts/agent-run.sh` invocation gated on `ANDROMEDA_PULSE_MCP_ENABLED`, matching the arch "not a CI gate" rule.
- **Source:** https://nexte.st/book/junit.html , https://docs.mergify.com/ci-insights/test-frameworks/rust/ , https://v2.tauri.app/develop/tests/webdriver/ci/

## Structured Log Parsing

### tracing + tracing-subscriber (JSON) — product-side emission

- **Version:** tracing 0.1.x, tracing-subscriber 0.3.x (features `["json", "env-filter"]`)
- **Last release:** 2026-03-13
- **Status:** actively maintained (tokio-rs)
- **Agent-runnable:** yes — `format::Json` emits newline-delimited JSON (one object per line), parseable with jq/serde; configuration: `tracing_subscriber::fmt().json().flatten_event(true).init()`.
- **Fits because:** Section 3 names the per-run `<run_id>.jsonl` emission journal as ground truth ("one JSON object per line … agent-parseable with jq / serde") and the sanitized stderr ("Error output"). tracing-subscriber's JSON formatter is the canonical Rust mechanism; the harness asserts journal lines via serde. Crucially, the **journal wall-clock stamps must come from `std::time::SystemTime`/`Instant`, NOT tokio's virtual clock** (security anti-pattern) — assert this explicitly in a golden test even though time is paused for determinism elsewhere.
- **Key detail:** tracing layers must NOT leak absolute host paths or internal seam-crate struct names into the journal/report (security anti-pattern + Section 3 envelope rule) — add a redaction/field-allowlist layer and a negative test asserting artifacts carry only `run_id`/`seed`/`scenario`/`p_ids`/`verdict`/`state`/`latency_ms`/`slo_tier`/timestamps/`fingerprints`.
- **Source:** https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/index.html , https://github.com/tokio-rs/tracing , https://oneuptime.com/blog/post/2026-01-25-structured-json-logs-tracing-rust/view

### serde_json + jq — test-side journal/envelope assertion

- **Version:** serde_json 1.x (Rust-side); `jq` 1.7+ (shell-level smoke)
- **Last release:** 2026-05-21
- **Status:** actively maintained
- **Agent-runnable:** yes — deserialize each JSONL line into the Run-report envelope struct and assert fields; `jq -e` returns a non-zero exit code on a failed predicate for shell-level CI smoke; configuration: `serde_json::from_str::<RunReportEnvelope>(line)` in-test, or `jq -e '.verdict=="Pass" and .state=="Pass"' runs/$RUN.jsonl`.
- **Fits because:** Section 3's status mechanism is "read the `runs.db` row (embedded SQL) and/or the per-run JSONL journal — no HTTP endpoint exists." The harness asserts the envelope `{run_id, seed, scenario, p_ids, verdict ∈ {Pass,Fail,CalibrationRegion}, state ∈ {Pass,Fail,ManualCheck,KnownResidual,Blocked}, latency_ms, slo_tier, journal_emitted_at, read_back_observed_at, fingerprints}` for every Critical Path (1-6). `jq -e` gives a CI smoke gate; serde gives typed in-test assertions.
- **Key detail:** `jq -e` exits 1 when the last output is `false`/`null` and 4 on a parse error — usable directly as a `scripts/agent-run.sh` post-run gate; for `runs.db` row checks use `rusqlite` in-test (not a shell), honoring the bound-parameter rule (Section 5 SQL-injection trigger).
- **Source:** https://docs.rs/serde_json , https://nexte.st/docs/running/

## Mocking & Stubbing

_[trigger-driven; pulled in by the security-vector negative-test + MCP-stub-server needs (test-scope Sec 5, Vectors 2/3/4) from test-scope Sec 5; not standard for Minimal but required for trigger coverage]_

### tokio time control (`start_paused` / `pause` / `advance`) — deterministic clock

- **Version:** bundled with tokio 1.48.x (the project's pinned runtime; `test-util` feature)
- **Last release:** 2025-10-14
- **Status:** actively maintained (tokio-rs)
- **Agent-runnable:** yes — built into the runtime, no monkey-patching; results surface through the test runner exit code; configuration: `#[tokio::test(flavor = "current_thread", start_paused = true)]`, then `tokio::time::advance(Duration)`; requires the `test-util` feature.
- **Fits because:** `conductor-timeline` is "a deterministic single-threaded scheduler built on `tokio::time`" with the `current_thread` flavor explicitly chosen for deterministic virtual-clock scheduling. `start_paused = true` + manual `advance` deterministically drives phase-scheduling tests in milliseconds (Section 5 determinism property-test trigger; Critical Path 1 "same seed ⇒ same stream shape"). This is the preferred DI-free, no-monkey-patch time mock the research-targets rule asks for.
- **Key detail:** the virtual clock is for **scheduling determinism only** — the emission-journal/report timestamps must still be `std::time::SystemTime`/`Instant` (security anti-pattern: "Writing journal/report wall-clock stamps from tokio's virtual clock — rejected"). Assert this boundary so a paused-clock leak into the ground-truth artifact is caught.
- **Source:** https://docs.rs/tokio/latest/tokio/time/fn.pause.html , https://tokio.rs/tokio/topics/testing

### rmcp in-process / `TokioChildProcess` stub server — process/fake-server mock

- **Version:** rmcp 1.7.0 (see Per-Surface Drivers for full verification)
- **Last release:** 2026-05-13
- **Status:** actively maintained (official MCP org)
- **Agent-runnable:** yes — typed JSON-RPC responses asserted in-test; configuration: a small `#[tool]`-annotated stub bound to an in-process duplex transport (fast path) or a test-binary spawned via `TokioChildProcess` (spawn-path).
- **Fits because:** this is the "process mocking / fake server" entry for the MCP read-back surface — the stub stands in for `andromeda-pulse-mcp` so the Section 5 Vector-4 negatives (argv-injection rejection, fixed program path, empty/malformed canary ⇒ `blocked`, bounded prost recursion no-panic) and the contract test (protocol negotiates down to `2024-11-05`) run in CI without a live Pulse. Uses dependency injection of the transport, not runtime monkey-patching — satisfying the research-targets preference.
- **Key detail:** keep two stub flavors — an in-process duplex transport for verdict-logic unit tests, and a `TokioChildProcess` test-binary for the spawn/`.env()` injection-rejection path; only the latter exercises the actual `TokioChildProcess` command-construction code the Vector-4 anti-pattern targets.
- **Source:** https://github.com/modelcontextprotocol/rust-sdk , https://docs.rs/rmcp

### wiremock-rs — fake-server library (conditional / OTLP egress smoke)

- **Version:** 0.6.5
- **Last release:** 2025-08-24
- **Status:** actively maintained
- **Agent-runnable:** yes — spins an ephemeral local mock server, request matchers + `.verify()` return structured pass/fail via the runner exit code; configuration: `wiremock = "0.6"` under `[dev-dependencies]`, `MockServer::start().await`.
- **Fits because:** included for completeness as the canonical Rust fake-server library. **Applicability is limited:** Conductor's OTLP egress is gRPC (tonic, not HTTP/1) to `127.0.0.1:4317`, and the OTLP liveness check is a transport-connectability probe (a refused transport surfaces as `Result::Err`, not a verdict — security Vector 5). For the gRPC egress, prefer a hand-rolled in-process tonic test server (a `TraceServiceServer`/`LogsServiceServer` impl bound to an ephemeral loopback port) over wiremock; wiremock applies only if any HTTP smoke surface is later added.
- **Key detail:** do **not** force-fit wiremock onto the gRPC leg — `tonic` ships its own in-process server harness (serve a mock `TraceService` over a `tokio::net::TcpListener` on `127.0.0.1:0`) which is the correct fake-server for `conductor-emit`'s egress liveness/byte-level fault tests; reserve wiremock for any future HTTP need only.
- **Source:** https://github.com/LukeMathWalker/wiremock-rs , https://docs.rs/wiremock/latest/wiremock/

## Integration Test Patterns

_[trigger-driven; pulled in by the cross-surface-coordination trigger (test-scope Sec 5) from test-scope Sec 5; not standard for Minimal but required for trigger coverage]_

### rusqlite in-memory (`open_in_memory`) — DB integration & isolation

- **Version:** rusqlite 0.38.0 (the project's pin; `bundled` → SQLite 3.51.1)
- **Last release:** 2025-12-20
- **Status:** actively maintained (rusqlite org)
- **Agent-runnable:** yes — synchronous in-process API; assertions return `Result` checked by the runner exit code; configuration: `Connection::open_in_memory()` per test for a clean schema, or a `TempDir`-backed file DB via `assert_fs` for file-path tests.
- **Fits because:** Section 2's cli signal and Section 3's `status`/`cleanup` mechanisms read the `runs.db` row directly via embedded SQL (no service to stand up — `bundled` SQLite needs no external process). In-memory connections give each test a clean, isolated `runs.db` (parallel-safe for nextest's per-test process model), covering the Run-report envelope golden tests, the `cleanup` row-count idempotency check, and the Section 5 SQL-injection negative test (assert all access uses bound parameters, no `format!`-built SQL).
- **Key detail:** the `cleanup` idempotency and the cross-surface parity (Critical Path 7) need a **file-backed** `runs.db` in a `TempDir` (so the CLI subprocess and the Tauri mock-runtime test write to the same file and you assert identical envelopes) — use in-memory for pure unit/golden tests and `assert_fs::TempDir` for the cross-process parity/cleanup tests.
- **Source:** https://docs.rs/rusqlite/latest/rusqlite/struct.Connection.html , https://github.com/rusqlite/rusqlite/issues/616

### assert_fs — filesystem fixture & cross-process artifact sandbox

- **Version:** assert_fs 1.x (assert-rs; companion to assert_cmd)
- **Last release:** 2026-05-26
- **Status:** actively maintained
- **Agent-runnable:** yes — `TempDir`/`ChildPath` with `.assert(predicate)` returning structured pass/fail via exit code; configuration: `assert_fs = "1"` under `[dev-dependencies]`.
- **Fits because:** the integration setup/teardown the research-targets rule asks for — it sandboxes the `runs/` directory so the per-run `<run_id>.jsonl` journal + `<run_id>.md` report existence/content assertions (Section 2 cli signal, Section 3 `logs`) and the `cleanup` file-absence check run hermetically and in parallel. It is the harness's per-test setup/teardown for the on-disk artifact surface.
- **Key detail:** combine with `assert_cmd`'s `.env("CONDUCTOR_RUNS_DIR", temp.path())` to point each CLI subprocess at its own sandbox — this is also the vehicle for the Vector-1 path-traversal negative test (feed a `CONDUCTOR_*` path handle with `../` and assert rejection before any write).
- **Source:** https://github.com/assert-rs/assert_cmd , https://crates.io/crates/assert_cmd

## Property-Based Testing

_[trigger-driven; pulled in by the determinism-discipline property-test trigger (test-scope Sec 5) from test-scope Sec 5; not standard for Minimal but required for trigger coverage]_

### proptest

- **Version:** 1.9.0
- **Last release:** 2025-10-26
- **Status:** maintained (proptest-rs; "feature-complete, passive maintenance" — stable, not abandoned; no unresolved critical advisories)
- **Agent-runnable:** yes — failures surface via the runner exit code, and the **minimal failing input + seed are persisted to a regression file** (`proptest-regressions/`) for deterministic replay; configuration: `proptest = "1"` under `[dev-dependencies]`, `proptest! { #[test] fn p(x in strategy()) { ... } }`; tune via `ProptestConfig` (`max_shrink_iters`, `cases`, `failure_persistence`).
- **Fits because:** Section 5's two property-test triggers map here directly: (1) the **determinism discipline** — a property asserting a fixed scenario+seed reproduces an identical emission-journal stream shape across runs (the deterministic-replay invariant on `conductor-timeline`); (2) the **garde cross-field config invariants** (Vector 2) — generate configs and assert garde accepts iff `error_fraction ∈ [0,1]` ∧ `durations ≥ 0` ∧ `p50 ≤ p95 ≤ p99` ∧ severity-mix sums hold, and rejects otherwise.
- **Key detail:** proptest's seed/regression persistence is what makes it agent-runnable for a determinism bar — a discovered counterexample is checked into `proptest-regressions/` and replays deterministically in CI; set `failure_persistence` to a stable path so the agent can re-run the exact failing case. Choose proptest over quickcheck: per-value (not per-type) shrinking composes better for the structured scenario-config and stream-shape strategies.
- **Source:** https://crates.io/crates/proptest , https://docs.rs/proptest/latest/proptest/test_runner/struct.Config.html , https://altsysrq.github.io/proptest-book/print.html

### insta (CI/assert mode) — golden / snapshot for the Run-report envelope

- **Version:** 1.46.1 (+ `cargo-insta` 1.46.1)
- **Last release:** 2026-01-15
- **Status:** actively maintained (mitsuhiko)
- **Agent-runnable:** yes — in **CI mode** snapshot mismatches fail the test (non-zero exit) and no files are written; `INSTA_UPDATE=no` / the `CI` env var (auto-set by most CI) enforces this; configuration: `insta = { version = "1", features = ["json"] }`, `assert_json_snapshot!(envelope)`. **Constraint applied:** `cargo insta review` is an interactive human-in-loop accept UI — recommend ONLY the non-interactive assert/CI mode for the agent-driven gate; the review UI is excluded.
- **Fits because:** the determinism + golden-test triggers want stable artifact-shape assertions. JSON snapshots of the Run-report envelope and the JSONL journal lines are the canonical "golden test" the arch CI/CD note requires ("golden tests run in CI"). insta's redaction (`{ "..." => "[redacted]" }`) handles the non-deterministic `journal_emitted_at`/`read_back_observed_at`/`run_id` fields while keeping `verdict`/`state`/`p_ids`/`slo_tier` exact — and directly enforces the security artifact-sanitization rule (no absolute host paths, no internal struct names) by snapshotting the exact serialized shape.
- **Key detail:** set redactions for the wall-clock and `run_id` fields so the golden test is deterministic under the determinism bar; commit snapshots and rely on the `CI` env var to flip insta into fail-don't-write mode — this is the agent-runnable half, fully separate from the excluded interactive `review`.
- **Source:** https://github.com/mitsuhiko/insta , https://docs.rs/insta , https://www.rustfaq.org/en/how-to-use-snapshot-testing-in-rust-insta-crate/

## Chaos & Fault Injection

_[trigger-driven; pulled in by the bounded fault-injection chaos-test trigger (test-scope Sec 5) from test-scope Sec 5; not standard for Minimal but required for trigger coverage. Bounded "typical/high" profiles only — NOT a load/saturation test (Creator anti-pattern: 50k+ spans/sec is Pulse's own `perf_load_profiles.rs`).]_

### conductor-faults in-crate generators + loopback port-occupier (project-specific pattern)

- **Version:** Rust std (toolchain ≥ 1.94.1)
- **Last release:** 2026-03-26
- **Status:** actively maintained (stdlib)
- **Agent-runnable:** yes — fault generators are pure seeded functions (unit-assertable: same seed ⇒ same ramp/silence shape), and the `:4317` port-occupier is verified by a loopback reachability probe (`TcpListener::bind("127.0.0.1:4317")` success/`AddrInUse` → boolean signal) plus the `runs.db` verdict row; configuration: bind in a test, probe, assert release on `cleanup`.
- **Fits because:** Section 5's chaos-test trigger maps to `conductor-faults` (ramps, silence, port-occupier — security Vector 6) asserting **Pulse's reaction within SLO under bounded faults**, explicitly NOT saturation. The fault-injection *generation* is CI-unit-testable in isolation; the *reaction* assertion needs a live Pulse (local/operator gate). The port-occupier's bind-and-release is the one deliberate port exception — the `cleanup` test (Section 3) must assert it releases `:4317`.
- **Key detail:** because the chaos here is "bounded typical/high profiles" (P-060) not saturation, no packaged chaos framework is required for the product path — the deterministic seeded generators + a port probe are the agent-runnable mechanism. Keep this strictly bounded so it never drifts into the load-testing the Creator excludes.
- **Source:** https://doc.rust-lang.org/std/net/struct.TcpListener.html

### turmoil — optional deterministic network-fault simulation (CI-side, no live Pulse)

- **Version:** 0.7.2
- **Last release:** 2026-04-24
- **Status:** actively maintained (tokio-rs)
- **Agent-runnable:** yes — single-threaded, seeded-RNG deterministic network simulation; all faults reproducible by seed; results via runner exit code; configuration: `turmoil = "*"` under `[dev-dependencies]`, build hosts and inject partitions/latency under a fixed seed.
- **Fits because:** an optional CI-runnable way to exercise the OTLP/gRPC egress transport-fault path (Vector 5: refused/partitioned transport ⇒ `Result::Err` harness fault, NOT a verdict) and the MCP transport silence/drop behavior **deterministically and without a live Pulse** — complementing the local-gate live faults. Lets the harness assert "transport refusal surfaces as `Result::Err`, never a false Pass" in CI.
- **Key detail:** turmoil runs everything on one thread with a seeded RNG — a natural fit for the project's `current_thread` determinism bar; use it only for the transport-layer fault assertions (partition/latency/refusal), not for Pulse's reaction (which is genuinely out-of-process and local-gate-only). `fracture` is a newer drop-in-`#[tokio::test]` alternative if turmoil's host model is heavier than needed.
- **Source:** https://docs.rs/turmoil/latest/turmoil/ , https://lib.rs/crates/turmoil , https://lib.rs/crates/fracture

## Dependency / Supply-Chain Audit

_[trigger-driven; pulled in by the supply-chain audit trigger (test-scope Sec 5) — the Minimal-tier residual-risk control for the `bundled`-SQLite-from-C + OTLP/gRPC/MCP tree.]_

### cargo-audit (RustSec)

- **Version:** 0.22.2
- **Last release:** 2026-06-05
- **Status:** actively maintained (RustSec)
- **Agent-runnable:** yes — scans `Cargo.lock`, `--json` emits machine-parseable results, and **`--deny warnings` makes it exit non-zero on any finding** (without it, findings still exit 0 and CI silently passes); configuration: `cargo audit --deny warnings` (optionally `--json` for parsing).
- **Fits because:** Section 5's supply-chain trigger requires a `cargo-audit` green gate before `cargo build --release`/merge over the `bundled`-SQLite-from-C + OTLP/gRPC/MCP dependency tree, with a committed `Cargo.lock` (drift makes the scan non-deterministic and lets the bundled SQLite C version float past advisory tracking). It is the RustSec advisory check against the exact pinned tree (rmcp, tonic/prost, rusqlite/libsqlite3-sys, tauri).
- **Key detail:** the `--deny warnings` flag is load-bearing for the agent gate — default `cargo audit` reports but exits 0, so CI must pass `--deny warnings` (or `-D warnings`) to actually block. This gate is where the toolchain pin ≥ 1.94.1 (tar-rs CVE-2026-33056 / RUSTSEC-2026-0033, fixed in cargo's bundled tar 0.4.45, Rust 1.94.1 released 2026-03-26) and the `tauri` ≥ 2.10.3 pin (origin-confusion CVE-2026-42184, patch announced 2026-06-03) are enforced.
- **Source:** https://rustsec.org/ , https://www.rustfaq.org/en/how-to-use-cargo-audit-for-security-vulnerabilities/ , https://blog.rust-lang.org/2026/03/26/1.94.1-release

### cargo-deny

- **Version:** 0.19.6 (latest; docs.rs "latest")
- **Last release:** 2026-05-11
- **Status:** actively maintained (EmbarkStudios)
- **Agent-runnable:** yes — four independent checks (advisories, licenses, bans/multiple-versions, sources), non-zero exit on violation, `-D warnings` promotes warnings to errors; configuration: `cargo deny check` with a committed `deny.toml`; `cargo-deny-action` for GitHub Actions.
- **Fits because:** the trigger names cargo-deny as the recommended companion to cargo-audit. Beyond the RustSec advisories check (same DB as cargo-audit), its **bans/multiple-versions** check guards the large OTLP/gRPC/MCP/Tauri tree against duplicate-version drift, and **sources** ensures crates come only from crates.io — both residual-risk controls for a dependency tree this wide. Enforces the committed-`Cargo.lock` determinism the trigger requires.
- **Key detail:** since 0.18.x, dev-dependencies are excluded from multiple-versions and license checks by default (re-enable in `deny.toml` if you want the test-tooling tree audited too); pair the `advisories` check with cargo-audit rather than relying on either alone, and pin the `deny.toml` `[advisories]` to deny (not warn) for the agent gate.
- **Source:** https://docs.rs/crate/cargo-deny/latest/source/README.md , https://crates.io/crates/cargo-deny , https://www.systemshardening.com/articles/cicd/rust-cargo-supply-chain-security/
