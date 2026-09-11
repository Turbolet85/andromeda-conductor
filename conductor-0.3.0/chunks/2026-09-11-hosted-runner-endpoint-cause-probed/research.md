# Codebase Research — 2026-09-11-hosted-runner-endpoint-cause-probed

## Scope
- **Depth:** moderate · **Reads:** 7 · **Globs/Greps:** 11 · **Host probes:** 1 (read-only feasibility run)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — index read structurally (65 entries,
  multi-KB per line, per the long-single-line rule). **No firing-form gathered: this chunk names no live leg.**
  Its subject is a CI job, not a harness run against a real process, so the live-leg recipe clause does not fire.

## Files inspected
- `.github/workflows/ci.yml` (full structure + `226-533` in detail) — three jobs: `rust` (16), `frontend` (196),
  `a11y` (227, `windows-2025`). Trigger is `on: push:` / `pull_request:` at `3-5` with **no branch filter**.
  The `a11y` job already carries the diagnostic shape this chunk extends.
- `.andromeda/architecture.md` `142-201` (§Occupied Resources) — the complete env-var registry and its three
  registration bases; §Ports' single CI-scoped egress.
- `crates/conductor-tauri/ui/wdio.conf.ts` `190-250`, `395-435` — the driver spawn and the session readiness path.
- `scripts/agent-run.ps1` (grep, `58-99` + `252-274`) — `Assert-A11yVerdict`, the printed-verdict gate.
- `crates/conductor-run/tests/journal_conformance.rs` — located **by filename**; a content grep for the string
  across `crates/` missed it entirely (the string lives in `ci.yml` and `wdio.conf.ts`, not in the test file).

## Graph impact
**No query run — `derived-without-graph`, and not a cold start.** The modify-set is `.github/workflows/ci.yml`
plus a new evidence document: YAML is not an indexed plane, and the chunk changes no Rust or TS symbol, so there
is no symbol to query and no caller set to thread. This is "nothing to ask", not "the query returned nothing".
The one adjacent Rust artifact, `journal_conformance`, is explicitly untouched by this chunk.

## Patterns detected
- **Diagnostic vs gate posture is enforced by shape, not by comment** (`ci.yml:344`, `:376`, `:507`): every
  diagnostic carries `continue-on-error: true` (two also `if: always()`), while the install gate at `:264` and
  the conformance gate at `:518` carry neither. Three new probes take the diagnostic shape.
- **Runner env is read in the STEP SHELL, never via `${{ env.* }}`** (`ci.yml:396`, `:440`, `:490`, `:501`) —
  the mechanism architecture.md `:195` records as measured (run 34148079506).
- **Report presence/shape, never the value** (`ci.yml:365`, `:418`, `:491`): `Test-Path` booleans, leaf names,
  handle names. The existing probes never print a path.
- **Readiness is polled on an explicit signal, never slept** (`wdio.conf.ts:199-235`): `startNvda` polls NVDA's
  own "NVDA initialized" line for 30 s, with a comment stating why a sleep is wrong.
- **The a11y verdict is parsed from wdio's CAPTURED output, not the job log** (`agent-run.ps1:80-99`, reading
  `$RunsDir/a11y-e2e.log`) — which is why new job-log probe output cannot perturb it.

## Conventions to follow
- **Every runner env var a shipped artifact reads gets registered** in architecture.md §Occupied Resources under
  one of three bases (`architecture.md:195`, `:197`, `:198`): names-AND-reads · SETS-but-never-reads ·
  named-in-output-only.
- **CI jobs and steps are named, never cited by line coordinate** (test-plan §9, a11y-plan §3) — adding steps
  moves every downstream coordinate in this job.
- **An absent key is reported as absent, never omitted** (obs §6 presence-not-non-null).

## New files to create
- `conductor-0.3.0/chunks/2026-09-11-hosted-runner-endpoint-cause-probed/evidence/` — the recorded readings
  artifact (the `v3-01` `ref`), machine-parseable, host-path-free, pairing each CI reading with its dev-host
  control value.

## Files to modify
- `.github/workflows/ci.yml` — add diagnostic probe step(s) to the `a11y` job. No existing step's logic,
  ordering, `if:`, or `continue-on-error` posture changes. **No caller threading applies** (no symbol, no
  signature, no registry/allowlist entry is required code-side; the env registration is a spec-master entry and
  therefore an Expected amendment at wrap, never a touchpoint).

## Findings that change the plan

**1. All three probes are spawn-free — the governed form count stays at SIX.** Measured end-to-end on this host
(exit 0): policy state via `Test-Path` + `Get-ItemProperty`; loaded module version via `Get-Process` +
`.Modules`; session identity via `[System.Diagnostics.Process]::GetCurrentProcess()` +
`[Security.Principal.WindowsIdentity]`. No program is launched, so no seventh crossing and no operator
escalation is owed under security-plan §Security Anti-Patterns rule (b).

**2. The dev host supplies the known-good control the acceptance needs.** `v3-01` requires the readings be
"sufficient to decide whether the endpoint failure is remediable or permanent" — a CI reading alone cannot
decide that. Measured here, where `DevToolsActivePort` appears in ~1 s: **no Edge/EdgeUpdate/EdgeWebView policy
key present** (all four probed keys absent) · `msedgewebview2.exe` + `msedge_elf.dll` at **FileVersion
152.0.4191.66** across 6 live processes · **SessionId 1 · UserInteractive True · AuthenticationType CloudAP ·
IsSystem False · elevated False**. Each CI reading should be recorded beside its control value.

**3. obs's flagged risk does NOT apply.** The zero-unlogged-panics gate lives in the **`rust` job**
(`ci.yml:152-173`, between job starts 16 and 196) and greps two **named files** — `logs/agent-latest.jsonl` and
`logs/producer-stderr.log` — not the job's stderr stream. A diagnostic added to the `a11y` job cannot trip it.
The obs extract read the spec's wording ("plus stderr") rather than the file; the file is narrower.

**4. No automated gate covers this chunk's own artifacts.** The redaction-boundary grep at `ci.yml:165` applies
to `logs/agent-latest.jsonl` in the `rust` job only. Nothing checks host paths in the `a11y` job log or in a
committed `evidence/` file, though security · obs · tests · a11y all mandate their absence. The constraint is
real and unenforced — if the plan wants it proven rather than asserted, it must bring its own check.

**5. Pre-existing registration drift, not this chunk's to fix.** `ci.yml`'s `a11y` job reads `$env:TEMP`
(`:396`, `:440`) and `$env:LOCALAPPDATA` (`:490`, `:501`) — a shipped artifact reading runner variables, the
exact basis on which `RUNNER_TEMP` and `EDGEWEBDRIVER` **are** registered — yet neither name appears anywhere in
`architecture.md` (verified with a pattern wider than the backticked form). It predates this chunk. The plan
should prefer probe forms that add **no new env read** (the .NET session APIs need none), and surface the
existing pair as an Expected amendment rather than silently enlarging the gap.

**6. `Assert-A11yVerdict` is in force and cannot be perturbed by job-log output** (`agent-run.ps1:58-99`):
it requires a driven session, asserts 0 failed spec files, and holds skips at ≤ 2 — parsing wdio's output
captured to `$RunsDir/a11y-e2e.log`. New probe steps print to the job log, a different stream.

## Open questions
- **Will `.Modules` be readable for `msedgewebview2` processes on the hosted runner?** It succeeded here for
  processes this session owns; enumerating modules of a process owned by another account can throw
  Access Denied. → blocks: **implementation-scope** — the probe must record the throw as its reading rather
  than failing, which is what the acceptance's "a probe that returns nothing is recorded as having returned
  nothing" already requires.
- **Does the push that produces the reading also need to be the chunk's final commit?** The reading must cite a
  run of the code as committed, so a later amendment to `ci.yml` would invalidate it. → blocks:
  **plan-decision** — P4 must fix the commit/push/read ordering explicitly.
