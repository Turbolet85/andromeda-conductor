# Codebase Research — 2026-06-27-ci-quality-gate-config

## Scope
- **Depth:** moderate · **Reads:** 2 full (ci.yml, nextest.toml) + test-plan §9/§10 · **Globs/Greps:** 3
- **Key finding:** the scaffold is already in place — this chunk *promotes* an explicitly-marked "measure only" hook into a gating + artifact-upload step. Small, surgical, CI-config-only.

## Files inspected
- `.github/workflows/ci.yml` (full, 99 lines) — two jobs (`rust` + `frontend`), both `windows-latest`. The `rust` job already: installs `cargo-llvm-cov` (line 33), dogfoods `agent-run.ps1 run` for test+lint (lines 57–61, emits `target/nextest/ci/junit.xml`), runs `cargo audit` + `cargo deny check` (63–67), and ends with **`Coverage (measure only)`** — `cargo llvm-cov nextest --workspace --profile ci --summary-only` (70–71) carrying the literal comment *"the coverage threshold + JUnit/coverage artifact upload are Epoch 10."* → **this chunk's hook.** `permissions: contents: read` (line 8–9). No artifact uploads anywhere yet.
- `.config/nextest.toml` (full, 19 lines) — `[profile.ci] retries = 0` (line 8) + **`[profile.ci.junit] path = "junit.xml"`** (11–13, → `target/nextest/ci/junit.xml`, comment: "consumed by the Epoch-8/10 CI gates") + `[profile.default] retries = 0` (17). **JUnit emission + zero-retry are already configured** — not work to add.
- `scripts/agent-run.{sh,ps1}` (grep) — the `run` verb runs `cargo nextest run --workspace --profile ci` (sh:64 / ps1:67) which emits the JUnit; `ensure_frontend` couples it to a prior `npm run build`. **Coverage is NOT in the harness** (CI-only) — so the harness `run` verb is untouched and stays green.
- `.andromeda/test-plan.md` §9 (CI Integration) + §10 (Quality Gates) — the authority. §9 line 213 `quality-gate-config-emit` is verbatim this chunk: "emit GitHub Actions config enforcing coverage threshold (§10), zero-retry flakiness budget, cargo audit/deny, committed Cargo.lock, toolchain ≥1.94.1, tauri ≥2.10.3."

## Graph impact
- **N/A — config-only chunk.** Modifies `ci.yml` (YAML) + possibly `nextest.toml` (TOML); zero Rust symbols, zero crate edges, zero `#[tauri::command]`/seam surface. The code-graph query (symbols · callers · crate_edges) has no subject here, so it is skipped by design (not a cold-start/empty-DB case). No engine/seam impact to trace.

## Patterns detected
- **The Epoch-10 hook is pre-marked** (`ci.yml:69–71`): the `--summary-only` coverage step is the exact line to promote into a `--fail-under-lines` gate + file emission + upload.
- **JUnit already flows** (`nextest.toml:11–13` → `target/nextest/ci/junit.xml`): every `--profile ci` run (the `agent-run` dogfood step *and* the llvm-cov step) writes it; the chunk only needs to *upload* it.
- **Flakiness budget already pinned** (`nextest.toml:8,17`, `retries = 0` both profiles): CI runs `--profile ci`, so zero-retry is already enforced; the extracts ask only for an explicit CI-level *assertion* of it ("do not rely on the local file alone").
- **Supply-chain gates are discrete hard-fail steps** (`ci.yml:63–67`): `cargo audit` + `cargo deny check` — must stay green + untouched (security extract: never downgrade/suppress).
- **Action versions are pinned** (`@v4`/`@v2`) and **cargo subcommands come prebuilt** via `taiki-e/install-action` (`ci.yml:30–33`) — `cargo-llvm-cov` is already in that list.

## Conventions to follow
- Each CI step carries a **rationale comment** (`ci.yml` house style) tying it to a plan §/invariant; mirror this on the new gate + upload steps.
- **`windows-latest` single-OS** (matches the dev host + every current job); do not expand the runner matrix in this chunk.
- **Pinned `actions/*@vN`**: use `actions/upload-artifact@v4` (and, only if chosen, `dorny/test-reporter@v1`).
- Coverage exclusions are expressed as `cargo llvm-cov --ignore-filename-regex <re>` (test-plan §10: exclude tonic/prost codegen + rmcp/tauri stubs + rstest/insta fixtures); the binding gate is **line ≥ 60%** (`--fail-under-lines 60`), branch is informational/nightly-only.

## New files to create
- none (edits only).

## Files to modify
- `.github/workflows/ci.yml` — **(primary)** promote `Coverage (measure only)`:
  - emit report files (`--lcov --output-path lcov.info` + `--cobertura coverage.xml`) with the `--ignore-filename-regex` exclusions, and **enforce `--fail-under-lines 60`** (hard-fail under threshold);
  - add `actions/upload-artifact@v4` step(s) uploading the coverage report(s) + `target/nextest/ci/junit.xml` (always-run so failures still publish);
  - add a lightweight **zero-retry assertion** step (flakiness budget — confirm `.config/nextest.toml` `ci` profile is `retries = 0` at CI level);
  - *(P5 decision)* optionally add `dorny/test-reporter` for PR annotations + the `checks: write` permission.
- `.config/nextest.toml` — likely **no functional change** (JUnit already configured); at most a provenance-comment update pointing `[profile.ci.junit]` at this chunk. Confirm at implement.

## Open questions (surface at P5 review)
1. **JUnit consumption** — artifact-upload only (lean: solo dev on a long-lived `build/` branch with no PR-review surface to annotate; junit/coverage are read via the CI artifact API / `agent-run logs`; avoids elevating `permissions` to `checks: write`) **vs.** also wiring `dorny/test-reporter` (test-plan §9's PR-annotation vision). Recommend artifact-only; surface so the user can opt in.
2. **OS matrix** — out-of-scope/deferred: keep `windows-latest` single-OS. The chunk's four deliverables are coverage/flakiness/JUnit/artifact, not cross-platform expansion; test-plan §9's ubuntu/windows/macos matrix + the ubuntu+xvfb webview-E2E job are a separate concern (and the webview leg is display-gated). Flag so the user may pull it in.
3. **Flakiness-budget mechanism** — add the lightweight CI assertion (per extracts' "CI-level assertion, not the local `nextest.toml` alone"); decided default, minor.
