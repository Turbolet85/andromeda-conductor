# Report — 2026-09-04-sidecar-spawn-without-a-console-window

**Chunk:** Sidecar spawn without a console window — the GUI-launched MCP sidecar stops raising a foreground terminal pane that steals focus and publishes its absolute exe path
**Date:** 2026-09-04
**Commits:** (none yet — this wrap authors the chunk commit)

## Changes (structured — detectors read this)

- **Files:** `rustfmt.toml` (new) · `crates/conductor-verify/src/spawn.rs` · `crates/conductor-tauri/src/commands.rs` · `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` · `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` · `conductor-0.2.0/chunks/2026-09-04-sidecar-spawn-without-a-console-window/{scope,research,plan,report}.md` + `evidence/nvda-pass.json`

- **Symbols / APIs:**
  - NEW (crate-private, `conductor-verify`): `const CREATE_NO_WINDOW: u32 = 0x0800_0000` and `fn console_suppressing_flags() -> u32`, both `#[cfg(windows)]`.
  - CHANGED (behaviour, signature unchanged): `spawn::build_command(&Path) -> tokio::process::Command` now calls `.creation_flags(console_suppressing_flags())` under `#[cfg(windows)]`. Its signature is byte-unchanged, so **no caller threading was owed**. It keeps BOTH callers: `ReadbackClient::connect @ crates/conductor-verify/src/client.rs:48` (the sole production caller) and its own unit test `@ spawn.rs:306` (code-graph rust plane, rows=2; graph reports these 0-indexed as 47/305).
  - REMOVED (harness only, `conductor-tauri/ui/test`): `async function reactivateWindow()` and its sole call site — step 9 firing on step 8's measurement. `bringToForeground()` is UNCHANGED and keeps all three call sites (`:335`, `:546`, `:593`).
  - No public API of any shipped crate changed. No IPC method, endpoint, export, port or socket added or changed. No env var added or changed.

- **Crates / modules:** none added, removed, or restructured. The change is contained in `conductor-verify`; `conductor-tauri`'s only Rust delta is doc comments + formatting.

- **Dependencies:** **none added, none bumped.** `Cargo.toml` and `Cargo.lock` are byte-untouched by this chunk (`git status` clean on both); package count 564, unchanged. `creation_flags` is reached through the existing `tokio` dependency (`cfg_windows!`, `tokio-1.52.3/src/process/mod.rs:669+`, fn at `:675`) — no `windows-sys` was introduced.

- **Schema / config:** NEW workspace-root `rustfmt.toml` carrying one key, `edition = "2024"` (CARRY 2's instance remedy; the seeded PostToolUse hook row was NOT edited). No migrations, no config keys, no violation-schema or scrub/redaction-shape change.

- **Spec-master edits:** none by /implement. This wrap's P2 owns them.

- **Counts / qualifiers moved:**
  - **test-plan §12 mutation-survivor roster: 3 members → 4.** `conductor-verify` gains an accepted-deliberate **PAIR** (`spawn.rs:116:5` `sidecar_resolves_on_path -> true` and `-> false`), from that file's FIRST dispositioned score. Existing three members (§12 `:608`/`:609`/`:610`) are unchanged.
  - `conductor-verify` nextest count 96 → **112**; workspace nextest 852 → **853** (one new `#[cfg(windows)]` test).
  - a11y SR record: rows stay 51; outcomes move `34 announced-as-expected / 0 not-announced` → **`32 / 2`**; findings **15 → 16**.
  - Everything else: none — verified.

- **Dev-tool versions:** none. (Observed-not-installed: the WebView2 runtime the driven arm reports moved 152.0.4191.53 → **152.0.4191.62** between chunks — a host runtime update, not a tool this chunk installed.)

- **Harness / gate surface:**
  - `screen-reader.e2e.ts`: R0-01's action drops BOTH `browser.refresh()` and the focus warm-up `tab()`; the `@reactivate` timeline stamp is gone with `reactivateWindow()`.
  - `nvda-pass-spec.md` row R0-01: action text and `expected` re-worded — the row now grades the ORIGINAL load's alert, and **silence is defined as a FINDING**, never a pass and never a harness failure.
  - No `agent-run` script, xtask verb, CI step, or status/verdict shape changed.

- **Cross-project / external claims:** the live legs ran against a real `andromeda-pulse` (operator-launched 2026-09-04T19:34Z, det-L4 + MCP, data dir `…/pulse-legs/a11y-20260904-213403`). Basis read: Pulse's own corpus reachability via `boot`'s `ReadyState` (`ready:true`, `canary_round_trip:"ok"`) and NVDA 2026.2's speech log. No claim is made about Pulse's source.

- **Reverted / negative API facts:** none.

- **Insufficient fixes (written, kept, not the remedy):**
  - **The four `role="alert"` regions mounting empty at first paint** (shipped by the prior chunk, kept, correct) does NOT make the load-time alert audible. Defect: a first-load scenario-load error is never announced. What the change DID resolve: the region no longer mounts together with its text, so a *later* arrival announces. Remainder owner: an SR follow-up — the fix needs a re-announce mechanism, not another mount tweak.

- **Spec claims disproved by measurement:**
  1. **`spawn.rs:79`'s "The caller hands the result to `TokioChildProcess`"** — that type has been absent from the codebase since the 2026-06-27 rmcp removal; a grep across every crate `src/` found that doc comment as its only occurrence. **CORRECTED in this chunk.** The same retired name is restated in `architecture.md` §Occupied Resources — Service/process names and §Cross-cutting Patterns — Trust boundary, which are P2's to disposition.
  2. **security-plan §Anti-Patterns (a) and obs-plan §11 (×2) all say the `CREATE_NO_WINDOW`-class fix is "route-owned, not yet shipped"** — measured false as of this chunk: it is shipped and live-verified.
  3. **security-plan §Anti-Patterns (a)'s rationale that `sidecar_resolves_on_path` "constructs no `Command`, therefore opens no console window"** is now argued from a premise that no longer holds (the real spawn also opens none). The in-code twin at `spawn.rs:92` was corrected here; the master clause is P2's.
  4. **`.claude/rules/verification-harness.md` entry 59 item (4)** — "the sidecar's console pane did [take the foreground], ~200 ms after `Start` … re-activate after any action that spawns a process" — measured false at HEAD: no pane, and re-activating is now actively harmful. NOT a spec master; curation's to correct.
  5. **`a11y-plan.md:269` cites the NVDA pass spec at `test/a11y/screen-reader/nvda-pass-spec.md`**; it lives at `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md`. Found during phase attribution; P2's to disposition.
  6. **`crates/conductor-verify` does not compile standalone** — `tokio::time::sleep` at `preflight.rs:336` with `time` present only in `[dev-dependencies]` (`Cargo.toml:31`) and not `[dependencies]` (`:10`). Red under `cargo check -p conductor-verify --lib`, hidden by workspace feature unification. **PRE-EXISTING at HEAD** (`git show HEAD` confirms both the call site and the identical feature list); out of this chunk's scope.

- **Expected amendments (from plan):**
  - security-plan §Anti-Patterns (a) "not yet shipped" → **carried**, fact in *Spec claims disproved* #2.
  - obs-plan §11 PII Scrubbing + §11 Project-specific bans → **carried**, same fact (#2), both sites.
  - architecture §Occupied Resources — Service/process names + §Cross-cutting Patterns — Trust boundary (`TokioChildProcess`) → **carried**, fact in #1.
  - architecture §Infrastructure Patterns — Directory structure (add `rustfmt.toml`) → **carried**, fact in *Schema / config*.
  - a11y-plan §Pass spec format (`:269`) path → **carried**, fact in #5.
  - test-plan §6 / a11y-plan §3 re-activation registration → **carried and LIVE** (conditional on step 9, which fired), fact in *Symbols / APIs* (removed) and *Harness / gate surface*.
  - **NOT in the plan's list, added here:** test-plan §12 mutation roster gains a fourth member — fact in *Counts / qualifiers moved*.

- **Coverage of new surfaces:**
  - `spawn::console_suppressing_flags()` (crate-private pure accessor) → validation n/a (no external input) · instrumentation n/a (no new span; the bounded span-name set is untouched) · PII n/a (a `u32` literal; the resolved exe path is never returned, logged or rendered) · tests **unit ✓** (`spawn.rs::the_sidecar_spawn_suppresses_the_child_console`, `#[cfg(windows)]`) · a11y n/a · tokens n/a
  - `spawn::build_command`'s creation-flag application → validation n/a · instrumentation n/a · PII **✓ improved** (closes the console-pane host-path channel) · tests **unrunnable-here at unit tier** — `std::process::Command` exposes no creation-flags getter, so the APPLIED flag is not assertable through `as_std()`; recorded as a test-plan §1 untestable zone and measured instead by the live SR leg · a11y n/a · tokens n/a
  - `rustfmt.toml` (build config) → all flags n/a; effect visible as one-off edition-2024 reformatting of the touched `.rs` files.

## Deviations from intent

1. **Ran `a11y:sr-empty` and `a11y:sr-error` in addition to the listed `a11y:sr`.** *Justification:* plan step 7 changed row R0-01, which lives in the **`sr-error`** subject, but `## Test Commands` listed only `a11y:sr` — so the plan's own CARRY-4 acceptance criterion had no producer among its commands. A plan defect, not a scope widening; without it that criterion would have been reported met unproven.
2. **Removed R0-01's focus warm-up `tab()` in addition to the reload.** *Justification:* a follow-on harness fix. The warm-up existed only to give NVDA a focus event *before the reload*; with the reload gone it merely consumed R0-02's first Tab landing, which the reload had been silently undoing by resetting Chromium's sequential-focus start point. Measured: R0-02 `announced-differently` → `announced-as-expected`.
3. **`reactivateWindow()` retired.** Not a deviation — plan step 9 firing on step 8's measurement, exactly as the conditional specified.
4. **Ran the sr suites four times total** (one failed leg, one exploratory, two full passes). *Justification:* establishing that an S0-01/S0-02 flip was NVDA binding variance and not a regression required a second observation; a single run cannot distinguish them.

## Decisions & corrections

- **Overseer, at plan review (4 corrections, all applied):** redirect probe output into `target/` (a repo-root `audit.txt` is NOT gitignored and `git add -A` would commit it); give `cargo mutants` a fresh `--output target/…` and read the NESTED `mutants.out/` (the repo-root one is stale by construction); state the env block ONCE before both live steps, because a bare `boot` short-circuits at its `preconditions` arm with exit 1 and emits no `ReadyState`; and use the pinned firing form `cargo run -q -p conductor-cli --bin conductor -- preconditions` (`conductor` is not on `PATH`).
- **Overseer correction of fact:** `pulse-app` PID 17404 was stopped after `ca94697`; the handoff's "still running" line was stale. A fresh instance (PID 62628) was launched for this chunk.
- **Citation-base correction (mine, caught by the overseer's independent grep):** code-graph line numbers are **0-indexed** and grep/editors **1-indexed**. Plan/research/scope had cited graph values raw as file lines; all three artifacts were normalised to 1-indexed with the graph value in parentheses, each confirmed by grep rather than converted arithmetically.
- **Decision — mutation roster not shrunk by a host-dependent test.** An agreement test (`sidecar_resolves_on_path()` vs `resolves_on(<real env>)`) would kill exactly one of the two survivors, and *which* one depends on whether the sidecar happens to be on `PATH`. Rejected: a roster is an enumerated set that must mean the same thing everywhere, so a host-dependent kill is worse than an honest acceptance.
- **Decision — evidence read, never counted.** Every hygiene pattern hit was inspected: the lone `terminal|pane|.exe` hit was `.exe` inside NVDA's own `execute` debug line, and the three drive-letter hits were NVDA's startup config paths — **none inside a `Speaking [...]` line**, so none was spoken.

## Outcome

**Acceptance criteria, re-asserted against the diff:**

| Criterion | Verdict |
|---|---|
| (security) spawn sets a Windows console-suppressing flag; no pane titled with the absolute exe path | **MET, live.** S1-01's `heard` lost `<host-path>`, `<host-path> terminal blank` and `pane`; committed evidence has **0 `security_finding` rows and 0 `<host-path>` placeholders** where the prior record had both |
| (security) rule (a)'s other controls unchanged | **MET.** Diff shows only an added builder call: fixed program NAME, `.env(...)`-only data dir, injection-reject all byte-unchanged; `spawn.rs::build_command_targets_…` still asserts all three, plus a new "nothing operator-supplied reaches argv" assertion |
| (arch) contained in `conductor-verify`, dep edges unchanged, stdio still the JSON-RPC transport | **MET.** No manifest/lockfile delta; `boot` reached `ready:true` over the child's piped stdio |
| (arch) no new port/crate/env handle; `cfg`-gated so non-Windows still compiles | **MET by construction, NOT gate-verified** — both CI jobs run `windows-latest`, so no non-Windows target is compiled anywhere. Stated as a review obligation, per the plan |
| (tests) both runners green with the extended spawn test | **MET.** nextest `-p conductor-verify` 112; `cargo test -p` exit 0 |
| (tests) workspace nextest + doctest + clippy | **MET.** 853 passed · doctest 0 · clippy 0 · `typecheck:e2e` 0 |
| (tests) scoped mutation audit, every survivor killed or accepted | **MET.** 16 mutants: 13 caught, 1 unviable, 2 accepted-deliberate. Both `console_suppressing_flags` mutants **caught** |
| (security) deny 0, audit exit captured directly, count basis | **MET.** audit exit **1**, first diagnostic byte-matches the pinned signature (**51st re-pin**); deny exit 0; 564 packages, lockfile untouched |
| (a11y) S1-01 window free of the foreign-window observable | **MET** — the chunk's primary measurement |
| (a11y) `reactivateWindow()` retires only on a measured no-handoff result | **MET.** Retired on measurement; `bringToForeground()` untouched |
| (a11y/security) evidence carries no host path or seam-crate struct name | **MET.** 0 word-anchored drive letters, 0 `%APPDATA%`/`/Users`/`/home`/`.cargo`/`.rustup`, 0 seam-crate struct names |
| (a11y) `sr` leg keeps its `CONDUCTOR_NVDA` guard, stays non-CI, no second stack | **MET.** No harness stack added |
| (obs) no host path in self-obs/run-report; only existing span names + allowlisted fields | **MET.** `boot` printed `data_dir:"<redacted>"`; the fix adds no span, no log line, no field |
| (design/layouts) no foreign OS window over the console; no compensating affordance | **MET.** The pane is gone and nothing was added to explain it |
| (layouts) corrected comments still describe the `No run yet` absence prose | **MET.** Comment-only edits; the string is untouched |
| (a11y) R0-01 discriminates first-load from post-reload | **MET as a discriminator; the ANSWER is negative.** R0-01 = `not-announced`, `heard: []`. Recorded as a finding, which is what the spec row now prescribes |
| (tests/a11y) CARRY 3's focus start point MEASURED and recorded; no manufactured fix | **MET as measurement, UNFIXED by design.** `tabs_to_start` live 3 · empty 6 · error 5, unchanged. New fact: `browser.refresh()` was resetting the start point. Holder still unmeasured → routed |

**Gates run (all green):** `cargo nextest run -p conductor-verify` · `cargo test -p conductor-verify` · `cargo nextest run --workspace --profile ci` · `cargo test --workspace --doc` · `cargo clippy --workspace --all-targets -- -D warnings` · `npm run typecheck:e2e` · `cargo mutants -f crates/conductor-verify/src/spawn.rs --test-tool=nextest --output target/mutants-verify-2026-09-04 --jobs 2` · `cargo audit` (expected RED, signature unchanged) · `cargo deny check advisories bans licenses sources` · `bash scripts/agent-run.sh run --e2e` · `bash scripts/agent-run.sh boot` · `npm run a11y:sr` / `a11y:sr-empty` / `a11y:sr-error`.

**Smoke (boot-path changed):** `agent-run.sh boot` exit 0, 19:46:49→19:47:36Z, `ready:true` / `canary_round_trip:"ok"` / `data_dir:"<redacted>"`, zero `skipped preflight` lines. Headful self-verify ran as a P2 gate: `[webview2 152.0.4191.62 windows]`, 10 passing / 2 skipped (the two expected live-Pulse-subject specs).

**Process hygiene** (measured by `tasklist`, before and after, not recalled):

| Process | Started by | Final state |
|---|---|---|
| `nvda.exe` | the sr legs | terminated (census 0) |
| `msedgedriver.exe` · `tauri-driver.exe` · `conductor-tauri.exe` · `node.exe` | the legs | terminated (census 0, matches the pre-leg baseline) |
| `pulse-app.exe` (PID 62628) | the operator, for this run | **left running — the operator stops it after the commit** (kept up through this wrap's light gate by directive) |

Note: the first sr leg's session teardown timed out (`DELETE /session` hung 120 s), yet wdio's `onComplete` still cleaned the entire tree — verified by census, not assumed.
