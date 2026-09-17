# Codebase Research — 2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 17 · **Graph queries:** 2 — one rust (`rows: 0`,
  `db_state: fresh`), one ts (`rows: 1`, `db_state: fresh`); derived from the trace's own record count
  (`tree-query-{marker}.json`), which is the authority, not from recollection
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — structural extraction (over the
  25 000-token read cap at 71,485 B): `grep -n` index of 5 headers + 27 `## Session Additions` entry
  introducers, then offset-bounded reads of `:58` (5,687 B), `:62`, `:64`, `:66`. **None of this chunk's
  likely modify-set auto-loads it** — its `paths:` are `scripts/agent-run.*` · `conductor-cli/**` ·
  `conductor-verify/**` · `crates/**/tests/**`, and the set is `.github/workflows/ci.yml` +
  `scripts/a11y-*.ps1`. The rule's own `:58` entry documents this retrieval gap and names this read as the
  compensating channel.

## Files inspected
- `.github/workflows/ci.yml` — the `a11y` job, all steps. The asserting step (`A11y routine arm`) carries
  no `continue-on-error` and no `if:`, on `runs-on: windows-2025`; it resolves `EDGEWEBDRIVER` in the step
  SHELL, sets `CONDUCTOR_MSEDGEDRIVER`, then calls the launcher with `-Program (Get-Command pwsh).Source`,
  `-ArgumentList '-NoProfile','-File','scripts/a11y-token-witness.ps1'`, `-WorkingDirectory $PWD.Path`,
  `-ForwardEnv 'CONDUCTOR_A11Y_STRICT','CONDUCTOR_ENV','CONDUCTOR_MSEDGEDRIVER','EDGEWEBDRIVER'`; captures
  `$LASTEXITCODE`, prints the witness log and `runs/a11y-e2e.log`, then `exit $legExit`.
- `scripts/a11y-limited-token-launch.ps1` (full, 119 lines) — the registered SEVENTH governed spawn form.
  Params `-Program -ArgumentList -WorkingDirectory -ForwardEnv -TimeoutSeconds`; an UNSAFE guard
  (metacharacter class + whitespace) over every argv element → exit 90; program-resolution → 91; runas
  absent → 92; runas declined → 93; sentinel timeout → 94; unparseable sentinel → 95. Env forwarding is
  done by writing User-scope environment variables and restoring them in a `finally`.
- `scripts/a11y-token-witness.ps1` (key mechanics) — prints `IsElevatedAdmin`, `IntegrityLevelSid` /
  `IntegrityLevel` (read from `whoami /groups` by ABSOLUTE path, SID mapped in-script because `.NET`'s
  `WindowsIdentity.Groups` carries no `S-1-16-*`), `WindowStation` (P/Invoke `GetUserObjectInformation`),
  `UserInteractive`, `SessionName`; invokes `agent-run.ps1 run --e2e` (:168); writes the exit sentinel
  `runs/a11y-leg-exit.txt` (:206-208) and exits with the leg's code.
- `scripts/agent-run.ps1` — `$A11yExpectedSkips = 2` (:62); `Assert-A11yVerdict` (:68-100); `--e2e` stage
  (:260-276) building then capturing wdio output to `$RunsDir/a11y-e2e.log`.
- `scripts/agent-run.sh` — the twin: `A11Y_EXPECTED_SKIPS=2` (:188), skip line (:202), verdict line (:225).
- `crates/conductor-tauri/ui/wdio.conf.ts` — `specs: ['./test/a11y/accessibility.e2e.ts']` (:320) and the
  `driven` / `sr` / `sr-empty` / `sr-error` suites (:321-327); `seedFixtureRuns()` at :365; the app spawn's
  `appEnv` setting `CONDUCTOR_RUNS_DIR` per suite (:373, :376, :393) with :403 noting the app inherits it.
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` — `WCAG_TAGS = ['wcag2a','wcag2aa','wcag21aa']`
  (:21), `axeFindings()` (:107-113), 14 `it()` blocks, 4 `this.skip()` sites (:317, :329, :414, :424).
- `crates/conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts` :130-137 — `CENSUS_NAMES`, six
  entries, no webview host (CARRY 3 surface 2 confirmed, SR-leg territory, out of scope).
- `conductor-0.2.0/chunks/2026-09-07-sr-findings-fixed/plan.md:203-216` — the ci-probe form the phase
  directive cites. **Verified verbatim at those exact lines**, including run-id capture at invocation, the
  ban on authoring a run id into the plan, and the wrap-light-gate re-verification
  `gh run view <id> --json conclusion,headSha` with "never a second probe push".

## Graph impact (from the code-graph query; trace at `{run_dir}/tree-query-{marker}.json`)
- **`journal_conformance`** — rust plane, `db_state: fresh`, **rows: 0**. This is NOT an index gap and NOT
  a leaf finding: `grep -rn 'journal_conformance' --include=*.rs crates/` returns **0 lines**, because the
  name is a test-BINARY file (`crates/conductor-run/tests/journal_conformance.rs`, 11,482 B) selected by
  `cargo nextest run -p conductor-run --test journal_conformance`, never a Rust definition. A `symbol`
  query keyed on a file name can never match. Basis: the bare grep, not the exit code.
- **`seedFixtureRuns`** — ts plane, 1 caller: `wdio.conf.ts:365` (graph `line` 364 is 0-indexed; the editor
  line is 365). The routine arm's self-seeding has exactly one call site.
- **Both indexed planes are largely out of this chunk's way.** The modify-set is YAML + PowerShell, neither
  of which is an indexed plane, so the graph bounds the blast radius rather than locating the work:
  no Rust or TS signature changes, hence no caller threading.

## Patterns detected
- **Distinct exit code per launcher failure mode** (`a11y-limited-token-launch.ps1:56-95`): 90 argv
  rejected · 91 program unresolved · 92 mechanism absent · 93 launch declined · 94 sentinel timeout ·
  95 sentinel unparseable, each with its own `[precondition]` line. This is what keeps a token-drop failure
  from reading as a session failure, and the successor must preserve the property.
- **Printed-verdict assertion, not exit code** (`agent-run.ps1:68-100`): `Assert-A11yVerdict` requires the
  `^[webview2 [^\]]*windows` banner, a last `Spec Files:` line with zero failed, and a skip count `<=
  $A11yExpectedSkips` (2), then prints its own green line.
- **Handle guard mirroring** — the launcher's `$unsafe` class is a deliberate mirror of `wdio.conf.ts`'s
  UNSAFE_PATH guard, cited in its own comment as what keeps the composition outside rule (b)'s eval class.
- **Witness-from-inside** (`a11y-token-witness.ps1`): the launched process reports its OWN token and station
  to a file, because the launch mechanism does not carry the child's console back. Registering a token level
  is not evidence of running at it.

## Conventions to follow
- **CI-only script qualifier**: `scripts/`-resident, invoked solely by `ci.yml`'s `a11y` job, wired into
  neither harness shell, no sixth `agent-run` command (arch §Infrastructure Patterns — Directory structure).
- **Handle NAME + boolean, never a resolved path**, in every `[precondition]` / `[diag]` line — the launcher
  already holds this throughout.
- **Integrity read from the token, not from a role predicate** (`a11y-token-witness.ps1:52-73`; also
  `.claude/rules/host-win32.md` 2026-09-16): `IsInRole` returns False for a deny-only group, and `.NET`
  omits the `S-1-16-*` SID entirely.

## The two mechanism claims, re-derived at HEAD

**(1) Inherited premise — "integrity label, not the administrator role, decides the endpoint"**
(marker: `measured, not assumed`; the marker sets spot-check depth). Evidence pointer
`…/evidence/integrity-level-is-the-discriminator.md` is present (3,660 B) and its three legs are restated
verbatim in the route entry. **Still-true at HEAD on its own terms; NOT re-measured on the runner.** All
three legs are dev-host legs at runtime 153. The transfer to the runner is premise 1 below.

**(2) Originated premise — the remedy's API path.** The equality the plan needs is: *`CreateRestrictedToken`
+ `SetTokenInformation(TokenIntegrityLevel)` + `CreateProcessAsUser`, called from PowerShell 7 via
`Add-Type` P/Invoke with no new dependency, yields a child at Medium integrity.* Probed at HEAD
(scratchpad only, nothing added to the tree):

```
[probe] OpenProcessToken: ok
[probe] current token integrity SID: S-1-16-8192
[probe] CreateRestrictedToken(LUA_TOKEN): ok
[probe] SetTokenInformation(TokenIntegrityLevel=Medium): ok
[probe] RESTRICTED token integrity SID: S-1-16-8192
```

**What this shows:** the two token APIs are reachable from PowerShell 7 through `Add-Type`, succeed for this
account, and need no package — so the remedy can live in the existing `scripts/` PowerShell form with zero
dependency delta. **What it does NOT show, and must not be written as if it did:** (a) this shell was
ALREADY Medium (`S-1-16-8192`), so setting Medium was a no-op in effect — the High→Medium LOWERING direction
is unproven here, resting on the documented rule that a label may be lowered but never raised; (b)
`CreateProcessAsUser`, the third API, was not exercised at all, because it requires an actual launch; (c)
nothing about the runner. Two of three APIs, one host, wrong starting integrity.

## New files to create
- (none expected) — the remedy replaces the mechanism INSIDE `scripts/a11y-limited-token-launch.ps1`. A
  fourth `scripts/` entry point would need its own arch directory-tree registration; nesting a second
  launcher inside the first buys nothing and doubles the registered surface.

## Files to modify
- `scripts/a11y-limited-token-launch.ps1` — swap the `runas /trustlevel:0x20000` mechanism for the API
  triple. Three consequences the current file's shape makes cheap: `CreateProcessAsUser` does **not**
  detach, so the child handle gives the exit code directly and the sentinel wait (:96-110, exit codes 94/95)
  becomes a fallback rather than the channel; the env block can be passed to the child explicitly instead of
  via User-scope registry writes (:69-76), which removes a global side effect from a CI step; and the
  whitespace/metacharacter guard must STAY, because `CreateProcessAsUser` takes `lpCommandLine` as a string
  and the form therefore remains the composing class, not an arraying one.
- `.github/workflows/ci.yml` (the `a11y` job's asserting step) — **two stale comments that describe a
  mechanism that no longer ships**: "The scheduler does not capture the task's console" and "the task never
  reached its witness". The scheduled-task route was replaced by `runas` inside the predecessor chunk; these
  sentences survived it. Whether the step's `run:` body itself changes depends on the launcher's parameter
  surface staying fixed (recommended: keep it fixed, so the step is comment-only).
- `scripts/a11y-token-witness.ps1` — the `(task)` prefix on every `[diag]` line is the same stale naming,
  and the sentinel write (:206-208) is re-scoped by the non-detaching launch.
- **Not a touchpoint:** the spec-master text for the Evergreen disposition (CARRY 4 / obligation 1) and the
  spawn-form registry — those are wrap amendments, per the phase contract.

## Scope premise closure (amended into `scope.md` this phase)
1. **Dev-host finding transfers to the runner** — **OPEN, now with a named instrument.** Unmeasurable from
   this host by construction. The operator's phase directive authorizes the ci-probe, which is the only
   instrument that takes this measurement before the chunk ships; CARRY 2's window-station axis rides the
   same probe at zero extra cost, because `a11y-token-witness.ps1` already prints `WindowStation` /
   `UserInteractive` / `SessionName` from inside the leg (verified at :88-100).
2. **Routine specs vs the console as it stands** — **FALSIFIED in the favourable direction, four ways.**
   Obligation 2 feared the unbuilt `contentinfo` footer as an independent second cause of red. It cannot
   fire in the routine arm: (a) the arm runs `accessibility.e2e.ts` alone (`wdio.conf.ts:320`); (b) that
   file makes no landmark assertion — 14 `it()` blocks, none landmark-shaped; (c) axe runs
   `withTags(['wcag2a','wcag2aa','wcag21aa'])`, and in the installed **axe-core 4.12.0** every
   landmark-family rule (`region`, `landmark-one-main`, `landmark-unique`,
   `landmark-complementary-is-top-level`, `landmark-banner-is-top-level`,
   `landmark-contentinfo-is-top-level`, `landmark-no-duplicate-contentinfo`) is tagged `best-practice` and
   **none is selected** — 68 of 105 rules run; (d) the only three `contentinfo` mentions in the test tree
   are `screen-reader/nvda-pass-spec.md:117,200` and `screen-reader/rows.ts:115`, which record its absence
   as a FINDING in the operator-local SR arm. `grep -rc contentinfo crates/conductor-tauri/ui/src/` = 0 and
   `<footer` = 0, so the DOM gap is real; it is simply not something this gate can see. **a11y-plan §11's
   landmark-less-window ban is a spec rule with no automated enforcement in this arm** — worth saying
   plainly rather than leaving as an implied green.
3. **Replacement of the seventh governed form, or an eighth crossing** — **RESOLVED to a decisive lean:
   REPLACEMENT, count stays SEVEN.** Basis: the registry counts spawn FORMS, and this chunk changes the
   internal mechanism of the one launcher at the one call site rather than adding a spawn site; and the
   predecessor's own amendment ruled that "the mechanism registered must be the one that SHIPS" after a
   scheduled-task form was registered and then replaced inside a single chunk. The distinction is text-only:
   security-plan's §Security Anti-Patterns → Code Patterns rule (b) makes this an operator-ratified
   boundary-widening escalation **either way**, and the admitting argument genuinely changes (the seventh is
   admitted as `-File` only and never `-Command`; the successor is admitted as a fixed `lpApplicationName`
   with a guarded composed `lpCommandLine`). Carried to the P5 review card as a confirm-class item.
4. **API triple reachable from PowerShell** — **VERIFIED for two of three APIs, with the limits above.**
   No new dependency, so no `Cargo.lock` / `package-lock.json` delta and no new third-party class.

## Two mechanical hazards this chunk's gates will sit on
Both from `verification-harness.md:66`, measured at `2026-09-11-hosted-runner-endpoint-cause-closed`:
- **`runs/a11y-e2e.log` is UTF-16 with a `ff fe` BOM** — a PowerShell-redirected capture. `grep` returns
  **zero** on it, silently, which reads as "the leg never got that far". Decode before concluding
  (`raw[:2] == b'\xff\xfe'` → `.decode('utf-16')`).
- **GitHub Actions echoes each step's own source into the job log**, so a literal inside a `Write-Output
  "…"` template appears TWICE — once unexpanded in the echo, once for real. A counting gate over a captured
  job log needs a `| grep -av 'Write-Output'` stage; the tell that it is an echo is the unexpanded `$var`.

## Printed-verdict atoms — sources for the `expect` entries
`verification-harness.md:64` requires a live leg's atoms be read from a recorded output or the tool's own
print site, because a `leg` entry is exempt from P5's baseline run and is checked by nothing at authoring
time. Print sites, read at HEAD:

| atom | source | note |
|---|---|---|
| `contains [a11y] verdict asserted` | `scripts/agent-run.ps1:99` / `scripts/agent-run.sh:225` | green verdict |
| `lacks [a11y] leg skipped` | `scripts/agent-run.ps1:76` / `scripts/agent-run.sh:202` | the skip-green guard |

**Shell-parity finding — the two shells print DIFFERENT verdict text.** `.ps1:99` uses an ASCII hyphen and
`|` separators (`verdict asserted - 0 failed | … | driven session present`); `.sh:225` uses an EM DASH and
`·` separators (`verdict asserted — 0 failed · … · driven session present`). The same divergence sits on the
skip line (`:76` vs `:202`). The SEMANTICS are identical, which is what test-plan §3's parity contract
binds, so this is not a defect to fix here — but an `expect` atom must stop at the common prefix
(`[a11y] verdict asserted`, `[a11y] leg skipped`) or it will hold in one shell and fail in the other.

## Open questions
- Does a Medium-integrity `CreateProcessAsUser` launch on the RUNNER produce `DevToolsActivePort`?
  → blocks: **implementation-scope**. Only the ci-probe answers it; the plan names the probe, and the
  answer decides which terminal arm (green vs ratified exclusion) the chunk closes on.
- Do `:384`/`:397` hold at the runner's WebView2 152? → blocks: **implementation-scope**. They fail at 153
  on the dev host; their 152 status rests on the 2026-09-10 record, not a fresh run. If the probe opens a
  session, this becomes the next thing the same probe measures — and CARRY 1 says a session is not a green
  arm precisely because of these two.
