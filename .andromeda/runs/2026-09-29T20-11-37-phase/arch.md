# arch extract

## Relevance
partial — no new seam, surface or resource; the chunk touches arch through the harvest-tier grading home, the Probabilistic-Assertion / Timing-Tolerance decisions, the declare-only envelope shape, and the `contracts/` P-025 member's no-reader regime and provenance.

## Constraints
- The hue-shift bound is a timing claim on a deterministic mechanism, so it grades hard pass/fail (a breach is `Fail`), never calibration-region; the grade must not be routed to `CalibrationRegion → ManualCheck` (per architecture §Established Decisions → [Probabilistic-Assertion Policy]).
- The Conductor-side timing model (`read_back_observed_at − journal_emitted_at`, `ExpectedCheck::effective_deadline_ms`, per-check `budget_ms`/`latency_ms`/`deadline_ms`) is scoped to `[[expected]]` checks; the hue `duration_ms` is a SUT-emitted quantity and must not be bound into it (per architecture §Established Decisions → [Timing-Tolerance Model]; chunk scope Boundaries).
- `halo-hue-encoding` is in the declare-only delegated-timing family: zero `[[expected]]` checks, live claims graded at the harvest tier, envelope row landing `verdict: null` by one of the two named routes; the chunk must keep that shape (per architecture §Standard Contracts, the declare-only family list). Its `[[checklist]]` beside a non-empty `expected` is a load-time `CoreError::Config` via `Scenario::check_checklist()` (per architecture §Conventions → Config conventions).
- `contracts/pulse-p025-measurement-contract.md` is a `contracts/` member with NO Rust reader — no `default_path()`, no `resolve_under`, no bounds check, no `CONDUCTOR_*` handle; the chunk's grade must not make a Rust test read it (adding a reader would move it out of that regime and needs a wrap amendment) (per architecture §Occupied Resources → `contracts/pulse-p025-measurement-contract.md`).
- That contract's `provenance` distinguishes Conductor measurements from SUT coordinates and states its Pulse coordinates were read at HEAD `83d4060` and expire when HEAD moves; moving `pinned_at`/provenance to the re-verified HEAD must preserve that per-origin distinction (per architecture §Occupied Resources → `contracts/pulse-p025-measurement-contract.md`; compare the per-clause form of `contracts/pulse-real-model-leg-posture.md` in the same §).
- Verification outcomes stay typed values and `Result::Err` stays harness-faults-only; a malformed or absent capture line in the harvest is a harness fault or a typed non-pass, never a panic-as-verdict (per architecture §Cross-cutting Patterns → Verdict/error wall).
- Live proof requiring a live Pulse is a local operator gate, never a CI gate; CI runs only static gates over committed data (per architecture §Inherited Defaults → CI/CD; §Established Decisions → [CI/CD]).

## Patterns to follow
- Harvest-tier grading as a `conductor-run` test over committed evidence, beside the existing harvest homes (`lifecycle_harvest.rs`, `real_model_harvest.rs`) — no new `Verdict`/`ReportState` variant and no envelope key, as the pre-committed-rule arm did (per architecture §Established Decisions → [Probabilistic-Assertion Policy]). Whether `delegated_timing_harvest.rs` already grades against 2 000 ms is research's question (scope says it pins the old 15 s-tick mechanism).
- Contract-committed-before-drive: the grading rule is fixed in a committed `contracts/` document before the leg, so the grade cannot be fitted to the result (per architecture §Occupied Resources → `contracts/pulse-real-model-leg-posture.md`, which takes the P-025 regime unchanged).
- SUT coordinates are cited with the HEAD they were read at (`measured … HEAD {sha}`) and treated as expiring readings, not durable facts (per architecture §Established Decisions → [Read-Back Dependency Posture], e.g. the `83d4060`-stamped citations).
- Boot-time SUT posture (e.g. `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`) is carried by the operator's `pulse-app` launch and confirmed from Pulse's own log, never set or read by Conductor (per architecture §Occupied Resources → `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`). Whether the hue leg's rise case depends on that window is research's question.

## Anti-patterns to avoid
- Grading the hue bound through an `[[expected]]` check, `budget_ms` or `effective_deadline_ms` (per architecture §Established Decisions → [Timing-Tolerance Model]; §Conventions → Config conventions).
- Adding a Rust reader, `CONDUCTOR_*` override or new env handle for the P-025 contract or the capture path (per architecture §Occupied Resources; §Cross-cutting Patterns → Config management).
- Automating PULSE's UI to force the compact-widget mount; Pulse's visual claims ride the operator checklist (per architecture §Cross-cutting Patterns → Scope law).

## Contract bindings
- arch ↔ tests: the hard grade lives at the harvest tier in `conductor-run` tests over a committed capture fixture, operator-gated and outside CI (architecture §Established Decisions → [Probabilistic-Assertion Policy]; §Inherited Defaults → CI/CD).
- arch ↔ obs: the graded leaf is Pulse's emitted `metric.constellation.hue_update_ms`, not Conductor self-obs; Conductor's tracing field-allowlist does not govern it (architecture §Cross-cutting Patterns → Trust boundary). Whether any Conductor-side allowlist admission is still owed is research's question.
- arch ↔ coverage: claiming `v3-08` / reclassifying P-025 must stay consistent with the manifest-held classification and `check_scenario_backing` (architecture §Established Decisions → [Accepted Capability Set]).
- arch ↔ wrap: the §Occupied Resources entry for `contracts/pulse-p025-measurement-contract.md` states HEAD `83d4060` provenance; if the contract re-pins, that master sentence is a wrap amendment (masters read-only in this chunk).

## Acceptance criteria contributions
- (arch) The hue grade compares Pulse's emitted `duration_ms` to 2 000 ms at the harvest tier in a `conductor-run` test and a breach is a hard `Fail`; `halo-hue-encoding.toml` still declares zero `[[expected]]` checks and no `budget_ms` (per architecture §Established Decisions → [Probabilistic-Assertion Policy]; §Standard Contracts).
- (arch) No `Verdict`/`ReportState` variant, envelope key, `CONDUCTOR_*` handle or new `contracts/` Rust reader is added: `git diff --numstat cdb7082 --` over `crates/conductor-core/src` `crates/conductor-verify/src` shows no such addition (per architecture §Established Decisions → [Probabilistic-Assertion Policy]; §Occupied Resources).
- (arch) The re-pinned `contracts/pulse-p025-measurement-contract.md` carries `sut_version` · `captured_at` · `pinned_at` · `provenance` on its face, names the Pulse HEAD every coordinate was re-read at, and keeps Conductor measurements distinguished from SUT records (per architecture §Occupied Resources → `contracts/pulse-p025-measurement-contract.md`).
