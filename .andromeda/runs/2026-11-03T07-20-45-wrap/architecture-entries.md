## 2026-11-03-linux-only-base-ci — tokio 1.54.1 at the runtime's three sites
**Section:** §Stack and Technologies (Async runtime row) · §Established Decisions ([Language / Runtime]) · §Inherited Defaults (Language / runtime)
**Change:**
- The Async runtime row reads tokio 1.54.1 (`current_thread` flavor) (was tokio 1.52.3); its role text unchanged.
- The [Language / Runtime] decision reads Rust 2024 + tokio 1.54.1 (was tokio 1.52.3); its rationale unchanged.
- Inherited Defaults' Language / runtime reads Rust 2024 (cargo 1.85, MSRV 1.94.1) + tokio 1.54.1 `current_thread` (was tokio 1.52.3).
**Why:** the lock moved tokio 1.52.3 → 1.54.1, lock-only — the workspace pin `tokio = "1.48"` admits it and no other package moved — closing RUSTSEC-2026-0386 (`unsound`, `AsyncFd::new`, dated 2026-10-13), external decay on an unchanged tree that turned `cargo deny` red on the chunk's first Linux CI run; the Linux CI run of the bumped HEAD is green. The bump was taken inside the chunk on the overseer's word (founder-delegated, PROVISIONAL). Standing: `cargo audit` green is not `cargo deny` green — audit passed the advisory as an allowed warning while deny refused it.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — the blake3 SUT pin cited by its text
**Section:** §Stack and Technologies (Hashing / digest row)
**Change:**
- The row's version-match citation names the SUT's pin by its text — `blake3 = "1"` in `andromeda-pulse`'s `Cargo.toml` — (was a line-numbered citation into that file, line 154); the rest of the row unchanged.
**Why:** the citation sweep read the cited line stale: line 154 of the SUT's `Cargo.toml` is blank and the `blake3 = "1"` pin stands one line above, in another repository's file this tree's citations cannot re-point into; the claim itself holds. A digit-only re-point is the cascade sweep's, never an in-line edit, so the citation now names the pin rather than its line.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — agent-run.sh the one harness script
**Section:** §Stack and Technologies (Operator instruments row) · §Established Decisions ([ORM]) · §Occupied Resources (`runs/live-suite/{leg}.jsonl`; `CONDUCTOR_AGENT_MODE`, `CONDUCTOR_PREFLIGHT_TIMEOUT`, `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`, `RUST_LOG`) · §Infrastructure Patterns → Directory structure (the `mutation-gate.py` tree comment)
**Change:**
- Each standing harness reference names `scripts/agent-run.sh` alone (was `scripts/agent-run.{sh,ps1}` / "the harness shells"): the Operator instruments row (none adds a sixth `agent-run.sh` command; the 5-command surface unchanged); the [ORM] decision (the `conductor cleanup <run_id>` verb the only caller, and `scripts/agent-run.sh` issues no SQL of its own); the `runs/live-suite` owner; `CONDUCTOR_AGENT_MODE`'s exporter; `CONDUCTOR_PREFLIGHT_TIMEOUT`'s `boot` reader; the `run --live` banner that registers `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`.
- `CONDUCTOR_PREFLIGHT_TIMEOUT`'s paid timeout reads ~47s wall-clock as measured at its 2026-09-04 report (was "in both shipped shells").
- `RUST_LOG`: the `run --live real-model` arm of `scripts/agent-run.sh` removes it for the capture's two test invocations (was "both harness shells"); the clause on the `.ps1` reading and restoring the caller's value in a `finally` is gone.
- The `mutation-gate.py` tree comment reads not wired into `agent-run.sh` (was "wired into neither harness shell").
**Why:** the PowerShell twin `scripts/agent-run.ps1` retired, with `Test-RunId`, its `$LASTEXITCODE` checks and its `RUST_LOG` save/restore: harness scripts 2 → 1. `scripts/agent-run.sh` stays the one harness, byte-unchanged — the five verbs, `valid_run_id` ahead of `conductor cleanup`, the closed `--live [real-model]` refusal.
**Kept:** the dated 2026-09-04 measured two-shell `boot` runs stand as history; the Operator instruments row's `arch-registry-check.py` clause stays, its retirement a separate entry; `runs/live-suite`'s owner narrows within its registered class, no second writer.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — the one CI job on ubuntu-latest
**Section:** §Established Decisions ([CI/CD]) · §Infrastructure Patterns → Build system · §Infrastructure Patterns → CI/CD approach
**Change:**
- [CI/CD]: `cargo build` / nextest / clippy run on the job's `ubuntu-latest` runner (was "on the dev OS target"); the gate set, the fmt gate at step index 2 and the one-job shape unchanged.
- CI/CD approach: GitHub Actions on an `ubuntu-latest` runner runs one job, `rust` (was "on the dev OS target").
- Build system: `core.autocrlf=true` is the norm on a Windows checkout, while the one CI job, `rust`, runs on an `ubuntu-latest` runner (was the norm "on the Windows dev host" with `rust` on a `windows-latest` runner); the LF pin's rationale unchanged.
**Why:** the Windows CI runner retired: the `rust` job moved from `windows-latest` to `ubuntu-latest` with no step added or removed and no gate weakened, and the whole gate set ran green on the Linux leg. Trap for later chunks: no CI leg now witnesses `#[cfg(windows)]` code — `spawn.rs`'s Windows arms are compiled out on every leg.
**Kept:** the build-system key's parenthetical on the retired `a11y` job's pinned `windows-2022` image stays as history; the §Stack CI/CD row and §Inherited Defaults (CI/CD) name no OS and stand unchanged; `GHA_ENV_CONTEXT_PROBE` carries no date and no OS, so nothing was re-dated.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — capture evidence committed under crates/conductor-run/tests/evidence
**Section:** §Standard Contracts (Readiness gate: the corpus-access exception)
**Change:**
- The one ratified, scoped exception to never persisting `corpus.db` data reads scrubbed real-model capture evidence committed under `crates/conductor-run/tests/evidence/{marker}/` (was "in a chunk's `evidence/` tree"); the 2026-09-29 breach and its 2026-09-30 / 2026-10-02 remedy record stand as written.
**Why:** the capture evidence moved byte-equal — 46 files in 11 per-source-chunk folders, sha256 pins and the 26-capture walk unchanged — out of the `conductor-0.3.0/chunks/` tree, which left git. The founder ratified the move by his own live word, by dialog in pc-overseer's session, relayed verbatim: the Data Protection exception extends to the new locus, the scrub, pins and no-capture-text arm unchanged. The overseer (founder-delegated) confirmed this restatement lies inside that ratification, PROVISIONAL as a confirm.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/
