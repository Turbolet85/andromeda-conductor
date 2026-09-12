# Codebase Research — 2026-09-11-hosted-runner-endpoint-cause-closed

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 6
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read as a structural extraction
  (69,621 bytes over 65 lines; `grep -n` index of the 4 section headers + 26 dated entry introducers, then
  offset-bounded reads of L17/L26/L31/L35 and entries L42, L57, L58, L62, L64, L65). Four additions applied:
  the PowerShell-`bash`-is-WSL trap (L57a), the `--e2e` exit-code-cannot-separate-pass-from-skip rule + the
  process-census obligation (L58), the `--e2e`-is-the-only-rebuilding-arm rule (L62), and the
  `expect`-atoms-on-a-`leg`-entry-are-checked-by-nothing rule (L64).

## Files inspected
- `.github/workflows/ci.yml` (`:227`–`:545`, the `a11y` job) — step set, postures, and the exact live-process
  window arm A must fire inside.
- `scripts/webview2-cause-probe.ps1` (full, 141 lines) — `param([string]$OutFile)`, three sections, the
  `Emit`-into-`$out`-then-dump structure, the completion marker, unconditional `exit 0`.
- `scripts/agent-run.ps1` (`:62`–`:100`, `:190`–`:210`, `:252`–`:280`) — `Assert-A11yVerdict`, the `--e2e`
  arm's build+capture+assert sequence, the driver-handle guard.
- `scripts/agent-run.sh` (`:188`–`:250`) — the `.sh` twin of the verdict assertion, for parity.
- `.andromeda/architecture.md` `:197` — the `WEBVIEW2_*` registration and its stated lifetime.
- `conductor-0.3.0/chunks/2026-09-11-hosted-runner-endpoint-cause-probed/plan.md` (`## Test Commands`,
  steps 4–5) — the `leg = 'operator'` + `<id>` shape and the operator-push ordering.
- `conductor-0.3.0/verification-matrix.json` via `matrix.py show --id v3-01` — acceptance, `observed_gap`,
  and the un-claim note.
- `.gitignore` `:17` — `/runs/` ignored, confirming the raw-capture split.

## Graph impact
**Graph NOT APPLICABLE — the change surface lies outside every indexed plane.** This is a scope fact, not a
degradation: the graph was considered and correctly has no subject here (distinct from graph UNAVAILABLE —
cold-start, stale, or a skipped plane). The two indexed planes are `rust` and `ts`; this
chunk's modify-set is `.github/workflows/ci.yml` (YAML) and `scripts/webview2-cause-probe.ps1` (PowerShell),
plus committed Markdown evidence. No symbol is added, removed or re-signatured, so there is no caller set to
thread and no impact query with a subject. Basis: the modify-set below contains no `.rs` or `.ts` path — not
an empty or stale DB (both plane DBs refreshed 2026-09-11T10:28Z).

The one cross-plane READER that matters is `crates/conductor-tauri/ui/wdio.conf.ts`, the sole reader and sole
guard site of `CONDUCTOR_MSEDGEDRIVER`. It is **read, never modified**, by this chunk.

## Patterns detected
- **The verdict is a printed line, and the harness prints it itself** (`scripts/agent-run.ps1:100`):
  `[a11y] verdict asserted - 0 failed | {skips} skipped (expected 2) | driven session present`. This is a
  low-cardinality token the tool alone produces — the atom class the harness rules prescribe for a `leg`
  entry whose `expect` no baseline run will check.
- **The two shells' verdict lines are NOT byte-identical.** `.ps1` prints ASCII `- 0 failed | N skipped`
  (`agent-run.ps1:100`); `.sh` prints `— 0 failed · N skipped` with an em-dash and middots
  (`agent-run.sh:225`). The common prefix `[a11y] verdict asserted` is the only substring safe across both.
- **Strictness is what makes a null result legible** (`agent-run.ps1:70`–`:77`): with no driven-session
  banner, `CONDUCTOR_A11Y_STRICT` in `('1','true')` ⇒ stderr `error: CONDUCTOR_A11Y_STRICT is set but no
  driven session attached - the leg proved nothing.` + `exit 1`; unset ⇒ `[a11y] leg skipped (no driven
  session) - exit 0 preserved for an unconfigured host` and a **green exit**.
- **The routine-arm step deliberately omits `$PSNativeCommandUseErrorActionPreference`** (`ci.yml:365`–`:371`,
  a seven-line comment): setting it makes the harness's non-zero a terminating error at the `npm run a11y`
  call, *before* `agent-run.ps1` prints the captured wdio output the verdict is read from — measured
  2026-09-07. Any shell arm B runs in must not set it either.
- **The live-process window is probe (a) of the isolation step** (`ci.yml:411`–`:422`): `Start-Process` of the
  release binary at `:411`; `tasklist` for `conductor-tauri.exe` and `msedgewebview2.exe` at `:420`–`:421`;
  `Stop-Process -Id $p.Id` at `:422`. Section (2) of the probe enumerates exactly those images
  (`webview2-cause-probe.ps1:59`), so the reading it needs exists between `:411` and `:422` and nowhere else
  in the job.
- **Diagnose-only posture is already uniform** (`ci.yml:382`–`:384`, `:513`–`:515`, `:331`–`:333`): the
  isolation step, the cause-probe step and the versions step each carry `if: always()` +
  `continue-on-error: true`. The routine arm at `:352` carries neither — it is the gate.

## Conventions to follow
- **Raw capture out, graded record in** (`.gitignore:17` `/runs/`): the cause-probe step invokes the script
  bare (`ci.yml:517`, no `-OutFile`), so in CI the reading reaches the job log only; `-OutFile` is the
  dev-host control's channel.
- **Host-path-free by output shape, not by a redaction layer**: section (2) prints `ModuleName` +
  `FileVersion` and never `FileName` (`webview2-cause-probe.ps1:57`), and section (1) probes in the
  PowerShell provider form but reports in the colon-free `reg.exe` form (`:27`–`:30`), deliberately so the
  drive-letter anchor `[A-Za-z]:[\/]` cannot false-positive on `HKLM:\`.
- **`Emit`-then-dump**: every line accumulates into `$out` and is written once at `:126`, with the optional
  `-OutFile` copy at `:128`–`:139` and an unconditional `exit 0` at `:141`.

## New files to create
- `conductor-0.3.0/chunks/2026-09-11-hosted-runner-endpoint-cause-closed/evidence/reading.md` — the graded
  record: the module-version reading from the live window, the elevated dev-host arm-B reading beside the
  committed non-elevated control, and the conclusion the pair supports.

## Files to modify
- `.github/workflows/ci.yml` — inside the `WebView2 session isolation (diagnostic)` step, add the
  section-selected probe invocation in probe (a)'s live window, above the `Stop-Process` that ends it. Steps
  are named, not line-cited, per test-plan §9 and a11y-plan §3; the coordinates above are this document's
  measurement basis and will move when the file is edited.
- `scripts/webview2-cause-probe.ps1` — add the section selector parameter and make the completion marker
  report the sections actually run. No second script, no inline copy.

**Caller threading:** none. The probe script has exactly one existing invocation
(`ci.yml`'s `WebView2 cause probes (diagnostic)` step) plus the dev-host control invocation named in the
predecessor plan's gate fence; this chunk adds a second in-workflow invocation. Both are `run:`-step
invocations of a repo-vendored script with literal argv — no registration table, no allowlist, no manifest.
No dependency delta, so no manifest rides this list: `Cargo.lock` and `package-lock.json` are untouched.

## Scope premise closure

1. **`[inferred]` — "a section-selectable invocation of the existing script, not an inline copy and not a
   second script" → VERIFIED, and sharpened.** `architecture.md` §Infrastructure Patterns — Directory
   structure registers the script's character as CI-only / invoked solely by the `a11y` job / wired into
   neither harness shell / adding no 6th agent-run command; all four survive a second in-workflow
   invocation. **The selector does not exist yet** — the script's signature today is
   `param([string]$OutFile)` (`:15`), so adding it is this chunk's work, not an existing affordance.

2. **`[inferred]` — "the completion marker is asserted by a committed gate" → FALSIFIED.**
   `[premise-corrected: `grep -rn "3/3 sections"` over `*.yml` `*.ps1` `*.sh` `*.ts` `*.rs` returns exactly
   ONE hit — the emit site at `scripts/webview2-cause-probe.ps1:124`. No committed gate asserts it. The only
   assertion ever written is a `[[gate]]` entry in the PREDECESSOR CHUNK'S plan fence, which is per-chunk and
   is not re-run by this chunk.]** There is therefore no standing contract to break and no cross-chunk
   coupling to settle — the marker's arity is this chunk's to choose. What remains binding is the *class*
   rule the tests extract states: whatever marker this chunk's own gate asserts must be unable to pass
   vacuously, and two markers must not be substring-satisfiable by one another.

3. **`[inferred]` — "elevation is the surviving candidate cause, carried as a difference and not a
   demonstrated cause" → VERIFIED at HEAD.** `architecture.md` §Established Decisions [CI/CD] carries the
   cause OPEN with no named replacement candidate, and `v3-01`'s notes state elevation as a difference with
   no probe having varied it. The claim re-derives true *as a hypothesis*, which is what this chunk tests.

4. **`[inferred]` — "a local elevated run reproducing the runner's failure mode is itself unproven; dev-host
   elevation and hosted-image elevation are not self-evidently the same condition" → VERIFIED, and it is
   stronger than a caution.** a11y-plan §9 CI Integration → Platform *requires* distinguishing the measured
   platform SET from the CI arrangement and claiming only what was measured; test-plan §6 carries the
   2026-09-08 precedent bounding a reading to the host it was taken on. A universal retirement of elevation
   would violate both. So arm B's null result retires elevation **as measured on this host** — never wider.

**One requirement scope did not state, added by this closure:** arm B must run with
`CONDUCTOR_A11Y_STRICT=1`. Without it, a run where elevation kills `DevToolsActivePort` produces no
driven-session banner and the harness prints `[a11y] leg skipped (no driven session)` and **exits 0**
(`agent-run.ps1:70`–`:77`) — byte-indistinguishable from an unconfigured host. The whole discriminating power
of arm B depends on that handle being set. `scope.md` is amended with this.

## Open questions
- Where the section-2 invocation reads from — probe (a)'s bare-app window only, or also probe (b)'s
  driver-launched host — → blocks: **plan-decision**. Probe (b) is where the failure actually occurs, but its
  process exists only transiently during a POST that fails, and the isolation step's own comment records that
  msedgedriver REMOVES its scoped profile dir when a session fails. Resolved at P4 in favour of (a) alone,
  per the directive's own coordinates; recorded under rejected approaches.
