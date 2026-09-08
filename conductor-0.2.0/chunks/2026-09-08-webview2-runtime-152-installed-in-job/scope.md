# Scope — WebView2 runtime 152+ installed in-job

**Marker:** `2026-09-08-webview2-runtime-152-installed-in-job`
**Working entry:** `conductor-0.2.0/working-route.md:127` (Epoch 6b — Polish & ship)
**Version:** conductor-0.2.0

---

## The one-line intent

Vary **the runtime major** — the one variable every passing case shares and the failing case lacks — by
installing WebView2 Evergreen runtime **152+** in-job before the `a11y` job's probes run, then re-running
probe (a). Claim or defer `v2-24` on what that measures.

---

## What this builds

1. **A runtime-install step in the `a11y` job** of `.github/workflows/ci.yml` (`runs-on: windows-2025`),
   placed so it precedes the steps named **"WebView2 driver + runtime versions (diagnostic)"**,
   **"A11y routine arm (tauri-driver + axe + contrast)"** and **"WebView2 session isolation
   (diagnostic)"**. It downloads the **signed Evergreen bootstrapper**, verifies its signature with
   `Get-AuthenticodeSignature` **before executing it**, and installs.
   *(Anchored by STEP NAME, not line number — test-plan §9 and a11y-plan §3 both require it, since
   coordinates move as the workflow grows and this chunk's own insertion shifts every later line.)*
2. **A signature gate that is a real gate.** The signature check runs BEFORE the bootstrapper executes and
   an unsigned/invalid/untrusted result must stop the step — never a warning that proceeds. This is the
   security half of the entry's "with `Get-AuthenticodeSignature` CHECKED before it runs".
3. **Re-run of probe (a)** — the existing bare-app control inside the **"WebView2 session isolation
   (diagnostic)"** step, unchanged in logic, now observed against a 152+ runtime. Its decisive output is
   `[diag] (a) bare-app DevToolsActivePort first seen:` — `never within 90s` (the 151 measurement)
   versus a number of seconds.
4. **The runtime version printed before and after the install**, so the run's own log establishes which
   major actually ran — the existing versions diagnostic reads the registry key
   (`webview2_runtime_registry`) and already prints it; the install must be observable as having changed it,
   or the probe measures nothing.
5. **`v2-24` claimed or deferred.** This entry OWNS that disposition. The cap currently sits
   `chunk: null · status: planned · method: a11y`, deliberately pooled rather than `deferred` (a `deferred`
   status is terminal until the next version's intake) precisely so it could wait for this probe.

## The PREREQ this chunk absorbs

**`PREREQ: close rust gate deferral (deferred since `2026-09-08-hosted-runner-webview2-session`)`.**

The prior chunk deferred the Rust workspace compile gates under the source-delta-proportional rule (its
report.md:113 — "zero compiled-source delta; declared in the plan"). That deferral is now this chunk's to
close: **`cargo nextest run --workspace --profile ci` and `cargo clippy --workspace --all-targets -- -D
warnings` are MANDATORY Test Commands here**, not deferrable, regardless of this chunk's own source delta.

The standing external-decay supply-chain pin (`cargo audit` + the `cargo deny` overlap, with the local
advisory-db porcelain probe ahead of it) rides this chunk as it rides every chunk taken up.

## Boundaries — what this chunk must NOT do

- **ONE push.** The entry budgets a single push of the probe. Iterating in CI is out of scope.
- **No gate step gains `continue-on-error`.** The `a11y` job stays **RED** on the build branch while the
  cause is open; a green-by-suppression job is exactly the hollow pass this project's rules forbid.
  (`continue-on-error: true` on the *diagnostic* steps at `:315` is pre-existing and correct — a diagnostic
  must not decide the gate. The distinction is diagnostic-vs-gate, not new-vs-old.)
- **No driver-layer change.** `H1` (official `tauri-driver` crate) and `H5` (`tauri-plugin-automation` +
  test-runner-backend) are **retired on evidence, do not revisit** — both are driver-layer and the fault is
  measured upstream of every driver. `H2` (`WEBVIEW2_USER_DATA_FOLDER`) is doubly retired. `H3` as
  driver-skew is rejected (runner driver = runtime = Edge, all `151.0.4129.101`, self-consistent).
- **No self-hosted runner.** `H4` is **route-forward ONLY** — none exists for this project
  (operator-confirmed 2026-09-08) and the repo is public.
- **No scenario, crate or contract change.** This is a CI-workflow chunk plus a ledger disposition.
- **No Pulse dependency.** The `a11y` job runs without a live Pulse; that stays true.

## The measured premise this chunk rests on (folded freight, verbatim marker text preserved)

- **[inferred]** *(entry-stated, measured-marked)* "**the hosted `windows-2025` image's WebView2 runtime
  opens no remote-debugging endpoint, and this is measured DRIVER-INDEPENDENT**" — probe (a) is a bare
  launch with no driver in it at all: `WEBVIEW2_USER_DATA_FOLDER` is honoured (`EBWebView` created under
  it), the app stays alive the whole window, and `DevToolsActivePort` never appears within 90 s, where the
  identical form on the dev host produces it in 1 s (run 34234558853;
  `chunks/2026-09-08-hosted-runner-webview2-session/evidence/probe-verdict.md`).
- **[inferred]** *(entry-stated, measured-marked)* "**the driver was never the discriminator**" —
  everything downstream (msedgedriver's 60 s wait, `session not created`, tauri-driver's relay) follows
  from the endpoint never opening.
- **[inferred]** *(entry-stated, measured-marked)* "**The version axis is the one that survives, on the
  RUNTIME not the driver:**" passing = {runtime 152.0.4191.x × driver 151, 2026-09-02 morning} ∪ {runtime
  152.0.4191.x × driver 152.0.4191.53, since 2026-09-02 14:45}; failing = {runtime 151.0.4129.101 × driver
  151.0.4129.101} on the runner. Every passing case runs runtime **152**; the single failing case runs
  **151**.
- **[inferred]** *(entry-stated, `hypothesis:`-marked — NOT a conclusion)* probe (b)'s `count: 0` (no fresh
  `scoped_dir` seen) is **not** a conclusion — the SR chunk's runner verbose log shows the driver
  populating `Preferences`, so a profile dir existed there; candidates are a `TMP` vs `TEMP` divergence in
  the step shell, or removal inside the 1 s poll interval. **Unmeasured.** Not this chunk's subject; it must
  not be inherited as a cause and must not silently become one.

## Premises this chunk itself introduces — closed at P3

- **VERIFIED** The versions-diagnostic registry read is the right post-install observable. The step
  "WebView2 driver + runtime versions (diagnostic)" reads the EdgeUpdate client key and prints a version
  string only; it returned `151.0.4129.101` on run 34234558853, so the observable demonstrably works and
  is host-path-free.
- **VERIFIED** No other `a11y`-job step depends on the runtime being 151. No step pins a runtime version;
  the only version-sensitive handle is the image-shipped driver (`EDGEWEBDRIVER`), and a11y-plan §1/§3
  already record runtime 152 × driver 151 as a **measured-working pair** (2026-09-02), so installing 152+
  over the image's 151 driver is not a new unsupported combination.
- **VERIFIED (and it is the load-bearing one)** `CONDUCTOR_A11Y_STRICT: '1'` is already set on the routine
  arm, and `continue-on-error` appears at exactly three sites — the two diagnostics and the
  session-diag upload — none of them on a gate path. The chunk's "no gate step gains `continue-on-error`"
  boundary is therefore a *preserve*, not a *change*.
- **VERIFIED — and it confirms the security extract's premise** The `a11y` job performs **no non-loopback
  fetch today**; the only `Invoke-WebRequest` in the whole workflow targets `http://127.0.0.1:9515/session`.
  The bootstrapper download is genuinely the project's **first non-loopback egress in a job step**, which
  is what makes the rule-(b) governed-form question (below) real rather than hypothetical.
- **[inferred — NOT closable in this repo; the plan must MEASURE it, not assume it]** The Evergreen
  bootstrapper installs *latest* and has no version selector, so "152+" is whatever latest is. The half
  that matters is the **no-op risk**: if the install silently declines because a per-machine runtime is
  already present at 151, the probe measures nothing and would read as a falsified hypothesis. This is
  why the pre/post version print is a plan step and not a nicety.
- **[inferred — runner-only, dev-host validation is the mitigation]** The install succeeds on
  `windows-2025` without an interactive session or elevation prompt, and `Get-AuthenticodeSignature`
  returns `Valid` with a Microsoft signer on that image. Neither is verifiable from this host; the
  prior chunk's precedent (dev-host validation caught two probe defects before its single budgeted
  push) applies directly.

## Mechanism-claim re-derivation (folded freight, re-checked at HEAD)

- The three **measured-marked** CARRY claims — endpoint-never-opens, driver-not-the-discriminator, and the
  runtime-major version axis — had their evidence pointer spot-checked still-true at HEAD:
  `evidence/probe-verdict.md` is present and its probe (a) description matches the shipped code in the
  "WebView2 session isolation (diagnostic)" step. **All three stand.**
- The **`hypothesis:`-marked** claim (probe (b)'s `count: 0`) is **not verifiable from this repo** — it
  needs a CI run. It stays marked, and **the plan must not lean on it**; it is explicitly not this
  chunk's subject.

## Research addition — a second, independent liveness witness the probe does not capture

`crates/conductor-tauri/src/main.rs:48` writes `conductor-tauri.jsonl` into `<runs_dir.parent()>/logs/`
unconditionally, and the isolation step already sets `CONDUCTOR_RUNS_DIR: runs/e2e-fixture` — so probe
(a)'s bare launch should be producing `runs/logs/conductor-tauri.jsonl`. The step's staging block copies
only `app.stderr.txt`, `msedgedriver.log` and a Crashpad dir; **it does not stage that file.** Capturing it
gives an app-liveness witness that does not depend on `DevToolsActivePort` at all — directly strengthening
the chunk's decisive "app alive but no endpoint" reading, at the cost of one path in an existing
`foreach`. In scope as a probe-strengthening addition under the ONE-push budget.

## Success shape (what "done" looks like, either way)

This chunk is a **probe**, so both outcomes are legitimate completions:

- **Endpoint opens on 152+** → the runtime major was the cause; the routine arm may now reach a session.
  `v2-24`'s acceptance ("...run in the `a11y` CI job... any violation fails the gate with a non-zero exit")
  becomes reachable and the cap can be claimed on the run's evidence.
- **Endpoint still never opens on 152+** → the runtime-major hypothesis is **falsified**, which is a real
  measurement and must be recorded as one (premise-disproof with its mechanism, per this project's standing
  rule). The remaining unmeasured candidates from the prior verdict — no interactive desktop session on a
  hosted runner, a Session-0 / service-account restriction on the WebView2 browser process, an image policy
  on remote debugging — are then the surviving set, and `v2-24` is **deferred** rather than claimed.

Either way the record names WHICH, with the run id and the `[diag] (a)` line as evidence.

## Surfaces touched

| Surface | Nature |
|---|---|
| `.github/workflows/ci.yml` — `a11y` job | the install step + signature gate; probe (a) re-observed |
| `conductor-0.2.0/verification-matrix.json` — `v2-24` | claimed or deferred; a `notes` line either way |
| `conductor-0.2.0/chunks/{marker}/evidence/` | the probe verdict for this run |

Spec masters (`test-plan.md`, `a11y-plan.md`, `architecture.md` §CI/CD) are **read-only here**; any
amendment they need rides wrap's Expected-amendments flow, never a phase edit.
