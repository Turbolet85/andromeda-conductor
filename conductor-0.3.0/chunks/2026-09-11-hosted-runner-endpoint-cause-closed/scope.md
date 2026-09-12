# Scope — Hosted-runner endpoint cause closed

**Marker:** `2026-09-11-hosted-runner-endpoint-cause-closed`
**Version:** `conductor-0.3.0` · Epoch 1 — Foundation: the measurements the closures rest on
**Working entry (verbatim):**
> Hosted-runner endpoint cause closed — the unread module-version probe placed where the app is alive, and the
> elevation difference varied  PREREQ: close rust gate deferral (deferred since
> `2026-09-11-hosted-runner-endpoint-cause-probed`)

---

## What this chunk is

The predecessor chunk (`2026-09-11-hosted-runner-endpoint-cause-probed`) drove three named probes on the hosted
`windows-2025` runner at CI run `34586959536` and recorded every reading. It retired two of the three candidates
and left the capability `v3-01` deliberately **un-claimed**, because it could not reach the acceptance's second
half — that the readings are *together sufficient to decide* whether the endpoint failure is remediable or
permanent. This chunk owes exactly the two measurements that were missing, and nothing else.

**Measurement A — the module-version reading, taken where its subject is alive.** The probe is correct; its
POSITION made the reading unreachable. It runs in the `WebView2 cause probes (diagnostic)` step, which sits
AFTER the `WebView2 session isolation (diagnostic)` step has already stopped the app it started — measured at
0.645 s of separation on run `34586959536`. Section (2) therefore reported `no live process` for all three
process names. The fix is **placement**, not a longer timeout: a timeout cannot recover a process that no longer
exists.

**Measurement B — elevation varied, on the side where the control already exists.** The predecessor named
elevation as the one surviving difference (`IsElevatedAdmin: True` on the runner, `False` on the dev host) and
explicitly recorded it as *a difference, not a demonstrated cause* — no probe varied it. This chunk varies it,
and it varies it on the **dev host**, where the endpoint currently works, rather than on the runner.

## The two measurements

### A. Module-version probe placed above the teardown

The subject is a live process. `scripts/webview2-cause-probe.ps1` section (2) enumerates loaded modules of
`msedgewebview2`, `conductor-tauri` and `msedge`, reporting `ModuleName` + `FileVersion` only (never `FileName`,
which is a host path), and treating an `ACCESS DENIED` throw as a reading in its own right.

The window where those processes exist in the `a11y` job is inside the isolation step's arm (a): from the
`Start-Process` of the app (`ci.yml:411`) to the `Stop-Process` that ends it (`ci.yml:422`) — the same window
whose `tasklist` at `10:13:21.76Z` showed `conductor-tauri.exe` plus three `msedgewebview2.exe` alive. The
section must fire inside that window.

- The probe body stays in `scripts/webview2-cause-probe.ps1`. The predecessor's recorded reason binds here:
  the same code must produce the dev-host control the CI reading is compared against, and an inline CI-only
  body can never do that. So this is a **section-selectable invocation of the existing script**, not an inline
  copy and not a second script. *(P3: verified — `architecture.md` §Infrastructure Patterns registers all four
  properties and a second in-workflow invocation preserves them. Sharpened: the selector does NOT exist today;
  the signature is `param([string]$OutFile)`, so adding it is this chunk's work.)*
- **The selector must be a validated, strictly-bound parameter** *(added at P5; planning surfaced it)*. Measured
  on the untouched tree: `powershell.exe -File scripts/webview2-cause-probe.ps1 -Section modules` exits **0**,
  writes nothing to stderr and runs all three sections — a plain `param()` block without `[CmdletBinding()]`
  collects an unknown parameter NAME into `$args` rather than rejecting it. So a section selector that is not
  strictly bound degrades silently to a full run, and any gate keyed on the exit code or on a section-(2) line
  would pass identically with the selector never implemented.
- `[premise-corrected: the completion marker is asserted by NO committed gate — `grep -rn "3/3 sections"` over
  `*.yml` `*.ps1` `*.sh` `*.ts` `*.rs` returns exactly one hit, the emit site at
  `scripts/webview2-cause-probe.ps1:124`. The only assertion ever written is a `[[gate]]` entry in the
  PREDECESSOR chunk's plan fence, which is per-chunk and is not re-run here.]* There is no standing contract to
  break, so the marker's arity is this chunk's to choose. What stays binding is the class rule: this chunk's own
  gate must be unable to pass vacuously, and two markers must not be substring-satisfiable by one another.
- The three-section step at `ci.yml:513` keeps its `continue-on-error: true` + `if: always()` diagnose-only
  posture, and so does anything added inside the isolation step. No probe may move the job's verdict.

### B. Elevation varied on the dev host

The **inverse** of the expensive direction. A hosted step is elevated by construction, so varying elevation on
the runner means de-elevating inside it — a scheduled-task or `runas` token dance whose own failure modes would
then have to be separated from the endpoint's. Elevating the dev host is one run.

- **The non-elevated control already exists and is not re-derived here:** the routine arm over the shipped
  bundle at **12 passing / 2 skipped**, banner `[webview2 152.0.4191.66 windows]`, recorded 2026-09-10
  (`conductor-0.3.0/intent.md:28`; `conductor-0.2.0/chunks/2026-09-10-release-build-and-bundle/report.md:95`).
  The probe's own dev-host capture records `IsElevatedAdmin: False` beside the runner's `True`.
- **The arm:** re-run the routine arm on this host from an **elevated** shell, and capture the probe's own
  elevated dev-host reading beside it.
- **This run is an OPERATOR step.** Elevation cannot be self-granted. The plan names the step; the operator runs
  it; the reading is recorded beside the existing non-elevated one. It is **not** modelled as a gate the
  fix-loop can drive.
- **`CONDUCTOR_A11Y_STRICT=1` is mandatory on arm B** *(added by P3's closure; scope did not state it)*. Without
  it, a run where elevation kills `DevToolsActivePort` yields no driven-session banner and the harness prints
  `[a11y] leg skipped (no driven session)` and **exits 0** (`scripts/agent-run.ps1:70`–`:77`) — byte-identical to
  an unconfigured host. Arm B's entire discriminating power rests on that handle being set.
- **The invoking shell is elevated PowerShell running `scripts/agent-run.ps1`, never `bash scripts/agent-run.sh`**
  — in PowerShell `bash` resolves to the WSL relay and dies (`verification-harness.md` L57a) — and that shell must
  NOT set `$PSNativeCommandUseErrorActionPreference`, which destroys the printed verdict the reading is taken from
  (`ci.yml`'s routine-arm step records the measurement).
- **Both outcomes are deliverables, and the plan states both in advance.** If `DevToolsActivePort` disappears
  under local elevation, elevation is established as the cause with a control on both sides and no CI
  round-trip. If it survives, elevation is **retired** — and only then does a runner-side arm earn its cost.
- The discipline this satisfies is the one the earlier raw-`msedgedriver` probe broke: a probe that has not
  produced the KNOWN-GOOD outcome on the known-good host discriminates nothing. Here the known-good outcome is
  already on record, from the same host, under the one variable being changed.

## Causal-mechanism claims entering as hypotheses

- Elevation is the surviving candidate cause of the missing endpoint. Carried with the predecessor's own marker
  text verbatim: *"a difference, not a demonstrated cause — no probe varied it, and nothing here says
  de-elevating opens the endpoint."* This chunk's arm B is what tests it; scope does not assume its outcome.
  *(P3: re-derived true at HEAD — `architecture.md` §Established Decisions [CI/CD] carries the cause OPEN with no
  named replacement candidate, and `v3-01`'s notes state elevation as an unvaried difference. Verified as a
  correctly-stated hypothesis, not as a cause.)*
- That a local elevated run reproduces the runner's failure mode *at all* is itself unproven — elevation on the
  dev host and elevation on a hosted image are not self-evidently the same condition (they differ in principal,
  profile and image). A null result from arm B retires elevation **as measured on this host**; the plan must
  state that reach honestly rather than over-claiming a universal retirement. *(P3: verified, and it is
  mandatory rather than merely prudent — a11y-plan §9 CI Integration → Platform requires distinguishing the
  measured platform SET from the CI arrangement, and test-plan §6 carries the 2026-09-08 precedent bounding a
  reading to its host.)*

## Folded annotations

- **PREREQ (folded from the working entry):** close the rust gate deferral —
  `cargo nextest run --workspace --profile ci`, deferred at `2026-09-11-hosted-runner-endpoint-cause-probed`
  under the source-delta rule (that chunk had zero `.rs` / `Cargo.*` delta). It must appear in this chunk's
  `## Test Commands` with its disposition and its origin named, so the gate is actually re-run rather than
  re-deferred silently. Verified against the predecessor's plan: the `defer` key and its reason are on the
  `cargo nextest` entry there.
- **No `BLOCKED-ON`** on this entry, and none anywhere in `conductor-0.3.0/working-route.md`.
- **No standing external-decay PREREQ** is owed: the advisory-database deferral closed at
  `2026-09-05-audit-corrective`. Whether this chunk carries a dependency delta is a P4 question, not a
  standing pin.

## Sequencing and the operator's acts

Two operator steps, and the plan places them explicitly:

1. **Elevated dev-host run** (arm B) — the operator runs the routine arm plus the probe from an elevated shell.
2. **Commit, then push** — placed **before** any entry that consumes a CI run. The consuming entry is
   `leg = 'operator'` carrying `<id>`, the shape that worked at the predecessor. **The skill does not commit,
   and the plan does not ask it to** — the predecessor's wrap recorded exactly this as a deviation, with the
   attribution that the skill correctly refused.

Whether this chunk needs a CI run at all depends on arm B's outcome and on how arm A's reading is obtained —
a P4 question. If a CI run is consumed, the ordering above binds; a later edit to `ci.yml` or the probe script
invalidates a reading already taken.

## Boundaries — what this chunk does NOT do

- **Does not de-elevate on the runner.** Explicitly deferred: it earns its cost only if arm B leaves elevation
  standing.
- **Does not attempt to fix the endpoint.** The chunk closes a *measurement*. A remediation, if one is
  indicated, is a later route entry.
- **Does not change the job's verdict or the routine arm's logic.** The `a11y` job's pass/fail stays determined
  by the routine arm's printed verdict; every probe stays diagnose-only.
- **Does not re-derive the non-elevated control.** It is committed evidence and is cited, not re-measured.
- **Does not widen the trust boundary.** No new outbound target, no new bind, no new spawned program: the
  governed harness-spawn form count stays at SIX.
- **Does not touch spec masters.** Any spec↔reality gap surfaces at wrap, not here.

## Capability targeted

`v3-01` — *Hosted-runner cause measured, not listed* (`manual`, `planned`, currently `chunk: null`). Its
acceptance is unedited and stands as written; the predecessor un-claimed it deliberately rather than refining
the clause. This chunk exists to make the measurement whole so the clause can be met. Whether it is claimable
at this chunk's wrap — and on what concretized acceptance — is settled at P5 against the plan that actually
results, not assumed here.

## Touchpoints (provisional — P3 confirms)

- `.github/workflows/ci.yml` — the `a11y` job: the isolation step (`:382`–`:512`) and the cause-probe step
  (`:513`–`:517`).
- `scripts/webview2-cause-probe.ps1` — 141 lines, `param([string]$OutFile)`, three sections, completion marker
  at `:124`.
- `conductor-0.3.0/chunks/2026-09-11-hosted-runner-endpoint-cause-closed/evidence/` — the recorded reading.
- `conductor-0.3.0/verification-matrix.json` — `v3-01`.
- Raw captures stay under gitignored `runs/`; only the graded record is committed, per the SR leg's split.
