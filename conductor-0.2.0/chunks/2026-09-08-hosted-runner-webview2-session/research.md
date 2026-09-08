# Codebase Research — 2026-09-08-hosted-runner-webview2-session

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 11
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read IN FULL as a structural
  extraction (63 lines / 63.7 KB, up to 13.2 KB per line; index by header + per-entry introducer, then
  offset-bounded reads covering lines 1-46, 47, 48-56, 57-63 — every span covered). Also
  `.claude/rules/a11y.md` and `.claude/rules/frontend.md` (auto-loaded; `paths:` cover
  `crates/conductor-tauri/ui/**` and `**/*.ts`, which the modify-set touches). `observability.md` also
  scopes `scripts/agent-run.*` but this chunk edits no shell.
  **Retrieval note carried forward:** `verification-harness.md`'s `paths:` do NOT cover
  `crates/conductor-tauri/ui/test/`, so a ui-only chunk never auto-loads it — its own entry 58 records
  that this research-time read is the only retrieval path.

## Files inspected
- `.github/workflows/ci.yml` (218-395, the whole `a11y` job) — all three stages are wired; every
  research question the extracts raised about it is answered below.
- `crates/conductor-tauri/ui/wdio.conf.ts` (structural index + 314-423 in full) — capabilities, the ONE
  tauri-driver spawn site, the guard functions.
- `crates/conductor-tauri/ui/node_modules/@crabnebula/tauri-driver/` — `cli.js`, `main.js`, `README.md`,
  `CHANGELOG.md`, `src/lib.rs` (an 8-line stub; the real binary is the platform package).
- `crates/conductor-tauri/ui/node_modules/@crabnebula/tauri-driver-win32-x64-msvc/` — the compiled
  `.node` binary (string probe only).
- `crates/conductor-tauri/ui/package-lock.json` — driver pin.
- `scripts/agent-run.ps1` (`--e2e` arm + `Assert-A11yVerdict`).
- `.andromeda/test-plan.md:307` — the desktop-webview driver row, in full.
- CI run **34209940695** job log (the baseline) — read directly, not inherited.

## Graph impact (code-graph, plane `ts`, `db_state: fresh`, 8 rows, `probe_hits: null`)
Query: `calls` keyed on `callee_name` for the wdio guard/lifecycle functions.
- **`exitUnresolvedHandle`** — 2 callers, both in `onPrepare`: `wdio.conf.ts:357` (driver handle) and
  `:385` (NVDA handle). *(editor lines; the graph's are 0-indexed)*
- **`strictMode`** — 1 caller, `exitUnresolvedHandle` @ `wdio.conf.ts:76`. **The strictness inversion has
  exactly ONE routing site at HEAD**, so arch's "one skip/strict routing site" pattern holds and any new
  precondition arm must join it rather than add a second exit path.
- **`nativeDriver`** @ `:354` · **`nvdaExe`** @ `:382` · **`seedFixtureRuns`** @ `:365` · **`startNvda`**
  @ `:399` · **`writeA11yEnvelope`** @ `:438` — each a single call site, all inside the config.
- Consequence: the whole guard + envelope surface is config-local. A remedy confined to `wdio.conf.ts`
  has no cross-file caller threading; a remedy that changes the DRIVER PROGRAM does not either, but it
  changes a governed spawn form (below).

## Patterns detected
- **One tauri-driver spawn site** (`wdio.conf.ts:405-409`): `spawn(process.execPath, [driverCli,
  '--native-driver', native], { cwd: repoRoot, env: appEnv, stdio: [...] })`, where `driverCli =
  require.resolve('@crabnebula/tauri-driver/cli.js')` (`:49`) — the repo-derived resolved constant
  security-plan's spawn rule (b) names. `appEnv` is `{ ...process.env, CONDUCTOR_RUNS_DIR: <per-suite> }`.
- **The routine arm's capability is `browserName: 'wry'`** (`:339`) with `'wdio:enforceWebDriverClassic':
  true` and `'tauri:options': { application }` — *not* `webview2`. tauri-driver translates `wry` into the
  native driver's session; the CI isolation diagnostic posts `browserName: 'webview2'` straight at
  msedgedriver. These are two different launch paths to the same failure.
- **The a11y job is fully wired at HEAD** (`ci.yml:219-395`): `windows-2025` (explicit, per arch's pin) ·
  the two `continue-on-error` diagnostics (`:263`, `:314`, the latter `if: always()`) · the
  `a11y-session-diag` upload (`:366`) · the routine arm (`:284`) · the **conformance gate (`:379`) with NO
  `continue-on-error`** · the violation-record upload (`:388`).
- **Verdict assertion is printed-verdict, mechanized** (`agent-run.ps1:68-99` `Assert-A11yVerdict`): the
  `Spec Files:` line (last match), the failed count, the skip tally vs `$A11yExpectedSkips`, and the
  driven-session banner; exit read from the bare command.

## Conventions to follow
- **Guard shape**: `UNSAFE_PATH = /[;&|`$<>\r\n"']/` (`wdio.conf.ts:54`) + existence/`isFile` in
  `nativeDriver()` (`:56-58`) / `nvdaExe()` (`:182-184`), then array-form spawn. Byte-unchanged is the bar.
- **Strictness is an added arm**: `strictMode()` (`:68`) only changes what an unresolved handle COSTS.
- **CI resource tags are read by the config, not set by the job**: `wdio.conf.ts:301-303` adds
  `ci.run.id` / `git.commit.sha` from `GITHUB_RUN_ID` / `GITHUB_SHA` under a `GITHUB_ACTIONS === 'true'`
  guard. The job sets only `CONDUCTOR_A11Y_STRICT: '1'` and `CONDUCTOR_ENV: dev` (`ci.yml:286-288`).

## Extract research questions — answered at HEAD
| Question (raiser) | Answer |
|---|---|
| Does the `a11y` job declare `CONDUCTOR_A11Y_STRICT`? (a11y) | **Yes** — `ci.yml:287`, `'1'`. |
| Does it assert the printed verdict? (a11y, layouts, tests) | **Yes** — via `agent-run.ps1`'s `Assert-A11yVerdict`, all three assertions present. |
| Does the job set `deployment.environment` / `ci.run.id` / `git.commit.sha`? (obs) | `CONDUCTOR_ENV: dev` on the step; the other two are added by `wdio.conf.ts:301-303` from the GitHub env. All three reach the record. |
| Are all three a11y stages wired? (obs) | **Yes** — routine arm, conformance gate, record upload, plus two diagnostics and the diag upload. |
| Does the gate step carry `continue-on-error`? (tests) | **No** — only the two diagnostics and the diag upload do. Constraint satisfied at HEAD. |
| Is the driver program a repo-derived resolved constant? (security) | **Yes** — `require.resolve('@crabnebula/tauri-driver/cli.js')`, `wdio.conf.ts:49`. |
| Is a wdio retry policy configured? (tests) | **No** — no `specFileRetries` / retry key anywhere in `wdio.conf.ts` or `package.json`. The baseline's `(1 retries)` is the spec reporter's attempt rendering, not a configured policy. |
| Does `ci.yml` still carry the explicit `windows-2025` label? (arch) | **Yes** — `:223`, with the pin's reason in a comment at `:221-222`. |

## The four hypotheses, re-derived at HEAD
Every one arrived `hypothesis:`-marked, so each is verified outright rather than spot-checked.

> **RETRACTION (P5 review, operator dev-host measurement).** The H2 conclusion below — that supplying
> `WEBVIEW2_USER_DATA_FOLDER` is "one added key on `appEnv`" and therefore the cheapest remedy — is **WRONG
> as a remedy claim** and H2 is retired. The env chain does propagate, and the loader does honour the handle
> on a BARE launch (`<dir>\EBWebView\DevToolsActivePort` in 1 s). But `msedgedriver` in webview2 mode puts
> the profile in its OWN `%TEMP%\scoped_dir<pid>_<rand>\EBWebView\` **and does so even with the handle
> exported to the driver**, leaving the inherited directory empty — the driver sets the folder itself and
> overrides any inherited value. **The mechanism of my error:** I probed `@crabnebula/tauri-driver` and found
> it neither sets nor overrides a user-data folder — true, and irrelevant, because the overriding writer is
> `msedgedriver`, which I never probed. Tracing the handle to a reader is not tracing it to its final writer.
> The paragraph below is left standing as the record of what was concluded and on what basis; read it only
> with this retraction attached.

- **H2 — `WEBVIEW2_USER_DATA_FOLDER` under `$RUNNER_TEMP`. Cheapest by a wide margin; LEVER VERIFIED
  REACHABLE, mechanism unverified (which is what the probe is for).** `tauri-driver`'s entire CLI surface
  is `--port` · `--native-port` · `--native-host` · `--native-driver` + the `REMOTE_WEBDRIVER_URL` env
  (measured: `node cli.js --help`) — there is **no user-data-folder or extra-browser-args option**. But
  the handle need not go through the CLI: the app inherits the environment down the spawn chain
  (`wdio.conf.ts:405-409` passes `env: appEnv` → tauri-driver → msedgedriver → the app), so H2 is
  implementable as **one added key on `appEnv`** — no dependency change, no spawn-form change, no new
  governed form. A string probe of the compiled bridge binary finds **0** occurrences of
  `WEBVIEW2_USER_DATA_FOLDER` / `user-data-dir` / `userDataFolder` /
  `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` / `remote-debugging-port` / `DevToolsActivePort`, against
  positive controls `webviewOptions` (1) · `webview2` (1) · `msedgedriver` (1) · `native-driver` (4)
  proving plain literals are readable in that binary — so **the bridge neither sets nor overrides a user
  data folder**. *(Bounded: a compiled-binary strings probe; a UTF-16 or split literal would not be
  caught. The positive controls make a plain-literal absence the likely reading, not a certain one.)*
  **This does NOT contradict the CARRY's "UDF/path excluded by measurement"** — that exclusion measured
  that *no path is passed* and that the verbose log *names* none; H2 proposes *supplying* one. The two
  are compatible, and reading the exclusion as foreclosing H2 would be a misreading.
- **H3 — driver 151 vs this host's 152: PREMISE FALSIFIED, and it points the wrong way.**
  `test-plan.md:307` records the measured working set as msedgedriver **151.0.4129.101 driving WebView2
  Runtime 152.0.4191.53 green** across three `sr*` sessions and the routine `--e2e` re-run, and again
  against Runtime **152.0.4191.62** un-refreshed. So a cross-major skew is measured GREEN on the dev
  host. The runner has **no skew at all** — driver = Runtime = Edge = 151.0.4129.101, self-consistent
  (the CARRY's own measurement). The failing side is the version-consistent one and the passing side is
  the skewed one, so the version axis cannot carry the cause as stated.
- **H1 — official `tauri-driver` crate vs `@crabnebula`'s translation: NOT drop-in; it is a governed-form
  change.** The swap replaces the spawn PROGRAM — today `require.resolve('@crabnebula/tauri-driver/cli.js')`,
  a repo-derived resolved constant — with a `cargo install`ed binary resolved from `PATH`, which
  security-plan's harness-spawn rule (b) does not admit among its five forms. Per that plan's own history
  a sixth form is boundary widening: escalate, never routine. Availability and CLI/protocol parity of the
  official crate are **unverified here** (no network read was taken).
- **H4 — self-hosted Windows runner: infrastructure, not code.** Availability is the operator's fact.
  test-plan §10's zero-flakiness clause ("never a runner to pin to") is aimed at a test that is green on
  one runner and red on another; whether a hosted-runner *capability gap* falls inside it is a judgment
  the plan must state rather than assume (the tests extract flags this explicitly).

**A fifth option the entry never named, found in the bridge's own README:** on Windows the supported
paths are "msedgedriver **or** the CrabNebula Webdriver", and "for Linux and Windows you can also use
`tauri-plugin-automation` and configure your test to execute the test runner backend"
(`@crabnebula/test-runner-backend`). This stays inside the ONE WebdriverIO + tauri-driver stack, so it is
not the banned second stack — but it adds a **runtime plugin to the shipped app**, which is a materially
larger change than H2 and carries its own security surface. Surfaced as an option, not a recommendation.

## Files to modify
- `crates/conductor-tauri/ui/wdio.conf.ts` — the probe/remedy lands here (an `appEnv` key for H2; the
  spawn program for H1). Guard functions and the single `exitUnresolvedHandle` routing site stay
  byte-unchanged. **Per test-plan §4, a change here is unproven until a wdio leg that LOADS it executes** —
  `tsc --noEmit` does not prove ESM loadability (the `__dirname` precedent).
- `.github/workflows/ci.yml` — any added probe step joins the `continue-on-error` diagnostic channel; the
  gate step and the conformance gate stay unconditioned.
- `conductor-0.2.0/verification-matrix.json#v2-24` — the disposition.
- `crates/conductor-tauri/ui/package.json` + `package-lock.json` — **only if H1 is taken.**

No caller threading is owed: the graph shows every touched symbol is config-local with a single call site.

## Open questions
1. Does the WebView2 loader honour `WEBVIEW2_USER_DATA_FOLDER` in the host-app mode msedgedriver
   launches, such that `DevToolsActivePort` lands where the driver looks? → blocks: **implementation-scope**
   (it is what H2's probe measures; the plan must state it as a measurement, not an expectation).
2. Is the official `tauri-driver` crate obtainable on the runner and CLI-compatible with the current
   `--native-driver` invocation? → blocks: **implementation-scope** (only if H1 is reached; it also needs
   the security escalation before it can land).
3. Does test-plan §10's "never a runner to pin to" reach H4, or is a hosted-runner capability gap outside
   it? → blocks: **plan-decision** — P4 must state the reading rather than assume it.

## Scope premise closure
Applied to `scope.md` (amended before P4 consumes it): bullets 1, 3, 6 and 7 **verified**; bullet 2
**narrowed**; bullet 4 **falsified** and rewritten `[premise-corrected]`; bullet 5 stands unverified as an
operator/infrastructure fact.
