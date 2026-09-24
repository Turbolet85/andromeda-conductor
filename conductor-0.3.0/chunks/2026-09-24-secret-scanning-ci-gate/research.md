# Codebase Research — 2026-09-24-secret-scanning-ci-gate

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 14 · **Probes run:** 4 (write-guard smoke · secret-pattern baseline · pattern positive control · ci.yml expression/env map)
- **Harness rules consulted:** none — no live leg in this chunk (no Pulse, no webview leg is driven; the only probe the plan names is the write-guard smoke, whose firing form is setup `validation.md` §Hook smoke test step 6, `:36-41`, read in full)
- **Platform issues consulted:** the `GITHUB_ENV` → `env`-context question (scope B.8) against GitHub's own docs, fetched 2026-09-24 — (1) `docs.github.com/en/actions/reference/workflows-and-actions/contexts`: "The `env` context contains variables that have been set in a workflow, job, or step. It does not contain variables inherited by the runner process." — does not address `GITHUB_ENV`; (2) `…/workflows-and-actions/workflow-commands`: "The step that creates or updates the environment variable does not have access to the new value, but all subsequent steps in a job will have access." — its example reads the key as a SHELL variable only, and the page does not say whether `${{ env.NAME }}` resolves it; (3) `…/how-tos/write-workflows/choose-what-workflows-do/use-variables`: "In most cases you can also use contexts … to access the same value." — does not address the combination. **Verdict: the docs do not settle it; the question stays open for a CI measurement.**
- **Graph:** not consulted — zero queries ran. The two new `.rs` files are test targets on the rust plane, but they call no workspace symbol and change no existing one; the rest of the modify-set (`ci.yml`, `.gitignore`, `.claude/settings.json`, the Cargo manifests) sits outside every indexed plane. Both planes are built and current, so this is a judgment that no query had a subject, not a degradation (P5 check 7 WARNs on it by design).

## Files inspected
- `.github/workflows/ci.yml` (`:1-206` full; `:238-367`; step/env index over all 745 lines) — three jobs: `rust` (windows-latest), `frontend` (windows-latest), `a11y` (windows-2022). The static gates sit in `rust`: `Coverage-completeness gate` (`:87-92`) and `Scenario-assertion audit gate` (`:99-103`), each `shell: bash`, a presence guard emitting `::error::` + `exit 1`, then `cargo nextest run -p {crate} --test {target} --profile ci`, no `continue-on-error`, no `if:`. The flakiness budget (`:61-63` comment, `:64-71` step) is an inline bash `grep` assertion with a `::error file=…::` annotation.
- `.gitignore` (full, 50 lines) — the secrets block (`:25-27`) holds `.env` + `.env.*` only; no `*.pem` / `*.p12` / `*.cer` (re-derived: `grep -nE 'pem|p12|cer|key|\.env' .gitignore` → `:26`, `:27`).
- `.gitattributes` (full) — `* text=auto eol=lf` repo-wide, so tracked text is LF on every host; line numbers a scanner reports agree dev-host ↔ CI.
- `.claude/settings.json` (`:1-40`) — `:14` PreToolUse write guard: `… | jq -r ".tool_input.file_path // empty" | tr -d "\r")`, then `grep -qE "(^|/)(dist|build|[.]next|node_modules|coverage|target|vendor)/"` — no separator normalisation.
- `andromeda-setup-project/references/hooks-matrix.md` (`:80-95`) — the fixed template at `:89` adds `| tr "\\134" "/"` inside `path=$(…)` after `tr -d "\\r"`.
- `andromeda-setup-project/references/validation.md` (`:22-43`) — §Hook smoke test step 6: the command piped AS STORED, printf OCTAL payloads, the backslash arms the discriminating ones.
- `crates/conductor-core/tests/scenario_audit_gate.rs` (`:1-80` of 240) — the static-gate test shape: `repo_root()` from `CARGO_MANIFEST_DIR` + `../..`; a POSITIVE arm over the real committed subject with vacuity guards; NEGATIVE arms mutating the real subject and asserting the message NAMES the offender; CONTROL arms.
- `crates/conductor-core/Cargo.toml` — deps: serde/serde_json/toml/thiserror/garde/tracing; dev-deps: tokio/rstest/proptest/insta. No `regex`.
- `.andromeda/security-plan.md` `:224`, `:242-262` (§Secret Management), `:352`, `:356`, `:387-400` (Decisions Log, open question 3).
- `.andromeda/architecture.md` `:60` (§Established Decisions [CI/CD]).
- `scripts/agent-run.sh` `:290-316` — `run` = `nextest run --workspace --profile ci` (`:314`) + doctest + clippy; `--unit` / `--integration` the same nextest over the workspace.
- `.config/nextest.toml` (full) — `retries = 0` in both profiles (the flakiness step's subject).

## Graph impact
- graph not applicable (no symbol in the modify-set). Companion sweep for the one artifact whose NAME a gate might pin: `grep -rn 'settings.json' crates/ scripts/` is not needed — `settings.json` is host tooling that no crate or harness script reads (re-derived: `grep -rln 'settings\.json' crates/ scripts/` → 0 files).

## Measurements (all at `4837210`, derivation beside each)
1. **Write-guard mechanism — RE-DERIVED, holds.** `cmd=$(jq -r '.hooks.PreToolUse[0].hooks[0].command' .claude/settings.json)` then each payload `| bash -c "$cmd"`: `src/x.rs` → exit 0 · `target/x.rs` → exit 2 (`Blocked: edit to a generated directory (target/x.rs)`) · `target\134\134x.rs` → **exit 0** · `C:\134\134p\134\134target\134\134x.rs` → **exit 0**. The first backslash payload decodes (jq) to `target\x.rs`. So the guard is blind to exactly the separator Write/Edit send on this host; the directive's 11-path probe is not re-run, its two discriminating arms are.
2. **Fix regression risk — none on tracked content.** `git ls-files | grep -cE '(^|/)(dist|build|[.]next|node_modules|coverage|target|vendor)/'` → 0 of 3 674 tracked files; the project root (`D:\dev\projects\conductor`) has no guarded ancestor name.
3. **Secret-pattern baseline — 0 hits, and the 0 is real.** Scratchpad probe `secret_baseline.py` over `git ls-files -z` (3 674 tracked; 3 670 text scanned, 4 binary skipped by a NUL sniff; 39.9 MB): 14 content patterns (private-key block · AWS access key id · GitHub classic + fine-grained tokens · Slack token + webhook · Stripe live key · Google API key · JWT · Anthropic key · OpenAI key · npm token · URL-embedded credentials · generic `password|secret|api_key|…` quoted assignment ≥ 8 chars) + a secret FILE-NAME class (`.env*`, `id_rsa`-family, `*.pem|p12|pfx|key|cer|crt|jks|keystore`) → **0 hits on all 15**. Positive control: each of the 14 content patterns FIRES on a synthetic sample built at runtime by concatenation (so no sample exists in any file). The scope's expected false-positive load (hex fingerprints, run ids, shas, `<redacted>` placeholders) does NOT materialise for this pattern set — none of those shapes carries a vendor prefix or a key-assignment context.
4. **Workflow expression surface.** `grep -noE '\$\{\{[^}]*\}\}' ci.yml` → `:12 ${{ github.ref }}` · `:354 ${{ env.* }}` (inside the comment block `:351-356`) · `:723 ${{ runner.temp }}` — so 0 live `env`-context reads (the CARRY's count re-derived: holds). `grep -nE '^\s*if:'` → 11 lines, all `if: always()`. `env:` blocks: step-level only, keys `ANDROMEDA_PULSE_DATA_DIR`, `CONDUCTOR_RUNS_DIR` (×3), `CONDUCTOR_A11Y_STRICT`, `CONDUCTOR_ENV` (×6); no workflow- or job-level `env:`. One `GITHUB_ENV` writer: `:342` `Add-Content -Path $env:GITHUB_ENV -Value "EDGEWEBDRIVER=$dir"`, read downstream in the step SHELL (`:351-356` comment).
5. **Dependency facts.** `regex` is in `Cargo.lock` at `1.12.4` (transitive, via `urlpattern` ← tauri-utils), not a `[workspace.dependencies]` entry; a dev-dependency on it adds an EDGE, not a package (the 2026-09-02 lockfile-edges learning: state the basis as package count).
6. **Spawn precedent.** `grep -rnE 'Command::new\(' crates/*/tests/` → 3 sites, all in-tree binaries via `env!("CARGO_BIN_EXE_*")`. A gate enumerating the tracked tree with `git ls-files` would be the first test binary spawning a PATH-resolved external program — a fixed program name with fixed argv and no operator-supplied value, which the security extract's rule-(b) reading places outside the governed forms.

## Patterns detected
- **Static gate over committed data** (`ci.yml:87-103`; `scenario_audit_gate.rs:1-80`): the Rust test IS the gate; the CI step is its presence-guarded `nextest --test` invocation; the same target runs locally inside `agent-run run` (`agent-run.sh:314`) with no sixth command.
- **Inline shell assertion** (`ci.yml:64-71`): used where no production source derives the subject — the flakiness budget has no Rust owner, so it is a `grep` with a `::error file=…::` annotation. It has no in-suite negative arm.
- **Handle-name-only diagnostics** (`ci.yml:300`, `:311`, `:321-338`): `[precondition] NAME: state`, never a value or a path.

## Conventions to follow
- **No `continue-on-error` / `if:` on an asserting step; presence guard first** — arch `:60` [CI/CD].
- **Paths resolve from `CARGO_MANIFEST_DIR`** — `scenario_audit_gate.rs:12-13, 22-24`.
- **Negative arms name the offender** — `scenario_audit_gate.rs:60-80`.
- **Refer to CI steps by NAME in prose, never by line** — a11y/tests extracts; `ci.yml` line numbers move with this chunk's own edits.

## New files to create
- A secret-scan gate test target (form per P4's fork — recommended `crates/conductor-core/tests/secret_scan_gate.rs`).
- A workflow env-context gate test target (form per P4 — recommended `crates/conductor-core/tests/workflow_env_gate.rs`), or an inline step per the CARRY's literal shape.

## Files to modify
- `.github/workflows/ci.yml` — two new named gate steps in the `rust` job (placement per P4: early, beside the flakiness budget, so a hit is reported before the ~18 min test path).
- `.gitignore` — the key/cert entries under the existing secrets block.
- `.claude/settings.json` — `:14` gains `| tr "\\134" "/"` (Edit tool only).
- `crates/conductor-core/Cargo.toml` + root `Cargo.toml` `[workspace.dependencies]` — IF the plan takes `regex` as a dev-dependency (package count unchanged; one new edge).
- `Cargo.lock` — the edge line, if so.

## Open questions
- Scanner FORM / tool selection — security-plan `:259` + Decisions Log open question (3) defer it to the operator → blocks: plan-decision (P4 asks).
- `GITHUB_ENV` → `${{ env.* }}` readability is undocumented and unmeasured → blocks: plan-decision (how the env-key gate treats a `GITHUB_ENV`-written key, and where the measurement comes from).
