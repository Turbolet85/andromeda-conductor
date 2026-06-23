# tests extract

## Relevance
partial — operator-pause adds a CLI interactive surface (new `PauseResolver` impl) that touches the `conductor-cli` test boundaries but does not introduce new core-level test mechanics

## Constraints
- Per §1 Scope Summary (entity `conductor-cli`): the headless `scripts/agent-run.sh` path is the source of truth and release gate; testability via `Command::cargo_bin` subprocess + exit-code/stdout assertions (agent-driven)
- Per §3 Test Harness Contract / 5-command discipline: `run` command must never block on an interactive prompt in headless/piped mode (isatty gate → `HeadlessResolver::proceed()` auto-resolves); `[HOLD]` label always present in output, never color-alone (Design System anchor)
- Per §6 E2E Test Strategy: the cli surface asserts exit code + structured `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]` labels (NO_COLOR-stable per predicates 3) with stdin closed to prove "never blocked on an interactive prompt" (Critical Path 1 + 2 legs, all E2E scenarios must pass headless)
- Per §11 Test Anti-Patterns (Universal): cli release-gate scenarios run the real binary via assert_cmd; the interactive path MUST be testable non-interactively (mocked/auto-resolved resolver)
- Per §2 Test Strategy (agent-runnable invariants): every test layer produces machine-parseable output + no human-in-loop verification; deterministic `#[tokio::test(flavor = "current_thread", start_paused = true)]` required

## Patterns to follow
- Interactive resolver is a new `PauseResolver` impl behind a thin `conductor-cli` seam; reuse existing `HeadlessResolver` logic from `conductor-core::pause` (zero core-model change)
- Unit-level resolver tests use rstest `#[fixture]` seeding + `#[tokio::test(flavor="current_thread")]` mock resolver injection, NEVER real TTY wait
- Integration/E2E hold-point tests exercise the resolver via assert_cmd with stdin closed + envelope/JSONL assertions (no hang on piped run); per §3 the paused-count spinner mirror must emit structured `[HOLD]` labels reusing §2 ch3 indicatif seam
- Golden tests (insta) on the envelope + JSONL journal redact `run_id`/timestamps but preserve the hold-path determinism assertion

## Anti-patterns to avoid
- NEVER allow an interactive `inquire` prompt to write to a pipe (§11 universal: "NEVER include a manual smoke step"); stdin-closed CLI runs MUST auto-resolve
- NEVER introduce a second `Decision` set or a second hold model; the single `PauseResolver` trait + `conductor-core::pause` types are the vocabulary
- NEVER assert on real wall-clock duration in the virtual-clock determinism test (§11 unit); mock resolver latency does not count against SLO bounds

## Contract bindings
- obs ↔ tests harness: the hold await may emit a bounded `tracing` event for paused/resumed transition (per scope), MUST use wall-clock `std::time::SystemTime` NOT virtual clock, MUST redact raw prompt (scope §Obs); this is the log-format binding point (§3 Test Harness Contract / Log format — the per-run emission journal is ground truth for SLO checks)
- core ↔ cli: the interactive resolver implements the one `conductor_core::pause::PauseResolver` trait; `HoldPoint`/`Decision`/`resolve_hold`/`HeadlessResolver` are read-only imports, zero core additions
- render seam: reuses §2 ch3's `stdout_color()` tty-gate + `indicatif` progress + `Lamp`→`[HOLD]` line mapping (no new rendering logic)

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-cli` passes for the new `PauseResolver` unit tests (mock injected resolver, virtual-clock determinism verified).
- (tests) E2E path (all 7 critical paths § §6): cli subprocess with stdin closed exits 0 on auto-resolve scenario + `[HOLD]` label never appears (headless never blocks); same scenario+seed re-run yields identical envelope/JSONL stream (determinism preserved across hold decision).
- (tests) Headless-gate negative test: confirm an interactive `inquire` call is never reached when stdin is closed or `--agent-mode` active (assert the isatty selector correctly gates to `HeadlessResolver`).
- (tests) No host-path / internal struct name leaked through the hold prompt or `HoldResolution` recorded in journal (artifact-sanitization assertion per §3).

## Relevant amendment history
(none) — the amendments file carries changes through 2026-06-17; this chunk is marked 2026-06-23; no prior amendments address the operator-pause surface (which deferred from Epoch 5 core work).
