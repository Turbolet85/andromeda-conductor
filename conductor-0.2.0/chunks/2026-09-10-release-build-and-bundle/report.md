# Report — 2026-09-10-release-build-and-bundle

**Chunk:** Release build and bundle — the version's closing chunk: a `cargo build --release` binary plus the
Tauri 2 bundle produced behind green `cargo audit` + `cargo deny` and an un-drifted `Cargo.lock`, with a final
SLO verification pass recorded against the SHIPPED artifact (`v2-27`), and `v2-21` claimable by a
by-construction enumeration over the matrix itself.
**Date:** 2026-09-10
**Commits:** none since `last_wrap` beyond `6ce161c feat(2026-09-10-live-pulse-in-lane-scenario-round)` (this
chunk is uncommitted at report time; wrap commits it).

## Changes (structured — detectors read this)

- **Files:** `Cargo.toml` · `Cargo.lock` · `crates/conductor-tauri/tauri.conf.json` ·
  `crates/conductor-core/src/obs.rs` · `crates/conductor-report/tests/matrix_ledger_gate.rs` (new) ·
  `conductor-0.2.0/verification-matrix.json` · `conductor-0.2.0/chunks/2026-09-10-release-build-and-bundle/**`
  (scope/research/plan/report + 11 evidence files).
- **Symbols / APIs:** NO public surface added or changed. Two new items, both PRIVATE to
  `conductor-core`'s inline `#[cfg(test)] mod tests`: `static PANIC_HOOK_GUARD: Mutex<()>` and
  `fn panic_hook_guard() -> MutexGuard<'static, ()>`. `lib.rs:48`'s re-export list is byte-unchanged; no
  `pub` was added anywhere, which is the point (the alternative remedy would have widened the crate's public
  API for tests alone). One new TEST TARGET, `conductor-report --test matrix_ledger_gate` (4 tests). No ports,
  sockets, env vars or IPC methods added. Remaining-caller facts, from the P3 code-graph query (rust plane,
  `db_state: fresh`): `log_panic` keeps exactly 3 call sites (production `install_panic_hook` @`obs.rs:172`
  plus the two guarded tests) and `build_subscriber` keeps 5 — this chunk removed no caller.
- **Crates / modules:** none added, none removed. Workspace stays 9 members.
- **Dependencies:** **none added, none bumped.** `Cargo.lock` package count **562 → 562** (basis:
  `grep -c '^\[\[package\]\]' Cargo.lock` against `git show HEAD:Cargo.lock`). The lock's 9-line delta is
  entirely the workspace members' own `version` fields moving 0.1.0 → 0.2.0.
- **Schema / config:** the workspace package-version stamp `[workspace.package] version` 0.1.0 → **0.2.0**
  (`Cargo.toml`), and `tauri.conf.json` `version` 0.1.0 → **0.2.0**. The two move together by requirement —
  apart, the binary and the bundle would disagree. Operator decision at the P4 review; no spec mandated a
  stamp either way.
- **Spec-master edits:** none in this chunk (implement authors no spec change; the amendment below is P2's).
- **Counts / qualifiers moved:** the package-version stamp 0.1.0 → 0.2.0 is the only derived value this chunk
  moved. **No master or leaf bakes it** — basis: `grep -rn 'CARGO_PKG_VERSION' .andromeda/*.md` returns
  `obs-plan.md:57` and `:194`, both stating the MECHANISM (`service.version` = `env!("CARGO_PKG_VERSION")`)
  rather than a literal, which is the correct set-naming form and not a hit. The `0.1.0` tokens elsewhere
  (`input.md`, `master-route.md ## conductor-0.1.0`) name the RELEASE VERSION of a prior epoch, a different
  referent from the package stamp. Supply-chain figures re-read, not carried: **1243 advisories · 562
  packages · 7 allowed** — identical to the 2026-09-10 reading already recorded, so nothing moved there.
- **Dev-tool versions:** **`tauri-cli`: absent → 2.11.4**, installed this chunk via
  `cargo install tauri-cli --locked`. It is a HOST DEV-TOOL, not a lockfile dependency — `tauri-cli` and
  `tauri-bundler` are both **absent from `Cargo.lock`** (basis: package-name probe over the lock). No other
  tool moved.
- **Harness / gate surface:** **none.** `scripts/agent-run.{sh,ps1}`, `.github/workflows/ci.yml`, the status
  and verdict shapes and the 5-command discipline are all byte-unchanged. The chunk drives existing verbs only.
- **Cross-project / external claims:** the three live legs and the preconditions probe measure a running
  `andromeda-pulse` instance (external repo), operator-launched 21:16:17 local, PID 29608, fresh data dir,
  deterministic L4, MCP on, default bootstrap window (no override). Basis read: the run envelopes and
  `runs.db` rows this session produced, committed under `evidence/`; no claim is made about Pulse's source.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none authored by this chunk. (A prior chunk's fix
  IS newly measured insufficient — `Titlebar.tsx:23-27` scoped `aria-live` assertive to `hold` citing SR row
  S3-04, and S3-04 still moved — but that change is not this chunk's, and it is recorded under CARRY C rather
  than claimed here.)
- **Spec claims disproved by measurement:** **ONE sentence, `architecture.md:206`**, carrying two false facts
  in the same clause: *"the Tauri 2 bundler (2.11.3) produces an optional ~3 MB GUI installer."*
  (a) The bundler is **not 2.11.3**. `tauri-cli`/`tauri-bundler` are absent from `Cargo.lock` altogether, so
  the repo never resolved a bundler version at all; the number mirrored the `tauri` CRATE version. First
  actual install measured **tauri-cli 2.11.4**.
  (b) The installer is **not ~3 MB**: measured **nsis 4 418 544 B (4.21 MB)** and **msi 6 152 192 B
  (5.87 MB)**, both 2026-09-10 21:31.
  **Re-measured at the wrap's light gate (23:19 local) and the byte figure did NOT hold:** the same
  `cargo tauri build` over unchanged source emitted nsis **4 414 280 B**, 4 264 bytes below the reading
  above, while the msi reproduced byte-exactly. Both nsis readings are 4.21 MB, so what is durable is
  the MB scale, not the byte count. The amendment applied for (b) therefore states the MB pair and names
  the variance — the byte-exact form it briefly carried was a literal MORE precise than its subject,
  which is the same defect class as the `~3 MB` it replaced. Evidence: `evidence/release-artifacts.md`
  second-reading note; sidecar entry appended at `architecture-amendments.md`.
  **Do not touch the CRATE sites** — `tauri` 2.11.3 is correct and still resolves (basis: lock probe):
  `architecture.md:204`, `security-plan.md:181` and `:379`, `.claude/rules/verification-harness.md:57` all
  say `tauri` the crate and are unaffected. The BUNDLER claim additionally co-occurs at
  `architecture.md:27`, `:58`, `:266` and `security-plan.md:84`, with the `~3 MB` half also at
  `design-system.md:117` and `:367` (as a depth-budget rationale) and leaf `.claude/docs/stack.md:45`.
- **Expected amendments (from plan):**
  - `test-plan.md` §4 — dev-tool roster gains `tauri-cli` as a host-tool FLOOR beside cargo-audit /
    cargo-deny / cargo-llvm-cov / cargo-mutants. **CARRIED** — its motivating fact is the *Dev-tool versions*
    bullet above. Search locating the site: `grep -c 'tauri-cli' .andromeda/{seven masters}` → **0 hits in
    all seven**, so the entry is an ADDITION, not a correction; the owning home is `test-plan.md:231`
    ("Tool-version policy… external-CLI tool versions… are reference floors, not exact pins"), which already
    enumerates exactly that class (`grep -n 'Tool-version policy' test-plan.md` → 1 hit).
  - Route-resolve hand-off (deliberately NOT an amendment entry): `.andromeda/residuals.md` is unreachable by
    the amendment flow; the SLO tier-class finding goes to P5 route-resolve, which owns that file's `open`
    appends. Its motivating fact is in *Deviations* below.
- **Coverage of new surfaces:**
  - `conductor-core::obs tests::PANIC_HOOK_GUARD` (test-only shared guard) → validation n/a (no external
    input) · instrumentation n/a (test scaffolding, raises no span) · PII n/a · tests **unit✓** (the two
    guarded tests green under BOTH runners, plus a 12/12 contention stress) · a11y n/a · tokens n/a
  - `conductor-report --test matrix_ledger_gate` (new committed-artifact gate) → validation n/a (reads two
    committed repo artifacts) · instrumentation n/a · PII n/a — it prints ids, never host paths ·
    tests **unit✓** (4 tests, incl. a known-positive control that drives each arm against an input it must
    reject) · a11y n/a · tokens n/a
  - Release artifacts (`target/release/bundle/{msi,nsis}/…`) → validation n/a · instrumentation n/a ·
    PII n/a (both under gitignored `target/`, neither committed) · tests **e2e✓** (the a11y routine arm drove
    the shipped bundle: `[webview2 152.0.4191.66 windows]`, 12 passing / 2 skipped) · a11y **✓** (that same
    arm) · tokens n/a

## Deviations from intent

1. **Leg 3 swapped between runs, operator-directed, after a hard failure.** The plan's third live leg was
   `cross-incident-recurrence` (P-036). Driven live for the **first time in the project's history** it
   hard-failed: `[FAIL]`, exit 1, `state: Fail`, on check index 0 — `kind = "Contains"`, `class = "Hard"`,
   `expected = "Previously seen"`. Not Blocked and not an SLO failure: read-back succeeded
   (`retrieve_telemetry_slice returned keys […]`, "read-back observed the incident corpus") and
   `latency_ms 6151 <= 20000`. The token is inferred — the scenario says so at `:15` while declaring it
   `Hard` at `:12` — and `grep -rn 'Previously seen' crates/` returns **4 hits**, all in
   `crates/conductor-core/src/scenario.rs:1189-1202`, inside a `#[test]` that parses the TOML and asserts the
   DECLARATION. So no PRODUCER exists on the SUT-facing path. **Justification for the swap:** the entry
   expects `exit 0`, so the light gate could not go green and no commit was possible either way; the
   replacement `investigate-actions-functional` (P-072) was chosen on a **prior live green** rather than tier
   arithmetic. The FAIL is retained as committed evidence, not re-rolled away.
2. **Authoring cause of (1), owned.** The legs were selected at P4 on TIER ATTAINABILITY alone — summed
   `gap_ms` against the tier deadline — never on whether their checks had passed live. Leg 1 had a prior live
   green; legs 2 and 3 did not, and leg 2 passing was luck. A leg selection needs a prior live green, or it
   is an experiment.
3. **CARRY C's measurement arm could not run.** Plan step 10 directs re-running the `sr` leg; the `[[gate]]`
   fence carries **no `a11y:sr` entry** (basis: `grep -c 'a11y:sr' plan.md` → 0) and `CONDUCTOR_NVDA` is
   unset. P5 check 4(6) keys on gates an ACCEPTANCE CRITERION names, and the CARRY C criterion names a
   disposition, so nothing fired. Disposition taken: the acceptance's own "explicit routed deferral" arm —
   analysis recorded at `evidence/carry-c-sr-variance.md`, remainder routed.
4. **A discovered SLO tier-corpus finding, recorded not fixed.** 17 of 36 scenarios declare a tier below
   their own summed phase duration (basis: `scratchpad/tier_audit.py`, summed `^gap_ms` against
   5000/20000/90000, validated against 7 live measurements predicting `latency_ms` to ≤151 ms). They split
   three ways: **9** correctly carry `<90s`, the operator-ratified "closest honest bucket" posture recorded
   in `matrix#v2-12`'s notes; **2** are beyond every tier yet declare smaller (`ack-cooldown` 370 s at
   `<20s`, `severity-tier-autonomous` 120 s at `<5s`); **6** could hold a larger tier. Only the last two
   groups are defects. Scenario calibration is not release work → routed to P5.

## Decisions & corrections

- **Operator, P4 review round (3 forks):** version stamp **bumps** 0.1.0 → 0.2.0; the SLO pass drives
  **bounded in-lane legs** (not the composed `run --live` suite, whose auto-resolve leg is recorded not
  run-stable); **CARRY C stays** in this chunk as a bounded investigation.
- **Operator, P5 review (2 corrections to my authored gates):** the three legs' `stop` key held PROSE where
  the key is defined as the terminator RUN — replaced with the runnable
  `pwsh … Get-Process andromeda-pulse-mcp | Stop-Process -Force` form, pulse-app ownership moved to `note`;
  and leg 1 lacked the `sleep 150` quiet window and `timeout` its siblings carried, making it safe only as a
  property of its neighbours — both applied, all three legs now uniform.
- **Operator, post-implement:** swap leg 3 rather than weaken the acceptance (above).
- **Operator correction of my report — a false claim from a truncated probe.** I wrote that
  `grep -rn 'Previously seen' crates/` "returns nothing". It returns **4 hits**. The probe I actually ran was
  `grep -rn 'Previously seen' scenarios/ crates/ | head -5`, whose five slots were entirely consumed by the
  scenario's own comment lines; I then wrote a claim about `crates/` from a clipped view of a *combined*
  search. A verdict read off truncation, which the shell discipline forbids by name. Corrected in the
  evidence with its mechanism, and retracted in the friction ledger.
- **Self-disclosed:** while appending that retraction I hand-wrote its `ts`/`id` (`19:52:10Z`) from memory
  instead of reading `date -u`, which showed `20:27:33Z` — ~35 min early. `ts` and `id` are mutually
  consistent so no contract check catches it; a correcting record was appended rather than leaving a
  fabricated stamp in an append-only ledger.
- **Measured, worth keeping:** the sink `logs/agent-latest.jsonl` genuinely truncates per `--agent-mode`
  invocation — each frozen per-leg capture carries ONLY its own `run_id` (94 / 79 / 99 / 76 lines), so the
  freeze-before-next-leg step is load-bearing, not ceremony.

## Outcome

**Acceptance criteria, re-asserted against the diff:**

| criterion | verdict |
|---|---|
| (tests) nextest workspace green AND `cargo test -p conductor-core` green, no retries/serialization knob | **MET** — 906/906; all 4 core targets; no `retries` key added, no `--test-threads` |
| (obs/security) both `obs.rs` tests still assert one error line + host-path redaction | **MET** — assertions byte-unchanged; only a guard acquisition added |
| (security/arch) bare `cargo audit` + `cargo deny` exit 0 BEFORE the build, `--locked` clean | **MET** — probes ran first, in order |
| (arch) release binary + `custom-protocol` GUI binary after `npm run build` | **MET** |
| (arch) a Tauri 2 bundle produced, installer paths + sizes recorded → `#v2-27` | **MET** |
| (obs) each live leg non-Blocked with `latency_ms <= threshold(slo_tier)`, journal + `runs.db` per scenario → `#v2-27` | **MET** on the three clean legs |
| (tests) the `v2-21` ledger gate passes as SET EQUALITY with a non-vacuity guard → `#v2-21` | **MET** |
| (a11y) `--e2e` graded on its PRINTED verdict, not the exit | **MET** — banner + `Spec Files:` + skip tally |
| (a11y) CARRY C disposition is a cause-level finding or explicit routed deferral, no retry/pin/soft SLO | **MET** — routed deferral, none of the three introduced |
| (security/obs) artifacts carry zero absolute host paths / internal struct names | **MET** |
| (arch) no new crate, port, or `CONDUCTOR_*` handle; no new inbound listener | **MET** — Changes bullets carry the negative facts |

**Gates** (the `[[gate]]` entries by `run`, in order — all 21 green at implement):
advisory-db porcelain `no output` ✓ · `cargo audit` exit 0 ✓ · `cargo deny …` exit 0 ✓ ·
`cargo metadata --locked` exit 0 ✓ · `cargo fmt --all --check` exit 0 ✓ · ensure_frontend exit 0 ✓ ·
`cargo build --release --workspace` exit 0 ✓ · `… -p conductor-tauri --features tauri/custom-protocol` exit 0 ✓ ·
`cargo clippy … -D warnings` exit 0 ✓ (1 fix-loop iteration: `collapsible_if` in the new test, fixed with a
let-chain) · `cargo nextest run --workspace --profile ci` exit 0 ✓ · `cargo test -p conductor-core` exit 0 ✓ ·
`cargo test --workspace --doc` exit 0 ✓ · `command -v cargo-tauri` exit 0 ✓ (baseline RED) ·
`cargo tauri build` exit 0 ✓ (baseline RED) · `cargo test -p conductor-report --test matrix_ledger_gate`
exit 0 ✓ (baseline RED) · `./target/release/conductor preconditions --json` `satisfied:true` ✓ (baseline RED) ·
3 × `leg = 'live'` ✓ (treatment: driven against the operator's live Pulse, still up for this wrap's re-run) ·
`agent-run.sh status <id>` exit 0 ✓ (mint-then-read) · `agent-run.sh run --e2e` printed-verdict ✓.
No `defer` entry; no deferral invoked — every language's gates ran (the chunk has Rust + manifest delta).

**Outcome basis:** implement's P4 report, **plus** two post-implement operator directives that supersede
parts of it — the leg-3 swap (with its re-run, `2026-09-10T20-29-33-466`) and the corrected `Previously seen`
grep. A detector reading this report gets the superseding facts, not implement's originals.

**Process hygiene** (implement P4's census, re-measured here — the host process list is readable):

| process | started by | final state |
|---|---|---|
| `andromeda-pulse-mcp` (sidecar, per leg) | this run's legs | `terminated` — exits with its parent; 0 present at census |
| `msedgedriver` / `tauri-driver` / `node` | the `--e2e` arm | `terminated` — wdio `onComplete` → `tauriDriver.kill()`; 0 present |
| `conductor-tauri.exe` (driven) | the `--e2e` arm | `terminated` — 0 present |
| `msedgewebview2` ×15 | **not this run** | `left running` — all start 21:16:17 (pulse-app's own children) or 08:02:05 (~13 h pre-session); none during the legs |
| `pulse-app` PID 29608 | **the operator** | `left running: the operator's SUT, held up deliberately through this wrap's light gate — the operator stops it` |
| loopback `4444`/`4445` | the driver stack | no LISTENING socket at census |
