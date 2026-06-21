# tests extract

## Relevance
Partial — operator-pause orchestration adds deterministic hold-point logic + resolver abstraction that affects timeline scheduling and verdict mapping; core holds are testable via stubs (no live Pulse leg). Integration point with timeline/verify still open (scope §8).

## Constraints
- Determinism preservation (test-plan.md §1 Definition + §7 §Seed strategies): same scenario + seed ⇒ identical emission-stream shape; hold introduces wall-clock gap only, no seeded-stream perturbation (§2 §Agent-runnable invariants + scope acceptance anchor).
- Hold resolution is a typed VALUE, never a panic or `Result::Err` (scope §2, verdict/error wall); `no-go` is an operator decision, mapped to a state, not a fault.
- Headless auto-resolve MUST NOT block (scope §3 + test-plan.md §3 5-command implementation `run`); the hold awaits under tokio virtual-time via stubs (test-plan.md §2 Agent-runnable invariants `#[tokio::test(flavor="current_thread", start_paused=true)]`).
- Hold-point model is serde-roundtrippable + redacted (test-plan.md §3 Status endpoint shape + Log format anti-pattern: no absolute host paths, no internal struct names).
- Resolver trait/abstraction decouples awaiting location from resolution mechanism (scope §3), enabling both headless + deferred interactive (Epoch 8/9) consumers on the same core.
- Integration point test setup: in-process duplex rmcp stub (test-plan.md §5 Mocking §What to mock) + seeded `conductor-timeline` fixture (§7 Fixture library) — no live Pulse leg required for unit/integration.

## Patterns to follow
- Seeded fixture pattern per §7: rstest `#[fixture]` producing auto-resolving hold resolver; `#[case]` table-driven valid/invalid hold-point configs (go/no-go, scenario/P-ID, prompt round-trip).
- Deterministic-replay golden test (test-plan.md §5 Critical path 1 + §5 coverage trigger property-test): fixed scenario+seed produces identical envelope + hold outcome across runs; insta redaction of run_id/timestamps/prompt (scope acceptance).
- Exact-assert unit serialization on hold-point envelope (test-plan.md §4 conductor-report bullet, amended 2026-06-16-emission-journal-writer): canonical line shape locked via `assert_eq!`, matching verdict.rs/report_state.rs pattern.
- Integration test boundary (test-plan.md §5): hold-resume cross-module via rmcp stub over stdio + shared `assert_fs::TempDir` `runs.db` (no new DB schema; hold outcome mapped to existing state enum).

## Anti-patterns to avoid
- Hold resolution as `Result::Err` or panic (scope verdict/error wall; test-plan.md §3 anti-pattern: "no harness fault on operator decision").
- Hold-point prompt or outcome leaking absolute paths or internal struct names into journal/report (test-plan.md §3 Log format anti-pattern + §5 artifact-sanitization).
- Headless resolver blocking on I/O or user input (scope §3 "never blocks"; test-plan.md §3 5-command invariant: "TTY-gated interactive `inquire` prompts ... must be agent-bypassable on the headless path").

## Contract bindings
Harness contract (test-plan.md §3): the hold outcome (go/no-go + metadata) is a recorded value in the Run-report envelope (§Status endpoint shape fields + §Log format JSONL shape); journal-emitted-at + resolved-at timestamps are ISO-8601 from `std::time::SystemTime` (NOT virtual clock). Hold-aware assertions fold into existing E2E critical-path tests (§6) via envelope redaction + insta goldens (scope Epoch 5 is last verification epoch; Epoch 8/9 shells inject interactive resolvers downstream).

## Acceptance criteria contributions
- (tests) Seeded scenario+seed + auto-resolving headless resolver ⇒ identical envelope verdict/state/fingerprints as no-hold run (per-scenario determinism-replay golden via insta, run_id/timestamps redacted).
- (tests) Hold-point model round-trips through serde (unit golden `assert_eq!` on canonical line, redaction boundary verified via negative test on journal).
- (tests) `cargo nextest run -p conductor-core conductor-timeline` (integration boundary): hold-resume orchestration asserts via rmcp stub + seeded fixture; go ⇒ resume, no-go ⇒ recorded as state value (neither Fail nor Err, per verdict/error wall).
- (tests) Hold outcome field present in Run-report envelope (test-plan.md §Status endpoint shape) with zero leaked host paths or struct names (negative test on per-run JSONL per data-classification hint §1).

## Relevant amendment history
(none) — no prior amendments to test-plan.md touch operator-pause/hold-point domain. The 2026-06-15-structured-logging-stack amendment (self-obs stream distinct from emission journal) and 2026-06-16-emission-journal-writer amendment (exact-assert unit goldens) establish the redaction + serialization pattern this chunk must follow, but no hold-specific amendments yet.