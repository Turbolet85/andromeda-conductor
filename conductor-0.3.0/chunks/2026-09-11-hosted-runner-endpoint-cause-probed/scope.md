# Scope — 2026-09-11-hosted-runner-endpoint-cause-probed

**Working-route entry (verbatim):**
> Hosted-runner endpoint cause probed — policy state, module versions, session identity; diagnose-only, the reading recorded whatever it says, host paths scrubbed

**Version:** conductor-0.3.0 · **Epoch 1 — Foundation: the measurements the closures rest on**
**Capability targeted:** `v3-01` — Hosted-runner cause measured, not listed (`method: manual`)

---

## What this chunk builds

Three **named one-line probes** added to the `a11y` job in `.github/workflows/ci.yml`, run on the hosted
`windows-2025` image, plus a **recorded reading** of what each returned on a real CI run.

> **Amended at P4/P5 (intent-incomplete, operator-ratified):** the probe *bodies* live in a new repo script,
> `scripts/webview2-cause-probe.ps1`, which the job's new step calls — not inline in `ci.yml` as this scope
> originally assumed. The reason is this capability's own acceptance: the readings must be sufficient to decide
> *remediable vs permanent*, which needs a dev-host control produced by the **same code** as the CI reading.
> An inline body can only ever run in CI. `ci.yml` already calls `.\scripts\agent-run.ps1` at `:80` and `:375`,
> so the shape is the workflow's own precedent. The probes answer the
three candidates that `v3-01`'s `observed_gap` names as UNMEASURED — none of which any prior run has probed:

1. **Policy state** — the WebView2 / Edge policy the image carries. A registry read of the Edge and WebView2
   policy keys (machine and user hives), reporting each value found and explicitly reporting the absence of a
   key rather than omitting it. The residual names this "the one-line probe".
2. **Module versions actually loaded** — the module version the *host processes* loaded, as distinct from the
   `EdgeUpdate` client `pv` registry value the existing install gate asserts. The install gate proves what was
   *installed*; it says nothing about what `msedgewebview2.exe` actually mapped. [verified P3] — the two
   sources are genuinely distinct and both readable: `Get-Process msedgewebview2 | .Modules` returned
   `msedgewebview2.exe` / `msedge_elf.dll` at `FileVersion 152.0.4191.66` across 6 live processes on the dev
   host, while the install gate reads EdgeUpdate's `pv` value.
3. **Session identity** — whether the runner's process runs in an interactive session with a window station and
   desktop, or in a service/session-0 context. [verified P3] — the property set is measurable and was measured
   on the dev host: `SessionId 1` · `UserInteractive True` · `AuthenticationType CloudAP` · `IsSystem False` ·
   `IsAuthenticated True` · elevated `False`.

Each probe is **diagnose-only**: `continue-on-error: true`, exits 0 whatever it finds, and can never decide the
gate. This matches the two diagnostics already in the job (`WebView2 driver + runtime versions`, `WebView2
session isolation`) and is the shape the job's own comments state for a diagnostic.

The deliverable is the **reading**, not a fix: a recorded artifact stating what each probe returned on the
hosted runner, including probes that returned nothing, and including any reading that contradicts the expected
cause — recorded unchanged.

## What this chunk does NOT build

- **No fix and no remediation.** Whether the endpoint failure is remediable is what the readings decide; acting
  on that decision is not this chunk.
- **No change to the a11y gate's verdict, assertions, or terminal.** That is `v3-02` (Epoch 3), which this
  chunk feeds. The routine arm stays exactly as it is.
- **No change to the routine arm's spec set, the strict-mode guard, or the conformance gate.**
- **No WebView2 runtime pin disposition.** `v3-02`'s `notes` explicitly folds that into `v3-02`, gated on the
  terminal fork — not here.
- **No console-footer / landmark work.** `v3-02`'s second folded obligation; a surface 0.3.0 deliberately does
  not build.
- **No local-host work.** The subject is the hosted image. A dev-host reading is useful only as the known-good
  control a probe is validated against before it is pushed (see Constraints).

## Surfaces and contracts touched

| Surface | Nature of the touch |
|---|---|
| `.github/workflows/ci.yml` — `a11y` job | ADD one diagnostic step only. No existing step's logic, ordering, or `continue-on-error` posture changes. |
| `scripts/webview2-cause-probe.ps1` | NEW (P4 amendment) — the three read-only probe bodies; always exit 0. First script in `scripts/` that is neither harness variant, so `architecture.md`'s directory tree gains an entry at wrap. |
| `conductor-0.3.0/chunks/{marker}/evidence/reading.md` | NEW — the recorded reading, COMMITTED (the `v3-01` `ref`). |
| `runs/webview2-probe/` | NEW, **gitignored** — the raw captures (dev-host control + the ~1.7 MB CI log). The SR leg's split: raw stays under `runs/`, only the graded record is committed. |
| `contracts/`, `crates/`, `scenarios/` | UNTOUCHED. This chunk ships no Rust and no scenario change. |

## Constraints this chunk inherits

- **Artifact hygiene.** No absolute host path may reach the job log or the recorded reading. The job's existing
  diagnostics state this rule inline ("Handle NAME + state only, never the path"; "Leaf names only, never a
  path"); new probes follow it, and a policy *value* that is itself a path is reported by presence/shape, not
  verbatim. (CLAUDE.md universal invariant; `security-plan.md` §Error Handling.)
- **Harness-spawn rule (b).** `security.md` governs SIX spawn forms across THREE loci, `ci.yml` among them, and
  states "a seventh crossing escalates again — this class never becomes routine." A probe that reads the
  registry or process state is not a spawn; a probe that launches a program is. [verified P3] — **none of the
  three probes needs a spawn.** All three run on cmdlets and .NET only (`Test-Path` + `Get-ItemProperty`;
  `Get-Process` + `.Modules`; `[System.Diagnostics.Process]::GetCurrentProcess()` +
  `[Security.Principal.WindowsIdentity]`), measured end-to-end on the dev host at exit 0. The governed form
  count stays at SIX and no operator escalation is owed.
- **A diagnostic can never gate.** `a11y-plan` §11 bans a gate that passes by skipping; symmetrically the job's
  own comments state a diagnostic "exits 0 whatever it finds, so it can never itself decide the gate."
- **Probe validity before belief.** CLAUDE.md's 2026-09-07 learning: a probe that fails where the real path
  passes is evidence about the probe. Each probe is validated against the dev host (a known-good path) before
  its reading from CI is trusted — the `browserName: "webview2"` omission is the measured precedent.

## The lifecycle question this chunk must answer at planning time

`v3-01`'s acceptance requires **a reading from the hosted runner** — which exists only after a push triggers a
CI run. `ci.yml` has `on: push:` with **no branch filter** (`ci.yml:3-5`), so a push of `build/conductor-0.3.0` does
trigger all three jobs; the run is therefore reachable inside this chunk rather than owed to a later one.
[verified P3] — the trigger is measured from the file and the branch carries no exclusion. Two facts shape how:

- The push is an **operator-authorized act**, and the branch currently has **no upstream**.
- The `a11y` job is **already red** for a pre-existing reason (the residual records the routine arm's red having
  moved to tauri-driver never listening on `:4444` — four refused POSTs inside ~1.5 s against a 6.9 s driver
  bind, present in runs 34162118841 / 34251573399 / 34256490781 at runtime 151, predating both the upgrade and
  any driver skew). A red CI run after this chunk's push is that standing red, not a regression this chunk
  introduced, and the plan must say so explicitly so the run is not misread.

## The `:4444` question — RESOLVED at P3, no fourth probe

The residual `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` (absorbed as `v3-02`) carries a **fourth**
finding beside the three candidates: the routine arm's red having moved to the `:4444` tauri-driver bind race.
That claim **re-derives structurally TRUE at HEAD** — `crates/conductor-tauri/ui/wdio.conf.ts:405` spawns
tauri-driver in `onPrepare` and returns immediately, with **no readiness wait on the `:4444` bind**; the only
wait in the file's session path is the `before:` hook (`:416`), which runs *after* WebdriverIO has already
POSTed to that port. The asymmetry is visible in the same file: `startNvda` (`:199-235`) polls NVDA's own
"NVDA initialized" log line for 30 s with a comment stating why a sleep is wrong, while the driver spawn has
neither poll nor sleep.

**Resolution: it earns NO probe here.** The race is a Conductor-side code defect in `wdio.conf.ts`, not a
property of the hosted image, and it is already diagnosed by reading the file — it needs a **fix** (a readiness
wait on the driver's bind, in the shape `startNvda` already models), which is outside this chunk's
diagnose-only boundary. It is recorded here with its mechanism and coordinates so `v3-02` or its own entry can
take it up. Adding a fourth probe would measure something no longer in question.
