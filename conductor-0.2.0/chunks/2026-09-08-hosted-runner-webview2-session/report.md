# Report — 2026-09-08-hosted-runner-webview2-session

**Chunk:** Hosted-runner WebView2 session — measure why a WebView2-mode session never exposes
`DevToolsActivePort` on the hosted `windows-2025` runner, and dispose of `v2-24` either way.
**Date:** 2026-09-08
**Commits:** none since `last_wrap` — this wrap authors the chunk's first commit.

## Changes (structured — detectors read this)

- **Files:** `.github/workflows/ci.yml` (+71/−4, basis `git diff --stat HEAD`) ·
  `conductor-0.2.0/chunks/2026-09-08-hosted-runner-webview2-session/{scope,research,plan,report}.md` (new) ·
  `…/evidence/probe-verdict.md` (new) · `.andromeda/runs/2026-09-08T09-42-42-phase/` (17 files) ·
  `.andromeda/runs/2026-09-08T14-20-00-wrap/` · route + matrix + state as usual.
- **Symbols / APIs:** none — **zero source delta**. No crate, binary, test or `src/` file was touched.
- **Crates / modules:** none.
- **Dependencies:** none added, none bumped. `Cargo.lock` and `package-lock.json` untouched.
- **Schema / config:** none.
- **Spec-master edits:** none applied at P1 (P2 owns them).
- **Counts / qualifiers moved:** the driver↔runtime pairing record. **Three sites state a driver/runtime
  version, in TWO masters — not one** (basis: `grep -cE '151\.0\.4129|152\.0\.4191' .andromeda/{seven}.md` →
  `test-plan 1`, `a11y-plan 2`, the other five `0`; a `msedgedriver`-token sweep is WIDER and hits
  `architecture:194`, `test-plan:307,:469`, `a11y-plan:115,:217,:218,:424`, so the token is a poor proxy for
  the claim):
  - **`test-plan.md:307`** — states the working set as msedgedriver `151.0.4129.101` driving Runtime 152 green
    across the `sr*` sessions and the routine `--e2e` re-run, "AND, un-refreshed across a host runtime bump,
    WebView2 Runtime 152.0.4191.62 green on four `sr*` sessions plus the headful self-verify (2026-09-04)".
    **The attribution of the 2026-09-04 runs to driver 151 is measured FALSE** — see *Spec claims disproved*.
  - **`a11y-plan.md:115`** — "driven there 2026-09-02 against WebView2 152.0.4191.53 under a 151 msedgedriver,
    a measured unsupported-but-working cross-major pair". Dated 2026-09-02 and therefore possibly the TRUE
    morning measurement (the driver was replaced that day at 14:45). **Flagged for verification, not asserted
    false** — the detector must date the runs before touching it.
  - **`a11y-plan.md:333`** — "this pass ran on WebView2 152.0.4191.53": a RUNTIME statement with no driver
    attribution. No claim to correct.
- **Dev-tool versions:** msedgedriver on the dev host **151.0.4129.101 → 152.0.4191.53**, mtime
  **2026-09-02 14:45**, with `msedgedriver-151.0.4129.101.exe` (2026-08-20) kept beside it. **Not installed by
  this chunk** — an operator host action of 2026-09-02 that this chunk MEASURED and that corrects the record.
  Basis: operator measurement of the dev host's edgedriver directory; plus `runs/a11y-e2e.log`, which I
  re-derived at **24** `webview2 152.0.4191.66 windows` banners, **all** of them runtime 152 and none any
  other version (`grep -aoE 'webview2 [0-9.]+ windows' … | sort | uniq -c`). The directive cites 46 from an
  earlier reading; that log truncates per `--e2e` invocation, so the two counts are different moments and the
  load-bearing fact — every banner is runtime 152 — holds on both.
- **Harness / gate surface:** the `a11y` job's step named **`WebView2 session isolation (diagnostic)`**
  (`continue-on-error: true`, `if: always()`) gains three measurements — (a) a bare-app control polling
  `EBWebView\DevToolsActivePort` for ≥90 s under `RUNNER_TEMP`, (b) a concurrent `Start-Job` poll of
  `$env:TEMP\scoped_dir*` across the driver's wait, (c) a `LOCALAPPDATA` default-profile check — plus a
  `TEMP`/`RUNNER_TEMP` same-volume boolean. **No gate step changed:** `A11y routine arm …` and
  `A11y violation-JSON conformance gate` remain `continue-on-error: False` (basis: YAML parse of the job's
  step list, post-edit). `runs-on: windows-2025` unchanged.
- **Env vars:** the diagnostic step SETS `WEBVIEW2_USER_DATA_FOLDER` and
  `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` for probe (a) and clears both before (b). **Neither is a
  `CONDUCTOR_*` handle, neither is read by any shipped binary, and both live inside one
  `continue-on-error` CI diagnostic** — `ci.yml` NAMES/sets them but does not READ them, so the
  `EDGEWEBDRIVER` precedent (registered because a shipped artifact both names AND reads it) does not
  cleanly apply. Recorded for the registry detector to judge rather than pre-decided.
- **Cross-project / external claims:** the GitHub-hosted `windows-2025` image's WebView2 runtime behaviour.
  Repo: `actions/runner-images` (not read). Basis: run **34234558853**'s `a11y` job log — driver, Runtime and
  Edge all `151.0.4129.101`.
- **Reverted / negative API facts:** the planned `appEnv` key setting `WEBVIEW2_USER_DATA_FOLDER` in
  `crates/conductor-tauri/ui/wdio.conf.ts` was **never written**. It was retired at the P5 review on an
  operator dev-host measurement (msedgedriver creates its own `%TEMP%\scoped_dir…\EBWebView` profile and
  overrides an inherited handle). `wdio.conf.ts` is therefore untouched by this chunk.
- **Insufficient fixes (written, kept, not the remedy):** none — this chunk ships no remedy by design.
- **Spec claims disproved by measurement:**
  1. **`test-plan.md:307` — the driver attribution.** The doc credits the 2026-09-02 `sr*`/`--e2e` runs and
     the 2026-09-04 runs to msedgedriver `151.0.4129.101`. Measured false: the dev-host driver was replaced
     with `152.0.4191.53` at 2026-09-02 **14:45**, so only the 2026-09-02 **morning** runs were driver-151.
     Everything since — SR chunk, `a11y-ci-gate`, today's probes — ran **runtime 152 with driver 152**.
     **Retire by naming the SET, not a fresh literal:** passing = {runtime 152.0.4191.x, driver 151 on
     2026-09-02 morning} ∪ {runtime 152.0.4191.x, driver 152.0.4191.53 since 14:45}; failing = {runtime
     151.0.4129.101, driver 151.0.4129.101} on the hosted runner. Evidence: operator measurement of the
     driver binary + mtime; run 34234558853's version diagnostic for the runner side; 24 runtime-152 banners
     in `runs/a11y-e2e.log` (re-derived). **Independently corroborated in this session:** today's dev-host
     driver-alone probe returned `"msedgedriverVersion":"152.0.4191.53"` and `"browserVersion":"152.0.4191.66"`
     in its own capability response — so the dev host is measurably driver-152/runtime-152 right now, from a
     reading of mine rather than a dictated fact.
  2. **This chunk's own evidence, self-retracted.** `evidence/probe-verdict.md` first read probe (b)'s
     `count: 0` as "consistent with (a) — the driver left no profile directory". Retracted: the SR chunk's
     runner verbose log shows msedgedriver populating `Preferences`, so a profile dir demonstrably existed
     there. Now recorded as `hypothesis:` (TMP vs TEMP, or removal inside the 1 s poll interval) —
     unmeasured, never a conclusion. Already corrected in the artifact; no spec states it.
- **Expected amendments (from plan):**
  - `test-plan.md` §9 / §6 — **carried.** Fact in *Counts / qualifiers moved* + *Spec claims disproved* #1.
    Located by `grep -cE '151\.0\.4129|152\.0\.4191' .andromeda/test-plan.md` → **1** hit (`:307`).
  - `a11y-plan.md` §11 → Strategy — **carried**, and **wider than the plan anticipated.** Two facts land
    here: (i) the "CI PROOF still owed" record now has a measured cause, located by
    `grep -c 'Hosted-runner WebView2 session' .andromeda/a11y-plan.md` → **1** hit; and (ii) `:115` carries a
    driver-attribution of its own (same `grep -cE` version sweep → **2** hits in this master), which the plan
    did not foresee because it assumed `test-plan:307` was the only site.
  - `matrix#v2-24 notes` — **ledger-note — owner P7.3.** Per the wrap directive `v2-24` stays `planned`
    (pooled); notes gain measurements (a)/(b)/(c) and the runtime hypothesis.
  - `architecture.md` §Occupied Resources and `security-plan.md` §Input Validation — **superseded.** Both
    entries existed only to register the new env handle H2 would have introduced; H2 was retired at P5 and
    no handle was added to any shipped reader. (The diagnostic-scope env vars above are recorded separately
    for the registry detector.)
- **Coverage of new surfaces:** no external surface, hot-path operation or UI element was added.
  - `ci.yml → WebView2 session isolation (diagnostic)` → validation n/a (no external input; all values are
    step-local or runner-provided) · instrumentation n/a (a CI diagnostic, not a Conductor code path) · PII
    n/a — but host-path hygiene ✓ (booleans, leaf names and elapsed seconds only; verified by a 16-line
    printed-output probe and by the staged-diff grep) · tests ✓ (its logic was executed on the dev host,
    regenerated byte-faithfully from `ci.yml`, and reproduced the known-good numbers) · a11y n/a · tokens n/a.

## Deviations from intent

1. **H2 retired before implement; the chunk became diagnose-only.** The plan as first written shipped a
   `WEBVIEW2_USER_DATA_FOLDER` remedy. The operator's dev-host measurement at the P5 review showed
   msedgedriver sets its own profile folder and overrides an inherited value, so the remedy could not work.
   Plan re-based, `wdio.conf.ts` dropped from the modify-set. *Justification:* a measurement beats a plan.
2. **Two probe defects fixed during step 3's dev-host validation.** `Receive-Job` has no `-Timeout`
   parameter (→ `Wait-Job -Timeout` + `Receive-Job`), and `Select-Object -First 1` over `scoped_dir*` could
   not discriminate — msedgedriver creates two siblings per session and prior runs leave residue, so the
   probe reported `EBWebView=False` on a session that returned `200`. *Justification:* both would have
   failed silently in CI under `continue-on-error` and consumed the single budgeted push.
3. **Rust workspace compile gates deferred** (`nextest --workspace`, `clippy`) — zero compiled-source delta;
   declared in the plan. Every changed-surface gate ran. *Justification:* source-delta-proportional rule.
4. **Test Command 5 is noisy unscoped.** Run without paths it returns 11 hits, all false positives (prose
   enumerating the banned tokens, the grep pattern itself, `p://` in URLs matching `[A-Za-z]:[\/]`); scoped
   it is clean. *Justification:* the pattern admits URL schemes; curation carries the fix.
5. **`v2-24` disposition changed from `deferred` to staying `planned`** (wrap directive 3). *Justification:*
   `deferred` is terminal until the next version's intake, and a follow-up probe is now routed.

## Decisions & corrections

- **Operator, P5 review:** H2 is not a lever — the driver owns the profile folder (measured: bare launch
  writes `DevToolsActivePort` in 1 s; through the driver the profile lands in its own `scoped_dir` even with
  the handle exported). My contrary research conclusion was retracted in `research.md` with its mechanism
  named: I traced the handle to a reader (`tauri-driver`) and never to its final writer (`msedgedriver`).
- **Operator, P5 review:** my validation-1 amendment was itself wrong — the claim arm IS reachable, because
  the probe push runs inside `/implement`'s lifecycle. Scope corrected.
- **Operator, this wrap (1):** the version-axis correction above. H3 stays rejected as **driver**-skew and is
  **OPEN as a runtime-version question** — every passing case runs runtime 152, the one failing case runs
  runtime 151, and the runtime is the variable this probe did not vary.
- **Operator, this wrap (4):** probe (b)'s `count: 0` is not "consistent with (a)" — recorded as
  `hypothesis:`, not a conclusion.
- **Operator, this wrap (2):** one follow-up probe entry is minted before *Release build and bundle*;
  H1/H5 retired on evidence; H4 route-forward only (no self-hosted runner; public repo).
- **Builder judgment, ratified (5):** the driver's `"userDataDir":""` capability response is the MECHANISM
  behind the CARRY's "the verbose log names no path" — it rides the `test-plan.md` §6 row.

## Outcome

**The measured finding:** on the hosted `windows-2025` runner the WebView2 runtime never opens its
remote-debugging endpoint, and this reproduces with **no driver in the picture at all**. Probe (a) — bare
launch, `WEBVIEW2_USER_DATA_FOLDER` honoured (`EBWebView` created under it), app alive the whole window —
never sees `DevToolsActivePort` within 90 s, where the identical form on the dev host produces it in 1 s.
Everything downstream (`session not created`, tauri-driver's relay) follows from that, which is why the
failure was byte-identical through the intermediary and direct. **The driver was never the discriminator.**

Acceptance criteria, re-asserted against the diff:

| Criterion | Verdict against the diff |
|---|---|
| (tests) `--e2e` printed verdict `0 failed · 2 skipped (expected 2) · driven session present` | **MET** — verbatim in the gate log |
| (tests) added steps diagnostic-only; gate steps unconditioned | **MET** — YAML parse: both gate steps `continue-on-error: False` |
| (tests) each probe confirmed against its dev-host reference before the push | **MET** — 1 s / 0.66 s / 2 siblings, from a script regenerated from `ci.yml` |
| (a11y) no skip guard widened; `CONDUCTOR_A11Y_STRICT` still exits non-zero; assertion set unchanged | **MET** — `wdio.conf.ts` not in the diff at all |
| (a11y) one WebdriverIO + tauri-driver stack; `driven`/`sr*` stay operator-local | **MET** — no stack change |
| (security) wdio guards byte-unchanged; CI producer precondition in force | **MET** — file untouched; `[precondition] CONDUCTOR_MSEDGEDRIVER: resolved` in the run log |
| (security) no host path in probe tree / `ci.yml` delta / notes / evidence | **MET** — scoped staged grep clean; 11 unscoped hits all read and classified false-positive |
| (obs) probes emit booleans, leaf names, seconds only | **MET** — 16 printed lines probed; none can emit a path |
| (obs) diagnostics still upload on every iteration | **MET** — `Upload session diagnostics` succeeded on the red run |
| (obs) `journal_conformance` passes under `CONDUCTOR_RUNS_DIR=runs/a11y` | **MET** — 8/8 |
| (arch) `windows-2025` retained; added steps diagnostic-only | **MET** |
| (layouts) no new bracket label / lamp / `ReportState` / ANSI entry / token | **MET** — no such surface touched |
| (process) `v2-24` leaves non-silent | **MET via P7.3** — stays `planned`, notes carry (a)/(b)/(c) + the runtime hypothesis |

**Gates green** (commands run): `test -f "$CONDUCTOR_MSEDGEDRIVER"` · pre/post `tasklist` census ·
`bash scripts/agent-run.sh run --e2e` (printed verdict asserted, not the exit code) ·
`CONDUCTOR_RUNS_DIR=runs/a11y cargo nextest run -p conductor-run --test journal_conformance --profile ci`
(8/8) · the host-path greps. **Smoke:** the self-verify is a plan Test Command and ran as a P2 gate —
recorded, not re-run.

**Outcome basis:** `/implement`'s P4 report, **plus** this wrap's operator directive, which changed four
things a detector must not inherit from the earlier text — the driver-version attribution (item 1), the
`v2-24` disposition from `deferred` to `planned` (item 3), probe (b)'s status from conclusion to
`hypothesis:` (item 4), and the follow-up route entry (item 2). The evidence artifact was corrected before
this report was authored.

**Process hygiene** (implement P4's census, re-measured here — host list readable):

| Process | Started by | Final state |
|---|---|---|
| `tauri-driver`, `node.exe` ×4, `conductor-tauri.exe` (`--e2e`) | this run | terminated — wdio `onComplete`; post-leg census matched the empty baseline |
| `msedgedriver.exe` ×3 (probe validation) | this run | terminated by the script's own `Stop-Process` |
| `conductor-tauri.exe` ×3 + webview children | this run | terminated — orphaned by (b)'s successful session; cleared with the interrupted-leg stop form |
| `msedgewebview2.exe` ×6 | **not this run** | left running — start times `2026-08-13 18:06`, 26 days prior |
| probe ref `ci-probe/2026-09-08-hosted-runner-webview2-session` | this run | **deleted** from origin after reading |
