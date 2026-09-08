# Scope — 2026-09-08-hosted-runner-webview2-session

**Chunk:** Hosted-runner WebView2 session — the a11y job's first green run (v2-24 claimed or deferred)
**Version:** conductor-0.2.0 · **Epoch:** Epoch 6b — Polish & ship
**Working entry:** `conductor-0.2.0/working-route.md:125`

---

## Intent

The `a11y` CI job is correctly wired and correctly RED. It reaches the webview leg, resolves its driver
handle, builds the release binary, and then dies at WebDriver session creation. This chunk measures WHY on
the hosted runner, applies whatever remedy the measurement supports, and — either way — **disposes of
`v2-24`**: claimed on a green a11y job, or explicitly `deferred` with the measured reason in `notes`.

The disposition is the deliverable. A green job is the preferred outcome, not the required one; what is
required is that `v2-24` stop being pooled on an unmeasured cause.

## Baseline — measured this session, on the build branch

**CI run 34209940695**, head `a71e6bf` (`build/conductor-0.2.0`; `origin` and `HEAD` now agree — the
operator's push has happened, so this is a build-branch run, not a probe ref).

| Job | Result |
|---|---|
| Rust gate (build · test · lint · supply-chain · coverage) | ✓ success (09:25:52 → 09:33:13Z) |
| Frontend gate (npm audit · build) | ✓ success (09:25:51 → 09:26:48Z) |
| A11y gate (routine arm · axe · contrast · violation JSON) | ✗ failure (09:25:50 → 09:37:36Z, 11 m 46 s) |

Read from the job log itself, not inherited:

- `09:26:37.132Z` — `[precondition] CONDUCTOR_MSEDGEDRIVER: resolved` — **the SR chunk's driver-handle fix
  holds on the build branch.** The `EDGEWEBDRIVER`-in-the-step-shell resolution works.
- `09:34:06.926Z` — `Execution of 1 workers started` (≈8 min of npm install + release build precede it).
- `09:35:08.923Z` — `[0-0] WARN webdriver: WebDriverError: session not created: DevToolsActivePort file
  doesn't exist when running "http://127.0.0.1:4444/session" with method "POST"`
- `09:36:09.055Z` — `Spec Files: 0 passed, 1 failed, 1 total (100% completed) in 00:02:01`, the spec
  `FAILED in wry` after `(1 retries)`; then `error: wdio exited 1` → `Process completed with exit code 1`.

Two properties of that log worth carrying into the work: **the decisive line is logged at `WARN`, not
`ERROR`** (the standing corpus rule about severity-as-proxy applies to any sweep of these logs), and
**wdio's full RED output now reaches the job log** — the second SR-chunk fix, also holding.

This run supersedes the four probe-ref runs as the citable baseline; those proved the two fixes on probe
branches and are evidence about the fixes, not about the current build-branch state.

## Folded CARRY — from `2026-09-07-sr-findings-fixed` (operator wrap-directive item 1)

**Standing premise, MEASURED:** on the hosted `windows-2025` runner **a WebView2-mode session never exposes
`DevToolsActivePort`** — through `@crabnebula/tauri-driver` and direct at `msedgedriver` alike, as measured
at run 34162118841, and now reproduced on the build branch at run 34209940695.

**The cause beyond that statement is UNMEASURED.** Do not inherit a mechanism for it.

**Excluded by measurement** (all of them, at the prior chunk):
- runtime absence — WebView2 Evergreen Runtime present;
- version mismatch — msedgedriver = Runtime = Edge, all `151.0.4129.101`, self-consistent (this dev host
  runs 152, so the runner is one major behind, but internally consistent);
- app crash — launched alone on the runner the app stays up with three `msedgewebview2.exe` children in
  Session 2, zero-byte stderr, no Crashpad dumps;
- GPU/sandbox;
- UDF/path — WebView2 mode passes no `--user-data-dir` at all and the verbose log names no path.

**RETRACTED — do not rebuild on it:** the earlier reading that msedgedriver "launched the Tauri binary as
though it were Edge" was a **PROBE ARTIFACT**. The isolation POST omitted `browserName: "webview2"`, without
which the same POST fails on the DEV HOST too against a binary that passes 12/12 through tauri-driver. With
the capability added, the driver's side is provably correct on the runner (`WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS`
populated including `--remote-debugging-port=0`; app launched bare) and the session still fails.
*Validate any diagnostic form where the real path PASSES before building on its failure.*

**Tooling already in place** (verified present at HEAD):
- `.github/workflows/ci.yml:263` — `WebView2 driver + runtime versions (diagnostic)`, `continue-on-error`;
- `.github/workflows/ci.yml:314` — `WebView2 session isolation (diagnostic)`, `continue-on-error`, `if: always()`;
- `.github/workflows/ci.yml:366` — `Upload session diagnostics` → artifact `a11y-session-diag`;
- `conductor-0.2.0/chunks/2026-09-07-sr-findings-fixed/evidence/ci-msedgedriver-verbose.log` (3 363 B, the
  corrected webview2-mode run, path-scrubbed).

**Next probes — every one `hypothesis:`, none measured:**
1. `hypothesis:` the official `tauri-driver` crate vs `@crabnebula/tauri-driver`'s translation.
   *(HEAD ships `@crabnebula/tauri-driver ^2.0.9` as a devDependency.)*
2. `hypothesis:` an explicit `WEBVIEW2_USER_DATA_FOLDER` under `$RUNNER_TEMP`.
3. `hypothesis:` driver 151's WebView2 mode vs this host's 152.
4. `hypothesis:` a self-hosted Windows runner as the fallback.

**`v2-24` is UN-CLAIMED and pooled** (`chunk: null`, status `planned`) with this premise and its retraction
already written into its `notes` — verified at HEAD. **This entry owns claiming or deferring it.**

**The *A11y CI gate* entry's `BLOCKED-ON` clearing event — one GREEN CI run of the a11y job after a push — is
NOT met.** Its premise lives here now, and is deliberately NOT re-raised as a fresh `BLOCKED-ON` on
*Release build and bundle*.

## What this chunk does

1. **Measure the cause**, working the four hypotheses on the hosted runner via probe refs. The standing
   premise bounds the search: the session-creation POST fails identically through the intermediary and
   direct, so the remedy is not in wdio's or tauri-driver's own configuration surface unless hypothesis 1
   shows the two drivers differ in what they hand the WebView2 loader.
2. **Apply the remedy the measurement supports** — or record that none of the four is it.
3. **Dispose of `v2-24`** on what the runner measurement shows. **Both arms are reachable inside this chunk,
   because the probe push happens inside `/implement`'s lifecycle** (corrected at the P5 review, A3 —
   an earlier draft of this scope wrongly called the claim arm structurally unreachable, reasoning from the
   BUILD-BRANCH run that follows the wrap and forgetting that the probe ref runs CI during implement):
   - **green `a11y` job** → `/implement` claims at P2 with `ref` = **the job's run id**, the external-harness
     id being the evidence;
   - **still red** → **`deferred`**, with `notes` citing the three probe measurements as the **cause basis**,
     never the exhaustion of a hypothesis list.
   Phase still does not PRE-claim: the outcome is a measurement not yet taken, and pre-claiming would
   guarantee a coverage-gate HALT on the red branch.

4. **Operator decisions taken this session (2026-09-08), now part of the scope:**
   - **Probe budget: ONE push.** The candidate remedy and a cause-discriminating diagnostic ride the same
     probe ref, so the chunk learns the cause whether or not the remedy works; if still red, dispose and route
     the boundary-crossing options forward.
   - **No self-hosted Windows runner exists.** Hypothesis 4 is a route-forward note, never a probe this chunk
     can fire.

## Boundaries — deliberately NOT this chunk

- **The Epoch-6b split** — the growth valve is a wrap-time word, the operator's.
- **The owners of `v2-04` / `v2-21`** — route-resolve's business at the wrap.
- **Re-raising a `BLOCKED-ON`** on *Release build and bundle* — the entry rules it out explicitly.
- **The build-branch push** — the operator's act (already performed for this baseline).
- **The two SR-chunk fixes** — done and proven in CI; not re-opened.

## Probe-ref hygiene (operator constraint, binding)

Probe refs push otherwise-uncommitted work to a **PUBLIC** repo (`Turbolet85/andromeda-conductor`). A probe
tree therefore **carries no host paths and no evidence captures**. This binds every probe branch this chunk
pushes; the artifact-hygiene invariant (never leak absolute host paths) applies to the probe tree itself,
not only to Conductor's generated artifacts.

## Surfaces and contracts touched

| Surface | Expected involvement |
|---|---|
| `.github/workflows/ci.yml` (`a11y` job) | the probe edits and any remedy land here |
| `crates/conductor-tauri/ui/wdio.conf.ts` | the ONE config all suite families fire; the driver spawn + `CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_A11Y_STRICT` guards |
| `crates/conductor-tauri/ui/package.json` | `@crabnebula/tauri-driver` dependency, if hypothesis 1 is taken up |
| `conductor-0.2.0/verification-matrix.json#v2-24` | the disposition — claim or `deferred` + `notes` |
| `runs/a11y/<run_id>.jsonl` + `journal_conformance` | only reachable once a session is created; unchanged by this chunk |
| `scripts/agent-run.ps1` (`run --e2e`) | the job's entrypoint; expected read-only |

## Premises — closed at P3 against research (`research.md`)

- **VERIFIED — The hypotheses are independently testable and orderable.** Research derived the order from
  the driver's real surfaces: **H2 is cheapest by a wide margin** (one added key on `appEnv`, no dependency
  and no spawn-form change), H4 is an operator/infrastructure fact, H1 is a governed-spawn-form change
  requiring escalation, and H3 is demoted on a falsified premise. A **fifth** option surfaced that the entry
  never named — `tauri-plugin-automation` + `@crabnebula/test-runner-backend`, the bridge README's other
  Windows path — inside the one stack but adding a runtime plugin to the shipped app.
- **NARROWED — the official `tauri-driver` crate is not "drop-in" (hypothesis 1).** Its availability and CLI
  parity remain unverified (no network read taken), but the swap is now known to change the spawn PROGRAM
  from a repo-derived resolved constant (`require.resolve('@crabnebula/tauri-driver/cli.js')`) to a
  `PATH`-resolved installed binary — a **sixth governed spawn form** under security-plan rule (b), which that
  plan's history classifies as boundary widening: escalate, never routine.
- `[premise-corrected: operator dev-host measurement at the P5 review — msedgedriver in webview2 mode puts the
  app's profile in its OWN %TEMP%\scoped_dir<pid>_<rand>\EBWebView\ and does so even with
  WEBVIEW2_USER_DATA_FOLDER exported to the driver, leaving the inherited dir empty]` **Hypothesis 2 is
  RETIRED as a remedy: the handle is not a lever on the driver path.** The P3 reading — that the env chain
  makes the handle "one added key on `appEnv`" — was right about propagation and wrong about effect: the
  chain propagates and **the driver overwrites at the end of it**. The loader does honour the handle on a
  BARE launch (`<dir>\EBWebView\DevToolsActivePort` in 1 s), which is precisely why it looked like a lever.
  My P3 probe measured that *tauri-driver* neither sets nor overrides a user-data folder — true, and
  irrelevant, because the overriding writer is **msedgedriver**, which I never probed. The chunk therefore
  ships no remedy and becomes diagnose-only.
- `[premise-corrected: test-plan.md:307 records msedgedriver 151 driving WebView2 Runtime 152 GREEN on the
  dev host across three sr* sessions and the routine --e2e re-run; the runner's driver, Runtime and Edge are
  all 151.0.4129.101, self-consistent]` **The version axis points the WRONG WAY and hypothesis 3 is demoted.**
  The failing side (the runner) has no version skew at all; the passing side (this dev host) has a cross-major
  one. Steerability was never the question — causality was, and the measurement contradicts it.
- **UNVERIFIED, and correctly so — a self-hosted Windows runner (hypothesis 4).** An infrastructure fact about
  the operator's environment, not a code fact. Additionally, whether test-plan §10's "never a runner to pin
  to" reaches a hosted-runner *capability gap* is a plan-decision P4 must state rather than assume.
- **VERIFIED — a `deferred` disposition is reachable without a green job.** The matrix contract's §Status
  lifecycle names terminal-for-done as `verified` **or** operator `deferred` with the reason in `notes`.
- **VERIFIED — the ~8-minute pre-spec build is the probe cycle's floor.** Run 34209940695: job start
  09:25:50Z → workers 09:34:06.926Z = 8 m 16 s, whole job 11 m 46 s. The plan's step count must respect it.
